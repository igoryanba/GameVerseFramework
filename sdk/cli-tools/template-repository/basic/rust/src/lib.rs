use std::ffi::{CStr, CString};
use std::os::raw::c_char;
use anyhow::Result;
use tracing::{info, error};

/// GameVerse Plugin trait - основной интерфейс для всех плагинов
pub trait GameVersePlugin: Send + Sync {
    /// Инициализация плагина
    fn initialize(&mut self) -> Result<()>;
    
    /// Основная логика плагина
    fn on_player_connect(&self, player_id: u32) -> Result<()>;
    fn on_player_disconnect(&self, player_id: u32) -> Result<()>;
    
    /// Обработка команд
    fn handle_command(&self, command: &str, args: &[&str]) -> Result<String>;
    
    /// Завершение работы плагина
    fn shutdown(&mut self) -> Result<()>;
}

/// Основная структура плагина
pub struct {{plugin_name_camel}}Plugin {
    initialized: bool,
}

impl Default for {{plugin_name_camel}}Plugin {
    fn default() -> Self {
        Self {
            initialized: false,
        }
    }
}

impl GameVersePlugin for {{plugin_name_camel}}Plugin {
    fn initialize(&mut self) -> Result<()> {
        info!("Инициализация плагина {{plugin_name}}");
        self.initialized = true;
        Ok(())
    }
    
    fn on_player_connect(&self, player_id: u32) -> Result<()> {
        info!("Игрок {} подключился", player_id);
        // Здесь ваша логика для подключения игрока
        Ok(())
    }
    
    fn on_player_disconnect(&self, player_id: u32) -> Result<()> {
        info!("Игрок {} отключился", player_id);
        // Здесь ваша логика для отключения игрока
        Ok(())
    }
    
    fn handle_command(&self, command: &str, args: &[&str]) -> Result<String> {
        match command {
            "hello" => Ok(format!("Привет из плагина {{plugin_name}}!")),
            "info" => Ok(format!("Плагин {{plugin_name}} v{{plugin_version}}")),
            _ => Ok(format!("Неизвестная команда: {}", command)),
        }
    }
    
    fn shutdown(&mut self) -> Result<()> {
        info!("Завершение работы плагина {{plugin_name}}");
        self.initialized = false;
        Ok(())
    }
}

// FFI интерфейс для динамической загрузки плагина
static mut PLUGIN_INSTANCE: Option<Box<dyn GameVersePlugin>> = None;

/// Создание экземпляра плагина - вызывается GameVerse при загрузке
#[no_mangle]
pub extern "C" fn create_plugin() -> *mut std::ffi::c_void {
    let plugin = Box::new({{plugin_name_camel}}Plugin::default());
    let plugin_ptr = Box::into_raw(plugin) as *mut std::ffi::c_void;
    
    unsafe {
        PLUGIN_INSTANCE = Some(Box::from_raw(plugin_ptr as *mut {{plugin_name_camel}}Plugin));
    }
    
    plugin_ptr
}

/// Инициализация плагина
#[no_mangle]
pub extern "C" fn initialize_plugin() -> bool {
    unsafe {
        if let Some(ref mut plugin) = PLUGIN_INSTANCE {
            match plugin.initialize() {
                Ok(()) => true,
                Err(e) => {
                    error!("Ошибка инициализации плагина: {}", e);
                    false
                }
            }
        } else {
            false
        }
    }
}

/// Обработка подключения игрока
#[no_mangle]
pub extern "C" fn on_player_connect(player_id: u32) -> bool {
    unsafe {
        if let Some(ref plugin) = PLUGIN_INSTANCE {
            plugin.on_player_connect(player_id).is_ok()
        } else {
            false
        }
    }
}

/// Завершение работы плагина - вызывается при выгрузке
#[no_mangle]
pub extern "C" fn destroy_plugin() {
    unsafe {
        if let Some(mut plugin) = PLUGIN_INSTANCE.take() {
            let _ = plugin.shutdown();
        }
    }
}

/// Получение версии плагина
#[no_mangle]
pub extern "C" fn get_plugin_version() -> *const c_char {
    let version = CString::new("{{plugin_version}}").unwrap();
    version.into_raw()
}

/// Получение названия плагина
#[no_mangle]
pub extern "C" fn get_plugin_name() -> *const c_char {
    let name = CString::new("{{plugin_name}}").unwrap();
    name.into_raw()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_plugin_creation() {
        let mut plugin = {{plugin_name_camel}}Plugin::default();
        assert!(!plugin.initialized);
        
        assert!(plugin.initialize().is_ok());
        assert!(plugin.initialized);
        
        assert!(plugin.shutdown().is_ok());
        assert!(!plugin.initialized);
    }
    
    #[tokio::test]
    async fn test_command_handling() {
        let plugin = {{plugin_name_camel}}Plugin::default();
        
        let result = plugin.handle_command("hello", &[]).unwrap();
        assert!(result.contains("Привет"));
        
        let result = plugin.handle_command("info", &[]).unwrap();
        assert!(result.contains("{{plugin_name}}"));
    }
} 