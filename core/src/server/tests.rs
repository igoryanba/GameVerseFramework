#[cfg(test)]
mod tests {
    use super::*;
    use crate::server::runtime;
    use std::path::Path;
    
    #[test]
    fn test_server_module_imports() {
        // Проверяем, что модуль runtime доступен
        let _: Option<runtime::ServerStatus> = None;
    }
} 