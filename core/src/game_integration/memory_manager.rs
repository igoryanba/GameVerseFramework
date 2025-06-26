//! # Memory Manager
//!
//! Этот модуль отвечает за безопасную работу с памятью игрового процесса.
//! Включает поиск паттернов, чтение/запись памяти и защиту от ошибок.

use anyhow::{Result, Context, bail};
use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::RwLock;
use tracing::{debug, info, warn, error};

use super::{GameType, game_hook::GameHook};

/// Менеджер памяти игрового процесса
#[derive(Debug)]
pub struct MemoryManager {
    /// Тип игры
    game_type: GameType,
    /// Хук к игровому процессу
    game_hook: Option<Arc<RwLock<GameHook>>>,
    /// Базовый адрес модуля игры
    base_address: Option<usize>,
    /// Размер модуля игры
    module_size: Option<usize>,
    /// Кэш найденных паттернов
    pattern_cache: HashMap<Vec<u8>, usize>,
    /// Состояние инициализации
    initialized: bool,
}

/// Информация о регионе памяти
#[derive(Debug, Clone)]
pub struct MemoryRegion {
    /// Начальный адрес
    pub start_address: usize,
    /// Размер региона
    pub size: usize,
    /// Права доступа
    pub protection: MemoryProtection,
    /// Тип региона
    pub region_type: MemoryRegionType,
}

/// Права доступа к памяти
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum MemoryProtection {
    /// Только чтение
    Read,
    /// Чтение и запись
    ReadWrite,
    /// Чтение и выполнение
    ReadExecute,
    /// Полный доступ
    ReadWriteExecute,
    /// Нет доступа
    NoAccess,
}

/// Тип региона памяти
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum MemoryRegionType {
    /// Код программы
    Code,
    /// Данные
    Data,
    /// Стек
    Stack,
    /// Куча
    Heap,
    /// Неизвестный тип
    Unknown,
}

/// Результат поиска паттерна
#[derive(Debug, Clone)]
pub struct PatternMatch {
    /// Адрес найденного паттерна
    pub address: usize,
    /// Смещение от начала паттерна
    pub offset: usize,
    /// Размер совпадения
    pub size: usize,
}

impl MemoryManager {
    /// Создать новый менеджер памяти
    pub async fn new(game_type: GameType) -> Result<Self> {
        Ok(Self {
            game_type,
            game_hook: None,
            base_address: None,
            module_size: None,
            pattern_cache: HashMap::new(),
            initialized: false,
        })
    }

    /// Инициализировать менеджер памяти
    pub async fn initialize(&mut self) -> Result<()> {
        if self.initialized {
            return Ok(());
        }

        info!("🧠 Initializing memory manager for {:?}...", self.game_type);

        // Получаем информацию о модуле игры
        self.analyze_game_module().await
            .context("Failed to analyze game module")?;

        // Сканируем регионы памяти
        self.scan_memory_regions().await
            .context("Failed to scan memory regions")?;

        self.initialized = true;
        info!("✅ Memory manager initialized - Base: 0x{:X}, Size: 0x{:X}", 
            self.base_address.unwrap_or(0), 
            self.module_size.unwrap_or(0)
        );

        Ok(())
    }

    /// Анализировать модуль игры
    async fn analyze_game_module(&mut self) -> Result<()> {
        debug!("Analyzing game module...");

        // Получаем базовый адрес и размер модуля
        let (base_addr, module_size) = match self.game_type {
            GameType::GtaV => self.get_gta5_module_info().await?,
            GameType::Rdr2 => self.get_rdr2_module_info().await?,
        };

        self.base_address = Some(base_addr);
        self.module_size = Some(module_size);

        debug!("Game module: Base=0x{:X}, Size=0x{:X}", base_addr, module_size);
        Ok(())
    }

    /// Получить информацию о модуле GTA V
    async fn get_gta5_module_info(&self) -> Result<(usize, usize)> {
        // Для GTA V ищем основной исполняемый модуль
        match self.get_module_info_by_name("GTA5.exe").await {
            Ok(info) => Ok(info),
            Err(_) => self.get_module_info_by_name("FiveM.exe").await,
        }
    }

    /// Получить информацию о модуле RDR2
    async fn get_rdr2_module_info(&self) -> Result<(usize, usize)> {
        // Для RDR2 ищем основной исполняемый модуль
        match self.get_module_info_by_name("RDR2.exe").await {
            Ok(info) => Ok(info),
            Err(_) => self.get_module_info_by_name("RedM.exe").await,
        }
    }

    /// Получить информацию о модуле по имени
    #[cfg(windows)]
    async fn get_module_info_by_name(&self, module_name: &str) -> Result<(usize, usize)> {
        use windows::Win32::System::Diagnostics::ToolHelp::*;
        use windows::Win32::Foundation::*;
        use windows::Win32::System::ProcessStatus::*;

        // Получаем PID текущего процесса или ищем процесс игры
        let pid = self.find_game_process_id(module_name).await?;

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
                        return Ok((entry.modBaseAddr as usize, entry.modBaseSize as usize));
                    }

                    if Module32NextW(snapshot, &mut entry).is_err() {
                        break;
                    }
                }
            }

            CloseHandle(snapshot);
        }

        bail!("Module {} not found", module_name)
    }

    /// Получить информацию о модуле по имени (Unix)
    #[cfg(unix)]
    async fn get_module_info_by_name(&self, module_name: &str) -> Result<(usize, usize)> {
        use std::fs;
        
        let pid = self.find_game_process_id(module_name).await?;
        let maps_path = format!("/proc/{}/maps", pid);
        let maps_content = fs::read_to_string(maps_path)?;
        
        for line in maps_content.lines() {
            if line.contains("r-xp") && line.contains(module_name) {
                let parts: Vec<&str> = line.split_whitespace().collect();
                if let Some(addr_range) = parts.first() {
                    let range_parts: Vec<&str> = addr_range.split('-').collect();
                    if range_parts.len() == 2 {
                        let start = usize::from_str_radix(range_parts[0], 16)?;
                        let end = usize::from_str_radix(range_parts[1], 16)?;
                        return Ok((start, end - start));
                    }
                }
            }
        }
        
        bail!("Module {} not found in memory maps", module_name)
    }

    /// Найти PID процесса игры
    async fn find_game_process_id(&self, process_name: &str) -> Result<u32> {
        // Реализация поиска PID процесса
        // Аналогично коду в game_hook.rs
        
        #[cfg(windows)]
        {
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
                            CloseHandle(snapshot);
                            return Ok(entry.th32ProcessID);
                        }

                        if Process32NextW(snapshot, &mut entry).is_err() {
                            break;
                        }
                    }
                }

                CloseHandle(snapshot);
            }
        }

        #[cfg(unix)]
        {
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
                                return Ok(pid);
                            }
                        }
                    }
                }
            }
        }

        bail!("Process {} not found", process_name)
    }

    /// Сканировать регионы памяти
    async fn scan_memory_regions(&mut self) -> Result<()> {
        debug!("Scanning memory regions...");

        // Получаем список всех регионов памяти процесса
        let regions = self.enumerate_memory_regions().await?;
        
        debug!("Found {} memory regions", regions.len());
        
        // Анализируем регионы для поиска интересных областей
        for region in &regions {
            if region.protection == MemoryProtection::ReadExecute || 
               region.protection == MemoryProtection::ReadWriteExecute {
                debug!("Executable region: 0x{:X}-0x{:X} ({} bytes)", 
                    region.start_address, 
                    region.start_address + region.size, 
                    region.size
                );
            }
        }

        Ok(())
    }

    /// Перечислить регионы памяти
    #[cfg(windows)]
    async fn enumerate_memory_regions(&self) -> Result<Vec<MemoryRegion>> {
        use windows::Win32::System::Memory::*;
        use windows::Win32::Foundation::*;

        let mut regions = Vec::new();
        let mut address = 0usize;
        
        // Получаем handle процесса (упрощенная версия)
        let process_handle = self.get_process_handle().await?;

        loop {
            let mut mbi = MEMORY_BASIC_INFORMATION::default();
            
            unsafe {
                let result = VirtualQueryEx(
                    process_handle,
                    Some(address as *const std::ffi::c_void),
                    &mut mbi,
                    std::mem::size_of::<MEMORY_BASIC_INFORMATION>()
                );

                if result == 0 {
                    break;
                }

                if mbi.State == MEM_COMMIT {
                    let protection = self.convert_windows_protection(mbi.Protect);
                    let region_type = self.determine_region_type(&mbi);

                    regions.push(MemoryRegion {
                        start_address: mbi.BaseAddress as usize,
                        size: mbi.RegionSize,
                        protection,
                        region_type,
                    });
                }

                address = (mbi.BaseAddress as usize) + mbi.RegionSize;
            }
        }

        Ok(regions)
    }

    /// Перечислить регионы памяти (Unix)
    #[cfg(unix)]
    async fn enumerate_memory_regions(&self) -> Result<Vec<MemoryRegion>> {
        use std::fs;
        
        let pid = self.find_game_process_id("").await?; // Нужно улучшить
        let maps_path = format!("/proc/{}/maps", pid);
        let maps_content = fs::read_to_string(maps_path)?;
        
        let mut regions = Vec::new();
        
        for line in maps_content.lines() {
            let parts: Vec<&str> = line.split_whitespace().collect();
            if parts.len() >= 2 {
                let addr_range = parts[0];
                let perms = parts[1];
                
                let range_parts: Vec<&str> = addr_range.split('-').collect();
                if range_parts.len() == 2 {
                    let start = usize::from_str_radix(range_parts[0], 16)?;
                    let end = usize::from_str_radix(range_parts[1], 16)?;
                    
                    let protection = self.convert_unix_protection(perms);
                    
                    regions.push(MemoryRegion {
                        start_address: start,
                        size: end - start,
                        protection,
                        region_type: MemoryRegionType::Unknown,
                    });
                }
            }
        }
        
        Ok(regions)
    }

    /// Получить handle процесса
    #[cfg(windows)]
    async fn get_process_handle(&self) -> Result<windows::Win32::Foundation::HANDLE> {
        // Упрощенная версия - в реальности нужно получить handle из game_hook
        use windows::Win32::System::Diagnostics::Debug::*;
        use windows::Win32::Foundation::*;

        let pid = self.find_game_process_id("GTA5.exe").await
            .or_else(|_| self.find_game_process_id("RDR2.exe")).await?;

        unsafe {
            let handle = OpenProcess(
                PROCESS_QUERY_INFORMATION | PROCESS_VM_READ,
                false,
                pid
            )?;
            Ok(handle)
        }
    }

    /// Преобразовать Windows protection в наш enum
    #[cfg(windows)]
    fn convert_windows_protection(&self, protect: windows::Win32::System::Memory::PAGE_PROTECTION_FLAGS) -> MemoryProtection {
        use windows::Win32::System::Memory::*;
        
        match protect {
            PAGE_READONLY => MemoryProtection::Read,
            PAGE_READWRITE => MemoryProtection::ReadWrite,
            PAGE_EXECUTE_READ => MemoryProtection::ReadExecute,
            PAGE_EXECUTE_READWRITE => MemoryProtection::ReadWriteExecute,
            PAGE_NOACCESS => MemoryProtection::NoAccess,
            _ => MemoryProtection::Read, // Fallback
        }
    }

    /// Преобразовать Unix permissions в наш enum
    #[cfg(unix)]
    fn convert_unix_protection(&self, perms: &str) -> MemoryProtection {
        match perms {
            "r--p" => MemoryProtection::Read,
            "rw-p" => MemoryProtection::ReadWrite,
            "r-xp" => MemoryProtection::ReadExecute,
            "rwxp" => MemoryProtection::ReadWriteExecute,
            "---p" => MemoryProtection::NoAccess,
            _ => MemoryProtection::Read, // По умолчанию
        }
    }

    /// Определить тип региона памяти
    #[cfg(windows)]
    fn determine_region_type(&self, mbi: &windows::Win32::System::Memory::MEMORY_BASIC_INFORMATION) -> MemoryRegionType {
        use windows::Win32::System::Memory::*;
        
        match mbi.Type {
            MEM_IMAGE => MemoryRegionType::Code,
            MEM_PRIVATE => MemoryRegionType::Heap,
            MEM_MAPPED => MemoryRegionType::Data,
            _ => MemoryRegionType::Unknown,
        }
    }

    /// Найти паттерн в памяти
    pub async fn find_pattern(&self, pattern: &[u8]) -> Result<usize> {
        if !self.initialized {
            bail!("Memory manager not initialized");
        }

        // Проверяем кэш
        if let Some(&cached_address) = self.pattern_cache.get(pattern) {
            debug!("Pattern found in cache: 0x{:X}", cached_address);
            return Ok(cached_address);
        }

        debug!("Searching for pattern: {:02X?}", pattern);

        let base_addr = self.base_address.ok_or_else(|| anyhow::anyhow!("Base address not set"))?;
        let module_size = self.module_size.ok_or_else(|| anyhow::anyhow!("Module size not set"))?;

        // Ищем паттерн в модуле игры
        let address = self.search_pattern_in_range(base_addr, module_size, pattern).await?;
        
        // Кэшируем результат (нужно сделать mutable)
        // self.pattern_cache.insert(pattern.to_vec(), address);
        
        debug!("Pattern found at: 0x{:X}", address);
        Ok(address)
    }

    /// Поиск паттерна в указанном диапазоне
    async fn search_pattern_in_range(&self, start_addr: usize, size: usize, pattern: &[u8]) -> Result<usize> {
        const CHUNK_SIZE: usize = 4096; // Читаем по 4KB
        
        let mut current_addr = start_addr;
        let end_addr = start_addr + size;
        
        while current_addr < end_addr {
            let read_size = std::cmp::min(CHUNK_SIZE, end_addr - current_addr);
            let mut buffer = vec![0u8; read_size];
            
            if self.read_memory(current_addr, &mut buffer).await.is_ok() {
                // Ищем паттерн в буфере
                if let Some(offset) = self.find_pattern_in_buffer(&buffer, pattern) {
                    return Ok(current_addr + offset);
                }
            }
            
            current_addr += read_size;
        }
        
        bail!("Pattern not found in specified range")
    }

    /// Найти паттерн в буфере
    fn find_pattern_in_buffer(&self, buffer: &[u8], pattern: &[u8]) -> Option<usize> {
        if pattern.is_empty() || buffer.len() < pattern.len() {
            return None;
        }
        
        for i in 0..=(buffer.len() - pattern.len()) {
            let mut matches = true;
            
            for j in 0..pattern.len() {
                // Поддержка wildcard (0x00 означает любой байт)
                if pattern[j] != 0x00 && buffer[i + j] != pattern[j] {
                    matches = false;
                    break;
                }
            }
            
            if matches {
                return Some(i);
            }
        }
        
        None
    }

    /// Установить хук к игровому процессу
    pub fn set_game_hook(&mut self, game_hook: Arc<RwLock<GameHook>>) {
        self.game_hook = Some(game_hook);
    }

    /// Прочитать память
    pub async fn read_memory(&self, address: usize, buffer: &mut [u8]) -> Result<()> {
        if !self.initialized {
            bail!("Memory manager not initialized");
        }

        debug!("Reading {} bytes from 0x{:X}", buffer.len(), address);
        
        if let Some(game_hook) = &self.game_hook {
            let hook = game_hook.read().await;
            hook.read_process_memory(address, buffer)
                .context("Failed to read process memory")?;
        } else {
            bail!("Game hook not available for memory reading");
        }
        
        Ok(())
    }

    /// Записать в память
    pub async fn write_memory(&self, address: usize, data: &[u8]) -> Result<()> {
        if !self.initialized {
            bail!("Memory manager not initialized");
        }

        debug!("Writing {} bytes to 0x{:X}", data.len(), address);
        
        if let Some(game_hook) = &self.game_hook {
            let hook = game_hook.read().await;
            hook.write_process_memory(address, data)
                .context("Failed to write process memory")?;
        } else {
            bail!("Game hook not available for memory writing");
        }
        
        Ok(())
    }

    /// Получить базовый адрес модуля
    pub fn base_address(&self) -> Option<usize> {
        self.base_address
    }

    /// Получить размер модуля
    pub fn module_size(&self) -> Option<usize> {
        self.module_size
    }

    /// Проверить инициализацию
    pub fn is_initialized(&self) -> bool {
        self.initialized
    }

    /// Получить дескриптор процесса для Windows API
    #[cfg(windows)]
    pub async fn get_process_handle(&self) -> Result<windows::Win32::Foundation::HANDLE> {
        if let Some(ref game_hook) = self.game_hook {
            // Получаем дескриптор процесса из game_hook
            // В реальной реализации это будет храниться в GameHook
            use windows::Win32::System::Threading::{OpenProcess, PROCESS_ALL_ACCESS};
            
            unsafe {
                let pid = self.get_process_id().await?;
                let handle = OpenProcess(PROCESS_ALL_ACCESS, false, pid as u32)?;
                Ok(handle)
            }
        } else {
            bail!("Game hook not initialized")
        }
    }

    /// Получить ID процесса
    pub async fn get_process_id(&self) -> Result<i32> {
        if let Some(ref game_hook) = self.game_hook {
            // В реальной реализации это будет храниться в GameHook
            // Пока что используем заглушку
            match self.game_type {
                GameType::GtaV => {
                    // Попытаемся найти процесс GTA V
                    if let Ok(pid) = self.find_game_process_id("GTA5.exe").await {
                        Ok(pid as i32)
                    } else {
                        self.find_game_process_id("FiveM.exe").await.map(|pid| pid as i32)
                    }
                },
                GameType::Rdr2 => {
                    // Попытаемся найти процесс RDR2
                    if let Ok(pid) = self.find_game_process_id("RDR2.exe").await {
                        Ok(pid as i32)
                    } else {
                        self.find_game_process_id("RedM.exe").await.map(|pid| pid as i32)
                    }
                }
            }
        } else {
            bail!("Game hook not initialized")
        }
    }
} 