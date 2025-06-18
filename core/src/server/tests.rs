#[cfg(test)]
mod tests {
    use super::*;
    use std::path::Path;
    
    #[test]
    fn test_server_module_imports() {
        // Проверяем, что модуль runtime доступен
        let _: Option<runtime::ServerStatus> = None;
    }
} 