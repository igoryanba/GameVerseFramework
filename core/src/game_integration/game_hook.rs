//! # Game Process Hook
//!
//! Этот модуль отвечает за подключение к игровому процессу и инжекцию кода.
//! Основано на анализе FiveM архитектуры, но с современным Rust подходом.

use anyhow::{Result, Context, bail};
use std::ffi::CString;
use std::ptr;
use tracing::{info, error, debug, warn};

use super::{GameType};

/// Хук к игровому процессу
#[derive(Debug)]
pub struct GameHook {
    /// Тип игры
    game_type: GameType,
    /// Handle процесса игры
    process_handle: Option<ProcessHandle>,
    /// Базовый адрес модуля игры
    base_address: Option<usize>,
    /// Состояние подключения
    is_attached: bool,
}

/// Cross-platform обертка для handle процесса
#[derive(Debug)]
pub struct ProcessHandle {
    #[cfg(windows)]
    handle: windows::Win32::Foundation::HANDLE,
    #[cfg(unix)]
    pid: i32,
}

/// Информация о процессе игры
#[derive(Debug, Clone)]
pub struct GameProcessInfo {
    /// Имя процесса
    pub process_name: String,
    /// PID процесса
    pub pid: u32,
    /// Базовый адрес
    pub base_address: usize,
    /// Размер модуля
    pub module_size: usize,
}

impl GameHook {
    /// Создать новый game hook
    pub async fn new(game_type: GameType) -> Result<Self> {
        Ok(Self {
            game_type,
            process_handle: None,
            base_address: None,
            is_attached: false,
        })
    }

    /// Подключиться к игровому процессу
    pub async fn attach(&mut self) -> Result<()> {
        if self.is_attached {
            warn!("Already attached to game process");
            return Ok(());
        }

        info!("🔍 Searching for {:?} game process...", self.game_type);

        // Ищем процесс игры
        let process_info = self.find_game_process().await
            .context("Failed to find game process")?;

        info!("🎯 Found {:?} process: PID={}, Base=0x{:X}", 
            self.game_type, process_info.pid, process_info.base_address);

        // Открываем процесс для доступа
        let process_handle = self.open_process(process_info.pid)
            .context("Failed to open game process")?;

        self.process_handle = Some(process_handle);
        self.base_address = Some(process_info.base_address);

        // Проверяем доступность процесса
        self.verify_process_access().await
            .context("Failed to verify process access")?;

        // Инициализируем хуки
        self.initialize_hooks().await
            .context("Failed to initialize game hooks")?;

        self.is_attached = true;
        info!("✅ Successfully attached to {:?} game process", self.game_type);

        Ok(())
    }

    /// Отключиться от игрового процесса
    pub async fn detach(&mut self) -> Result<()> {
        if !self.is_attached {
            return Ok(());
        }

        info!("🔌 Detaching from {:?} game process...", self.game_type);

        // Очищаем хуки
        self.cleanup_hooks().await?;

        // Закрываем handle процесса
        if let Some(handle) = self.process_handle.take() {
            self.close_process(handle)?;
        }

        self.base_address = None;
        self.is_attached = false;

        info!("✅ Successfully detached from game process");
        Ok(())
    }

    /// Найти процесс игры
    async fn find_game_process(&self) -> Result<GameProcessInfo> {
        let process_names = match self.game_type {
            GameType::GtaV => vec!["GTA5.exe", "FiveM.exe"],
            GameType::Rdr2 => vec!["RDR2.exe", "RedM.exe"],
        };

        for process_name in process_names {
            if let Ok(info) = self.find_process_by_name(process_name).await {
                return Ok(info);
            }
        }

        bail!("Game process not found for {:?}", self.game_type)
    }

    /// Найти процесс по имени
    #[cfg(windows)]
    async fn find_process_by_name(&self, process_name: &str) -> Result<GameProcessInfo> {
        use windows::Win32::System::Diagnostics::ToolHelp::*;
        use windows::Win32::Foundation::*;

        unsafe {
            let snapshot = CreateToolhelp32Snapshot(TH32CS_SNAPPROCESS, 0)?;
            
            let mut entry = PROCESSENTRY32W {
                dwSize: std::mem::size_of::<PROCESSENTRY32W>() as u32,
                ..Default::default()
            };

            if Process32FirstW(snapshot, &mut entry).is_ok() {
                loop {
                    let current_name = String::from_utf16_lossy(&entry.szExeFile)
                        .trim_end_matches('\0');

                    if current_name.eq_ignore_ascii_case(process_name) {
                        let base_address = self.get_module_base_address(entry.th32ProcessID, process_name)?;
                        
                        return Ok(GameProcessInfo {
                            process_name: current_name.to_string(),
                            pid: entry.th32ProcessID,
                            base_address,
                            module_size: 0, // Будет определен позже
                        });
                    }

                    if Process32NextW(snapshot, &mut entry).is_err() {
                        break;
                    }
                }
            }

            CloseHandle(snapshot);
        }

        bail!("Process {} not found", process_name)
    }

    /// Найти процесс по имени (Unix)
    #[cfg(unix)]
    async fn find_process_by_name(&self, process_name: &str) -> Result<GameProcessInfo> {
        use std::fs;
        use std::str::FromStr;

        let proc_dir = fs::read_dir("/proc")?;
        
        for entry in proc_dir {
            let entry = entry?;
            let path = entry.path();
            
            if let Some(pid_str) = path.file_name().and_then(|n| n.to_str()) {
                if let Ok(pid) = u32::from_str(pid_str) {
                    let comm_path = path.join("comm");
                    if let Ok(comm) = fs::read_to_string(comm_path) {
                        let comm = comm.trim();
                        if comm.eq_ignore_ascii_case(process_name) {
                            // На Unix получение base address требует дополнительной работы
                            let base_address = self.get_module_base_address_unix(pid as i32)?;
                            
                            return Ok(GameProcessInfo {
                                process_name: comm.to_string(),
                                pid,
                                base_address,
                                module_size: 0,
                            });
                        }
                    }
                }
            }
        }

        bail!("Process {} not found", process_name)
    }

    /// Получить базовый адрес модуля (Windows)
    #[cfg(windows)]
    fn get_module_base_address(&self, pid: u32, module_name: &str) -> Result<usize> {
        use windows::Win32::System::Diagnostics::ToolHelp::*;
        use windows::Win32::Foundation::*;

        unsafe {
            let snapshot = CreateToolhelp32Snapshot(TH32CS_SNAPMODULE, pid)?;
            
            let mut entry = MODULEENTRY32W {
                dwSize: std::mem::size_of::<MODULEENTRY32W>() as u32,
                ..Default::default()
            };

            if Module32FirstW(snapshot, &mut entry).is_ok() {
                loop {
                    let current_name = String::from_utf16_lossy(&entry.szModule)
                        .trim_end_matches('\0');

                    if current_name.eq_ignore_ascii_case(module_name) {
                        CloseHandle(snapshot);
                        return Ok(entry.modBaseAddr as usize);
                    }

                    if Module32NextW(snapshot, &mut entry).is_err() {
                        break;
                    }
                }
            }

            CloseHandle(snapshot);
        }

        bail!("Module {} not found in process {}", module_name, pid)
    }

    /// Получить базовый адрес модуля (Unix)
    #[cfg(unix)]
    fn get_module_base_address_unix(&self, pid: i32) -> Result<usize> {
        use std::fs;
        
        let maps_path = format!("/proc/{}/maps", pid);
        let maps_content = fs::read_to_string(maps_path)?;
        
        for line in maps_content.lines() {
            if line.contains("r-xp") && line.contains(&format!("/{}", pid)) {
                let parts: Vec<&str> = line.split('-').collect();
                if let Some(addr_str) = parts.first() {
                    if let Ok(addr) = usize::from_str_radix(addr_str, 16) {
                        return Ok(addr);
                    }
                }
            }
        }
        
        bail!("Could not find base address for process {}", pid)
    }

    /// Открыть процесс для доступа
    #[cfg(windows)]
    fn open_process(&self, pid: u32) -> Result<ProcessHandle> {
        use windows::Win32::System::Diagnostics::Debug::*;
        use windows::Win32::Foundation::*;

        unsafe {
            let handle = OpenProcess(
                PROCESS_VM_READ | PROCESS_VM_WRITE | PROCESS_VM_OPERATION,
                false,
                pid
            )?;

            Ok(ProcessHandle { handle })
        }
    }

    /// Открыть процесс для доступа (Unix)
    #[cfg(unix)]
    fn open_process(&self, pid: u32) -> Result<ProcessHandle> {
        // На Unix мы используем ptrace или /proc/pid/mem
        Ok(ProcessHandle { pid: pid as i32 })
    }

    /// Закрыть handle процесса
    #[cfg(windows)]
    fn close_process(&self, handle: ProcessHandle) -> Result<()> {
        use windows::Win32::Foundation::*;
        
        unsafe {
            CloseHandle(handle.handle);
        }
        Ok(())
    }

    /// Закрыть handle процесса (Unix)
    #[cfg(unix)]
    fn close_process(&self, _handle: ProcessHandle) -> Result<()> {
        // На Unix нет необходимости закрывать handle
        Ok(())
    }

    /// Проверить доступ к процессу
    async fn verify_process_access(&self) -> Result<()> {
        debug!("Verifying process access...");
        
        // Попробуем прочитать несколько байт из базового адреса
        if let Some(base_addr) = self.base_address {
            let mut buffer = [0u8; 4];
            self.read_process_memory(base_addr, &mut buffer)?;
            debug!("Successfully read from process memory: {:02X?}", buffer);
        }

        Ok(())
    }

    /// Инициализировать хуки
    async fn initialize_hooks(&self) -> Result<()> {
        debug!("Initializing game hooks...");
        
        match self.game_type {
            GameType::GtaV => self.initialize_gta5_hooks().await?,
            GameType::Rdr2 => self.initialize_rdr2_hooks().await?,
        }

        Ok(())
    }

    /// Инициализировать хуки для GTA V
    async fn initialize_gta5_hooks(&self) -> Result<()> {
        debug!("Initializing GTA V specific hooks...");
        
        // TODO: Реализовать специфичные для GTA V хуки
        // - Script engine hooks
        // - Native function table hooks
        // - Player management hooks
        
        Ok(())
    }

    /// Инициализировать хуки для RDR2
    async fn initialize_rdr2_hooks(&self) -> Result<()> {
        debug!("Initializing RDR2 specific hooks...");
        
        // TODO: Реализовать специфичные для RDR2 хуки
        
        Ok(())
    }

    /// Очистить хуки
    async fn cleanup_hooks(&self) -> Result<()> {
        debug!("Cleaning up game hooks...");
        
        // TODO: Реализовать очистку хуков
        
        Ok(())
    }

    /// Прочитать память процесса
    pub fn read_process_memory(&self, address: usize, buffer: &mut [u8]) -> Result<()> {
        if let Some(handle) = &self.process_handle {
            self.read_memory_impl(handle, address, buffer)
        } else {
            bail!("Process not attached")
        }
    }

    /// Записать в память процесса
    pub fn write_process_memory(&self, address: usize, data: &[u8]) -> Result<()> {
        if let Some(handle) = &self.process_handle {
            self.write_memory_impl(handle, address, data)
        } else {
            bail!("Process not attached")
        }
    }

    /// Реализация чтения памяти (Windows)
    #[cfg(windows)]
    fn read_memory_impl(&self, handle: &ProcessHandle, address: usize, buffer: &mut [u8]) -> Result<()> {
        use windows::Win32::System::Diagnostics::Debug::*;

        unsafe {
            let mut bytes_read = 0;
            ReadProcessMemory(
                handle.handle,
                address as *const std::ffi::c_void,
                buffer.as_mut_ptr() as *mut std::ffi::c_void,
                buffer.len(),
                Some(&mut bytes_read)
            )?;

            if bytes_read != buffer.len() {
                bail!("Only read {} of {} bytes", bytes_read, buffer.len());
            }
        }

        Ok(())
    }

    /// Реализация записи памяти (Windows)
    #[cfg(windows)]
    fn write_memory_impl(&self, handle: &ProcessHandle, address: usize, data: &[u8]) -> Result<()> {
        use windows::Win32::System::Diagnostics::Debug::*;

        unsafe {
            let mut bytes_written = 0;
            WriteProcessMemory(
                handle.handle,
                address as *mut std::ffi::c_void,
                data.as_ptr() as *const std::ffi::c_void,
                data.len(),
                Some(&mut bytes_written)
            )?;

            if bytes_written != data.len() {
                bail!("Only wrote {} of {} bytes", bytes_written, data.len());
            }
        }

        Ok(())
    }

    /// Реализация чтения памяти (Unix)
    #[cfg(unix)]
    fn read_memory_impl(&self, handle: &ProcessHandle, address: usize, buffer: &mut [u8]) -> Result<()> {
        use std::fs::File;
        use std::io::{Read, Seek, SeekFrom};

        let mem_path = format!("/proc/{}/mem", handle.pid);
        let mut file = File::open(mem_path)?;
        file.seek(SeekFrom::Start(address as u64))?;
        file.read_exact(buffer)?;

        Ok(())
    }

    /// Реализация записи памяти (Unix)
    #[cfg(unix)]
    fn write_memory_impl(&self, handle: &ProcessHandle, address: usize, data: &[u8]) -> Result<()> {
        use std::fs::OpenOptions;
        use std::io::{Write, Seek, SeekFrom};

        let mem_path = format!("/proc/{}/mem", handle.pid);
        let mut file = OpenOptions::new().write(true).open(mem_path)?;
        file.seek(SeekFrom::Start(address as u64))?;
        file.write_all(data)?;

        Ok(())
    }

    /// Получить базовый адрес
    pub fn base_address(&self) -> Option<usize> {
        self.base_address
    }

    /// Проверить подключение
    pub fn is_attached(&self) -> bool {
        self.is_attached
    }
} 