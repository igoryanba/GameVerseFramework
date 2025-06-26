//! STATS native functions
//! 
//! Functions for the stats category (local)

use crate::types::*;
use crate::errors::{NativeError, NativeResult, validation};
use nalgebra::Vector3;
use tracing::{debug, warn};

/// ## Parameters
*



pub fn _playstats_director_mode_safe(
        
        
            p0: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x46326E13DA4E0546u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x46326E13DA4E0546u64;
        
        let result = invoke_raw!(
            hash,
                p0
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn _playstats_director_mode_raw(
        p0: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x46326E13DA4E0546u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x46326E13DA4E0546u64;

        invoke_raw_typed!(
            hash,
                p0
        )
    }
}

/// ## Parameters
*



pub fn _0xdaf80797fc534bec_safe(
        
        
            p0: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xDAF80797FC534BECu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xDAF80797FC534BECu64;
        
        let result = invoke_raw!(
            hash,
                p0
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn _0xdaf80797fc534bec_raw(
        p0: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xDAF80797FC534BECu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xDAF80797FC534BECu64;

        invoke_raw_typed!(
            hash,
                p0
        )
    }
}

/// ```
NativeDB Introduced: v1868
```



pub fn _0xf9096193df1f99d4_safe(
        
        
            p0: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xF9096193DF1F99D4u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xF9096193DF1F99D4u64;
        
        let result = invoke_raw!(
            hash,
                p0
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn _0xf9096193df1f99d4_raw(
        p0: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xF9096193DF1F99D4u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xF9096193DF1F99D4u64;

        invoke_raw_typed!(
            hash,
                p0
        )
    }
}

/// ```
The following values have been found in the decompiled scripts:  
"RC_ABI1"  
"RC_ABI2"  
"RC_BA1"  
"RC_BA2"  
"RC_BA3"  
"RC_BA3A"  
"RC_BA3C"  
"RC_BA4"  
"RC_DRE1"  
"RC_EPS1"  
"RC_EPS2"  
"RC_EPS3"  
"RC_EPS4"  
"RC_EPS5"  
"RC_EPS6"  
"RC_EPS7"  
"RC_EPS8"  
"RC_EXT1"  
"RC_EXT2"  
"RC_EXT3"  
"RC_EXT4"  
"RC_FAN1"  
"RC_FAN2"  
"RC_FAN3"  
"RC_HAO1"  
"RC_HUN1"  
"RC_HUN2"  
"RC_JOS1"  
"RC_JOS2"  
"RC_JOS3"  
"RC_JOS4"  
"RC_MAU1"  
"RC_MIN1"  
"RC_MIN2"  
"RC_MIN3"  
"RC_MRS1"  
"RC_MRS2"  
"RC_NI1"  
"RC_NI1A"  
"RC_NI1B"  
"RC_NI1C"  
"RC_NI1D"  
"RC_NI2"  
"RC_NI3"  
"RC_OME1"  
"RC_OME2"  
"RC_PA1"  
"RC_PA2"  
"RC_PA3"  
"RC_PA3A"  
"RC_PA3B"  
"RC_PA4"  
"RC_RAM1"  
"RC_RAM2"  
"RC_RAM3"  
"RC_RAM4"  
"RC_RAM5"  
"RC_SAS1"  
"RC_TON1"  
"RC_TON2"  
"RC_TON3"  
"RC_TON4"  
"RC_TON5"  
```



pub fn stat_set_gxt_label_safe(
        
        
            statName: 
        , 
        
        
            value: 
        , 
        
        
            save: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x17695002FD8B2AE0u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x17695002FD8B2AE0u64;
        
        let result = invoke_raw!(
            hash,
                statName, 
                value, 
                save
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn stat_set_gxt_label_raw(
        statName: , 
        value: , 
        save: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x17695002FD8B2AE0u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x17695002FD8B2AE0u64;

        invoke_raw_typed!(
            hash,
                statName, 
                value, 
                save
        )
    }
}

/// ```
This function is hard-coded to always return 1.
NETWORK_IS_*
```



pub fn _0xb3da2606774a8e2d_safe(
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xB3DA2606774A8E2Du32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xB3DA2606774A8E2Du64;
        
        let result = invoke_raw!(
            hash,
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn _0xb3da2606774a8e2d_raw(
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xB3DA2606774A8E2Du32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xB3DA2606774A8E2Du64;

        invoke_raw_typed!(
            hash,
        )
    }
}

/// _0x6F361B8889A792A3 native function



pub fn _0x6f361b8889a792a3_safe(
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x6F361B8889A792A3u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x6F361B8889A792A3u64;
        
        let result = invoke_raw!(
            hash,
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn _0x6f361b8889a792a3_raw(
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x6F361B8889A792A3u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x6F361B8889A792A3u64;

        invoke_raw_typed!(
            hash,
        )
    }
}

/// ```
NativeDB Introduced: v1734
```



pub fn _playstats_casino_luckyseven_safe(
        
        
            p0: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x0C432C1435F5E4FAu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x0C432C1435F5E4FAu64;
        
        let result = invoke_raw!(
            hash,
                p0
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn _playstats_casino_luckyseven_raw(
        p0: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x0C432C1435F5E4FAu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x0C432C1435F5E4FAu64;

        invoke_raw_typed!(
            hash,
                p0
        )
    }
}

/// ## Parameters
*



pub fn stat_set_pos_safe(
        
        
            statName: 
        , 
        
        
            x: 
        , 
        
        
            y: 
        , 
        
        
            z: 
        , 
        
        
            save: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xDB283FDE680FE72Eu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xDB283FDE680FE72Eu64;
        
        let result = invoke_raw!(
            hash,
                statName, 
                x, 
                y, 
                z, 
                save
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn stat_set_pos_raw(
        statName: , 
        x: , 
        y: , 
        z: , 
        save: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xDB283FDE680FE72Eu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xDB283FDE680FE72Eu64;

        invoke_raw_typed!(
            hash,
                statName, 
                x, 
                y, 
                z, 
                save
        )
    }
}

/// ## Parameters
*



pub fn _0x015b03ee1c43e6ec_safe(
        
        
            p0: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x015B03EE1C43E6ECu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x015B03EE1C43E6ECu64;
        
        let result = invoke_raw!(
            hash,
                p0
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn _0x015b03ee1c43e6ec_raw(
        p0: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x015B03EE1C43E6ECu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x015B03EE1C43E6ECu64;

        invoke_raw_typed!(
            hash,
                p0
        )
    }
}

/// ```
Sets profile setting 866
SET_*
```



pub fn _set_has_content_unlocks_flags_safe(
        
        
            value: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xDAC073C7901F9E15u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xDAC073C7901F9E15u64;
        
        let result = invoke_raw!(
            hash,
                value
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn _set_has_content_unlocks_flags_raw(
        value: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xDAC073C7901F9E15u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xDAC073C7901F9E15u64;

        invoke_raw_typed!(
            hash,
                value
        )
    }
}

/// ## Parameters
*



pub fn _0x0a9c7f36e5d7b683_safe(
        
        
            p0: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x0A9C7F36E5D7B683u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x0A9C7F36E5D7B683u64;
        
        let result = invoke_raw!(
            hash,
                p0
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn _0x0a9c7f36e5d7b683_raw(
        p0: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x0A9C7F36E5D7B683u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x0A9C7F36E5D7B683u64;

        invoke_raw_typed!(
            hash,
                p0
        )
    }
}

/// ## Parameters
*



pub fn playstats_import_export_mission_done_safe(
        
        
            p0: 
        , 
        
        
            p1: 
        , 
        
        
            p2: 
        , 
        
        
            p3: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x2B69F5074C894811u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x2B69F5074C894811u64;
        
        let result = invoke_raw!(
            hash,
                p0, 
                p1, 
                p2, 
                p3
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn playstats_import_export_mission_done_raw(
        p0: , 
        p1: , 
        p2: , 
        p3: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x2B69F5074C894811u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x2B69F5074C894811u64;

        invoke_raw_typed!(
            hash,
                p0, 
                p1, 
                p2, 
                p3
        )
    }
}

/// ```
NativeDB Introduced: v1868
```



pub fn _0x53c31853ec9531ff_safe(
        
        
            p0: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x53C31853EC9531FFu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x53C31853EC9531FFu64;
        
        let result = invoke_raw!(
            hash,
                p0
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn _0x53c31853ec9531ff_raw(
        p0: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x53C31853EC9531FFu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x53C31853EC9531FFu64;

        invoke_raw_typed!(
            hash,
                p0
        )
    }
}

/// ```
NativeDB Introduced: v1734
```



pub fn _playstats_casino_insidetrack_safe(
        
        
            p0: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x049F059625058A86u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x049F059625058A86u64;
        
        let result = invoke_raw!(
            hash,
                p0
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn _playstats_casino_insidetrack_raw(
        p0: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x049F059625058A86u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x049F059625058A86u64;

        invoke_raw_typed!(
            hash,
                p0
        )
    }
}

/// Disallows CEventNetworkStuntPerformed to be triggered (Resets [`PLAYSTATS_START_TRACKING_STUNTS`](#_0x928DBFB892638EF3)).



pub fn playstats_stop_tracking_stunts_safe(
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x8A800DACCC0DA55Du32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x8A800DACCC0DA55Du64;
        
        let result = invoke_raw!(
            hash,
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn playstats_stop_tracking_stunts_raw(
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x8A800DACCC0DA55Du32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x8A800DACCC0DA55Du64;

        invoke_raw_typed!(
            hash,
        )
    }
}

/// ## Parameters
*



pub fn _0x5bd5f255321c4aaf_safe(
        
        
            p0: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x5BD5F255321C4AAFu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x5BD5F255321C4AAFu64;
        
        let result = invoke_raw!(
            hash,
                p0
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn _0x5bd5f255321c4aaf_raw(
        p0: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x5BD5F255321C4AAFu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x5BD5F255321C4AAFu64;

        invoke_raw_typed!(
            hash,
                p0
        )
    }
}

/// ## Parameters
*



pub fn leaderboards2_read_by_radius_safe(
        
        
            p0: 
        , 
        
        
            p1: 
        , 
        
        
            p2: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x5CE587FB5A42C8C4u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x5CE587FB5A42C8C4u64;
        
        let result = invoke_raw!(
            hash,
                p0, 
                p1, 
                p2
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn leaderboards2_read_by_radius_raw(
        p0: , 
        p1: , 
        p2: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x5CE587FB5A42C8C4u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x5CE587FB5A42C8C4u64;

        invoke_raw_typed!(
            hash,
                p0, 
                p1, 
                p2
        )
    }
}

/// ```
NativeDB Introduced: v2189
```



pub fn _0xd4367d310f079db0_safe(
        
        
            p0: 
        , 
        
        
            p1: 
        , 
        
        
            p2: 
        , 
        
        
            p3: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xD4367D310F079DB0u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xD4367D310F079DB0u64;
        
        let result = invoke_raw!(
            hash,
                p0, 
                p1, 
                p2, 
                p3
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn _0xd4367d310f079db0_raw(
        p0: , 
        p1: , 
        p2: , 
        p3: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xD4367D310F079DB0u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xD4367D310F079DB0u64;

        invoke_raw_typed!(
            hash,
                p0, 
                p1, 
                p2, 
                p3
        )
    }
}

/// ```
Does not take effect immediately, unfortunately.
profileSetting seems to only be 936, 937 and 938 in scripts
```



pub fn stat_set_profile_setting_value_safe(
        
        
            profileSetting: 
        , 
        
        
            value: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x68F01422BE1D838Fu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x68F01422BE1D838Fu64;
        
        let result = invoke_raw!(
            hash,
                profileSetting, 
                value
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn stat_set_profile_setting_value_raw(
        profileSetting: , 
        value: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x68F01422BE1D838Fu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x68F01422BE1D838Fu64;

        invoke_raw_typed!(
            hash,
                profileSetting, 
                value
        )
    }
}

/// ## Parameters
*



pub fn _0x27aa1c973cacfe63_safe(
        
        
            p0: 
        , 
        
        
            p1: 
        , 
        
        
            p2: 
        , 
        
        
            p3: 
        , 
        
        
            p4: 
        , 
        
        
            p5: 
        , 
        
        
            p6: 
        , 
        
        
            p7: 
        , 
        
        
            p8: 
        , 
        
        
            p9: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x27AA1C973CACFE63u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x27AA1C973CACFE63u64;
        
        let result = invoke_raw!(
            hash,
                p0, 
                p1, 
                p2, 
                p3, 
                p4, 
                p5, 
                p6, 
                p7, 
                p8, 
                p9
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn _0x27aa1c973cacfe63_raw(
        p0: , 
        p1: , 
        p2: , 
        p3: , 
        p4: , 
        p5: , 
        p6: , 
        p7: , 
        p8: , 
        p9: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x27AA1C973CACFE63u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x27AA1C973CACFE63u64;

        invoke_raw_typed!(
            hash,
                p0, 
                p1, 
                p2, 
                p3, 
                p4, 
                p5, 
                p6, 
                p7, 
                p8, 
                p9
        )
    }
}

/// Sets a byte that is then used in session_host and session_join metrics when hosting or joining a session



pub fn playstats_set_join_type_safe(
        
        
            joinType: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xD1032E482629049Eu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xD1032E482629049Eu64;
        
        let result = invoke_raw!(
            hash,
                joinType
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn playstats_set_join_type_raw(
        joinType: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xD1032E482629049Eu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xD1032E482629049Eu64;

        invoke_raw_typed!(
            hash,
                joinType
        )
    }
}

/// ## Return value



pub fn _0x9a62ec95ae10e011_safe(
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x9A62EC95AE10E011u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x9A62EC95AE10E011u64;
        
        let result = invoke_raw!(
            hash,
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn _0x9a62ec95ae10e011_raw(
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x9A62EC95AE10E011u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x9A62EC95AE10E011u64;

        invoke_raw_typed!(
            hash,
        )
    }
}

/// ```
NativeDB Introduced: v2372
```



pub fn _playstats_robbery_finale_safe(
        
        
            p0: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xBBA55BE9AAAABF44u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xBBA55BE9AAAABF44u64;
        
        let result = invoke_raw!(
            hash,
                p0
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn _playstats_robbery_finale_raw(
        p0: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xBBA55BE9AAAABF44u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xBBA55BE9AAAABF44u64;

        invoke_raw_typed!(
            hash,
                p0
        )
    }
}

/// ## Return value



pub fn _stat_save_migration_cancel_safe(
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x4FEF53183C3C6414u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x4FEF53183C3C6414u64;
        
        let result = invoke_raw!(
            hash,
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn _stat_save_migration_cancel_raw(
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x4FEF53183C3C6414u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x4FEF53183C3C6414u64;

        invoke_raw_typed!(
            hash,
        )
    }
}

/// ```
NativeDB Added Parameter 2: Any p1
NativeDB Introduced: v1493
```



pub fn _0xb26f670685631727_safe(
        
        
            p0: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xB26F670685631727u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xB26F670685631727u64;
        
        let result = invoke_raw!(
            hash,
                p0
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn _0xb26f670685631727_raw(
        p0: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xB26F670685631727u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xB26F670685631727u64;

        invoke_raw_typed!(
            hash,
                p0
        )
    }
}

/// ## Parameters
*



pub fn get_packed_tu_bool_stat_key_safe(
        
        
            index: 
        , 
        
        
            spStat: 
        , 
        
        
            charStat: 
        , 
        
        
            character: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xC4BB08EE7907471Eu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xC4BB08EE7907471Eu64;
        
        let result = invoke_raw!(
            hash,
                index, 
                spStat, 
                charStat, 
                character
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn get_packed_tu_bool_stat_key_raw(
        index: , 
        spStat: , 
        charStat: , 
        character: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xC4BB08EE7907471Eu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xC4BB08EE7907471Eu64;

        invoke_raw_typed!(
            hash,
                index, 
                spStat, 
                charStat, 
                character
        )
    }
}

/// ## Parameters
*



pub fn stat_slot_is_loaded_safe(
        
        
            p0: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x0D0A9F0E7BD91E3Cu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x0D0A9F0E7BD91E3Cu64;
        
        let result = invoke_raw!(
            hash,
                p0
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn stat_slot_is_loaded_raw(
        p0: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x0D0A9F0E7BD91E3Cu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x0D0A9F0E7BD91E3Cu64;

        invoke_raw_typed!(
            hash,
                p0
        )
    }
}

/// ## Parameters
*



pub fn _0xa3c53804bdb68ed2_safe(
        
        
            p0: 
        , 
        
        
            p1: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xA3C53804BDB68ED2u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xA3C53804BDB68ED2u64;
        
        let result = invoke_raw!(
            hash,
                p0, 
                p1
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn _0xa3c53804bdb68ed2_raw(
        p0: , 
        p1: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xA3C53804BDB68ED2u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xA3C53804BDB68ED2u64;

        invoke_raw_typed!(
            hash,
                p0, 
                p1
        )
    }
}

/// ## Parameters
*



pub fn _0x84dfc579c2fc214c_safe(
        
        
            p0: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x84DFC579C2FC214Cu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x84DFC579C2FC214Cu64;
        
        let result = invoke_raw!(
            hash,
                p0
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn _0x84dfc579c2fc214c_raw(
        p0: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x84DFC579C2FC214Cu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x84DFC579C2FC214Cu64;

        invoke_raw_typed!(
            hash,
                p0
        )
    }
}

/// ## Parameters
*



pub fn playstats_friend_activity_safe(
        
        
            p0: 
        , 
        
        
            p1: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x0F71DE29AB2258F1u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x0F71DE29AB2258F1u64;
        
        let result = invoke_raw!(
            hash,
                p0, 
                p1
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn playstats_friend_activity_raw(
        p0: , 
        p1: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x0F71DE29AB2258F1u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x0F71DE29AB2258F1u64;

        invoke_raw_typed!(
            hash,
                p0, 
                p1
        )
    }
}

/// ```
NativeDB Introduced: v1180
```



pub fn _playstats_pegasaircraft_safe(
        
        
            modelHash: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x9572BD4DD6B72122u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x9572BD4DD6B72122u64;
        
        let result = invoke_raw!(
            hash,
                modelHash
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn _playstats_pegasaircraft_raw(
        modelHash: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x9572BD4DD6B72122u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x9572BD4DD6B72122u64;

        invoke_raw_typed!(
            hash,
                modelHash
        )
    }
}

/// ```
PLAYSTATS_START_INVITE_DESPAWNING?
```



pub fn _playstats_start_offline_mode_safe(
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x098760C7461724CDu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x098760C7461724CDu64;
        
        let result = invoke_raw!(
            hash,
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn _playstats_start_offline_mode_raw(
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x098760C7461724CDu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x098760C7461724CDu64;

        invoke_raw_typed!(
            hash,
        )
    }
}

/// ```
NativeDB Introduced: v1493
```



pub fn _playstats_stone_hatchet_end_safe(
        
        
            data: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x35E39E5570358630u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x35E39E5570358630u64;
        
        let result = invoke_raw!(
            hash,
                data
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn _playstats_stone_hatchet_end_raw(
        data: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x35E39E5570358630u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x35E39E5570358630u64;

        invoke_raw_typed!(
            hash,
                data
        )
    }
}

/// ## Parameters
*



pub fn leaderboards_get_number_of_columns_safe(
        
        
            p0: 
        , 
        
        
            p1: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x117B45156D7EFF2Eu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x117B45156D7EFF2Eu64;
        
        let result = invoke_raw!(
            hash,
                p0, 
                p1
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn leaderboards_get_number_of_columns_raw(
        p0: , 
        p1: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x117B45156D7EFF2Eu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x117B45156D7EFF2Eu64;

        invoke_raw_typed!(
            hash,
                p0, 
                p1
        )
    }
}

/// ## Parameters
*



pub fn _0x6483c25849031c4f_safe(
        
        
            p0: 
        , 
        
        
            p1: 
        , 
        
        
            p2: 
        , 
        
        
            p3: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x6483C25849031C4Fu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x6483C25849031C4Fu64;
        
        let result = invoke_raw!(
            hash,
                p0, 
                p1, 
                p2, 
                p3
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn _0x6483c25849031c4f_raw(
        p0: , 
        p1: , 
        p2: , 
        p3: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x6483C25849031C4Fu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x6483C25849031C4Fu64;

        invoke_raw_typed!(
            hash,
                p0, 
                p1, 
                p2, 
                p3
        )
    }
}

/// ## Parameters
*



pub fn _0x7e6946f68a38b74f_safe(
        
        
            p0: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x7E6946F68A38B74Fu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x7E6946F68A38B74Fu64;
        
        let result = invoke_raw!(
            hash,
                p0
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn _0x7e6946f68a38b74f_raw(
        p0: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x7E6946F68A38B74Fu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x7E6946F68A38B74Fu64;

        invoke_raw_typed!(
            hash,
                p0
        )
    }
}

/// ## Return value



pub fn _0xaff47709f1d5dcce_safe(
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xAFF47709F1D5DCCEu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xAFF47709F1D5DCCEu64;
        
        let result = invoke_raw!(
            hash,
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn _0xaff47709f1d5dcce_raw(
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xAFF47709F1D5DCCEu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xAFF47709F1D5DCCEu64;

        invoke_raw_typed!(
            hash,
        )
    }
}

/// ```
STATS::0x343B27E2(0);  
STATS::0x343B27E2(1);  
STATS::0x343B27E2(2);  
STATS::0x343B27E2(3);  
STATS::0x343B27E2(4);  
STATS::0x343B27E2(5);  
STATS::0x343B27E2(6);  
STATS::0x343B27E2(7);  
Identical in ingamehud & maintransition.  
```



pub fn _0x26d7399b9587fe89_safe(
        
        
            p0: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x26D7399B9587FE89u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x26D7399B9587FE89u64;
        
        let result = invoke_raw!(
            hash,
                p0
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn _0x26d7399b9587fe89_raw(
        p0: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x26D7399B9587FE89u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x26D7399B9587FE89u64;

        invoke_raw_typed!(
            hash,
                p0
        )
    }
}

/// ```
NativeDB Introduced: v1734
```



pub fn _playstats_casino_roulette_light_safe(
        
        
            p0: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x6572ABA3DE1197FCu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x6572ABA3DE1197FCu64;
        
        let result = invoke_raw!(
            hash,
                p0
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn _playstats_casino_roulette_light_raw(
        p0: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x6572ABA3DE1197FCu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x6572ABA3DE1197FCu64;

        invoke_raw_typed!(
            hash,
                p0
        )
    }
}

/// ## Parameters
*



pub fn _0xa0f93d5465b3094d_safe(
        
        
            p0: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xA0F93D5465B3094Du32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xA0F93D5465B3094Du64;
        
        let result = invoke_raw!(
            hash,
                p0
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn _0xa0f93d5465b3094d_raw(
        p0: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xA0F93D5465B3094Du32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xA0F93D5465B3094Du64;

        invoke_raw_typed!(
            hash,
                p0
        )
    }
}

/// ## Parameters
*



pub fn _0xbf371cd2b64212fd_safe(
        
        
            p0: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xBF371CD2B64212FDu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xBF371CD2B64212FDu64;
        
        let result = invoke_raw!(
            hash,
                p0
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn _0xbf371cd2b64212fd_raw(
        p0: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xBF371CD2B64212FDu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xBF371CD2B64212FDu64;

        invoke_raw_typed!(
            hash,
                p0
        )
    }
}

/// ## Parameters
*



pub fn stat_get_save_migration_status_safe(
        
        
            data: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x886913BBEACA68C1u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x886913BBEACA68C1u64;
        
        let result = invoke_raw!(
            hash,
                data
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn stat_get_save_migration_status_raw(
        data: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x886913BBEACA68C1u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x886913BBEACA68C1u64;

        invoke_raw_typed!(
            hash,
                data
        )
    }
}

/// ## Parameters
*



pub fn _0x8989cbd7b4e82534_safe(
        
        
            p0: 
        , 
        
        
            p1: 
        , 
        
        
            p2: 
        , 
        
        
            p3: 
        , 
        
        
            p4: 
        , 
        
        
            p5: 
        , 
        
        
            p6: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x8989CBD7B4E82534u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x8989CBD7B4E82534u64;
        
        let result = invoke_raw!(
            hash,
                p0, 
                p1, 
                p2, 
                p3, 
                p4, 
                p5, 
                p6
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn _0x8989cbd7b4e82534_raw(
        p0: , 
        p1: , 
        p2: , 
        p3: , 
        p4: , 
        p5: , 
        p6: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x8989CBD7B4E82534u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x8989CBD7B4E82534u64;

        invoke_raw_typed!(
            hash,
                p0, 
                p1, 
                p2, 
                p3, 
                p4, 
                p5, 
                p6
        )
    }
}

/// ## Parameters
*



pub fn _0x38491439b6ba7f7d_safe(
        
        
            p0: 
        , 
        
        
            p1: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x38491439B6BA7F7Du32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x38491439B6BA7F7Du64;
        
        let result = invoke_raw!(
            hash,
                p0, 
                p1
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn _0x38491439b6ba7f7d_raw(
        p0: , 
        p1: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x38491439B6BA7F7Du32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x38491439B6BA7F7Du64;

        invoke_raw_typed!(
            hash,
                p0, 
                p1
        )
    }
}

/// ```
NativeDB Introduced: v2189
```



pub fn _0x2fa3173480008493_safe(
        
        
            p0: 
        , 
        
        
            p1: 
        , 
        
        
            p2: 
        , 
        
        
            p3: 
        , 
        
        
            p4: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x2FA3173480008493u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x2FA3173480008493u64;
        
        let result = invoke_raw!(
            hash,
                p0, 
                p1, 
                p2, 
                p3, 
                p4
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn _0x2fa3173480008493_raw(
        p0: , 
        p1: , 
        p2: , 
        p3: , 
        p4: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x2FA3173480008493u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x2FA3173480008493u64;

        invoke_raw_typed!(
            hash,
                p0, 
                p1, 
                p2, 
                p3, 
                p4
        )
    }
}

/// ```
NativeDB Introduced: v1734
```



pub fn _playstats_casino_blackjack_safe(
        
        
            p0: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x3EAE97309727E7ADu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x3EAE97309727E7ADu64;
        
        let result = invoke_raw!(
            hash,
                p0
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn _playstats_casino_blackjack_raw(
        p0: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x3EAE97309727E7ADu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x3EAE97309727E7ADu64;

        invoke_raw_typed!(
            hash,
                p0
        )
    }
}

/// ## Parameters
*



pub fn _0x0b565b0aae56a0e8_safe(
        
        
            p0: 
        , 
        
        
            p1: 
        , 
        
        
            p2: 
        , 
        
        
            p3: 
        , 
        
        
            p4: 
        , 
        
        
            p5: 
        , 
        
        
            p6: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x0B565B0AAE56A0E8u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x0B565B0AAE56A0E8u64;
        
        let result = invoke_raw!(
            hash,
                p0, 
                p1, 
                p2, 
                p3, 
                p4, 
                p5, 
                p6
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn _0x0b565b0aae56a0e8_raw(
        p0: , 
        p1: , 
        p2: , 
        p3: , 
        p4: , 
        p5: , 
        p6: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x0B565B0AAE56A0E8u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x0B565B0AAE56A0E8u64;

        invoke_raw_typed!(
            hash,
                p0, 
                p1, 
                p2, 
                p3, 
                p4, 
                p5, 
                p6
        )
    }
}

/// ## Parameters
*



pub fn _0xaa525dff66bb82f5_safe(
        
        
            p0: 
        , 
        
        
            p1: 
        , 
        
        
            p2: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xAA525DFF66BB82F5u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xAA525DFF66BB82F5u64;
        
        let result = invoke_raw!(
            hash,
                p0, 
                p1, 
                p2
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn _0xaa525dff66bb82f5_raw(
        p0: , 
        p1: , 
        p2: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xAA525DFF66BB82F5u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xAA525DFF66BB82F5u64;

        invoke_raw_typed!(
            hash,
                p0, 
                p1, 
                p2
        )
    }
}

/// ## Parameters
*



pub fn playstats_website_visited_safe(
        
        
            scaleformHash: 
        , 
        
        
            p1: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xDDF24D535060F811u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xDDF24D535060F811u64;
        
        let result = invoke_raw!(
            hash,
                scaleformHash, 
                p1
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn playstats_website_visited_raw(
        scaleformHash: , 
        p1: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xDDF24D535060F811u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xDDF24D535060F811u64;

        invoke_raw_typed!(
            hash,
                scaleformHash, 
                p1
        )
    }
}

/// ## Parameters
*



pub fn leaderboards_read_successful_safe(
        
        
            p0: 
        , 
        
        
            p1: 
        , 
        
        
            p2: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x2FB19228983E832Cu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x2FB19228983E832Cu64;
        
        let result = invoke_raw!(
            hash,
                p0, 
                p1, 
                p2
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn leaderboards_read_successful_raw(
        p0: , 
        p1: , 
        p2: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x2FB19228983E832Cu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x2FB19228983E832Cu64;

        invoke_raw_typed!(
            hash,
                p0, 
                p1, 
                p2
        )
    }
}

/// ## Return value



pub fn _0xf9f2922717b819ec_safe(
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xF9F2922717B819ECu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xF9F2922717B819ECu64;
        
        let result = invoke_raw!(
            hash,
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn _0xf9f2922717b819ec_raw(
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xF9F2922717B819ECu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xF9F2922717B819ECu64;

        invoke_raw_typed!(
            hash,
        )
    }
}

/// ## Parameters
*



pub fn _0xfcc228e07217fcac_safe(
        
        
            p0: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xFCC228E07217FCACu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xFCC228E07217FCACu64;
        
        let result = invoke_raw!(
            hash,
                p0
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn _0xfcc228e07217fcac_raw(
        p0: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xFCC228E07217FCACu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xFCC228E07217FCACu64;

        invoke_raw_typed!(
            hash,
                p0
        )
    }
}

/// ```
NativeDB Removed Parameter 5: Any p4
NativeDB Removed Parameter 6: Any p5
NativeDB Removed Parameter 7: Any p6
NativeDB Removed Parameter 8: Any p7
NativeDB Removed Parameter 9: Any p8
NativeDB Removed Parameter 10: Any p9
```



pub fn playstats_race_to_point_mission_done_safe(
        
        
            p0: 
        , 
        
        
            p1: 
        , 
        
        
            p2: 
        , 
        
        
            p3: 
        , 
        
        
            p4: 
        , 
        
        
            p5: 
        , 
        
        
            p6: 
        , 
        
        
            p7: 
        , 
        
        
            p8: 
        , 
        
        
            p9: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xADDD1C754E2E2914u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xADDD1C754E2E2914u64;
        
        let result = invoke_raw!(
            hash,
                p0, 
                p1, 
                p2, 
                p3, 
                p4, 
                p5, 
                p6, 
                p7, 
                p8, 
                p9
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn playstats_race_to_point_mission_done_raw(
        p0: , 
        p1: , 
        p2: , 
        p3: , 
        p4: , 
        p5: , 
        p6: , 
        p7: , 
        p8: , 
        p9: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xADDD1C754E2E2914u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xADDD1C754E2E2914u64;

        invoke_raw_typed!(
            hash,
                p0, 
                p1, 
                p2, 
                p3, 
                p4, 
                p5, 
                p6, 
                p7, 
                p8, 
                p9
        )
    }
}

/// ## Parameters
*



pub fn leaderboards2_read_by_rank_safe(
        
        
            p0: 
        , 
        
        
            p1: 
        , 
        
        
            p2: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xBA2C7DB0C129449Au32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xBA2C7DB0C129449Au64;
        
        let result = invoke_raw!(
            hash,
                p0, 
                p1, 
                p2
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn leaderboards2_read_by_rank_raw(
        p0: , 
        p1: , 
        p2: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xBA2C7DB0C129449Au32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xBA2C7DB0C129449Au64;

        invoke_raw_typed!(
            hash,
                p0, 
                p1, 
                p2
        )
    }
}

/// ## Parameters
*



pub fn _0x3ebeac6c3f81f6bd_safe(
        
        
            p0: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x3EBEAC6C3F81F6BDu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x3EBEAC6C3F81F6BDu64;
        
        let result = invoke_raw!(
            hash,
                p0
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn _0x3ebeac6c3f81f6bd_raw(
        p0: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x3EBEAC6C3F81F6BDu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x3EBEAC6C3F81F6BDu64;

        invoke_raw_typed!(
            hash,
                p0
        )
    }
}

/// ```
Needs more research. Gets the stat name of a masked bool?
section - values used in the decompiled scripts:
"_NGPSTAT_BOOL"
"_NGTATPSTAT_BOOL"
"_NGDLCPSTAT_BOOL"
"_DLCBIKEPSTAT_BOOL"
"_DLCGUNPSTAT_BOOL"
"_GUNTATPSTAT_BOOL"
"_DLCSMUGCHARPSTAT_BOOL"
"_GANGOPSPSTAT_BOOL"
"_BUSINESSBATPSTAT_BOOL"
"_ARENAWARSPSTAT_BOOL"
"_CASINOPSTAT_BOOL"
"_CASINOHSTPSTAT_BOOL"
"_HEIST3TATTOOSTAT_BOOL"
```



pub fn _get_ngstat_bool_hash_safe(
        
        
            index: 
        , 
        
        
            spStat: 
        , 
        
        
            charStat: 
        , 
        
        
            character: 
        , 
        
        
            section: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xBA52FF538ED2BC71u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xBA52FF538ED2BC71u64;
        
        let result = invoke_raw!(
            hash,
                index, 
                spStat, 
                charStat, 
                character, 
                section
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn _get_ngstat_bool_hash_raw(
        index: , 
        spStat: , 
        charStat: , 
        character: , 
        section: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xBA52FF538ED2BC71u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xBA52FF538ED2BC71u64;

        invoke_raw_typed!(
            hash,
                index, 
                spStat, 
                charStat, 
                character, 
                section
        )
    }
}

/// ## Parameters
*



pub fn get_packed_bool_stat_key_safe(
        
        
            index: 
        , 
        
        
            spStat: 
        , 
        
        
            charStat: 
        , 
        
        
            character: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x80C75307B1C42837u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x80C75307B1C42837u64;
        
        let result = invoke_raw!(
            hash,
                index, 
                spStat, 
                charStat, 
                character
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn get_packed_bool_stat_key_raw(
        index: , 
        spStat: , 
        charStat: , 
        character: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x80C75307B1C42837u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x80C75307B1C42837u64;

        invoke_raw_typed!(
            hash,
                index, 
                spStat, 
                charStat, 
                character
        )
    }
}

/// ## Parameters
*



pub fn _0x73001e34f85137f8_safe(
        
        
            p0: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x73001E34F85137F8u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x73001E34F85137F8u64;
        
        let result = invoke_raw!(
            hash,
                p0
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn _0x73001e34f85137f8_raw(
        p0: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x73001E34F85137F8u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x73001E34F85137F8u64;

        invoke_raw_typed!(
            hash,
                p0
        )
    }
}

/// ```
NativeDB Introduced: v2189
```



pub fn _0x2818ff6638cb09de_safe(
        
        
            p0: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x2818FF6638CB09DEu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x2818FF6638CB09DEu64;
        
        let result = invoke_raw!(
            hash,
                p0
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn _0x2818ff6638cb09de_raw(
        p0: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x2818FF6638CB09DEu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x2818FF6638CB09DEu64;

        invoke_raw_typed!(
            hash,
                p0
        )
    }
}

/// ## Parameters
*



pub fn _0x88087ee1f28024ae_safe(
        
        
            p0: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x88087EE1F28024AEu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x88087EE1F28024AEu64;
        
        let result = invoke_raw!(
            hash,
                p0
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn _0x88087ee1f28024ae_raw(
        p0: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x88087EE1F28024AEu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x88087EE1F28024AEu64;

        invoke_raw_typed!(
            hash,
                p0
        )
    }
}

/// ## Parameters
*



pub fn _0x03c2eebb04b3fb72_safe(
        
        
            p0: 
        , 
        
        
            p1: 
        , 
        
        
            p2: 
        , 
        
        
            p3: 
        , 
        
        
            p4: 
        , 
        
        
            p5: 
        , 
        
        
            p6: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x03C2EEBB04B3FB72u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x03C2EEBB04B3FB72u64;
        
        let result = invoke_raw!(
            hash,
                p0, 
                p1, 
                p2, 
                p3, 
                p4, 
                p5, 
                p6
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn _0x03c2eebb04b3fb72_raw(
        p0: , 
        p1: , 
        p2: , 
        p3: , 
        p4: , 
        p5: , 
        p6: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x03C2EEBB04B3FB72u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x03C2EEBB04B3FB72u64;

        invoke_raw_typed!(
            hash,
                p0, 
                p1, 
                p2, 
                p3, 
                p4, 
                p5, 
                p6
        )
    }
}

/// ## Parameters
*



pub fn playstats_prop_change_safe(
        
        
            p0: 
        , 
        
        
            p1: 
        , 
        
        
            p2: 
        , 
        
        
            p3: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xBA739D6D5A05D6E7u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xBA739D6D5A05D6E7u64;
        
        let result = invoke_raw!(
            hash,
                p0, 
                p1, 
                p2, 
                p3
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn playstats_prop_change_raw(
        p0: , 
        p1: , 
        p2: , 
        p3: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xBA739D6D5A05D6E7u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xBA739D6D5A05D6E7u64;

        invoke_raw_typed!(
            hash,
                p0, 
                p1, 
                p2, 
                p3
        )
    }
}

/// ## Parameters
*



pub fn _0xa736cf7fb7c5bff4_safe(
        
        
            p0: 
        , 
        
        
            p1: 
        , 
        
        
            p2: 
        , 
        
        
            p3: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xA736CF7FB7C5BFF4u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xA736CF7FB7C5BFF4u64;
        
        let result = invoke_raw!(
            hash,
                p0, 
                p1, 
                p2, 
                p3
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn _0xa736cf7fb7c5bff4_raw(
        p0: , 
        p1: , 
        p2: , 
        p3: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xA736CF7FB7C5BFF4u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xA736CF7FB7C5BFF4u64;

        invoke_raw_typed!(
            hash,
                p0, 
                p1, 
                p2, 
                p3
        )
    }
}

/// ## Parameters
*



pub fn _0x164c5ff663790845_safe(
        
        
            p0: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x164C5FF663790845u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x164C5FF663790845u64;
        
        let result = invoke_raw!(
            hash,
                p0
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn _0x164c5ff663790845_raw(
        p0: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x164C5FF663790845u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x164C5FF663790845u64;

        invoke_raw_typed!(
            hash,
                p0
        )
    }
}

/// ```
NativeDB Added Parameter 4: Any p3
```



pub fn stat_save_safe(
        
        
            p0: 
        , 
        
        
            p1: 
        , 
        
        
            p2: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xE07BCA305B82D2FDu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xE07BCA305B82D2FDu64;
        
        let result = invoke_raw!(
            hash,
                p0, 
                p1, 
                p2
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn stat_save_raw(
        p0: , 
        p1: , 
        p2: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xE07BCA305B82D2FDu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xE07BCA305B82D2FDu64;

        invoke_raw_typed!(
            hash,
                p0, 
                p1, 
                p2
        )
    }
}

/// ## Parameters
*



pub fn playstats_background_script_action_safe(
        
        
            action: 
        , 
        
        
            value: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x5009DFD741329729u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x5009DFD741329729u64;
        
        let result = invoke_raw!(
            hash,
                action, 
                value
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn playstats_background_script_action_raw(
        action: , 
        value: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x5009DFD741329729u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x5009DFD741329729u64;

        invoke_raw_typed!(
            hash,
                action, 
                value
        )
    }
}

/// ```
_PLAYSTATS_ROB_ARMOURD_TRUCK  
```



pub fn _0x7eec2a316c250073_safe(
        
        
            p0: 
        , 
        
        
            p1: 
        , 
        
        
            p2: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x7EEC2A316C250073u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x7EEC2A316C250073u64;
        
        let result = invoke_raw!(
            hash,
                p0, 
                p1, 
                p2
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn _0x7eec2a316c250073_raw(
        p0: , 
        p1: , 
        p2: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x7EEC2A316C250073u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x7EEC2A316C250073u64;

        invoke_raw_typed!(
            hash,
                p0, 
                p1, 
                p2
        )
    }
}

/// ## Parameters
*



pub fn leaderboards_cache_data_row_safe(
        
        
            p0: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xB9BB18E2C40142EDu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xB9BB18E2C40142EDu64;
        
        let result = invoke_raw!(
            hash,
                p0
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn leaderboards_cache_data_row_raw(
        p0: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xB9BB18E2C40142EDu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xB9BB18E2C40142EDu64;

        invoke_raw_typed!(
            hash,
                p0
        )
    }
}

/// ## Parameters
*



pub fn playstats_weapon_mode_change_safe(
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xE95C8A1875A02CA4u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xE95C8A1875A02CA4u64;
        
        let result = invoke_raw!(
            hash,
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn playstats_weapon_mode_change_raw(
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xE95C8A1875A02CA4u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xE95C8A1875A02CA4u64;

        invoke_raw_typed!(
            hash,
        )
    }
}

/// ## Parameters
*



pub fn stat_get_license_plate_safe(
        
        
            statName: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x5473D4195058B2E4u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x5473D4195058B2E4u64;
        
        let result = invoke_raw!(
            hash,
                statName
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn stat_get_license_plate_raw(
        statName: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x5473D4195058B2E4u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x5473D4195058B2E4u64;

        invoke_raw_typed!(
            hash,
                statName
        )
    }
}

/// ```
p1 is always -1 in the script files  
```



pub fn stat_get_string_safe(
        
        
            statHash: 
        , 
        
        
            p1: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xE50384ACC2C3DB74u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xE50384ACC2C3DB74u64;
        
        let result = invoke_raw!(
            hash,
                statHash, 
                p1
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn stat_get_string_raw(
        statHash: , 
        p1: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xE50384ACC2C3DB74u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xE50384ACC2C3DB74u64;

        invoke_raw_typed!(
            hash,
                statHash, 
                p1
        )
    }
}

/// ```
NativeDB Introduced: v1493
```



pub fn _0x2d7a9b577e72385e_safe(
        
        
            p0: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x2D7A9B577E72385Eu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x2D7A9B577E72385Eu64;
        
        let result = invoke_raw!(
            hash,
                p0
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn _0x2d7a9b577e72385e_raw(
        p0: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x2D7A9B577E72385Eu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x2D7A9B577E72385Eu64;

        invoke_raw_typed!(
            hash,
                p0
        )
    }
}

/// ## Parameters
*



pub fn playstats_npc_invite_safe(
        
        
            p0: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x93054C88E6AA7C44u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x93054C88E6AA7C44u64;
        
        let result = invoke_raw!(
            hash,
                p0
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn playstats_npc_invite_raw(
        p0: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x93054C88E6AA7C44u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x93054C88E6AA7C44u64;

        invoke_raw_typed!(
            hash,
                p0
        )
    }
}

/// ```
Sets profile setting 940 and 941

_SET_F* - _SET_G*
```

```
NativeDB Introduced: v2699
```



pub fn _0x79d310a861697cc9_safe(
        
        
            profileSetting: 
        , 
        
        
            settingValue: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x79D310A861697CC9u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x79D310A861697CC9u64;
        
        let result = invoke_raw!(
            hash,
                profileSetting, 
                settingValue
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn _0x79d310a861697cc9_raw(
        profileSetting: , 
        settingValue: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x79D310A861697CC9u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x79D310A861697CC9u64;

        invoke_raw_typed!(
            hash,
                profileSetting, 
                settingValue
        )
    }
}

/// ## Return value



pub fn leaderboards_read_any_pending_safe(
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xA31FD15197B192BDu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xA31FD15197B192BDu64;
        
        let result = invoke_raw!(
            hash,
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn leaderboards_read_any_pending_raw(
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xA31FD15197B192BDu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xA31FD15197B192BDu64;

        invoke_raw_typed!(
            hash,
        )
    }
}

/// ## Parameters
*



pub fn playstats_shop_item_safe(
        
        
            p0: 
        , 
        
        
            p1: 
        , 
        
        
            p2: 
        , 
        
        
            p3: 
        , 
        
        
            p4: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x176852ACAAC173D1u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x176852ACAAC173D1u64;
        
        let result = invoke_raw!(
            hash,
                p0, 
                p1, 
                p2, 
                p3, 
                p4
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn playstats_shop_item_raw(
        p0: , 
        p1: , 
        p2: , 
        p3: , 
        p4: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x176852ACAAC173D1u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x176852ACAAC173D1u64;

        invoke_raw_typed!(
            hash,
                p0, 
                p1, 
                p2, 
                p3, 
                p4
        )
    }
}

/// ## Parameters
*



pub fn stat_get_pos_safe(
        
        
            p0: 
        , 
        
        
            p1: 
        , 
        
        
            p2: 
        , 
        
        
            p3: 
        , 
        
        
            p4: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x350F82CCB186AA1Bu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x350F82CCB186AA1Bu64;
        
        let result = invoke_raw!(
            hash,
                p0, 
                p1, 
                p2, 
                p3, 
                p4
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn stat_get_pos_raw(
        p0: , 
        p1: , 
        p2: , 
        p3: , 
        p4: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x350F82CCB186AA1Bu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x350F82CCB186AA1Bu64;

        invoke_raw_typed!(
            hash,
                p0, 
                p1, 
                p2, 
                p3, 
                p4
        )
    }
}

/// ## Parameters
*



pub fn stat_set_license_plate_safe(
        
        
            statName: 
        , 
        
        
            str: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x69FF13266D7296DAu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x69FF13266D7296DAu64;
        
        let result = invoke_raw!(
            hash,
                statName, 
                str
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn stat_set_license_plate_raw(
        statName: , 
        str: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x69FF13266D7296DAu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x69FF13266D7296DAu64;

        invoke_raw_typed!(
            hash,
                statName, 
                str
        )
    }
}

/// ## Return value



pub fn leaderboards_read_clear_all_safe(
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xA34CB6E6F0DF4A0Bu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xA34CB6E6F0DF4A0Bu64;
        
        let result = invoke_raw!(
            hash,
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn leaderboards_read_clear_all_raw(
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xA34CB6E6F0DF4A0Bu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xA34CB6E6F0DF4A0Bu64;

        invoke_raw_typed!(
            hash,
        )
    }
}

/// ## Parameters
*



pub fn _stat_save_migration_consume_content_unlock_safe(
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x3270F67EED31FBC1u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x3270F67EED31FBC1u64;
        
        let result = invoke_raw!(
            hash,
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn _stat_save_migration_consume_content_unlock_raw(
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x3270F67EED31FBC1u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x3270F67EED31FBC1u64;

        invoke_raw_typed!(
            hash,
        )
    }
}

/// ```
NativeDB Introduced: v1734
```



pub fn _playstats_casino_threecardpoker_safe(
        
        
            p0: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xF740FB339D471C35u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xF740FB339D471C35u64;
        
        let result = invoke_raw!(
            hash,
                p0
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn _playstats_casino_threecardpoker_raw(
        p0: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xF740FB339D471C35u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xF740FB339D471C35u64;

        invoke_raw_typed!(
            hash,
                p0
        )
    }
}

/// ## Parameters
*



pub fn _playstats_ban_alert_safe(
        
        
            p0: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x516FC96EB88EEFE5u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x516FC96EB88EEFE5u64;
        
        let result = invoke_raw!(
            hash,
                p0
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn _playstats_ban_alert_raw(
        p0: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x516FC96EB88EEFE5u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x516FC96EB88EEFE5u64;

        invoke_raw_typed!(
            hash,
                p0
        )
    }
}

/// ## Parameters
*



pub fn _0x6bccf9948492fd85_safe(
        
        
            p0: 
        , 
        
        
            p1: 
        , 
        
        
            p2: 
        , 
        
        
            p3: 
        , 
        
        
            p4: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x6BCCF9948492FD85u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x6BCCF9948492FD85u64;
        
        let result = invoke_raw!(
            hash,
                p0, 
                p1, 
                p2, 
                p3, 
                p4
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn _0x6bccf9948492fd85_raw(
        p0: , 
        p1: , 
        p2: , 
        p3: , 
        p4: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x6BCCF9948492FD85u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x6BCCF9948492FD85u64;

        invoke_raw_typed!(
            hash,
                p0, 
                p1, 
                p2, 
                p3, 
                p4
        )
    }
}

/// ```
NativeDB Introduced: v1734
```



pub fn _playstats_casino_insidetrack_light_safe(
        
        
            p0: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x23A3CBCD50D54E47u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x23A3CBCD50D54E47u64;
        
        let result = invoke_raw!(
            hash,
                p0
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn _playstats_casino_insidetrack_light_raw(
        p0: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x23A3CBCD50D54E47u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x23A3CBCD50D54E47u64;

        invoke_raw_typed!(
            hash,
                p0
        )
    }
}

/// ## Parameters
*



pub fn _0xa6f54bb2ffca35ea_safe(
        
        
            p0: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xA6F54BB2FFCA35EAu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xA6F54BB2FFCA35EAu64;
        
        let result = invoke_raw!(
            hash,
                p0
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn _0xa6f54bb2ffca35ea_raw(
        p0: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xA6F54BB2FFCA35EAu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xA6F54BB2FFCA35EAu64;

        invoke_raw_typed!(
            hash,
                p0
        )
    }
}

/// ## Parameters
*



pub fn playstats_mission_checkpoint_safe(
        
        
            p0: 
        , 
        
        
            p1: 
        , 
        
        
            p2: 
        , 
        
        
            p3: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xC900596A63978C1Du32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xC900596A63978C1Du64;
        
        let result = invoke_raw!(
            hash,
                p0, 
                p1, 
                p2, 
                p3
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn playstats_mission_checkpoint_raw(
        p0: , 
        p1: , 
        p2: , 
        p3: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xC900596A63978C1Du32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xC900596A63978C1Du64;

        invoke_raw_typed!(
            hash,
                p0, 
                p1, 
                p2, 
                p3
        )
    }
}

/// ## Parameters
*



pub fn _0xf534d94dfa2ead26_safe(
        
        
            p0: 
        , 
        
        
            p1: 
        , 
        
        
            p2: 
        , 
        
        
            p3: 
        , 
        
        
            p4: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xF534D94DFA2EAD26u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xF534D94DFA2EAD26u64;
        
        let result = invoke_raw!(
            hash,
                p0, 
                p1, 
                p2, 
                p3, 
                p4
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn _0xf534d94dfa2ead26_raw(
        p0: , 
        p1: , 
        p2: , 
        p3: , 
        p4: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xF534D94DFA2EAD26u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xF534D94DFA2EAD26u64;

        invoke_raw_typed!(
            hash,
                p0, 
                p1, 
                p2, 
                p3, 
                p4
        )
    }
}

/// ## Parameters
*



pub fn _0x3de3aa516fb126a4_safe(
        
        
            p0: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x3DE3AA516FB126A4u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x3DE3AA516FB126A4u64;
        
        let result = invoke_raw!(
            hash,
                p0
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn _0x3de3aa516fb126a4_raw(
        p0: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x3DE3AA516FB126A4u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x3DE3AA516FB126A4u64;

        invoke_raw_typed!(
            hash,
                p0
        )
    }
}

/// ```
p2 - Default value? Seems to be -1 most of the time.  
```



pub fn stat_get_bool_masked_safe(
        
        
            statName: 
        , 
        
        
            mask: 
        , 
        
        
            p2: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x10FE3F1B79F9B071u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x10FE3F1B79F9B071u64;
        
        let result = invoke_raw!(
            hash,
                statName, 
                mask, 
                p2
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn stat_get_bool_masked_raw(
        statName: , 
        mask: , 
        p2: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x10FE3F1B79F9B071u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x10FE3F1B79F9B071u64;

        invoke_raw_typed!(
            hash,
                statName, 
                mask, 
                p2
        )
    }
}

/// ## Parameters
*



pub fn _0x06eaf70ae066441e_safe(
        
        
            p0: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x06EAF70AE066441Eu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x06EAF70AE066441Eu64;
        
        let result = invoke_raw!(
            hash,
                p0
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn _0x06eaf70ae066441e_raw(
        p0: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x06EAF70AE066441Eu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x06EAF70AE066441Eu64;

        invoke_raw_typed!(
            hash,
                p0
        )
    }
}

/// ## Parameters
*



pub fn _playstats_change_mc_emblem_safe(
        
        
            p0: 
        , 
        
        
            p1: 
        , 
        
        
            p2: 
        , 
        
        
            p3: 
        , 
        
        
            p4: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x0A50D2604E05CB94u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x0A50D2604E05CB94u64;
        
        let result = invoke_raw!(
            hash,
                p0, 
                p1, 
                p2, 
                p3, 
                p4
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn _playstats_change_mc_emblem_raw(
        p0: , 
        p1: , 
        p2: , 
        p3: , 
        p4: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x0A50D2604E05CB94u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x0A50D2604E05CB94u64;

        invoke_raw_typed!(
            hash,
                p0, 
                p1, 
                p2, 
                p3, 
                p4
        )
    }
}

/// ```
NativeDB Introduced: v1734
```



pub fn _playstats_casino_slotmachine_safe(
        
        
            p0: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xEF5EC67D392B830Au32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xEF5EC67D392B830Au64;
        
        let result = invoke_raw!(
            hash,
                p0
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn _playstats_casino_slotmachine_raw(
        p0: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xEF5EC67D392B830Au32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xEF5EC67D392B830Au64;

        invoke_raw_typed!(
            hash,
                p0
        )
    }
}

/// ## Parameters
*



pub fn _0xcc25a4553dfbf9ea_safe(
        
        
            p0: 
        , 
        
        
            p1: 
        , 
        
        
            p2: 
        , 
        
        
            p3: 
        , 
        
        
            p4: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xCC25A4553DFBF9EAu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xCC25A4553DFBF9EAu64;
        
        let result = invoke_raw!(
            hash,
                p0, 
                p1, 
                p2, 
                p3, 
                p4
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn _0xcc25a4553dfbf9ea_raw(
        p0: , 
        p1: , 
        p2: , 
        p3: , 
        p4: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xCC25A4553DFBF9EAu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xCC25A4553DFBF9EAu64;

        invoke_raw_typed!(
            hash,
                p0, 
                p1, 
                p2, 
                p3, 
                p4
        )
    }
}

/// ```
NativeDB Added Parameter 7: Any p6
NativeDB Added Parameter 8: Any p7
```



pub fn playstats_crate_drop_mission_done_safe(
        
        
            p0: 
        , 
        
        
            p1: 
        , 
        
        
            p2: 
        , 
        
        
            p3: 
        , 
        
        
            p4: 
        , 
        
        
            p5: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x1CAE5D2E3F9A07F0u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x1CAE5D2E3F9A07F0u64;
        
        let result = invoke_raw!(
            hash,
                p0, 
                p1, 
                p2, 
                p3, 
                p4, 
                p5
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn playstats_crate_drop_mission_done_raw(
        p0: , 
        p1: , 
        p2: , 
        p3: , 
        p4: , 
        p5: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x1CAE5D2E3F9A07F0u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x1CAE5D2E3F9A07F0u64;

        invoke_raw_typed!(
            hash,
                p0, 
                p1, 
                p2, 
                p3, 
                p4, 
                p5
        )
    }
}

/// _0x629526ABA383BCAA native function



pub fn _0x629526aba383bcaa_safe(
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x629526ABA383BCAAu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x629526ABA383BCAAu64;
        
        let result = invoke_raw!(
            hash,
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn _0x629526aba383bcaa_raw(
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x629526ABA383BCAAu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x629526ABA383BCAAu64;

        invoke_raw_typed!(
            hash,
        )
    }
}

/// ## Parameters
*



pub fn _playstats_gunrun_mission_ended_safe(
        
        
            data: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x0EACDF8487D5155Au32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x0EACDF8487D5155Au64;
        
        let result = invoke_raw!(
            hash,
                data
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn _playstats_gunrun_mission_ended_raw(
        data: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x0EACDF8487D5155Au32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x0EACDF8487D5155Au64;

        invoke_raw_typed!(
            hash,
                data
        )
    }
}

/// ## Return value



pub fn _0xe8853fbce7d8d0d6_safe(
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xE8853FBCE7D8D0D6u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xE8853FBCE7D8D0D6u64;
        
        let result = invoke_raw!(
            hash,
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn _0xe8853fbce7d8d0d6_raw(
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xE8853FBCE7D8D0D6u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xE8853FBCE7D8D0D6u64;

        invoke_raw_typed!(
            hash,
        )
    }
}

/// ## Return value



pub fn _0x5a556b229a169402_safe(
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x5A556B229A169402u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x5A556B229A169402u64;
        
        let result = invoke_raw!(
            hash,
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn _0x5a556b229a169402_raw(
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x5A556B229A169402u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x5A556B229A169402u64;

        invoke_raw_typed!(
            hash,
        )
    }
}

/// ## Parameters
*



pub fn leaderboards_write_add_column_long_safe(
        
        
            p0: 
        , 
        
        
            p1: 
        , 
        
        
            p2: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x2E65248609523599u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x2E65248609523599u64;
        
        let result = invoke_raw!(
            hash,
                p0, 
                p1, 
                p2
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn leaderboards_write_add_column_long_raw(
        p0: , 
        p1: , 
        p2: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x2E65248609523599u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x2E65248609523599u64;

        invoke_raw_typed!(
            hash,
                p0, 
                p1, 
                p2
        )
    }
}

/// ## Return value



pub fn _0x6e0a5253375c4584_safe(
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x6E0A5253375C4584u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x6E0A5253375C4584u64;
        
        let result = invoke_raw!(
            hash,
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn _0x6e0a5253375c4584_raw(
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x6E0A5253375C4584u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x6E0A5253375C4584u64;

        invoke_raw_typed!(
            hash,
        )
    }
}

/// ## Parameters
*



pub fn leaderboards2_write_data_for_event_type_safe(
        
        
            p0: 
        , 
        
        
            p1: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xC980E62E33DF1D5Cu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xC980E62E33DF1D5Cu64;
        
        let result = invoke_raw!(
            hash,
                p0, 
                p1
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn leaderboards2_write_data_for_event_type_raw(
        p0: , 
        p1: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xC980E62E33DF1D5Cu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xC980E62E33DF1D5Cu64;

        invoke_raw_typed!(
            hash,
                p0, 
                p1
        )
    }
}

/// ## Parameters
*



pub fn leaderboards_read_clear_safe(
        
        
            p0: 
        , 
        
        
            p1: 
        , 
        
        
            p2: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x7CCE5C737A665701u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x7CCE5C737A665701u64;
        
        let result = invoke_raw!(
            hash,
                p0, 
                p1, 
                p2
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn leaderboards_read_clear_raw(
        p0: , 
        p1: , 
        p2: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x7CCE5C737A665701u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x7CCE5C737A665701u64;

        invoke_raw_typed!(
            hash,
                p0, 
                p1, 
                p2
        )
    }
}

/// ## Parameters
*



pub fn _0x14eda9ee27bd1626_safe(
        
        
            p0: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x14EDA9EE27BD1626u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x14EDA9EE27BD1626u64;
        
        let result = invoke_raw!(
            hash,
                p0
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn _0x14eda9ee27bd1626_raw(
        p0: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x14EDA9EE27BD1626u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x14EDA9EE27BD1626u64;

        invoke_raw_typed!(
            hash,
                p0
        )
    }
}

/// ```
NativeDB Introduced: v2372
```



pub fn _playstats_robbery_prep_safe(
        
        
            p0: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x1A67DFBF1F5C3835u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x1A67DFBF1F5C3835u64;
        
        let result = invoke_raw!(
            hash,
                p0
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn _playstats_robbery_prep_raw(
        p0: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x1A67DFBF1F5C3835u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x1A67DFBF1F5C3835u64;

        invoke_raw_typed!(
            hash,
                p0
        )
    }
}

/// ```
NativeDB Introduced: v2372
```



pub fn _playstats_carclub_challenge_safe(
        
        
            p0: 
        , 
        
        
            p1: 
        , 
        
        
            p2: 
        , 
        
        
            p3: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x1187CB58D7F3BED7u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x1187CB58D7F3BED7u64;
        
        let result = invoke_raw!(
            hash,
                p0, 
                p1, 
                p2, 
                p3
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn _playstats_carclub_challenge_raw(
        p0: , 
        p1: , 
        p2: , 
        p3: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x1187CB58D7F3BED7u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x1187CB58D7F3BED7u64;

        invoke_raw_typed!(
            hash,
                p0, 
                p1, 
                p2, 
                p3
        )
    }
}

/// ## Parameters
*



pub fn stat_get_bool_safe(
        
        
            statHash: 
        , 
        
        
            outValue: 
        , 
        
        
            p2: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x11B5E6D2AE73F48Eu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x11B5E6D2AE73F48Eu64;
        
        let result = invoke_raw!(
            hash,
                statHash, 
                outValue, 
                p2
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn stat_get_bool_raw(
        statHash: , 
        outValue: , 
        p2: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x11B5E6D2AE73F48Eu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x11B5E6D2AE73F48Eu64;

        invoke_raw_typed!(
            hash,
                statHash, 
                outValue, 
                p2
        )
    }
}

/// ```
NativeDB Introduced: v1734
```



pub fn _playstats_collectible_safe(
        
        
            p0: 
        , 
        
        
            p1: 
        , 
        
        
            p2: 
        , 
        
        
            p3: 
        , 
        
        
            p4: 
        , 
        
        
            p5: 
        , 
        
        
            p6: 
        , 
        
        
            p7: 
        , 
        
        
            p8: 
        , 
        
        
            p9: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xCD0A8A9338681CF2u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xCD0A8A9338681CF2u64;
        
        let result = invoke_raw!(
            hash,
                p0, 
                p1, 
                p2, 
                p3, 
                p4, 
                p5, 
                p6, 
                p7, 
                p8, 
                p9
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn _playstats_collectible_raw(
        p0: , 
        p1: , 
        p2: , 
        p3: , 
        p4: , 
        p5: , 
        p6: , 
        p7: , 
        p8: , 
        p9: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xCD0A8A9338681CF2u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xCD0A8A9338681CF2u64;

        invoke_raw_typed!(
            hash,
                p0, 
                p1, 
                p2, 
                p3, 
                p4, 
                p5, 
                p6, 
                p7, 
                p8, 
                p9
        )
    }
}

/// ## Parameters
*



pub fn _0x723c1ce13fbfdb67_safe(
        
        
            p0: 
        , 
        
        
            p1: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x723C1CE13FBFDB67u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x723C1CE13FBFDB67u64;
        
        let result = invoke_raw!(
            hash,
                p0, 
                p1
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn _0x723c1ce13fbfdb67_raw(
        p0: , 
        p1: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x723C1CE13FBFDB67u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x723C1CE13FBFDB67u64;

        invoke_raw_typed!(
            hash,
                p0, 
                p1
        )
    }
}

/// ## Parameters
*



pub fn leaderboards2_read_by_score_float_safe(
        
        
            p0: 
        , 
        
        
            p1: 
        , 
        
        
            p2: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xE662C8B759D08F3Cu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xE662C8B759D08F3Cu64;
        
        let result = invoke_raw!(
            hash,
                p0, 
                p1, 
                p2
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn leaderboards2_read_by_score_float_raw(
        p0: , 
        p1: , 
        p2: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xE662C8B759D08F3Cu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xE662C8B759D08F3Cu64;

        invoke_raw_typed!(
            hash,
                p0, 
                p1, 
                p2
        )
    }
}

/// ```
NativeDB Introduced: v1290
```



pub fn _playstats_h2_fmprep_end_safe(
        
        
            data: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xD8AFB345A9C5CCBBu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xD8AFB345A9C5CCBBu64;
        
        let result = invoke_raw!(
            hash,
                data
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn _playstats_h2_fmprep_end_raw(
        data: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xD8AFB345A9C5CCBBu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xD8AFB345A9C5CCBBu64;

        invoke_raw_typed!(
            hash,
                data
        )
    }
}

/// ## Return value



pub fn _0xba9749cc94c1fd85_safe(
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xBA9749CC94C1FD85u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xBA9749CC94C1FD85u64;
        
        let result = invoke_raw!(
            hash,
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn _0xba9749cc94c1fd85_raw(
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xBA9749CC94C1FD85u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xBA9749CC94C1FD85u64;

        invoke_raw_typed!(
            hash,
        )
    }
}

/// ## Parameters
*



pub fn stat_set_string_safe(
        
        
            statName: 
        , 
        
        
            value: 
        , 
        
        
            save: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xA87B2335D12531D7u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xA87B2335D12531D7u64;
        
        let result = invoke_raw!(
            hash,
                statName, 
                value, 
                save
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn stat_set_string_raw(
        statName: , 
        value: , 
        save: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xA87B2335D12531D7u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xA87B2335D12531D7u64;

        invoke_raw_typed!(
            hash,
                statName, 
                value, 
                save
        )
    }
}

/// ```
NativeDB Introduced: v1734
```



pub fn _playstats_casino_story_mission_ended_safe(
        
        
            p0: 
        , 
        
        
            p1: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xFCCCAC2BD3C1F180u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xFCCCAC2BD3C1F180u64;
        
        let result = invoke_raw!(
            hash,
                p0, 
                p1
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn _playstats_casino_story_mission_ended_raw(
        p0: , 
        p1: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xFCCCAC2BD3C1F180u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xFCCCAC2BD3C1F180u64;

        invoke_raw_typed!(
            hash,
                p0, 
                p1
        )
    }
}

/// ```
NativeDB Introduced: v1290
```



pub fn _playstats_h2_instance_end_safe(
        
        
            data: 
        , 
        
        
            p1: 
        , 
        
        
            p2: 
        , 
        
        
            p3: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x1E1497D0D2108115u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x1E1497D0D2108115u64;
        
        let result = invoke_raw!(
            hash,
                data, 
                p1, 
                p2, 
                p3
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn _playstats_h2_instance_end_raw(
        data: , 
        p1: , 
        p2: , 
        p3: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x1E1497D0D2108115u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x1E1497D0D2108115u64;

        invoke_raw_typed!(
            hash,
                data, 
                p1, 
                p2, 
                p3
        )
    }
}

/// ## Parameters
*



pub fn playstats_rank_up_safe(
        
        
            rank: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xC7F2DE41D102BFB4u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xC7F2DE41D102BFB4u64;
        
        let result = invoke_raw!(
            hash,
                rank
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn playstats_rank_up_raw(
        rank: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xC7F2DE41D102BFB4u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xC7F2DE41D102BFB4u64;

        invoke_raw_typed!(
            hash,
                rank
        )
    }
}

/// ```
NativeDB Introduced: v2372
```



pub fn _playstats_extra_event_safe(
        
        
            p0: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xFA5B74BAB8A7EF99u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xFA5B74BAB8A7EF99u64;
        
        let result = invoke_raw!(
            hash,
                p0
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn _playstats_extra_event_raw(
        p0: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xFA5B74BAB8A7EF99u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xFA5B74BAB8A7EF99u64;

        invoke_raw_typed!(
            hash,
                p0
        )
    }
}

/// ```
Sets profile setting 933  
```



pub fn _0xf1a1803d3476f215_safe(
        
        
            value: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xF1A1803D3476F215u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xF1A1803D3476F215u64;
        
        let result = invoke_raw!(
            hash,
                value
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn _0xf1a1803d3476f215_raw(
        value: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xF1A1803D3476F215u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xF1A1803D3476F215u64;

        invoke_raw_typed!(
            hash,
                value
        )
    }
}

/// ## Parameters
*



pub fn _0xa8733668d1047b51_safe(
        
        
            p0: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xA8733668D1047B51u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xA8733668D1047B51u64;
        
        let result = invoke_raw!(
            hash,
                p0
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn _0xa8733668d1047b51_raw(
        p0: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xA8733668D1047B51u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xA8733668D1047B51u64;

        invoke_raw_typed!(
            hash,
                p0
        )
    }
}

/// ## Parameters
*



pub fn _0x6dee77aff8c21bd1_safe(
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x6DEE77AFF8C21BD1u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x6DEE77AFF8C21BD1u64;
        
        let result = invoke_raw!(
            hash,
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn _0x6dee77aff8c21bd1_raw(
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x6DEE77AFF8C21BD1u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x6DEE77AFF8C21BD1u64;

        invoke_raw_typed!(
            hash,
        )
    }
}

/// ```
NativeDB Introduced: v1868
```



pub fn _0xdfbd93bf2943e29b_safe(
        
        
            p0: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xDFBD93BF2943E29Bu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xDFBD93BF2943E29Bu64;
        
        let result = invoke_raw!(
            hash,
                p0
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn _0xdfbd93bf2943e29b_raw(
        p0: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xDFBD93BF2943E29Bu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xDFBD93BF2943E29Bu64;

        invoke_raw_typed!(
            hash,
                p0
        )
    }
}

/// ```
platformName must be one of the following: ps3, xbox360, ps4, xboxone
```



pub fn _stat_migrate_save_safe(
        
        
            platformName: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xA5C80D8E768A9E66u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xA5C80D8E768A9E66u64;
        
        let result = invoke_raw!(
            hash,
                platformName
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn _stat_migrate_save_raw(
        platformName: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xA5C80D8E768A9E66u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xA5C80D8E768A9E66u64;

        invoke_raw_typed!(
            hash,
                platformName
        )
    }
}

/// ## Parameters
*



pub fn _0x53cae13e9b426993_safe(
        
        
            p0: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x53CAE13E9B426993u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x53CAE13E9B426993u64;
        
        let result = invoke_raw!(
            hash,
                p0
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn _0x53cae13e9b426993_raw(
        p0: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x53CAE13E9B426993u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x53CAE13E9B426993u64;

        invoke_raw_typed!(
            hash,
                p0
        )
    }
}

/// ```
NativeDB Introduced: v1868
```



pub fn _0x2e0259babc27a327_safe(
        
        
            p0: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x2E0259BABC27A327u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x2E0259BABC27A327u64;
        
        let result = invoke_raw!(
            hash,
                p0
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn _0x2e0259babc27a327_raw(
        p0: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x2E0259BABC27A327u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x2E0259BABC27A327u64;

        invoke_raw_typed!(
            hash,
                p0
        )
    }
}

/// ```
NativeDB Introduced: v1604
```



pub fn _playstats_spectator_wheel_spin_safe(
        
        
            p0: 
        , 
        
        
            p1: 
        , 
        
        
            p2: 
        , 
        
        
            p3: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x6731DE84A38BFAD0u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x6731DE84A38BFAD0u64;
        
        let result = invoke_raw!(
            hash,
                p0, 
                p1, 
                p2, 
                p3
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn _playstats_spectator_wheel_spin_raw(
        p0: , 
        p1: , 
        p2: , 
        p3: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x6731DE84A38BFAD0u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x6731DE84A38BFAD0u64;

        invoke_raw_typed!(
            hash,
                p0, 
                p1, 
                p2, 
                p3
        )
    }
}

/// ```
NativeDB Introduced: v1734
```



pub fn _playstats_casino_mission_ended_safe(
        
        
            data: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x1A0D4A6C336B7BC5u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x1A0D4A6C336B7BC5u64;
        
        let result = invoke_raw!(
            hash,
                data
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn _playstats_casino_mission_ended_raw(
        data: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x1A0D4A6C336B7BC5u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x1A0D4A6C336B7BC5u64;

        invoke_raw_typed!(
            hash,
                data
        )
    }
}

/// ## Parameters
*



pub fn _0xdeaaf77eb3687e97_safe(
        
        
            p0: 
        , 
        
        
            p1: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xDEAAF77EB3687E97u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xDEAAF77EB3687E97u64;
        
        let result = invoke_raw!(
            hash,
                p0, 
                p1
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn _0xdeaaf77eb3687e97_raw(
        p0: , 
        p1: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xDEAAF77EB3687E97u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xDEAAF77EB3687E97u64;

        invoke_raw_typed!(
            hash,
                p0, 
                p1
        )
    }
}

/// ```
NativeDB Introduced: v1493
```



pub fn _0x830c3a44eb3f2cf9_safe(
        
        
            p0: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x830C3A44EB3F2CF9u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x830C3A44EB3F2CF9u64;
        
        let result = invoke_raw!(
            hash,
                p0
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn _0x830c3a44eb3f2cf9_raw(
        p0: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x830C3A44EB3F2CF9u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x830C3A44EB3F2CF9u64;

        invoke_raw_typed!(
            hash,
                p0
        )
    }
}

/// ## Parameters
*



pub fn stat_get_number_of_seconds_safe(
        
        
            statName: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x2CE056FF3723F00Bu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x2CE056FF3723F00Bu64;
        
        let result = invoke_raw!(
            hash,
                statName
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn stat_get_number_of_seconds_raw(
        statName: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x2CE056FF3723F00Bu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x2CE056FF3723F00Bu64;

        invoke_raw_typed!(
            hash,
                statName
        )
    }
}

/// ## Parameters
*



pub fn stat_set_block_saves_safe(
        
        
            toggle: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xF434A10BA01C37D0u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xF434A10BA01C37D0u64;
        
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
pub fn stat_set_block_saves_raw(
        toggle: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xF434A10BA01C37D0u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xF434A10BA01C37D0u64;

        invoke_raw_typed!(
            hash,
                toggle
        )
    }
}

/// ## Parameters
*



pub fn _0x1a7ce7cd3e653485_safe(
        
        
            p0: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x1A7CE7CD3E653485u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x1A7CE7CD3E653485u64;
        
        let result = invoke_raw!(
            hash,
                p0
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn _0x1a7ce7cd3e653485_raw(
        p0: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x1A7CE7CD3E653485u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x1A7CE7CD3E653485u64;

        invoke_raw_typed!(
            hash,
                p0
        )
    }
}

/// ## Parameters
*



pub fn leaderboards2_write_data_safe(
        
        
            p0: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xAE2206545888AE49u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xAE2206545888AE49u64;
        
        let result = invoke_raw!(
            hash,
                p0
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn leaderboards2_write_data_raw(
        p0: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xAE2206545888AE49u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xAE2206545888AE49u64;

        invoke_raw_typed!(
            hash,
                p0
        )
    }
}

/// ## Parameters
*



pub fn _playstats_earned_mc_points_safe(
        
        
            p0: 
        , 
        
        
            p1: 
        , 
        
        
            p2: 
        , 
        
        
            p3: 
        , 
        
        
            p4: 
        , 
        
        
            p5: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x501478855A6074CEu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x501478855A6074CEu64;
        
        let result = invoke_raw!(
            hash,
                p0, 
                p1, 
                p2, 
                p3, 
                p4, 
                p5
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn _playstats_earned_mc_points_raw(
        p0: , 
        p1: , 
        p2: , 
        p3: , 
        p4: , 
        p5: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x501478855A6074CEu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x501478855A6074CEu64;

        invoke_raw_typed!(
            hash,
                p0, 
                p1, 
                p2, 
                p3, 
                p4, 
                p5
        )
    }
}

/// ## Parameters
*



pub fn leaderboards2_read_friends_by_row_safe(
        
        
            p0: 
        , 
        
        
            p1: 
        , 
        
        
            p2: 
        , 
        
        
            p3: 
        , 
        
        
            p4: 
        , 
        
        
            p5: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x918B101666F9CB83u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x918B101666F9CB83u64;
        
        let result = invoke_raw!(
            hash,
                p0, 
                p1, 
                p2, 
                p3, 
                p4, 
                p5
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn leaderboards2_read_friends_by_row_raw(
        p0: , 
        p1: , 
        p2: , 
        p3: , 
        p4: , 
        p5: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x918B101666F9CB83u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x918B101666F9CB83u64;

        invoke_raw_typed!(
            hash,
                p0, 
                p1, 
                p2, 
                p3, 
                p4, 
                p5
        )
    }
}

/// ## Parameters
*



pub fn stat_set_masked_int_safe(
        
        
            statName: 
        , 
        
        
            p1: 
        , 
        
        
            p2: 
        , 
        
        
            p3: 
        , 
        
        
            save: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x7BBB1B54583ED410u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x7BBB1B54583ED410u64;
        
        let result = invoke_raw!(
            hash,
                statName, 
                p1, 
                p2, 
                p3, 
                save
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn stat_set_masked_int_raw(
        statName: , 
        p1: , 
        p2: , 
        p3: , 
        save: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x7BBB1B54583ED410u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x7BBB1B54583ED410u64;

        invoke_raw_typed!(
            hash,
                statName, 
                p1, 
                p2, 
                p3, 
                save
        )
    }
}

/// ```
NativeDB Introduced: v1734
```



pub fn _playstats_casino_blackjack_light_safe(
        
        
            p0: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xD5451C7BF151EB6Fu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xD5451C7BF151EB6Fu64;
        
        let result = invoke_raw!(
            hash,
                p0
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn _playstats_casino_blackjack_light_raw(
        p0: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xD5451C7BF151EB6Fu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xD5451C7BF151EB6Fu64;

        invoke_raw_typed!(
            hash,
                p0
        )
    }
}

/// SET_PROFILE_SETTING_PROLOGUE_COMPLETE native function



pub fn set_profile_setting_prologue_complete_safe(
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xB475F27C6A994D65u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xB475F27C6A994D65u64;
        
        let result = invoke_raw!(
            hash,
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn set_profile_setting_prologue_complete_raw(
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xB475F27C6A994D65u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xB475F27C6A994D65u64;

        invoke_raw_typed!(
            hash,
        )
    }
}

/// ## Parameters
*



pub fn _0x6a60e43998228229_safe(
        
        
            p0: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x6A60E43998228229u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x6A60E43998228229u64;
        
        let result = invoke_raw!(
            hash,
                p0
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn _0x6a60e43998228229_raw(
        p0: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x6A60E43998228229u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x6A60E43998228229u64;

        invoke_raw_typed!(
            hash,
                p0
        )
    }
}

/// ```
longest time being ilde?  
```



pub fn playstats_idle_kick_safe(
        
        
            time: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x5DA3A8DE8CB6226Fu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x5DA3A8DE8CB6226Fu64;
        
        let result = invoke_raw!(
            hash,
                time
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn playstats_idle_kick_raw(
        time: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x5DA3A8DE8CB6226Fu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x5DA3A8DE8CB6226Fu64;

        invoke_raw_typed!(
            hash,
                time
        )
    }
}

/// ## Return value



pub fn _0x9ec8858184cd253a_safe(
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x9EC8858184CD253Au32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x9EC8858184CD253Au64;
        
        let result = invoke_raw!(
            hash,
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn _0x9ec8858184cd253a_raw(
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x9EC8858184CD253Au32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x9EC8858184CD253Au64;

        invoke_raw_typed!(
            hash,
        )
    }
}

/// ## Parameters
*



pub fn _playstats_dupe_detection_safe(
        
        
            data: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x848B66100EE33B05u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x848B66100EE33B05u64;
        
        let result = invoke_raw!(
            hash,
                data
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn _playstats_dupe_detection_raw(
        data: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x848B66100EE33B05u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x848B66100EE33B05u64;

        invoke_raw_typed!(
            hash,
                data
        )
    }
}

/// ## Parameters
*



pub fn _0x8c9d11605e59d955_safe(
        
        
            p0: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x8C9D11605E59D955u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x8C9D11605E59D955u64;
        
        let result = invoke_raw!(
            hash,
                p0
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn _0x8c9d11605e59d955_raw(
        p0: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x8C9D11605E59D955u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x8C9D11605E59D955u64;

        invoke_raw_typed!(
            hash,
                p0
        )
    }
}

/// ```
Sets profile setting 935  
```



pub fn _0x55384438fc55ad8e_safe(
        
        
            value: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x55384438FC55AD8Eu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x55384438FC55AD8Eu64;
        
        let result = invoke_raw!(
            hash,
                value
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn _0x55384438fc55ad8e_raw(
        value: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x55384438FC55AD8Eu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x55384438FC55AD8Eu64;

        invoke_raw_typed!(
            hash,
                value
        )
    }
}

/// _0x9B4BD21D69B1E609 native function



pub fn _0x9b4bd21d69b1e609_safe(
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x9B4BD21D69B1E609u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x9B4BD21D69B1E609u64;
        
        let result = invoke_raw!(
            hash,
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn _0x9b4bd21d69b1e609_raw(
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x9B4BD21D69B1E609u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x9B4BD21D69B1E609u64;

        invoke_raw_typed!(
            hash,
        )
    }
}

/// ## Parameters
*



pub fn playstats_mission_over_safe(
        
        
            p0: 
        , 
        
        
            p1: 
        , 
        
        
            p2: 
        , 
        
        
            p3: 
        , 
        
        
            p4: 
        , 
        
        
            p5: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x7C4BB33A8CED7324u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x7C4BB33A8CED7324u64;
        
        let result = invoke_raw!(
            hash,
                p0, 
                p1, 
                p2, 
                p3, 
                p4, 
                p5
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn playstats_mission_over_raw(
        p0: , 
        p1: , 
        p2: , 
        p3: , 
        p4: , 
        p5: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x7C4BB33A8CED7324u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x7C4BB33A8CED7324u64;

        invoke_raw_typed!(
            hash,
                p0, 
                p1, 
                p2, 
                p3, 
                p4, 
                p5
        )
    }
}

/// ## Parameters
*



pub fn _0x678f86d8fc040bdb_safe(
        
        
            p0: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x678F86D8FC040BDBu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x678F86D8FC040BDBu64;
        
        let result = invoke_raw!(
            hash,
                p0
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn _0x678f86d8fc040bdb_raw(
        p0: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x678F86D8FC040BDBu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x678F86D8FC040BDBu64;

        invoke_raw_typed!(
            hash,
                p0
        )
    }
}

/// **This native does absolutely nothing, just a nullsub**

```
NativeDB Introduced: v2372
```



pub fn _0xc01d2470f22cde5a_safe(
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xC01D2470F22CDE5Au32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xC01D2470F22CDE5Au64;
        
        let result = invoke_raw!(
            hash,
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn _0xc01d2470f22cde5a_raw(
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xC01D2470F22CDE5Au32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xC01D2470F22CDE5Au64;

        invoke_raw_typed!(
            hash,
        )
    }
}

/// ## Parameters
*



pub fn _playstats_buy_contraband_safe(
        
        
            data: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xD6781E42755531F7u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xD6781E42755531F7u64;
        
        let result = invoke_raw!(
            hash,
                data
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn _playstats_buy_contraband_raw(
        data: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xD6781E42755531F7u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xD6781E42755531F7u64;

        invoke_raw_typed!(
            hash,
                data
        )
    }
}

/// ## Parameters
*



pub fn _0x28ecb8ac2f607db2_safe(
        
        
            p0: 
        , 
        
        
            p1: 
        , 
        
        
            p2: 
        , 
        
        
            p3: 
        , 
        
        
            p4: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x28ECB8AC2F607DB2u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x28ECB8AC2F607DB2u64;
        
        let result = invoke_raw!(
            hash,
                p0, 
                p1, 
                p2, 
                p3, 
                p4
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn _0x28ecb8ac2f607db2_raw(
        p0: , 
        p1: , 
        p2: , 
        p3: , 
        p4: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x28ECB8AC2F607DB2u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x28ECB8AC2F607DB2u64;

        invoke_raw_typed!(
            hash,
                p0, 
                p1, 
                p2, 
                p3, 
                p4
        )
    }
}

/// ## Parameters
*



pub fn _playstats_copy_rank_into_new_slot_safe(
        
        
            p0: 
        , 
        
        
            p1: 
        , 
        
        
            p2: 
        , 
        
        
            p3: 
        , 
        
        
            p4: 
        , 
        
        
            p5: 
        , 
        
        
            p6: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xB7257BA2550EA10Au32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xB7257BA2550EA10Au64;
        
        let result = invoke_raw!(
            hash,
                p0, 
                p1, 
                p2, 
                p3, 
                p4, 
                p5, 
                p6
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn _playstats_copy_rank_into_new_slot_raw(
        p0: , 
        p1: , 
        p2: , 
        p3: , 
        p4: , 
        p5: , 
        p6: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xB7257BA2550EA10Au32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xB7257BA2550EA10Au64;

        invoke_raw_typed!(
            hash,
                p0, 
                p1, 
                p2, 
                p3, 
                p4, 
                p5, 
                p6
        )
    }
}

/// ## Parameters
*



pub fn _0x1a8ea222f9c67dbb_safe(
        
        
            p0: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x1A8EA222F9C67DBBu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x1A8EA222F9C67DBBu64;
        
        let result = invoke_raw!(
            hash,
                p0
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn _0x1a8ea222f9c67dbb_raw(
        p0: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x1A8EA222F9C67DBBu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x1A8EA222F9C67DBBu64;

        invoke_raw_typed!(
            hash,
                p0
        )
    }
}

/// ## Parameters
*



pub fn _0x8ec74ceb042e7cff_safe(
        
        
            p0: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x8EC74CEB042E7CFFu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x8EC74CEB042E7CFFu64;
        
        let result = invoke_raw!(
            hash,
                p0
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn _0x8ec74ceb042e7cff_raw(
        p0: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x8EC74CEB042E7CFFu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x8EC74CEB042E7CFFu64;

        invoke_raw_typed!(
            hash,
                p0
        )
    }
}

/// ```
NativeDB Introduced: v2189
```



pub fn _0xd6ca58b3b53a0f22_safe(
        
        
            p0: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xD6CA58B3B53A0F22u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xD6CA58B3B53A0F22u64;
        
        let result = invoke_raw!(
            hash,
                p0
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn _0xd6ca58b3b53a0f22_raw(
        p0: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xD6CA58B3B53A0F22u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xD6CA58B3B53A0F22u64;

        invoke_raw_typed!(
            hash,
                p0
        )
    }
}

/// ```
NativeDB Introduced: v1734
```



pub fn _playstats_arcadegame_safe(
        
        
            p0: 
        , 
        
        
            p1: 
        , 
        
        
            p2: 
        , 
        
        
            p3: 
        , 
        
        
            p4: 
        , 
        
        
            p5: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x533A7D1EA58DF958u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x533A7D1EA58DF958u64;
        
        let result = invoke_raw!(
            hash,
                p0, 
                p1, 
                p2, 
                p3, 
                p4, 
                p5
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn _playstats_arcadegame_raw(
        p0: , 
        p1: , 
        p2: , 
        p3: , 
        p4: , 
        p5: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x533A7D1EA58DF958u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x533A7D1EA58DF958u64;

        invoke_raw_typed!(
            hash,
                p0, 
                p1, 
                p2, 
                p3, 
                p4, 
                p5
        )
    }
}

/// _0x98E2BC1CA26287C3 native function



pub fn _0x98e2bc1ca26287c3_safe(
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x98E2BC1CA26287C3u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x98E2BC1CA26287C3u64;
        
        let result = invoke_raw!(
            hash,
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn _0x98e2bc1ca26287c3_raw(
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x98E2BC1CA26287C3u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x98E2BC1CA26287C3u64;

        invoke_raw_typed!(
            hash,
        )
    }
}

/// ```
NativeDB Introduced: v1734
```



pub fn _playstats_passive_mode_safe(
        
        
            p0: 
        , 
        
        
            p1: 
        , 
        
        
            p2: 
        , 
        
        
            p3: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x35EEC6C2BC821A71u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x35EEC6C2BC821A71u64;
        
        let result = invoke_raw!(
            hash,
                p0, 
                p1, 
                p2, 
                p3
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn _playstats_passive_mode_raw(
        p0: , 
        p1: , 
        p2: , 
        p3: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x35EEC6C2BC821A71u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x35EEC6C2BC821A71u64;

        invoke_raw_typed!(
            hash,
                p0, 
                p1, 
                p2, 
                p3
        )
    }
}

/// _0xC141B8917E0017EC native function



pub fn _0xc141b8917e0017ec_safe(
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xC141B8917E0017ECu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xC141B8917E0017ECu64;
        
        let result = invoke_raw!(
            hash,
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn _0xc141b8917e0017ec_raw(
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xC141B8917E0017ECu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xC141B8917E0017ECu64;

        invoke_raw_typed!(
            hash,
        )
    }
}

/// ```
NativeDB Introduced: v1868
```



pub fn _0x810b5fcc52ec7ff0_safe(
        
        
            p0: 
        , 
        
        
            p1: 
        , 
        
        
            p2: 
        , 
        
        
            p3: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x810B5FCC52EC7FF0u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x810B5FCC52EC7FF0u64;
        
        let result = invoke_raw!(
            hash,
                p0, 
                p1, 
                p2, 
                p3
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn _0x810b5fcc52ec7ff0_raw(
        p0: , 
        p1: , 
        p2: , 
        p3: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x810B5FCC52EC7FF0u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x810B5FCC52EC7FF0u64;

        invoke_raw_typed!(
            hash,
                p0, 
                p1, 
                p2, 
                p3
        )
    }
}

/// ```
Example:
 STATS::STAT_SET_BOOL(MISC::GET_HASH_KEY("MPPLY_MELEECHLENGECOMPLETED"), trur, true);
```



pub fn stat_set_bool_safe(
        
        
            statName: 
        , 
        
        
            value: 
        , 
        
        
            save: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x4B33C4243DE0C432u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x4B33C4243DE0C432u64;
        
        let result = invoke_raw!(
            hash,
                statName, 
                value, 
                save
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn stat_set_bool_raw(
        statName: , 
        value: , 
        save: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x4B33C4243DE0C432u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x4B33C4243DE0C432u64;

        invoke_raw_typed!(
            hash,
                statName, 
                value, 
                save
        )
    }
}

/// ```
STATS::0xE3247582(0);  
STATS::0xE3247582(1);  
STATS::0xE3247582(2);  
STATS::0xE3247582(3);  
STATS::0xE3247582(4);  
STATS::0xE3247582(5);  
STATS::0xE3247582(6);  
```



pub fn _0xa78b8fa58200da56_safe(
        
        
            p0: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xA78B8FA58200DA56u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xA78B8FA58200DA56u64;
        
        let result = invoke_raw!(
            hash,
                p0
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn _0xa78b8fa58200da56_raw(
        p0: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xA78B8FA58200DA56u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xA78B8FA58200DA56u64;

        invoke_raw_typed!(
            hash,
                p0
        )
    }
}

/// ## Parameters
*



pub fn _0x88578f6ec36b4a3a_safe(
        
        
            p0: 
        , 
        
        
            p1: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x88578F6EC36B4A3Au32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x88578F6EC36B4A3Au64;
        
        let result = invoke_raw!(
            hash,
                p0, 
                p1
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn _0x88578f6ec36b4a3a_raw(
        p0: , 
        p1: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x88578F6EC36B4A3Au32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x88578F6EC36B4A3Au64;

        invoke_raw_typed!(
            hash,
                p0, 
                p1
        )
    }
}

/// ## Parameters
*



pub fn _playstats_defend_contraband_safe(
        
        
            data: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x2605663BD4F23B5Du32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x2605663BD4F23B5Du64;
        
        let result = invoke_raw!(
            hash,
                data
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn _playstats_defend_contraband_raw(
        data: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x2605663BD4F23B5Du32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x2605663BD4F23B5Du64;

        invoke_raw_typed!(
            hash,
                data
        )
    }
}

/// ## Parameters
*



pub fn _playstats_recover_contraband_safe(
        
        
            data: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x04D90BA8207ADA2Du32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x04D90BA8207ADA2Du64;
        
        let result = invoke_raw!(
            hash,
                data
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn _playstats_recover_contraband_raw(
        data: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x04D90BA8207ADA2Du32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x04D90BA8207ADA2Du64;

        invoke_raw_typed!(
            hash,
                data
        )
    }
}

/// ## Parameters
*



pub fn playstats_leave_job_chain_safe(
        
        
            p0: 
        , 
        
        
            p1: 
        , 
        
        
            p2: 
        , 
        
        
            p3: 
        , 
        
        
            p4: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xC5BE134EC7BA96A0u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xC5BE134EC7BA96A0u64;
        
        let result = invoke_raw!(
            hash,
                p0, 
                p1, 
                p2, 
                p3, 
                p4
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn playstats_leave_job_chain_raw(
        p0: , 
        p1: , 
        p2: , 
        p3: , 
        p4: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xC5BE134EC7BA96A0u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xC5BE134EC7BA96A0u64;

        invoke_raw_typed!(
            hash,
                p0, 
                p1, 
                p2, 
                p3, 
                p4
        )
    }
}

/// ## Return value



pub fn _0x32cac93c9de73d32_safe(
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x32CAC93C9DE73D32u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x32CAC93C9DE73D32u64;
        
        let result = invoke_raw!(
            hash,
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn _0x32cac93c9de73d32_raw(
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x32CAC93C9DE73D32u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x32CAC93C9DE73D32u64;

        invoke_raw_typed!(
            hash,
        )
    }
}

/// ## Parameters
*



pub fn _0x2cd90358f67d0aa8_safe(
        
        
            p0: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x2CD90358F67D0AA8u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x2CD90358F67D0AA8u64;
        
        let result = invoke_raw!(
            hash,
                p0
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn _0x2cd90358f67d0aa8_raw(
        p0: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x2CD90358F67D0AA8u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x2CD90358F67D0AA8u64;

        invoke_raw_typed!(
            hash,
                p0
        )
    }
}

/// _0x71B008056E5692D6 native function



pub fn _0x71b008056e5692d6_safe(
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x71B008056E5692D6u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x71B008056E5692D6u64;
        
        let result = invoke_raw!(
            hash,
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn _0x71b008056e5692d6_raw(
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x71B008056E5692D6u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x71B008056E5692D6u64;

        invoke_raw_typed!(
            hash,
        )
    }
}

/// ```
NativeDB Introduced: v1290
```



pub fn _playstats_dar_mission_end_safe(
        
        
            data: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x0BC254FF3A911501u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x0BC254FF3A911501u64;
        
        let result = invoke_raw!(
            hash,
                data
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn _playstats_dar_mission_end_raw(
        data: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x0BC254FF3A911501u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x0BC254FF3A911501u64;

        invoke_raw_typed!(
            hash,
                data
        )
    }
}

/// ## Parameters
*



pub fn _0xf06a6f41cb445443_safe(
        
        
            p0: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xF06A6F41CB445443u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xF06A6F41CB445443u64;
        
        let result = invoke_raw!(
            hash,
                p0
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn _0xf06a6f41cb445443_raw(
        p0: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xF06A6F41CB445443u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xF06A6F41CB445443u64;

        invoke_raw_typed!(
            hash,
                p0
        )
    }
}

/// ## Return value



pub fn _0xc0e0d686ddfc6eae_safe(
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xC0E0D686DDFC6EAEu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xC0E0D686DDFC6EAEu64;
        
        let result = invoke_raw!(
            hash,
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn _0xc0e0d686ddfc6eae_raw(
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xC0E0D686DDFC6EAEu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xC0E0D686DDFC6EAEu64;

        invoke_raw_typed!(
            hash,
        )
    }
}

/// ## Parameters
*



pub fn _leaderboards_deaths_safe(
        
        
            statName: 
        , 
        
        
            value: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x428EAF89E24F6C36u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x428EAF89E24F6C36u64;
        
        let result = invoke_raw!(
            hash,
                statName, 
                value
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn _leaderboards_deaths_raw(
        statName: , 
        value: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x428EAF89E24F6C36u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x428EAF89E24F6C36u64;

        invoke_raw_typed!(
            hash,
                statName, 
                value
        )
    }
}

/// ## Parameters
*



pub fn _0x14e0b2d1ad1044e0_safe(
        
        
            p0: 
        , 
        
        
            p1: 
        , 
        
        
            p2: 
        , 
        
        
            p3: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x14E0B2D1AD1044E0u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x14E0B2D1AD1044E0u64;
        
        let result = invoke_raw!(
            hash,
                p0, 
                p1, 
                p2, 
                p3
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn _0x14e0b2d1ad1044e0_raw(
        p0: , 
        p1: , 
        p2: , 
        p3: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x14E0B2D1AD1044E0u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x14E0B2D1AD1044E0u64;

        invoke_raw_typed!(
            hash,
                p0, 
                p1, 
                p2, 
                p3
        )
    }
}

/// ```
NativeDB Added Parameter 2: Any p1
NativeDB Introduced: v1493
```



pub fn _0xc14bd9f5337219b2_safe(
        
        
            p0: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xC14BD9F5337219B2u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xC14BD9F5337219B2u64;
        
        let result = invoke_raw!(
            hash,
                p0
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn _0xc14bd9f5337219b2_raw(
        p0: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xC14BD9F5337219B2u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xC14BD9F5337219B2u64;

        invoke_raw_typed!(
            hash,
                p0
        )
    }
}

/// ```
NativeDB Introduced: v2189
```



pub fn _0x4dc416f246a41fc8_safe(
        
        
            p0: 
        , 
        
        
            p1: 
        , 
        
        
            p2: 
        , 
        
        
            p3: 
        , 
        
        
            p4: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x4DC416F246A41FC8u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x4DC416F246A41FC8u64;
        
        let result = invoke_raw!(
            hash,
                p0, 
                p1, 
                p2, 
                p3, 
                p4
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn _0x4dc416f246a41fc8_raw(
        p0: , 
        p1: , 
        p2: , 
        p3: , 
        p4: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x4DC416F246A41FC8u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x4DC416F246A41FC8u64;

        invoke_raw_typed!(
            hash,
                p0, 
                p1, 
                p2, 
                p3, 
                p4
        )
    }
}

/// ```
NativeDB Removed Parameter 4: Any p3
NativeDB Removed Parameter 5: Any p4
NativeDB Removed Parameter 6: Any p5
NativeDB Removed Parameter 7: Any p6
```



pub fn playstats_match_started_safe(
        
        
            p0: 
        , 
        
        
            p1: 
        , 
        
        
            p2: 
        , 
        
        
            p3: 
        , 
        
        
            p4: 
        , 
        
        
            p5: 
        , 
        
        
            p6: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xBC80E22DED931E3Du32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xBC80E22DED931E3Du64;
        
        let result = invoke_raw!(
            hash,
                p0, 
                p1, 
                p2, 
                p3, 
                p4, 
                p5, 
                p6
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn playstats_match_started_raw(
        p0: , 
        p1: , 
        p2: , 
        p3: , 
        p4: , 
        p5: , 
        p6: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xBC80E22DED931E3Du32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xBC80E22DED931E3Du64;

        invoke_raw_typed!(
            hash,
                p0, 
                p1, 
                p2, 
                p3, 
                p4, 
                p5, 
                p6
        )
    }
}

/// ```
NativeDB Introduced: v1493
```



pub fn _0x316db59cd14c1774_safe(
        
        
            p0: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x316DB59CD14C1774u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x316DB59CD14C1774u64;
        
        let result = invoke_raw!(
            hash,
                p0
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn _0x316db59cd14c1774_raw(
        p0: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x316DB59CD14C1774u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x316DB59CD14C1774u64;

        invoke_raw_typed!(
            hash,
                p0
        )
    }
}

/// ```
NativeDB Introduced: v1868
```



pub fn _0x4fcdbd3f0a813c25_safe(
        
        
            p0: 
        , 
        
        
            p1: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x4FCDBD3F0A813C25u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x4FCDBD3F0A813C25u64;
        
        let result = invoke_raw!(
            hash,
                p0, 
                p1
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn _0x4fcdbd3f0a813c25_raw(
        p0: , 
        p1: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x4FCDBD3F0A813C25u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x4FCDBD3F0A813C25u64;

        invoke_raw_typed!(
            hash,
                p0, 
                p1
        )
    }
}

/// ## Parameters
*



pub fn stat_get_number_of_days_safe(
        
        
            statName: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xE0E854F5280FB769u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xE0E854F5280FB769u64;
        
        let result = invoke_raw!(
            hash,
                statName
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn stat_get_number_of_days_raw(
        statName: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xE0E854F5280FB769u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xE0E854F5280FB769u64;

        invoke_raw_typed!(
            hash,
                statName
        )
    }
}

/// ## Parameters
*



pub fn presence_event_updatestat_float_safe(
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x30A6614C1F7799B8u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x30A6614C1F7799B8u64;
        
        let result = invoke_raw!(
            hash,
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn presence_event_updatestat_float_raw(
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x30A6614C1F7799B8u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x30A6614C1F7799B8u64;

        invoke_raw_typed!(
            hash,
        )
    }
}

/// ## Parameters
*



pub fn _0x7033eefd9b28088e_safe(
        
        
            p0: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x7033EEFD9B28088Eu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x7033EEFD9B28088Eu64;
        
        let result = invoke_raw!(
            hash,
                p0
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn _0x7033eefd9b28088e_raw(
        p0: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x7033EEFD9B28088Eu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x7033EEFD9B28088Eu64;

        invoke_raw_typed!(
            hash,
                p0
        )
    }
}

/// STAT_SET_CHEAT_IS_ACTIVE native function



pub fn stat_set_cheat_is_active_safe(
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x047CBED6F6F8B63Cu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x047CBED6F6F8B63Cu64;
        
        let result = invoke_raw!(
            hash,
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn stat_set_cheat_is_active_raw(
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x047CBED6F6F8B63Cu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x047CBED6F6F8B63Cu64;

        invoke_raw_typed!(
            hash,
        )
    }
}

/// ## Parameters
*



pub fn _playstats_sell_contraband_safe(
        
        
            data: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xC729991A9065376Eu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xC729991A9065376Eu64;
        
        let result = invoke_raw!(
            hash,
                data
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn _playstats_sell_contraband_raw(
        data: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xC729991A9065376Eu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xC729991A9065376Eu64;

        invoke_raw_typed!(
            hash,
                data
        )
    }
}

/// ## Parameters
*



pub fn stat_load_pending_safe(
        
        
            p0: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xA1750FFAFA181661u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xA1750FFAFA181661u64;
        
        let result = invoke_raw!(
            hash,
                p0
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn stat_load_pending_raw(
        p0: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xA1750FFAFA181661u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xA1750FFAFA181661u64;

        invoke_raw_typed!(
            hash,
                p0
        )
    }
}

/// ## Parameters
*



pub fn _stat_get_save_migration_consume_content_unlock_status_safe(
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xCE5AA445ABA8DEE0u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xCE5AA445ABA8DEE0u64;
        
        let result = invoke_raw!(
            hash,
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn _stat_get_save_migration_consume_content_unlock_status_raw(
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xCE5AA445ABA8DEE0u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xCE5AA445ABA8DEE0u64;

        invoke_raw_typed!(
            hash,
        )
    }
}

/// ## Parameters
*



pub fn _0xf8c54a461c3e11dc_safe(
        
        
            p0: 
        , 
        
        
            p1: 
        , 
        
        
            p2: 
        , 
        
        
            p3: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xF8C54A461C3E11DCu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xF8C54A461C3E11DCu64;
        
        let result = invoke_raw!(
            hash,
                p0, 
                p1, 
                p2, 
                p3
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn _0xf8c54a461c3e11dc_raw(
        p0: , 
        p1: , 
        p2: , 
        p3: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xF8C54A461C3E11DCu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xF8C54A461C3E11DCu64;

        invoke_raw_typed!(
            hash,
                p0, 
                p1, 
                p2, 
                p3
        )
    }
}

/// ## Parameters
*



pub fn stat_get_masked_int_safe(
        
        
            p0: 
        , 
        
        
            p1: 
        , 
        
        
            p2: 
        , 
        
        
            p3: 
        , 
        
        
            p4: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x655185A06D9EEAABu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x655185A06D9EEAABu64;
        
        let result = invoke_raw!(
            hash,
                p0, 
                p1, 
                p2, 
                p3, 
                p4
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn stat_get_masked_int_raw(
        p0: , 
        p1: , 
        p2: , 
        p3: , 
        p4: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x655185A06D9EEAABu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x655185A06D9EEAABu64;

        invoke_raw_typed!(
            hash,
                p0, 
                p1, 
                p2, 
                p3, 
                p4
        )
    }
}

/// ## Return value



pub fn _0xb1d2bb1e1631f5b1_safe(
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xB1D2BB1E1631F5B1u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xB1D2BB1E1631F5B1u64;
        
        let result = invoke_raw!(
            hash,
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn _0xb1d2bb1e1631f5b1_raw(
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xB1D2BB1E1631F5B1u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xB1D2BB1E1631F5B1u64;

        invoke_raw_typed!(
            hash,
        )
    }
}

/// ```
NativeDB Introduced: v1604
```



pub fn _playstats_arena_wars_ended_safe(
        
        
            data: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xB479D9F0D48A1BC5u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xB479D9F0D48A1BC5u64;
        
        let result = invoke_raw!(
            hash,
                data
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn _playstats_arena_wars_ended_raw(
        data: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xB479D9F0D48A1BC5u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xB479D9F0D48A1BC5u64;

        invoke_raw_typed!(
            hash,
                data
        )
    }
}

/// ```
NativeDB Introduced: v1868
```



pub fn _0x0077f15613d36993_safe(
        
        
            p0: 
        , 
        
        
            p1: 
        , 
        
        
            p2: 
        , 
        
        
            p3: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x0077F15613D36993u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x0077F15613D36993u64;
        
        let result = invoke_raw!(
            hash,
                p0, 
                p1, 
                p2, 
                p3
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn _0x0077f15613d36993_raw(
        p0: , 
        p1: , 
        p2: , 
        p3: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x0077F15613D36993u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x0077F15613D36993u64;

        invoke_raw_typed!(
            hash,
                p0, 
                p1, 
                p2, 
                p3
        )
    }
}

/// ## Parameters
*



pub fn _hired_limo_safe(
        
        
            p0: 
        , 
        
        
            p1: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x792271AB35C356A4u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x792271AB35C356A4u64;
        
        let result = invoke_raw!(
            hash,
                p0, 
                p1
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn _hired_limo_raw(
        p0: , 
        p1: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x792271AB35C356A4u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x792271AB35C356A4u64;

        invoke_raw_typed!(
            hash,
                p0, 
                p1
        )
    }
}

/// ## Parameters
*



pub fn _0x34770b9ce0e03b91_safe(
        
        
            p0: 
        , 
        
        
            p1: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x34770B9CE0E03B91u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x34770B9CE0E03B91u64;
        
        let result = invoke_raw!(
            hash,
                p0, 
                p1
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn _0x34770b9ce0e03b91_raw(
        p0: , 
        p1: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x34770B9CE0E03B91u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x34770B9CE0E03B91u64;

        invoke_raw_typed!(
            hash,
                p0, 
                p1
        )
    }
}

/// ```
NativeDB Introduced: v1868
```



pub fn _0x5bf29846c6527c54_safe(
        
        
            p0: 
        , 
        
        
            p1: 
        , 
        
        
            p2: 
        , 
        
        
            p3: 
        , 
        
        
            p4: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x5BF29846C6527C54u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x5BF29846C6527C54u64;
        
        let result = invoke_raw!(
            hash,
                p0, 
                p1, 
                p2, 
                p3, 
                p4
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn _0x5bf29846c6527c54_raw(
        p0: , 
        p1: , 
        p2: , 
        p3: , 
        p4: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x5BF29846C6527C54u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x5BF29846C6527C54u64;

        invoke_raw_typed!(
            hash,
                p0, 
                p1, 
                p2, 
                p3, 
                p4
        )
    }
}

/// ```
NativeDB Introduced: v2189
```



pub fn _0xc1e963c58664b556_safe(
        
        
            p0: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xC1E963C58664B556u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xC1E963C58664B556u64;
        
        let result = invoke_raw!(
            hash,
                p0
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn _0xc1e963c58664b556_raw(
        p0: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xC1E963C58664B556u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xC1E963C58664B556u64;

        invoke_raw_typed!(
            hash,
                p0
        )
    }
}

/// ```
Example:
for (v_2 = 0; v_2 <= 4; v_2 += 1) {
    STATS::STAT_CLEAR_SLOT_FOR_RELOAD(v_2);
}
```



pub fn stat_clear_slot_for_reload_safe(
        
        
            statSlot: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xEB0A72181D4AA4ADu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xEB0A72181D4AA4ADu64;
        
        let result = invoke_raw!(
            hash,
                statSlot
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn stat_clear_slot_for_reload_raw(
        statSlot: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xEB0A72181D4AA4ADu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xEB0A72181D4AA4ADu64;

        invoke_raw_typed!(
            hash,
                statSlot
        )
    }
}

/// ## Parameters
*



pub fn leaderboards2_read_by_score_int_safe(
        
        
            p0: 
        , 
        
        
            p1: 
        , 
        
        
            p2: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x7EEC7E4F6984A16Au32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x7EEC7E4F6984A16Au64;
        
        let result = invoke_raw!(
            hash,
                p0, 
                p1, 
                p2
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn leaderboards2_read_by_score_int_raw(
        p0: , 
        p1: , 
        p2: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x7EEC7E4F6984A16Au32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x7EEC7E4F6984A16Au64;

        invoke_raw_typed!(
            hash,
                p0, 
                p1, 
                p2
        )
    }
}

/// ```
Sets profile setting 934  
```



pub fn _0x38baaa5dd4c9d19f_safe(
        
        
            value: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x38BAAA5DD4C9D19Fu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x38BAAA5DD4C9D19Fu64;
        
        let result = invoke_raw!(
            hash,
                value
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn _0x38baaa5dd4c9d19f_raw(
        value: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x38BAAA5DD4C9D19Fu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x38BAAA5DD4C9D19Fu64;

        invoke_raw_typed!(
            hash,
                value
        )
    }
}

/// ## Parameters
*



pub fn _0x6551b1f7f6cd46ea_safe(
        
        
            p0: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x6551B1F7F6CD46EAu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x6551B1F7F6CD46EAu64;
        
        let result = invoke_raw!(
            hash,
                p0
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn _0x6551b1f7f6cd46ea_raw(
        p0: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x6551B1F7F6CD46EAu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x6551B1F7F6CD46EAu64;

        invoke_raw_typed!(
            hash,
                p0
        )
    }
}

/// ## Parameters
*



pub fn leaderboards2_read_by_row_safe(
        
        
            p0: 
        , 
        
        
            p1: 
        , 
        
        
            p2: 
        , 
        
        
            p3: 
        , 
        
        
            p4: 
        , 
        
        
            p5: 
        , 
        
        
            p6: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xA9CDB1E3F0A49883u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xA9CDB1E3F0A49883u64;
        
        let result = invoke_raw!(
            hash,
                p0, 
                p1, 
                p2, 
                p3, 
                p4, 
                p5, 
                p6
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn leaderboards2_read_by_row_raw(
        p0: , 
        p1: , 
        p2: , 
        p3: , 
        p4: , 
        p5: , 
        p6: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xA9CDB1E3F0A49883u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xA9CDB1E3F0A49883u64;

        invoke_raw_typed!(
            hash,
                p0, 
                p1, 
                p2, 
                p3, 
                p4, 
                p5, 
                p6
        )
    }
}

/// ## Parameters
*



pub fn stat_get_number_of_minutes_safe(
        
        
            statName: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x7583B4BE4C5A41B5u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x7583B4BE4C5A41B5u64;
        
        let result = invoke_raw!(
            hash,
                statName
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn stat_get_number_of_minutes_raw(
        statName: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x7583B4BE4C5A41B5u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x7583B4BE4C5A41B5u64;

        invoke_raw_typed!(
            hash,
                statName
        )
    }
}

/// ## Parameters
*



pub fn _0xd558bec0bba7e8d2_safe(
        
        
            p0: 
        , 
        
        
            p1: 
        , 
        
        
            p2: 
        , 
        
        
            p3: 
        , 
        
        
            p4: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xD558BEC0BBA7E8D2u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xD558BEC0BBA7E8D2u64;
        
        let result = invoke_raw!(
            hash,
                p0, 
                p1, 
                p2, 
                p3, 
                p4
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn _0xd558bec0bba7e8d2_raw(
        p0: , 
        p1: , 
        p2: , 
        p3: , 
        p4: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xD558BEC0BBA7E8D2u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xD558BEC0BBA7E8D2u64;

        invoke_raw_typed!(
            hash,
                p0, 
                p1, 
                p2, 
                p3, 
                p4
        )
    }
}

/// ## Parameters
*



pub fn _playstats_award_badsport_safe(
        
        
            id: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x47B32F5611E6E483u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x47B32F5611E6E483u64;
        
        let result = invoke_raw!(
            hash,
                id
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn _playstats_award_badsport_raw(
        id: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x47B32F5611E6E483u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x47B32F5611E6E483u64;

        invoke_raw_typed!(
            hash,
                id
        )
    }
}

/// ```
NativeDB Introduced: v1734
```



pub fn _playstats_casino_slotmachine_light_safe(
        
        
            p0: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xE60054A0FAE8227Fu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xE60054A0FAE8227Fu64;
        
        let result = invoke_raw!(
            hash,
                p0
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn _playstats_casino_slotmachine_light_raw(
        p0: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xE60054A0FAE8227Fu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xE60054A0FAE8227Fu64;

        invoke_raw_typed!(
            hash,
                p0
        )
    }
}

/// ```
NativeDB Introduced: v2372
```



pub fn _playstats_carclub_prize_safe(
        
        
            p0: 
        , 
        
        
            p1: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x69C922B677621428u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x69C922B677621428u64;
        
        let result = invoke_raw!(
            hash,
                p0, 
                p1
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn _playstats_carclub_prize_raw(
        p0: , 
        p1: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x69C922B677621428u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x69C922B677621428u64;

        invoke_raw_typed!(
            hash,
                p0, 
                p1
        )
    }
}

/// ## Parameters
*



pub fn _0x7b18da61f6bae9d5_safe(
        
        
            p0: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x7B18DA61F6BAE9D5u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x7B18DA61F6BAE9D5u64;
        
        let result = invoke_raw!(
            hash,
                p0
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn _0x7b18da61f6bae9d5_raw(
        p0: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x7B18DA61F6BAE9D5u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x7B18DA61F6BAE9D5u64;

        invoke_raw_typed!(
            hash,
                p0
        )
    }
}

/// ```
NativeDB Introduced: v1290
```



pub fn _playstats_enter_session_pack_safe(
        
        
            data: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x878FF156D36E9956u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x878FF156D36E9956u64;
        
        let result = invoke_raw!(
            hash,
                data
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn _playstats_enter_session_pack_raw(
        data: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x878FF156D36E9956u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x878FF156D36E9956u64;

        invoke_raw_typed!(
            hash,
                data
        )
    }
}

/// ## Parameters
*



pub fn leaderboards_get_column_type_safe(
        
        
            p0: 
        , 
        
        
            p1: 
        , 
        
        
            p2: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xBF4FEF46DB7894D3u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xBF4FEF46DB7894D3u64;
        
        let result = invoke_raw!(
            hash,
                p0, 
                p1, 
                p2
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn leaderboards_get_column_type_raw(
        p0: , 
        p1: , 
        p2: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xBF4FEF46DB7894D3u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xBF4FEF46DB7894D3u64;

        invoke_raw_typed!(
            hash,
                p0, 
                p1, 
                p2
        )
    }
}

/// ## Parameters
*



pub fn playstats_heist_save_cheat_safe(
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xF4FF020A08BC8863u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xF4FF020A08BC8863u64;
        
        let result = invoke_raw!(
            hash,
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn playstats_heist_save_cheat_raw(
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xF4FF020A08BC8863u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xF4FF020A08BC8863u64;

        invoke_raw_typed!(
            hash,
        )
    }
}

/// ## Parameters
*



pub fn leaderboards_write_add_column_safe(
        
        
            p0: 
        , 
        
        
            p1: 
        , 
        
        
            p2: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x0BCA1D2C47B0D269u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x0BCA1D2C47B0D269u64;
        
        let result = invoke_raw!(
            hash,
                p0, 
                p1, 
                p2
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn leaderboards_write_add_column_raw(
        p0: , 
        p1: , 
        p2: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x0BCA1D2C47B0D269u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x0BCA1D2C47B0D269u64;

        invoke_raw_typed!(
            hash,
                p0, 
                p1, 
                p2
        )
    }
}

/// ```
NativeDB Introduced: v1868
```



pub fn _0x5cdaed54b34b0ed0_safe(
        
        
            p0: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x5CDAED54B34B0ED0u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x5CDAED54B34B0ED0u64;
        
        let result = invoke_raw!(
            hash,
                p0
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn _0x5cdaed54b34b0ed0_raw(
        p0: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x5CDAED54B34B0ED0u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x5CDAED54B34B0ED0u64;

        invoke_raw_typed!(
            hash,
                p0
        )
    }
}

/// ## Return value



pub fn stat_save_pending_safe(
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x7D3A583856F2C5ACu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x7D3A583856F2C5ACu64;
        
        let result = invoke_raw!(
            hash,
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn stat_save_pending_raw(
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x7D3A583856F2C5ACu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x7D3A583856F2C5ACu64;

        invoke_raw_typed!(
            hash,
        )
    }
}

/// ## Parameters
*



pub fn _0xd1c9b92bdd3f151d_safe(
        
        
            p0: 
        , 
        
        
            p1: 
        , 
        
        
            p2: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xD1C9B92BDD3F151Du32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xD1C9B92BDD3F151Du64;
        
        let result = invoke_raw!(
            hash,
                p0, 
                p1, 
                p2
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn _0xd1c9b92bdd3f151d_raw(
        p0: , 
        p1: , 
        p2: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xD1C9B92BDD3F151Du32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xD1C9B92BDD3F151Du64;

        invoke_raw_typed!(
            hash,
                p0, 
                p1, 
                p2
        )
    }
}

/// ## Return value



pub fn stat_save_pending_or_requested_safe(
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xBBB6AD006F1BBEA3u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xBBB6AD006F1BBEA3u64;
        
        let result = invoke_raw!(
            hash,
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn stat_save_pending_or_requested_raw(
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xBBB6AD006F1BBEA3u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xBBB6AD006F1BBEA3u64;

        invoke_raw_typed!(
            hash,
        )
    }
}

/// ## Parameters
*



pub fn stat_get_number_of_hours_safe(
        
        
            statName: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xF2D4B2FE415AAFC3u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xF2D4B2FE415AAFC3u64;
        
        let result = invoke_raw!(
            hash,
                statName
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn stat_get_number_of_hours_raw(
        statName: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xF2D4B2FE415AAFC3u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xF2D4B2FE415AAFC3u64;

        invoke_raw_typed!(
            hash,
                statName
        )
    }
}

/// ## Parameters
*



pub fn _0xf11f01d98113536a_safe(
        
        
            p0: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xF11F01D98113536Au32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xF11F01D98113536Au64;
        
        let result = invoke_raw!(
            hash,
                p0
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn _0xf11f01d98113536a_raw(
        p0: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xF11F01D98113536Au32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xF11F01D98113536Au64;

        invoke_raw_typed!(
            hash,
                p0
        )
    }
}

/// ## Parameters
*



pub fn get_packed_tu_int_stat_key_safe(
        
        
            index: 
        , 
        
        
            spStat: 
        , 
        
        
            charStat: 
        , 
        
        
            character: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xD16C2AD6B8E32854u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xD16C2AD6B8E32854u64;
        
        let result = invoke_raw!(
            hash,
                index, 
                spStat, 
                charStat, 
                character
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn get_packed_tu_int_stat_key_raw(
        index: , 
        spStat: , 
        charStat: , 
        character: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xD16C2AD6B8E32854u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xD16C2AD6B8E32854u64;

        invoke_raw_typed!(
            hash,
                index, 
                spStat, 
                charStat, 
                character
        )
    }
}

/// ## Parameters
*



pub fn _0x60eedc12af66e846_safe(
        
        
            p0: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x60EEDC12AF66E846u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x60EEDC12AF66E846u64;
        
        let result = invoke_raw!(
            hash,
                p0
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn _0x60eedc12af66e846_raw(
        p0: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x60EEDC12AF66E846u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x60EEDC12AF66E846u64;

        invoke_raw_typed!(
            hash,
                p0
        )
    }
}

/// ## Parameters
*



pub fn _0xbaa2f0490e146be8_safe(
        
        
            p0: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xBAA2F0490E146BE8u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xBAA2F0490E146BE8u64;
        
        let result = invoke_raw!(
            hash,
                p0
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn _0xbaa2f0490e146be8_raw(
        p0: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xBAA2F0490E146BE8u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xBAA2F0490E146BE8u64;

        invoke_raw_typed!(
            hash,
                p0
        )
    }
}

/// ## Parameters
*



pub fn _0x0d01d20616fc73fb_safe(
        
        
            p0: 
        , 
        
        
            p1: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x0D01D20616FC73FBu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x0D01D20616FC73FBu64;
        
        let result = invoke_raw!(
            hash,
                p0, 
                p1
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn _0x0d01d20616fc73fb_raw(
        p0: , 
        p1: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x0D01D20616FC73FBu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x0D01D20616FC73FBu64;

        invoke_raw_typed!(
            hash,
                p0, 
                p1
        )
    }
}

/// ## Parameters
*



pub fn playstats_quickfix_tool_safe(
        
        
            element: 
        , 
        
        
            item: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x90D0622866E80445u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x90D0622866E80445u64;
        
        let result = invoke_raw!(
            hash,
                element, 
                item
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn playstats_quickfix_tool_raw(
        element: , 
        item: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x90D0622866E80445u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x90D0622866E80445u64;

        invoke_raw_typed!(
            hash,
                element, 
                item
        )
    }
}

/// ## Parameters
*



pub fn _playstats_spent_pi_custom_loadout_safe(
        
        
            amount: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xBE509B0A3693DE8Bu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xBE509B0A3693DE8Bu64;
        
        let result = invoke_raw!(
            hash,
                amount
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn _playstats_spent_pi_custom_loadout_raw(
        amount: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xBE509B0A3693DE8Bu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xBE509B0A3693DE8Bu64;

        invoke_raw_typed!(
            hash,
                amount
        )
    }
}

/// LEADERBOARDS_CLEAR_CACHE_DATA native function



pub fn leaderboards_clear_cache_data_safe(
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xD4B02A6B476E1FDCu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xD4B02A6B476E1FDCu64;
        
        let result = invoke_raw!(
            hash,
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn leaderboards_clear_cache_data_raw(
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xD4B02A6B476E1FDCu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xD4B02A6B476E1FDCu64;

        invoke_raw_typed!(
            hash,
        )
    }
}

/// ## Parameters
*



pub fn leaderboards_get_cache_number_of_rows_safe(
        
        
            p0: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x58A651CD201D89ADu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x58A651CD201D89ADu64;
        
        let result = invoke_raw!(
            hash,
                p0
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn leaderboards_get_cache_number_of_rows_raw(
        p0: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x58A651CD201D89ADu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x58A651CD201D89ADu64;

        invoke_raw_typed!(
            hash,
                p0
        )
    }
}

/// ## Parameters
*



pub fn leaderboards_read_pending_safe(
        
        
            p0: 
        , 
        
        
            p1: 
        , 
        
        
            p2: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xAC392C8483342AC2u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xAC392C8483342AC2u64;
        
        let result = invoke_raw!(
            hash,
                p0, 
                p1, 
                p2
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn leaderboards_read_pending_raw(
        p0: , 
        p1: , 
        p2: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xAC392C8483342AC2u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xAC392C8483342AC2u64;

        invoke_raw_typed!(
            hash,
                p0, 
                p1, 
                p2
        )
    }
}

/// ## Parameters
*



pub fn leaderboards2_read_rank_prediction_safe(
        
        
            p0: 
        , 
        
        
            p1: 
        , 
        
        
            p2: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xC38DC1E90D22547Cu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xC38DC1E90D22547Cu64;
        
        let result = invoke_raw!(
            hash,
                p0, 
                p1, 
                p2
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn leaderboards2_read_rank_prediction_raw(
        p0: , 
        p1: , 
        p2: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xC38DC1E90D22547Cu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xC38DC1E90D22547Cu64;

        invoke_raw_typed!(
            hash,
                p0, 
                p1, 
                p2
        )
    }
}

/// ```
Needs more research. Possibly used to calculate the "mask" when calling "STAT_SET_MASKED_INT"?  
```



pub fn _stat_get_packed_int_mask_safe(
        
        
            p0: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x94F12ABF9C79E339u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x94F12ABF9C79E339u64;
        
        let result = invoke_raw!(
            hash,
                p0
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn _stat_get_packed_int_mask_raw(
        p0: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x94F12ABF9C79E339u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x94F12ABF9C79E339u64;

        invoke_raw_typed!(
            hash,
                p0
        )
    }
}

/// ## Parameters
*



pub fn playstats_job_bend_safe(
        
        
            p0: 
        , 
        
        
            p1: 
        , 
        
        
            p2: 
        , 
        
        
            p3: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xF5BB8DAC426A52C0u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xF5BB8DAC426A52C0u64;
        
        let result = invoke_raw!(
            hash,
                p0, 
                p1, 
                p2, 
                p3
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn playstats_job_bend_raw(
        p0: , 
        p1: , 
        p2: , 
        p3: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xF5BB8DAC426A52C0u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xF5BB8DAC426A52C0u64;

        invoke_raw_typed!(
            hash,
                p0, 
                p1, 
                p2, 
                p3
        )
    }
}

/// ```
NativeDB Introduced: v1180
```



pub fn _playstats_smug_mission_ended_safe(
        
        
            data: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x320C35147D5B5DDDu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x320C35147D5B5DDDu64;
        
        let result = invoke_raw!(
            hash,
                data
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn _playstats_smug_mission_ended_raw(
        data: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x320C35147D5B5DDDu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x320C35147D5B5DDDu64;

        invoke_raw_typed!(
            hash,
                data
        )
    }
}

/// ## Parameters
*



pub fn get_packed_int_stat_key_safe(
        
        
            index: 
        , 
        
        
            spStat: 
        , 
        
        
            charStat: 
        , 
        
        
            character: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x61E111E323419E07u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x61E111E323419E07u64;
        
        let result = invoke_raw!(
            hash,
                index, 
                spStat, 
                charStat, 
                character
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn get_packed_int_stat_key_raw(
        index: , 
        spStat: , 
        charStat: , 
        character: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x61E111E323419E07u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x61E111E323419E07u64;

        invoke_raw_typed!(
            hash,
                index, 
                spStat, 
                charStat, 
                character
        )
    }
}

/// ## Parameters
*



pub fn leaderboards_get_cache_data_row_safe(
        
        
            p0: 
        , 
        
        
            p1: 
        , 
        
        
            p2: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x9120E8DBA3D69273u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x9120E8DBA3D69273u64;
        
        let result = invoke_raw!(
            hash,
                p0, 
                p1, 
                p2
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn leaderboards_get_cache_data_row_raw(
        p0: , 
        p1: , 
        p2: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x9120E8DBA3D69273u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x9120E8DBA3D69273u64;

        invoke_raw_typed!(
            hash,
                p0, 
                p1, 
                p2
        )
    }
}

/// ## Parameters
*



pub fn playstats_crate_created_safe(
        
        
            p0: 
        , 
        
        
            p1: 
        , 
        
        
            p2: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xAFC7E5E075A96F46u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xAFC7E5E075A96F46u64;
        
        let result = invoke_raw!(
            hash,
                p0, 
                p1, 
                p2
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn playstats_crate_created_raw(
        p0: , 
        p1: , 
        p2: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xAFC7E5E075A96F46u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xAFC7E5E075A96F46u64;

        invoke_raw_typed!(
            hash,
                p0, 
                p1, 
                p2
        )
    }
}

/// ## Return value



pub fn _0xa943fd1722e11efd_safe(
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xA943FD1722E11EFDu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xA943FD1722E11EFDu64;
        
        let result = invoke_raw!(
            hash,
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn _0xa943fd1722e11efd_raw(
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xA943FD1722E11EFDu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xA943FD1722E11EFDu64;

        invoke_raw_typed!(
            hash,
        )
    }
}

/// ## Parameters
*



pub fn _leaderboards2_read_by_platform_safe(
        
        
            p0: 
        , 
        
        
            gamerHandleCsv: 
        , 
        
        
            platformName: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xF1AE5DCDBFCA2721u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xF1AE5DCDBFCA2721u64;
        
        let result = invoke_raw!(
            hash,
                p0, 
                gamerHandleCsv, 
                platformName
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn _leaderboards2_read_by_platform_raw(
        p0: , 
        gamerHandleCsv: , 
        platformName: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xF1AE5DCDBFCA2721u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xF1AE5DCDBFCA2721u64;

        invoke_raw_typed!(
            hash,
                p0, 
                gamerHandleCsv, 
                platformName
        )
    }
}

/// ```
Needs more research. Seems to return "STAT_UNKNOWN" if no such user id exists.  
```



pub fn stat_get_user_id_safe(
        
        
            p0: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x2365C388E393BBE2u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x2365C388E393BBE2u64;
        
        let result = invoke_raw!(
            hash,
                p0
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn stat_get_user_id_raw(
        p0: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x2365C388E393BBE2u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x2365C388E393BBE2u64;

        invoke_raw_typed!(
            hash,
                p0
        )
    }
}

/// ## Parameters
*



pub fn stat_get_float_safe(
        
        
            statHash: 
        , 
        
        
            outValue: 
        , 
        
        
            p2: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xD7AE6C9C9C6AC54Cu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xD7AE6C9C9C6AC54Cu64;
        
        let result = invoke_raw!(
            hash,
                statHash, 
                outValue, 
                p2
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn stat_get_float_raw(
        statHash: , 
        outValue: , 
        p2: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xD7AE6C9C9C6AC54Cu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xD7AE6C9C9C6AC54Cu64;

        invoke_raw_typed!(
            hash,
                statHash, 
                outValue, 
                p2
        )
    }
}

/// ## Parameters
*



pub fn leaderboards_get_cache_exists_safe(
        
        
            p0: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x9C51349BE6CDFE2Cu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x9C51349BE6CDFE2Cu64;
        
        let result = invoke_raw!(
            hash,
                p0
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn leaderboards_get_cache_exists_raw(
        p0: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x9C51349BE6CDFE2Cu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x9C51349BE6CDFE2Cu64;

        invoke_raw_typed!(
            hash,
                p0
        )
    }
}

/// ## Parameters
*



pub fn _0x6bc0acd0673acebe_safe(
        
        
            p0: 
        , 
        
        
            valueA: 
        , 
        
        
            valueB: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x6BC0ACD0673ACEBEu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x6BC0ACD0673ACEBEu64;
        
        let result = invoke_raw!(
            hash,
                p0, 
                valueA, 
                valueB
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn _0x6bc0acd0673acebe_raw(
        p0: , 
        valueA: , 
        valueB: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x6BC0ACD0673ACEBEu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x6BC0ACD0673ACEBEu64;

        invoke_raw_typed!(
            hash,
                p0, 
                valueA, 
                valueB
        )
    }
}

/// ## Parameters
*



pub fn playstats_random_mission_done_safe(
        
        
            name: 
        , 
        
        
            p1: 
        , 
        
        
            p2: 
        , 
        
        
            p3: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x71862B1D855F32E1u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x71862B1D855F32E1u64;
        
        let result = invoke_raw!(
            hash,
                name, 
                p1, 
                p2, 
                p3
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn playstats_random_mission_done_raw(
        name: , 
        p1: , 
        p2: , 
        p3: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x71862B1D855F32E1u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x71862B1D855F32E1u64;

        invoke_raw_typed!(
            hash,
                name, 
                p1, 
                p2, 
                p3
        )
    }
}

/// ## Parameters
*



pub fn _ordered_boss_vehicle_safe(
        
        
            p0: 
        , 
        
        
            p1: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xCEA553E35C2246E1u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xCEA553E35C2246E1u64;
        
        let result = invoke_raw!(
            hash,
                p0, 
                p1
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn _ordered_boss_vehicle_raw(
        p0: , 
        p1: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xCEA553E35C2246E1u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xCEA553E35C2246E1u64;

        invoke_raw_typed!(
            hash,
                p0, 
                p1
        )
    }
}

/// ## Parameters
*



pub fn _0x96e6d5150dbf1c09_safe(
        
        
            p0: 
        , 
        
        
            p1: 
        , 
        
        
            p2: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x96E6D5150DBF1C09u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x96E6D5150DBF1C09u64;
        
        let result = invoke_raw!(
            hash,
                p0, 
                p1, 
                p2
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn _0x96e6d5150dbf1c09_raw(
        p0: , 
        p1: , 
        p2: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x96E6D5150DBF1C09u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x96E6D5150DBF1C09u64;

        invoke_raw_typed!(
            hash,
                p0, 
                p1, 
                p2
        )
    }
}

/// ## Parameters
*



pub fn _0x282b6739644f4347_safe(
        
        
            p0: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x282B6739644F4347u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x282B6739644F4347u64;
        
        let result = invoke_raw!(
            hash,
                p0
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn _0x282b6739644f4347_raw(
        p0: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x282B6739644F4347u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x282B6739644F4347u64;

        invoke_raw_typed!(
            hash,
                p0
        )
    }
}

/// ## Return value



pub fn _0xa761d4ac6115623d_safe(
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xA761D4AC6115623Du32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xA761D4AC6115623Du64;
        
        let result = invoke_raw!(
            hash,
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn _0xa761d4ac6115623d_raw(
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xA761D4AC6115623Du32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xA761D4AC6115623Du64;

        invoke_raw_typed!(
            hash,
        )
    }
}

/// ## Parameters
*



pub fn _0xe3261d791eb44acb_safe(
        
        
            p0: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xE3261D791EB44ACBu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xE3261D791EB44ACBu64;
        
        let result = invoke_raw!(
            hash,
                p0
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn _0xe3261d791eb44acb_raw(
        p0: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xE3261D791EB44ACBu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xE3261D791EB44ACBu64;

        invoke_raw_typed!(
            hash,
                p0
        )
    }
}

/// ```
NativeDB Introduced: v2189
```



pub fn _0xdfcdb14317a9b361_safe(
        
        
            p0: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xDFCDB14317A9B361u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xDFCDB14317A9B361u64;
        
        let result = invoke_raw!(
            hash,
                p0
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn _0xdfcdb14317a9b361_raw(
        p0: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xDFCDB14317A9B361u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xDFCDB14317A9B361u64;

        invoke_raw_typed!(
            hash,
                p0
        )
    }
}

/// ## Return value



pub fn _0x8b9cdbd6c566c38c_safe(
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x8B9CDBD6C566C38Cu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x8B9CDBD6C566C38Cu64;
        
        let result = invoke_raw!(
            hash,
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn _0x8b9cdbd6c566c38c_raw(
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x8B9CDBD6C566C38Cu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x8B9CDBD6C566C38Cu64;

        invoke_raw_typed!(
            hash,
        )
    }
}

/// ## Parameters
*



pub fn playstats_cloth_change_safe(
        
        
            p0: 
        , 
        
        
            p1: 
        , 
        
        
            p2: 
        , 
        
        
            p3: 
        , 
        
        
            p4: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x34B973047A2268B9u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x34B973047A2268B9u64;
        
        let result = invoke_raw!(
            hash,
                p0, 
                p1, 
                p2, 
                p3, 
                p4
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn playstats_cloth_change_raw(
        p0: , 
        p1: , 
        p2: , 
        p3: , 
        p4: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x34B973047A2268B9u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x34B973047A2268B9u64;

        invoke_raw_typed!(
            hash,
                p0, 
                p1, 
                p2, 
                p3, 
                p4
        )
    }
}

/// ## Parameters
*



pub fn playstats_cheat_applied_safe(
        
        
            cheat: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x6058665D72302D3Fu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x6058665D72302D3Fu64;
        
        let result = invoke_raw!(
            hash,
                cheat
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn playstats_cheat_applied_raw(
        cheat: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x6058665D72302D3Fu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x6058665D72302D3Fu64;

        invoke_raw_typed!(
            hash,
                cheat
        )
    }
}

/// ## Parameters
*



pub fn _playstats_pi_menu_hide_settings_safe(
        
        
            data: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x203B381133817079u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x203B381133817079u64;
        
        let result = invoke_raw!(
            hash,
                data
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn _playstats_pi_menu_hide_settings_raw(
        data: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x203B381133817079u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x203B381133817079u64;

        invoke_raw_typed!(
            hash,
                data
        )
    }
}

/// ## Parameters
*



pub fn _0x419615486bbf1956_safe(
        
        
            p0: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x419615486BBF1956u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x419615486BBF1956u64;
        
        let result = invoke_raw!(
            hash,
                p0
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn _0x419615486bbf1956_raw(
        p0: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x419615486BBF1956u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x419615486BBF1956u64;

        invoke_raw_typed!(
            hash,
                p0
        )
    }
}

/// ## Parameters
*



pub fn leaderboards_get_cache_time_safe(
        
        
            p0: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xF04C1C27DA35F6C8u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xF04C1C27DA35F6C8u64;
        
        let result = invoke_raw!(
            hash,
                p0
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn leaderboards_get_cache_time_raw(
        p0: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xF04C1C27DA35F6C8u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xF04C1C27DA35F6C8u64;

        invoke_raw_typed!(
            hash,
                p0
        )
    }
}

/// ## Parameters
*



pub fn leaderboards_get_column_id_safe(
        
        
            p0: 
        , 
        
        
            p1: 
        , 
        
        
            p2: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xC4B5467A1886EA7Eu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xC4B5467A1886EA7Eu64;
        
        let result = invoke_raw!(
            hash,
                p0, 
                p1, 
                p2
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn leaderboards_get_column_id_raw(
        p0: , 
        p1: , 
        p2: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xC4B5467A1886EA7Eu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xC4B5467A1886EA7Eu64;

        invoke_raw_typed!(
            hash,
                p0, 
                p1, 
                p2
        )
    }
}

/// ```
Sets profile setting 501
```



pub fn _set_save_migration_transaction_id_safe(
        
        
            transactionId: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xF6792800AC95350Du32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xF6792800AC95350Du64;
        
        let result = invoke_raw!(
            hash,
                transactionId
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn _set_save_migration_transaction_id_raw(
        transactionId: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xF6792800AC95350Du32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xF6792800AC95350Du64;

        invoke_raw_typed!(
            hash,
                transactionId
        )
    }
}

/// ## Parameters
*



pub fn stat_set_bool_masked_safe(
        
        
            statName: 
        , 
        
        
            value: 
        , 
        
        
            mask: 
        , 
        
        
            save: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x5BC62EC1937B9E5Bu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x5BC62EC1937B9E5Bu64;
        
        let result = invoke_raw!(
            hash,
                statName, 
                value, 
                mask, 
                save
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn stat_set_bool_masked_raw(
        statName: , 
        value: , 
        mask: , 
        save: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x5BC62EC1937B9E5Bu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x5BC62EC1937B9E5Bu64;

        invoke_raw_typed!(
            hash,
                statName, 
                value, 
                mask, 
                save
        )
    }
}

/// ## Parameters
*



pub fn playstats_ros_bet_safe(
        
        
            amount: 
        , 
        
        
            act: 
        , 
        
        
            player: 
        , 
        
        
            cm: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x121FB4DDDC2D5291u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x121FB4DDDC2D5291u64;
        
        let result = invoke_raw!(
            hash,
                amount, 
                act, 
                player, 
                cm
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn playstats_ros_bet_raw(
        amount: , 
        act: , 
        player: , 
        cm: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x121FB4DDDC2D5291u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x121FB4DDDC2D5291u64;

        invoke_raw_typed!(
            hash,
                amount, 
                act, 
                player, 
                cm
        )
    }
}

/// ## Return value



pub fn _0x84a810b375e69c0e_safe(
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x84A810B375E69C0Eu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x84A810B375E69C0Eu64;
        
        let result = invoke_raw!(
            hash,
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn _0x84a810b375e69c0e_raw(
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x84A810B375E69C0Eu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x84A810B375E69C0Eu64;

        invoke_raw_typed!(
            hash,
        )
    }
}

/// ```
NativeDB Introduced: v2699
```



pub fn _playstats_inventory_safe(
        
        
            p0: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x887DAD63CF5B7908u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x887DAD63CF5B7908u64;
        
        let result = invoke_raw!(
            hash,
                p0
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn _playstats_inventory_raw(
        p0: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x887DAD63CF5B7908u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x887DAD63CF5B7908u64;

        invoke_raw_typed!(
            hash,
                p0
        )
    }
}

/// ```
NativeDB Introduced: v2060
```



pub fn _0x6a7f19756f1a9016_safe(
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x6A7F19756F1A9016u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x6A7F19756F1A9016u64;
        
        let result = invoke_raw!(
            hash,
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn _0x6a7f19756f1a9016_raw(
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x6A7F19756F1A9016u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x6A7F19756F1A9016u64;

        invoke_raw_typed!(
            hash,
        )
    }
}

/// ```
NativeDB Introduced: v1734
```



pub fn _playstats_casino_roulette_safe(
        
        
            p0: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x95101C443A84E7F1u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x95101C443A84E7F1u64;
        
        let result = invoke_raw!(
            hash,
                p0
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn _playstats_casino_roulette_raw(
        p0: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x95101C443A84E7F1u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x95101C443A84E7F1u64;

        invoke_raw_typed!(
            hash,
                p0
        )
    }
}

/// ```
NativeDB Introduced: v1868
```



pub fn _0x92fc0eedfac04a14_safe(
        
        
            p0: 
        , 
        
        
            p1: 
        , 
        
        
            p2: 
        , 
        
        
            p3: 
        , 
        
        
            p4: 
        , 
        
        
            p5: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x92FC0EEDFAC04A14u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x92FC0EEDFAC04A14u64;
        
        let result = invoke_raw!(
            hash,
                p0, 
                p1, 
                p2, 
                p3, 
                p4, 
                p5
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn _0x92fc0eedfac04a14_raw(
        p0: , 
        p1: , 
        p2: , 
        p3: , 
        p4: , 
        p5: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x92FC0EEDFAC04A14u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x92FC0EEDFAC04A14u64;

        invoke_raw_typed!(
            hash,
                p0, 
                p1, 
                p2, 
                p3, 
                p4, 
                p5
        )
    }
}

/// ```
example from completionpercentage_controller.ysc.c4
if (STATS::_5EAD2BF6484852E4()) {
            MISC::SET_BIT(g_17b95._f20df._ff10, 15);
            STATS::_11FF1C80276097ED(0xe9ec4dd1, 200, 0);
        }
```



pub fn _0x5ead2bf6484852e4_safe(
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x5EAD2BF6484852E4u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x5EAD2BF6484852E4u64;
        
        let result = invoke_raw!(
            hash,
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn _0x5ead2bf6484852e4_raw(
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x5EAD2BF6484852E4u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x5EAD2BF6484852E4u64;

        invoke_raw_typed!(
            hash,
        )
    }
}

/// ## Parameters
*



pub fn stat_delete_slot_safe(
        
        
            p0: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x49A49BED12794D70u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x49A49BED12794D70u64;
        
        let result = invoke_raw!(
            hash,
                p0
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn stat_delete_slot_raw(
        p0: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x49A49BED12794D70u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x49A49BED12794D70u64;

        invoke_raw_typed!(
            hash,
                p0
        )
    }
}

/// ## Parameters
*



pub fn _0x930f504203f561c9_safe(
        
        
            p0: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x930F504203F561C9u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x930F504203F561C9u64;
        
        let result = invoke_raw!(
            hash,
                p0
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn _0x930f504203f561c9_raw(
        p0: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x930F504203F561C9u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x930F504203F561C9u64;

        invoke_raw_typed!(
            hash,
                p0
        )
    }
}

/// ## Parameters
*



pub fn playstats_mission_started_safe(
        
        
            p0: 
        , 
        
        
            p1: 
        , 
        
        
            p2: 
        , 
        
        
            p3: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xC19A2925C34D2231u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xC19A2925C34D2231u64;
        
        let result = invoke_raw!(
            hash,
                p0, 
                p1, 
                p2, 
                p3
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn playstats_mission_started_raw(
        p0: , 
        p1: , 
        p2: , 
        p3: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xC19A2925C34D2231u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xC19A2925C34D2231u64;

        invoke_raw_typed!(
            hash,
                p0, 
                p1, 
                p2, 
                p3
        )
    }
}

/// ```
'value' is a structure to a structure, 'numFields' is how many fields there are in said structure (usually 7).  
The structure looks like this:  
int year  
int month  
int day  
int hour  
int minute  
int second  
int millisecond  
The decompiled scripts use TIME::GET_POSIX_TIME to fill this structure.  
```



pub fn stat_set_date_safe(
        
        
            statName: 
        , 
        
        
            value: 
        , 
        
        
            numFields: 
        , 
        
        
            save: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x2C29BFB64F4FCBE4u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x2C29BFB64F4FCBE4u64;
        
        let result = invoke_raw!(
            hash,
                statName, 
                value, 
                numFields, 
                save
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn stat_set_date_raw(
        statName: , 
        value: , 
        numFields: , 
        save: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x2C29BFB64F4FCBE4u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x2C29BFB64F4FCBE4u64;

        invoke_raw_typed!(
            hash,
                statName, 
                value, 
                numFields, 
                save
        )
    }
}

/// ## Parameters
*



pub fn _0xbed9f5693f34ed17_safe(
        
        
            statName: 
        , 
        
        
            p1: 
        , 
        
        
            outValue: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xBED9F5693F34ED17u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xBED9F5693F34ED17u64;
        
        let result = invoke_raw!(
            hash,
                statName, 
                p1, 
                outValue
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn _0xbed9f5693f34ed17_raw(
        statName: , 
        p1: , 
        outValue: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xBED9F5693F34ED17u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xBED9F5693F34ED17u64;

        invoke_raw_typed!(
            hash,
                statName, 
                p1, 
                outValue
        )
    }
}

/// ```
NativeDB Introduced: v1868
```



pub fn _0xc03fab2c2f92289b_safe(
        
        
            p0: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xC03FAB2C2F92289Bu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xC03FAB2C2F92289Bu64;
        
        let result = invoke_raw!(
            hash,
                p0
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn _0xc03fab2c2f92289b_raw(
        p0: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xC03FAB2C2F92289Bu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xC03FAB2C2F92289Bu64;

        invoke_raw_typed!(
            hash,
                p0
        )
    }
}

/// ```
p2 appears to always be -1  
```



pub fn stat_get_int_safe(
        
        
            statHash: 
        , 
        
        
            outValue: 
        , 
        
        
            p2: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x767FBC2AC802EF3Du32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x767FBC2AC802EF3Du64;
        
        let result = invoke_raw!(
            hash,
                statHash, 
                outValue, 
                p2
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn stat_get_int_raw(
        statHash: , 
        outValue: , 
        p2: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x767FBC2AC802EF3Du32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x767FBC2AC802EF3Du64;

        invoke_raw_typed!(
            hash,
                statHash, 
                outValue, 
                p2
        )
    }
}

/// ```
p1 always true.  
```



pub fn stat_set_current_posix_time_safe(
        
        
            statName: 
        , 
        
        
            p1: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xC2F84B7F9C4D0C61u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xC2F84B7F9C4D0C61u64;
        
        let result = invoke_raw!(
            hash,
                statName, 
                p1
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn stat_set_current_posix_time_raw(
        statName: , 
        p1: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xC2F84B7F9C4D0C61u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xC2F84B7F9C4D0C61u64;

        invoke_raw_typed!(
            hash,
                statName, 
                p1
        )
    }
}

/// ## Return value



pub fn _0x0b8b7f74bf061c6d_safe(
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x0B8B7F74BF061C6Du32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x0B8B7F74BF061C6Du64;
        
        let result = invoke_raw!(
            hash,
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn _0x0b8b7f74bf061c6d_raw(
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x0B8B7F74BF061C6Du32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x0B8B7F74BF061C6Du64;

        invoke_raw_typed!(
            hash,
        )
    }
}

/// ## Parameters
*



pub fn stat_set_user_id_safe(
        
        
            statName: 
        , 
        
        
            value: 
        , 
        
        
            save: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x8CDDF1E452BABE11u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x8CDDF1E452BABE11u64;
        
        let result = invoke_raw!(
            hash,
                statName, 
                value, 
                save
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn stat_set_user_id_raw(
        statName: , 
        value: , 
        save: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x8CDDF1E452BABE11u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x8CDDF1E452BABE11u64;

        invoke_raw_typed!(
            hash,
                statName, 
                value, 
                save
        )
    }
}

/// Sets profile setting 939

```
NativeDB Introduced: v1734
```



pub fn _0xc67e2da1cbe759e2_safe(
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xC67E2DA1CBE759E2u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xC67E2DA1CBE759E2u64;
        
        let result = invoke_raw!(
            hash,
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn _0xc67e2da1cbe759e2_raw(
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xC67E2DA1CBE759E2u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xC67E2DA1CBE759E2u64;

        invoke_raw_typed!(
            hash,
        )
    }
}

/// _0xC847B43F369AC0B5 native function



pub fn _0xc847b43f369ac0b5_safe(
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xC847B43F369AC0B5u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xC847B43F369AC0B5u64;
        
        let result = invoke_raw!(
            hash,
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn _0xc847b43f369ac0b5_raw(
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xC847B43F369AC0B5u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xC847B43F369AC0B5u64;

        invoke_raw_typed!(
            hash,
        )
    }
}

/// ## Parameters
*



pub fn _0xd1a1ee3b4fa8e760_safe(
        
        
            p0: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xD1A1EE3B4FA8E760u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xD1A1EE3B4FA8E760u64;
        
        let result = invoke_raw!(
            hash,
                p0
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn _0xd1a1ee3b4fa8e760_raw(
        p0: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xD1A1EE3B4FA8E760u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xD1A1EE3B4FA8E760u64;

        invoke_raw_typed!(
            hash,
                p0
        )
    }
}

/// ## Return value



pub fn _0xbe3db208333d9844_safe(
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xBE3DB208333D9844u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xBE3DB208333D9844u64;
        
        let result = invoke_raw!(
            hash,
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn _0xbe3db208333d9844_raw(
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xBE3DB208333D9844u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xBE3DB208333D9844u64;

        invoke_raw_typed!(
            hash,
        )
    }
}

/// ```
Needs more research. Possibly used to calculate the "mask" when calling "STAT_SET_BOOL_MASKED"?  
```



pub fn _stat_get_packed_bool_mask_safe(
        
        
            p0: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xF4D8E7AC2A27758Cu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xF4D8E7AC2A27758Cu64;
        
        let result = invoke_raw!(
            hash,
                p0
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn _stat_get_packed_bool_mask_raw(
        p0: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xF4D8E7AC2A27758Cu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xF4D8E7AC2A27758Cu64;

        invoke_raw_typed!(
            hash,
                p0
        )
    }
}

/// ## Return value



pub fn stat_save_migration_status_start_safe(
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xC70DDCE56D0D3A99u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xC70DDCE56D0D3A99u64;
        
        let result = invoke_raw!(
            hash,
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn stat_save_migration_status_start_raw(
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xC70DDCE56D0D3A99u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xC70DDCE56D0D3A99u64;

        invoke_raw_typed!(
            hash,
        )
    }
}

/// ## Parameters
*



pub fn _0x33d72899e24c3365_safe(
        
        
            p0: 
        , 
        
        
            p1: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x33D72899E24C3365u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x33D72899E24C3365u64;
        
        let result = invoke_raw!(
            hash,
                p0, 
                p1
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn _0x33d72899e24c3365_raw(
        p0: , 
        p1: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x33D72899E24C3365u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x33D72899E24C3365u64;

        invoke_raw_typed!(
            hash,
                p0, 
                p1
        )
    }
}

/// ## Parameters
*



pub fn _0x7d8ba05688ad64c7_safe(
        
        
            p0: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x7D8BA05688AD64C7u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x7D8BA05688AD64C7u64;
        
        let result = invoke_raw!(
            hash,
                p0
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn _0x7d8ba05688ad64c7_raw(
        p0: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x7D8BA05688AD64C7u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x7D8BA05688AD64C7u64;

        invoke_raw_typed!(
            hash,
                p0
        )
    }
}

/// ## Return value



pub fn _0xc6e0e2616a7576bb_safe(
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xC6E0E2616A7576BBu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xC6E0E2616A7576BBu64;
        
        let result = invoke_raw!(
            hash,
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn _0xc6e0e2616a7576bb_raw(
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xC6E0E2616A7576BBu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xC6E0E2616A7576BBu64;

        invoke_raw_typed!(
            hash,
        )
    }
}

/// ## Parameters
*



pub fn stat_set_int_safe(
        
        
            statName: 
        , 
        
        
            value: 
        , 
        
        
            save: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xB3271D7AB655B441u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xB3271D7AB655B441u64;
        
        let result = invoke_raw!(
            hash,
                statName, 
                value, 
                save
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn stat_set_int_raw(
        statName: , 
        value: , 
        save: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xB3271D7AB655B441u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xB3271D7AB655B441u64;

        invoke_raw_typed!(
            hash,
                statName, 
                value, 
                save
        )
    }
}

/// ```
STAT_SET_*
```



pub fn _0x5688585e6d563cd8_safe(
        
        
            p0: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x5688585E6D563CD8u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x5688585E6D563CD8u64;
        
        let result = invoke_raw!(
            hash,
                p0
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn _0x5688585e6d563cd8_raw(
        p0: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x5688585E6D563CD8u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x5688585E6D563CD8u64;

        invoke_raw_typed!(
            hash,
                p0
        )
    }
}

/// ## Parameters
*



pub fn _0x7f2c4cdf2e82df4c_safe(
        
        
            p0: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x7F2C4CDF2E82DF4Cu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x7F2C4CDF2E82DF4Cu64;
        
        let result = invoke_raw!(
            hash,
                p0
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn _0x7f2c4cdf2e82df4c_raw(
        p0: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x7F2C4CDF2E82DF4Cu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x7F2C4CDF2E82DF4Cu64;

        invoke_raw_typed!(
            hash,
                p0
        )
    }
}

/// ## Parameters
*



pub fn _0x5ff2c33b13a02a11_safe(
        
        
            p0: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x5FF2C33B13A02A11u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x5FF2C33B13A02A11u64;
        
        let result = invoke_raw!(
            hash,
                p0
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn _0x5ff2c33b13a02a11_raw(
        p0: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x5FF2C33B13A02A11u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x5FF2C33B13A02A11u64;

        invoke_raw_typed!(
            hash,
                p0
        )
    }
}

/// ## Parameters
*



pub fn presence_event_updatestat_int_safe(
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x11FF1C80276097EDu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x11FF1C80276097EDu64;
        
        let result = invoke_raw!(
            hash,
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn presence_event_updatestat_int_raw(
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x11FF1C80276097EDu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x11FF1C80276097EDu64;

        invoke_raw_typed!(
            hash,
        )
    }
}

/// ```
NativeDB Introduced: v2372
```



pub fn _playstats_carclub_points_safe(
        
        
            p0: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xFF14D6FEEC507BBEu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xFF14D6FEEC507BBEu64;
        
        let result = invoke_raw!(
            hash,
                p0
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn _playstats_carclub_points_raw(
        p0: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xFF14D6FEEC507BBEu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xFF14D6FEEC507BBEu64;

        invoke_raw_typed!(
            hash,
                p0
        )
    }
}

/// ## Parameters
*



pub fn playstats_acquired_hidden_package_safe(
        
        
            p0: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x79AB33F0FBFAC40Cu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x79AB33F0FBFAC40Cu64;
        
        let result = invoke_raw!(
            hash,
                p0
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn playstats_acquired_hidden_package_raw(
        p0: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x79AB33F0FBFAC40Cu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x79AB33F0FBFAC40Cu64;

        invoke_raw_typed!(
            hash,
                p0
        )
    }
}

/// ## Parameters
*



pub fn playstats_hold_up_mission_done_safe(
        
        
            p0: 
        , 
        
        
            p1: 
        , 
        
        
            p2: 
        , 
        
        
            p3: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xCB00196B31C39EB1u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xCB00196B31C39EB1u64;
        
        let result = invoke_raw!(
            hash,
                p0, 
                p1, 
                p2, 
                p3
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn playstats_hold_up_mission_done_raw(
        p0: , 
        p1: , 
        p2: , 
        p3: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xCB00196B31C39EB1u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xCB00196B31C39EB1u64;

        invoke_raw_typed!(
            hash,
                p0, 
                p1, 
                p2, 
                p3
        )
    }
}

/// ```
Needs more research. Gets the stat name of a masked int?
section - values used in the decompiled scripts:
"_NGPSTAT_INT"
"_MP_NGPSTAT_INT"
"_MP_LRPSTAT_INT"
"_MP_APAPSTAT_INT"
"_MP_LR2PSTAT_INT"
"_MP_BIKEPSTAT_INT"
"_MP_IMPEXPPSTAT_INT"
"_MP_GUNRPSTAT_INT"
"_NGDLCPSTAT_INT"
"_MP_NGDLCPSTAT_INT"
"_DLCSMUGCHARPSTAT_INT"
"_GANGOPSPSTAT_INT"
"_BUSINESSBATPSTAT_INT"
"_ARENAWARSPSTAT_INT"
"_CASINOPSTAT_INT"
"_CASINOHSTPSTAT_INT"
```



pub fn _get_ngstat_int_hash_safe(
        
        
            index: 
        , 
        
        
            spStat: 
        , 
        
        
            charStat: 
        , 
        
        
            character: 
        , 
        
        
            section: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x2B4CDCA6F07FF3DAu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x2B4CDCA6F07FF3DAu64;
        
        let result = invoke_raw!(
            hash,
                index, 
                spStat, 
                charStat, 
                character, 
                section
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn _get_ngstat_int_hash_raw(
        index: , 
        spStat: , 
        charStat: , 
        character: , 
        section: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x2B4CDCA6F07FF3DAu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x2B4CDCA6F07FF3DAu64;

        invoke_raw_typed!(
            hash,
                index, 
                spStat, 
                charStat, 
                character, 
                section
        )
    }
}

/// ## Parameters
*



pub fn playstats_award_xp_safe(
        
        
            amount: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x46F917F6B4128FE4u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x46F917F6B4128FE4u64;
        
        let result = invoke_raw!(
            hash,
                amount
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn playstats_award_xp_raw(
        amount: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x46F917F6B4128FE4u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x46F917F6B4128FE4u64;

        invoke_raw_typed!(
            hash,
                amount
        )
    }
}

/// ## Parameters
*



pub fn _0xedbf6c9b0d2c65c8_safe(
        
        
            p0: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xEDBF6C9B0D2C65C8u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xEDBF6C9B0D2C65C8u64;
        
        let result = invoke_raw!(
            hash,
                p0
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn _0xedbf6c9b0d2c65c8_raw(
        p0: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xEDBF6C9B0D2C65C8u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xEDBF6C9B0D2C65C8u64;

        invoke_raw_typed!(
            hash,
                p0
        )
    }
}

/// ```
STAT_LOAD_*
```



pub fn _0xecb41ac6ab754401_safe(
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xECB41AC6AB754401u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xECB41AC6AB754401u64;
        
        let result = invoke_raw!(
            hash,
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn _0xecb41ac6ab754401_raw(
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xECB41AC6AB754401u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xECB41AC6AB754401u64;

        invoke_raw_typed!(
            hash,
        )
    }
}

/// ## Return value



pub fn _stat_get_cancel_save_migration_status_safe(
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x567384DFA67029E6u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x567384DFA67029E6u64;
        
        let result = invoke_raw!(
            hash,
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn _stat_get_cancel_save_migration_status_raw(
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x567384DFA67029E6u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x567384DFA67029E6u64;

        invoke_raw_typed!(
            hash,
        )
    }
}

/// ## Parameters
*



pub fn playstats_race_checkpoint_safe(
        
        
            p0: 
        , 
        
        
            p1: 
        , 
        
        
            p2: 
        , 
        
        
            p3: 
        , 
        
        
            p4: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x9C375C315099DDE4u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x9C375C315099DDE4u64;
        
        let result = invoke_raw!(
            hash,
                p0, 
                p1, 
                p2, 
                p3, 
                p4
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn playstats_race_checkpoint_raw(
        p0: , 
        p1: , 
        p2: , 
        p3: , 
        p4: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x9C375C315099DDE4u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x9C375C315099DDE4u64;

        invoke_raw_typed!(
            hash,
                p0, 
                p1, 
                p2, 
                p3, 
                p4
        )
    }
}

/// ## Parameters
*



pub fn playstats_activity_done_safe(
        
        
            p0: 
        , 
        
        
            p1: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xA071E0ED98F91286u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xA071E0ED98F91286u64;
        
        let result = invoke_raw!(
            hash,
                p0, 
                p1
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn playstats_activity_done_raw(
        p0: , 
        p1: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xA071E0ED98F91286u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xA071E0ED98F91286u64;

        invoke_raw_typed!(
            hash,
                p0, 
                p1
        )
    }
}

/// ```
NativeDB Introduced: v1604
```



pub fn _playstats_arena_war_spectator_safe(
        
        
            p0: 
        , 
        
        
            p1: 
        , 
        
        
            p2: 
        , 
        
        
            p3: 
        , 
        
        
            p4: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x6F4F599753F8200Au32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x6F4F599753F8200Au64;
        
        let result = invoke_raw!(
            hash,
                p0, 
                p1, 
                p2, 
                p3, 
                p4
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn _playstats_arena_war_spectator_raw(
        p0: , 
        p1: , 
        p2: , 
        p3: , 
        p4: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x6F4F599753F8200Au32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x6F4F599753F8200Au64;

        invoke_raw_typed!(
            hash,
                p0, 
                p1, 
                p2, 
                p3, 
                p4
        )
    }
}

/// ```
NativeDB Introduced: v1493
```



pub fn _playstats_drone_usage_safe(
        
        
            p0: 
        , 
        
        
            p1: 
        , 
        
        
            p2: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x66C7BB2416ED3FCEu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x66C7BB2416ED3FCEu64;
        
        let result = invoke_raw!(
            hash,
                p0, 
                p1, 
                p2
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn _playstats_drone_usage_raw(
        p0: , 
        p1: , 
        p2: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x66C7BB2416ED3FCEu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x66C7BB2416ED3FCEu64;

        invoke_raw_typed!(
            hash,
                p0, 
                p1, 
                p2
        )
    }
}

/// ## Parameters
*



pub fn _0xbfafdb5faaa5c5ab_safe(
        
        
            p0: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xBFAFDB5FAAA5C5ABu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xBFAFDB5FAAA5C5ABu64;
        
        let result = invoke_raw!(
            hash,
                p0
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn _0xbfafdb5faaa5c5ab_raw(
        p0: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xBFAFDB5FAAA5C5ABu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xBFAFDB5FAAA5C5ABu64;

        invoke_raw_typed!(
            hash,
                p0
        )
    }
}

/// ```
Example:
 STATS::STAT_SET_FLOAT(MISC::GET_HASH_KEY("MP0_WEAPON_ACCURACY"), 66.6f, true);
```



pub fn stat_set_float_safe(
        
        
            statName: 
        , 
        
        
            value: 
        , 
        
        
            save: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x4851997F37FE9B3Cu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x4851997F37FE9B3Cu64;
        
        let result = invoke_raw!(
            hash,
                statName, 
                value, 
                save
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn stat_set_float_raw(
        statName: , 
        value: , 
        save: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x4851997F37FE9B3Cu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x4851997F37FE9B3Cu64;

        invoke_raw_typed!(
            hash,
                statName, 
                value, 
                save
        )
    }
}

/// ## Parameters
*



pub fn _0xe496a53ba5f50a56_safe(
        
        
            p0: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xE496A53BA5F50A56u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xE496A53BA5F50A56u64;
        
        let result = invoke_raw!(
            hash,
                p0
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn _0xe496a53ba5f50a56_raw(
        p0: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xE496A53BA5F50A56u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xE496A53BA5F50A56u64;

        invoke_raw_typed!(
            hash,
                p0
        )
    }
}

/// ## Parameters
*



pub fn stat_increment_safe(
        
        
            statName: 
        , 
        
        
            value: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x9B5A68C6489E9909u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x9B5A68C6489E9909u64;
        
        let result = invoke_raw!(
            hash,
                statName, 
                value
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn stat_increment_raw(
        statName: , 
        value: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x9B5A68C6489E9909u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x9B5A68C6489E9909u64;

        invoke_raw_typed!(
            hash,
                statName, 
                value
        )
    }
}

/// ## Parameters
*



pub fn _0x8d8adb562f09a245_safe(
        
        
            p0: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x8D8ADB562F09A245u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x8D8ADB562F09A245u64;
        
        let result = invoke_raw!(
            hash,
                p0
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn _0x8d8adb562f09a245_raw(
        p0: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x8D8ADB562F09A245u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x8D8ADB562F09A245u64;

        invoke_raw_typed!(
            hash,
                p0
        )
    }
}

/// ## Parameters
*



pub fn leaderboards2_read_by_handle_safe(
        
        
            p0: 
        , 
        
        
            p1: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xC30713A383BFBF0Eu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xC30713A383BFBF0Eu64;
        
        let result = invoke_raw!(
            hash,
                p0, 
                p1
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn leaderboards2_read_by_handle_raw(
        p0: , 
        p1: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xC30713A383BFBF0Eu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xC30713A383BFBF0Eu64;

        invoke_raw_typed!(
            hash,
                p0, 
                p1
        )
    }
}

/// ## Return value



pub fn _0x55a8becaf28a4eb7_safe(
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x55A8BECAF28A4EB7u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x55A8BECAF28A4EB7u64;
        
        let result = invoke_raw!(
            hash,
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn _0x55a8becaf28a4eb7_raw(
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x55A8BECAF28A4EB7u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x55A8BECAF28A4EB7u64;

        invoke_raw_typed!(
            hash,
        )
    }
}

/// Allows stunts to be triggered and sent as a `CEventNetworkStuntPerformed` event.

Event types are shown below:

```c
enum eTrackedStuntType
{
    ST_FRONTFLIP = 0,
    ST_BACKFLIP = 1,
    ST_SPIN = 2,
    ST_WHEELIE = 3,
    ST_STOPPIE = 4,
    ST_BOWLING_PIN = 5,
    ST_FOOTBALL = 6,
    ST_ROLL = 7
};
```



pub fn playstats_start_tracking_stunts_safe(
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x928DBFB892638EF3u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x928DBFB892638EF3u64;
        
        let result = invoke_raw!(
            hash,
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn playstats_start_tracking_stunts_raw(
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x928DBFB892638EF3u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x928DBFB892638EF3u64;

        invoke_raw_typed!(
            hash,
        )
    }
}

/// ## Parameters
*



pub fn _0x44919cc079bb60bf_safe(
        
        
            p0: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x44919CC079BB60BFu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x44919CC079BB60BFu64;
        
        let result = invoke_raw!(
            hash,
                p0
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn _0x44919cc079bb60bf_raw(
        p0: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x44919CC079BB60BFu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x44919CC079BB60BFu64;

        invoke_raw_typed!(
            hash,
                p0
        )
    }
}

/// **This native does absolutely nothing, just a nullsub**



pub fn playstats_oddjob_done_safe(
        
        
            p0: 
        , 
        
        
            p1: 
        , 
        
        
            p2: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x69DEA3E9DB727B4Cu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x69DEA3E9DB727B4Cu64;
        
        let result = invoke_raw!(
            hash,
                p0, 
                p1, 
                p2
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn playstats_oddjob_done_raw(
        p0: , 
        p1: , 
        p2: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x69DEA3E9DB727B4Cu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x69DEA3E9DB727B4Cu64;

        invoke_raw_typed!(
            hash,
                p0, 
                p1, 
                p2
        )
    }
}

/// ## Return value



pub fn _0x4c89fe2bdeb3f169_safe(
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x4C89FE2BDEB3F169u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x4C89FE2BDEB3F169u64;
        
        let result = invoke_raw!(
            hash,
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn _0x4c89fe2bdeb3f169_raw(
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x4C89FE2BDEB3F169u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x4C89FE2BDEB3F169u64;

        invoke_raw_typed!(
            hash,
        )
    }
}

/// ## Parameters
*



pub fn stat_load_safe(
        
        
            p0: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xA651443F437B1CE6u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xA651443F437B1CE6u64;
        
        let result = invoke_raw!(
            hash,
                p0
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn stat_load_raw(
        p0: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xA651443F437B1CE6u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xA651443F437B1CE6u64;

        invoke_raw_typed!(
            hash,
                p0
        )
    }
}

/// ## Parameters
*



pub fn stat_get_date_safe(
        
        
            statHash: 
        , 
        
        
            p1: 
        , 
        
        
            p2: 
        , 
        
        
            p3: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x8B0FACEFC36C824Bu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x8B0FACEFC36C824Bu64;
        
        let result = invoke_raw!(
            hash,
                statHash, 
                p1, 
                p2, 
                p3
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn stat_get_date_raw(
        statHash: , 
        p1: , 
        p2: , 
        p3: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x8B0FACEFC36C824Bu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x8B0FACEFC36C824Bu64;

        invoke_raw_typed!(
            hash,
                statHash, 
                p1, 
                p2, 
                p3
        )
    }
}

/// ```
NativeDB Introduced: v1868
```



pub fn _0x4aff7e02e485e92b_safe(
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x4AFF7E02E485E92Bu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x4AFF7E02E485E92Bu64;
        
        let result = invoke_raw!(
            hash,
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn _0x4aff7e02e485e92b_raw(
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x4AFF7E02E485E92Bu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x4AFF7E02E485E92Bu64;

        invoke_raw_typed!(
            hash,
        )
    }
}

/// ```
NativeDB Introduced: v1734
```



pub fn _playstats_casino_threecardpoker_light_safe(
        
        
            p0: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xC9001364B4388F22u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xC9001364B4388F22u64;
        
        let result = invoke_raw!(
            hash,
                p0
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn _playstats_casino_threecardpoker_light_raw(
        p0: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xC9001364B4388F22u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xC9001364B4388F22u64;

        invoke_raw_typed!(
            hash,
                p0
        )
    }
}

/// ## Parameters
*



pub fn _0x7d36291161859389_safe(
        
        
            p0: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x7D36291161859389u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x7D36291161859389u64;
        
        let result = invoke_raw!(
            hash,
                p0
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn _0x7d36291161859389_raw(
        p0: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x7D36291161859389u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x7D36291161859389u64;

        invoke_raw_typed!(
            hash,
                p0
        )
    }
}

/// ```
NativeDB Introduced: v1734
```



pub fn _playstats_casino_chip_safe(
        
        
            p0: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x0999F3F090EC5012u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x0999F3F090EC5012u64;
        
        let result = invoke_raw!(
            hash,
                p0
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn _playstats_casino_chip_raw(
        p0: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x0999F3F090EC5012u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x0999F3F090EC5012u64;

        invoke_raw_typed!(
            hash,
                p0
        )
    }
}

