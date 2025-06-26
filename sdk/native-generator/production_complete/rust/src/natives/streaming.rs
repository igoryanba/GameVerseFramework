//! STREAMING native functions
//! 
//! Functions for the streaming category (local)

use crate::types::*;
use crate::errors::{NativeError, NativeResult, validation};
use nalgebra::Vector3;
use tracing::{debug, warn};

/// ```
Probably IS_SWITCH_*
```



pub fn _0x933bbeeb8c61b5f4_safe(
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x933BBEEB8C61B5F4u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x933BBEEB8C61B5F4u64;
        
        let result = invoke_raw!(
            hash,
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn _0x933bbeeb8c61b5f4_raw(
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x933BBEEB8C61B5F4u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x933BBEEB8C61B5F4u64;

        invoke_raw_typed!(
            hash,
        )
    }
}

/// Releases the script ownership assigned by REQUEST\_MODEL. This command should be used when done using a specific model hash in script.



pub fn set_model_as_no_longer_needed_safe(
        
        
            model: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xE532F5D78798DAABu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xE532F5D78798DAABu64;
        
        let result = invoke_raw!(
            hash,
                model
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn set_model_as_no_longer_needed_raw(
        model: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xE532F5D78798DAABu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xE532F5D78798DAABu64;

        invoke_raw_typed!(
            hash,
                model
        )
    }
}

/// ## Parameters
*



pub fn _0xef39ee20c537e98c_safe(
        
        
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
        let hash = 0xEF39EE20C537E98Cu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xEF39EE20C537E98Cu64;
        
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
pub fn _0xef39ee20c537e98c_raw(
        p0: , 
        p1: , 
        p2: , 
        p3: , 
        p4: , 
        p5: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xEF39EE20C537E98Cu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xEF39EE20C537E98Cu64;

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
Returns true if the player is currently switching, false otherwise.  
(When the camera is in the sky moving from Trevor to Franklin for example)  
```



pub fn is_player_switch_in_progress_safe(
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xD9D2CFFF49FAB35Fu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xD9D2CFFF49FAB35Fu64;
        
        let result = invoke_raw!(
            hash,
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn is_player_switch_in_progress_raw(
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xD9D2CFFF49FAB35Fu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xD9D2CFFF49FAB35Fu64;

        invoke_raw_typed!(
            hash,
        )
    }
}

/// ## Parameters
*



pub fn streamvol_is_valid_safe(
        
        
            unused: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x07C313F94746702Cu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x07C313F94746702Cu64;
        
        let result = invoke_raw!(
            hash,
                unused
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn streamvol_is_valid_raw(
        unused: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x07C313F94746702Cu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x07C313F94746702Cu64;

        invoke_raw_typed!(
            hash,
                unused
        )
    }
}

/// ## Parameters
*



pub fn _0xf8155a7f03ddfc8e_safe(
        
        
            p0: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xF8155A7F03DDFC8Eu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xF8155A7F03DDFC8Eu64;
        
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
pub fn _0xf8155a7f03ddfc8e_raw(
        p0: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xF8155A7F03DDFC8Eu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xF8155A7F03DDFC8Eu64;

        invoke_raw_typed!(
            hash,
                p0
        )
    }
}

/// ## Parameters
*



pub fn remove_model_from_creator_budget_safe(
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xF086AD9354FAC3A3u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xF086AD9354FAC3A3u64;
        
        let result = invoke_raw!(
            hash,
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn remove_model_from_creator_budget_raw(
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xF086AD9354FAC3A3u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xF086AD9354FAC3A3u64;

        invoke_raw_typed!(
            hash,
        )
    }
}

/// ```
Alias for HAS_ANIM_SET_LOADED.  
```



pub fn has_clip_set_loaded_safe(
        
        
            clipSet: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x318234F4F3738AF3u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x318234F4F3738AF3u64;
        
        let result = invoke_raw!(
            hash,
                clipSet
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn has_clip_set_loaded_raw(
        clipSet: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x318234F4F3738AF3u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x318234F4F3738AF3u64;

        invoke_raw_typed!(
            hash,
                clipSet
        )
    }
}

/// ## Parameters
*



pub fn load_scene_safe(
        
        
            x: 
        , 
        
        
            y: 
        , 
        
        
            z: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x4448EB75B4904BDBu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x4448EB75B4904BDBu64;
        
        let result = invoke_raw!(
            hash,
                x, 
                y, 
                z
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn load_scene_raw(
        x: , 
        y: , 
        z: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x4448EB75B4904BDBu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x4448EB75B4904BDBu64;

        invoke_raw_typed!(
            hash,
                x, 
                y, 
                z
        )
    }
}

/// ## Parameters
*



pub fn set_reduce_ped_model_budget_safe(
        
        
            toggle: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x77B5F9A36BF96710u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x77B5F9A36BF96710u64;
        
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
pub fn set_reduce_ped_model_budget_raw(
        toggle: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x77B5F9A36BF96710u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x77B5F9A36BF96710u64;

        invoke_raw_typed!(
            hash,
                toggle
        )
    }
}

/// ## Parameters
*



pub fn request_clip_set_safe(
        
        
            clipSet: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xD2A71E1A77418A49u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xD2A71E1A77418A49u64;
        
        let result = invoke_raw!(
            hash,
                clipSet
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn request_clip_set_raw(
        clipSet: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xD2A71E1A77418A49u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xD2A71E1A77418A49u64;

        invoke_raw_typed!(
            hash,
                clipSet
        )
    }
}

/// ## Parameters
*



pub fn set_vehicle_population_budget_safe(
        
        
            p0: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xCB9E1EB3BE2AF4E9u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xCB9E1EB3BE2AF4E9u64;
        
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
pub fn set_vehicle_population_budget_raw(
        p0: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xCB9E1EB3BE2AF4E9u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xCB9E1EB3BE2AF4E9u64;

        invoke_raw_typed!(
            hash,
                p0
        )
    }
}

/// Returns true when the srl from BEGIN_SRL is loaded.



pub fn is_srl_loaded_safe(
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xD0263801A4C5B0BBu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xD0263801A4C5B0BBu64;
        
        let result = invoke_raw!(
            hash,
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn is_srl_loaded_raw(
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xD0263801A4C5B0BBu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xD0263801A4C5B0BBu64;

        invoke_raw_typed!(
            hash,
        )
    }
}

/// Unloads the specified animation set. An animation set provides movement animations for a ped.

Animation set and clip set are synonymous. See [`SET_PED_MOVEMENT_CLIPSET`](#_0xAF8A94EDE7712BEF).



pub fn remove_anim_set_safe(
        
        
            animSet: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x16350528F93024B3u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x16350528F93024B3u64;
        
        let result = invoke_raw!(
            hash,
                animSet
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn remove_anim_set_raw(
        animSet: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x16350528F93024B3u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x16350528F93024B3u64;

        invoke_raw_typed!(
            hash,
                animSet
        )
    }
}

/// ## Parameters
*



pub fn is_ipl_active_safe(
        
        
            iplName: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x88A741E44A2B3495u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x88A741E44A2B3495u64;
        
        let result = invoke_raw!(
            hash,
                iplName
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn is_ipl_active_raw(
        iplName: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x88A741E44A2B3495u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x88A741E44A2B3495u64;

        invoke_raw_typed!(
            hash,
                iplName
        )
    }
}

/// ## Parameters
*



pub fn request_anim_dict_safe(
        
        
            animDict: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xD3BD40951412FEF6u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xD3BD40951412FEF6u64;
        
        let result = invoke_raw!(
            hash,
                animDict
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn request_anim_dict_raw(
        animDict: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xD3BD40951412FEF6u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xD3BD40951412FEF6u64;

        invoke_raw_typed!(
            hash,
                animDict
        )
    }
}

/// Sets the memory budget level for ped population.



pub fn set_ped_population_budget_safe(
        
        
            budgetLevel: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x8C95333CFC3340F3u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x8C95333CFC3340F3u64;
        
        let result = invoke_raw!(
            hash,
                budgetLevel
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn set_ped_population_budget_raw(
        budgetLevel: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x8C95333CFC3340F3u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x8C95333CFC3340F3u64;

        invoke_raw_typed!(
            hash,
                budgetLevel
        )
    }
}

/// STOP_PLAYER_SWITCH native function



pub fn stop_player_switch_safe(
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x95C0A5BBDC189AA1u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x95C0A5BBDC189AA1u64;
        
        let result = invoke_raw!(
            hash,
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn stop_player_switch_raw(
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x95C0A5BBDC189AA1u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x95C0A5BBDC189AA1u64;

        invoke_raw_typed!(
            hash,
        )
    }
}

/// ## Parameters
*



pub fn remove_anim_dict_safe(
        
        
            animDict: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xF66A602F829E2A06u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xF66A602F829E2A06u64;
        
        let result = invoke_raw!(
            hash,
                animDict
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn remove_anim_dict_raw(
        animDict: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xF66A602F829E2A06u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xF66A602F829E2A06u64;

        invoke_raw_typed!(
            hash,
                animDict
        )
    }
}

/// ## Return value



pub fn _0xfb199266061f820a_safe(
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xFB199266061F820Au32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xFB199266061F820Au64;
        
        let result = invoke_raw!(
            hash,
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn _0xfb199266061f820a_raw(
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xFB199266061F820Au32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xFB199266061F820Au64;

        invoke_raw_typed!(
            hash,
        )
    }
}

/// ```
 From the b678d decompiled scripts:
 STREAMING::REQUEST_NAMED_PTFX_ASSET("core_snow");
 STREAMING::REQUEST_NAMED_PTFX_ASSET("fm_mission_controler");
 STREAMING::REQUEST_NAMED_PTFX_ASSET("proj_xmas_firework");
 STREAMING::REQUEST_NAMED_PTFX_ASSET("scr_apartment_mp");
 STREAMING::REQUEST_NAMED_PTFX_ASSET("scr_biolab_heist");
 STREAMING::REQUEST_NAMED_PTFX_ASSET("scr_indep_fireworks");
 STREAMING::REQUEST_NAMED_PTFX_ASSET("scr_indep_parachute");
 STREAMING::REQUEST_NAMED_PTFX_ASSET("scr_indep_wheelsmoke");
 STREAMING::REQUEST_NAMED_PTFX_ASSET("scr_mp_cig_plane");
 STREAMING::REQUEST_NAMED_PTFX_ASSET("scr_mp_creator");
 STREAMING::REQUEST_NAMED_PTFX_ASSET("scr_mp_tankbattle");
 STREAMING::REQUEST_NAMED_PTFX_ASSET("scr_ornate_heist");
 STREAMING::REQUEST_NAMED_PTFX_ASSET("scr_prison_break_heist_station");
```



pub fn request_named_ptfx_asset_safe(
        
        
            fxName: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xB80D8756B4668AB6u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xB80D8756B4668AB6u64;
        
        let result = invoke_raw!(
            hash,
                fxName
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn request_named_ptfx_asset_raw(
        fxName: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xB80D8756B4668AB6u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xB80D8756B4668AB6u64;

        invoke_raw_typed!(
            hash,
                fxName
        )
    }
}

/// ## Parameters
*



pub fn set_streaming_safe(
        
        
            toggle: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x6E0C692677008888u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x6E0C692677008888u64;
        
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
pub fn set_streaming_raw(
        toggle: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x6E0C692677008888u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x6E0C692677008888u64;

        invoke_raw_typed!(
            hash,
                toggle
        )
    }
}

/// _0x472397322E92A856 native function



pub fn _0x472397322e92a856_safe(
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x472397322E92A856u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x472397322E92A856u64;
        
        let result = invoke_raw!(
            hash,
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn _0x472397322e92a856_raw(
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x472397322E92A856u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x472397322E92A856u64;

        invoke_raw_typed!(
            hash,
        )
    }
}

/// ## Return value



pub fn is_streamvol_active_safe(
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xBC9823AB80A3DCACu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xBC9823AB80A3DCACu64;
        
        let result = invoke_raw!(
            hash,
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn is_streamvol_active_raw(
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xBC9823AB80A3DCACu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xBC9823AB80A3DCACu64;

        invoke_raw_typed!(
            hash,
        )
    }
}

/// ## Parameters
*



pub fn has_named_ptfx_asset_loaded_safe(
        
        
            fxName: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x8702416E512EC454u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x8702416E512EC454u64;
        
        let result = invoke_raw!(
            hash,
                fxName
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn has_named_ptfx_asset_loaded_raw(
        fxName: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x8702416E512EC454u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x8702416E512EC454u64;

        invoke_raw_typed!(
            hash,
                fxName
        )
    }
}

/// ## Return value
- Returns 5 if the player is in the air (in a state of switch).
- Returns 12 if the player is either not in the air or if the switch is completed.



pub fn get_player_switch_state_safe(
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x470555300D10B2A5u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x470555300D10B2A5u64;
        
        let result = invoke_raw!(
            hash,
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn get_player_switch_state_raw(
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x470555300D10B2A5u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x470555300D10B2A5u64;

        invoke_raw_typed!(
            hash,
        )
    }
}

/// Calls [`REQUEST_MODEL`](#_0x963D27A58DF860AC) with the `STRFLAG_PRIORITY_LOAD` and `STRFLAG_FORCE_LOAD` set.



pub fn request_menu_ped_model_safe(
        
        
            model: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xA0261AEF7ACFC51Eu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xA0261AEF7ACFC51Eu64;
        
        let result = invoke_raw!(
            hash,
                model
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn request_menu_ped_model_raw(
        model: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xA0261AEF7ACFC51Eu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xA0261AEF7ACFC51Eu64;

        invoke_raw_typed!(
            hash,
                model
        )
    }
}

/// CLEAR_FOCUS native function



pub fn clear_focus_safe(
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x31B73D1EA9F01DA2u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x31B73D1EA9F01DA2u64;
        
        let result = invoke_raw!(
            hash,
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn clear_focus_raw(
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x31B73D1EA9F01DA2u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x31B73D1EA9F01DA2u64;

        invoke_raw_typed!(
            hash,
        )
    }
}

/// ALLOW_PLAYER_SWITCH_PAN native function



pub fn allow_player_switch_pan_safe(
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x43D1680C6D19A8E9u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x43D1680C6D19A8E9u64;
        
        let result = invoke_raw!(
            hash,
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn allow_player_switch_pan_raw(
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x43D1680C6D19A8E9u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x43D1680C6D19A8E9u64;

        invoke_raw_typed!(
            hash,
        )
    }
}

/// NEW_LOAD_SCENE_STOP native function



pub fn new_load_scene_stop_safe(
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xC197616D221FF4A4u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xC197616D221FF4A4u64;
        
        let result = invoke_raw!(
            hash,
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn new_load_scene_stop_raw(
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xC197616D221FF4A4u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xC197616D221FF4A4u64;

        invoke_raw_typed!(
            hash,
        )
    }
}

/// ## Return value



pub fn get_number_of_streaming_requests_safe(
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x4060057271CEBC89u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x4060057271CEBC89u64;
        
        let result = invoke_raw!(
            hash,
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn get_number_of_streaming_requests_raw(
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x4060057271CEBC89u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x4060057271CEBC89u64;

        invoke_raw_typed!(
            hash,
        )
    }
}

/// LOAD_ALL_OBJECTS_NOW native function



pub fn load_all_objects_now_safe(
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xBD6E84632DD4CB3Fu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xBD6E84632DD4CB3Fu64;
        
        let result = invoke_raw!(
            hash,
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn load_all_objects_now_raw(
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xBD6E84632DD4CB3Fu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xBD6E84632DD4CB3Fu64;

        invoke_raw_typed!(
            hash,
        )
    }
}

/// DISABLE_SWITCH_OUTRO_FX native function



pub fn disable_switch_outro_fx_safe(
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xBD605B8E0E18B3BBu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xBD605B8E0E18B3BBu64;
        
        let result = invoke_raw!(
            hash,
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn disable_switch_outro_fx_raw(
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xBD605B8E0E18B3BBu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xBD605B8E0E18B3BBu64;

        invoke_raw_typed!(
            hash,
        )
    }
}

/// ```
STREAMING::REQUEST_MODELS_IN_ROOM(l_13BC, "V_FIB01_cur_elev");
STREAMING::REQUEST_MODELS_IN_ROOM(l_13BC, "limbo");
STREAMING::REQUEST_MODELS_IN_ROOM(l_13BB, "V_Office_gnd_lifts");
STREAMING::REQUEST_MODELS_IN_ROOM(l_13BB, "limbo");
STREAMING::REQUEST_MODELS_IN_ROOM(l_13BC, "v_fib01_jan_elev");
STREAMING::REQUEST_MODELS_IN_ROOM(l_13BC, "limbo");
```



pub fn request_models_in_room_safe(
        
        
            interior: 
        , 
        
        
            roomName: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x8A7A40100EDFEC58u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x8A7A40100EDFEC58u64;
        
        let result = invoke_raw!(
            hash,
                interior, 
                roomName
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn request_models_in_room_raw(
        interior: , 
        roomName: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x8A7A40100EDFEC58u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x8A7A40100EDFEC58u64;

        invoke_raw_typed!(
            hash,
                interior, 
                roomName
        )
    }
}

/// ## Parameters
*



pub fn set_player_short_switch_style_safe(
        
        
            style: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x5F2013F8BC24EE69u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x5F2013F8BC24EE69u64;
        
        let result = invoke_raw!(
            hash,
                style
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn set_player_short_switch_style_raw(
        style: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x5F2013F8BC24EE69u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x5F2013F8BC24EE69u64;

        invoke_raw_typed!(
            hash,
                style
        )
    }
}

/// ## Parameters
*



pub fn _0x20c6c7e4eb082a7f_safe(
        
        
            p0: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x20C6C7E4EB082A7Fu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x20C6C7E4EB082A7Fu64;
        
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
pub fn _0x20c6c7e4eb082a7f_raw(
        p0: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x20C6C7E4EB082A7Fu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x20C6C7E4EB082A7Fu64;

        invoke_raw_typed!(
            hash,
                p0
        )
    }
}

/// INIT_CREATOR_BUDGET native function



pub fn init_creator_budget_safe(
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xB5A4DB34FE89B88Au32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xB5A4DB34FE89B88Au64;
        
        let result = invoke_raw!(
            hash,
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn init_creator_budget_raw(
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xB5A4DB34FE89B88Au32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xB5A4DB34FE89B88Au64;

        invoke_raw_typed!(
            hash,
        )
    }
}

/// ## Parameters
*



pub fn does_anim_dict_exist_safe(
        
        
            animDict: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x2DA49C3B79856961u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x2DA49C3B79856961u64;
        
        let result = invoke_raw!(
            hash,
                animDict
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn does_anim_dict_exist_raw(
        animDict: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x2DA49C3B79856961u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x2DA49C3B79856961u64;

        invoke_raw_typed!(
            hash,
                animDict
        )
    }
}

/// ```
Exemple: REQUEST_IPL("TrevorsTrailerTrash");
```



pub fn request_ipl_safe(
        
        
            iplName: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x41B4893843BBDB74u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x41B4893843BBDB74u64;
        
        let result = invoke_raw!(
            hash,
                iplName
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn request_ipl_raw(
        iplName: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x41B4893843BBDB74u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x41B4893843BBDB74u64;

        invoke_raw_typed!(
            hash,
                iplName
        )
    }
}

/// ## Return value



pub fn is_new_load_scene_active_safe(
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xA41A05B6CB741B85u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xA41A05B6CB741B85u64;
        
        let result = invoke_raw!(
            hash,
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn is_new_load_scene_active_raw(
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xA41A05B6CB741B85u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xA41A05B6CB741B85u64;

        invoke_raw_typed!(
            hash,
        )
    }
}

/// ## Return value



pub fn get_player_short_switch_state_safe(
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x20F898A5D9782800u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x20F898A5D9782800u64;
        
        let result = invoke_raw!(
            hash,
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn get_player_short_switch_state_raw(
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x20F898A5D9782800u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x20F898A5D9782800u64;

        invoke_raw_typed!(
            hash,
        )
    }
}

/// ## Return value



pub fn get_lodscale_safe(
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x0C15B0E443B2349Du32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x0C15B0E443B2349Du64;
        
        let result = invoke_raw!(
            hash,
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn get_lodscale_raw(
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x0C15B0E443B2349Du32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x0C15B0E443B2349Du64;

        invoke_raw_typed!(
            hash,
        )
    }
}

/// This native is used to attribute the SRL that BeginSrl is going to load. This is usually used for 'in-game' cinematics (not cutscenes but camera stuff) instead of SetFocusArea because it loads a specific area of the map which is pretty useful when the camera moves from distant areas.
For instance, GTA:O opening cutscene.

https://pastebin.com/2EeKVeLA : a list of SRL found in srllist.meta
https://pastebin.com/zd9XYUWY : here is the content of a SRL file opened with codewalker.



pub fn prefetch_srl_safe(
        
        
            srl: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x3D245789CE12982Cu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x3D245789CE12982Cu64;
        
        let result = invoke_raw!(
            hash,
                srl
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn prefetch_srl_raw(
        srl: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x3D245789CE12982Cu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x3D245789CE12982Cu64;

        invoke_raw_typed!(
            hash,
                srl
        )
    }
}

/// ```
x1, y1, z1 -- Coords of your ped model  
x2, y2, z2 -- Coords of the ped you want to switch to  
```



pub fn get_ideal_player_switch_type_safe(
        
        
            x1: 
        , 
        
        
            y1: 
        , 
        
        
            z1: 
        , 
        
        
            x2: 
        , 
        
        
            y2: 
        , 
        
        
            z2: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xB5D7B26B45720E05u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xB5D7B26B45720E05u64;
        
        let result = invoke_raw!(
            hash,
                x1, 
                y1, 
                z1, 
                x2, 
                y2, 
                z2
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn get_ideal_player_switch_type_raw(
        x1: , 
        y1: , 
        z1: , 
        x2: , 
        y2: , 
        z2: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xB5D7B26B45720E05u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xB5D7B26B45720E05u64;

        invoke_raw_typed!(
            hash,
                x1, 
                y1, 
                z1, 
                x2, 
                y2, 
                z2
        )
    }
}

/// ## Parameters
*



pub fn _0xbed8ca5ff5e04113_safe(
        
        
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
        let hash = 0xBED8CA5FF5E04113u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xBED8CA5FF5E04113u64;
        
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
pub fn _0xbed8ca5ff5e04113_raw(
        p0: , 
        p1: , 
        p2: , 
        p3: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xBED8CA5FF5E04113u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xBED8CA5FF5E04113u64;

        invoke_raw_typed!(
            hash,
                p0, 
                p1, 
                p2, 
                p3
        )
    }
}

/// ALLOW_PLAYER_SWITCH_OUTRO native function



pub fn allow_player_switch_outro_safe(
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x74DE2E8739086740u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x74DE2E8739086740u64;
        
        let result = invoke_raw!(
            hash,
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn allow_player_switch_outro_raw(
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x74DE2E8739086740u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x74DE2E8739086740u64;

        invoke_raw_typed!(
            hash,
        )
    }
}

/// ## Return value



pub fn is_switch_ready_for_descent_safe(
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xDFA80CB25D0A19B3u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xDFA80CB25D0A19B3u64;
        
        let result = invoke_raw!(
            hash,
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn is_switch_ready_for_descent_raw(
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xDFA80CB25D0A19B3u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xDFA80CB25D0A19B3u64;

        invoke_raw_typed!(
            hash,
        )
    }
}

/// ```
Returns whether the specified model exists in the game.  
```



pub fn is_model_valid_safe(
        
        
            model: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xC0296A2EDF545E92u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xC0296A2EDF545E92u64;
        
        let result = invoke_raw!(
            hash,
                model
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn is_model_valid_raw(
        model: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xC0296A2EDF545E92u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xC0296A2EDF545E92u64;

        invoke_raw_typed!(
            hash,
                model
        )
    }
}

/// ## Return value



pub fn get_player_switch_interp_out_current_time_safe(
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x5B48A06DD0E792A5u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x5B48A06DD0E792A5u64;
        
        let result = invoke_raw!(
            hash,
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn get_player_switch_interp_out_current_time_raw(
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x5B48A06DD0E792A5u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x5B48A06DD0E792A5u64;

        invoke_raw_typed!(
            hash,
        )
    }
}

/// ```
NativeDB Introduced: v2189
```



pub fn _get_global_water_type_safe(
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xF741BD853611592Du32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xF741BD853611592Du64;
        
        let result = invoke_raw!(
            hash,
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn _get_global_water_type_raw(
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xF741BD853611592Du32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xF741BD853611592Du64;

        invoke_raw_typed!(
            hash,
        )
    }
}

/// ## Parameters
*



pub fn set_game_pauses_for_streaming_safe(
        
        
            toggle: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x717CD6E6FAEBBEDCu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x717CD6E6FAEBBEDCu64;
        
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
pub fn set_game_pauses_for_streaming_raw(
        toggle: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x717CD6E6FAEBBEDCu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x717CD6E6FAEBBEDCu64;

        invoke_raw_typed!(
            hash,
                toggle
        )
    }
}

/// ## Return value



pub fn is_new_load_scene_loaded_safe(
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x01B8247A7A8B9AD1u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x01B8247A7A8B9AD1u64;
        
        let result = invoke_raw!(
            hash,
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn is_new_load_scene_loaded_raw(
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x01B8247A7A8B9AD1u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x01B8247A7A8B9AD1u64;

        invoke_raw_typed!(
            hash,
        )
    }
}

/// ```
0.0 = no memory used
1.0 = all memory used

Maximum model memory (as defined in common\data\missioncreatordata.meta) is 100 MiB

GET_*
```



pub fn _get_used_creator_model_memory_percentage_safe(
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x3D3D8B3BE5A83D35u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x3D3D8B3BE5A83D35u64;
        
        let result = invoke_raw!(
            hash,
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn _get_used_creator_model_memory_percentage_raw(
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x3D3D8B3BE5A83D35u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x3D3D8B3BE5A83D35u64;

        invoke_raw_typed!(
            hash,
        )
    }
}

/// ## Parameters
*



pub fn streamvol_has_loaded_safe(
        
        
            unused: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x7D41E9D2D17C5B2Du32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x7D41E9D2D17C5B2Du64;
        
        let result = invoke_raw!(
            hash,
                unused
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn streamvol_has_loaded_raw(
        unused: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x7D41E9D2D17C5B2Du32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x7D41E9D2D17C5B2Du64;

        invoke_raw_typed!(
            hash,
                unused
        )
    }
}

/// ```
It seems to make the entity's coords mark the point from which LOD-distances are measured. In my testing, setting a vehicle as the focus entity and moving that vehicle more than 300 distance units away from the player will make the level of detail around the player go down drastically (shadows disappear, textures go extremely low res, etc). The player seems to be the default focus entity.  
```



pub fn set_focus_entity_safe(
        
        
            entity: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x198F77705FA0931Du32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x198F77705FA0931Du64;
        
        let result = invoke_raw!(
            hash,
                entity
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn set_focus_entity_raw(
        entity: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x198F77705FA0931Du32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x198F77705FA0931Du64;

        invoke_raw_typed!(
            hash,
                entity
        )
    }
}

/// ## Return value



pub fn get_player_switch_interp_out_duration_safe(
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x08C2D6C52A3104BBu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x08C2D6C52A3104BBu64;
        
        let result = invoke_raw!(
            hash,
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn get_player_switch_interp_out_duration_raw(
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x08C2D6C52A3104BBu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x08C2D6C52A3104BBu64;

        invoke_raw_typed!(
            hash,
        )
    }
}

/// ## Return value



pub fn _0x5068f488ddb54dd8_safe(
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x5068F488DDB54DD8u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x5068F488DDB54DD8u64;
        
        let result = invoke_raw!(
            hash,
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn _0x5068f488ddb54dd8_raw(
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x5068F488DDB54DD8u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x5068F488DDB54DD8u64;

        invoke_raw_typed!(
            hash,
        )
    }
}

/// ALLOW_PLAYER_SWITCH_ASCENT native function



pub fn allow_player_switch_ascent_safe(
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x8E2A065ABDAE6994u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x8E2A065ABDAE6994u64;
        
        let result = invoke_raw!(
            hash,
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn allow_player_switch_ascent_raw(
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x8E2A065ABDAE6994u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x8E2A065ABDAE6994u64;

        invoke_raw_typed!(
            hash,
        )
    }
}

/// ENABLE_SWITCH_PAUSE_BEFORE_DESCENT native function



pub fn enable_switch_pause_before_descent_safe(
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xD4793DFF3AF2ABCDu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xD4793DFF3AF2ABCDu64;
        
        let result = invoke_raw!(
            hash,
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn enable_switch_pause_before_descent_raw(
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xD4793DFF3AF2ABCDu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xD4793DFF3AF2ABCDu64;

        invoke_raw_typed!(
            hash,
        )
    }
}

/// ```c
enum ePlayerSwitchType  
{  
	SWITCH_TYPE_AUTO = 0,
	SWITCH_TYPE_LONG = 1,
	SWITCH_TYPE_MEDIUM = 2,
	SWITCH_TYPE_SHORT = 3
};  
```


```c
enum eSwitchFlags {
	SKIP_INTRO = 1,
	SKIP_OUTRO = 2,
	PAUSE_BEFORE_PAN = 4,
	PAUSE_BEFORE_OUTRO = 8,
	SKIP_PAN = 16,
	UNKNOWN_DEST = 32,
	DESCENT_ONLY = 64,
	START_FROM_CAMPOS = 128,
	PAUSE_BEFORE_ASCENT = 256,
	PAUSE_BEFORE_DESCENT = 512,
	ALLOW_SNIPER_AIM_INTRO = 1024,
	ALLOW_SNIPER_AIM_OUTRO = 2048,
	SKIP_TOP_DESCENT = 4096,
	SUPPRESS_OUTRO_FX = 8192,
	SUPPRESS_INTRO_FX = 16384,
	DELAY_ASCENT_FX = 32768
}
```



pub fn start_player_switch_safe(
        
        
            from: 
        , 
        
        
            to: 
        , 
        
        
            flags: 
        , 
        
        
            switchType: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xFAA23F2CBA159D67u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xFAA23F2CBA159D67u64;
        
        let result = invoke_raw!(
            hash,
                from, 
                to, 
                flags, 
                switchType
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn start_player_switch_raw(
        from: , 
        to: , 
        flags: , 
        switchType: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xFAA23F2CBA159D67u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xFAA23F2CBA159D67u64;

        invoke_raw_typed!(
            hash,
                from, 
                to, 
                flags, 
                switchType
        )
    }
}

/// ## Parameters
*



pub fn request_collision_at_coord_safe(
        
        
            x: 
        , 
        
        
            y: 
        , 
        
        
            z: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x07503F7948F491A7u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x07503F7948F491A7u64;
        
        let result = invoke_raw!(
            hash,
                x, 
                y, 
                z
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn request_collision_at_coord_raw(
        x: , 
        y: , 
        z: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x07503F7948F491A7u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x07503F7948F491A7u64;

        invoke_raw_typed!(
            hash,
                x, 
                y, 
                z
        )
    }
}

/// ```
Possible p0 values:  
"prologue"  
"Prologue_Main"  
```



pub fn set_mapdatacullbox_enabled_safe(
        
        
            name: 
        , 
        
        
            toggle: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xAF12610C644A35C9u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xAF12610C644A35C9u64;
        
        let result = invoke_raw!(
            hash,
                name, 
                toggle
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn set_mapdatacullbox_enabled_raw(
        name: , 
        toggle: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xAF12610C644A35C9u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xAF12610C644A35C9u64;

        invoke_raw_typed!(
            hash,
                name, 
                toggle
        )
    }
}

/// ## Parameters
*



pub fn set_render_hd_only_safe(
        
        
            toggle: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x40AEFD1A244741F2u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x40AEFD1A244741F2u64;
        
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
pub fn set_render_hd_only_raw(
        toggle: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x40AEFD1A244741F2u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x40AEFD1A244741F2u64;

        invoke_raw_typed!(
            hash,
                toggle
        )
    }
}

/// Gets whether the specified animation set has finished loading. An animation set provides movement animations for a ped.

Animation set and clip set are synonymous. See [`SET_PED_MOVEMENT_CLIPSET`](#_0xAF8A94EDE7712BEF).



pub fn has_anim_set_loaded_safe(
        
        
            animSet: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xC4EA073D86FB29B0u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xC4EA073D86FB29B0u64;
        
        let result = invoke_raw!(
            hash,
                animSet
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn has_anim_set_loaded_raw(
        animSet: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xC4EA073D86FB29B0u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xC4EA073D86FB29B0u64;

        invoke_raw_typed!(
            hash,
                animSet
        )
    }
}

/// _0x03F1A106BDA7DD3E native function



pub fn _0x03f1a106bda7dd3e_safe(
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x03F1A106BDA7DD3Eu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x03F1A106BDA7DD3Eu64;
        
        let result = invoke_raw!(
            hash,
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn _0x03f1a106bda7dd3e_raw(
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x03F1A106BDA7DD3Eu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x03F1A106BDA7DD3Eu64;

        invoke_raw_typed!(
            hash,
        )
    }
}

/// ## Parameters
*



pub fn is_entity_focus_safe(
        
        
            entity: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x2DDFF3FB9075D747u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x2DDFF3FB9075D747u64;
        
        let result = invoke_raw!(
            hash,
                entity
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn is_entity_focus_raw(
        entity: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x2DDFF3FB9075D747u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x2DDFF3FB9075D747u64;

        invoke_raw_typed!(
            hash,
                entity
        )
    }
}

/// _0x1E9057A74FD73E23 native function



pub fn _0x1e9057a74fd73e23_safe(
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x1E9057A74FD73E23u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x1E9057A74FD73E23u64;
        
        let result = invoke_raw!(
            hash,
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn _0x1e9057a74fd73e23_raw(
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x1E9057A74FD73E23u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x1E9057A74FD73E23u64;

        invoke_raw_typed!(
            hash,
        )
    }
}

/// ## Parameters
*



pub fn _0xbeb2d9a1d9a8f55a_safe(
        
        
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
        let hash = 0xBEB2D9A1D9A8F55Au32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xBEB2D9A1D9A8F55Au64;
        
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
pub fn _0xbeb2d9a1d9a8f55a_raw(
        p0: , 
        p1: , 
        p2: , 
        p3: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xBEB2D9A1D9A8F55Au32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xBEB2D9A1D9A8F55Au64;

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



pub fn request_additional_collision_at_coord_safe(
        
        
            x: 
        , 
        
        
            y: 
        , 
        
        
            z: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xC9156DC11411A9EAu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xC9156DC11411A9EAu64;
        
        let result = invoke_raw!(
            hash,
                x, 
                y, 
                z
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn request_additional_collision_at_coord_raw(
        x: , 
        y: , 
        z: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xC9156DC11411A9EAu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xC9156DC11411A9EAu64;

        invoke_raw_typed!(
            hash,
                x, 
                y, 
                z
        )
    }
}

/// CLEAR_HD_AREA native function



pub fn clear_hd_area_safe(
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xCE58B1CFB9290813u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xCE58B1CFB9290813u64;
        
        let result = invoke_raw!(
            hash,
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn clear_hd_area_raw(
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xCE58B1CFB9290813u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xCE58B1CFB9290813u64;

        invoke_raw_typed!(
            hash,
        )
    }
}

/// _0x63EB2B972A218CAC native function



pub fn _0x63eb2b972a218cac_safe(
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x63EB2B972A218CACu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x63EB2B972A218CACu64;
        
        let result = invoke_raw!(
            hash,
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn _0x63eb2b972a218cac_raw(
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x63EB2B972A218CACu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x63EB2B972A218CACu64;

        invoke_raw_typed!(
            hash,
        )
    }
}

/// ```
if (!sub_8f12("START LOAD SCENE SAFE")) {
    if (CUTSCENE::GET_CUTSCENE_TIME() > 4178) {
        STREAMING::_ACCFB4ACF53551B0(1973.845458984375, 3818.447265625, 32.43629837036133, 15.0, 2);
        sub_8e9e("START LOAD SCENE SAFE", 1);
    }
}
(Previously known as STREAMING::_NEW_LOAD_SCENE_START_SAFE)
```



pub fn new_load_scene_start_sphere_safe(
        
        
            x: 
        , 
        
        
            y: 
        , 
        
        
            z: 
        , 
        
        
            radius: 
        , 
        
        
            p4: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xACCFB4ACF53551B0u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xACCFB4ACF53551B0u64;
        
        let result = invoke_raw!(
            hash,
                x, 
                y, 
                z, 
                radius, 
                p4
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn new_load_scene_start_sphere_raw(
        x: , 
        y: , 
        z: , 
        radius: , 
        p4: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xACCFB4ACF53551B0u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xACCFB4ACF53551B0u64;

        invoke_raw_typed!(
            hash,
                x, 
                y, 
                z, 
                radius, 
                p4
        )
    }
}

/// ## Parameters
*



pub fn set_interior_active_safe(
        
        
            interiorID: 
        , 
        
        
            toggle: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xE37B76C387BE28EDu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xE37B76C387BE28EDu64;
        
        let result = invoke_raw!(
            hash,
                interiorID, 
                toggle
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn set_interior_active_raw(
        interiorID: , 
        toggle: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xE37B76C387BE28EDu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xE37B76C387BE28EDu64;

        invoke_raw_typed!(
            hash,
                interiorID, 
                toggle
        )
    }
}

/// 0 - default
1 - HeistIsland

```
NativeDB Introduced: v2189
```



pub fn _load_global_water_type_safe(
        
        
            waterType: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x7E3F55ED251B76D3u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x7E3F55ED251B76D3u64;
        
        let result = invoke_raw!(
            hash,
                waterType
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn _load_global_water_type_raw(
        waterType: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x7E3F55ED251B76D3u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x7E3F55ED251B76D3u64;

        invoke_raw_typed!(
            hash,
                waterType
        )
    }
}

/// ## Parameters
*



pub fn set_reduce_vehicle_model_budget_safe(
        
        
            toggle: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x80C527893080CCF3u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x80C527893080CCF3u64;
        
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
pub fn set_reduce_vehicle_model_budget_raw(
        toggle: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x80C527893080CCF3u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x80C527893080CCF3u64;

        invoke_raw_typed!(
            hash,
                toggle
        )
    }
}

/// ```
All names can be found in playerswitchestablishingshots.meta
```



pub fn set_player_switch_establishing_shot_safe(
        
        
            name: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x0FDE9DBFC0A6BC65u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x0FDE9DBFC0A6BC65u64;
        
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
pub fn set_player_switch_establishing_shot_raw(
        name: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x0FDE9DBFC0A6BC65u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x0FDE9DBFC0A6BC65u64;

        invoke_raw_typed!(
            hash,
                name
        )
    }
}

/// ## Parameters
*



pub fn has_anim_dict_loaded_safe(
        
        
            animDict: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xD031A9162D01088Cu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xD031A9162D01088Cu64;
        
        let result = invoke_raw!(
            hash,
                animDict
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn has_anim_dict_loaded_raw(
        animDict: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xD031A9162D01088Cu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xD031A9162D01088Cu64;

        invoke_raw_typed!(
            hash,
                animDict
        )
    }
}

/// ## Parameters
*



pub fn remove_named_ptfx_asset_safe(
        
        
            fxName: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x5F61EBBE1A00F96Du32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x5F61EBBE1A00F96Du64;
        
        let result = invoke_raw!(
            hash,
                fxName
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn remove_named_ptfx_asset_raw(
        fxName: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x5F61EBBE1A00F96Du32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x5F61EBBE1A00F96Du64;

        invoke_raw_typed!(
            hash,
                fxName
        )
    }
}

/// ## Return value



pub fn is_network_loading_scene_safe(
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x41CA5A33160EA4ABu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x41CA5A33160EA4ABu64;
        
        let result = invoke_raw!(
            hash,
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn is_network_loading_scene_raw(
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x41CA5A33160EA4ABu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x41CA5A33160EA4ABu64;

        invoke_raw_typed!(
            hash,
        )
    }
}

/// SHUTDOWN_CREATOR_BUDGET native function



pub fn shutdown_creator_budget_safe(
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xCCE26000E9A6FAD7u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xCCE26000E9A6FAD7u64;
        
        let result = invoke_raw!(
            hash,
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn shutdown_creator_budget_raw(
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xCCE26000E9A6FAD7u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xCCE26000E9A6FAD7u64;

        invoke_raw_typed!(
            hash,
        )
    }
}

/// ## Return value



pub fn get_player_switch_type_safe(
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xB3C94A90D9FC9E62u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xB3C94A90D9FC9E62u64;
        
        let result = invoke_raw!(
            hash,
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn get_player_switch_type_raw(
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xB3C94A90D9FC9E62u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xB3C94A90D9FC9E62u64;

        invoke_raw_typed!(
            hash,
        )
    }
}

/// After using [`SWITCH_TO_MULTI_FIRSTPART`](#_0xAAB3200ED59016BC) , use this native to smoothly return the camera to the player's character.



pub fn switch_to_multi_secondpart_safe(
        
        
            ped: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xD8295AF639FD9CB8u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xD8295AF639FD9CB8u64;
        
        let result = invoke_raw!(
            hash,
                ped
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn switch_to_multi_secondpart_raw(
        ped: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xD8295AF639FD9CB8u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xD8295AF639FD9CB8u64;

        invoke_raw_typed!(
            hash,
                ped
        )
    }
}

/// ## Parameters
*



pub fn set_player_switch_outro_safe(
        
        
            cameraCoordX: 
        , 
        
        
            cameraCoordY: 
        , 
        
        
            cameraCoordZ: 
        , 
        
        
            camRotationX: 
        , 
        
        
            camRotationY: 
        , 
        
        
            camRotationZ: 
        , 
        
        
            camFov: 
        , 
        
        
            camFarClip: 
        , 
        
        
            rotationOrder: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xC208B673CE446B61u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xC208B673CE446B61u64;
        
        let result = invoke_raw!(
            hash,
                cameraCoordX, 
                cameraCoordY, 
                cameraCoordZ, 
                camRotationX, 
                camRotationY, 
                camRotationZ, 
                camFov, 
                camFarClip, 
                rotationOrder
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn set_player_switch_outro_raw(
        cameraCoordX: , 
        cameraCoordY: , 
        cameraCoordZ: , 
        camRotationX: , 
        camRotationY: , 
        camRotationZ: , 
        camFov: , 
        camFarClip: , 
        rotationOrder: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xC208B673CE446B61u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xC208B673CE446B61u64;

        invoke_raw_typed!(
            hash,
                cameraCoordX, 
                cameraCoordY, 
                cameraCoordZ, 
                camRotationX, 
                camRotationY, 
                camRotationZ, 
                camFov, 
                camFarClip, 
                rotationOrder
        )
    }
}

/// ## Parameters
*



pub fn set_srl_time_safe(
        
        
            p0: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xA74A541C6884E7B8u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xA74A541C6884E7B8u64;
        
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
pub fn set_srl_time_raw(
        p0: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xA74A541C6884E7B8u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xA74A541C6884E7B8u64;

        invoke_raw_typed!(
            hash,
                p0
        )
    }
}

/// ```
Always returns zero.
```



pub fn streamvol_create_frustum_safe(
        
        
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
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x1F3F018BC3AFA77Cu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x1F3F018BC3AFA77Cu64;
        
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
                p8
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn streamvol_create_frustum_raw(
        p0: , 
        p1: , 
        p2: , 
        p3: , 
        p4: , 
        p5: , 
        p6: , 
        p7: , 
        p8: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x1F3F018BC3AFA77Cu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x1F3F018BC3AFA77Cu64;

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
                p8
        )
    }
}

/// ## Parameters
*



pub fn set_hd_area_safe(
        
        
            x: 
        , 
        
        
            y: 
        , 
        
        
            z: 
        , 
        
        
            radius: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xB85F26619073E775u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xB85F26619073E775u64;
        
        let result = invoke_raw!(
            hash,
                x, 
                y, 
                z, 
                radius
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn set_hd_area_raw(
        x: , 
        y: , 
        z: , 
        radius: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xB85F26619073E775u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xB85F26619073E775u64;

        invoke_raw_typed!(
            hash,
                x, 
                y, 
                z, 
                radius
        )
    }
}

/// NETWORK_UPDATE_LOAD_SCENE native function



pub fn network_update_load_scene_safe(
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xC4582015556D1C46u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xC4582015556D1C46u64;
        
        let result = invoke_raw!(
            hash,
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn network_update_load_scene_raw(
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xC4582015556D1C46u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xC4582015556D1C46u64;

        invoke_raw_typed!(
            hash,
        )
    }
}

/// ```
`radius` value is usually between `3f` and `7000f` in original 1868 scripts.
`p7` is 0, 1, 2, 3 or 4 used in decompiled scripts, 0 is by far the most common.
Returns True if success, used only 7 times in decompiled scripts of 1868
```



pub fn new_load_scene_start_safe(
        
        
            posX: 
        , 
        
        
            posY: 
        , 
        
        
            posZ: 
        , 
        
        
            offsetX: 
        , 
        
        
            offsetY: 
        , 
        
        
            offsetZ: 
        , 
        
        
            radius: 
        , 
        
        
            p7: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x212A8D0D2BABFAC2u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x212A8D0D2BABFAC2u64;
        
        let result = invoke_raw!(
            hash,
                posX, 
                posY, 
                posZ, 
                offsetX, 
                offsetY, 
                offsetZ, 
                radius, 
                p7
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn new_load_scene_start_raw(
        posX: , 
        posY: , 
        posZ: , 
        offsetX: , 
        offsetY: , 
        offsetZ: , 
        radius: , 
        p7: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x212A8D0D2BABFAC2u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x212A8D0D2BABFAC2u64;

        invoke_raw_typed!(
            hash,
                posX, 
                posY, 
                posZ, 
                offsetX, 
                offsetY, 
                offsetZ, 
                radius, 
                p7
        )
    }
}

/// ```
maps script name (thread + 0xD0) by lookup via scriptfx.dat - does nothing when script name is empty
```



pub fn request_ptfx_asset_safe(
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x944955FB2A3935C8u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x944955FB2A3935C8u64;
        
        let result = invoke_raw!(
            hash,
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn request_ptfx_asset_raw(
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x944955FB2A3935C8u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x944955FB2A3935C8u64;

        invoke_raw_typed!(
            hash,
        )
    }
}

/// ```
This is a NOP function. It does nothing at all.  
```



pub fn set_ditch_police_models_safe(
        
        
            toggle: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x42CBE54462D92634u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x42CBE54462D92634u64;
        
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
pub fn set_ditch_police_models_raw(
        toggle: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x42CBE54462D92634u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x42CBE54462D92634u64;

        invoke_raw_typed!(
            hash,
                toggle
        )
    }
}

/// ## Parameters
*



pub fn _0x0811381ef5062fec_safe(
        
        
            p0: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x0811381EF5062FECu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x0811381EF5062FECu64;
        
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
pub fn _0x0811381ef5062fec_raw(
        p0: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x0811381EF5062FECu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x0811381EF5062FECu64;

        invoke_raw_typed!(
            hash,
                p0
        )
    }
}

/// ## Parameters
*



pub fn _0x95a7dabddbb78ae7_safe(
        
        
            iplName1: 
        , 
        
        
            iplName2: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x95A7DABDDBB78AE7u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x95A7DABDDBB78AE7u64;
        
        let result = invoke_raw!(
            hash,
                iplName1, 
                iplName2
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn _0x95a7dabddbb78ae7_raw(
        iplName1: , 
        iplName2: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x95A7DABDDBB78AE7u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x95A7DABDDBB78AE7u64;

        invoke_raw_typed!(
            hash,
                iplName1, 
                iplName2
        )
    }
}

/// **This native does absolutely nothing, just a nullsub**



pub fn _0x4e52e752c76e7e7a_safe(
        
        
            p0: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x4E52E752C76E7E7Au32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x4E52E752C76E7E7Au64;
        
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
pub fn _0x4e52e752c76e7e7a_raw(
        p0: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x4E52E752C76E7E7Au32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x4E52E752C76E7E7Au64;

        invoke_raw_typed!(
            hash,
                p0
        )
    }
}

/// Returns whether the specified model (archetype) is currently loaded.

Note that this will return 'true' even if the model has been requested and loaded by something other than the current script, if you're intending to actually use the model in a later frame, you should call REQUEST\_MODEL anyway.



pub fn has_model_loaded_safe(
        
        
            model: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x98A4EB5D89A0C952u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x98A4EB5D89A0C952u64;
        
        let result = invoke_raw!(
            hash,
                model
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn has_model_loaded_raw(
        model: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x98A4EB5D89A0C952u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x98A4EB5D89A0C952u64;

        invoke_raw_typed!(
            hash,
                model
        )
    }
}

/// You can check if the player is in a Switch state with [`IS_PLAYER_SWITCH_IN_PROGRESS`](#_0xD9D2CFFF49FAB35F).

_**Note:** Doesn't act normally when used on Mount Chiliad._



pub fn switch_to_multi_firstpart_safe(
        
        
            ped: 
        , 
        
        
            flags: 
        , 
        
        
            switchType: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xAAB3200ED59016BCu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xAAB3200ED59016BCu64;
        
        let result = invoke_raw!(
            hash,
                ped, 
                flags, 
                switchType
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn switch_to_multi_firstpart_raw(
        ped: , 
        flags: , 
        switchType: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xAAB3200ED59016BCu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xAAB3200ED59016BCu64;

        invoke_raw_typed!(
            hash,
                ped, 
                flags, 
                switchType
        )
    }
}

/// ## Parameters
*



pub fn _is_model_a_ped_safe(
        
        
            model: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x75816577FEA6DAD5u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x75816577FEA6DAD5u64;
        
        let result = invoke_raw!(
            hash,
                model
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn _is_model_a_ped_raw(
        model: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x75816577FEA6DAD5u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x75816577FEA6DAD5u64;

        invoke_raw_typed!(
            hash,
                model
        )
    }
}

/// NETWORK_STOP_LOAD_SCENE native function



pub fn network_stop_load_scene_safe(
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x64E630FAF5F60F44u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x64E630FAF5F60F44u64;
        
        let result = invoke_raw!(
            hash,
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn network_stop_load_scene_raw(
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x64E630FAF5F60F44u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x64E630FAF5F60F44u64;

        invoke_raw_typed!(
            hash,
        )
    }
}

/// Enables the specified island. For more information, see islandhopper.meta

```
NativeDB Introduced: v2189
```



pub fn set_island_enabled_safe(
        
        
            islandName: 
        , 
        
        
            toggle: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x9A9D1BA639675CF1u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x9A9D1BA639675CF1u64;
        
        let result = invoke_raw!(
            hash,
                islandName, 
                toggle
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn set_island_enabled_raw(
        islandName: , 
        toggle: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x9A9D1BA639675CF1u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x9A9D1BA639675CF1u64;

        invoke_raw_typed!(
            hash,
                islandName, 
                toggle
        )
    }
}

/// ## Return value



pub fn is_switch_skipping_descent_safe(
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x5B74EA8CFD5E3E7Eu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x5B74EA8CFD5E3E7Eu64;
        
        let result = invoke_raw!(
            hash,
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn is_switch_skipping_descent_raw(
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x5B74EA8CFD5E3E7Eu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x5B74EA8CFD5E3E7Eu64;

        invoke_raw_typed!(
            hash,
        )
    }
}

/// ## Return value



pub fn get_player_switch_jump_cut_index_safe(
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x78C0D93253149435u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x78C0D93253149435u64;
        
        let result = invoke_raw!(
            hash,
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn get_player_switch_jump_cut_index_raw(
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x78C0D93253149435u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x78C0D93253149435u64;

        invoke_raw_typed!(
            hash,
        )
    }
}

/// Request a model (archetype) to be loaded for use by the current script. Use SET\_MODEL\_AS\_NO\_LONGER\_NEEDED when done using the model in script.



pub fn request_model_safe(
        
        
            model: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x963D27A58DF860ACu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x963D27A58DF860ACu64;
        
        let result = invoke_raw!(
            hash,
                model
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn request_model_raw(
        model: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x963D27A58DF860ACu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x963D27A58DF860ACu64;

        invoke_raw_typed!(
            hash,
                model
        )
    }
}

/// IPL list can be found [here](https://gist.github.com/4mmonium/4c8a076b5f712a7cc64666003009a2e7).



pub fn remove_ipl_safe(
        
        
            iplName: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xEE6C5AD3ECE0A82Du32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xEE6C5AD3ECE0A82Du64;
        
        let result = invoke_raw!(
            hash,
                iplName
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn remove_ipl_raw(
        iplName: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xEE6C5AD3ECE0A82Du32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xEE6C5AD3ECE0A82Du64;

        invoke_raw_typed!(
            hash,
                iplName
        )
    }
}

/// ## Parameters
*



pub fn request_collision_for_model_safe(
        
        
            model: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x923CB32A3B874FCBu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x923CB32A3B874FCBu64;
        
        let result = invoke_raw!(
            hash,
                model
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn request_collision_for_model_raw(
        model: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x923CB32A3B874FCBu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x923CB32A3B874FCBu64;

        invoke_raw_typed!(
            hash,
                model
        )
    }
}

/// Starts loading the specified animation set. An animation set provides movement animations for a ped. See [`SET_PED_MOVEMENT_CLIPSET`](#_0xAF8A94EDE7712BEF).



pub fn request_anim_set_safe(
        
        
            animSet: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x6EA47DAE7FAD0EEDu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x6EA47DAE7FAD0EEDu64;
        
        let result = invoke_raw!(
            hash,
                animSet
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn request_anim_set_raw(
        animSet: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x6EA47DAE7FAD0EEDu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x6EA47DAE7FAD0EEDu64;

        invoke_raw_typed!(
            hash,
                animSet
        )
    }
}

/// Clear the current srl and stop rendering the area selected by PREFETCH_SRL and started with BEGIN_SRL.



pub fn end_srl_safe(
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x0A41540E63C9EE17u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x0A41540E63C9EE17u64;
        
        let result = invoke_raw!(
            hash,
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn end_srl_raw(
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x0A41540E63C9EE17u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x0A41540E63C9EE17u64;

        invoke_raw_typed!(
            hash,
        )
    }
}

/// ```
This allows you to override "extended distance scaling" setting. Needs to be called each frame.
Max scaling seems to be 200.0, normal is 1.0
See https://gfycat.com/DetailedHauntingIncatern
```



pub fn override_lodscale_this_frame_safe(
        
        
            scaling: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xA76359FC80B2438Eu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xA76359FC80B2438Eu64;
        
        let result = invoke_raw!(
            hash,
                scaling
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn override_lodscale_this_frame_raw(
        scaling: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xA76359FC80B2438Eu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xA76359FC80B2438Eu64;

        invoke_raw_typed!(
            hash,
                scaling
        )
    }
}

/// ## Parameters
*



pub fn add_model_to_creator_budget_safe(
        
        
            modelHash: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x0BC3144DEB678666u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x0BC3144DEB678666u64;
        
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
pub fn add_model_to_creator_budget_raw(
        modelHash: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x0BC3144DEB678666u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x0BC3144DEB678666u64;

        invoke_raw_typed!(
            hash,
                modelHash
        )
    }
}

/// ## Parameters
*



pub fn has_collision_for_model_loaded_safe(
        
        
            model: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x22CCA434E368F03Au32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x22CCA434E368F03Au64;
        
        let result = invoke_raw!(
            hash,
                model
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn has_collision_for_model_loaded_raw(
        model: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x22CCA434E368F03Au32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x22CCA434E368F03Au64;

        invoke_raw_typed!(
            hash,
                model
        )
    }
}

/// ## Return value



pub fn has_ptfx_asset_loaded_safe(
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xCA7D9B86ECA7481Bu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xCA7D9B86ECA7481Bu64;
        
        let result = invoke_raw!(
            hash,
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn has_ptfx_asset_loaded_raw(
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xCA7D9B86ECA7481Bu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xCA7D9B86ECA7481Bu64;

        invoke_raw_typed!(
            hash,
        )
    }
}

/// ```
Always returns zero.
```



pub fn streamvol_create_line_safe(
        
        
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
        let hash = 0x0AD9710CEE2F590Fu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x0AD9710CEE2F590Fu64;
        
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
pub fn streamvol_create_line_raw(
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
        let hash = 0x0AD9710CEE2F590Fu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x0AD9710CEE2F590Fu64;

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



pub fn streamvol_delete_safe(
        
        
            unused: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x1EE7D8DF4425F053u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x1EE7D8DF4425F053u64;
        
        let result = invoke_raw!(
            hash,
                unused
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn streamvol_delete_raw(
        unused: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x1EE7D8DF4425F053u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x1EE7D8DF4425F053u64;

        invoke_raw_typed!(
            hash,
                unused
        )
    }
}

/// ```
Override the area where the camera will render the terrain.
p3, p4 and p5 are usually set to 0.0
```



pub fn set_focus_pos_and_vel_safe(
        
        
            x: 
        , 
        
        
            y: 
        , 
        
        
            z: 
        , 
        
        
            offsetX: 
        , 
        
        
            offsetY: 
        , 
        
        
            offsetZ: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xBB7454BAFF08FE25u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xBB7454BAFF08FE25u64;
        
        let result = invoke_raw!(
            hash,
                x, 
                y, 
                z, 
                offsetX, 
                offsetY, 
                offsetZ
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn set_focus_pos_and_vel_raw(
        x: , 
        y: , 
        z: , 
        offsetX: , 
        offsetY: , 
        offsetZ: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xBB7454BAFF08FE25u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xBB7454BAFF08FE25u64;

        invoke_raw_typed!(
            hash,
                x, 
                y, 
                z, 
                offsetX, 
                offsetY, 
                offsetZ
        )
    }
}

/// ```
Check if model is in cdimage(rpf)  
```



pub fn is_model_in_cdimage_safe(
        
        
            model: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x35B9E0803292B641u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x35B9E0803292B641u64;
        
        let result = invoke_raw!(
            hash,
                model
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn is_model_in_cdimage_raw(
        model: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x35B9E0803292B641u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x35B9E0803292B641u64;

        invoke_raw_typed!(
            hash,
                model
        )
    }
}

/// ```
Alias for REMOVE_ANIM_SET.  
```



pub fn remove_clip_set_safe(
        
        
            clipSet: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x01F73A131C18CD94u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x01F73A131C18CD94u64;
        
        let result = invoke_raw!(
            hash,
                clipSet
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn remove_clip_set_raw(
        clipSet: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x01F73A131C18CD94u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x01F73A131C18CD94u64;

        invoke_raw_typed!(
            hash,
                clipSet
        )
    }
}

/// ```
Always returns zero.
```



pub fn streamvol_create_sphere_safe(
        
        
            x: 
        , 
        
        
            y: 
        , 
        
        
            z: 
        , 
        
        
            rad: 
        , 
        
        
            p4: 
        , 
        
        
            p5: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x219C7B8D53E429FDu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x219C7B8D53E429FDu64;
        
        let result = invoke_raw!(
            hash,
                x, 
                y, 
                z, 
                rad, 
                p4, 
                p5
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn streamvol_create_sphere_raw(
        x: , 
        y: , 
        z: , 
        rad: , 
        p4: , 
        p5: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x219C7B8D53E429FDu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x219C7B8D53E429FDu64;

        invoke_raw_typed!(
            hash,
                x, 
                y, 
                z, 
                rad, 
                p4, 
                p5
        )
    }
}

/// ```
Returns whether the specified model represents a vehicle.  
```



pub fn is_model_a_vehicle_safe(
        
        
            model: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x19AAC8F07BFEC53Eu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x19AAC8F07BFEC53Eu64;
        
        let result = invoke_raw!(
            hash,
                model
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn is_model_a_vehicle_raw(
        model: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x19AAC8F07BFEC53Eu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x19AAC8F07BFEC53Eu64;

        invoke_raw_typed!(
            hash,
                model
        )
    }
}

/// BEGIN_SRL native function



pub fn begin_srl_safe(
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x9BADDC94EF83B823u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x9BADDC94EF83B823u64;
        
        let result = invoke_raw!(
            hash,
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn begin_srl_raw(
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x9BADDC94EF83B823u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x9BADDC94EF83B823u64;

        invoke_raw_typed!(
            hash,
        )
    }
}

/// _0xF4A0DADB70F57FA6 native function



pub fn _0xf4a0dadb70f57fa6_safe(
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xF4A0DADB70F57FA6u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xF4A0DADB70F57FA6u64;
        
        let result = invoke_raw!(
            hash,
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn _0xf4a0dadb70f57fa6_raw(
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xF4A0DADB70F57FA6u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xF4A0DADB70F57FA6u64;

        invoke_raw_typed!(
            hash,
        )
    }
}

/// ALLOW_PLAYER_SWITCH_DESCENT native function



pub fn allow_player_switch_descent_safe(
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xAD5FDF34B81BFE79u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xAD5FDF34B81BFE79u64;
        
        let result = invoke_raw!(
            hash,
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn allow_player_switch_descent_raw(
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xAD5FDF34B81BFE79u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xAD5FDF34B81BFE79u64;

        invoke_raw_typed!(
            hash,
        )
    }
}

/// REMOVE_PTFX_ASSET native function



pub fn remove_ptfx_asset_safe(
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x88C6814073DD4A73u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x88C6814073DD4A73u64;
        
        let result = invoke_raw!(
            hash,
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn remove_ptfx_asset_raw(
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x88C6814073DD4A73u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x88C6814073DD4A73u64;

        invoke_raw_typed!(
            hash,
        )
    }
}

/// ## Return value



pub fn _0x71e7b2e657449aad_safe(
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x71E7B2E657449AADu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x71E7B2E657449AADu64;
        
        let result = invoke_raw!(
            hash,
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn _0x71e7b2e657449aad_raw(
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x71E7B2E657449AADu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x71E7B2E657449AADu64;

        invoke_raw_typed!(
            hash,
        )
    }
}

