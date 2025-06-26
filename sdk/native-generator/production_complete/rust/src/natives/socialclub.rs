//! SOCIALCLUB native functions
//! 
//! Functions for the socialclub category (local)

use crate::types::*;
use crate::errors::{NativeError, NativeResult, validation};
use nalgebra::Vector3;
use tracing::{debug, warn};

/// ```
NativeDB Introduced: v323
```



pub fn sc_gamerdata_get_float_safe(
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xA770C8EEC6FB2AC5u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xA770C8EEC6FB2AC5u64;
        
        let result = invoke_raw!(
            hash,
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn sc_gamerdata_get_float_raw(
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xA770C8EEC6FB2AC5u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xA770C8EEC6FB2AC5u64;

        invoke_raw_typed!(
            hash,
        )
    }
}

/// ## Parameters
*



pub fn _sc_inbox_message_get_bounty_data_safe(
        
        
            index: 
        , 
        
        
            outData: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x87E0052F08BD64E6u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x87E0052F08BD64E6u64;
        
        let result = invoke_raw!(
            hash,
                index, 
                outData
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn _sc_inbox_message_get_bounty_data_raw(
        index: , 
        outData: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x87E0052F08BD64E6u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x87E0052F08BD64E6u64;

        invoke_raw_typed!(
            hash,
                index, 
                outData
        )
    }
}

/// ## Parameters
*



pub fn _0xfe4c1d0d3b9cc17e_safe(
        
        
            p0: 
        , 
        
        
            p1: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xFE4C1D0D3B9CC17Eu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xFE4C1D0D3B9CC17Eu64;
        
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
pub fn _0xfe4c1d0d3b9cc17e_raw(
        p0: , 
        p1: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xFE4C1D0D3B9CC17Eu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xFE4C1D0D3B9CC17Eu64;

        invoke_raw_typed!(
            hash,
                p0, 
                p1
        )
    }
}

/// ```
NativeDB Added Parameter 1: char* p0
NativeDB Added Parameter 2: int* p1
NativeDB Introduced: v323
```



pub fn _0x710bcda8071eded1_safe(
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x710BCDA8071EDED1u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x710BCDA8071EDED1u64;
        
        let result = invoke_raw!(
            hash,
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn _0x710bcda8071eded1_raw(
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x710BCDA8071EDED1u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x710BCDA8071EDED1u64;

        invoke_raw_typed!(
            hash,
        )
    }
}

/// ## Parameters
*



pub fn sc_presence_attr_set_string_safe(
        
        
            attrHash: 
        , 
        
        
            value: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x287F1F75D2803595u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x287F1F75D2803595u64;
        
        let result = invoke_raw!(
            hash,
                attrHash, 
                value
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn sc_presence_attr_set_string_raw(
        attrHash: , 
        value: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x287F1F75D2803595u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x287F1F75D2803595u64;

        invoke_raw_typed!(
            hash,
                attrHash, 
                value
        )
    }
}

/// ## Parameters
*



pub fn _0x116fb94dc4b79f17_safe(
        
        
            p0: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x116FB94DC4B79F17u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x116FB94DC4B79F17u64;
        
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
pub fn _0x116fb94dc4b79f17_raw(
        p0: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x116FB94DC4B79F17u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x116FB94DC4B79F17u64;

        invoke_raw_typed!(
            hash,
                p0
        )
    }
}

/// ```
NativeDB Added Parameter 1: char* p0
NativeDB Added Parameter 2: float* p1
NativeDB Introduced: v323
```



pub fn _0x50a8a36201dbf83e_safe(
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x50A8A36201DBF83Eu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x50A8A36201DBF83Eu64;
        
        let result = invoke_raw!(
            hash,
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn _0x50a8a36201dbf83e_raw(
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x50A8A36201DBF83Eu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x50A8A36201DBF83Eu64;

        invoke_raw_typed!(
            hash,
        )
    }
}

/// ## Return value



pub fn _0xd8122c407663b995_safe(
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xD8122C407663B995u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xD8122C407663B995u64;
        
        let result = invoke_raw!(
            hash,
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn _0xd8122c407663b995_raw(
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xD8122C407663B995u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xD8122C407663B995u64;

        invoke_raw_typed!(
            hash,
        )
    }
}

/// ```
NativeDB Introduced: v1290
```



pub fn _0xe4f6e8d07a2f0f51_safe(
        
        
            p0: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xE4F6E8D07A2F0F51u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xE4F6E8D07A2F0F51u64;
        
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
pub fn _0xe4f6e8d07a2f0f51_raw(
        p0: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xE4F6E8D07A2F0F51u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xE4F6E8D07A2F0F51u64;

        invoke_raw_typed!(
            hash,
                p0
        )
    }
}

/// ## Parameters
*



pub fn _sc_inbox_message_get_string_safe(
        
        
            p0: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xF3E31D16CBDCB304u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xF3E31D16CBDCB304u64;
        
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
pub fn _sc_inbox_message_get_string_raw(
        p0: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xF3E31D16CBDCB304u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xF3E31D16CBDCB304u64;

        invoke_raw_typed!(
            hash,
                p0
        )
    }
}

/// ## Parameters
*



pub fn _set_handle_rockstar_message_via_script_safe(
        
        
            toggle: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xBFA0A56A817C6C7Du32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xBFA0A56A817C6C7Du64;
        
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
pub fn _set_handle_rockstar_message_via_script_raw(
        toggle: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xBFA0A56A817C6C7Du32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xBFA0A56A817C6C7Du64;

        invoke_raw_typed!(
            hash,
                toggle
        )
    }
}

/// ## Parameters
*



pub fn sc_inbox_message_get_data_string_safe(
        
        
            p0: 
        , 
        
        
            context: 
        , 
        
        
            out: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x7572EF42FC6A9B6Du32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x7572EF42FC6A9B6Du64;
        
        let result = invoke_raw!(
            hash,
                p0, 
                context, 
                out
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn sc_inbox_message_get_data_string_raw(
        p0: , 
        context: , 
        out: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x7572EF42FC6A9B6Du32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x7572EF42FC6A9B6Du64;

        invoke_raw_typed!(
            hash,
                p0, 
                context, 
                out
        )
    }
}

/// Gets number of plates in the response of the get license plates request.
Range: [0, count) can be used as second argument to _0x1D4446A62D35B0D0 and _0x2E89990DDFF670C3



pub fn sc_licenseplate_get_count_safe(
        
        
            token: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x700569DBA175A77Cu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x700569DBA175A77Cu64;
        
        let result = invoke_raw!(
            hash,
                token
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn sc_licenseplate_get_count_raw(
        token: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x700569DBA175A77Cu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x700569DBA175A77Cu64;

        invoke_raw_typed!(
            hash,
                token
        )
    }
}

/// ## Parameters
*



pub fn sc_inbox_message_get_data_int_safe(
        
        
            p0: 
        , 
        
        
            context: 
        , 
        
        
            out: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xA00EFE4082C4056Eu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xA00EFE4082C4056Eu64;
        
        let result = invoke_raw!(
            hash,
                p0, 
                context, 
                out
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn sc_inbox_message_get_data_int_raw(
        p0: , 
        context: , 
        out: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xA00EFE4082C4056Eu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xA00EFE4082C4056Eu64;

        invoke_raw_typed!(
            hash,
                p0, 
                context, 
                out
        )
    }
}

/// ## Parameters
*



pub fn _0x92da6e70ef249bd1_safe(
        
        
            p0: 
        , 
        
        
            p1: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x92DA6E70EF249BD1u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x92DA6E70EF249BD1u64;
        
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
pub fn _0x92da6e70ef249bd1_raw(
        p0: , 
        p1: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x92DA6E70EF249BD1u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x92DA6E70EF249BD1u64;

        invoke_raw_typed!(
            hash,
                p0, 
                p1
        )
    }
}

/// ## Parameters
*



pub fn _sc_inbox_message_send_ugc_stat_update_event_safe(
        
        
            data: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xA68D3D229F4F3B06u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xA68D3D229F4F3B06u64;
        
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
pub fn _sc_inbox_message_send_ugc_stat_update_event_raw(
        data: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xA68D3D229F4F3B06u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xA68D3D229F4F3B06u64;

        invoke_raw_typed!(
            hash,
                data
        )
    }
}

/// ## Parameters
*



pub fn _sc_inbox_message_send_bounty_presence_event_safe(
        
        
            data: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x6AFD2CD753FEEF83u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x6AFD2CD753FEEF83u64;
        
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
pub fn _sc_inbox_message_send_bounty_presence_event_raw(
        data: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x6AFD2CD753FEEF83u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x6AFD2CD753FEEF83u64;

        invoke_raw_typed!(
            hash,
                data
        )
    }
}

/// ```
sfink: from scripts:
func_720(socialclub::_0x8416FE4E4629D7D7("bIgnoreCheaterOverride"));
func_719(socialclub::_0x8416FE4E4629D7D7("bIgnoreBadSportOverride"));
```



pub fn sc_gamerdata_get_bool_safe(
        
        
            name: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x8416FE4E4629D7D7u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x8416FE4E4629D7D7u64;
        
        let result = invoke_raw!(
            hash,
                name
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn sc_gamerdata_get_bool_raw(
        name: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x8416FE4E4629D7D7u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x8416FE4E4629D7D7u64;

        invoke_raw_typed!(
            hash,
                name
        )
    }
}

/// Checks if the "is valid license plate" request is still pending.



pub fn sc_licenseplate_get_isvalid_is_pending_safe(
        
        
            token: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xD302E99EDF0449CFu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xD302E99EDF0449CFu64;
        
        let result = invoke_raw!(
            hash,
                token
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn sc_licenseplate_get_isvalid_is_pending_raw(
        token: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xD302E99EDF0449CFu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xD302E99EDF0449CFu64;

        invoke_raw_typed!(
            hash,
                token
        )
    }
}

/// ```
NativeDB Added Parameter 1: char* p0
NativeDB Added Parameter 2: char* p1
NativeDB Added Parameter 3: char* p2
NativeDB Introduced: v323
```



pub fn _0x1d12a56fc95be92e_safe(
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x1D12A56FC95BE92Eu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x1D12A56FC95BE92Eu64;
        
        let result = invoke_raw!(
            hash,
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn _0x1d12a56fc95be92e_raw(
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x1D12A56FC95BE92Eu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x1D12A56FC95BE92Eu64;

        invoke_raw_typed!(
            hash,
        )
    }
}

/// ## Parameters
*



pub fn _0x44aca259d67651db_safe(
        
        
            p0: 
        , 
        
        
            p1: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x44ACA259D67651DBu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x44ACA259D67651DBu64;
        
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
pub fn _0x44aca259d67651db_raw(
        p0: , 
        p1: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x44ACA259D67651DBu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x44ACA259D67651DBu64;

        invoke_raw_typed!(
            hash,
                p0, 
                p1
        )
    }
}

/// ## Parameters
*



pub fn _sc_profanity_check_ugc_string_safe(
        
        
            string: 
        , 
        
        
            token: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xEB2BF817463DFA28u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xEB2BF817463DFA28u64;
        
        let result = invoke_raw!(
            hash,
                string, 
                token
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn _sc_profanity_check_ugc_string_raw(
        string: , 
        token: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xEB2BF817463DFA28u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xEB2BF817463DFA28u64;

        invoke_raw_typed!(
            hash,
                string, 
                token
        )
    }
}

/// ```
NativeDB Added Parameter 1: char* p0
NativeDB Added Parameter 2: float* p1
NativeDB Added Parameter 3: char* p2
NativeDB Introduced: v323
```



pub fn _0x2570e26be63964e3_safe(
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x2570E26BE63964E3u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x2570E26BE63964E3u64;
        
        let result = invoke_raw!(
            hash,
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn _0x2570e26be63964e3_raw(
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x2570E26BE63964E3u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x2570E26BE63964E3u64;

        invoke_raw_typed!(
            hash,
        )
    }
}

/// ## Parameters
*



pub fn _0x07dbd622d9533857_safe(
        
        
            p0: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x07DBD622D9533857u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x07DBD622D9533857u64;
        
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
pub fn _0x07dbd622d9533857_raw(
        p0: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x07DBD622D9533857u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x07DBD622D9533857u64;

        invoke_raw_typed!(
            hash,
                p0
        )
    }
}

/// Gets license plate data related to the get license plate info request



pub fn sc_licenseplate_get_plate_data_safe(
        
        
            token: 
        , 
        
        
            plateIndex: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x2E89990DDFF670C3u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x2E89990DDFF670C3u64;
        
        let result = invoke_raw!(
            hash,
                token, 
                plateIndex
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn sc_licenseplate_get_plate_data_raw(
        token: , 
        plateIndex: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x2E89990DDFF670C3u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x2E89990DDFF670C3u64;

        invoke_raw_typed!(
            hash,
                token, 
                plateIndex
        )
    }
}

/// ## Parameters
*



pub fn sc_inbox_message_do_apply_safe(
        
        
            p0: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x9A2C8064B6C1E41Au32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x9A2C8064B6C1E41Au64;
        
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
pub fn sc_inbox_message_do_apply_raw(
        p0: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x9A2C8064B6C1E41Au32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x9A2C8064B6C1E41Au64;

        invoke_raw_typed!(
            hash,
                p0
        )
    }
}

/// Starts a task to check an entered string for profanity on the ROS/Social Club services.  

See also: [`SC_PROFANITY_GET_CHECK_IS_VALID`](#_0x1753344C770358AE) and [`SC_PROFANITY_GET_CHECK_IS_PENDING`](#_0x82E4A58BABC15AE7).



pub fn sc_profanity_check_string_safe(
        
        
            string: 
        , 
        
        
            token: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x75632C5ECD7ED843u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x75632C5ECD7ED843u64;
        
        let result = invoke_raw!(
            hash,
                string, 
                token
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn sc_profanity_check_string_raw(
        string: , 
        token: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x75632C5ECD7ED843u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x75632C5ECD7ED843u64;

        invoke_raw_typed!(
            hash,
                string, 
                token
        )
    }
}

/// ## Parameters
*



pub fn sc_profanity_get_check_is_pending_safe(
        
        
            token: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x82E4A58BABC15AE7u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x82E4A58BABC15AE7u64;
        
        let result = invoke_raw!(
            hash,
                token
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn sc_profanity_get_check_is_pending_raw(
        token: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x82E4A58BABC15AE7u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x82E4A58BABC15AE7u64;

        invoke_raw_typed!(
            hash,
                token
        )
    }
}

/// ## Return value



pub fn _is_rockstar_message_ready_for_script_safe(
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xBC1CC91205EC8D6Eu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xBC1CC91205EC8D6Eu64;
        
        let result = invoke_raw!(
            hash,
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn _is_rockstar_message_ready_for_script_raw(
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xBC1CC91205EC8D6Eu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xBC1CC91205EC8D6Eu64;

        invoke_raw_typed!(
            hash,
        )
    }
}

/// ```
NativeDB Added Parameter 1: char* p0
NativeDB Added Parameter 2: char* p1
NativeDB Introduced: v323
```



pub fn _0x9de5d2f723575ed0_safe(
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x9DE5D2F723575ED0u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x9DE5D2F723575ED0u64;
        
        let result = invoke_raw!(
            hash,
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn _0x9de5d2f723575ed0_raw(
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x9DE5D2F723575ED0u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x9DE5D2F723575ED0u64;

        invoke_raw_typed!(
            hash,
        )
    }
}

/// ## Return value



pub fn _0x7db18ca8cad5b098_safe(
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x7DB18CA8CAD5B098u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x7DB18CA8CAD5B098u64;
        
        let result = invoke_raw!(
            hash,
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn _0x7db18ca8cad5b098_raw(
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x7DB18CA8CAD5B098u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x7DB18CA8CAD5B098u64;

        invoke_raw_typed!(
            hash,
        )
    }
}

/// ## Parameters
*



pub fn _0x4737980e8a283806_safe(
        
        
            p0: 
        , 
        
        
            p1: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x4737980E8A283806u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x4737980E8A283806u64;
        
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
pub fn _0x4737980e8a283806_raw(
        p0: , 
        p1: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x4737980E8A283806u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x4737980E8A283806u64;

        invoke_raw_typed!(
            hash,
                p0, 
                p1
        )
    }
}

/// ## Parameters
*



pub fn sc_email_message_push_gamer_to_recip_list_safe(
        
        
            networkHandle: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x2330C12A7A605D16u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x2330C12A7A605D16u64;
        
        let result = invoke_raw!(
            hash,
                networkHandle
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn sc_email_message_push_gamer_to_recip_list_raw(
        networkHandle: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x2330C12A7A605D16u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x2330C12A7A605D16u64;

        invoke_raw_typed!(
            hash,
                networkHandle
        )
    }
}

/// ## Parameters
*



pub fn _0xf6baaaf762e1bf40_safe(
        
        
            p0: 
        , 
        
        
            p1: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xF6BAAAF762E1BF40u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xF6BAAAF762E1BF40u64;
        
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
pub fn _0xf6baaaf762e1bf40_raw(
        p0: , 
        p1: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xF6BAAAF762E1BF40u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xF6BAAAF762E1BF40u64;

        invoke_raw_typed!(
            hash,
                p0, 
                p1
        )
    }
}

/// ## Parameters
*



pub fn sc_inbox_message_get_ugcdata_safe(
        
        
            p0: 
        , 
        
        
            p1: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x69D82604A1A5A254u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x69D82604A1A5A254u64;
        
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
pub fn sc_inbox_message_get_ugcdata_raw(
        p0: , 
        p1: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x69D82604A1A5A254u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x69D82604A1A5A254u64;

        invoke_raw_typed!(
            hash,
                p0, 
                p1
        )
    }
}

/// ## Parameters
*



pub fn sc_presence_attr_set_int_safe(
        
        
            attrHash: 
        , 
        
        
            value: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x1F1E9682483697C7u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x1F1E9682483697C7u64;
        
        let result = invoke_raw!(
            hash,
                attrHash, 
                value
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn sc_presence_attr_set_int_raw(
        attrHash: , 
        value: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x1F1E9682483697C7u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x1F1E9682483697C7u64;

        invoke_raw_typed!(
            hash,
                attrHash, 
                value
        )
    }
}

/// ## Return value



pub fn _0x3001bef2feca3680_safe(
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x3001BEF2FECA3680u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x3001BEF2FECA3680u64;
        
        let result = invoke_raw!(
            hash,
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn _0x3001bef2feca3680_raw(
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x3001BEF2FECA3680u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x3001BEF2FECA3680u64;

        invoke_raw_typed!(
            hash,
        )
    }
}

/// ```
NativeDB Added Parameter 1: char* p0
NativeDB Added Parameter 2: char* p1
NativeDB Introduced: v323
```



pub fn _0x33df47cc0642061b_safe(
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x33DF47CC0642061Bu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x33DF47CC0642061Bu64;
        
        let result = invoke_raw!(
            hash,
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn _0x33df47cc0642061b_raw(
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x33DF47CC0642061Bu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x33DF47CC0642061Bu64;

        invoke_raw_typed!(
            hash,
        )
    }
}

/// ```
NativeDB Introduced: v323
```



pub fn _0x4ed9c8d6da297639_safe(
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x4ED9C8D6DA297639u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x4ED9C8D6DA297639u64;
        
        let result = invoke_raw!(
            hash,
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn _0x4ed9c8d6da297639_raw(
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x4ED9C8D6DA297639u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x4ED9C8D6DA297639u64;

        invoke_raw_typed!(
            hash,
        )
    }
}

/// ## Parameters
*



pub fn _0xf22ca0fd74b80e7a_safe(
        
        
            p0: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xF22CA0FD74B80E7Au32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xF22CA0FD74B80E7Au64;
        
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
pub fn _0xf22ca0fd74b80e7a_raw(
        p0: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xF22CA0FD74B80E7Au32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xF22CA0FD74B80E7Au64;

        invoke_raw_typed!(
            hash,
                p0
        )
    }
}

/// ## Parameters
*



pub fn _0x225798743970412b_safe(
        
        
            p0: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x225798743970412Bu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x225798743970412Bu64;
        
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
pub fn _0x225798743970412b_raw(
        p0: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x225798743970412Bu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x225798743970412Bu64;

        invoke_raw_typed!(
            hash,
                p0
        )
    }
}

/// ## Return value



pub fn _rockstar_message_get_string_safe(
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xDF649C4E9AFDD788u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xDF649C4E9AFDD788u64;
        
        let result = invoke_raw!(
            hash,
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn _rockstar_message_get_string_raw(
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xDF649C4E9AFDD788u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xDF649C4E9AFDD788u64;

        invoke_raw_typed!(
            hash,
        )
    }
}

/// Gets the status of the is valid license plate request



pub fn sc_licenseplate_get_isvalid_status_safe(
        
        
            token: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x5C4EBFFA98BDB41Cu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x5C4EBFFA98BDB41Cu64;
        
        let result = invoke_raw!(
            hash,
                token
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn sc_licenseplate_get_isvalid_status_raw(
        token: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x5C4EBFFA98BDB41Cu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x5C4EBFFA98BDB41Cu64;

        invoke_raw_typed!(
            hash,
                token
        )
    }
}

/// ## Return value



pub fn _0x16da8172459434aa_safe(
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x16DA8172459434AAu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x16DA8172459434AAu64;
        
        let result = invoke_raw!(
            hash,
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn _0x16da8172459434aa_raw(
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x16DA8172459434AAu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x16DA8172459434AAu64;

        invoke_raw_typed!(
            hash,
        )
    }
}

/// ## Parameters
*



pub fn _sc_inbox_message_pop_safe(
        
        
            p0: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x2C015348CF19CA1Du32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x2C015348CF19CA1Du64;
        
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
pub fn _sc_inbox_message_pop_raw(
        p0: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x2C015348CF19CA1Du32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x2C015348CF19CA1Du64;

        invoke_raw_typed!(
            hash,
                p0
        )
    }
}

/// Returns a status for adding the license plate



pub fn sc_licenseplate_get_add_status_safe(
        
        
            token: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x8147FFF6A718E1ADu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x8147FFF6A718E1ADu64;
        
        let result = invoke_raw!(
            hash,
                token
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn sc_licenseplate_get_add_status_raw(
        token: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x8147FFF6A718E1ADu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x8147FFF6A718E1ADu64;

        invoke_raw_typed!(
            hash,
                token
        )
    }
}

/// SC_EMAIL_MESSAGE_CLEAR_RECIP_LIST native function



pub fn sc_email_message_clear_recip_list_safe(
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x55DF6DB45179236Eu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x55DF6DB45179236Eu64;
        
        let result = invoke_raw!(
            hash,
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn sc_email_message_clear_recip_list_raw(
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x55DF6DB45179236Eu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x55DF6DB45179236Eu64;

        invoke_raw_typed!(
            hash,
        )
    }
}

/// ## Parameters
*



pub fn _0x4a7d6e727f941747_safe(
        
        
            p0: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x4A7D6E727F941747u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x4A7D6E727F941747u64;
        
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
pub fn _0x4a7d6e727f941747_raw(
        p0: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x4A7D6E727F941747u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x4A7D6E727F941747u64;

        invoke_raw_typed!(
            hash,
                p0
        )
    }
}

/// ```
NativeDB Introduced: v1290
```



pub fn _0x8a4416c0db05fa66_safe(
        
        
            newsStoryData: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x8A4416C0DB05FA66u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x8A4416C0DB05FA66u64;
        
        let result = invoke_raw!(
            hash,
                newsStoryData
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn _0x8a4416c0db05fa66_raw(
        newsStoryData: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x8A4416C0DB05FA66u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x8A4416C0DB05FA66u64;

        invoke_raw_typed!(
            hash,
                newsStoryData
        )
    }
}

/// SC native to start a request for if license plate text is valid



pub fn sc_licenseplate_isvalid_safe(
        
        
            plateText: 
        , 
        
        
            tokenOut: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x0F73393BAC7E6730u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x0F73393BAC7E6730u64;
        
        let result = invoke_raw!(
            hash,
                plateText, 
                tokenOut
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn sc_licenseplate_isvalid_raw(
        plateText: , 
        tokenOut: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x0F73393BAC7E6730u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x0F73393BAC7E6730u64;

        invoke_raw_typed!(
            hash,
                plateText, 
                tokenOut
        )
    }
}

/// ```
Returns the nickname of the logged-in Rockstar Social Club account.
```



pub fn _sc_get_nickname_safe(
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x198D161F458ECC7Fu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x198D161F458ECC7Fu64;
        
        let result = invoke_raw!(
            hash,
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn _sc_get_nickname_raw(
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x198D161F458ECC7Fu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x198D161F458ECC7Fu64;

        invoke_raw_typed!(
            hash,
        )
    }
}

/// ## Parameters
*



pub fn _0x9237e334f6e43156_safe(
        
        
            p0: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x9237E334F6E43156u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x9237E334F6E43156u64;
        
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
pub fn _0x9237e334f6e43156_raw(
        p0: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x9237E334F6E43156u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x9237E334F6E43156u64;

        invoke_raw_typed!(
            hash,
                p0
        )
    }
}

/// ```
NativeDB Added Parameter 1: char* p0
NativeDB Introduced: v323
```



pub fn _0xc2c97ea97711d1ae_safe(
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xC2C97EA97711D1AEu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xC2C97EA97711D1AEu64;
        
        let result = invoke_raw!(
            hash,
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn _0xc2c97ea97711d1ae_raw(
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xC2C97EA97711D1AEu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xC2C97EA97711D1AEu64;

        invoke_raw_typed!(
            hash,
        )
    }
}

/// ## Parameters
*



pub fn sc_profanity_get_string_status_safe(
        
        
            token: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x930DE22F07B1CCE3u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x930DE22F07B1CCE3u64;
        
        let result = invoke_raw!(
            hash,
                token
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn sc_profanity_get_string_status_raw(
        token: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x930DE22F07B1CCE3u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x930DE22F07B1CCE3u64;

        invoke_raw_typed!(
            hash,
                token
        )
    }
}

/// Social Club native to add license plate for the user



pub fn sc_licenseplate_add_safe(
        
        
            plateText: 
        , 
        
        
            plateData: 
        , 
        
        
            tokenOut: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x1989C6E6F67E76A8u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x1989C6E6F67E76A8u64;
        
        let result = invoke_raw!(
            hash,
                plateText, 
                plateData, 
                tokenOut
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn sc_licenseplate_add_raw(
        plateText: , 
        plateData: , 
        tokenOut: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x1989C6E6F67E76A8u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x1989C6E6F67E76A8u64;

        invoke_raw_typed!(
            hash,
                plateText, 
                plateData, 
                tokenOut
        )
    }
}

/// ```
NativeDB Introduced: v323
```



pub fn _0x7ffcbfee44ecfabf_safe(
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x7FFCBFEE44ECFABFu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x7FFCBFEE44ECFABFu64;
        
        let result = invoke_raw!(
            hash,
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn _0x7ffcbfee44ecfabf_raw(
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x7FFCBFEE44ECFABFu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x7FFCBFEE44ECFABFu64;

        invoke_raw_typed!(
            hash,
        )
    }
}

/// ## Parameters
*



pub fn sc_inbox_get_message_type_at_index_safe(
        
        
            msgIndex: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xBB8EA16ECBC976C4u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xBB8EA16ECBC976C4u64;
        
        let result = invoke_raw!(
            hash,
                msgIndex
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn sc_inbox_get_message_type_at_index_raw(
        msgIndex: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xBB8EA16ECBC976C4u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xBB8EA16ECBC976C4u64;

        invoke_raw_typed!(
            hash,
                msgIndex
        )
    }
}

/// ## Return value



pub fn _0xff8f3a92b75ed67a_safe(
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xFF8F3A92B75ED67Au32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xFF8F3A92B75ED67Au64;
        
        let result = invoke_raw!(
            hash,
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn _0xff8f3a92b75ed67a_raw(
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xFF8F3A92B75ED67Au32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xFF8F3A92B75ED67Au64;

        invoke_raw_typed!(
            hash,
        )
    }
}

/// Returns true if the add license plate text request is still pending.



pub fn sc_licenseplate_get_add_is_pending_safe(
        
        
            token: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x07C61676E5BB52CDu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x07C61676E5BB52CDu64;
        
        let result = invoke_raw!(
            hash,
                token
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn sc_licenseplate_get_add_is_pending_raw(
        token: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x07C61676E5BB52CDu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x07C61676E5BB52CDu64;

        invoke_raw_typed!(
            hash,
                token
        )
    }
}

/// ## Parameters
*



pub fn _sc_inbox_message_get_data_bool_safe(
        
        
            p0: 
        , 
        
        
            p1: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xFFE5C16F402D851Du32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xFFE5C16F402D851Du64;
        
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
pub fn _sc_inbox_message_get_data_bool_raw(
        p0: , 
        p1: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xFFE5C16F402D851Du32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xFFE5C16F402D851Du64;

        invoke_raw_typed!(
            hash,
                p0, 
                p1
        )
    }
}

/// ## Parameters
*



pub fn _0x487912fd248efddf_safe(
        
        
            p0: 
        , 
        
        
            p1: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x487912FD248EFDDFu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x487912FD248EFDDFu64;
        
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
pub fn _0x487912fd248efddf_raw(
        p0: , 
        p1: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x487912FD248EFDDFu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x487912FD248EFDDFu64;

        invoke_raw_typed!(
            hash,
                p0, 
                p1
        )
    }
}

/// Changes the license plate for the user (no insert).



pub fn sc_licenseplate_set_plate_data_safe(
        
        
            oldPlateText: 
        , 
        
        
            newPlateText: 
        , 
        
        
            plateData: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xD0EE05FE193646EAu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xD0EE05FE193646EAu64;
        
        let result = invoke_raw!(
            hash,
                oldPlateText, 
                newPlateText, 
                plateData
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn sc_licenseplate_set_plate_data_raw(
        oldPlateText: , 
        newPlateText: , 
        plateData: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xD0EE05FE193646EAu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xD0EE05FE193646EAu64;

        invoke_raw_typed!(
            hash,
                oldPlateText, 
                newPlateText, 
                plateData
        )
    }
}

/// ```
NativeDB Added Parameter 1: char* p0
NativeDB Introduced: v323
```



pub fn _0x450819d8cf90c416_safe(
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x450819D8CF90C416u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x450819D8CF90C416u64;
        
        let result = invoke_raw!(
            hash,
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn _0x450819d8cf90c416_raw(
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x450819D8CF90C416u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x450819D8CF90C416u64;

        invoke_raw_typed!(
            hash,
        )
    }
}

/// ```
NativeDB Introduced: v1290
```



pub fn _0xea95c0853a27888e_safe(
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xEA95C0853A27888Eu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xEA95C0853A27888Eu64;
        
        let result = invoke_raw!(
            hash,
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn _0xea95c0853a27888e_raw(
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xEA95C0853A27888Eu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xEA95C0853A27888Eu64;

        invoke_raw_typed!(
            hash,
        )
    }
}

/// ## Parameters
*



pub fn _sc_inbox_get_emails_safe(
        
        
            offset: 
        , 
        
        
            limit: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x040ADDCBAFA1018Au32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x040ADDCBAFA1018Au64;
        
        let result = invoke_raw!(
            hash,
                offset, 
                limit
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn _sc_inbox_get_emails_raw(
        offset: , 
        limit: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x040ADDCBAFA1018Au32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x040ADDCBAFA1018Au64;

        invoke_raw_typed!(
            hash,
                offset, 
                limit
        )
    }
}

/// Gets license plate text related to the get license plate info request



pub fn sc_licenseplate_get_plate_safe(
        
        
            token: 
        , 
        
        
            plateIndex: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x1D4446A62D35B0D0u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x1D4446A62D35B0D0u64;
        
        let result = invoke_raw!(
            hash,
                token, 
                plateIndex
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn sc_licenseplate_get_plate_raw(
        token: , 
        plateIndex: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x1D4446A62D35B0D0u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x1D4446A62D35B0D0u64;

        invoke_raw_typed!(
            hash,
                token, 
                plateIndex
        )
    }
}

/// ```
NativeDB Added Parameter 1: char* p0
NativeDB Added Parameter 2: int* p1
NativeDB Added Parameter 3: char* p2
NativeDB Introduced: v323
```



pub fn _0xe75a4a2e5e316d86_safe(
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xE75A4A2E5E316D86u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xE75A4A2E5E316D86u64;
        
        let result = invoke_raw!(
            hash,
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn _0xe75a4a2e5e316d86_raw(
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xE75A4A2E5E316D86u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xE75A4A2E5E316D86u64;

        invoke_raw_typed!(
            hash,
        )
    }
}

/// ```
NativeDB Introduced: v323
```



pub fn _0x2d874d4ae612a65f_safe(
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x2D874D4AE612A65Fu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x2D874D4AE612A65Fu64;
        
        let result = invoke_raw!(
            hash,
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn _0x2d874d4ae612a65f_raw(
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x2D874D4AE612A65Fu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x2D874D4AE612A65Fu64;

        invoke_raw_typed!(
            hash,
        )
    }
}

/// ## Parameters
*



pub fn sc_inbox_get_message_is_read_at_index_safe(
        
        
            msgIndex: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x93028F1DB42BFD08u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x93028F1DB42BFD08u64;
        
        let result = invoke_raw!(
            hash,
                msgIndex
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn sc_inbox_get_message_is_read_at_index_raw(
        msgIndex: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x93028F1DB42BFD08u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x93028F1DB42BFD08u64;

        invoke_raw_typed!(
            hash,
                msgIndex
        )
    }
}

/// ```
NativeDB Added Parameter 1: int p0
NativeDB Added Parameter 2: char* p1
NativeDB Added Parameter 3: float* p2
NativeDB Introduced: v323
```



pub fn _0xc5a35c73b68f3c49_safe(
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xC5A35C73B68F3C49u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xC5A35C73B68F3C49u64;
        
        let result = invoke_raw!(
            hash,
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn _0xc5a35c73b68f3c49_raw(
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xC5A35C73B68F3C49u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xC5A35C73B68F3C49u64;

        invoke_raw_typed!(
            hash,
        )
    }
}

/// ## Return value



pub fn sc_inbox_get_total_num_messages_safe(
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x03A93FF1A2CA0864u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x03A93FF1A2CA0864u64;
        
        let result = invoke_raw!(
            hash,
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn sc_inbox_get_total_num_messages_raw(
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x03A93FF1A2CA0864u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x03A93FF1A2CA0864u64;

        invoke_raw_typed!(
            hash,
        )
    }
}

/// ```
Same as HAS_ACHIEVEMENT_BEEN_PASSED
```



pub fn _sc_get_has_achievement_been_passed_safe(
        
        
            achievement: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x418DC16FAE452C1Cu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x418DC16FAE452C1Cu64;
        
        let result = invoke_raw!(
            hash,
                achievement
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn _sc_get_has_achievement_been_passed_raw(
        achievement: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x418DC16FAE452C1Cu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x418DC16FAE452C1Cu64;

        invoke_raw_typed!(
            hash,
                achievement
        )
    }
}

/// ## Parameters
*



pub fn _0x8cc469ab4d349b7c_safe(
        
        
            p0: 
        , 
        
        
            p1: 
        , 
        
        
            p2: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x8CC469AB4D349B7Cu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x8CC469AB4D349B7Cu64;
        
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
pub fn _0x8cc469ab4d349b7c_raw(
        p0: , 
        p1: , 
        p2: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x8CC469AB4D349B7Cu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x8CC469AB4D349B7Cu64;

        invoke_raw_typed!(
            hash,
                p0, 
                p1, 
                p2
        )
    }
}

/// ```
NativeDB Added Parameter 1: int p0
NativeDB Introduced: v323
```



pub fn _0xa468e0be12b12c70_safe(
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xA468E0BE12B12C70u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xA468E0BE12B12C70u64;
        
        let result = invoke_raw!(
            hash,
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn _0xa468e0be12b12c70_raw(
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xA468E0BE12B12C70u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xA468E0BE12B12C70u64;

        invoke_raw_typed!(
            hash,
        )
    }
}

/// ## Parameters
*



pub fn sc_profanity_get_string_passed_safe(
        
        
            token: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x85535ACF97FC0969u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x85535ACF97FC0969u64;
        
        let result = invoke_raw!(
            hash,
                token
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn sc_profanity_get_string_passed_raw(
        token: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x85535ACF97FC0969u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x85535ACF97FC0969u64;

        invoke_raw_typed!(
            hash,
                token
        )
    }
}

/// ## Parameters
*



pub fn _0x6bfb12ce158e3dd4_safe(
        
        
            p0: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x6BFB12CE158E3DD4u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x6BFB12CE158E3DD4u64;
        
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
pub fn _0x6bfb12ce158e3dd4_raw(
        p0: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x6BFB12CE158E3DD4u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x6BFB12CE158E3DD4u64;

        invoke_raw_typed!(
            hash,
                p0
        )
    }
}

/// ```
NativeDB Introduced: v323
NativeDB Added Parameter 1: Hash attrHash
NativeDB Added Parameter 2: float value
```



pub fn sc_presence_attr_set_float_safe(
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xC4C4575F62534A24u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xC4C4575F62534A24u64;
        
        let result = invoke_raw!(
            hash,
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn sc_presence_attr_set_float_raw(
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xC4C4575F62534A24u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xC4C4575F62534A24u64;

        invoke_raw_typed!(
            hash,
        )
    }
}

/// ## Parameters
*



pub fn _0x19853b5b17d77bca_safe(
        
        
            p0: 
        , 
        
        
            p1: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x19853B5B17D77BCAu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x19853B5B17D77BCAu64;
        
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
pub fn _0x19853b5b17d77bca_raw(
        p0: , 
        p1: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x19853B5B17D77BCAu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x19853B5B17D77BCAu64;

        invoke_raw_typed!(
            hash,
                p0, 
                p1
        )
    }
}

/// ## Parameters
*



pub fn _0x699e4a5c8c893a18_safe(
        
        
            p0: 
        , 
        
        
            p1: 
        , 
        
        
            p2: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x699E4A5C8C893A18u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x699E4A5C8C893A18u64;
        
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
pub fn _0x699e4a5c8c893a18_raw(
        p0: , 
        p1: , 
        p2: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x699E4A5C8C893A18u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x699E4A5C8C893A18u64;

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



pub fn _sc_inbox_message_push_gamer_to_event_recip_list_safe(
        
        
            networkHandle: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xDA024BDBD600F44Au32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xDA024BDBD600F44Au64;
        
        let result = invoke_raw!(
            hash,
                networkHandle
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn _sc_inbox_message_push_gamer_to_event_recip_list_raw(
        networkHandle: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xDA024BDBD600F44Au32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xDA024BDBD600F44Au64;

        invoke_raw_typed!(
            hash,
                networkHandle
        )
    }
}

/// _0x675721C9F644D161 native function



pub fn _0x675721c9f644d161_safe(
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x675721C9F644D161u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x675721C9F644D161u64;
        
        let result = invoke_raw!(
            hash,
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn _0x675721c9f644d161_raw(
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x675721C9F644D161u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x675721C9F644D161u64;

        invoke_raw_typed!(
            hash,
        )
    }
}

/// ## Parameters
*



pub fn sc_profanity_get_check_is_valid_safe(
        
        
            token: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x1753344C770358AEu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x1753344C770358AEu64;
        
        let result = invoke_raw!(
            hash,
                token
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn sc_profanity_get_check_is_valid_raw(
        token: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x1753344C770358AEu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x1753344C770358AEu64;

        invoke_raw_typed!(
            hash,
                token
        )
    }
}

/// ```
NativeDB Introduced: v323
```



pub fn sc_gamerdata_get_int_safe(
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xC85A7127E7AD02AAu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xC85A7127E7AD02AAu64;
        
        let result = invoke_raw!(
            hash,
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn sc_gamerdata_get_int_raw(
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xC85A7127E7AD02AAu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xC85A7127E7AD02AAu64;

        invoke_raw_typed!(
            hash,
        )
    }
}

