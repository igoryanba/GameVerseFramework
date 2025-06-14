use anyhow::{Result};
use std::path::PathBuf;
use tracing::{warn};

use crate::fivem_parser::FiveMDocParser;
use crate::native_types::{NativeCollection, NativeCategory, NativeFunction, NativeParameter, NativeType};

/// Simple wrapper around existing `FiveMDocParser` that allows us to start working
/// with RedM / RDR2 natives. At ранней стадии формат Markdown очень похож, поэтому
/// мы переиспользуем парсер и, в случае ошибок, подставляем минимальную коллекцию
/// для демонстрации генерации SDK.
pub struct Rdr2DocParser {
    inner: FiveMDocParser,
}

impl Rdr2DocParser {
    pub fn new(local_natives_path: Option<PathBuf>, native_configs_path: Option<PathBuf>) -> Self {
        Self { inner: FiveMDocParser::new(local_natives_path, native_configs_path) }
    }

    /// Try to parse RDR2 docs (Markdown) from URL/local path. If невозможнo (пока нет
    /// полноценного источника), возвращается черновая коллекция с двумя функциями –
    /// достаточно для первого цикла генерации и тестов.
    pub async fn parse_from_url(&self, base_url: &str, specified_categories: Option<&Vec<String>>) -> Result<NativeCollection> {
        match self.inner.parse_from_url(base_url, specified_categories).await {
            Ok(col) if col.total_functions() > 0 => Ok(col),
            Ok(_) | Err(_) => {
                warn!("RDR2 parser: falling back to minimal sample collection");
                Ok(Self::sample_collection())
            }
        }
    }

    pub fn parse_from_local_only(&self, path_to_natives_root: &str, specified_categories: Option<&Vec<String>>) -> Result<NativeCollection> {
        match self.inner.parse_from_local_only(path_to_natives_root, specified_categories) {
            Ok(col) if col.total_functions() > 0 => Ok(col),
            Ok(_) | Err(_) => Ok(Self::sample_collection()),
        }
    }

    /// Very small stub with two RDR2-specific natives for early SDK generation.
    fn sample_collection() -> NativeCollection {
        let mut collection = NativeCollection::new();

        // Category HORSE
        let mut horse_category = NativeCategory {
            name: "HORSE".to_string(),
            description: Some("Horse related natives (sample)".to_string()),
            functions: Vec::new(),
        };

        // Example 1: GET_PLAYER_HORSE
        let func_get_horse = NativeFunction::new("GET_PLAYER_HORSE".to_string(), "HORSE".to_string())
            .with_return_type(NativeType::HorseEntity)
            .with_parameter(NativeParameter::new("player_id".to_string(), NativeType::Player));
        horse_category.functions.push(func_get_horse);

        // Example 2: GET_HORSE_SPEED
        let func_get_speed = NativeFunction::new("GET_HORSE_SPEED".to_string(), "HORSE".to_string())
            .with_return_type(NativeType::Float)
            .with_parameter(NativeParameter::new("horse".to_string(), NativeType::HorseEntity));
        horse_category.functions.push(func_get_speed);

        collection.add_category(horse_category);
        collection
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::TempDir;
    use crate::rust_generator::RustWrapperGenerator;

    #[tokio::test]
    async fn test_sample_collection_generation() {
        let parser = Rdr2DocParser::new(None, None);
        let collection = parser.parse_from_url("https://example.com", None).await.unwrap();
        assert_eq!(collection.total_functions(), 2);

        let tmp_dir = TempDir::new().unwrap();
        let gen = RustWrapperGenerator::new();
        gen.generate_all(&collection, &tmp_dir.path().join("rust")).unwrap();

        // Ensure at least one file generated
        let entries = std::fs::read_dir(tmp_dir.path().join("rust")).unwrap();
        assert!(entries.count() > 0);
    }
} 