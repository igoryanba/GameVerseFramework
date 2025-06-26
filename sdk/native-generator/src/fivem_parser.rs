use anyhow::{anyhow, Result, Context};
use reqwest;
use scraper::{Html, Selector};
use std::path::{Path, PathBuf};
use std::fs;
use std::collections::HashMap;
use tracing::{info, warn, debug, trace};
use regex::Regex;
use url::Url;
use serde::Deserialize;

use crate::native_types::{
    NativeCollection, NativeCategory, NativeFunction, NativeParameter, NativeType
};

// Структуры для парсинга native_configs.toml
#[derive(Deserialize, Debug, Clone)]
struct NativeOverrideConfig {
    name: String,
    return_array_length_out_param: Option<String>,
    parameter_type_overrides: Option<std::collections::HashMap<String, String>>,
    return_type_override: Option<String>,
    // TypeScript specific overrides for the function
    typescript_name_override: Option<String>,
    typescript_return_type_override: Option<String>,
    // Parameter-specific TypeScript overrides
    parameter_typescript_overrides: Option<Vec<ParameterTypeScriptOverride>>,
    // Новые Rust-специфичные поля уровня функции
    mark_safe_wrapper_unsafe: Option<bool>,
    prologue_code: Option<String>,
    epilogue_code: Option<String>,
    // Новое поле для Rust-специфичных переопределений параметров
    parameter_rust_overrides: Option<Vec<ParameterRustOverride>>,
}

#[derive(Deserialize, Debug, Clone)]
struct ParameterTypeScriptOverride {
    original_name: String, // Name of the parameter in the original signature
    typescript_name_override: Option<String>,
    typescript_type_override: Option<String>,
}

// Новая структура для Rust-специфичных переопределений параметров
#[derive(Deserialize, Debug, Clone)]
struct ParameterRustOverride {
    original_name: String, // Имя параметра из документации или авто-генерации
    new_rust_name: Option<String>,
    type_override: Option<String>,
    any_type_hint: Option<String>,
    transform_input_with: Option<String>, 
    default_value_for_optional: Option<String>,
}

#[derive(Deserialize, Debug, Default, Clone)] // Default нужен для случая, если файл не найден
struct AllNativeOverrides {
    #[serde(rename = "override")]
    overrides: Vec<NativeOverrideConfig>,
}

pub struct FiveMDocParser {
    client: reqwest::Client,
    cache_dir: PathBuf,
    local_natives_path: Option<PathBuf>,
    native_overrides: AllNativeOverrides,
}

// Вспомогательная функция для парсинга параметров из сигнатуры в Markdown
fn parse_parameters_from_md_signature(params_str: &str, _function_name_for_debug: &str) -> Result<Vec<NativeParameter>> {
    let mut parameters = Vec::new();
    if params_str.trim().is_empty() {
        return Ok(parameters);
    }
    // println!("[PARAM_PARSE_DEBUG] Parsing params for {}: \"{}\"", _function_name_for_debug, params_str);
    let param_regex = Regex::new(concat!(
        r"(?s)\s*",
        r"(?:/\*.*?\*/\s*)?", 
        r"(?:cs_type\s*\([^)]*\)\s*)?", 
        r"((?:(?:const|volatile|static|unsigned|signed)\s+)*)",
        r"((?:[\w:<>]+\s*[*&]?\s*)+?(?:(?:\s*\[[^\]]*\]\s*[*&]?\s*)*?))",
        r"\s+([a-zA-Z_][\w_]*)",
        r"(\s*\[([^\]]*)\])?", 
        r"\s*(?:,|$)\s*"
    )).unwrap();

    for cap in param_regex.captures_iter(params_str) {
        // let _full_match = cap.get(0).map_or("", |m| m.as_str());
        let type_prefixes = cap.get(1).map_or("", |m| m.as_str()).trim();
        let base_type_name = cap.get(2).map_or("", |m| m.as_str()).trim();
        let param_name_str = cap.get(3).map_or("", |m| m.as_str()).trim();
        let array_suffix_content = cap.get(5).map(|m| m.as_str().trim());

        let mut type_for_native_type_parser = format!("{}{}", type_prefixes, base_type_name);
        if !type_prefixes.is_empty() && !base_type_name.is_empty() {
            type_for_native_type_parser = format!("{} {}", type_prefixes, base_type_name);
        } else if type_prefixes.is_empty() {
            type_for_native_type_parser = base_type_name.to_string();
        }
        type_for_native_type_parser = type_for_native_type_parser.split_whitespace().collect::<Vec<&str>>().join(" ");
        
        let mut element_type_str_for_parsing = type_for_native_type_parser.clone();
        let mut is_array_from_type_suffix = false; 
        let mut size_from_type_suffix_content: Option<String> = None; 

        if element_type_str_for_parsing.ends_with(']') {
            if let Some(bracket_start_pos) = element_type_str_for_parsing.rfind('[') {
                let content_in_brackets = element_type_str_for_parsing[bracket_start_pos + 1 .. element_type_str_for_parsing.len() - 1].trim().to_string();
                size_from_type_suffix_content = if content_in_brackets.is_empty() { 
                    Some("".to_string()) 
                } else {
                    Some(content_in_brackets)
                };
                element_type_str_for_parsing = element_type_str_for_parsing[..bracket_start_pos].trim_end().to_string();
                is_array_from_type_suffix = true;
            }
        }

        /* println!(
            "[PARAM_PARSE_DEBUG]   Param match for {}: full=\"{}\", prefixes=\"{}\", base_type=\"{}\", name=\"{}\", array_content_on_name=\"{:?}\", size_from_type_suffix=\"{:?}\", element_type_str_for_parsing=\"{}\"", 
            _function_name_for_debug, _full_match, type_prefixes, base_type_name, param_name_str, array_suffix_content, size_from_type_suffix_content, element_type_str_for_parsing
        ); */

        if !element_type_str_for_parsing.is_empty() && !param_name_str.is_empty() {
            let param_type_parsed_element = NativeType::from_fivem_type(&element_type_str_for_parsing);
            let mut final_size_info: Option<crate::native_types::ArraySizeInfo> = None;
            let mut is_array = false;

            if let Some(content_on_name) = array_suffix_content {
                is_array = true;
                if content_on_name.is_empty() { 
                    final_size_info = Some(crate::native_types::ArraySizeInfo::Infer);
                } else if let Ok(known_size) = content_on_name.parse::<usize>() {
                    final_size_info = Some(crate::native_types::ArraySizeInfo::Known(known_size));
                } else {
                    final_size_info = Some(crate::native_types::ArraySizeInfo::Dynamic{ size_param_name: content_on_name.to_string() });
                }
            } else if is_array_from_type_suffix {
                is_array = true;
                if let Some(content_on_type) = size_from_type_suffix_content {
                    if content_on_type.is_empty() { 
                        final_size_info = Some(crate::native_types::ArraySizeInfo::Infer);
                    } else if let Ok(known_size) = content_on_type.parse::<usize>() {
                        final_size_info = Some(crate::native_types::ArraySizeInfo::Known(known_size));
                    } else {
                        final_size_info = Some(crate::native_types::ArraySizeInfo::Dynamic{ size_param_name: content_on_type.to_string() });
                    }
                } else {
                    final_size_info = Some(crate::native_types::ArraySizeInfo::Infer); 
                }
            }
            
            if is_array {
                 if let NativeType::Char = param_type_parsed_element {
                     if final_size_info == Some(crate::native_types::ArraySizeInfo::Infer) || final_size_info.is_none() {
                         final_size_info = Some(crate::native_types::ArraySizeInfo::NullTerminated);
                     }
                 }
                 else if let NativeType::String = param_type_parsed_element {
                     if final_size_info == Some(crate::native_types::ArraySizeInfo::Infer) || final_size_info.is_none() {
                         final_size_info = Some(crate::native_types::ArraySizeInfo::NullTerminated);
                     }
                 }
            }

            let final_param_type = if is_array {
                NativeType::Array {
                    element_type: Box::new(param_type_parsed_element),
                    size_info: final_size_info
                }
            } else {
                param_type_parsed_element
            };

            parameters.push(NativeParameter::new(
                param_name_str.to_string(),
                final_param_type,
            ));
        } else {
            /* println!(
                "[PARAM_PARSE_DEBUG] Skipping param for {} due to empty type or name. Full: \"{}\", Base: \"{}\", Name: \"{}\"",
                _function_name_for_debug, _full_match, base_type_name, param_name_str
            ); */
        }
    }
    if parameters.is_empty() && !params_str.trim().is_empty() {
        /* println!(
            "[PARAM_PARSE_DEBUG] Could not parse ANY parameters from non-empty string for {}: \"{}\"",
            _function_name_for_debug,
            params_str
        ); */
    }
    // println!("[PARAM_PARSE_DEBUG] Parsed {} params for {}: {:?}", parameters.len(), _function_name_for_debug, parameters.iter().map(|p| format!("{}:{:?}",p.name, p.param_type)).collect::<Vec<_>>());
    Ok(parameters)
}

impl FiveMDocParser {
    pub fn new(local_natives_path: Option<PathBuf>, native_configs_path: Option<PathBuf>) -> Self {
        let client = reqwest::Client::new();
        let cache_dir = PathBuf::from(".cache");
        if !cache_dir.exists() {
            fs::create_dir_all(&cache_dir).unwrap_or_else(|e| {
                warn!("Failed to create cache directory: {}, proceeding without cache.", e);
            });
        }

        let mut native_overrides = AllNativeOverrides::default();
        if let Some(config_path) = native_configs_path {
            if config_path.exists() {
                match fs::read_to_string(&config_path) {
                    Ok(contents) => {
                        match toml::from_str::<AllNativeOverrides>(&contents) {
                            Ok(parsed_configs) => {
                                info!("Successfully loaded {} native overrides from {}", parsed_configs.overrides.len(), config_path.display());
                                native_overrides = parsed_configs;
                            }
                            Err(e) => {
                                warn!("Failed to parse native_configs.toml at {}: {}. Proceeding without overrides.", config_path.display(), e);
                            }
                        }
                    }
                    Err(e) => {
                        warn!("Failed to read native_configs.toml at {}: {}. Proceeding without overrides.", config_path.display(), e);
                    }
                }
            } else {
                info!("Native configs file not found at {}. Proceeding without overrides.", config_path.display());
            }
        } else {
            info!("No native_configs.toml path provided. Proceeding without overrides.");
        }

        Self {
            client,
            cache_dir,
            local_natives_path,
            native_overrides,
        }
    }
    
    pub async fn parse_from_url(&self, base_url: &str, specified_categories: Option<&Vec<String>>) -> Result<NativeCollection> {
        info!("📥 Downloading FiveM native documentation from: {} (or using local override)", base_url);
        
        // Определение списка категорий для обработки
        let categories_to_process = self.determine_categories_to_process(base_url, specified_categories).await?;
        
        let mut collection = NativeCollection::new();
        collection.metadata.source = if self.local_natives_path.is_some() { 
            format!("local: {:?}, fallback_url: {}", self.local_natives_path.as_ref().unwrap(), base_url) 
        } else { 
            base_url.to_string() 
        };

        for (category_name, category_url_option) in categories_to_process {
            info!("Processing category: '{}'", category_name);
            let mut category_functions: Vec<NativeFunction> = Vec::new();
            let mut processed_locally = false;

            // 1. Попытка парсинга из локальных файлов, если путь указан
            if let Some(local_repo_path) = &self.local_natives_path {
                let category_dir_path = local_repo_path.join(&category_name); 
                if category_dir_path.is_dir() {
                    debug!("Attempting to parse category '{}' from local directory: {}", category_name, category_dir_path.display());
                    match self.parse_category_from_local_markdown_dir(&category_dir_path, &category_name) {
                        Ok(parsed_functions) => {
                            if !parsed_functions.is_empty() {
                                category_functions.extend(parsed_functions);
                                processed_locally = true;
                                info!("Successfully parsed {} functions locally for category '{}'", category_functions.len(), category_name);
                            } else {
                                info!("No functions parsed locally for category '{}'.", category_name);
                            }
                        }
                        Err(e) => warn!("Error parsing local markdown for category '{}' from {}: {}", category_name, category_dir_path.display(), e),
                    }
                } else {
                     debug!("Local directory for category '{}' not found at {}, will try URL if available.", category_name, category_dir_path.display());
                }
            }

            // 2. Если локально не обработано (или нет локального пути) И есть URL, пробуем URL
            if !processed_locally {
                if let Some(category_url) = category_url_option {
                    info!("Category '{}' not fully processed locally (or no local path), attempting URL: {}", category_name, category_url);
                    match self.parse_category_from_html_url(&category_url, &category_name).await {
                        Ok(parsed_functions_from_url) => {
                            if !parsed_functions_from_url.is_empty() {
                                info!("Parsed {} functions from URL for category '{}'", parsed_functions_from_url.len(), category_name);
                                // Избегаем дублирования, если какая-то функция была частично найдена локально (маловероятно при текущей логике, но для надежности)
                                for func in parsed_functions_from_url {
                                    if !category_functions.iter().any(|existing_f| existing_f.name == func.name) {
                                        category_functions.push(func);
                                    }
                                }
                            } else {
                                info!("No functions found for category '{}' at URL {}", category_name, category_url);
                            }
                        }
                        Err(e) => warn!("Failed to parse category '{}' from URL {}: {}", category_name, category_url, e),
                    }
                } else {
                    info!("No URL available for category '{}' and not processed locally.", category_name);
                }
            }
            
            if !category_functions.is_empty() {
                collection.add_category(NativeCategory {
                    name: category_name.clone(),
                    description: Some(format!("Functions for the {} category", category_name.to_lowercase())),
                    functions: category_functions,
                });
            } else {
                warn!("No functions found for category '{}' after attempting all parsing methods.", category_name);
            }
        }
        
        info!("✅ Finished processing. Total functions parsed: {}. Total categories: {}.", 
              collection.total_functions(), collection.categories().len());
              
        // Кеширование результата
        info!("[CACHE_DEBUG] Attempting to cache collection. Total functions: {}. Total categories: {}", collection.total_functions(), collection.categories().len());
        match self.cache_collection(&collection).await {
            Ok(_) => info!("[CACHE_DEBUG] Successfully cached collection."),
            Err(e) => warn!("[CACHE_DEBUG] Failed to cache collection: {}", e),
        }
       
        Ok(collection)
    }

    async fn determine_categories_to_process(&self, base_url: &str, specified_categories_opt: Option<&Vec<String>>) -> Result<HashMap<String, Option<String>>> {
        let mut categories_map: HashMap<String, Option<String>> = HashMap::new();

        // 1. Если указан local_natives_path, сканируем его на категории (директории)
        if let Some(local_path) = &self.local_natives_path {
            info!("Scanning local path for categories: {}", local_path.display());
            if local_path.is_dir() {
                for entry in fs::read_dir(local_path)? {
                    let entry = entry?;
                    let path = entry.path();
                    if path.is_dir() {
                        if let Some(dir_name) = path.file_name().and_then(|n| n.to_str()) {
                            let category_name_upper = dir_name.to_uppercase();
                            if self.is_valid_category_name_heuristic(&category_name_upper) { // Используем более мягкую проверку для имен директорий
                                // Если категории указаны явно, добавляем только их
                                if let Some(specified_list) = specified_categories_opt {
                                    if specified_list.contains(&category_name_upper) || specified_list.contains(&dir_name.to_string()) {
                                        debug!("Found specified local category: {}", category_name_upper);
                                        categories_map.insert(category_name_upper.clone(), None); // URL не нужен, парсится локально
                                    }
                                } else {
                                    // Если категории не указаны, добавляем все найденные локально
                                    debug!("Found local category: {}", category_name_upper);
                                    categories_map.insert(category_name_upper.clone(), None);
                                }
                            }
                        }
                    }
                }
            } else {
                warn!("Specified local_natives_path is not a directory: {}", local_path.display());
            }
        }

        // 2. Если локально ничего не найдено ИЛИ local_natives_path не был указан, ИЛИ нужно дополнить URL-ами
        // (Например, если пользователь указал категории, которых нет локально)
        // Мы также должны получить URL для категорий, которые были найдены локально, если хотим смешанный режим
        // Но текущая логика приоритезирует локальный парсинг. Для простоты, если категории найдены локально, URL не ищем.

        if categories_map.is_empty() || specified_categories_opt.is_some() { // Если не нашли локально или есть спец. список, который может требовать URL
            info!("Attempting to discover categories from URL: {} (or use predefined if discovery fails)", base_url);
            let response = self.client.get(base_url).send().await.context("Failed to download FiveM docs main page for category discovery")?;
            let html_content = response.text().await.context("Failed to read response text from FiveM docs main page")?;
            let document = Html::parse_document(&html_content);

            let link_selector = Selector::parse("a[href*='/natives/']").map_err(|e| anyhow!("Invalid CSS selector for category links: {:?}", e))?;
            
            for element in document.select(&link_selector) {
                if let Some(href) = element.value().attr("href") {
                    let category_name_from_href = href.split('/').filter(|s| !s.is_empty()).last().unwrap_or("").to_uppercase();
                    let category_name_from_text = element.text().collect::<String>().trim().to_uppercase();
                    let category_name = if self.is_valid_category_name_heuristic(&category_name_from_text) { category_name_from_text } 
                                        else if self.is_valid_category_name_heuristic(&category_name_from_href) { category_name_from_href } 
                                        else { continue; };

                    if !category_name.is_empty() {
                        let full_url = self.resolve_url(base_url, href)?;
                        // Если есть указанный список, добавляем только то, что в нем
                        if let Some(specified_list) = specified_categories_opt {
                            if specified_list.contains(&category_name) {
                                // Добавляем/обновляем, только если еще не было добавлено из локального пути
                                categories_map.entry(category_name.clone()).or_insert_with(|| Some(full_url));
                            }
                        } else {
                             // Добавляем/обновляем, только если еще не было добавлено из локального пути
                            categories_map.entry(category_name.clone()).or_insert_with(|| Some(full_url));
                        }
                    }
                }
            }
        }

        // 3. Если ВООБЩЕ ничего не найдено (ни локально, ни по URL), И нет явно указанных категорий, используем предопределенный список
        if categories_map.is_empty() && specified_categories_opt.is_none() {
            warn!("No categories found locally or via URL discovery, using predefined list as a fallback.");
            self.get_known_categories(base_url).into_iter().for_each(|(name, url)| {
                categories_map.insert(name, Some(url));
            });
        } else if specified_categories_opt.is_some() && categories_map.is_empty() {
             warn!("Specified categories were not found locally or on the main URL page. Consider checking category names.");
        }
        
        info!("Final list of categories to process ({}): {:?}", categories_map.len(), categories_map.keys());
        Ok(categories_map)
    }
    
    async fn parse_category_from_html_url(&self, category_url: &str, category_name: &str) -> Result<Vec<NativeFunction>> {
        debug!("🌐 Attempting to parse category '{}' from HTML URL: {}", category_name, category_url);
        
        let response = self.client.get(category_url).send().await.context("Failed to download category page via URL")?;
        let _html_content = response.text().await.context("Failed to read HTML content from category page URL")?;
        // let document = Html::parse_document(&html_content);
        
        let functions = Vec::new();
        
        // HTML парсинг все еще является ЗАГЛУШКОЙ/ПРОБЛЕМНЫМ.
        // Он будет находить очень мало или ничего осмысленного до исправления селекторов.
        warn!("HTML parsing for category '{}' from URL {} is currently a stub and likely ineffective.", category_name, category_url);
        // Пример старой логики, которая, вероятно, не будет работать:
        // self.scan_html_text_for_function_patterns(&html_content, category_name, &mut functions)?;
        if functions.is_empty() {
            info!("No functions extracted via (stubbed) HTML URL parsing for category '{}' from {}. This is expected.", category_name, category_url);
        } else {
            info!("Found {} functions in category '{}' from URL {} (stubbed HTML parser)", functions.len(), category_name, category_url);
        }
        Ok(functions)
    }
    
    #[allow(dead_code)]
    pub fn parse_from_local_only(&self, path_to_natives_root: &str, specified_categories_opt: Option<&Vec<String>>) -> Result<NativeCollection> {
        info!("📁 Loading FiveM documentation exclusively from local path: {}", path_to_natives_root);
        let base_path = PathBuf::from(path_to_natives_root);
        let mut collection = NativeCollection::new();
        collection.metadata.source = format!("local_only:{}", path_to_natives_root);

        if !base_path.is_dir() {
            return Err(anyhow!("Local path {} is not a directory.", path_to_natives_root));
        }

        let categories_to_scan = if let Some(specified_list) = specified_categories_opt {
            // Если указан "ALL", загружаем все доступные директории
            if specified_list.len() == 1 && specified_list[0].to_uppercase() == "ALL" {
                fs::read_dir(&base_path)?
                    .filter_map(|entry_res| entry_res.ok())
                    .filter(|entry| entry.path().is_dir())
                    .filter_map(|entry| entry.file_name().into_string().ok())
                    .filter(|name| self.is_valid_category_name_heuristic(name))
                    .map(|name| name.to_uppercase())
                    .collect::<Vec<_>>()
            } else {
                specified_list.iter().map(|s| s.to_uppercase()).collect::<Vec<_>>()
            }
        } else {
            // Сканировать все поддиректории, если категории не указаны
            fs::read_dir(&base_path)?
                .filter_map(|entry_res| entry_res.ok())
                .filter(|entry| entry.path().is_dir())
                .filter_map(|entry| entry.file_name().into_string().ok())
                .map(|name| name.to_uppercase())
                .collect::<Vec<_>>()
        };
        info!("Target categories for local parsing: {:?}", categories_to_scan);

        for category_name_upper in categories_to_scan {
             if !self.is_valid_category_name_heuristic(&category_name_upper) && specified_categories_opt.is_none() {
                debug!("Skipping directory '{}' as it does not seem like a valid category name.", category_name_upper);
                continue;
            }
            let category_dir_path = base_path.join(&category_name_upper); // Имена папок могут быть в любом регистре, но мы привели к UPPER для сравнения
             // Для фактического пути используем исходное имя папки, если нужно, но join сработает и так
            // Здесь мы уже имеем category_name_upper, так что просто используем его для формирования пути.

            info!("Processing local category directory: {} (derived from '{}')", category_dir_path.display(), category_name_upper);
            match self.parse_category_from_local_markdown_dir(&category_dir_path, &category_name_upper) {
                Ok(parsed_functions) => {
                    if !parsed_functions.is_empty() {
                        collection.add_category(NativeCategory {
                            name: category_name_upper.clone(),
                            description: Some(format!("Functions for the {} category (local)", category_name_upper.to_lowercase())),
                            functions: parsed_functions,
                        });
                    }
                }
                Err(e) => warn!("Failed to parse local category '{}' from directory {}: {}", category_name_upper, category_dir_path.display(), e),
            }
        }
        info!("✅ Successfully parsed {} functions in {} categories from local path {}", 
              collection.total_functions(), collection.categories().len(), path_to_natives_root);
        Ok(collection)
    }
    
    fn is_valid_category_name_heuristic(&self, name: &str) -> bool {
        if name.is_empty() || name.starts_with('.') || name.to_lowercase() == "readme" || name.to_lowercase() == "shared" || name.to_lowercase() == "gameNatives" {
            return false;
        }
        // Простое правило: не слишком короткое, состоит из букв и цифр, может содержать подчеркивания.
        name.len() >= 2 && name.chars().all(|c| c.is_ascii_alphanumeric() || c == '_' || c == '-')
    }
    
    fn get_known_categories(&self, base_url: &str) -> HashMap<String, String> {
        let mut categories = HashMap::new();
        let base = if base_url.ends_with('/') { base_url.to_string() } else { format!("{}/", base_url) };
        
        let known_cats = [
            "PLAYER", "VEHICLE", "PED", "WEAPON", "ENTITY", "OBJECT", "GRAPHICS", "HUD", "CAM", 
            "STREAMING", "DECORATOR", "AUDIO", "FIRE", "WATER", "PHYSICS", "BRAIN", "TASK", 
            "PATHFIND", "AI", "NETWORK", "SCRIPT", "EVENT", "MISC", "ZONE", "INTERIOR", 
            "CUTSCENE", "REPLAY", "CLOCK", "STATS", "MONEY", "SHAPETEST", "FILES", "DATAFILE", 
            "DLC", "MOBILE", "NETSHOPPING", "SOCIALCLUB", "LOCALIZATION", "LOADINGSCREEN", 
            "SYSTEM", "RECORDING", "PAD", "CONTROLS", "ITEMSET", "APP", "EMAIL", "GTAONLINE", 
            "BLIP", "GARAGE", "SHOP", "TRAIN", "AIRCRAFT", "BOAT", "SUBMARINE", "CASINO", 
            "NIGHTCLUB", "ARENA", "ANIMPOSTFX", "ANIMSCENE", "ATTRIBUTE", "COMPANION", 
            "INVENTORY", "LAWBALANCE", "MAGAZINE", "MINIGAME", "PHOTO", "POPULATION", 
            "SCRIPT_EVENT", "TELEMETRY", "UIFEED", "UIINVENTORY", "WEAPON_WHEEL",
        ];
        
        for category_name_str in known_cats {
            let category_name = category_name_str.to_string();
            let url_path_segment = category_name.to_lowercase();
            categories.insert(
                category_name.clone(),
                format!("{}{}", base, url_path_segment)
            );
        }
        
        debug!("Initialized with {} known native categories for fallback URL parsing.", categories.len());
        categories
    }

    fn resolve_url(&self, base: &str, href: &str) -> Result<String> {
        if href.starts_with("http://") || href.starts_with("https://") {
            Ok(href.to_string())
        } else {
            let base_url_obj = Url::parse(base).with_context(|| format!("Invalid base URL for resolving: '{}'", base))?;
            let resolved = base_url_obj.join(href).with_context(|| format!("Failed to join base URL '{}' with href '{}'", base, href))?;
            Ok(resolved.to_string())
        }
    }

    async fn cache_collection(&self, collection: &NativeCollection) -> Result<()> {
        if !self.cache_dir.exists() {
            info!("[CACHE_DEBUG] Cache directory {} does not exist, creating.", self.cache_dir.display());
            fs::create_dir_all(&self.cache_dir).context(format!("Failed to create cache directory {:?}", self.cache_dir))?;
        } else {
            info!("[CACHE_DEBUG] Cache directory {} already exists.", self.cache_dir.display());
        }
        let cache_file_path = self.cache_dir.join("natives_cache.json");
        info!("[CACHE_DEBUG] Attempting to write cache to: {}", cache_file_path.display());
        
        let serialized_collection = serde_json::to_string_pretty(collection).context("Failed to serialize collection for cache")?;
        
        fs::write(&cache_file_path, serialized_collection)
            .context(format!("Failed to write cache file at {:?}", cache_file_path))?;
            
        info!("💾 Cached {} native functions to: {}", collection.total_functions(), cache_file_path.display());
        Ok(())
    }

    #[allow(dead_code)]
    pub fn load_from_cache(&self) -> Result<Option<NativeCollection>> {
        let cache_file = self.cache_dir.join("natives_cache.json");
        if !cache_file.exists() {
            debug!("Cache file not found at {}", cache_file.display());
            return Ok(None);
        }
        let content = fs::read_to_string(&cache_file).context("Failed to read native collection cache file")?;
        let collection: NativeCollection = serde_json::from_str(&content).context("Failed to parse cached native collection")?;
        info!("📥 Loaded {} functions from cache: {}", collection.total_functions(), cache_file.display());
        Ok(Some(collection))
    }

    pub async fn update_cache(&self, force: bool) -> Result<()> {
        let cache_file = self.cache_dir.join("natives_cache.json");
        if !force && cache_file.exists() {
            if let Ok(metadata) = fs::metadata(&cache_file) {
                if let Ok(modified) = metadata.modified() {
                    if let Ok(age) = std::time::SystemTime::now().duration_since(modified) {
                        if age.as_secs() < 24 * 60 * 60 { // 24 часа
                            info!("📅 Cache is fresh (updated {:.1} hours ago), skipping update unless forced.", age.as_secs_f32() / 3600.0);
                            return Ok(());
                        }
                    }
                }
            }
        }
        info!("🔄 Updating FiveM native documentation cache (force: {})...", force);
        let collection = self.parse_from_url("https://docs.fivem.net/natives/", None).await?;
        // No need to call self.cache_collection here as parse_from_url already does it if data is found
        if collection.total_functions() > 0 {
            info!("✅ Documentation cache updated successfully via parse_from_url.");
        } else {
            warn!("Cache update via parse_from_url resulted in no functions. Cache might not be updated.");
        }
        Ok(())
    }

    // Новая функция для парсинга одной нативной функции из содержимого .md файла
    fn parse_native_from_markdown_content(&self, md_content: &str, default_category: &str) -> Result<Option<NativeFunction>> {
        // --- Sanitization step --------------------------------------------------------------
        // Некоторые Markdown-файлы FiveM содержат YAML-директивы вида
        // `%YAML_directive "returns"`, которые вызывают предупреждения у YAML-парсера
        // сторонних библиотек. Эти директивы нам не нужны – удаляем их до дальнейшего
        // разбора, чтобы избежать лишнего шума в логах.
        let cleaned_md: String = md_content
            .lines()
            .filter(|l| !l.trim_start().starts_with("%YAML_directive"))
            .collect::<Vec<_>>()
            .join("\n");

        // ------------------------------------------------------------------------------------
 
        let front_matter_re = Regex::new(r"(?s)---\s*ns:\s*(\w+)\s*---").unwrap();
        let name_re = Regex::new(r"(?m)^##\s*([\w_]+)\s*$").unwrap();
        // Обновленный signature_re: пропускаем cs_type перед типом возврата
        let signature_re = Regex::new(&format!(
            "{}{}{}{}{}{}{}",
            r#"(?s)```c(?:pp)?\s*"#,
            r#"(?://\s*(0x[0-9a-zA-Z_]+)(?:\s*(0x[0-9a-zA-Z_]+))?)?\s*"#,
            r#"(?:(?:/\*[\s\S]*?\*/|//.*?)\s*)*"#,
            r#"(?:(?:cs_type\s*\([^)]*\)\s*)?)?"#,
            // Группа для типа возвращаемого значения (группа 3): разрешаем скобки [] и их отсутствие, любые пробелы
            r#"((?:const\s+)?[\w<>.\*]+(?:\s*\[\s*\d*\s*\])*)"#,
            r#"\s+([\w_]+)\s*"#,
            r#"\s*\(([^)]*?)\)[^`]*?```"# 
        )).unwrap();
        let description_re = Regex::new(r"(?s)```[^`]*```\s+(.*?)(?:\s+(?:##|---|\*\*))|(?s)```[^`]*```\s+(.*?)\z").unwrap();

        let category = front_matter_re.captures(&cleaned_md)
            .and_then(|cap| cap.get(1).map(|m| m.as_str().to_uppercase()))
            .unwrap_or_else(|| default_category.to_uppercase());

        let name_from_h2 = match name_re.captures(&cleaned_md) {
            Some(cap) => cap.get(1).map_or("", |m| m.as_str()).to_string(),
            None => {
                debug!("No H2 ## name found in markdown for category {}", category);
                return Ok(None);
            }
        };

        // Отладка
        // debug!("MD_CONTENT for '{}': |||{}|||", name_from_h2, cleaned_md);
        println!("MD_CONTENT for '{}': |||{}|||", name_from_h2, cleaned_md);
        let sig_caps_option = signature_re.captures(&cleaned_md);
        // debug!("SIG_CAPS_OPTION for '{}': {:?}", name_from_h2, sig_caps_option.is_some());
        println!("SIG_CAPS_OPTION for '{}': {:?}", name_from_h2, sig_caps_option.is_some());

        let sig_caps = match sig_caps_option {
            Some(cap) => cap,
            None => {
                if !name_from_h2.is_empty() {
                    warn!("Could not parse C signature for H2 '{}' in category {}", name_from_h2, category);
                } else {
                    // debug!("Skipping signature block without H2 name and unparsable/missing signature in category {}", category);
                    println!("Skipping signature block without H2 name and unparsable/missing signature in category {}", category);
                }
                return Ok(None);
            }
        };

        let first_hash = sig_caps.get(1).map(|m| m.as_str().trim().to_string());
        let second_hash = sig_caps.get(2).map(|m| m.as_str().trim().to_string());
        let return_type_str = sig_caps.get(3).map_or("void", |m| m.as_str()).trim().to_string();
        let name_in_signature = sig_caps.get(4).map_or("", |m| m.as_str()).trim().to_string();
        let params_str = sig_caps.get(5).map_or("", |m| m.as_str()).trim().to_string();

        let final_name;
        if name_in_signature.is_empty() {
            if name_from_h2.is_empty() {
                debug!("Could not determine function name (H2 and signature are empty) in category {}", category);
                return Ok(None)
            }
            final_name = name_from_h2.clone(); // Использовать имя из H2 если в сигнатуре нет
        } else {
            final_name = name_in_signature.clone(); // По умолчанию использовать имя из сигнатуры
            if !name_from_h2.is_empty() && name_from_h2.to_ascii_lowercase() != name_in_signature.to_ascii_lowercase() {
                warn!(
                    "Mismatched function name casing or content: H2='{}', Signature='{}' in category {}. Using name from signature.",
                    name_from_h2, name_in_signature, category
                );
            }
        }

        if final_name.is_empty() {
            debug!("Could not determine function name for a signature block in category {}", category);
            return Ok(None)
        }

        debug!("Processing native: {} in category {}", final_name, category);

        // Добавляем новую логику разбора возвращаемого типа здесь
        debug!("Attempting to parse return type: '{}' for function '{}'", return_type_str, final_name);

        let final_return_type: NativeType;
        let mut return_type_element_str_for_parsing = return_type_str.clone();
        let mut return_array_size_info: Option<crate::native_types::ArraySizeInfo> = None;
        let mut is_return_array = false;

        if return_type_element_str_for_parsing.ends_with(']') {
            if let Some(bracket_start_pos) = return_type_element_str_for_parsing.rfind('[') {
                let content_in_brackets = return_type_element_str_for_parsing[bracket_start_pos + 1 .. return_type_element_str_for_parsing.len() - 1].trim().to_string();
                return_type_element_str_for_parsing = return_type_element_str_for_parsing[..bracket_start_pos].trim_end().to_string();
                is_return_array = true;

                if content_in_brackets.is_empty() {
                    if return_type_element_str_for_parsing == "char*" || return_type_element_str_for_parsing == "const char*" {
                         return_array_size_info = Some(crate::native_types::ArraySizeInfo::NullTerminated);
                    } else {
                         return_array_size_info = Some(crate::native_types::ArraySizeInfo::Infer);
                    }
                } else if let Ok(known_size) = content_in_brackets.parse::<usize>() {
                    return_array_size_info = Some(crate::native_types::ArraySizeInfo::Known(known_size));
                } else {
                    warn!("Return type '{}' for '{}' has non-numeric content '{}' in brackets. Inferring size.", return_type_str, final_name, content_in_brackets);
                    return_array_size_info = Some(crate::native_types::ArraySizeInfo::Infer);
                }
            }
        } else if return_type_element_str_for_parsing.ends_with("**") {
            if return_type_element_str_for_parsing == "const char**" || return_type_element_str_for_parsing == "char**" {
                is_return_array = true;
                return_type_element_str_for_parsing = return_type_element_str_for_parsing[..return_type_element_str_for_parsing.len()-1].to_string();
                return_array_size_info = Some(crate::native_types::ArraySizeInfo::NullTerminated);
                debug!("Interpreting return type '{}' for '{}' as a null-terminated array of strings. Element type for parsing: '{}'", return_type_str, final_name, return_type_element_str_for_parsing);
            }
        }

        let parsed_element_type = NativeType::from_fivem_type(&return_type_element_str_for_parsing);

        if is_return_array {
            if parsed_element_type == NativeType::Char && return_array_size_info.as_ref().map_or(false, |s| s == &crate::native_types::ArraySizeInfo::Infer) {
                 return_array_size_info = Some(crate::native_types::ArraySizeInfo::NullTerminated);
                 debug!("Return type for '{}' is char array with inferred size, assuming NullTerminated string-like buffer.", final_name);
            }

            final_return_type = NativeType::Array {
                element_type: Box::new(parsed_element_type.clone()),
                size_info: return_array_size_info.clone(),
            };
            // debug!("Parsed return type for '{}' as ARRAY: Element: {:?}, SizeInfo: {:?}. Original str: '{}', Element str for parsing: '{}'", final_name, parsed_element_type, return_array_size_info, return_type_str, return_type_element_str_for_parsing);
            println!("Parsed return type for '{}' as ARRAY: Element: {:?}, SizeInfo: {:?}. Original str: '{}', Element str for parsing: '{}'", final_name, parsed_element_type, return_array_size_info, return_type_str, return_type_element_str_for_parsing);
        } else {
            final_return_type = parsed_element_type.clone();
            // Логирование для не-массивных типов, если нужно
            // debug!("Parsed return type for '{}' as SCALAR: {:?}. Original str: '{}'", final_name, final_return_type, return_type_str);
            println!("Parsed return type for '{}' as SCALAR: {:?}. Original str: '{}'", final_name, final_return_type, return_type_str);
        }

        let parameters = parse_parameters_from_md_signature(&params_str, &final_name)?;
        
        let description = description_re.captures(&cleaned_md)
            .and_then(|cap| cap.get(1).or_else(|| cap.get(2)).map(|m| m.as_str().trim().to_string()))
            .filter(|s| !s.is_empty());

        let mut native_fn = NativeFunction::new(final_name.clone(), category)
            .with_return_type(final_return_type.clone()); // Используем final_return_type
        native_fn.parameters = parameters;

        // Применяем оверрайды из конфигурации
        if let Some(override_config) = self.native_overrides.overrides.iter().find(|ov| ov.name == final_name) {
            if let Some(ref out_param_name) = override_config.return_array_length_out_param {
                native_fn.return_array_length_out_param = Some(out_param_name.clone());
                debug!("Applied override for '{}': set return_array_length_out_param to '{}'", final_name, out_param_name);
            }

            // Применяем override для return_type
            if let Some(ref return_type_override_str) = override_config.return_type_override {
                let overridden_return_type = NativeType::from_fivem_type(return_type_override_str);
                // Это переопределение для Rust-типа, возвращаемого из unsafe { invoke_raw! ... }
                // Оно отличается от typescript_return_type_override
                debug!("Applied override for '{}': set original return_type to '{:?}' (was '{:?}')", final_name, overridden_return_type, native_fn.return_type);
                native_fn.return_type = overridden_return_type; 
            }
            
            // Применяем Rust-специфичные оверрайды уровня функции
            if let Some(mark_unsafe) = override_config.mark_safe_wrapper_unsafe {
                native_fn.rust_mark_safe_wrapper_unsafe = Some(mark_unsafe);
                debug!("Applied override for '{}': set rust_mark_safe_wrapper_unsafe to '{}'", final_name, mark_unsafe);
            }
            if let Some(ref prologue) = override_config.prologue_code {
                native_fn.rust_prologue_code = Some(prologue.clone());
                debug!("Applied override for '{}': set rust_prologue_code", final_name);
            }
            if let Some(ref epilogue) = override_config.epilogue_code {
                native_fn.rust_epilogue_code = Some(epilogue.clone());
                debug!("Applied override for '{}': set rust_epilogue_code", final_name);
            }
            // >>> ДОБАВИТЬ ЭТОТ БЛОК
            if let Some(ref rust_return_override_str) = override_config.return_type_override { // Используем существующее поле return_type_override, т.к. оно было для Rust
                native_fn.rust_return_type_override = Some(rust_return_override_str.clone());
                debug!("Applied override for '{}': set rust_return_type_override to '{}'", final_name, rust_return_override_str);
            }
            // <<< КОНЕЦ ДОБАВЛЕННОГО БЛОКА

            // Применяем TypeScript-специфичные оверрайды уровня функции
            if let Some(ref ts_name) = override_config.typescript_name_override {
                native_fn.typescript_name_override = Some(ts_name.clone());
                debug!("Applied typescript_name_override for '{}': '{}'", final_name, ts_name);
            }
            if let Some(ref ts_return_type_override) = override_config.typescript_return_type_override {
                native_fn.typescript_return_type_override = Some(ts_return_type_override.clone());
                debug!("Applied typescript_return_type_override for '{}': '{}'", final_name, ts_return_type_override);
            }

            // Применяем override для типов параметров
            if let Some(ref param_overrides_map) = override_config.parameter_type_overrides {
                for param_to_update in native_fn.parameters.iter_mut() {
                    if let Some(param_type_override_str) = param_overrides_map.get(&param_to_update.name) {
                        let overridden_param_type = NativeType::from_fivem_type(param_type_override_str);
                        if !matches!(overridden_param_type, NativeType::Any(_)) || param_type_override_str.eq_ignore_ascii_case("any") {
                            trace!("Applying override for parameter '{}' in function: {}. Old: {:?}, New: {:?}, From String: '{}'", 
                                   param_to_update.name, final_name, param_to_update.param_type, overridden_param_type, param_type_override_str);
                            param_to_update.param_type = overridden_param_type;
                        } else {
                            warn!("Failed to parse parameter_type_override '{}' for param '{}' in function {}. Kept original: {:?}. Parsed as: {:?}.",
                                   param_type_override_str, param_to_update.name, final_name, param_to_update.param_type, overridden_param_type);
                        }
                    }
                }
            }

            // Применяем новые Rust-специфичные оверрайды для параметров
            if let Some(ref param_rust_overrides_vec) = override_config.parameter_rust_overrides {
                for param_to_update in native_fn.parameters.iter_mut() {
                    if let Some(rust_override) = param_rust_overrides_vec.iter().find(|pro| pro.original_name == param_to_update.name) {
                        if let Some(ref new_name) = rust_override.new_rust_name {
                            param_to_update.rust_new_name = Some(new_name.clone());
                            debug!("Applied rust_new_name for param '{}' in '{}': '{}'", param_to_update.name, final_name, new_name);
                        }
                        if let Some(ref type_override_str) = rust_override.type_override {
                            // Здесь мы просто сохраняем строку, сам тип NativeType::from_fivem_type будет применяться в rust_generator
                            param_to_update.rust_type_override = Some(type_override_str.clone());
                            debug!("Stored rust_type_override string for param '{}' in '{}': '{}'", param_to_update.name, final_name, type_override_str);
                        }
                        if let Some(ref hint) = rust_override.any_type_hint {
                            param_to_update.rust_any_type_hint = Some(hint.clone());
                            debug!("Applied rust_any_type_hint for param '{}' in '{}': '{}'", param_to_update.name, final_name, hint);
                        }
                        if let Some(ref transform) = rust_override.transform_input_with {
                            param_to_update.rust_transform_input_with = Some(transform.clone());
                            debug!("Applied rust_transform_input_with for param '{}' in '{}': '{}'", param_to_update.name, final_name, transform);
                        }
                        if let Some(ref default_val) = rust_override.default_value_for_optional {
                            param_to_update.rust_default_value_for_optional = Some(default_val.clone());
                            debug!("Applied rust_default_value_for_optional for param '{}' in '{}': '{}'", param_to_update.name, final_name, default_val);
                        }
                    }
                }
            }

            // Apply TypeScript specific overrides for parameters
            if let Some(ref param_ts_overrides) = override_config.parameter_typescript_overrides {
                for param_to_update in native_fn.parameters.iter_mut() {
                    if let Some(ts_override) = param_ts_overrides.iter().find(|po| po.original_name == param_to_update.name) {
                        if let Some(ref name_override) = ts_override.typescript_name_override {
                            param_to_update.typescript_name_override = Some(name_override.clone());
                            debug!("Applied typescript_name_override for param '{}' in '{}': '{}'", param_to_update.name, final_name, name_override);
                        }
                        if let Some(ref type_override) = ts_override.typescript_type_override {
                            param_to_update.typescript_type_override = Some(type_override.clone());
                            debug!("Applied typescript_type_override for param '{}' in '{}': '{}'", param_to_update.name, final_name, type_override);
                        }
                    }
                }
            }
        }

        if let Some(desc) = description {
            native_fn = native_fn.with_description(desc);
        }
        
        if let Some(h) = first_hash {
            native_fn = native_fn.with_hash(h); // Main hash
        }
        if let Some(h2_val) = second_hash {
            native_fn.raw_data.insert("secondary_hash".to_string(), h2_val);
        }

        Ok(Some(native_fn))
    }

    // Новая функция для парсинга всех .md файлов в директории категории
    fn parse_category_from_local_markdown_dir(&self, category_dir_path: &Path, category_name: &str) -> Result<Vec<NativeFunction>> {
        let mut functions = Vec::new();
        info!("📖 Parsing local markdown files for category '{}' in directory: {}", category_name, category_dir_path.display());

        if !category_dir_path.exists() { // Проверка существования
            warn!("Category directory NOT FOUND: {}", category_dir_path.display());
            return Ok(functions);
        }
        if !category_dir_path.is_dir() { // Проверка, что это директория
            warn!("Path is NOT a directory: {}", category_dir_path.display());
            return Ok(functions);
        }
        
        debug!("Attempting to read directory: {}", category_dir_path.display()); // DEBUG
        let dir_entries = match fs::read_dir(category_dir_path) {
            Ok(entries) => entries,
            Err(e) => {
                warn!("Failed to read directory {}: {}", category_dir_path.display(), e); // DEBUG
                return Err(e.into()); // Возвращаем ошибку, если не можем прочитать директорию
            }
        };

        for entry_result in dir_entries {
            match entry_result {
                Ok(entry) => {
                    let path = entry.path();
                    debug!("Found entry: {:?}, is_file: {}", path, path.is_file()); // DEBUG
                    if path.is_file() {
                        if let Some(ext) = path.extension().and_then(|s| s.to_str()) {
                            debug!("File extension: {}", ext); // DEBUG
                            if ext == "md" {
                                debug!("Processing markdown file: {}", path.display()); // DEBUG
                                match fs::read_to_string(&path) {
                                    Ok(md_content) => {
                                        match self.parse_native_from_markdown_content(&md_content, category_name) {
                                            Ok(Some(func)) => {
                                                debug!("Successfully parsed function '{}' from {}", func.name, path.display()); // DEBUG
                                                functions.push(func);
                                            }
                                            Ok(None) => {
                                                warn!("No function parsed (Ok(None)) from file: {}", path.display()); // DEBUG
                                            }
                                            Err(e) => {
                                                warn!("Error parsing native from markdown file {}: {}", path.display(), e); // DEBUG
                                            }
                                        }
                                    }
                                    Err(e) => {
                                        warn!("Failed to read markdown file {}: {}", path.display(), e); // DEBUG
                                    }
                                }
                            } else {
                                trace!("Skipping file with non-md extension: {}", path.display()); // TRACE
                            }
                        } else {
                            trace!("Skipping file with no extension: {}", path.display()); // TRACE
                        }
                    } else {
                        trace!("Skipping non-file entry: {}", path.display()); // TRACE
                    }
                }
                Err(e) => {
                     warn!("Error reading directory entry in {}: {}", category_dir_path.display(), e); // DEBUG
                }
            }
        }
        
        if functions.is_empty() {
            warn!("No functions successfully parsed from local markdown files in category: {}", category_name); // Этот лог уже был
        } else {
            info!("Successfully parsed {} functions from local markdown files in category: {}", functions.len(), category_name);
        }
        Ok(functions)
    }

    fn detect_array_size_param(&self, function: &mut NativeFunction) {
        // Эвристика для автоматического определения параметра размера
        let size_params = ["size", "count", "length", "arraySize"];
        
        let size_param_names: Vec<String> = function
            .parameters
            .iter()
            .filter(|p| size_params.iter().any(|sp| p.name.contains(sp)))
            .map(|p| p.name.clone())
            .collect();

        for array_param in function.parameters.iter_mut().filter(|p| matches!(p.param_type, NativeType::Array { size_info: None, .. })) {
            if let NativeType::Array { size_info: ref mut si, .. } = &mut array_param.param_type {
                if si.is_none() {
                    if let Some(size_name) = size_param_names.first() {
                        *si = Some(crate::native_types::ArraySizeInfo::Dynamic { size_param_name: size_name.clone() });
                    }
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    // use tempfile; // Не используется

    fn create_mock_md_file(dir: &Path, filename: &str, content: &str) -> PathBuf {
        // Убедимся, что директория существует перед записью файла
        if !dir.exists() {
            fs::create_dir_all(dir).unwrap();
        }
        let file_path = dir.join(filename);
        fs::write(&file_path, content).unwrap();
        file_path
    }

    // Вспомогательная функция для создания парсера по умолчанию для тестов
    fn default_parser() -> FiveMDocParser {
        FiveMDocParser::new(None, None) // Без локального пути и без конфигов по умолчанию
    }

    #[test]
    fn test_parse_native_from_markdown_content_basic() {
        let parser = default_parser();
        let md_content = "---\nns: PLAYER\n---\n## GET_PLAYER_NAME\n```cpp\n// 0x43A66C31C68491C0\nconst char* GET_PLAYER_NAME(Player player)\n```\nThis is the description for GET_PLAYER_NAME.";
        let native = parser.parse_native_from_markdown_content(md_content, "PLAYER").unwrap().unwrap();
        assert_eq!(native.name, "GET_PLAYER_NAME");
        assert_eq!(native.return_type, NativeType::String);
        assert_eq!(native.parameters.len(), 1);
        assert_eq!(native.parameters[0].name, "player");
        assert_eq!(native.parameters[0].param_type, NativeType::Player);
        assert_eq!(native.description, Some("This is the description for GET_PLAYER_NAME.".to_string()));
        assert_eq!(native.hash, Some("0x43A66C31C68491C0".to_string()));
        assert_eq!(native.category, "PLAYER");
    }
    
    #[test]
    fn test_parse_native_from_markdown_content_no_params_void_return() {
        let parser = default_parser();
        let md_content = "---\nns: SYSTEM\n---\n## WAIT\n```cpp\n// 0xBD5452442A039340\nvoid WAIT(int ms)\n```\nDescription for WAIT.";
        let native = parser.parse_native_from_markdown_content(md_content, "SYSTEM").unwrap().unwrap();
        assert_eq!(native.name, "WAIT");
        assert_eq!(native.return_type, NativeType::Void);
        assert_eq!(native.parameters.len(), 1);
        assert_eq!(native.parameters[0].name, "ms");
        assert_eq!(native.parameters[0].param_type, NativeType::Int);
        assert_eq!(native.hash, Some("0xBD5452442A039340".to_string()));
    }

    #[test]
    fn test_parse_parameters_from_md_signature_advanced() {
        let signature = "BOOL p0, int p1, float p2, const char* p3, Ped p4, float arr[3], void* p6";
        let params = parse_parameters_from_md_signature(signature, "test_func_advanced").unwrap();
        assert_eq!(params.len(), 7);
        assert_eq!(params[0].param_type, NativeType::Bool);
        assert_eq!(params[1].param_type, NativeType::Int);
        assert_eq!(params[2].param_type, NativeType::Float);
        assert_eq!(params[3].param_type, NativeType::String);
        assert_eq!(params[4].param_type, NativeType::Ped);
        
        // Проверка для params[5] (float arr[3])
        assert_eq!(params[5].name, "arr");
        match &params[5].param_type {
            NativeType::Array { element_type, size_info } => {
                assert_eq!(**element_type, NativeType::Float);
                assert_eq!(*size_info, Some(crate::native_types::ArraySizeInfo::Known(3)));
            }
            _ => panic!("Expected NativeType::Array for params[5], got {:?}", params[5].param_type),
        }

        // Проверка для params[6] (void* p6)
        assert_eq!(params[6].name, "p6");
        match &params[6].param_type {
            NativeType::Pointer(inner_type) => {
                 // NativeType::from_fivem_type("void") is NativeType::Void
                 // So Pointer(Void) for "void*"
                assert_eq!(**inner_type, NativeType::Void); 
            }
            _ => panic!("Expected NativeType::Pointer for params[6], got {:?}", params[6].param_type),
        }
    }

    #[test]
    fn test_parse_parameters_from_md_signature_with_sized_array() {
        let params_str = "float color[3], int id";
        let params = parse_parameters_from_md_signature(params_str, "test_func_sized_array").unwrap();
        assert_eq!(params.len(), 2);
        assert_eq!(params[0].name, "color");
        match &params[0].param_type {
            NativeType::Array { element_type, size_info } => {
                assert_eq!(**element_type, NativeType::Float);
                assert_eq!(*size_info, Some(crate::native_types::ArraySizeInfo::Known(3)));
            }
            _ => panic!("Expected NativeType::Array for color, got {:?}", params[0].param_type),
        }
        assert_eq!(params[1].name, "id");
        assert_eq!(params[1].param_type, NativeType::Int);

        let params_str_single = "char name[64]";
        let params_single = parse_parameters_from_md_signature(params_str_single, "test_func_char_array").unwrap();
        assert_eq!(params_single.len(), 1);
        assert_eq!(params_single[0].name, "name");
        match &params_single[0].param_type {
            NativeType::Array { element_type, size_info } => {
                assert_eq!(**element_type, NativeType::Char);
                assert_eq!(*size_info, Some(crate::native_types::ArraySizeInfo::Known(64)));
            }
            _ => panic!("Expected NativeType::Array for name, got {:?}", params_single[0].param_type),
        }
    }

    #[test]
    fn test_parse_parameters_from_md_signature_array_on_type() {
        let params_str = "const char* names[], int count";
        let params = parse_parameters_from_md_signature(params_str, "test_array_on_type").unwrap();
        assert_eq!(params.len(), 2);
        assert_eq!(params[0].name, "names");
        match &params[0].param_type {
            NativeType::Array { element_type, size_info } => {
                assert_eq!(**element_type, NativeType::String); 
                assert_eq!(*size_info, Some(crate::native_types::ArraySizeInfo::NullTerminated)); // Используется полный путь
            }
            _ => panic!("Expected NativeType::Array for names, got {:?}", params[0].param_type),
        }
        assert_eq!(params[1].name, "count");
        assert_eq!(params[1].param_type, NativeType::Int);

        let _params_str_2 = "float positions[][3]"; // Заменено на _params_str_2
        
        let params_str_3 = "Ped peds[MAX_PEDS]";
        let params3 = parse_parameters_from_md_signature(params_str_3, "test_peds_array").unwrap();
        assert_eq!(params3.len(), 1);
        assert_eq!(params3[0].name, "peds");
        match &params3[0].param_type {
            NativeType::Array { element_type, size_info } => {
                assert_eq!(**element_type, NativeType::Ped);
                assert_eq!(*size_info, Some(crate::native_types::ArraySizeInfo::Dynamic{size_param_name: "MAX_PEDS".to_string()}));
            }
            _ => panic!("Expected NativeType::Array for peds, got {:?}", params3[0].param_type),
        }

        let params_str_4 = "char text_buffer[]"; 
        let params4 = parse_parameters_from_md_signature(params_str_4, "test_char_buffer").unwrap();
        assert_eq!(params4.len(), 1);
        assert_eq!(params4[0].name, "text_buffer");
        match &params4[0].param_type {
            NativeType::Array { element_type, size_info } => {
                assert_eq!(**element_type, NativeType::Char);
                assert_eq!(*size_info, Some(crate::native_types::ArraySizeInfo::NullTerminated));
            }
            _ => panic!("Expected NativeType::Array for text_buffer, got {:?}", params4[0].param_type),
        }
    }

    #[test]
    fn test_parse_native_from_markdown_content_sized_array_return() {
        let parser = default_parser();
        let md_content = "---\nns: TEST\n---\n## GET_ARRAY\n```c\n// 0xABCDEF\nint[3] GET_ARRAY(Player player)\n```\nDescription for get_array.";
        let native_fn_result = parser.parse_native_from_markdown_content(md_content, "TEST");
        println!("[DEBUG] native_fn_result: {:?}", native_fn_result);
        let native_fn = native_fn_result.unwrap().unwrap();
        assert_eq!(native_fn.name, "GET_ARRAY");
        
        match native_fn.return_type {
            NativeType::Array { ref element_type, ref size_info } => {
                assert_eq!(**element_type, NativeType::Int);
                assert_eq!(*size_info, Some(crate::native_types::ArraySizeInfo::Known(3)));
            }
            _ => panic!("Return type is not NativeType::Array {{ Known(3) }} as expected, got {:?}", native_fn.return_type),
        }

        assert_eq!(native_fn.parameters.len(), 1);
        assert_eq!(native_fn.parameters[0].name, "player");
        assert_eq!(native_fn.parameters[0].param_type, NativeType::Player);
        assert_eq!(native_fn.hash, Some("0xABCDEF".to_string()));
    }

    #[test]
    fn test_parse_native_from_markdown_content_array_return_types() {
        let parser = default_parser();

        // Test case 1: const char*[]
        let md_content1 = "---\nns: TEST\n---\n## GET_STRING_ARRAY\n```cpp\nconst char*[] GET_STRING_ARRAY()\n```";
        let native1_result = parser.parse_native_from_markdown_content(md_content1, "TEST");
        println!("[DEBUG] native1_result: {:?}", native1_result);
        let native1 = native1_result.unwrap().unwrap();
        assert_eq!(native1.name, "GET_STRING_ARRAY");
        if let NativeType::Array { element_type, size_info } = native1.return_type {
            assert_eq!(*element_type, NativeType::String); // NativeType::from_fivem_type("const char*") должен дать String
            assert_eq!(size_info, Some(crate::native_types::ArraySizeInfo::NullTerminated));
        } else {
            panic!("Expected Array return type for GET_STRING_ARRAY, got {:?}", native1.return_type);
        }

        // Test case 2: Vector3[]
        let md_content2 = "---\nns: TEST\n---\n## GET_VECTOR_ARRAY\n```cpp\nVector3[] GET_VECTOR_ARRAY()\n```";
        let native2 = parser.parse_native_from_markdown_content(md_content2, "TEST").unwrap().unwrap();
        assert_eq!(native2.name, "GET_VECTOR_ARRAY");
        if let NativeType::Array { element_type, size_info } = native2.return_type {
            assert_eq!(*element_type, NativeType::Vector3);
            assert_eq!(size_info, Some(crate::native_types::ArraySizeInfo::Infer));
        } else {
            panic!("Expected Array return type for GET_VECTOR_ARRAY, got {:?}", native2.return_type);
        }

        // Test case 3: Entity[10]
        let md_content3 = "---\nns: TEST\n---\n## GET_ENTITY_FIXED_ARRAY\n```cpp\nEntity[10] GET_ENTITY_FIXED_ARRAY()\n```";
        let native3 = parser.parse_native_from_markdown_content(md_content3, "TEST").unwrap().unwrap();
        assert_eq!(native3.name, "GET_ENTITY_FIXED_ARRAY");
        if let NativeType::Array { element_type, size_info } = native3.return_type {
            assert_eq!(*element_type, NativeType::Entity);
            assert_eq!(size_info, Some(crate::native_types::ArraySizeInfo::Known(10)));
        } else {
            panic!("Expected Array return type for GET_ENTITY_FIXED_ARRAY, got {:?}", native3.return_type);
        }
        
        // Test case 4: const char**
        let md_content4 = "---\nns: TEST\n---\n## GET_STRING_DOUBLE_POINTER_ARRAY\n```cpp\nconst char** GET_STRING_DOUBLE_POINTER_ARRAY()\n```";
        let native4 = parser.parse_native_from_markdown_content(md_content4, "TEST").unwrap().unwrap();
        assert_eq!(native4.name, "GET_STRING_DOUBLE_POINTER_ARRAY");
        if let NativeType::Array { element_type, size_info } = native4.return_type {
            assert_eq!(*element_type, NativeType::String);
            assert_eq!(size_info, Some(crate::native_types::ArraySizeInfo::NullTerminated));
        } else {
            panic!("Expected Array return type for GET_STRING_DOUBLE_POINTER_ARRAY, got {:?}", native4.return_type);
        }

        // Test case 5: char[] (empty brackets, should be NullTerminated char array)
        let md_content5 = "---\nns: TEST\n---\n## GET_CHAR_ARRAY_IMPLICIT_STRING\n```cpp\nchar[] GET_CHAR_ARRAY_IMPLICIT_STRING()\n```";
        let native5 = parser.parse_native_from_markdown_content(md_content5, "TEST").unwrap().unwrap();
        assert_eq!(native5.name, "GET_CHAR_ARRAY_IMPLICIT_STRING");
        if let NativeType::Array { element_type, size_info } = native5.return_type {
            assert_eq!(*element_type, NativeType::Char);
            assert_eq!(size_info, Some(crate::native_types::ArraySizeInfo::NullTerminated));
        } else {
            panic!("Expected Array return type for GET_CHAR_ARRAY_IMPLICIT_STRING, got {:?}", native5.return_type);
        }
    }

    #[test]
    fn test_parse_parameters_and_return_types_edge_cases() {
        // char[] с фиксированным размером
        let params_str = "char buffer[64], int id";
        let params = parse_parameters_from_md_signature(params_str, "test_func_char_fixed").unwrap();
        assert_eq!(params.len(), 2);
        assert_eq!(params[0].name, "buffer");
        match &params[0].param_type {
            NativeType::Array { element_type, size_info } => {
                assert_eq!(**element_type, NativeType::Char);
                assert_eq!(*size_info, Some(crate::native_types::ArraySizeInfo::Known(64)));
            }
            _ => panic!("Expected NativeType::Array for buffer, got {:?}", params[0].param_type),
        }

        // const char*[] (массив строк)
        let params_str2 = "const char* names[], int count";
        let params2 = parse_parameters_from_md_signature(params_str2, "test_func_string_array").unwrap();
        assert_eq!(params2.len(), 2);
        assert_eq!(params2[0].name, "names");
        match &params2[0].param_type {
            NativeType::Array { element_type, size_info } => {
                assert_eq!(**element_type, NativeType::String);
                assert_eq!(*size_info, Some(crate::native_types::ArraySizeInfo::NullTerminated));
            }
            _ => panic!("Expected NativeType::Array for names, got {:?}", params2[0].param_type),
        }

        // Vector3[] с динамическим размером
        let params_str3 = "Vector3 positions[MAX_PLAYERS], int num";
        let params3 = parse_parameters_from_md_signature(params_str3, "test_func_vec3_array").unwrap();
        assert_eq!(params3.len(), 2);
        assert_eq!(params3[0].name, "positions");
        match &params3[0].param_type {
            NativeType::Array { element_type, size_info } => {
                assert_eq!(**element_type, NativeType::Vector3);
                if let Some(crate::native_types::ArraySizeInfo::Dynamic { size_param_name }) = size_info {
                    assert_eq!(size_param_name, "MAX_PLAYERS");
                } else {
                    panic!("Expected Dynamic size_info for positions, got {:?}", size_info);
                }
            }
            _ => panic!("Expected NativeType::Array for positions, got {:?}", params3[0].param_type),
        }

        // char[] null-terminated (строка)
        let params_str4 = "char text[]";
        let params4 = parse_parameters_from_md_signature(params_str4, "test_func_char_nullterm").unwrap();
        assert_eq!(params4.len(), 1);
        match &params4[0].param_type {
            NativeType::Array { element_type, size_info } => {
                assert_eq!(**element_type, NativeType::Char);
                assert_eq!(*size_info, Some(crate::native_types::ArraySizeInfo::NullTerminated));
            }
            _ => panic!("Expected NativeType::Array for text, got {:?}", params4[0].param_type),
        }
    }

    /// Регрессионный тест: убеждаемся, что наличие нестандартной YAML-директивы
    /// (например, `returns:`) во фронт-маттере **не** приводит к ошибкам парсинга
    /// и функция корректно извлекается.
    #[test]
    fn test_regression_no_yaml_directive_warning() {
        let parser = default_parser();

        // Фронт-маттер содержит лишнюю директиву `returns`, которая раньше вызывала warning
        let md_content = r#"---
ns: PLAYER
returns: BOOL
---
## REGRESSION_TEST_FUNC

```c
// 0xDEADBEEF
void REGRESSION_TEST_FUNC();
```
"#;

        let native_opt = parser
            .parse_native_from_markdown_content(md_content, "PLAYER")
            .expect("Парсер не должен возвращать ошибку");

        // Должна быть успешно распознана одна функция
        let native = native_opt.expect("Функция должна быть распознана");
        assert_eq!(native.name, "REGRESSION_TEST_FUNC");
        assert_eq!(native.category, "PLAYER");
        // Тип возврата по умолчанию void, так как сигнатура void
        assert_eq!(native.return_type, crate::native_types::NativeType::Void);
    }
} 