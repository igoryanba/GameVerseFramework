//! # Native Function Executor
//!
//! Этот модуль отвечает за выполнение нативных функций игры.
//! Основано на анализе FiveM scrEngine, но с современной архитектурой.

use anyhow::{Result, Context, bail};
use std::collections::HashMap;
use std::sync::Arc;
use std::ffi::{CStr, CString};
use tokio::sync::RwLock;
use tracing::{debug, info, warn, error};

#[cfg(windows)]
use windows::Win32::{
    Foundation::{HANDLE, INVALID_HANDLE_VALUE, CloseHandle},
    System::{
        Memory::{VirtualAllocEx, VirtualFreeEx, MEM_COMMIT, MEM_RESERVE, MEM_RELEASE, PAGE_EXECUTE_READWRITE, PAGE_READWRITE},
        Threading::{CreateRemoteThread, WaitForSingleObject, GetExitCodeThread, INFINITE},
        Diagnostics::Debug::{WriteProcessMemory, ReadProcessMemory},
    },
};

#[cfg(unix)]
use std::os::unix::io::RawFd;

use super::{GameType, memory_manager::MemoryManager};

/// Исполнитель нативных функций
#[derive(Debug)]
pub struct NativeExecutor {
    /// Тип игры
    game_type: GameType,
    /// Менеджер памяти
    memory_manager: Arc<RwLock<MemoryManager>>,
    /// Таблица нативных функций
    native_table: HashMap<u64, NativeFunction>,
    /// Адрес таблицы регистрации нативов
    registration_table_address: Option<usize>,
    /// Состояние инициализации
    initialized: bool,
}

/// Информация о нативной функции
#[derive(Debug, Clone)]
pub struct NativeFunction {
    /// Хеш функции
    pub hash: u64,
    /// Имя функции
    pub name: String,
    /// Адрес функции в памяти игры
    pub address: usize,
    /// Количество параметров
    pub param_count: u8,
    /// Тип возвращаемого значения
    pub return_type: NativeReturnType,
    /// Проверена ли функция
    pub verified: bool,
}

/// Тип возвращаемого значения нативной функции
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum NativeReturnType {
    /// Void (ничего не возвращает)
    Void,
    /// Boolean
    Bool,
    /// Integer (32-bit)
    Int,
    /// Float (32-bit)
    Float,
    /// String
    String,
    /// Vector3
    Vector3,
    /// Entity handle
    Entity,
    /// Pointer
    Pointer,
}

/// Контекст выполнения нативной функции
#[derive(Debug)]
pub struct NativeContext {
    /// Параметры функции
    pub params: Vec<NativeValue>,
    /// Результат выполнения
    pub result: Option<NativeValue>,
    /// Ошибка выполнения
    pub error: Option<String>,
}

/// Значение параметра или результата нативной функции
#[derive(Debug, Clone)]
pub enum NativeValue {
    /// Boolean значение
    Bool(bool),
    /// Integer значение
    Int(i32),
    /// Float значение
    Float(f32),
    /// String значение
    String(String),
    /// Vector3 значение
    Vector3 { x: f32, y: f32, z: f32 },
    /// Entity handle
    Entity(u32),
    /// Pointer значение
    Pointer(usize),
    /// Null/None
    Null,
}

impl NativeExecutor {
    /// Создать новый исполнитель нативов
    pub async fn new(game_type: GameType, memory_manager: Arc<RwLock<MemoryManager>>) -> Result<Self> {
        Ok(Self {
            game_type,
            memory_manager,
            native_table: HashMap::new(),
            registration_table_address: None,
            initialized: false,
        })
    }

    /// Инициализировать исполнитель
    pub async fn initialize(&mut self) -> Result<()> {
        if self.initialized {
            return Ok(());
        }

        info!("🔧 Initializing native executor for {:?}...", self.game_type);

        // Найти таблицу регистрации нативов
        self.find_registration_table().await
            .context("Failed to find native registration table")?;

        // Загрузить нативные функции
        self.load_native_functions().await
            .context("Failed to load native functions")?;

        // Верифицировать критические функции
        self.verify_critical_natives().await
            .context("Failed to verify critical native functions")?;

        self.initialized = true;
        info!("✅ Native executor initialized with {} functions", self.native_table.len());

        Ok(())
    }

    /// Найти таблицу регистрации нативов в памяти игры
    async fn find_registration_table(&mut self) -> Result<()> {
        debug!("Searching for native registration table...");

        let table_address = match self.game_type {
            GameType::GtaV => self.find_gta5_registration_table().await?,
            GameType::Rdr2 => self.find_rdr2_registration_table().await?,
        };

        self.registration_table_address = Some(table_address);
        debug!("Found native registration table at 0x{:X}", table_address);

        Ok(())
    }

    /// Найти таблицу регистрации для GTA V
    async fn find_gta5_registration_table(&self) -> Result<usize> {
        // Паттерны для поиска таблицы регистрации в GTA V
        // Основано на анализе FiveM кода
        let patterns = [
            // Паттерн 1: Поиск по сигнатуре
            &[0x48, 0x8D, 0x0D, 0x00, 0x00, 0x00, 0x00, 0x48, 0x8B, 0xD0],
            // Паттерн 2: Альтернативный поиск
            &[0x4C, 0x8D, 0x05, 0x00, 0x00, 0x00, 0x00, 0x48, 0x8B, 0xCB],
        ];

        for pattern in &patterns {
            if let Ok(address) = self.memory_manager.read().await.find_pattern(*pattern).await {
                // Проверяем, что это действительно таблица регистрации
                if self.verify_registration_table(address).await? {
                    return Ok(address);
                }
            }
        }

        bail!("Could not find GTA V native registration table")
    }

    /// Найти таблицу регистрации для RDR2
    async fn find_rdr2_registration_table(&self) -> Result<usize> {
        // Паттерны для RDR2 (аналогично GTA V, но с другими сигнатурами)
        let patterns = [
            &[0x48, 0x8D, 0x15, 0x00, 0x00, 0x00, 0x00, 0x48, 0x8B, 0xC8],
            &[0x4C, 0x8D, 0x0D, 0x00, 0x00, 0x00, 0x00, 0x48, 0x8B, 0xD1],
        ];

        for pattern in &patterns {
            if let Ok(address) = self.memory_manager.read().await.find_pattern(*pattern).await {
                if self.verify_registration_table(address).await? {
                    return Ok(address);
                }
            }
        }

        bail!("Could not find RDR2 native registration table")
    }

    /// Проверить, что найденный адрес действительно таблица регистрации
    async fn verify_registration_table(&self, address: usize) -> Result<bool> {
        // Читаем первые несколько записей и проверяем их валидность
        let mut buffer = [0u8; 64];
        self.memory_manager.read().await.read_memory(address, &mut buffer).await?;

        // Простая проверка: первые 8 байт должны быть валидным указателем
        let first_ptr = u64::from_le_bytes([
            buffer[0], buffer[1], buffer[2], buffer[3],
            buffer[4], buffer[5], buffer[6], buffer[7],
        ]) as usize;

        // Проверяем, что указатель находится в разумных пределах
        Ok(first_ptr > 0x10000 && first_ptr < 0x7FFFFFFFFFFF)
    }

    /// Загрузить нативные функции из таблицы регистрации
    async fn load_native_functions(&mut self) -> Result<()> {
        let table_address = self.registration_table_address
            .ok_or_else(|| anyhow::anyhow!("Registration table not found"))?;

        debug!("Loading native functions from table at 0x{:X}...", table_address);

        // Читаем таблицу регистрации
        // Структура основана на анализе FiveM NativeRegistration
        let mut current_address = table_address;
        let mut loaded_count = 0;

        for _ in 0..1024 { // Максимум 1024 записи для безопасности
            let registration = self.read_native_registration(current_address).await?;
            
            if registration.is_null() {
                break;
            }

            // Обрабатываем каждую функцию в регистрации
            for i in 0..registration.num_entries {
                let hash = registration.hashes[i as usize];
                let handler = registration.handlers[i as usize];

                if hash != 0 && handler != 0 {
                    let native_func = NativeFunction {
                        hash,
                        name: format!("native_0x{:016X}", hash),
                        address: handler,
                        param_count: 0, // Будет определено позже
                        return_type: NativeReturnType::Void, // Будет определено позже
                        verified: false,
                    };

                    self.native_table.insert(hash, native_func);
                    loaded_count += 1;
                }
            }

            current_address = registration.next_registration;
            if current_address == 0 {
                break;
            }
        }

        info!("Loaded {} native functions", loaded_count);
        Ok(())
    }

    /// Прочитать структуру NativeRegistration
    async fn read_native_registration(&self, address: usize) -> Result<NativeRegistration> {
        let mut buffer = [0u8; std::mem::size_of::<NativeRegistration>()];
        self.memory_manager.read().await.read_memory(address, &mut buffer).await?;

        // Преобразуем байты в структуру
        // Это упрощенная версия, в реальности может потребоваться обработка обфускации
        unsafe {
            Ok(std::ptr::read(buffer.as_ptr() as *const NativeRegistration))
        }
    }

    /// Верифицировать критические нативные функции
    async fn verify_critical_natives(&mut self) -> Result<()> {
        debug!("Verifying critical native functions...");

        // Список критически важных функций для проверки
        let critical_hashes = match self.game_type {
            GameType::GtaV => vec![
                0x4F8644AF03D0E0D6, // GET_PLAYER_PED
                0x6E192E33AD436366, // GET_ENTITY_COORDS
                0x06843DA7060A026B, // SET_ENTITY_COORDS
                0xEE778F8C7E1142E2, // ADD_TEXT_COMPONENT_SUBSTRING_PLAYER_NAME
            ],
            GameType::Rdr2 => vec![
                0x275F255ED201B937, // GET_PLAYER_PED
                0x3FEF770D40960D5A, // GET_ENTITY_COORDS
                0x621873ECE1178967, // SET_ENTITY_COORDS
            ],
        };

        let mut verified_count = 0;
        for &hash in &critical_hashes {
            if let Some(native_func) = self.native_table.get_mut(&hash) {
                // Простая проверка: пытаемся прочитать первые байты функции
                let mut code_buffer = [0u8; 16];
                if self.memory_manager.read().await.read_memory(native_func.address, &mut code_buffer).await.is_ok() {
                    // Проверяем, что это похоже на начало функции
                    if code_buffer[0] != 0x00 && code_buffer[0] != 0xCC {
                        native_func.verified = true;
                        verified_count += 1;
                        debug!("Verified native 0x{:016X} at 0x{:X}", hash, native_func.address);
                    }
                }
            }
        }

        if verified_count == 0 {
            bail!("Could not verify any critical native functions");
        }

        info!("Verified {}/{} critical native functions", verified_count, critical_hashes.len());
        Ok(())
    }

    /// Выполнить нативную функцию
    pub async fn execute_native(&self, hash: u64, params: Vec<NativeValue>) -> Result<NativeValue> {
        if !self.initialized {
            bail!("Native executor not initialized");
        }

        let native_func = self.native_table.get(&hash)
            .ok_or_else(|| anyhow::anyhow!("Native function 0x{:016X} not found", hash))?;

        if !native_func.verified {
            warn!("Executing unverified native function 0x{:016X}", hash);
        }

        debug!("Executing native 0x{:016X} with {} params", hash, params.len());

        // Создаем контекст выполнения
        let mut context = NativeContext {
            params,
            result: None,
            error: None,
        };

        // Выполняем функцию через call gate
        self.execute_native_impl(native_func, &mut context).await?;

        if let Some(error) = context.error {
            bail!("Native execution failed: {}", error);
        }

        Ok(context.result.unwrap_or(NativeValue::Null))
    }

    /// Реализация выполнения нативной функции
    async fn execute_native_impl(&self, native_func: &NativeFunction, context: &mut NativeContext) -> Result<()> {
        // Это критическая часть - здесь происходит вызов функции в игре
        // Реализуем базовый call gate для безопасного вызова функций
        
        debug!("Setting up call gate for native 0x{:016X} at 0x{:X}", native_func.hash, native_func.address);
        
        // Проверяем, что у нас есть доступ к памяти
        let memory_manager = self.memory_manager.read().await;
        
        // Создаем структуру для передачи параметров в игру
        let call_context = self.prepare_call_context(&context.params, native_func).await?;
        
        // Выполняем вызов через инжекцию кода
        let result = self.execute_injected_call(native_func.address, call_context).await?;
        
        // Интерпретируем результат
        context.result = Some(self.interpret_result(result, native_func.return_type));
        
        Ok(())
    }

    /// Подготовить контекст вызова
    async fn prepare_call_context(&self, params: &[NativeValue], native_func: &NativeFunction) -> Result<CallContext> {
        let mut call_context = CallContext::new();
        
        // Конвертируем параметры в формат, понятный игре
        for (i, param) in params.iter().enumerate() {
            let raw_value = match param {
                NativeValue::Bool(b) => if *b { 1u64 } else { 0u64 },
                NativeValue::Int(i) => *i as u64,
                NativeValue::Float(f) => f.to_bits() as u64,
                NativeValue::Entity(e) => *e as u64,
                NativeValue::Pointer(p) => *p as u64,
                NativeValue::String(s) => {
                    // Для строк нужно выделить память в игре и скопировать данные
                    self.allocate_string_in_game(s).await? as u64
                },
                NativeValue::Vector3 { x, y, z } => {
                    // Vector3 передается как указатель на структуру
                    self.allocate_vector3_in_game(*x, *y, *z).await? as u64
                },
                NativeValue::Null => 0u64,
            };
            
            call_context.add_parameter(raw_value);
        }
        
        Ok(call_context)
    }

    /// Выполнить инжекцию кода и вызов функции
    async fn execute_injected_call(&self, function_address: usize, call_context: CallContext) -> Result<u64> {
        // Создаем шеллкод для вызова
        let shellcode = self.create_call_shellcode(function_address, &call_context)?;
        
        // Выделяем исполняемую память
        let shellcode_address = self.allocate_executable_memory(shellcode.len()).await
            .context("Failed to allocate executable memory")?;
        
        // Записываем шеллкод
        self.write_shellcode(shellcode_address, &shellcode).await
            .context("Failed to write shellcode")?;
        
        // Выполняем в удаленном потоке
        let result = self.execute_remote_thread(shellcode_address).await
            .context("Failed to execute remote thread")?;
        
        // Освобождаем память
        if let Err(e) = self.free_memory(shellcode_address).await {
            warn!("Failed to free shellcode memory: {}", e);
        }
        
        Ok(result)
    }

    /// Создать shellcode для вызова функции
    fn create_call_shellcode(&self, function_address: usize, call_context: &CallContext) -> Result<Vec<u8>> {
        let mut shellcode = Vec::new();
        
        #[cfg(target_arch = "x86_64")]
        {
            // x64 calling convention (Windows/System V)
            // Сохраняем регистры
            shellcode.extend_from_slice(&[0x50]); // push rax
            shellcode.extend_from_slice(&[0x51]); // push rcx
            shellcode.extend_from_slice(&[0x52]); // push rdx
            shellcode.extend_from_slice(&[0x53]); // push rbx
            
            // Загружаем параметры в регистры (упрощенная версия)
            for (i, &param) in call_context.parameters.iter().enumerate() {
                match i {
                    0 => {
                        // mov rcx, param
                        shellcode.extend_from_slice(&[0x48, 0xB9]);
                        shellcode.extend_from_slice(&param.to_le_bytes());
                    },
                    1 => {
                        // mov rdx, param
                        shellcode.extend_from_slice(&[0x48, 0xBA]);
                        shellcode.extend_from_slice(&param.to_le_bytes());
                    },
                    // Дополнительные параметры через стек
                    _ => {
                        // push param
                        shellcode.extend_from_slice(&[0x68]);
                        shellcode.extend_from_slice(&(param as u32).to_le_bytes());
                    }
                }
            }
            
            // Вызов функции
            // mov rax, function_address
            shellcode.extend_from_slice(&[0x48, 0xB8]);
            shellcode.extend_from_slice(&(function_address as u64).to_le_bytes());
            
            // call rax
            shellcode.extend_from_slice(&[0xFF, 0xD0]);
            
            // Восстанавливаем регистры
            shellcode.extend_from_slice(&[0x5B]); // pop rbx
            shellcode.extend_from_slice(&[0x5A]); // pop rdx
            shellcode.extend_from_slice(&[0x59]); // pop rcx
            shellcode.extend_from_slice(&[0x58]); // pop rax
            
            // Возврат
            shellcode.extend_from_slice(&[0xC3]); // ret
        }
        
        #[cfg(target_arch = "x86")]
        {
            // x86 calling convention
            bail!("x86 architecture not yet supported for call gate");
        }
        
        Ok(shellcode)
    }

    /// Выделить память для строки в игре
    async fn allocate_string_in_game(&self, s: &str) -> Result<usize> {
        let bytes = s.as_bytes();
        let address = self.allocate_memory(bytes.len() + 1).await?;
        
        let memory_manager = self.memory_manager.read().await;
        memory_manager.write_memory(address, bytes).await?;
        memory_manager.write_memory(address + bytes.len(), &[0]).await?; // null terminator
        
        Ok(address)
    }

    /// Выделить память для Vector3 в игре
    async fn allocate_vector3_in_game(&self, x: f32, y: f32, z: f32) -> Result<usize> {
        let address = self.allocate_memory(12).await?; // 3 * sizeof(f32)
        
        let memory_manager = self.memory_manager.read().await;
        memory_manager.write_memory(address, &x.to_le_bytes()).await?;
        memory_manager.write_memory(address + 4, &y.to_le_bytes()).await?;
        memory_manager.write_memory(address + 8, &z.to_le_bytes()).await?;
        
        Ok(address)
    }

    /// Выделить исполняемую память в процессе игры
    async fn allocate_executable_memory(&self, size: usize) -> Result<usize> {
        #[cfg(windows)]
        {
            let memory_manager = self.memory_manager.read().await;
            let process_handle = memory_manager.get_process_handle().await
                .context("Failed to get process handle")?;

            unsafe {
                let address = VirtualAllocEx(
                    process_handle,
                    None,
                    size,
                    MEM_COMMIT | MEM_RESERVE,
                    PAGE_EXECUTE_READWRITE,
                );

                if address.is_null() {
                    bail!("VirtualAllocEx failed for executable memory");
                }

                Ok(address as usize)
            }
        }

        #[cfg(unix)]
        {
            // Unix implementation using mmap with PROT_EXEC
            use libc::{mmap, MAP_PRIVATE, MAP_ANONYMOUS, PROT_READ, PROT_WRITE, PROT_EXEC};
            
            unsafe {
                let address = mmap(
                    std::ptr::null_mut(),
                    size,
                    PROT_READ | PROT_WRITE | PROT_EXEC,
                    MAP_PRIVATE | MAP_ANONYMOUS,
                    -1,
                    0,
                );

                if address == libc::MAP_FAILED {
                    bail!("mmap failed for executable memory");
                }

                Ok(address as usize)
            }
        }
    }

    /// Выделить память в процессе игры
    async fn allocate_memory(&self, size: usize) -> Result<usize> {
        #[cfg(windows)]
        {
            let memory_manager = self.memory_manager.read().await;
            let process_handle = memory_manager.get_process_handle().await
                .context("Failed to get process handle")?;

            unsafe {
                let address = VirtualAllocEx(
                    process_handle,
                    None,
                    size,
                    MEM_COMMIT | MEM_RESERVE,
                    PAGE_READWRITE,
                );

                if address.is_null() {
                    bail!("VirtualAllocEx failed");
                }

                Ok(address as usize)
            }
        }

        #[cfg(unix)]
        {
            // Unix implementation using process_vm_writev for injection
            // For now, use local allocation and copy via process_vm_writev
            use libc::{mmap, MAP_PRIVATE, MAP_ANONYMOUS, PROT_READ, PROT_WRITE};
            
            unsafe {
                let address = mmap(
                    std::ptr::null_mut(),
                    size,
                    PROT_READ | PROT_WRITE,
                    MAP_PRIVATE | MAP_ANONYMOUS,
                    -1,
                    0,
                );

                if address == libc::MAP_FAILED {
                    bail!("mmap failed");
                }

                Ok(address as usize)
            }
        }
    }

    /// Освободить память в процессе игры
    async fn free_memory(&self, address: usize) -> Result<()> {
        #[cfg(windows)]
        {
            let memory_manager = self.memory_manager.read().await;
            let process_handle = memory_manager.get_process_handle().await
                .context("Failed to get process handle")?;

            unsafe {
                let success = VirtualFreeEx(
                    process_handle,
                    address as *const std::ffi::c_void,
                    0,
                    MEM_RELEASE,
                );

                if !success.as_bool() {
                    warn!("VirtualFreeEx failed for address 0x{:X}", address);
                }
            }
        }

        #[cfg(unix)]
        {
            // Unix implementation using munmap
            use libc::munmap;
            
            unsafe {
                // Note: We need to track the size for munmap
                // For now, we'll skip the actual free operation
                // In a real implementation, we'd track allocated regions
                debug!("Skipping munmap for address 0x{:X} (size tracking needed)", address);
            }
        }

        Ok(())
    }

    /// Выполнить код в удаленном потоке
    async fn execute_remote_thread(&self, shellcode_address: usize) -> Result<u64> {
        #[cfg(windows)]
        {
            let memory_manager = self.memory_manager.read().await;
            let process_handle = memory_manager.get_process_handle().await
                .context("Failed to get process handle")?;

            unsafe {
                // Создаем удаленный поток
                let thread_handle = CreateRemoteThread(
                    process_handle,
                    None,
                    0,
                    Some(std::mem::transmute(shellcode_address)),
                    None,
                    0,
                    None,
                );

                if thread_handle.is_invalid() {
                    bail!("CreateRemoteThread failed");
                }

                // Ждем завершения потока
                WaitForSingleObject(thread_handle, INFINITE);

                // Получаем код возврата
                let mut exit_code = 0u32;
                GetExitCodeThread(thread_handle, &mut exit_code);

                // Закрываем дескриптор потока
                CloseHandle(thread_handle);

                Ok(exit_code as u64)
            }
        }

        #[cfg(unix)]
        {
            // Unix implementation using ptrace for code injection
            // This is more complex and requires ptrace privileges
            #[cfg(target_os = "linux")]
            use libc::{ptrace, PTRACE_ATTACH, PTRACE_DETACH, PTRACE_GETREGS, PTRACE_SETREGS, PTRACE_CONT};
            
            #[cfg(target_os = "macos")]
            use libc::{ptrace, PT_ATTACH as PTRACE_ATTACH, PT_DETACH as PTRACE_DETACH, PT_CONTINUE as PTRACE_CONT};
            
            let memory_manager = self.memory_manager.read().await;
            let pid = memory_manager.get_process_id().await
                .context("Failed to get process ID")?;

            unsafe {
                #[cfg(target_os = "linux")]
                {
                    // Attach to process
                    if ptrace(PTRACE_ATTACH, pid, std::ptr::null_mut(), std::ptr::null_mut()) == -1 {
                        bail!("ptrace PTRACE_ATTACH failed");
                    }

                    // Wait for process to stop
                    let mut status = 0i32;
                    libc::waitpid(pid, &mut status, 0);

                    // Get current registers
                    let mut regs: libc::user_regs_struct = std::mem::zeroed();
                    if ptrace(PTRACE_GETREGS, pid, std::ptr::null_mut(), &mut regs as *mut _ as *mut std::ffi::c_void) == -1 {
                        ptrace(PTRACE_DETACH, pid, std::ptr::null_mut(), std::ptr::null_mut());
                        bail!("ptrace PTRACE_GETREGS failed");
                    }

                    // Save original instruction pointer
                    let original_rip = regs.rip;

                    // Set instruction pointer to our shellcode
                    regs.rip = shellcode_address as u64;

                    // Set registers
                    if ptrace(PTRACE_SETREGS, pid, std::ptr::null_mut(), &regs as *const _ as *const std::ffi::c_void) == -1 {
                        ptrace(PTRACE_DETACH, pid, std::ptr::null_mut(), std::ptr::null_mut());
                        bail!("ptrace PTRACE_SETREGS failed");
                    }

                    // Continue execution
                    if ptrace(PTRACE_CONT, pid, std::ptr::null_mut(), std::ptr::null_mut()) == -1 {
                        ptrace(PTRACE_DETACH, pid, std::ptr::null_mut(), std::ptr::null_mut());
                        bail!("ptrace PTRACE_CONT failed");
                    }

                    // Wait for completion (this is simplified - in reality we'd need better synchronization)
                    libc::waitpid(pid, &mut status, 0);

                    // Get result from registers
                    if ptrace(PTRACE_GETREGS, pid, std::ptr::null_mut(), &mut regs as *mut _ as *mut std::ffi::c_void) == -1 {
                        ptrace(PTRACE_DETACH, pid, std::ptr::null_mut(), std::ptr::null_mut());
                        bail!("ptrace PTRACE_GETREGS failed");
                    }

                    let result = regs.rax;

                    // Restore original instruction pointer
                    regs.rip = original_rip;
                    ptrace(PTRACE_SETREGS, pid, std::ptr::null_mut(), &regs as *const _ as *const std::ffi::c_void);

                    // Detach from process
                    ptrace(PTRACE_DETACH, pid, std::ptr::null_mut(), std::ptr::null_mut());

                    Ok(result)
                }

                #[cfg(target_os = "macos")]
                {
                    // macOS ptrace has different signature: ptrace(request, pid, addr, data)
                    // where data is c_int, not a pointer
                    if ptrace(PTRACE_ATTACH, pid, std::ptr::null_mut(), 0) == -1 {
                        bail!("ptrace PTRACE_ATTACH failed");
                    }

                    // Wait for process to stop
                    let mut status = 0i32;
                    libc::waitpid(pid, &mut status, 0);

                    // For macOS, we would need different approach for registers
                    // This is a simplified implementation
                    warn!("Remote thread execution not fully implemented for macOS");

                    // Continue execution
                    if ptrace(PTRACE_CONT, pid, std::ptr::null_mut(), 0) == -1 {
                        ptrace(PTRACE_DETACH, pid, std::ptr::null_mut(), 0);
                        bail!("ptrace PTRACE_CONT failed");
                    }

                    // Detach from process
                    ptrace(PTRACE_DETACH, pid, std::ptr::null_mut(), 0);

                    // Return dummy value for now
                    Ok(0)
                }

                #[cfg(not(any(target_os = "linux", target_os = "macos")))]
                {
                    bail!("Remote thread execution not supported on this platform");
                }
            }
        }
    }

    /// Записать шеллкод в выделенную память
    async fn write_shellcode(&self, address: usize, shellcode: &[u8]) -> Result<()> {
        #[cfg(windows)]
        {
            let memory_manager = self.memory_manager.read().await;
            let process_handle = memory_manager.get_process_handle().await
                .context("Failed to get process handle")?;

            unsafe {
                let mut bytes_written = 0usize;
                let success = WriteProcessMemory(
                    process_handle,
                    address as *const std::ffi::c_void,
                    shellcode.as_ptr() as *const std::ffi::c_void,
                    shellcode.len(),
                    Some(&mut bytes_written),
                );

                if !success.as_bool() || bytes_written != shellcode.len() {
                    bail!("WriteProcessMemory failed");
                }
            }
        }

        #[cfg(unix)]
        {
            // Unix implementation using process_vm_writev
            let memory_manager = self.memory_manager.read().await;
            let pid = memory_manager.get_process_id().await
                .context("Failed to get process ID")?;

            #[cfg(target_os = "linux")]
            use libc::{process_vm_writev, iovec};

            unsafe {
                #[cfg(target_os = "linux")]
                {
                    let local_iov = iovec {
                        iov_base: shellcode.as_ptr() as *mut std::ffi::c_void,
                        iov_len: shellcode.len(),
                    };

                    let remote_iov = iovec {
                        iov_base: address as *mut std::ffi::c_void,
                        iov_len: shellcode.len(),
                    };

                    let bytes_written = process_vm_writev(
                        pid,
                        &local_iov,
                        1,
                        &remote_iov,
                        1,
                        0,
                    );

                    if bytes_written == -1 || bytes_written as usize != shellcode.len() {
                        bail!("process_vm_writev failed");
                    }
                }

                #[cfg(target_os = "macos")]
                {
                    // macOS doesn't have process_vm_writev, use ptrace for memory writing
                    warn!("Memory writing via ptrace not fully implemented for macOS");
                    // For now, just log that we would write the shellcode
                    debug!("Would write {} bytes to address 0x{:X}", shellcode.len(), address);
                }
            }
        }

        Ok(())
    }

    /// Интерпретировать результат вызова
    fn interpret_result(&self, raw_result: u64, return_type: NativeReturnType) -> NativeValue {
        match return_type {
            NativeReturnType::Void => NativeValue::Null,
            NativeReturnType::Bool => NativeValue::Bool(raw_result != 0),
            NativeReturnType::Int => NativeValue::Int(raw_result as i32),
            NativeReturnType::Float => NativeValue::Float(f32::from_bits(raw_result as u32)),
            NativeReturnType::Entity => NativeValue::Entity(raw_result as u32),
            NativeReturnType::Pointer => NativeValue::Pointer(raw_result as usize),
            NativeReturnType::String => {
                // Для строк нужно прочитать из памяти
                // TODO: Реализовать чтение строки по адресу
                NativeValue::String("".to_string())
            },
            NativeReturnType::Vector3 => {
                // Для Vector3 нужно прочитать структуру
                // TODO: Реализовать чтение Vector3 по адресу
                NativeValue::Vector3 { x: 0.0, y: 0.0, z: 0.0 }
            },
        }
    }

    /// Получить информацию о нативной функции
    pub fn get_native_info(&self, hash: u64) -> Option<&NativeFunction> {
        self.native_table.get(&hash)
    }

    /// Получить список всех загруженных нативов
    pub fn list_natives(&self) -> Vec<&NativeFunction> {
        self.native_table.values().collect()
    }

    /// Проверить инициализацию
    pub fn is_initialized(&self) -> bool {
        self.initialized
    }

    /// Получить количество загруженных функций
    pub fn native_count(&self) -> usize {
        self.native_table.len()
    }
}

/// Структура регистрации нативов (упрощенная версия из FiveM)
#[repr(C)]
#[derive(Debug, Clone)]
struct NativeRegistration {
    next_registration: usize,
    handlers: [usize; 7],
    num_entries: u32,
    hashes: [u64; 7],
}

impl NativeRegistration {
    fn is_null(&self) -> bool {
        self.next_registration == 0 && self.num_entries == 0
    }
}

impl NativeValue {
    /// Преобразовать в boolean
    pub fn as_bool(&self) -> Option<bool> {
        match self {
            NativeValue::Bool(b) => Some(*b),
            NativeValue::Int(i) => Some(*i != 0),
            _ => None,
        }
    }

    /// Преобразовать в integer
    pub fn as_int(&self) -> Option<i32> {
        match self {
            NativeValue::Int(i) => Some(*i),
            NativeValue::Bool(b) => Some(if *b { 1 } else { 0 }),
            NativeValue::Entity(e) => Some(*e as i32),
            _ => None,
        }
    }

    /// Преобразовать в float
    pub fn as_float(&self) -> Option<f32> {
        match self {
            NativeValue::Float(f) => Some(*f),
            NativeValue::Int(i) => Some(*i as f32),
            _ => None,
        }
    }

    /// Преобразовать в string
    pub fn as_string(&self) -> Option<String> {
        match self {
            NativeValue::String(s) => Some(s.clone()),
            _ => None,
        }
    }
}

/// Контекст вызова нативной функции
#[derive(Debug)]
struct CallContext {
    parameters: Vec<u64>,
}

impl CallContext {
    fn new() -> Self {
        Self {
            parameters: Vec::new(),
        }
    }
    
    fn add_parameter(&mut self, value: u64) {
        self.parameters.push(value);
    }
} 