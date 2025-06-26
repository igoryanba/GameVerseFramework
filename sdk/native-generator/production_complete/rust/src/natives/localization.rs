//! LOCALIZATION native functions
//! 
//! Functions for the localization category (local)

use crate::types::*;
use crate::errors::{NativeError, NativeResult, validation};
use nalgebra::Vector3;
use tracing::{debug, warn};

/// ```
Possible return values: 0, 1, 2
```



pub fn _localization_get_system_date_format_safe(
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xA8AE43AEC1A61314u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xA8AE43AEC1A61314u64;
        
        let result = invoke_raw!(
            hash,
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn _localization_get_system_date_format_raw(
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xA8AE43AEC1A61314u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xA8AE43AEC1A61314u64;

        invoke_raw_typed!(
            hash,
        )
    }
}

/// ```
0 = american (en-US)
1 = french (fr-FR)
2 = german (de-DE)
3 = italian (it-IT)
4 = spanish (es-ES)
5 = brazilian (pt-BR)
6 = polish (pl-PL)
7 = russian (ru-RU)
8 = korean (ko-KR)
9 = chinesetrad (zh-TW)
10 = japanese (ja-JP)
11 = mexican (es-MX)
12 = chinesesimp (zh-CN)
```



pub fn get_current_language_safe(
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x2BDD44CC428A7EAEu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x2BDD44CC428A7EAEu64;
        
        let result = invoke_raw!(
            hash,
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn get_current_language_raw(
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x2BDD44CC428A7EAEu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x2BDD44CC428A7EAEu64;

        invoke_raw_typed!(
            hash,
        )
    }
}

/// ```
Same return values as GET_CURRENT_LANGUAGE
```



pub fn _localization_get_system_language_safe(
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x497420E022796B3Fu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x497420E022796B3Fu64;
        
        let result = invoke_raw!(
            hash,
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn _localization_get_system_language_raw(
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x497420E022796B3Fu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x497420E022796B3Fu64;

        invoke_raw_typed!(
            hash,
        )
    }
}

