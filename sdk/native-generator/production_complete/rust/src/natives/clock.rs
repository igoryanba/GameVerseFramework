//! CLOCK native functions
//! 
//! Functions for the clock category (local)

use crate::types::*;
use crate::errors::{NativeError, NativeResult, validation};
use nalgebra::Vector3;
use tracing::{debug, warn};

/// ```
Gets current UTC time
```



pub fn get_utc_time_safe(
        
        
            year: 
        , 
        
        
            month: 
        , 
        
        
            day: 
        , 
        
        
            hour: 
        , 
        
        
            minute: 
        , 
        
        
            second: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x8117E09A19EEF4D3u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x8117E09A19EEF4D3u64;
        
        let result = invoke_raw!(
            hash,
                year, 
                month, 
                day, 
                hour, 
                minute, 
                second
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn get_utc_time_raw(
        year: , 
        month: , 
        day: , 
        hour: , 
        minute: , 
        second: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x8117E09A19EEF4D3u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x8117E09A19EEF4D3u64;

        invoke_raw_typed!(
            hash,
                year, 
                month, 
                day, 
                hour, 
                minute, 
                second
        )
    }
}

/// ```
Gets system time as year, month, day, hour, minute and second.  
Example usage:  
	int year;  
	int month;  
	int day;  
	int hour;  
	int minute;  
	int second;  
	TIME::GET_POSIX_TIME(&year, &month, &day, &hour, &minute, &second);  
```



pub fn get_posix_time_safe(
        
        
            year: 
        , 
        
        
            month: 
        , 
        
        
            day: 
        , 
        
        
            hour: 
        , 
        
        
            minute: 
        , 
        
        
            second: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xDA488F299A5B164Eu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xDA488F299A5B164Eu64;
        
        let result = invoke_raw!(
            hash,
                year, 
                month, 
                day, 
                hour, 
                minute, 
                second
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn get_posix_time_raw(
        year: , 
        month: , 
        day: , 
        hour: , 
        minute: , 
        second: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xDA488F299A5B164Eu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xDA488F299A5B164Eu64;

        invoke_raw_typed!(
            hash,
                year, 
                month, 
                day, 
                hour, 
                minute, 
                second
        )
    }
}

/// ```
Gets the current ingame clock second. Note that ingame clock seconds change really fast since a day in GTA is only 48 minutes in real life.  
```



pub fn get_clock_seconds_safe(
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x494E97C2EF27C470u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x494E97C2EF27C470u64;
        
        let result = invoke_raw!(
            hash,
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn get_clock_seconds_raw(
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x494E97C2EF27C470u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x494E97C2EF27C470u64;

        invoke_raw_typed!(
            hash,
        )
    }
}

/// ```
Gets the current ingame hour, expressed without zeros. (09:34 will be represented as 9)  
```



pub fn get_clock_hours_safe(
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x25223CA6B4D20B7Fu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x25223CA6B4D20B7Fu64;
        
        let result = invoke_raw!(
            hash,
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn get_clock_hours_raw(
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x25223CA6B4D20B7Fu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x25223CA6B4D20B7Fu64;

        invoke_raw_typed!(
            hash,
        )
    }
}

/// ## Return value



pub fn get_clock_day_of_month_safe(
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x3D10BC92A4DB1D35u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x3D10BC92A4DB1D35u64;
        
        let result = invoke_raw!(
            hash,
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn get_clock_day_of_month_raw(
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x3D10BC92A4DB1D35u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x3D10BC92A4DB1D35u64;

        invoke_raw_typed!(
            hash,
        )
    }
}

/// ```
SET_CLOCK_TIME(12, 34, 56);  
```



pub fn set_clock_time_safe(
        
        
            hour: 
        , 
        
        
            minute: 
        , 
        
        
            second: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x47C3B5848C3E45D8u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x47C3B5848C3E45D8u64;
        
        let result = invoke_raw!(
            hash,
                hour, 
                minute, 
                second
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn set_clock_time_raw(
        hour: , 
        minute: , 
        second: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x47C3B5848C3E45D8u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x47C3B5848C3E45D8u64;

        invoke_raw_typed!(
            hash,
                hour, 
                minute, 
                second
        )
    }
}

/// ## Parameters
*



pub fn pause_clock_safe(
        
        
            toggle: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x4055E40BD2DBEC1Du32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x4055E40BD2DBEC1Du64;
        
        let result = invoke_raw!(
            hash,
                toggle
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn pause_clock_raw(
        toggle: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x4055E40BD2DBEC1Du32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x4055E40BD2DBEC1Du64;

        invoke_raw_typed!(
            hash,
                toggle
        )
    }
}

/// Returns how many real ms are equal to one game minute.
A getter for [`SetMillisecondsPerGameMinute`](#_0x36CA2554).



pub fn get_milliseconds_per_game_minute_safe(
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x2F8B4D1C595B11DBu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x2F8B4D1C595B11DBu64;
        
        let result = invoke_raw!(
            hash,
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn get_milliseconds_per_game_minute_raw(
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x2F8B4D1C595B11DBu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x2F8B4D1C595B11DBu64;

        invoke_raw_typed!(
            hash,
        )
    }
}

/// ## Parameters
*



pub fn add_to_clock_time_safe(
        
        
            hours: 
        , 
        
        
            minutes: 
        , 
        
        
            seconds: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xD716F30D8C8980E2u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xD716F30D8C8980E2u64;
        
        let result = invoke_raw!(
            hash,
                hours, 
                minutes, 
                seconds
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn add_to_clock_time_raw(
        hours: , 
        minutes: , 
        seconds: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xD716F30D8C8980E2u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xD716F30D8C8980E2u64;

        invoke_raw_typed!(
            hash,
                hours, 
                minutes, 
                seconds
        )
    }
}

/// ## Parameters
*



pub fn set_clock_date_safe(
        
        
            day: 
        , 
        
        
            month: 
        , 
        
        
            year: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xB096419DF0D06CE7u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xB096419DF0D06CE7u64;
        
        let result = invoke_raw!(
            hash,
                day, 
                month, 
                year
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn set_clock_date_raw(
        day: , 
        month: , 
        year: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xB096419DF0D06CE7u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xB096419DF0D06CE7u64;

        invoke_raw_typed!(
            hash,
                day, 
                month, 
                year
        )
    }
}

/// ## Return value



pub fn get_clock_month_safe(
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xBBC72712E80257A1u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xBBC72712E80257A1u64;
        
        let result = invoke_raw!(
            hash,
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn get_clock_month_raw(
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xBBC72712E80257A1u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xBBC72712E80257A1u64;

        invoke_raw_typed!(
            hash,
        )
    }
}

/// ## Return value



pub fn get_clock_year_safe(
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x961777E64BDAF717u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x961777E64BDAF717u64;
        
        let result = invoke_raw!(
            hash,
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn get_clock_year_raw(
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x961777E64BDAF717u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x961777E64BDAF717u64;

        invoke_raw_typed!(
            hash,
        )
    }
}

/// ```
Gets the current ingame clock minute.  
```



pub fn get_clock_minutes_safe(
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x13D2B8ADD79640F2u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x13D2B8ADD79640F2u64;
        
        let result = invoke_raw!(
            hash,
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn get_clock_minutes_raw(
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x13D2B8ADD79640F2u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x13D2B8ADD79640F2u64;

        invoke_raw_typed!(
            hash,
        )
    }
}

/// ```
Gets the current day of the week.  
0: Sunday  
1: Monday  
2: Tuesday  
3: Wednesday  
4: Thursday  
5: Friday  
6: Saturday  
```



pub fn get_clock_day_of_week_safe(
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xD972E4BD7AEB235Fu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xD972E4BD7AEB235Fu64;
        
        let result = invoke_raw!(
            hash,
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn get_clock_day_of_week_raw(
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xD972E4BD7AEB235Fu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xD972E4BD7AEB235Fu64;

        invoke_raw_typed!(
            hash,
        )
    }
}

/// ```
Gets local system time as year, month, day, hour, minute and second.  
Example usage:  
int year;  
int month;  
int day;  
int hour;  
int minute;  
int second;  
or use std::tm struct  
TIME::GET_LOCAL_TIME(&year, &month, &day, &hour, &minute, &second);  
```



pub fn get_local_time_safe(
        
        
            year: 
        , 
        
        
            month: 
        , 
        
        
            day: 
        , 
        
        
            hour: 
        , 
        
        
            minute: 
        , 
        
        
            second: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x50C7A99057A69748u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x50C7A99057A69748u64;
        
        let result = invoke_raw!(
            hash,
                year, 
                month, 
                day, 
                hour, 
                minute, 
                second
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn get_local_time_raw(
        year: , 
        month: , 
        day: , 
        hour: , 
        minute: , 
        second: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x50C7A99057A69748u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x50C7A99057A69748u64;

        invoke_raw_typed!(
            hash,
                year, 
                month, 
                day, 
                hour, 
                minute, 
                second
        )
    }
}

/// ## Parameters
*



pub fn advance_clock_time_to_safe(
        
        
            hour: 
        , 
        
        
            minute: 
        , 
        
        
            second: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xC8CA9670B9D83B3Bu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xC8CA9670B9D83B3Bu64;
        
        let result = invoke_raw!(
            hash,
                hour, 
                minute, 
                second
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn advance_clock_time_to_raw(
        hour: , 
        minute: , 
        second: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xC8CA9670B9D83B3Bu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xC8CA9670B9D83B3Bu64;

        invoke_raw_typed!(
            hash,
                hour, 
                minute, 
                second
        )
    }
}

