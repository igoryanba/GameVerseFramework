//! FILES native functions
//! 
//! Functions for the files category (local)

use crate::types::*;
use crate::errors::{NativeError, NativeResult, validation};
use nalgebra::Vector3;
use tracing::{debug, warn};

/// ## Parameters
*



pub fn init_shop_ped_component_safe(
        
        
            outComponent: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x1E8C308FD312C036u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x1E8C308FD312C036u64;
        
        let result = invoke_raw!(
            hash,
                outComponent
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn init_shop_ped_component_raw(
        outComponent: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x1E8C308FD312C036u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x1E8C308FD312C036u64;

        invoke_raw_typed!(
            hash,
                outComponent
        )
    }
}

/// ```
From fm_deathmatch_creator and fm_race_creator:

FILES::_UNLOAD_CONTENT_CHANGE_SET_GROUP(joaat("GROUP_MAP_SP"));
FILES::_LOAD_CONTENT_CHANGE_SET_GROUP(joaat("GROUP_MAP"));

NativeDB Introduced: v1604
```



pub fn _unload_content_change_set_group_safe(
        
        
            hash: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x3C1978285B036B25u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x3C1978285B036B25u64;
        
        let result = invoke_raw!(
            hash,
                hash
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn _unload_content_change_set_group_raw(
        hash: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x3C1978285B036B25u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x3C1978285B036B25u64;

        invoke_raw_typed!(
            hash,
                hash
        )
    }
}

/// ## Parameters
*



pub fn is_content_item_locked_safe(
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xD4D7B033C3AA243Cu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xD4D7B033C3AA243Cu64;
        
        let result = invoke_raw!(
            hash,
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn is_content_item_locked_raw(
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xD4D7B033C3AA243Cu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xD4D7B033C3AA243Cu64;

        invoke_raw_typed!(
            hash,
        )
    }
}

/// ```
p0 seems to be the weapon index  
p1 seems to be the weapon component index  
struct DlcComponentData{  
int attachBone;  
int padding1;  
int bActiveByDefault;  
int padding2;  
int unk;  
int padding3;  
int componentHash;  
int padding4;  
int unk2;  
int padding5;  
int componentCost;  
int padding6;  
char nameLabel[64];  
char descLabel[64];  
};  
```



pub fn get_dlc_weapon_component_data_safe(
        
        
            dlcWeaponIndex: 
        , 
        
        
            dlcWeapCompIndex: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x6CF598A2957C2BF8u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x6CF598A2957C2BF8u64;
        
        let result = invoke_raw!(
            hash,
                dlcWeaponIndex, 
                dlcWeapCompIndex
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn get_dlc_weapon_component_data_raw(
        dlcWeaponIndex: , 
        dlcWeapCompIndex: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x6CF598A2957C2BF8u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x6CF598A2957C2BF8u64;

        invoke_raw_typed!(
            hash,
                dlcWeaponIndex, 
                dlcWeapCompIndex
        )
    }
}

/// ## Parameters
*



pub fn is_dlc_vehicle_mod_safe(
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x0564B9FF9631B82Cu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x0564B9FF9631B82Cu64;
        
        let result = invoke_raw!(
            hash,
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn is_dlc_vehicle_mod_raw(
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x0564B9FF9631B82Cu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x0564B9FF9631B82Cu64;

        invoke_raw_typed!(
            hash,
        )
    }
}

/// ## Parameters
*



pub fn get_dlc_vehicle_mod_lock_hash_safe(
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xC098810437312FFFu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xC098810437312FFFu64;
        
        let result = invoke_raw!(
            hash,
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn get_dlc_vehicle_mod_lock_hash_raw(
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xC098810437312FFFu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xC098810437312FFFu64;

        invoke_raw_typed!(
            hash,
        )
    }
}

/// ```
Returns number of possible values of the forcedPropIndex argument of GET_FORCED_PROP.
```



pub fn get_shop_ped_apparel_forced_prop_count_safe(
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x017568A8182D98A6u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x017568A8182D98A6u64;
        
        let result = invoke_raw!(
            hash,
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn get_shop_ped_apparel_forced_prop_count_raw(
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x017568A8182D98A6u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x017568A8182D98A6u64;

        invoke_raw_typed!(
            hash,
        )
    }
}

/// ## Parameters
*



pub fn get_shop_ped_query_prop_safe(
        
        
            componentId: 
        , 
        
        
            outProp: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xDE44A00999B2837Du32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xDE44A00999B2837Du64;
        
        let result = invoke_raw!(
            hash,
                componentId, 
                outProp
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn get_shop_ped_query_prop_raw(
        componentId: , 
        outProp: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xDE44A00999B2837Du32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xDE44A00999B2837Du64;

        invoke_raw_typed!(
            hash,
                componentId, 
                outProp
        )
    }
}

/// ## Parameters
*



pub fn get_hash_name_for_prop_safe(
        
        
            entity: 
        , 
        
        
            componentId: 
        , 
        
        
            propIndex: 
        , 
        
        
            propTextureIndex: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x5D6160275CAEC8DDu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x5D6160275CAEC8DDu64;
        
        let result = invoke_raw!(
            hash,
                entity, 
                componentId, 
                propIndex, 
                propTextureIndex
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn get_hash_name_for_prop_raw(
        entity: , 
        componentId: , 
        propIndex: , 
        propTextureIndex: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x5D6160275CAEC8DDu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x5D6160275CAEC8DDu64;

        invoke_raw_typed!(
            hash,
                entity, 
                componentId, 
                propIndex, 
                propTextureIndex
        )
    }
}

/// ## Parameters
*



pub fn does_shop_ped_apparel_have_restriction_tag_safe(
        
        
            componentHash: 
        , 
        
        
            restrictionTagHash: 
        , 
        
        
            componentId: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x341DE7ED1D2A1BFDu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x341DE7ED1D2A1BFDu64;
        
        let result = invoke_raw!(
            hash,
                componentHash, 
                restrictionTagHash, 
                componentId
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn does_shop_ped_apparel_have_restriction_tag_raw(
        componentHash: , 
        restrictionTagHash: , 
        componentId: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x341DE7ED1D2A1BFDu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x341DE7ED1D2A1BFDu64;

        invoke_raw_typed!(
            hash,
                componentHash, 
                restrictionTagHash, 
                componentId
        )
    }
}

/// Returns the total number of DLC weapons that are available in SP (availableInSP field in shop_weapon.meta).

```
NativeDB Introduced: v2060
```



pub fn _get_num_dlc_weapons_sp_safe(
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x4160B65AE085B5A9u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x4160B65AE085B5A9u64;
        
        let result = invoke_raw!(
            hash,
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn _get_num_dlc_weapons_sp_raw(
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x4160B65AE085B5A9u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x4160B65AE085B5A9u64;

        invoke_raw_typed!(
            hash,
        )
    }
}

/// ```
Returns number of possible values of the forcedComponentIndex argument of GET_FORCED_COMPONENT.
```



pub fn get_shop_ped_apparel_forced_component_count_safe(
        
        
            componentHash: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xC6B9DB42C04DD8C3u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xC6B9DB42C04DD8C3u64;
        
        let result = invoke_raw!(
            hash,
                componentHash
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn get_shop_ped_apparel_forced_component_count_raw(
        componentHash: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xC6B9DB42C04DD8C3u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xC6B9DB42C04DD8C3u64;

        invoke_raw_typed!(
            hash,
                componentHash
        )
    }
}

/// Same as GET_DLC_WEAPON_COMPONENT_DATA but only works for DLC components that are available in SP.

```
NativeDB Introduced: v2060
```



pub fn _get_dlc_weapon_component_data_sp_safe(
        
        
            dlcWeaponIndex: 
        , 
        
        
            dlcWeapCompIndex: 
        , 
        
        
            ComponentDataPtr: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x31D5E073B6F93CDCu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x31D5E073B6F93CDCu64;
        
        let result = invoke_raw!(
            hash,
                dlcWeaponIndex, 
                dlcWeapCompIndex, 
                ComponentDataPtr
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn _get_dlc_weapon_component_data_sp_raw(
        dlcWeaponIndex: , 
        dlcWeapCompIndex: , 
        ComponentDataPtr: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x31D5E073B6F93CDCu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x31D5E073B6F93CDCu64;

        invoke_raw_typed!(
            hash,
                dlcWeaponIndex, 
                dlcWeapCompIndex, 
                ComponentDataPtr
        )
    }
}

/// ## Parameters
*



pub fn get_shop_ped_outfit_locate_safe(
        
        
            p0: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x073CA26B079F956Eu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x073CA26B079F956Eu64;
        
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
pub fn get_shop_ped_outfit_locate_raw(
        p0: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x073CA26B079F956Eu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x073CA26B079F956Eu64;

        invoke_raw_typed!(
            hash,
                p0
        )
    }
}

/// ## Parameters
*



pub fn _get_variant_prop_safe(
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xD81B7F27BC773E66u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xD81B7F27BC773E66u64;
        
        let result = invoke_raw!(
            hash,
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn _get_variant_prop_raw(
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xD81B7F27BC773E66u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xD81B7F27BC773E66u64;

        invoke_raw_typed!(
            hash,
        )
    }
}

/// ```
characters
0: Michael
1: Franklin
2: Trevor
3: MPMale
4: MPFemale
```



pub fn setup_shop_ped_outfit_query_safe(
        
        
            character: 
        , 
        
        
            p1: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xF3FBE2D50A6A8C28u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xF3FBE2D50A6A8C28u64;
        
        let result = invoke_raw!(
            hash,
                character, 
                p1
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn setup_shop_ped_outfit_query_raw(
        character: , 
        p1: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xF3FBE2D50A6A8C28u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xF3FBE2D50A6A8C28u64;

        invoke_raw_typed!(
            hash,
                character, 
                p1
        )
    }
}

/// ## Parameters
*



pub fn get_hash_name_for_component_safe(
        
        
            entity: 
        , 
        
        
            componentId: 
        , 
        
        
            drawableVariant: 
        , 
        
        
            textureVariant: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x0368B3A838070348u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x0368B3A838070348u64;
        
        let result = invoke_raw!(
            hash,
                entity, 
                componentId, 
                drawableVariant, 
                textureVariant
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn get_hash_name_for_component_raw(
        entity: , 
        componentId: , 
        drawableVariant: , 
        textureVariant: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x0368B3A838070348u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x0368B3A838070348u64;

        invoke_raw_typed!(
            hash,
                entity, 
                componentId, 
                drawableVariant, 
                textureVariant
        )
    }
}

/// ## Parameters
*



pub fn get_shop_ped_outfit_safe(
        
        
            p0: 
        , 
        
        
            p1: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xB7952076E444979Du32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xB7952076E444979Du64;
        
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
pub fn get_shop_ped_outfit_raw(
        p0: , 
        p1: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xB7952076E444979Du32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xB7952076E444979Du64;

        invoke_raw_typed!(
            hash,
                p0, 
                p1
        )
    }
}

/// ```
More info here: https://gist.github.com/root-cause/3b80234367b0c856d60bf5cb4b826f86
```



pub fn get_shop_ped_component_safe(
        
        
            componentHash: 
        , 
        
        
            outComponent: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x74C0E2A57EC66760u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x74C0E2A57EC66760u64;
        
        let result = invoke_raw!(
            hash,
                componentHash, 
                outComponent
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn get_shop_ped_component_raw(
        componentHash: , 
        outComponent: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x74C0E2A57EC66760u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x74C0E2A57EC66760u64;

        invoke_raw_typed!(
            hash,
                componentHash, 
                outComponent
        )
    }
}

/// ```
Returns the total number of DLC weapons.
```



pub fn get_num_dlc_weapons_safe(
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xEE47635F352DA367u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xEE47635F352DA367u64;
        
        let result = invoke_raw!(
            hash,
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn get_num_dlc_weapons_raw(
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xEE47635F352DA367u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xEE47635F352DA367u64;

        invoke_raw_typed!(
            hash,
        )
    }
}

/// ```
dlcVehicleIndex is 0 to GET_NUM_DLC_VEHICLS()  
```



pub fn get_dlc_vehicle_model_safe(
        
        
            dlcVehicleIndex: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xECC01B7C5763333Cu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xECC01B7C5763333Cu64;
        
        let result = invoke_raw!(
            hash,
                dlcVehicleIndex
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn get_dlc_vehicle_model_raw(
        dlcVehicleIndex: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xECC01B7C5763333Cu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xECC01B7C5763333Cu64;

        invoke_raw_typed!(
            hash,
                dlcVehicleIndex
        )
    }
}

/// ```
dlcWeaponIndex takes a number from 0 - GET_NUM_DLC_WEAPONS() - 1.  
struct DlcWeaponData  
{  
int emptyCheck; //use DLC1::_IS_DLC_DATA_EMPTY on this  
int padding1;  
int weaponHash;  
int padding2;  
int unk;  
int padding3;  
int weaponCost;  
int padding4;  
int ammoCost;  
int padding5;  
int ammoType;  
int padding6;  
int defaultClipSize;  
int padding7;  
char nameLabel[64];  
char descLabel[64];  
char desc2Label[64]; // usually "the" + name  
char upperCaseNameLabel[64];  
};  
```



pub fn get_dlc_weapon_data_safe(
        
        
            dlcWeaponIndex: 
        , 
        
        
            outData: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x79923CD21BECE14Eu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x79923CD21BECE14Eu64;
        
        let result = invoke_raw!(
            hash,
                dlcWeaponIndex, 
                outData
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn get_dlc_weapon_data_raw(
        dlcWeaponIndex: , 
        outData: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x79923CD21BECE14Eu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x79923CD21BECE14Eu64;

        invoke_raw_typed!(
            hash,
                dlcWeaponIndex, 
                outData
        )
    }
}

/// ```
NativeDB Introduced: v2189
```



pub fn get_tattoo_shop_dlc_item_index_safe(
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x10144267DD22866Cu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x10144267DD22866Cu64;
        
        let result = invoke_raw!(
            hash,
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn get_tattoo_shop_dlc_item_index_raw(
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x10144267DD22866Cu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x10144267DD22866Cu64;

        invoke_raw_typed!(
            hash,
        )
    }
}

/// ```
Returns the total number of DLC weapon components.
```



pub fn get_num_dlc_weapon_components_safe(
        
        
            dlcWeaponIndex: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x405425358A7D61FEu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x405425358A7D61FEu64;
        
        let result = invoke_raw!(
            hash,
                dlcWeaponIndex
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn get_num_dlc_weapon_components_raw(
        dlcWeaponIndex: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x405425358A7D61FEu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x405425358A7D61FEu64;

        invoke_raw_typed!(
            hash,
                dlcWeaponIndex
        )
    }
}

/// ## Parameters
*



pub fn setup_shop_ped_apparel_query_safe(
        
        
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
        let hash = 0x50F457823CE6EB5Fu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x50F457823CE6EB5Fu64;
        
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
pub fn setup_shop_ped_apparel_query_raw(
        p0: , 
        p1: , 
        p2: , 
        p3: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x50F457823CE6EB5Fu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x50F457823CE6EB5Fu64;

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
character is 0 for Michael, 1 for Franklin, 2 for Trevor, 3 for freemode male, and 4 for freemode female.
componentId is between 0 and 11 and corresponds to the usual component slots.
p1 could be the outfit number; unsure.
p2 is usually -1; unknown function.
p3 appears to be for selecting between clothes and props; false is used with components/clothes, true is used with props.
p4 is usually -1; unknown function.
componentId is -1 when p3 is true in decompiled scripts.
```



pub fn setup_shop_ped_apparel_query_tu_safe(
        
        
            character: 
        , 
        
        
            p1: 
        , 
        
        
            p2: 
        , 
        
        
            p3: 
        , 
        
        
            p4: 
        , 
        
        
            componentId: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x9BDF59818B1E38C1u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x9BDF59818B1E38C1u64;
        
        let result = invoke_raw!(
            hash,
                character, 
                p1, 
                p2, 
                p3, 
                p4, 
                componentId
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn setup_shop_ped_apparel_query_tu_raw(
        character: , 
        p1: , 
        p2: , 
        p3: , 
        p4: , 
        componentId: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x9BDF59818B1E38C1u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x9BDF59818B1E38C1u64;

        invoke_raw_typed!(
            hash,
                character, 
                p1, 
                p2, 
                p3, 
                p4, 
                componentId
        )
    }
}

/// ## Parameters
*



pub fn _get_shop_ped_apparel_variant_prop_count_safe(
        
        
            propHash: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xD40AAC51E8E4C663u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xD40AAC51E8E4C663u64;
        
        let result = invoke_raw!(
            hash,
                propHash
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn _get_shop_ped_apparel_variant_prop_count_raw(
        propHash: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xD40AAC51E8E4C663u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xD40AAC51E8E4C663u64;

        invoke_raw_typed!(
            hash,
                propHash
        )
    }
}

/// ## Parameters
*



pub fn get_forced_component_safe(
        
        
            componentHash: 
        , 
        
        
            forcedComponentIndex: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x6C93ED8C2F74859Bu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x6C93ED8C2F74859Bu64;
        
        let result = invoke_raw!(
            hash,
                componentHash, 
                forcedComponentIndex
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn get_forced_component_raw(
        componentHash: , 
        forcedComponentIndex: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x6C93ED8C2F74859Bu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x6C93ED8C2F74859Bu64;

        invoke_raw_typed!(
            hash,
                componentHash, 
                forcedComponentIndex
        )
    }
}

/// Returns some sort of index/offset for components.
Needs _GET_NUM_PROPS_FROM_OUTFIT to be called with p3 = false and componentId with the drawable's component slot first, returns -1 otherwise.

```
NativeDB Introduced: v2189
```



pub fn _0x96e2929292a4db77_safe(
        
        
            componentHash: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x96E2929292A4DB77u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x96E2929292A4DB77u64;
        
        let result = invoke_raw!(
            hash,
                componentHash
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn _0x96e2929292a4db77_raw(
        componentHash: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x96E2929292A4DB77u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x96E2929292A4DB77u64;

        invoke_raw_typed!(
            hash,
                componentHash
        )
    }
}

/// ## Parameters
*



pub fn get_shop_ped_apparel_variant_component_count_safe(
        
        
            componentHash: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xC17AD0E5752BECDAu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xC17AD0E5752BECDAu64;
        
        let result = invoke_raw!(
            hash,
                componentHash
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn get_shop_ped_apparel_variant_component_count_raw(
        componentHash: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xC17AD0E5752BECDAu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xC17AD0E5752BECDAu64;

        invoke_raw_typed!(
            hash,
                componentHash
        )
    }
}

/// ## Parameters
*



pub fn get_shop_ped_outfit_prop_variant_safe(
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xA9F9C2E0FDE11CBBu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xA9F9C2E0FDE11CBBu64;
        
        let result = invoke_raw!(
            hash,
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn get_shop_ped_outfit_prop_variant_raw(
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xA9F9C2E0FDE11CBBu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xA9F9C2E0FDE11CBBu64;

        invoke_raw_typed!(
            hash,
        )
    }
}

/// ## Parameters
*



pub fn get_variant_component_safe(
        
        
            componentHash: 
        , 
        
        
            variantComponentIndex: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x6E11F282F11863B6u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x6E11F282F11863B6u64;
        
        let result = invoke_raw!(
            hash,
                componentHash, 
                variantComponentIndex
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn get_variant_component_raw(
        componentHash: , 
        variantComponentIndex: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x6E11F282F11863B6u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x6E11F282F11863B6u64;

        invoke_raw_typed!(
            hash,
                componentHash, 
                variantComponentIndex
        )
    }
}

/// ## Parameters
*



pub fn get_dlc_vehicle_flags_safe(
        
        
            dlcVehicleIndex: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x5549EE11FA22FCF2u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x5549EE11FA22FCF2u64;
        
        let result = invoke_raw!(
            hash,
                dlcVehicleIndex
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn get_dlc_vehicle_flags_raw(
        dlcVehicleIndex: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x5549EE11FA22FCF2u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x5549EE11FA22FCF2u64;

        invoke_raw_typed!(
            hash,
                dlcVehicleIndex
        )
    }
}

/// ```
From fm_deathmatch_creator and fm_race_creator:

FILES::_UNLOAD_CONTENT_CHANGE_SET_GROUP(joaat("GROUP_MAP_SP"));
FILES::_LOAD_CONTENT_CHANGE_SET_GROUP(joaat("GROUP_MAP"));

NativeDB Introduced: v1604
```



pub fn _load_content_change_set_group_safe(
        
        
            hash: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x6BEDF5769AC2DC07u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x6BEDF5769AC2DC07u64;
        
        let result = invoke_raw!(
            hash,
                hash
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn _load_content_change_set_group_raw(
        hash: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x6BEDF5769AC2DC07u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x6BEDF5769AC2DC07u64;

        invoke_raw_typed!(
            hash,
                hash
        )
    }
}

/// ## Parameters
*



pub fn get_forced_prop_safe(
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xE1CA84EBF72E691Du32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xE1CA84EBF72E691Du64;
        
        let result = invoke_raw!(
            hash,
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn get_forced_prop_raw(
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xE1CA84EBF72E691Du32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xE1CA84EBF72E691Du64;

        invoke_raw_typed!(
            hash,
        )
    }
}

/// Returns the total number of DLC vehicles.



pub fn get_num_dlc_vehicles_safe(
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xA7A866D21CD2329Bu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xA7A866D21CD2329Bu64;
        
        let result = invoke_raw!(
            hash,
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn get_num_dlc_vehicles_raw(
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xA7A866D21CD2329Bu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xA7A866D21CD2329Bu64;

        invoke_raw_typed!(
            hash,
        )
    }
}

/// ## Parameters
*



pub fn get_shop_ped_prop_safe(
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x5D5CAFF661DDF6FCu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x5D5CAFF661DDF6FCu64;
        
        let result = invoke_raw!(
            hash,
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn get_shop_ped_prop_raw(
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x5D5CAFF661DDF6FCu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x5D5CAFF661DDF6FCu64;

        invoke_raw_typed!(
            hash,
        )
    }
}

/// ## Parameters
*



pub fn get_shop_ped_outfit_component_variant_safe(
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x19F2A026EDF0013Fu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x19F2A026EDF0013Fu64;
        
        let result = invoke_raw!(
            hash,
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn get_shop_ped_outfit_component_variant_raw(
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x19F2A026EDF0013Fu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x19F2A026EDF0013Fu64;

        invoke_raw_typed!(
            hash,
        )
    }
}

/// Returns the total number of DLC weapon components that are available in SP.

```
NativeDB Introduced: v2060
```



pub fn _get_num_dlc_weapon_components_sp_safe(
        
        
            dlcWeaponIndex: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xAD2A7A6DFF55841Bu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xAD2A7A6DFF55841Bu64;
        
        let result = invoke_raw!(
            hash,
                dlcWeaponIndex
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn _get_num_dlc_weapon_components_sp_raw(
        dlcWeaponIndex: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xAD2A7A6DFF55841Bu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xAD2A7A6DFF55841Bu64;

        invoke_raw_typed!(
            hash,
                dlcWeaponIndex
        )
    }
}

/// ## Parameters
*



pub fn get_shop_ped_query_component_safe(
        
        
            componentId: 
        , 
        
        
            outComponent: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x249E310B2D920699u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x249E310B2D920699u64;
        
        let result = invoke_raw!(
            hash,
                componentId, 
                outComponent
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn get_shop_ped_query_component_raw(
        componentId: , 
        outComponent: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x249E310B2D920699u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x249E310B2D920699u64;

        invoke_raw_typed!(
            hash,
                componentId, 
                outComponent
        )
    }
}

/// Same as GET_DLC_WEAPON_DATA but only works for DLC weapons that are available in SP.

```
NativeDB Introduced: v2060
```



pub fn _get_dlc_weapon_data_sp_safe(
        
        
            dlcWeaponIndex: 
        , 
        
        
            outData: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x310836EE7129BA33u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x310836EE7129BA33u64;
        
        let result = invoke_raw!(
            hash,
                dlcWeaponIndex, 
                outData
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn _get_dlc_weapon_data_sp_raw(
        dlcWeaponIndex: , 
        outData: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x310836EE7129BA33u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x310836EE7129BA33u64;

        invoke_raw_typed!(
            hash,
                dlcWeaponIndex, 
                outData
        )
    }
}

/// ```
struct Outfit_s  
{  
	int mask, torso, pants, parachute, shoes, misc1, tops1, armour, crew, tops2, hat, glasses, earpiece;  
	int maskTexture, torsoTexture, pantsTexture, parachuteTexture, shoesTexture, misc1Texture, tops1Texture,   
		armourTexture, crewTexture, tops2Texture, hatTexture, glassesTexture, earpieceTexture;  
};  
```



pub fn get_shop_ped_query_outfit_safe(
        
        
            outfitIndex: 
        , 
        
        
            outfit: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x6D793F03A631FE56u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x6D793F03A631FE56u64;
        
        let result = invoke_raw!(
            hash,
                outfitIndex, 
                outfit
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn get_shop_ped_query_outfit_raw(
        outfitIndex: , 
        outfit: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x6D793F03A631FE56u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x6D793F03A631FE56u64;

        invoke_raw_typed!(
            hash,
                outfitIndex, 
                outfit
        )
    }
}

/// The Second item in the struct `*(Hash *)(outData + 1)` is the vehicle hash.



pub fn get_dlc_vehicle_data_safe(
        
        
            dlcVehicleIndex: 
        , 
        
        
            outData: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x33468EDC08E371F6u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x33468EDC08E371F6u64;
        
        let result = invoke_raw!(
            hash,
                dlcVehicleIndex, 
                outData
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn get_dlc_vehicle_data_raw(
        dlcVehicleIndex: , 
        outData: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x33468EDC08E371F6u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x33468EDC08E371F6u64;

        invoke_raw_typed!(
            hash,
                dlcVehicleIndex, 
                outData
        )
    }
}

/// Returns data that adheres to the tattoo shop item data that is used in shop_tattoo.meta

Character types:
```c
enum eTattooFaction
{
	TATTOO_SP_MICHAEL = 0,
	TATTOO_SP_FRANKLIN = 1,
	TATTOO_SP_TREVOR = 2,
	TATTOO_MP_FM = 3,
	TATTOO_MP_FM_F = 4
}
```

Returned struct properties:
```c
struct sTattooShopItemValues
{
	// Lock hash, used with IS_CONTENT_ITEM_LOCKED
	int LockHash;
	// Unique ID of this slot. It can also be 0.
	int Index;
	// Collection hash of this tattoo
	int CollectionHash;
	// Preset hash of this tattoo
	int PresetHash;
	// Cost of this tattoo in shops.
	int Cost;
	// Secondary placement of this tattoo.
	int eFacing;
	// Location of this tattoo on the body (for example, for torso there would be chest upper, stomach, etc)
	int UpdateGroup;
	// This tattoo's name in the form of a text label.
	const char* NameTextLabel;
};
```



pub fn get_tattoo_shop_dlc_item_data_safe(
        
        
            characterType: 
        , 
        
        
            decorationIndex: 
        , 
        
        
            outComponent: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xFF56381874F82086u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xFF56381874F82086u64;
        
        let result = invoke_raw!(
            hash,
                characterType, 
                decorationIndex, 
                outComponent
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn get_tattoo_shop_dlc_item_data_raw(
        characterType: , 
        decorationIndex: , 
        outComponent: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xFF56381874F82086u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xFF56381874F82086u64;

        invoke_raw_typed!(
            hash,
                characterType, 
                decorationIndex, 
                outComponent
        )
    }
}

/// ```
Character types:
0 = Michael,
1 = Franklin,
2 = Trevor,
3 = MPMale,
4 = MPFemale
```



pub fn get_num_tattoo_shop_dlc_items_safe(
        
        
            character: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x278F76C3B0A8F109u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x278F76C3B0A8F109u64;
        
        let result = invoke_raw!(
            hash,
                character
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn get_num_tattoo_shop_dlc_items_raw(
        character: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x278F76C3B0A8F109u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x278F76C3B0A8F109u64;

        invoke_raw_typed!(
            hash,
                character
        )
    }
}

/// ## Parameters
*



pub fn init_shop_ped_prop_safe(
        
        
            outProp: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xEB0A2B758F7B850Fu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xEB0A2B758F7B850Fu64;
        
        let result = invoke_raw!(
            hash,
                outProp
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn init_shop_ped_prop_raw(
        outProp: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xEB0A2B758F7B850Fu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xEB0A2B758F7B850Fu64;

        invoke_raw_typed!(
            hash,
                outProp
        )
    }
}

/// Returns some sort of index/offset for props.
Needs _GET_NUM_PROPS_FROM_OUTFIT to be called with p3 = true and componentId = -1 first, returns -1 otherwise.

```
NativeDB Introduced: v2189
```



pub fn _0x6cebe002e58dee97_safe(
        
        
            componentHash: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x6CEBE002E58DEE97u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x6CEBE002E58DEE97u64;
        
        let result = invoke_raw!(
            hash,
                componentHash
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn _0x6cebe002e58dee97_raw(
        componentHash: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x6CEBE002E58DEE97u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x6CEBE002E58DEE97u64;

        invoke_raw_typed!(
            hash,
                componentHash
        )
    }
}

