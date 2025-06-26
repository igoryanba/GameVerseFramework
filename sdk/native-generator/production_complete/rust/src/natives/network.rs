//! NETWORK native functions
//! 
//! Functions for the network category (local)

use crate::types::*;
use crate::errors::{NativeError, NativeResult, validation};
use nalgebra::Vector3;
use tracing::{debug, warn};

/// ```
Does nothing in online but in offline it will cause the screen to fade to black. Nothing happens past then, the screen will sit at black until you restart GTA. Other stuff must be needed to actually host a session.  
```



pub fn network_session_host_safe(
        
        
            p0: 
        , 
        
        
            maxPlayers: 
        , 
        
        
            p2: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x6F3D4ED9BEE4E61Du32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x6F3D4ED9BEE4E61Du64;
        
        let result = invoke_raw!(
            hash,
                p0, 
                maxPlayers, 
                p2
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn network_session_host_raw(
        p0: , 
        maxPlayers: , 
        p2: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x6F3D4ED9BEE4E61Du32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x6F3D4ED9BEE4E61Du64;

        invoke_raw_typed!(
            hash,
                p0, 
                maxPlayers, 
                p2
        )
    }
}

/// ## Parameters
*



pub fn ugc_publish_safe(
        
        
            contentId: 
        , 
        
        
            baseContentId: 
        , 
        
        
            contentTypeName: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x1DE0F5F50D723CAAu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x1DE0F5F50D723CAAu64;
        
        let result = invoke_raw!(
            hash,
                contentId, 
                baseContentId, 
                contentTypeName
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn ugc_publish_raw(
        contentId: , 
        baseContentId: , 
        contentTypeName: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x1DE0F5F50D723CAAu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x1DE0F5F50D723CAAu64;

        invoke_raw_typed!(
            hash,
                contentId, 
                baseContentId, 
                contentTypeName
        )
    }
}

/// ## Parameters
*



pub fn network_does_tunable_exist_hash_safe(
        
        
            tunableContext: 
        , 
        
        
            tunableName: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xE4E53E1419D81127u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xE4E53E1419D81127u64;
        
        let result = invoke_raw!(
            hash,
                tunableContext, 
                tunableName
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn network_does_tunable_exist_hash_raw(
        tunableContext: , 
        tunableName: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xE4E53E1419D81127u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xE4E53E1419D81127u64;

        invoke_raw_typed!(
            hash,
                tunableContext, 
                tunableName
        )
    }
}

/// ## Parameters
*



pub fn network_are_transition_details_valid_safe(
        
        
            p0: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x2615AA2A695930C1u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x2615AA2A695930C1u64;
        
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
pub fn network_are_transition_details_valid_raw(
        p0: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x2615AA2A695930C1u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x2615AA2A695930C1u64;

        invoke_raw_typed!(
            hash,
                p0
        )
    }
}

/// ```
Takes a 24 char buffer. Returns the buffer or "**Invalid**" if CPlayerInfo of the given player cannot be retrieved or the player doesn't exist.
```



pub fn network_player_get_userid_safe(
        
        
            player: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x4927FC39CD0869A0u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x4927FC39CD0869A0u64;
        
        let result = invoke_raw!(
            hash,
                player
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn network_player_get_userid_raw(
        player: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x4927FC39CD0869A0u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x4927FC39CD0869A0u64;

        invoke_raw_typed!(
            hash,
                player
        )
    }
}

/// ## Parameters
*



pub fn network_stop_synchronised_scene_safe(
        
        
            netScene: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xC254481A4574CB2Fu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xC254481A4574CB2Fu64;
        
        let result = invoke_raw!(
            hash,
                netScene
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn network_stop_synchronised_scene_raw(
        netScene: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xC254481A4574CB2Fu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xC254481A4574CB2Fu64;

        invoke_raw_typed!(
            hash,
                netScene
        )
    }
}

/// ```
Does nothing (it's a nullsub).

NativeDB Introduced: v323
```



pub fn _0x6fd992c4a1c1b986_safe(
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x6FD992C4A1C1B986u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x6FD992C4A1C1B986u64;
        
        let result = invoke_raw!(
            hash,
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn _0x6fd992c4a1c1b986_raw(
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x6FD992C4A1C1B986u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x6FD992C4A1C1B986u64;

        invoke_raw_typed!(
            hash,
        )
    }
}

/// ## Return value



pub fn _0x63b406d7884bfa95_safe(
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x63B406D7884BFA95u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x63B406D7884BFA95u64;
        
        let result = invoke_raw!(
            hash,
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn _0x63b406d7884bfa95_raw(
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x63B406D7884BFA95u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x63B406D7884BFA95u64;

        invoke_raw_typed!(
            hash,
        )
    }
}

/// ## Return value



pub fn _0x9fedf86898f100e9_safe(
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x9FEDF86898F100E9u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x9FEDF86898F100E9u64;
        
        let result = invoke_raw!(
            hash,
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn _0x9fedf86898f100e9_raw(
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x9FEDF86898F100E9u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x9FEDF86898F100E9u64;

        invoke_raw_typed!(
            hash,
        )
    }
}

/// Allows scripts to use attachment commands on entities (eg. [`ATTACH_ENTITY_TO_ENTITY`](#_0x6B9BBD38AB0796DF)) that are not controlled by the client.



pub fn network_allow_remote_attachment_modification_safe(
        
        
            entity: 
        , 
        
        
            toggle: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x267C78C60E806B9Au32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x267C78C60E806B9Au64;
        
        let result = invoke_raw!(
            hash,
                entity, 
                toggle
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn network_allow_remote_attachment_modification_raw(
        entity: , 
        toggle: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x267C78C60E806B9Au32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x267C78C60E806B9Au64;

        invoke_raw_typed!(
            hash,
                entity, 
                toggle
        )
    }
}

/// ## Parameters
*



pub fn network_get_presence_invite_playlist_current_safe(
        
        
            p0: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x728C4CC7920CD102u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x728C4CC7920CD102u64;
        
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
pub fn network_get_presence_invite_playlist_current_raw(
        p0: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x728C4CC7920CD102u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x728C4CC7920CD102u64;

        invoke_raw_typed!(
            hash,
                p0
        )
    }
}

/// ```
value must be < 255
```



pub fn network_set_property_id_safe(
        
        
            id: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x1775961C2FBBCB5Cu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x1775961C2FBBCB5Cu64;
        
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
pub fn network_set_property_id_raw(
        id: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x1775961C2FBBCB5Cu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x1775961C2FBBCB5Cu64;

        invoke_raw_typed!(
            hash,
                id
        )
    }
}

/// ## Parameters
*



pub fn _ugc_query_recently_created_content_safe(
        
        
            offset: 
        , 
        
        
            count: 
        , 
        
        
            contentTypeName: 
        , 
        
        
            p3: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x6D4CB481FAC835E8u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x6D4CB481FAC835E8u64;
        
        let result = invoke_raw!(
            hash,
                offset, 
                count, 
                contentTypeName, 
                p3
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn _ugc_query_recently_created_content_raw(
        offset: , 
        count: , 
        contentTypeName: , 
        p3: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x6D4CB481FAC835E8u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x6D4CB481FAC835E8u64;

        invoke_raw_typed!(
            hash,
                offset, 
                count, 
                contentTypeName, 
                p3
        )
    }
}

/// ## Return value



pub fn network_session_is_private_safe(
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xCEF70AA5B3F89BA1u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xCEF70AA5B3F89BA1u64;
        
        let result = invoke_raw!(
            hash,
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn network_session_is_private_raw(
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xCEF70AA5B3F89BA1u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xCEF70AA5B3F89BA1u64;

        invoke_raw_typed!(
            hash,
        )
    }
}

/// ## Parameters
*



pub fn network_attach_synchronised_scene_to_entity_safe(
        
        
            netScene: 
        , 
        
        
            entity: 
        , 
        
        
            bone: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x478DCBD2A98B705Au32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x478DCBD2A98B705Au64;
        
        let result = invoke_raw!(
            hash,
                netScene, 
                entity, 
                bone
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn network_attach_synchronised_scene_to_entity_raw(
        netScene: , 
        entity: , 
        bone: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x478DCBD2A98B705Au32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x478DCBD2A98B705Au64;

        invoke_raw_typed!(
            hash,
                netScene, 
                entity, 
                bone
        )
    }
}

/// ## Return value



pub fn network_is_transition_host_safe(
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x0B824797C9BF2159u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x0B824797C9BF2159u64;
        
        let result = invoke_raw!(
            hash,
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn network_is_transition_host_raw(
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x0B824797C9BF2159u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x0B824797C9BF2159u64;

        invoke_raw_typed!(
            hash,
        )
    }
}

/// ## Parameters
*



pub fn network_access_tunable_int_safe(
        
        
            tunableContext: 
        , 
        
        
            tunableName: 
        , 
        
        
            value: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x8BE1146DFD5D4468u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x8BE1146DFD5D4468u64;
        
        let result = invoke_raw!(
            hash,
                tunableContext, 
                tunableName, 
                value
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn network_access_tunable_int_raw(
        tunableContext: , 
        tunableName: , 
        value: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x8BE1146DFD5D4468u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x8BE1146DFD5D4468u64;

        invoke_raw_typed!(
            hash,
                tunableContext, 
                tunableName, 
                value
        )
    }
}

/// ## Parameters
*



pub fn network_clan_join_safe(
        
        
            clanDesc: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x9FAAA4F4FC71F87Fu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x9FAAA4F4FC71F87Fu64;
        
        let result = invoke_raw!(
            hash,
                clanDesc
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn network_clan_join_raw(
        clanDesc: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x9FAAA4F4FC71F87Fu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x9FAAA4F4FC71F87Fu64;

        invoke_raw_typed!(
            hash,
                clanDesc
        )
    }
}

/// ```
Return the local Participant ID.  
This native is exactly the same as 'PARTICIPANT_ID' native.  
```



pub fn participant_id_to_int_safe(
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x57A3BDDAD8E5AA0Au32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x57A3BDDAD8E5AA0Au64;
        
        let result = invoke_raw!(
            hash,
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn participant_id_to_int_raw(
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x57A3BDDAD8E5AA0Au32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x57A3BDDAD8E5AA0Au64;

        invoke_raw_typed!(
            hash,
        )
    }
}

/// ## Parameters
*



pub fn ugc_get_friend_content_safe(
        
        
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
        let hash = 0xF9E1CCAE8BA4C281u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xF9E1CCAE8BA4C281u64;
        
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
pub fn ugc_get_friend_content_raw(
        p0: , 
        p1: , 
        p2: , 
        p3: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xF9E1CCAE8BA4C281u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xF9E1CCAE8BA4C281u64;

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
Internal logging string: SCRIPT_RESERVING_LOCAL_PEDS
```

```
NativeDB Introduced: v1493
```



pub fn _reserve_network_local_peds_safe(
        
        
            amount: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x2C8DF5D129595281u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x2C8DF5D129595281u64;
        
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
pub fn _reserve_network_local_peds_raw(
        amount: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x2C8DF5D129595281u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x2C8DF5D129595281u64;

        invoke_raw_typed!(
            hash,
                amount
        )
    }
}

/// ```
index2 is unused
```



pub fn get_commerce_item_cat_safe(
        
        
            index: 
        , 
        
        
            index2: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x6F44CBF56D79FAC0u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x6F44CBF56D79FAC0u64;
        
        let result = invoke_raw!(
            hash,
                index, 
                index2
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn get_commerce_item_cat_raw(
        index: , 
        index2: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x6F44CBF56D79FAC0u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x6F44CBF56D79FAC0u64;

        invoke_raw_typed!(
            hash,
                index, 
                index2
        )
    }
}

/// ## Parameters
*



pub fn network_clan_download_membership_safe(
        
        
            networkHandle: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xA989044E70010ABEu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xA989044E70010ABEu64;
        
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
pub fn network_clan_download_membership_raw(
        networkHandle: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xA989044E70010ABEu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xA989044E70010ABEu64;

        invoke_raw_typed!(
            hash,
                networkHandle
        )
    }
}

/// ## Return value



pub fn network_get_instance_id_of_this_script_safe(
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x638A3A81733086DBu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x638A3A81733086DBu64;
        
        let result = invoke_raw!(
            hash,
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn network_get_instance_id_of_this_script_raw(
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x638A3A81733086DBu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x638A3A81733086DBu64;

        invoke_raw_typed!(
            hash,
        )
    }
}

/// ## Parameters
*



pub fn network_get_primary_clan_data_start_safe(
        
        
            p0: 
        , 
        
        
            p1: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xCE86D8191B762107u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xCE86D8191B762107u64;
        
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
pub fn network_get_primary_clan_data_start_raw(
        p0: , 
        p1: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xCE86D8191B762107u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xCE86D8191B762107u64;

        invoke_raw_typed!(
            hash,
                p0, 
                p1
        )
    }
}

/// NETWORK_CLEAR_PROPERTY_ID native function



pub fn network_clear_property_id_safe(
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xC2B82527CA77053Eu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xC2B82527CA77053Eu64;
        
        let result = invoke_raw!(
            hash,
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn network_clear_property_id_raw(
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xC2B82527CA77053Eu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xC2B82527CA77053Eu64;

        invoke_raw_typed!(
            hash,
        )
    }
}

/// From what I can tell it looks like it does the following:

Creates/hosts a new transition to another online session, using this in FiveM will result in other players being disconencted from the server/preventing them from joining. This is most likely because I entered the wrong session parameters since they're pretty much all unknown right now.

You also need to use `NetworkJoinTransition(Player player)` and `NetworkLaunchTransition()`.



pub fn network_host_transition_safe(
        
        
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
        let hash = 0xA60BB5CE242BB254u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xA60BB5CE242BB254u64;
        
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
pub fn network_host_transition_raw(
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
        let hash = 0xA60BB5CE242BB254u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xA60BB5CE242BB254u64;

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



pub fn _0x692d58df40657e8c_safe(
        
        
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
        let hash = 0x692D58DF40657E8Cu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x692D58DF40657E8Cu64;
        
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
pub fn _0x692d58df40657e8c_raw(
        p0: , 
        p1: , 
        p2: , 
        p3: , 
        p4: , 
        p5: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x692D58DF40657E8Cu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x692D58DF40657E8Cu64;

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



pub fn _0xca575c391fea25cc_safe(
        
        
            p0: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xCA575C391FEA25CCu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xCA575C391FEA25CCu64;
        
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
pub fn _0xca575c391fea25cc_raw(
        p0: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xCA575C391FEA25CCu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xCA575C391FEA25CCu64;

        invoke_raw_typed!(
            hash,
                p0
        )
    }
}

/// ## Return value



pub fn network_session_is_closed_crew_safe(
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x74732C6CA90DA2B4u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x74732C6CA90DA2B4u64;
        
        let result = invoke_raw!(
            hash,
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn network_session_is_closed_crew_raw(
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x74732C6CA90DA2B4u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x74732C6CA90DA2B4u64;

        invoke_raw_typed!(
            hash,
        )
    }
}

/// Returns a local synchronized scene handle of a networked synchronised scene.



pub fn network_get_local_scene_from_network_id_safe(
        
        
            netSceneId: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x02C40BF885C567B6u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x02C40BF885C567B6u64;
        
        let result = invoke_raw!(
            hash,
                netSceneId
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn network_get_local_scene_from_network_id_raw(
        netSceneId: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x02C40BF885C567B6u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x02C40BF885C567B6u64;

        invoke_raw_typed!(
            hash,
                netSceneId
        )
    }
}

/// ## Parameters
*



pub fn network_session_set_matchmaking_mental_state_safe(
        
        
            p0: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xF1EEA2DDA9FFA69Du32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xF1EEA2DDA9FFA69Du64;
        
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
pub fn network_session_set_matchmaking_mental_state_raw(
        p0: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xF1EEA2DDA9FFA69Du32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xF1EEA2DDA9FFA69Du64;

        invoke_raw_typed!(
            hash,
                p0
        )
    }
}

/// ## Parameters
*



pub fn ugc_get_get_by_category_safe(
        
        
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
        let hash = 0x678BB03C1A3BD51Eu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x678BB03C1A3BD51Eu64;
        
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
pub fn ugc_get_get_by_category_raw(
        p0: , 
        p1: , 
        p2: , 
        p3: , 
        p4: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x678BB03C1A3BD51Eu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x678BB03C1A3BD51Eu64;

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



pub fn network_session_is_voice_session_busy_safe(
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xEF0912DDF7C4CB4Bu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xEF0912DDF7C4CB4Bu64;
        
        let result = invoke_raw!(
            hash,
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn network_session_is_voice_session_busy_raw(
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xEF0912DDF7C4CB4Bu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xEF0912DDF7C4CB4Bu64;

        invoke_raw_typed!(
            hash,
        )
    }
}

/// ## Return value



pub fn network_clan_service_is_valid_safe(
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x579CCED0265D4896u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x579CCED0265D4896u64;
        
        let result = invoke_raw!(
            hash,
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn network_clan_service_is_valid_raw(
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x579CCED0265D4896u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x579CCED0265D4896u64;

        invoke_raw_typed!(
            hash,
        )
    }
}

/// ## Parameters
*



pub fn cloud_did_request_succeed_safe(
        
        
            handle: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x3A3D5568AF297CD5u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x3A3D5568AF297CD5u64;
        
        let result = invoke_raw!(
            hash,
                handle
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn cloud_did_request_succeed_raw(
        handle: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x3A3D5568AF297CD5u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x3A3D5568AF297CD5u64;

        invoke_raw_typed!(
            hash,
                handle
        )
    }
}

/// ## Return value



pub fn network_is_cable_connected_safe(
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xEFFB25453D8600F9u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xEFFB25453D8600F9u64;
        
        let result = invoke_raw!(
            hash,
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn network_is_cable_connected_raw(
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xEFFB25453D8600F9u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xEFFB25453D8600F9u64;

        invoke_raw_typed!(
            hash,
        )
    }
}

/// Enables ghosting between specific players. Name is between `_SET_RELATIONSHIP_GROUP_DONT_AFFECT_WANTED_LEVEL` and `SET_ROADS_BACK_TO_ORIGINAL`.



pub fn _set_relationship_to_player_safe(
        
        
            player: 
        , 
        
        
            p1: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xA7C511FA1C5BDA38u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xA7C511FA1C5BDA38u64;
        
        let result = invoke_raw!(
            hash,
                player, 
                p1
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn _set_relationship_to_player_raw(
        player: , 
        p1: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xA7C511FA1C5BDA38u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xA7C511FA1C5BDA38u64;

        invoke_raw_typed!(
            hash,
                player, 
                p1
        )
    }
}

/// ## Parameters
*



pub fn network_get_global_multiplayer_clock_safe(
        
        
            hours: 
        , 
        
        
            minutes: 
        , 
        
        
            seconds: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x6D03BFBD643B2A02u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x6D03BFBD643B2A02u64;
        
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
pub fn network_get_global_multiplayer_clock_raw(
        hours: , 
        minutes: , 
        seconds: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x6D03BFBD643B2A02u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x6D03BFBD643B2A02u64;

        invoke_raw_typed!(
            hash,
                hours, 
                minutes, 
                seconds
        )
    }
}

/// ## Return value



pub fn network_has_social_networking_sharing_priv_safe(
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x76BF03FADBF154F5u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x76BF03FADBF154F5u64;
        
        let result = invoke_raw!(
            hash,
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn network_has_social_networking_sharing_priv_raw(
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x76BF03FADBF154F5u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x76BF03FADBF154F5u64;

        invoke_raw_typed!(
            hash,
        )
    }
}

/// ## Parameters
*



pub fn network_get_activity_player_num_safe(
        
        
            p0: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x73E2B500410DA5A2u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x73E2B500410DA5A2u64;
        
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
pub fn network_get_activity_player_num_raw(
        p0: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x73E2B500410DA5A2u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x73E2B500410DA5A2u64;

        invoke_raw_typed!(
            hash,
                p0
        )
    }
}

/// ```
Subtracts the first argument from the second, then returns whether the result is negative.  
```



pub fn is_time_more_than_safe(
        
        
            timeA: 
        , 
        
        
            timeB: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xDE350F8651E4346Cu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xDE350F8651E4346Cu64;
        
        let result = invoke_raw!(
            hash,
                timeA, 
                timeB
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn is_time_more_than_raw(
        timeA: , 
        timeB: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xDE350F8651E4346Cu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xDE350F8651E4346Cu64;

        invoke_raw_typed!(
            hash,
                timeA, 
                timeB
        )
    }
}

/// ## Parameters
*



pub fn set_player_invisible_locally_safe(
        
        
            player: 
        , 
        
        
            toggle: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x12B37D54667DB0B8u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x12B37D54667DB0B8u64;
        
        let result = invoke_raw!(
            hash,
                player, 
                toggle
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn set_player_invisible_locally_raw(
        player: , 
        toggle: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x12B37D54667DB0B8u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x12B37D54667DB0B8u64;

        invoke_raw_typed!(
            hash,
                player, 
                toggle
        )
    }
}

/// ## Parameters
*



pub fn ugc_get_content_rating_count_safe(
        
        
            p0: 
        , 
        
        
            p1: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x759299C5BB31D2A9u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x759299C5BB31D2A9u64;
        
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
pub fn ugc_get_content_rating_count_raw(
        p0: , 
        p1: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x759299C5BB31D2A9u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x759299C5BB31D2A9u64;

        invoke_raw_typed!(
            hash,
                p0, 
                p1
        )
    }
}

/// ## Return value



pub fn network_is_session_active_safe(
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xD83C2B94E7508980u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xD83C2B94E7508980u64;
        
        let result = invoke_raw!(
            hash,
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn network_is_session_active_raw(
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xD83C2B94E7508980u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xD83C2B94E7508980u64;

        invoke_raw_typed!(
            hash,
        )
    }
}

/// ## Return value



pub fn network_have_ros_leaderboard_write_priv_safe(
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x422D396F80A96547u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x422D396F80A96547u64;
        
        let result = invoke_raw!(
            hash,
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn network_have_ros_leaderboard_write_priv_raw(
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x422D396F80A96547u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x422D396F80A96547u64;

        invoke_raw_typed!(
            hash,
        )
    }
}

/// ## Parameters
*



pub fn network_is_participant_active_safe(
        
        
            p0: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x6FF8FF40B6357D45u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x6FF8FF40B6357D45u64;
        
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
pub fn network_is_participant_active_raw(
        p0: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x6FF8FF40B6357D45u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x6FF8FF40B6357D45u64;

        invoke_raw_typed!(
            hash,
                p0
        )
    }
}

/// ## Return value



pub fn network_is_cloud_available_safe(
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x9A4CF4F48AD77302u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x9A4CF4F48AD77302u64;
        
        let result = invoke_raw!(
            hash,
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn network_is_cloud_available_raw(
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x9A4CF4F48AD77302u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x9A4CF4F48AD77302u64;

        invoke_raw_typed!(
            hash,
        )
    }
}

/// ```
returns true if someone is screaming or talking in a microphone  
```



pub fn network_is_player_talking_safe(
        
        
            player: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x031E11F3D447647Eu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x031E11F3D447647Eu64;
        
        let result = invoke_raw!(
            hash,
                player
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn network_is_player_talking_raw(
        player: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x031E11F3D447647Eu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x031E11F3D447647Eu64;

        invoke_raw_typed!(
            hash,
                player
        )
    }
}

/// ## Parameters
*



pub fn ugc_request_content_data_from_params_safe(
        
        
            contentTypeName: 
        , 
        
        
            contentId: 
        , 
        
        
            p2: 
        , 
        
        
            p3: 
        , 
        
        
            p4: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x7FD2990AF016795Eu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x7FD2990AF016795Eu64;
        
        let result = invoke_raw!(
            hash,
                contentTypeName, 
                contentId, 
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
pub fn ugc_request_content_data_from_params_raw(
        contentTypeName: , 
        contentId: , 
        p2: , 
        p3: , 
        p4: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x7FD2990AF016795Eu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x7FD2990AF016795Eu64;

        invoke_raw_typed!(
            hash,
                contentTypeName, 
                contentId, 
                p2, 
                p3, 
                p4
        )
    }
}

/// ## Parameters
*



pub fn network_has_player_started_transition_safe(
        
        
            player: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x9AC9CCBFA8C29795u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x9AC9CCBFA8C29795u64;
        
        let result = invoke_raw!(
            hash,
                player
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn network_has_player_started_transition_raw(
        player: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x9AC9CCBFA8C29795u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x9AC9CCBFA8C29795u64;

        invoke_raw_typed!(
            hash,
                player
        )
    }
}

/// ## Parameters
*



pub fn network_has_control_of_pickup_safe(
        
        
            pickup: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x5BC9495F0B3B6FA6u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x5BC9495F0B3B6FA6u64;
        
        let result = invoke_raw!(
            hash,
                pickup
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn network_has_control_of_pickup_raw(
        pickup: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x5BC9495F0B3B6FA6u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x5BC9495F0B3B6FA6u64;

        invoke_raw_typed!(
            hash,
                pickup
        )
    }
}

/// ## Parameters
*



pub fn network_set_voice_channel_safe(
        
        
            channel: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xEF6212C2EFEF1A23u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xEF6212C2EFEF1A23u64;
        
        let result = invoke_raw!(
            hash,
                channel
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn network_set_voice_channel_raw(
        channel: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xEF6212C2EFEF1A23u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xEF6212C2EFEF1A23u64;

        invoke_raw_typed!(
            hash,
                channel
        )
    }
}

/// ## Parameters
*



pub fn network_session_voice_set_timeout_safe(
        
        
            timeout: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x5B8ED3DB018927B1u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x5B8ED3DB018927B1u64;
        
        let result = invoke_raw!(
            hash,
                timeout
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn network_session_voice_set_timeout_raw(
        timeout: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x5B8ED3DB018927B1u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x5B8ED3DB018927B1u64;

        invoke_raw_typed!(
            hash,
                timeout
        )
    }
}

/// NETWORK_CLEAR_GROUP_ACTIVITY native function



pub fn network_clear_group_activity_safe(
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x1888694923EF4591u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x1888694923EF4591u64;
        
        let result = invoke_raw!(
            hash,
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn network_clear_group_activity_raw(
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x1888694923EF4591u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x1888694923EF4591u64;

        invoke_raw_typed!(
            hash,
        )
    }
}

/// ## Parameters
*



pub fn net_to_veh_safe(
        
        
            netHandle: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x367B936610BA360Cu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x367B936610BA360Cu64;
        
        let result = invoke_raw!(
            hash,
                netHandle
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn net_to_veh_raw(
        netHandle: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x367B936610BA360Cu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x367B936610BA360Cu64;

        invoke_raw_typed!(
            hash,
                netHandle
        )
    }
}

/// **This native does absolutely nothing, just a nullsub**



pub fn _clear_launch_params_safe(
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x966DD84FB6A46017u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x966DD84FB6A46017u64;
        
        let result = invoke_raw!(
            hash,
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn _clear_launch_params_raw(
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x966DD84FB6A46017u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x966DD84FB6A46017u64;

        invoke_raw_typed!(
            hash,
        )
    }
}

/// ## Parameters
*



pub fn _0xbf22e0f32968e967_safe(
        
        
            player: 
        , 
        
        
            p1: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xBF22E0F32968E967u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xBF22E0F32968E967u64;
        
        let result = invoke_raw!(
            hash,
                player, 
                p1
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn _0xbf22e0f32968e967_raw(
        player: , 
        p1: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xBF22E0F32968E967u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xBF22E0F32968E967u64;

        invoke_raw_typed!(
            hash,
                player, 
                p1
        )
    }
}

/// ```
SET_NETWORK_*
```



pub fn _0x13f1fcb111b820b0_safe(
        
        
            p0: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x13F1FCB111B820B0u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x13F1FCB111B820B0u64;
        
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
pub fn _0x13f1fcb111b820b0_raw(
        p0: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x13F1FCB111B820B0u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x13F1FCB111B820B0u64;

        invoke_raw_typed!(
            hash,
                p0
        )
    }
}

/// ```
NativeDB Introduced: v2189
```



pub fn _0x0f1a4b45b7693b95_safe(
        
        
            p0: 
        , 
        
        
            p1: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x0F1A4B45B7693B95u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x0F1A4B45B7693B95u64;
        
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
pub fn _0x0f1a4b45b7693b95_raw(
        p0: , 
        p1: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x0F1A4B45B7693B95u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x0F1A4B45B7693B95u64;

        invoke_raw_typed!(
            hash,
                p0, 
                p1
        )
    }
}

/// NETWORK_SESSION_FORCE_CANCEL_INVITE native function



pub fn network_session_force_cancel_invite_safe(
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xA29177F7703B5644u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xA29177F7703B5644u64;
        
        let result = invoke_raw!(
            hash,
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn network_session_force_cancel_invite_raw(
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xA29177F7703B5644u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xA29177F7703B5644u64;

        invoke_raw_typed!(
            hash,
        )
    }
}

/// ```
NativeDB Introduced: v2245
```



pub fn _network_is_script_active_by_hash_safe(
        
        
            scriptHash: 
        , 
        
        
            p1: 
        , 
        
        
            p2: 
        , 
        
        
            p3: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xDA7DE67F5FE5EE13u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xDA7DE67F5FE5EE13u64;
        
        let result = invoke_raw!(
            hash,
                scriptHash, 
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
pub fn _network_is_script_active_by_hash_raw(
        scriptHash: , 
        p1: , 
        p2: , 
        p3: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xDA7DE67F5FE5EE13u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xDA7DE67F5FE5EE13u64;

        invoke_raw_typed!(
            hash,
                scriptHash, 
                p1, 
                p2, 
                p3
        )
    }
}

/// ## Return value
Returns the network time this tick, if you want a tick-agnostic version use [`GetNetworkTimeAccurate`](#_0x89023FBBF9200E9F).



pub fn get_network_time_safe(
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x7A5487FE9FAA6B48u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x7A5487FE9FAA6B48u64;
        
        let result = invoke_raw!(
            hash,
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn get_network_time_raw(
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x7A5487FE9FAA6B48u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x7A5487FE9FAA6B48u64;

        invoke_raw_typed!(
            hash,
        )
    }
}

/// ## Return value



pub fn _network_get_num_body_trackers_safe(
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xD38C4A6D047C019Du32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xD38C4A6D047C019Du64;
        
        let result = invoke_raw!(
            hash,
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn _network_get_num_body_trackers_raw(
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xD38C4A6D047C019Du32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xD38C4A6D047C019Du64;

        invoke_raw_typed!(
            hash,
        )
    }
}

/// ## Parameters
*



pub fn network_session_get_inviter_safe(
        
        
            networkHandle: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xE57397B4A3429DD0u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xE57397B4A3429DD0u64;
        
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
pub fn network_session_get_inviter_raw(
        networkHandle: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xE57397B4A3429DD0u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xE57397B4A3429DD0u64;

        invoke_raw_typed!(
            hash,
                networkHandle
        )
    }
}

/// ## Return value



pub fn network_is_transition_open_to_matchmaking_safe(
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x37A4494483B9F5C9u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x37A4494483B9F5C9u64;
        
        let result = invoke_raw!(
            hash,
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn network_is_transition_open_to_matchmaking_raw(
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x37A4494483B9F5C9u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x37A4494483B9F5C9u64;

        invoke_raw_typed!(
            hash,
        )
    }
}

/// ## Parameters
*



pub fn _0x3fa36981311fa4ff_safe(
        
        
            netId: 
        , 
        
        
            state: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x3FA36981311FA4FFu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x3FA36981311FA4FFu64;
        
        let result = invoke_raw!(
            hash,
                netId, 
                state
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn _0x3fa36981311fa4ff_raw(
        netId: , 
        state: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x3FA36981311FA4FFu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x3FA36981311FA4FFu64;

        invoke_raw_typed!(
            hash,
                netId, 
                state
        )
    }
}

/// ```
Only works as host.
```



pub fn network_session_kick_player_safe(
        
        
            player: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xFA8904DC5F304220u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xFA8904DC5F304220u64;
        
        let result = invoke_raw!(
            hash,
                player
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn network_session_kick_player_raw(
        player: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xFA8904DC5F304220u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xFA8904DC5F304220u64;

        invoke_raw_typed!(
            hash,
                player
        )
    }
}

/// ## Parameters
*



pub fn _network_set_current_data_manager_handle_safe(
        
        
            p0: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x796A87B3B68D1F3Du32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x796A87B3B68D1F3Du64;
        
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
pub fn _network_set_current_data_manager_handle_raw(
        p0: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x796A87B3B68D1F3Du32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x796A87B3B68D1F3Du64;

        invoke_raw_typed!(
            hash,
                p0
        )
    }
}

/// Sets the alpha value used by [`_SET_LOCAL_PLAYER_AS_GHOST`](#_0x5FFE9B4144F9712F), [`SET_NETWORK_VEHICLE_AS_GHOST`](#_0x6274C4712850841E), and [`_NETWORK_SET_ENTITY_GHOSTED_WITH_OWNER`](#_0x4BA166079D658ED4).

'Solidness' cannot be achieved using 255 - this will have the opposite effect of it defaulting to 128 it seems (or just having no effect at all).



pub fn _set_ghosted_entity_alpha_safe(
        
        
            alpha: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x658500AE6D723A7Eu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x658500AE6D723A7Eu64;
        
        let result = invoke_raw!(
            hash,
                alpha
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn _set_ghosted_entity_alpha_raw(
        alpha: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x658500AE6D723A7Eu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x658500AE6D723A7Eu64;

        invoke_raw_typed!(
            hash,
                alpha
        )
    }
}

/// ## Parameters
*



pub fn network_request_control_of_door_safe(
        
        
            doorID: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x870DDFD5A4A796E4u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x870DDFD5A4A796E4u64;
        
        let result = invoke_raw!(
            hash,
                doorID
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn network_request_control_of_door_raw(
        doorID: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x870DDFD5A4A796E4u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x870DDFD5A4A796E4u64;

        invoke_raw_typed!(
            hash,
                doorID
        )
    }
}

/// ```
NativeDB Introduced: v1604
```



pub fn _0x95baf97c82464629_safe(
        
        
            p0: 
        , 
        
        
            p1: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x95BAF97C82464629u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x95BAF97C82464629u64;
        
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
pub fn _0x95baf97c82464629_raw(
        p0: , 
        p1: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x95BAF97C82464629u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x95BAF97C82464629u64;

        invoke_raw_typed!(
            hash,
                p0, 
                p1
        )
    }
}

/// ## Parameters
*



pub fn network_get_entity_is_networked_safe(
        
        
            entity: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xC7827959479DCC78u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xC7827959479DCC78u64;
        
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
pub fn network_get_entity_is_networked_raw(
        entity: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xC7827959479DCC78u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xC7827959479DCC78u64;

        invoke_raw_typed!(
            hash,
                entity
        )
    }
}

/// ## Return value



pub fn network_is_cloud_background_script_request_pending_safe(
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x8132C0EB8B2B3293u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x8132C0EB8B2B3293u64;
        
        let result = invoke_raw!(
            hash,
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn network_is_cloud_background_script_request_pending_raw(
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x8132C0EB8B2B3293u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x8132C0EB8B2B3293u64;

        invoke_raw_typed!(
            hash,
        )
    }
}

/// ```
NativeDB Added Parameter 1: Player player
NativeDB Added Parameter 2: int a
NativeDB Added Parameter 3: int b
```



pub fn _remote_cheat_detected_safe(
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x472841A026D26D8Bu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x472841A026D26D8Bu64;
        
        let result = invoke_raw!(
            hash,
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn _remote_cheat_detected_raw(
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x472841A026D26D8Bu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x472841A026D26D8Bu64;

        invoke_raw_typed!(
            hash,
        )
    }
}

/// ```
Returns true if the two times are equal; otherwise returns false.  
```



pub fn is_time_equal_to_safe(
        
        
            timeA: 
        , 
        
        
            timeB: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xF5BC95857BD6D512u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xF5BC95857BD6D512u64;
        
        let result = invoke_raw!(
            hash,
                timeA, 
                timeB
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn is_time_equal_to_raw(
        timeA: , 
        timeB: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xF5BC95857BD6D512u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xF5BC95857BD6D512u64;

        invoke_raw_typed!(
            hash,
                timeA, 
                timeB
        )
    }
}

/// ## Parameters
*



pub fn network_access_tunable_float_hash_safe(
        
        
            tunableContext: 
        , 
        
        
            tunableName: 
        , 
        
        
            value: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x972BC203BBC4C4D5u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x972BC203BBC4C4D5u64;
        
        let result = invoke_raw!(
            hash,
                tunableContext, 
                tunableName, 
                value
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn network_access_tunable_float_hash_raw(
        tunableContext: , 
        tunableName: , 
        value: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x972BC203BBC4C4D5u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x972BC203BBC4C4D5u64;

        invoke_raw_typed!(
            hash,
                tunableContext, 
                tunableName, 
                value
        )
    }
}

/// ## Return value



pub fn get_max_num_network_pickups_safe(
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xA72835064DD63E4Cu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xA72835064DD63E4Cu64;
        
        let result = invoke_raw!(
            hash,
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn get_max_num_network_pickups_raw(
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xA72835064DD63E4Cu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xA72835064DD63E4Cu64;

        invoke_raw_typed!(
            hash,
        )
    }
}

/// ## Parameters
*



pub fn network_entity_area_does_exist_safe(
        
        
            areaHandle: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xE64A3CA08DFA37A9u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xE64A3CA08DFA37A9u64;
        
        let result = invoke_raw!(
            hash,
                areaHandle
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn network_entity_area_does_exist_raw(
        areaHandle: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xE64A3CA08DFA37A9u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xE64A3CA08DFA37A9u64;

        invoke_raw_typed!(
            hash,
                areaHandle
        )
    }
}

/// ## Parameters
*



pub fn network_get_primary_clan_data_new_safe(
        
        
            p0: 
        , 
        
        
            p1: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xC080FF658B2E41DAu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xC080FF658B2E41DAu64;
        
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
pub fn network_get_primary_clan_data_new_raw(
        p0: , 
        p1: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xC080FF658B2E41DAu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xC080FF658B2E41DAu64;

        invoke_raw_typed!(
            hash,
                p0, 
                p1
        )
    }
}

/// ## Parameters
*



pub fn network_set_local_player_invincible_time_safe(
        
        
            time: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x2D95C7E2D7E07307u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x2D95C7E2D7E07307u64;
        
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
pub fn network_set_local_player_invincible_time_raw(
        time: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x2D95C7E2D7E07307u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x2D95C7E2D7E07307u64;

        invoke_raw_typed!(
            hash,
                time
        )
    }
}

/// **This native does absolutely nothing, just a nullsub**



pub fn _0x5c497525f803486b_safe(
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x5C497525F803486Bu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x5C497525F803486Bu64;
        
        let result = invoke_raw!(
            hash,
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn _0x5c497525f803486b_raw(
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x5C497525F803486Bu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x5C497525F803486Bu64;

        invoke_raw_typed!(
            hash,
        )
    }
}

/// ## Parameters
*



pub fn get_num_created_mission_objects_safe(
        
        
            p0: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x12B6281B6C6706C0u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x12B6281B6C6706C0u64;
        
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
pub fn get_num_created_mission_objects_raw(
        p0: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x12B6281B6C6706C0u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x12B6281B6C6706C0u64;

        invoke_raw_typed!(
            hash,
                p0
        )
    }
}

/// ## Parameters
*



pub fn network_has_control_of_door_safe(
        
        
            doorHash: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xCB3C68ADB06195DFu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xCB3C68ADB06195DFu64;
        
        let result = invoke_raw!(
            hash,
                doorHash
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn network_has_control_of_door_raw(
        doorHash: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xCB3C68ADB06195DFu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xCB3C68ADB06195DFu64;

        invoke_raw_typed!(
            hash,
                doorHash
        )
    }
}

/// ## Parameters
*



pub fn network_set_entity_can_blend_safe(
        
        
            entity: 
        , 
        
        
            toggle: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xD830567D88A1E873u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xD830567D88A1E873u64;
        
        let result = invoke_raw!(
            hash,
                entity, 
                toggle
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn network_set_entity_can_blend_raw(
        entity: , 
        toggle: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xD830567D88A1E873u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xD830567D88A1E873u64;

        invoke_raw_typed!(
            hash,
                entity, 
                toggle
        )
    }
}

/// ## Parameters
*



pub fn _0x4df7cfff471a7fb1_safe(
        
        
            p0: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x4DF7CFFF471A7FB1u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x4DF7CFFF471A7FB1u64;
        
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
pub fn _0x4df7cfff471a7fb1_raw(
        p0: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x4DF7CFFF471A7FB1u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x4DF7CFFF471A7FB1u64;

        invoke_raw_typed!(
            hash,
                p0
        )
    }
}

/// ## Parameters
*



pub fn network_set_in_free_cam_mode_safe(
        
        
            toggle: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xFC18DB55AE19E046u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xFC18DB55AE19E046u64;
        
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
pub fn network_set_in_free_cam_mode_raw(
        toggle: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xFC18DB55AE19E046u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xFC18DB55AE19E046u64;

        invoke_raw_typed!(
            hash,
                toggle
        )
    }
}

/// ## Parameters
*



pub fn use_player_colour_instead_of_team_colour_safe(
        
        
            toggle: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x77758139EC9B66C7u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x77758139EC9B66C7u64;
        
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
pub fn use_player_colour_instead_of_team_colour_raw(
        toggle: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x77758139EC9B66C7u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x77758139EC9B66C7u64;

        invoke_raw_typed!(
            hash,
                toggle
        )
    }
}

/// ```
11 - Need to download tunables.  
12 - Need to download background script.  
Returns 1 if the multiplayer is loaded, otherwhise 0.  
```



pub fn network_can_access_multiplayer_safe(
        
        
            loadingState: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xAF50DA1A3F8B1BA4u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xAF50DA1A3F8B1BA4u64;
        
        let result = invoke_raw!(
            hash,
                loadingState
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn network_can_access_multiplayer_raw(
        loadingState: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xAF50DA1A3F8B1BA4u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xAF50DA1A3F8B1BA4u64;

        invoke_raw_typed!(
            hash,
                loadingState
        )
    }
}

/// ```
Only documented...  
```



pub fn _network_clan_animation_safe(
        
        
            animDict: 
        , 
        
        
            animName: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x729E3401F0430686u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x729E3401F0430686u64;
        
        let result = invoke_raw!(
            hash,
                animDict, 
                animName
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn _network_clan_animation_raw(
        animDict: , 
        animName: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x729E3401F0430686u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x729E3401F0430686u64;

        invoke_raw_typed!(
            hash,
                animDict, 
                animName
        )
    }
}

/// ## Parameters
*



pub fn network_has_transition_invite_been_acked_safe(
        
        
            networkHandle: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x3F9990BF5F22759Cu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x3F9990BF5F22759Cu64;
        
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
pub fn network_has_transition_invite_been_acked_raw(
        networkHandle: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x3F9990BF5F22759Cu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x3F9990BF5F22759Cu64;

        invoke_raw_typed!(
            hash,
                networkHandle
        )
    }
}

/// ## Return value
Hard-coded to always return 1.



pub fn _0xebcab9e5048434f4_safe(
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xEBCAB9E5048434F4u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xEBCAB9E5048434F4u64;
        
        let result = invoke_raw!(
            hash,
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn _0xebcab9e5048434f4_raw(
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xEBCAB9E5048434F4u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xEBCAB9E5048434F4u64;

        invoke_raw_typed!(
            hash,
        )
    }
}

/// ## Parameters
*



pub fn can_register_mission_entities_safe(
        
        
            ped_amt: 
        , 
        
        
            vehicle_amt: 
        , 
        
        
            object_amt: 
        , 
        
        
            pickup_amt: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x69778E7564BADE6Du32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x69778E7564BADE6Du64;
        
        let result = invoke_raw!(
            hash,
                ped_amt, 
                vehicle_amt, 
                object_amt, 
                pickup_amt
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn can_register_mission_entities_raw(
        ped_amt: , 
        vehicle_amt: , 
        object_amt: , 
        pickup_amt: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x69778E7564BADE6Du32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x69778E7564BADE6Du64;

        invoke_raw_typed!(
            hash,
                ped_amt, 
                vehicle_amt, 
                object_amt, 
                pickup_amt
        )
    }
}

/// _0x68103E2247887242 native function



pub fn _0x68103e2247887242_safe(
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x68103E2247887242u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x68103E2247887242u64;
        
        let result = invoke_raw!(
            hash,
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn _0x68103e2247887242_raw(
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x68103E2247887242u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x68103E2247887242u64;

        invoke_raw_typed!(
            hash,
        )
    }
}

/// NETWORK_SESSION_JOIN_INVITE native function



pub fn network_session_join_invite_safe(
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xC6F8AB8A4189CF3Au32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xC6F8AB8A4189CF3Au64;
        
        let result = invoke_raw!(
            hash,
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn network_session_join_invite_raw(
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xC6F8AB8A4189CF3Au32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xC6F8AB8A4189CF3Au64;

        invoke_raw_typed!(
            hash,
        )
    }
}

/// ## Parameters
*



pub fn ugc_set_query_data_from_offline_safe(
        
        
            p0: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xF98DDE0A8ED09323u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xF98DDE0A8ED09323u64;
        
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
pub fn ugc_set_query_data_from_offline_raw(
        p0: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xF98DDE0A8ED09323u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xF98DDE0A8ED09323u64;

        invoke_raw_typed!(
            hash,
                p0
        )
    }
}

/// ## Parameters
*



pub fn network_get_num_script_participants_safe(
        
        
            p0: 
        , 
        
        
            p1: 
        , 
        
        
            p2: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x3658E8CD94FC121Au32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x3658E8CD94FC121Au64;
        
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
pub fn network_get_num_script_participants_raw(
        p0: , 
        p1: , 
        p2: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x3658E8CD94FC121Au32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x3658E8CD94FC121Au64;

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



pub fn network_remove_transition_invite_safe(
        
        
            p0: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x7524B431B2E6F7EEu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x7524B431B2E6F7EEu64;
        
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
pub fn network_remove_transition_invite_raw(
        p0: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x7524B431B2E6F7EEu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x7524B431B2E6F7EEu64;

        invoke_raw_typed!(
            hash,
                p0
        )
    }
}

/// ```
Subtracts the second argument from the first.  
```



pub fn get_time_difference_safe(
        
        
            timeA: 
        , 
        
        
            timeB: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xA2C6FC031D46FFF0u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xA2C6FC031D46FFF0u64;
        
        let result = invoke_raw!(
            hash,
                timeA, 
                timeB
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn get_time_difference_raw(
        timeA: , 
        timeB: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xA2C6FC031D46FFF0u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xA2C6FC031D46FFF0u64;

        invoke_raw_typed!(
            hash,
                timeA, 
                timeB
        )
    }
}

/// ## Parameters
*



pub fn _network_register_tunable_int_hash_safe(
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x3A8B55FDA4C8DDEFu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x3A8B55FDA4C8DDEFu64;
        
        let result = invoke_raw!(
            hash,
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn _network_register_tunable_int_hash_raw(
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x3A8B55FDA4C8DDEFu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x3A8B55FDA4C8DDEFu64;

        invoke_raw_typed!(
            hash,
        )
    }
}

/// ```
NativeDB Introduced: v1180
```



pub fn _0xfac18e7356bd3210_safe(
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xFAC18E7356BD3210u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xFAC18E7356BD3210u64;
        
        let result = invoke_raw!(
            hash,
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn _0xfac18e7356bd3210_raw(
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xFAC18E7356BD3210u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xFAC18E7356BD3210u64;

        invoke_raw_typed!(
            hash,
        )
    }
}

/// ## Parameters
*



pub fn network_session_set_matchmaking_property_id_safe(
        
        
            p0: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x3F52E880AAF6C8CAu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x3F52E880AAF6C8CAu64;
        
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
pub fn network_session_set_matchmaking_property_id_raw(
        p0: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x3F52E880AAF6C8CAu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x3F52E880AAF6C8CAu64;

        invoke_raw_typed!(
            hash,
                p0
        )
    }
}

/// NETWORK_CLEAR_TRANSITION_CREATOR_HANDLE native function



pub fn network_clear_transition_creator_handle_safe(
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xFB3272229A82C759u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xFB3272229A82C759u64;
        
        let result = invoke_raw!(
            hash,
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn network_clear_transition_creator_handle_raw(
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xFB3272229A82C759u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xFB3272229A82C759u64;

        invoke_raw_typed!(
            hash,
        )
    }
}

/// ```
String "NETWORK_SEND_PRESENCE_TRANSITION_INVITE" is contained in the function in ida so this one is correct.  
```



pub fn _network_send_presence_transition_invite_safe(
        
        
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
        let hash = 0xC116FF9B4D488291u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xC116FF9B4D488291u64;
        
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
pub fn _network_send_presence_transition_invite_raw(
        p0: , 
        p1: , 
        p2: , 
        p3: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xC116FF9B4D488291u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xC116FF9B4D488291u64;

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



pub fn get_commerce_item_id_safe(
        
        
            index: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x662635855957C411u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x662635855957C411u64;
        
        let result = invoke_raw!(
            hash,
                index
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn get_commerce_item_id_raw(
        index: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x662635855957C411u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x662635855957C411u64;

        invoke_raw_typed!(
            hash,
                index
        )
    }
}

/// ## Parameters
*



pub fn network_get_presence_invite_id_safe(
        
        
            p0: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xDFF09646E12EC386u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xDFF09646E12EC386u64;
        
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
pub fn network_get_presence_invite_id_raw(
        p0: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xDFF09646E12EC386u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xDFF09646E12EC386u64;

        invoke_raw_typed!(
            hash,
                p0
        )
    }
}

/// NETWORK_CLEAR_VOICE_PROXIMITY_OVERRIDE native function



pub fn network_clear_voice_proximity_override_safe(
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xF03755696450470Cu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xF03755696450470Cu64;
        
        let result = invoke_raw!(
            hash,
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn network_clear_voice_proximity_override_raw(
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xF03755696450470Cu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xF03755696450470Cu64;

        invoke_raw_typed!(
            hash,
        )
    }
}

/// ## Parameters
*



pub fn _0xaeab987727c5a8a4_safe(
        
        
            p0: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xAEAB987727C5A8A4u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xAEAB987727C5A8A4u64;
        
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
pub fn _0xaeab987727c5a8a4_raw(
        p0: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xAEAB987727C5A8A4u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xAEAB987727C5A8A4u64;

        invoke_raw_typed!(
            hash,
                p0
        )
    }
}

/// _0x2302C0264EA58D31 native function



pub fn _0x2302c0264ea58d31_safe(
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x2302C0264EA58D31u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x2302C0264EA58D31u64;
        
        let result = invoke_raw!(
            hash,
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn _0x2302c0264ea58d31_raw(
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x2302C0264EA58D31u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x2302C0264EA58D31u64;

        invoke_raw_typed!(
            hash,
        )
    }
}

/// ## Parameters
*



pub fn _reserve_network_local_vehicles_safe(
        
        
            amount: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x42613035157E4208u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x42613035157E4208u64;
        
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
pub fn _reserve_network_local_vehicles_raw(
        amount: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x42613035157E4208u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x42613035157E4208u64;

        invoke_raw_typed!(
            hash,
                amount
        )
    }
}

/// ## Parameters
*



pub fn _0x04918a41bc9b8157_safe(
        
        
            p0: 
        , 
        
        
            p1: 
        , 
        
        
            p2: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x04918A41BC9B8157u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x04918A41BC9B8157u64;
        
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
pub fn _0x04918a41bc9b8157_raw(
        p0: , 
        p1: , 
        p2: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x04918A41BC9B8157u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x04918A41BC9B8157u64;

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



pub fn _0xf49abc20d8552257_safe(
        
        
            p0: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xF49ABC20D8552257u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xF49ABC20D8552257u64;
        
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
pub fn _0xf49abc20d8552257_raw(
        p0: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xF49ABC20D8552257u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xF49ABC20D8552257u64;

        invoke_raw_typed!(
            hash,
                p0
        )
    }
}

/// ## Parameters
*



pub fn network_is_player_muted_by_me_safe(
        
        
            player: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x8C71288AE68EDE39u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x8C71288AE68EDE39u64;
        
        let result = invoke_raw!(
            hash,
                player
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn network_is_player_muted_by_me_raw(
        player: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x8C71288AE68EDE39u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x8C71288AE68EDE39u64;

        invoke_raw_typed!(
            hash,
                player
        )
    }
}

/// Adds a ped to a networked synchronized scene but extends [`NETWORK_ADD_PED_TO_SYNCHRONISED_SCENE`](#_0x742A637471BCECD9) to support IK flags.
There is barely any difference between this and [`NETWORK_ADD_PED_TO_SYNCHRONISED_SCENE`](#_0x742A637471BCECD9).

```
NativeDB Introduced: v1290
```



pub fn network_add_ped_to_synchronised_scene_with_ik_safe(
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xA5EAFE473E45C442u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xA5EAFE473E45C442u64;
        
        let result = invoke_raw!(
            hash,
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn network_add_ped_to_synchronised_scene_with_ik_raw(
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xA5EAFE473E45C442u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xA5EAFE473E45C442u64;

        invoke_raw_typed!(
            hash,
        )
    }
}

/// ## Return value



pub fn network_is_transition_solo_safe(
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x5DC577201723960Au32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x5DC577201723960Au64;
        
        let result = invoke_raw!(
            hash,
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn network_is_transition_solo_raw(
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x5DC577201723960Au32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x5DC577201723960Au64;

        invoke_raw_typed!(
            hash,
        )
    }
}

/// ## Parameters
*



pub fn network_member_id_from_gamer_handle_safe(
        
        
            networkHandle: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xC82630132081BB6Fu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xC82630132081BB6Fu64;
        
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
pub fn network_member_id_from_gamer_handle_raw(
        networkHandle: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xC82630132081BB6Fu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xC82630132081BB6Fu64;

        invoke_raw_typed!(
            hash,
                networkHandle
        )
    }
}

/// ```
This is used alongside the native,
'NETWORK_OVERRIDE_RECEIVE_RESTRICTIONS'. Read its description for more info.
```



pub fn network_override_send_restrictions_safe(
        
        
            player: 
        , 
        
        
            toggle: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x97DD4C5944CC2E6Au32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x97DD4C5944CC2E6Au64;
        
        let result = invoke_raw!(
            hash,
                player, 
                toggle
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn network_override_send_restrictions_raw(
        player: , 
        toggle: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x97DD4C5944CC2E6Au32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x97DD4C5944CC2E6Au64;

        invoke_raw_typed!(
            hash,
                player, 
                toggle
        )
    }
}

/// ## Return value



pub fn network_has_pending_invite_safe(
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xAC8C7B9B88C4A668u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xAC8C7B9B88C4A668u64;
        
        let result = invoke_raw!(
            hash,
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn network_has_pending_invite_raw(
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xAC8C7B9B88C4A668u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xAC8C7B9B88C4A668u64;

        invoke_raw_typed!(
            hash,
        )
    }
}

/// ## Parameters
*



pub fn network_have_communication_privileges_safe(
        
        
            p0: 
        , 
        
        
            player: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xAEEF48CDF5B6CE7Cu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xAEEF48CDF5B6CE7Cu64;
        
        let result = invoke_raw!(
            hash,
                p0, 
                player
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn network_have_communication_privileges_raw(
        p0: , 
        player: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xAEEF48CDF5B6CE7Cu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xAEEF48CDF5B6CE7Cu64;

        invoke_raw_typed!(
            hash,
                p0, 
                player
        )
    }
}

/// ## Parameters
*



pub fn _0x144da052257ae7d8_safe(
        
        
            p0: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x144DA052257AE7D8u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x144DA052257AE7D8u64;
        
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
pub fn _0x144da052257ae7d8_raw(
        p0: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x144DA052257AE7D8u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x144DA052257AE7D8u64;

        invoke_raw_typed!(
            hash,
                p0
        )
    }
}

/// ## Parameters
*



pub fn network_have_user_content_privileges_safe(
        
        
            p0: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x72D918C99BCACC54u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x72D918C99BCACC54u64;
        
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
pub fn network_have_user_content_privileges_raw(
        p0: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x72D918C99BCACC54u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x72D918C99BCACC54u64;

        invoke_raw_typed!(
            hash,
                p0
        )
    }
}

/// ## Return value



pub fn network_leave_transition_safe(
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xD23A1A815D21DB19u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xD23A1A815D21DB19u64;
        
        let result = invoke_raw!(
            hash,
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn network_leave_transition_raw(
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xD23A1A815D21DB19u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xD23A1A815D21DB19u64;

        invoke_raw_typed!(
            hash,
        )
    }
}

/// ## Return value



pub fn network_join_previously_failed_transition_safe(
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xFFE1E5B792D92B34u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xFFE1E5B792D92B34u64;
        
        let result = invoke_raw!(
            hash,
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn network_join_previously_failed_transition_raw(
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xFFE1E5B792D92B34u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xFFE1E5B792D92B34u64;

        invoke_raw_typed!(
            hash,
        )
    }
}

/// ```
NativeDB Introduced: v1290
```



pub fn _0xe42d626eec94e5d9_safe(
        
        
            p0: 
        , 
        
        
            p1: 
        , 
        
        
            p2: 
        , 
        
        
            p3: 
        , 
        
        
            numReservedPeds: 
        , 
        
        
            numReservedVehicles: 
        , 
        
        
            numReservedObjects: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xE42D626EEC94E5D9u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xE42D626EEC94E5D9u64;
        
        let result = invoke_raw!(
            hash,
                p0, 
                p1, 
                p2, 
                p3, 
                numReservedPeds, 
                numReservedVehicles, 
                numReservedObjects
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn _0xe42d626eec94e5d9_raw(
        p0: , 
        p1: , 
        p2: , 
        p3: , 
        numReservedPeds: , 
        numReservedVehicles: , 
        numReservedObjects: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xE42D626EEC94E5D9u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xE42D626EEC94E5D9u64;

        invoke_raw_typed!(
            hash,
                p0, 
                p1, 
                p2, 
                p3, 
                numReservedPeds, 
                numReservedVehicles, 
                numReservedObjects
        )
    }
}

/// ```
calls from vehicle to net.  
```



pub fn veh_to_net_safe(
        
        
            vehicle: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xB4C94523F023419Cu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xB4C94523F023419Cu64;
        
        let result = invoke_raw!(
            hash,
                vehicle
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn veh_to_net_raw(
        vehicle: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xB4C94523F023419Cu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xB4C94523F023419Cu64;

        invoke_raw_typed!(
            hash,
                vehicle
        )
    }
}

/// ```
Same as _IS_TEXT_CHAT_ACTIVE, except it does not check if the text chat HUD component is initialized, and therefore may crash.  
```



pub fn _network_is_text_chat_active_safe(
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x5FCF4D7069B09026u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x5FCF4D7069B09026u64;
        
        let result = invoke_raw!(
            hash,
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn _network_is_text_chat_active_raw(
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x5FCF4D7069B09026u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x5FCF4D7069B09026u64;

        invoke_raw_typed!(
            hash,
        )
    }
}

/// ## Return value



pub fn network_has_received_host_broadcast_data_safe(
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x5D10B3795F3FC886u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x5D10B3795F3FC886u64;
        
        let result = invoke_raw!(
            hash,
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn network_has_received_host_broadcast_data_raw(
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x5D10B3795F3FC886u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x5D10B3795F3FC886u64;

        invoke_raw_typed!(
            hash,
        )
    }
}

/// NETWORK_CLEAR_FOUND_GAMERS native function



pub fn network_clear_found_gamers_safe(
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x6D14CCEE1B40381Au32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x6D14CCEE1B40381Au64;
        
        let result = invoke_raw!(
            hash,
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn network_clear_found_gamers_raw(
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x6D14CCEE1B40381Au32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x6D14CCEE1B40381Au64;

        invoke_raw_typed!(
            hash,
        )
    }
}

/// ## Parameters
*



pub fn _can_register_mission_pickups_safe(
        
        
            amount: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x0A49D1CB6E34AF72u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x0A49D1CB6E34AF72u64;
        
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
pub fn _can_register_mission_pickups_raw(
        amount: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x0A49D1CB6E34AF72u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x0A49D1CB6E34AF72u64;

        invoke_raw_typed!(
            hash,
                amount
        )
    }
}

/// ## Parameters
*



pub fn network_set_no_spectator_chat_safe(
        
        
            toggle: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xF46A1E03E8755980u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xF46A1E03E8755980u64;
        
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
pub fn network_set_no_spectator_chat_raw(
        toggle: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xF46A1E03E8755980u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xF46A1E03E8755980u64;

        invoke_raw_typed!(
            hash,
                toggle
        )
    }
}

/// ## Return value



pub fn network_is_host_of_this_script_safe(
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x83CD99A1E6061AB5u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x83CD99A1E6061AB5u64;
        
        let result = invoke_raw!(
            hash,
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn network_is_host_of_this_script_raw(
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x83CD99A1E6061AB5u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x83CD99A1E6061AB5u64;

        invoke_raw_typed!(
            hash,
        )
    }
}

/// ## Return value



pub fn ugc_get_content_num_safe(
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xE0A6138401BCB837u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xE0A6138401BCB837u64;
        
        let result = invoke_raw!(
            hash,
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn ugc_get_content_num_raw(
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xE0A6138401BCB837u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xE0A6138401BCB837u64;

        invoke_raw_typed!(
            hash,
        )
    }
}

/// ```
NETWORK_GET_NUM_*
```



pub fn _0x617f49c2668e6155_safe(
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x617F49C2668E6155u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x617F49C2668E6155u64;
        
        let result = invoke_raw!(
            hash,
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn _0x617f49c2668e6155_raw(
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x617F49C2668E6155u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x617F49C2668E6155u64;

        invoke_raw_typed!(
            hash,
        )
    }
}

/// ## Return value



pub fn network_is_in_tutorial_session_safe(
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xADA24309FE08DACFu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xADA24309FE08DACFu64;
        
        let result = invoke_raw!(
            hash,
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn network_is_in_tutorial_session_raw(
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xADA24309FE08DACFu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xADA24309FE08DACFu64;

        invoke_raw_typed!(
            hash,
        )
    }
}

/// ```
bufferSize is 35 in the scripts.
```



pub fn network_clan_get_ui_formatted_tag_safe(
        
        
            clanDesc: 
        , 
        
        
            bufferSize: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xF45352426FF3A4F0u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xF45352426FF3A4F0u64;
        
        let result = invoke_raw!(
            hash,
                clanDesc, 
                bufferSize
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn network_clan_get_ui_formatted_tag_raw(
        clanDesc: , 
        bufferSize: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xF45352426FF3A4F0u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xF45352426FF3A4F0u64;

        invoke_raw_typed!(
            hash,
                clanDesc, 
                bufferSize
        )
    }
}

/// NETWORK_SET_MISSION_FINISHED native function



pub fn network_set_mission_finished_safe(
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x3B3D11CD9FFCDFC9u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x3B3D11CD9FFCDFC9u64;
        
        let result = invoke_raw!(
            hash,
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn network_set_mission_finished_raw(
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x3B3D11CD9FFCDFC9u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x3B3D11CD9FFCDFC9u64;

        invoke_raw_typed!(
            hash,
        )
    }
}

/// ## Return value



pub fn network_is_finding_gamers_safe(
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xDDDF64C91BFCF0AAu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xDDDF64C91BFCF0AAu64;
        
        let result = invoke_raw!(
            hash,
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn network_is_finding_gamers_raw(
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xDDDF64C91BFCF0AAu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xDDDF64C91BFCF0AAu64;

        invoke_raw_typed!(
            hash,
        )
    }
}

/// ## Parameters
*



pub fn network_is_player_concealed_safe(
        
        
            player: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x919B3C98ED8292F9u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x919B3C98ED8292F9u64;
        
        let result = invoke_raw!(
            hash,
                player
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn network_is_player_concealed_raw(
        player: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x919B3C98ED8292F9u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x919B3C98ED8292F9u64;

        invoke_raw_typed!(
            hash,
                player
        )
    }
}

/// ## Parameters
*



pub fn network_set_activity_spectator_safe(
        
        
            toggle: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x75138790B4359A74u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x75138790B4359A74u64;
        
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
pub fn network_set_activity_spectator_raw(
        toggle: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x75138790B4359A74u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x75138790B4359A74u64;

        invoke_raw_typed!(
            hash,
                toggle
        )
    }
}

/// _0xF814FEC6A19FD6E0 native function



pub fn _0xf814fec6a19fd6e0_safe(
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xF814FEC6A19FD6E0u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xF814FEC6A19FD6E0u64;
        
        let result = invoke_raw!(
            hash,
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn _0xf814fec6a19fd6e0_raw(
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xF814FEC6A19FD6E0u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xF814FEC6A19FD6E0u64;

        invoke_raw_typed!(
            hash,
        )
    }
}

/// ## Return value



pub fn network_have_ros_social_club_priv_safe(
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x606E4D3E3CCCF3EBu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x606E4D3E3CCCF3EBu64;
        
        let result = invoke_raw!(
            hash,
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn network_have_ros_social_club_priv_raw(
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x606E4D3E3CCCF3EBu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x606E4D3E3CCCF3EBu64;

        invoke_raw_typed!(
            hash,
        )
    }
}

/// ```
Returns count.
```



pub fn network_get_transition_members_safe(
        
        
            data: 
        , 
        
        
            dataCount: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x73B000F7FBC55829u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x73B000F7FBC55829u64;
        
        let result = invoke_raw!(
            hash,
                data, 
                dataCount
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn network_get_transition_members_raw(
        data: , 
        dataCount: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x73B000F7FBC55829u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x73B000F7FBC55829u64;

        invoke_raw_typed!(
            hash,
                data, 
                dataCount
        )
    }
}

/// ## Return value



pub fn network_can_set_waypoint_safe(
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xC927EC229934AF60u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xC927EC229934AF60u64;
        
        let result = invoke_raw!(
            hash,
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn network_can_set_waypoint_raw(
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xC927EC229934AF60u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xC927EC229934AF60u64;

        invoke_raw_typed!(
            hash,
        )
    }
}

/// ## Parameters
*



pub fn _0x5ecd378ee64450ab_safe(
        
        
            p0: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x5ECD378EE64450ABu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x5ECD378EE64450ABu64;
        
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
pub fn _0x5ecd378ee64450ab_raw(
        p0: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x5ECD378EE64450ABu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x5ECD378EE64450ABu64;

        invoke_raw_typed!(
            hash,
                p0
        )
    }
}

/// ## Parameters
*



pub fn _0x1d610eb0fea716d9_safe(
        
        
            p0: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x1D610EB0FEA716D9u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x1D610EB0FEA716D9u64;
        
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
pub fn _0x1d610eb0fea716d9_raw(
        p0: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x1D610EB0FEA716D9u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x1D610EB0FEA716D9u64;

        invoke_raw_typed!(
            hash,
                p0
        )
    }
}

/// ## Parameters
*



pub fn _0x5539c3ebf104a53a_safe(
        
        
            p0: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x5539C3EBF104A53Au32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x5539C3EBF104A53Au64;
        
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
pub fn _0x5539c3ebf104a53a_raw(
        p0: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x5539C3EBF104A53Au32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x5539C3EBF104A53Au64;

        invoke_raw_typed!(
            hash,
                p0
        )
    }
}

/// ## Parameters
*



pub fn _0xfd75dabc0957bf33_safe(
        
        
            p0: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xFD75DABC0957BF33u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xFD75DABC0957BF33u64;
        
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
pub fn _0xfd75dabc0957bf33_raw(
        p0: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xFD75DABC0957BF33u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xFD75DABC0957BF33u64;

        invoke_raw_typed!(
            hash,
                p0
        )
    }
}

/// Checks if a crew/membership for a player, from the cache (i.e. downloaded via NETWORK_CLAN_DOWNLOAD_MEMBERSHIP) is valid.



pub fn network_clan_get_membership_valid_safe(
        
        
            networkHandle: 
        , 
        
        
            membershipIndex: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x48A59CF88D43DF0Eu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x48A59CF88D43DF0Eu64;
        
        let result = invoke_raw!(
            hash,
                networkHandle, 
                membershipIndex
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn network_clan_get_membership_valid_raw(
        networkHandle: , 
        membershipIndex: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x48A59CF88D43DF0Eu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x48A59CF88D43DF0Eu64;

        invoke_raw_typed!(
            hash,
                networkHandle, 
                membershipIndex
        )
    }
}

/// ## Return value



pub fn is_store_available_to_user_safe(
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x883D79C4071E18B3u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x883D79C4071E18B3u64;
        
        let result = invoke_raw!(
            hash,
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn is_store_available_to_user_raw(
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x883D79C4071E18B3u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x883D79C4071E18B3u64;

        invoke_raw_typed!(
            hash,
        )
    }
}

/// ## Parameters
*



pub fn _0xba7f0b77d80a4eb7_safe(
        
        
            p0: 
        , 
        
        
            p1: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xBA7F0B77D80A4EB7u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xBA7F0B77D80A4EB7u64;
        
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
pub fn _0xba7f0b77d80a4eb7_raw(
        p0: , 
        p1: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xBA7F0B77D80A4EB7u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xBA7F0B77D80A4EB7u64;

        invoke_raw_typed!(
            hash,
                p0, 
                p1
        )
    }
}

/// I've had this return the player's ped handle sometimes, but also other random entities.

Whatever p0 is, it's at least not synced to other players.
At least not all the time, some p0 values actually output the same entity, (different handle of course, but same entity).
But another p0 value may return an entity for player x, but not for player y (it'll just return -1 even if the entity exists on both clients).



pub fn _0x37d5f739fd494675_safe(
        
        
            p0: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x37D5F739FD494675u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x37D5F739FD494675u64;
        
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
pub fn _0x37d5f739fd494675_raw(
        p0: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x37D5F739FD494675u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x37D5F739FD494675u64;

        invoke_raw_typed!(
            hash,
                p0
        )
    }
}

/// ## Parameters
*



pub fn _facebook_set_heist_complete_safe(
        
        
            heistName: 
        , 
        
        
            cashEarned: 
        , 
        
        
            xpEarned: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x098AB65B9ED9A9ECu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x098AB65B9ED9A9ECu64;
        
        let result = invoke_raw!(
            hash,
                heistName, 
                cashEarned, 
                xpEarned
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn _facebook_set_heist_complete_raw(
        heistName: , 
        cashEarned: , 
        xpEarned: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x098AB65B9ED9A9ECu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x098AB65B9ED9A9ECu64;

        invoke_raw_typed!(
            hash,
                heistName, 
                cashEarned, 
                xpEarned
        )
    }
}

/// ```
R* uses this to hear all player when spectating.   
It allows you to hear other online players when their chat is on none, crew and or friends  
```



pub fn network_override_receive_restrictions_safe(
        
        
            player: 
        , 
        
        
            toggle: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xDDF73E2B1FEC5AB4u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xDDF73E2B1FEC5AB4u64;
        
        let result = invoke_raw!(
            hash,
                player, 
                toggle
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn network_override_receive_restrictions_raw(
        player: , 
        toggle: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xDDF73E2B1FEC5AB4u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xDDF73E2B1FEC5AB4u64;

        invoke_raw_typed!(
            hash,
                player, 
                toggle
        )
    }
}

/// ```
milestoneId:  
0 = "percentcomplete"  
1 = "storycomplete"  
2 = "vehicles"  
3 = "properties"  
4 = "psych"  
5 = "mapreveal"  
6 = "prologue"  
```



pub fn _facebook_set_milestone_complete_safe(
        
        
            milestoneId: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x0AE1F1653B554AB9u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x0AE1F1653B554AB9u64;
        
        let result = invoke_raw!(
            hash,
                milestoneId
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn _facebook_set_milestone_complete_raw(
        milestoneId: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x0AE1F1653B554AB9u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x0AE1F1653B554AB9u64;

        invoke_raw_typed!(
            hash,
                milestoneId
        )
    }
}

/// ## Parameters
*



pub fn _0xb746d20b17f2a229_safe(
        
        
            p0: 
        , 
        
        
            p1: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xB746D20B17F2A229u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xB746D20B17F2A229u64;
        
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
pub fn _0xb746d20b17f2a229_raw(
        p0: , 
        p1: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xB746D20B17F2A229u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xB746D20B17F2A229u64;

        invoke_raw_typed!(
            hash,
                p0, 
                p1
        )
    }
}

/// ```
scriptName examples:  
"freemode", "AM_CR_SecurityVan", ...  
Most of the time, these values are used:  
p1 = -1  
p2 = 0  
```



pub fn network_get_host_of_script_safe(
        
        
            scriptName: 
        , 
        
        
            p1: 
        , 
        
        
            p2: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x1D6A14F1F9A736FCu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x1D6A14F1F9A736FCu64;
        
        let result = invoke_raw!(
            hash,
                scriptName, 
                p1, 
                p2
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn network_get_host_of_script_raw(
        scriptName: , 
        p1: , 
        p2: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x1D6A14F1F9A736FCu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x1D6A14F1F9A736FCu64;

        invoke_raw_typed!(
            hash,
                scriptName, 
                p1, 
                p2
        )
    }
}

/// ## Parameters
*



pub fn _network_is_friend_handle_online_safe(
        
        
            networkHandle: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x87EB7A3FFCB314DBu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x87EB7A3FFCB314DBu64;
        
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
pub fn _network_is_friend_handle_online_raw(
        networkHandle: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x87EB7A3FFCB314DBu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x87EB7A3FFCB314DBu64;

        invoke_raw_typed!(
            hash,
                networkHandle
        )
    }
}

/// ## Parameters
*



pub fn _0x8c8d2739ba44af0f_safe(
        
        
            p0: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x8C8D2739BA44AF0Fu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x8C8D2739BA44AF0Fu64;
        
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
pub fn _0x8c8d2739ba44af0f_raw(
        p0: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x8C8D2739BA44AF0Fu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x8C8D2739BA44AF0Fu64;

        invoke_raw_typed!(
            hash,
                p0
        )
    }
}

/// ## Parameters
*



pub fn network_is_friend_in_multiplayer_safe(
        
        
            friendName: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x57005C18827F3A28u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x57005C18827F3A28u64;
        
        let result = invoke_raw!(
            hash,
                friendName
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn network_is_friend_in_multiplayer_raw(
        friendName: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x57005C18827F3A28u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x57005C18827F3A28u64;

        invoke_raw_typed!(
            hash,
                friendName
        )
    }
}

/// ## Parameters
*



pub fn network_set_transition_visibility_lock_safe(
        
        
            p0: 
        , 
        
        
            p1: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x0C978FDA19692C2Cu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x0C978FDA19692C2Cu64;
        
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
pub fn network_set_transition_visibility_lock_raw(
        p0: , 
        p1: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x0C978FDA19692C2Cu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x0C978FDA19692C2Cu64;

        invoke_raw_typed!(
            hash,
                p0, 
                p1
        )
    }
}

/// ```
Online version is defined here: update\update.rpf\common\data\version.txt
Example:
[ONLINE_VERSION_NUMBER]
1.33
_GET_ONLINE_VERSION() will return "1.33"
```



pub fn _get_online_version_safe(
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xFCA9373EF340AC0Au32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xFCA9373EF340AC0Au64;
        
        let result = invoke_raw!(
            hash,
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn _get_online_version_raw(
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xFCA9373EF340AC0Au32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xFCA9373EF340AC0Au64;

        invoke_raw_typed!(
            hash,
        )
    }
}

/// ## Parameters
*



pub fn network_session_host_closed_safe(
        
        
            p0: 
        , 
        
        
            maxPlayers: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xED34C0C02C098BB7u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xED34C0C02C098BB7u64;
        
        let result = invoke_raw!(
            hash,
                p0, 
                maxPlayers
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn network_session_host_closed_raw(
        p0: , 
        maxPlayers: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xED34C0C02C098BB7u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xED34C0C02C098BB7u64;

        invoke_raw_typed!(
            hash,
                p0, 
                maxPlayers
        )
    }
}

/// ```
NETWORK_ARE_*  
```



pub fn _network_is_player_equal_to_index_safe(
        
        
            player: 
        , 
        
        
            index: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x9DE986FC9A87C474u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x9DE986FC9A87C474u64;
        
        let result = invoke_raw!(
            hash,
                player, 
                index
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn _network_is_player_equal_to_index_raw(
        player: , 
        index: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x9DE986FC9A87C474u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x9DE986FC9A87C474u64;

        invoke_raw_typed!(
            hash,
                player, 
                index
        )
    }
}

/// RELEASE_ALL_COMMERCE_ITEM_IMAGES native function



pub fn release_all_commerce_item_images_safe(
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x72D0706CD6CCDB58u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x72D0706CD6CCDB58u64;
        
        let result = invoke_raw!(
            hash,
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn release_all_commerce_item_images_raw(
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x72D0706CD6CCDB58u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x72D0706CD6CCDB58u64;

        invoke_raw_typed!(
            hash,
        )
    }
}

/// ## Parameters
*



pub fn _0x584770794d758c18_safe(
        
        
            p0: 
        , 
        
        
            p1: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x584770794D758C18u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x584770794D758C18u64;
        
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
pub fn _0x584770794d758c18_raw(
        p0: , 
        p1: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x584770794D758C18u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x584770794D758C18u64;

        invoke_raw_typed!(
            hash,
                p0, 
                p1
        )
    }
}

/// ## Return value
Hard-coded to always return 0.



pub fn _0x6fb7bb3607d27fa2_safe(
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x6FB7BB3607D27FA2u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x6FB7BB3607D27FA2u64;
        
        let result = invoke_raw!(
            hash,
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn _0x6fb7bb3607d27fa2_raw(
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x6FB7BB3607D27FA2u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x6FB7BB3607D27FA2u64;

        invoke_raw_typed!(
            hash,
        )
    }
}

/// ## Return value



pub fn ugc_get_create_content_id_safe(
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xC55A0B40FFB1ED23u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xC55A0B40FFB1ED23u64;
        
        let result = invoke_raw!(
            hash,
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn ugc_get_create_content_id_raw(
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xC55A0B40FFB1ED23u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xC55A0B40FFB1ED23u64;

        invoke_raw_typed!(
            hash,
        )
    }
}

/// ## Parameters
*



pub fn network_access_tunable_float_safe(
        
        
            tunableContext: 
        , 
        
        
            tunableName: 
        , 
        
        
            value: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xE5608CA7BC163A5Fu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xE5608CA7BC163A5Fu64;
        
        let result = invoke_raw!(
            hash,
                tunableContext, 
                tunableName, 
                value
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn network_access_tunable_float_raw(
        tunableContext: , 
        tunableName: , 
        value: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xE5608CA7BC163A5Fu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xE5608CA7BC163A5Fu64;

        invoke_raw_typed!(
            hash,
                tunableContext, 
                tunableName, 
                value
        )
    }
}

/// ## Return value



pub fn network_have_ros_create_ticket_priv_safe(
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xA0AD7E2AF5349F61u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xA0AD7E2AF5349F61u64;
        
        let result = invoke_raw!(
            hash,
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn network_have_ros_create_ticket_priv_raw(
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xA0AD7E2AF5349F61u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xA0AD7E2AF5349F61u64;

        invoke_raw_typed!(
            hash,
        )
    }
}

/// ## Return value



pub fn _0xa306f470d1660581_safe(
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xA306F470D1660581u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xA306F470D1660581u64;
        
        let result = invoke_raw!(
            hash,
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn _0xa306f470d1660581_raw(
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xA306F470D1660581u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xA306F470D1660581u64;

        invoke_raw_typed!(
            hash,
        )
    }
}

/// ## Parameters
*



pub fn network_get_presence_invite_is_tournament_safe(
        
        
            p0: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x8806CEBFABD3CE05u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x8806CEBFABD3CE05u64;
        
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
pub fn network_get_presence_invite_is_tournament_raw(
        p0: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x8806CEBFABD3CE05u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x8806CEBFABD3CE05u64;

        invoke_raw_typed!(
            hash,
                p0
        )
    }
}

/// ```
Returns true if dinput8.dll is present in the game directory.
You will get following error message if that is true: "You are attempting to access GTA Online servers with an altered version of the game."
```



pub fn _network_has_game_been_altered_safe(
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x659CF2EF7F550C4Fu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x659CF2EF7F550C4Fu64;
        
        let result = invoke_raw!(
            hash,
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn _network_has_game_been_altered_raw(
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x659CF2EF7F550C4Fu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x659CF2EF7F550C4Fu64;

        invoke_raw_typed!(
            hash,
        )
    }
}

/// ## Parameters
*



pub fn network_is_player_a_participant_safe(
        
        
            player: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x3CA58F6CB7CBD784u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x3CA58F6CB7CBD784u64;
        
        let result = invoke_raw!(
            hash,
                player
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn network_is_player_a_participant_raw(
        player: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x3CA58F6CB7CBD784u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x3CA58F6CB7CBD784u64;

        invoke_raw_typed!(
            hash,
                player
        )
    }
}

/// ## Parameters
*



pub fn _0x5a34cd9c3c5bec44_safe(
        
        
            p0: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x5A34CD9C3C5BEC44u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x5A34CD9C3C5BEC44u64;
        
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
pub fn _0x5a34cd9c3c5bec44_raw(
        p0: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x5A34CD9C3C5BEC44u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x5A34CD9C3C5BEC44u64;

        invoke_raw_typed!(
            hash,
                p0
        )
    }
}

/// ## Parameters
*



pub fn network_entity_area_is_occupied_safe(
        
        
            areaHandle: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x4A2D4E8BF4265B0Fu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x4A2D4E8BF4265B0Fu64;
        
        let result = invoke_raw!(
            hash,
                areaHandle
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn network_entity_area_is_occupied_raw(
        areaHandle: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x4A2D4E8BF4265B0Fu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x4A2D4E8BF4265B0Fu64;

        invoke_raw_typed!(
            hash,
                areaHandle
        )
    }
}

/// ## Parameters
*



pub fn network_set_invite_on_call_for_invite_menu_safe(
        
        
            networkHandle: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x66F010A4B031A331u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x66F010A4B031A331u64;
        
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
pub fn network_set_invite_on_call_for_invite_menu_raw(
        networkHandle: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x66F010A4B031A331u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x66F010A4B031A331u64;

        invoke_raw_typed!(
            hash,
                networkHandle
        )
    }
}

/// ```
Always returns -1. Seems to be XB1 specific.
```



pub fn _network_start_user_content_permissions_check_safe(
        
        
            netHandle: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xDEB2B99A1AF1A2A6u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xDEB2B99A1AF1A2A6u64;
        
        let result = invoke_raw!(
            hash,
                netHandle
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn _network_start_user_content_permissions_check_raw(
        netHandle: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xDEB2B99A1AF1A2A6u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xDEB2B99A1AF1A2A6u64;

        invoke_raw_typed!(
            hash,
                netHandle
        )
    }
}

/// ## Return value



pub fn network_is_transition_closed_crew_safe(
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x0DBD5D7E3C5BEC3Bu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x0DBD5D7E3C5BEC3Bu64;
        
        let result = invoke_raw!(
            hash,
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn network_is_transition_closed_crew_raw(
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x0DBD5D7E3C5BEC3Bu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x0DBD5D7E3C5BEC3Bu64;

        invoke_raw_typed!(
            hash,
        )
    }
}

/// ## Return value



pub fn ugc_did_get_succeed_safe(
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x941E5306BCD7C2C7u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x941E5306BCD7C2C7u64;
        
        let result = invoke_raw!(
            hash,
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn ugc_did_get_succeed_raw(
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x941E5306BCD7C2C7u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x941E5306BCD7C2C7u64;

        invoke_raw_typed!(
            hash,
        )
    }
}

/// ```
Return the mission id of a job.
```



pub fn ugc_get_content_id_safe(
        
        
            p0: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x55AA95F481D694D2u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x55AA95F481D694D2u64;
        
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
pub fn ugc_get_content_id_raw(
        p0: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x55AA95F481D694D2u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x55AA95F481D694D2u64;

        invoke_raw_typed!(
            hash,
                p0
        )
    }
}

/// ## Parameters
*



pub fn network_override_coords_and_heading_safe(
        
        
            entity: 
        , 
        
        
            x: 
        , 
        
        
            y: 
        , 
        
        
            z: 
        , 
        
        
            heading: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xA7E30DE9272B6D49u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xA7E30DE9272B6D49u64;
        
        let result = invoke_raw!(
            hash,
                entity, 
                x, 
                y, 
                z, 
                heading
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn network_override_coords_and_heading_raw(
        entity: , 
        x: , 
        y: , 
        z: , 
        heading: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xA7E30DE9272B6D49u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xA7E30DE9272B6D49u64;

        invoke_raw_typed!(
            hash,
                entity, 
                x, 
                y, 
                z, 
                heading
        )
    }
}

/// ## Parameters
*



pub fn network_clan_is_emblem_ready_safe(
        
        
            p0: 
        , 
        
        
            p1: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xA134777FF7F33331u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xA134777FF7F33331u64;
        
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
pub fn network_clan_is_emblem_ready_raw(
        p0: , 
        p1: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xA134777FF7F33331u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xA134777FF7F33331u64;

        invoke_raw_typed!(
            hash,
                p0, 
                p1
        )
    }
}

/// ```
Returns the amount of players connected in the current session. Only works when connected to a session/server.  
```



pub fn network_get_num_connected_players_safe(
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xA4A79DD2D9600654u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xA4A79DD2D9600654u64;
        
        let result = invoke_raw!(
            hash,
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn network_get_num_connected_players_raw(
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xA4A79DD2D9600654u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xA4A79DD2D9600654u64;

        invoke_raw_typed!(
            hash,
        )
    }
}

/// _0x265635150FB0D82E native function



pub fn _0x265635150fb0d82e_safe(
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x265635150FB0D82Eu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x265635150FB0D82Eu64;
        
        let result = invoke_raw!(
            hash,
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn _0x265635150fb0d82e_raw(
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x265635150FB0D82Eu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x265635150FB0D82Eu64;

        invoke_raw_typed!(
            hash,
        )
    }
}

/// ## Parameters
*



pub fn _0x32ebd154cb6b8b99_safe(
        
        
            p0: 
        , 
        
        
            p1: 
        , 
        
        
            p2: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x32EBD154CB6B8B99u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x32EBD154CB6B8B99u64;
        
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
pub fn _0x32ebd154cb6b8b99_raw(
        p0: , 
        p1: , 
        p2: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x32EBD154CB6B8B99u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x32EBD154CB6B8B99u64;

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



pub fn _0x162c23ca83ed0a62_safe(
        
        
            p0: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x162C23CA83ED0A62u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x162C23CA83ED0A62u64;
        
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
pub fn _0x162c23ca83ed0a62_raw(
        p0: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x162C23CA83ED0A62u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x162C23CA83ED0A62u64;

        invoke_raw_typed!(
            hash,
                p0
        )
    }
}

/// ## Return value



pub fn network_is_activity_session_safe(
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x05095437424397FAu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x05095437424397FAu64;
        
        let result = invoke_raw!(
            hash,
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn network_is_activity_session_raw(
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x05095437424397FAu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x05095437424397FAu64;

        invoke_raw_typed!(
            hash,
        )
    }
}

/// ```
bufferSize is 35 in the scripts.  
bufferSize is the elementCount of p0(desc), sizeof(p0) == 280 == p1*8 == 35 * 8, p2(netHandle) is obtained from NETWORK::NETWORK_HANDLE_FROM_PLAYER.  And no, I can't explain why 35 * sizeof(int) == 280 and not 140, but I'll get back to you on that.  
the answer is: because p0 an int64_t* / int64_t[35].  and FYI p2 is an int64_t[13]  
pastebin.com/cSZniHak  
```



pub fn network_clan_player_get_desc_safe(
        
        
            clanDesc: 
        , 
        
        
            bufferSize: 
        , 
        
        
            networkHandle: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xEEE6EACBE8874FBAu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xEEE6EACBE8874FBAu64;
        
        let result = invoke_raw!(
            hash,
                clanDesc, 
                bufferSize, 
                networkHandle
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn network_clan_player_get_desc_raw(
        clanDesc: , 
        bufferSize: , 
        networkHandle: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xEEE6EACBE8874FBAu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xEEE6EACBE8874FBAu64;

        invoke_raw_typed!(
            hash,
                clanDesc, 
                bufferSize, 
                networkHandle
        )
    }
}

/// UGC_CLEAR_QUERY_RESULTS native function



pub fn ugc_clear_query_results_safe(
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xBA96394A0EECFA65u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xBA96394A0EECFA65u64;
        
        let result = invoke_raw!(
            hash,
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn ugc_clear_query_results_raw(
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xBA96394A0EECFA65u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xBA96394A0EECFA65u64;

        invoke_raw_typed!(
            hash,
        )
    }
}

/// ## Parameters
*



pub fn set_local_player_visible_in_cutscene_safe(
        
        
            p0: 
        , 
        
        
            p1: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xD1065D68947E7B6Eu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xD1065D68947E7B6Eu64;
        
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
pub fn set_local_player_visible_in_cutscene_raw(
        p0: , 
        p1: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xD1065D68947E7B6Eu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xD1065D68947E7B6Eu64;

        invoke_raw_typed!(
            hash,
                p0, 
                p1
        )
    }
}

/// ## Parameters
*



pub fn _0x1f8e00fb18239600_safe(
        
        
            p0: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x1F8E00FB18239600u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x1F8E00FB18239600u64;
        
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
pub fn _0x1f8e00fb18239600_raw(
        p0: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x1F8E00FB18239600u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x1F8E00FB18239600u64;

        invoke_raw_typed!(
            hash,
                p0
        )
    }
}

/// ## Parameters
*



pub fn ugc_get_content_updated_date_safe(
        
        
            p0: 
        , 
        
        
            p1: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xCFD115B373C0DF63u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xCFD115B373C0DF63u64;
        
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
pub fn ugc_get_content_updated_date_raw(
        p0: , 
        p1: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xCFD115B373C0DF63u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xCFD115B373C0DF63u64;

        invoke_raw_typed!(
            hash,
                p0, 
                p1
        )
    }
}

/// ## Parameters
*



pub fn network_request_control_of_entity_safe(
        
        
            entity: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xB69317BF5E782347u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xB69317BF5E782347u64;
        
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
pub fn network_request_control_of_entity_raw(
        entity: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xB69317BF5E782347u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xB69317BF5E782347u64;

        invoke_raw_typed!(
            hash,
                entity
        )
    }
}

/// ## Return value



pub fn _network_get_ros_privilege_24_safe(
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x593570C289A77688u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x593570C289A77688u64;
        
        let result = invoke_raw!(
            hash,
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn _network_get_ros_privilege_24_raw(
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x593570C289A77688u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x593570C289A77688u64;

        invoke_raw_typed!(
            hash,
        )
    }
}

/// ## Return value



pub fn network_get_primary_clan_data_clear_safe(
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x9AA46BADAD0E27EDu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x9AA46BADAD0E27EDu64;
        
        let result = invoke_raw!(
            hash,
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn network_get_primary_clan_data_clear_raw(
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x9AA46BADAD0E27EDu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x9AA46BADAD0E27EDu64;

        invoke_raw_typed!(
            hash,
        )
    }
}

/// ## Parameters
*



pub fn network_do_transition_to_game_safe(
        
        
            p0: 
        , 
        
        
            maxPlayers: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x3E9BB38102A589B0u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x3E9BB38102A589B0u64;
        
        let result = invoke_raw!(
            hash,
                p0, 
                maxPlayers
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn network_do_transition_to_game_raw(
        p0: , 
        maxPlayers: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x3E9BB38102A589B0u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x3E9BB38102A589B0u64;

        invoke_raw_typed!(
            hash,
                p0, 
                maxPlayers
        )
    }
}

/// ## Return value



pub fn network_has_follow_invite_safe(
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x76D9B976C4C09FDEu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x76D9B976C4C09FDEu64;
        
        let result = invoke_raw!(
            hash,
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn network_has_follow_invite_raw(
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x76D9B976C4C09FDEu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x76D9B976C4C09FDEu64;

        invoke_raw_typed!(
            hash,
        )
    }
}

/// ```
Enables a periodic ShapeTest within the NetBlender and invokes rage::netBlenderLinInterp::GoStraightToTarget (or some functional wrapper).
```



pub fn _set_network_enable_vehicle_position_correction_safe(
        
        
            vehicle: 
        , 
        
        
            toggle: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x838DA0936A24ED4Du32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x838DA0936A24ED4Du64;
        
        let result = invoke_raw!(
            hash,
                vehicle, 
                toggle
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn _set_network_enable_vehicle_position_correction_raw(
        vehicle: , 
        toggle: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x838DA0936A24ED4Du32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x838DA0936A24ED4Du64;

        invoke_raw_typed!(
            hash,
                vehicle, 
                toggle
        )
    }
}

/// ## Return value



pub fn network_get_this_script_is_network_script_safe(
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x2910669969E9535Eu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x2910669969E9535Eu64;
        
        let result = invoke_raw!(
            hash,
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn network_get_this_script_is_network_script_raw(
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x2910669969E9535Eu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x2910669969E9535Eu64;

        invoke_raw_typed!(
            hash,
        )
    }
}

/// ```
Return the content modifier id (the tunables context if you want) of a specific content.  
It takes the content hash (which is the mission id hash), and return the content modifier id, used as the tunables context.  
The mission id can be found on the Social club, for example, 'socialclub.rockstargames.com/games/gtav/jobs/job/A8M6Bz8MLEC5xngvDCzGwA'  
'A8M6Bz8MLEC5xngvDCzGwA' is the mission id, so the game hash this and use it as the parameter for this native.  
```



pub fn network_get_content_modifier_list_id_safe(
        
        
            contentHash: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x187382F8A3E0A6C3u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x187382F8A3E0A6C3u64;
        
        let result = invoke_raw!(
            hash,
                contentHash
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn network_get_content_modifier_list_id_raw(
        contentHash: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x187382F8A3E0A6C3u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x187382F8A3E0A6C3u64;

        invoke_raw_typed!(
            hash,
                contentHash
        )
    }
}

/// ## Parameters
*



pub fn is_network_id_owned_by_participant_safe(
        
        
            netId: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xA1607996431332DFu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xA1607996431332DFu64;
        
        let result = invoke_raw!(
            hash,
                netId
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn is_network_id_owned_by_participant_raw(
        netId: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xA1607996431332DFu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xA1607996431332DFu64;

        invoke_raw_typed!(
            hash,
                netId
        )
    }
}

/// ## Parameters
*



pub fn network_session_block_join_requests_safe(
        
        
            toggle: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xA73667484D7037C3u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xA73667484D7037C3u64;
        
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
pub fn network_session_block_join_requests_raw(
        toggle: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xA73667484D7037C3u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xA73667484D7037C3u64;

        invoke_raw_typed!(
            hash,
                toggle
        )
    }
}

/// ## Return value



pub fn network_is_transition_to_game_safe(
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x9D7696D8F4FA6CB7u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x9D7696D8F4FA6CB7u64;
        
        let result = invoke_raw!(
            hash,
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn network_is_transition_to_game_raw(
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x9D7696D8F4FA6CB7u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x9D7696D8F4FA6CB7u64;

        invoke_raw_typed!(
            hash,
        )
    }
}

/// ## Return value



pub fn network_get_total_num_players_safe(
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xCF61D4B4702EE9EBu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xCF61D4B4702EE9EBu64;
        
        let result = invoke_raw!(
            hash,
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn network_get_total_num_players_raw(
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xCF61D4B4702EE9EBu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xCF61D4B4702EE9EBu64;

        invoke_raw_typed!(
            hash,
        )
    }
}

/// ## Parameters
*



pub fn network_apply_cached_player_head_blend_data_safe(
        
        
            ped: 
        , 
        
        
            player: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x99B72C7ABDE5C910u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x99B72C7ABDE5C910u64;
        
        let result = invoke_raw!(
            hash,
                ped, 
                player
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn network_apply_cached_player_head_blend_data_raw(
        ped: , 
        player: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x99B72C7ABDE5C910u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x99B72C7ABDE5C910u64;

        invoke_raw_typed!(
            hash,
                ped, 
                player
        )
    }
}

/// ## Return value



pub fn _0x7db53b37a2f211a0_safe(
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x7DB53B37A2F211A0u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x7DB53B37A2F211A0u64;
        
        let result = invoke_raw!(
            hash,
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn _0x7db53b37a2f211a0_raw(
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x7DB53B37A2F211A0u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x7DB53B37A2F211A0u64;

        invoke_raw_typed!(
            hash,
        )
    }
}

/// ```
Note according to IDA TU27 X360(Console),  
This native & 'NETWORK_IS_PARTY_MEMBER' both jump to the same location.  
Side note: This location just stops where it's at once jumped to.  
Screenshot for side note,   
h t t p ://i.imgur.com/m2ci1mF.png  
h t t p://i.imgur.com/Z0Wx2B6.png  
```



pub fn network_is_party_member_safe(
        
        
            networkHandle: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x676ED266AADD31E0u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x676ED266AADD31E0u64;
        
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
pub fn network_is_party_member_raw(
        networkHandle: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x676ED266AADD31E0u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x676ED266AADD31E0u64;

        invoke_raw_typed!(
            hash,
                networkHandle
        )
    }
}

/// ## Return value



pub fn cloud_is_checking_availability_safe(
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xC7ABAC5DE675EE3Bu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xC7ABAC5DE675EE3Bu64;
        
        let result = invoke_raw!(
            hash,
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn cloud_is_checking_availability_raw(
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xC7ABAC5DE675EE3Bu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xC7ABAC5DE675EE3Bu64;

        invoke_raw_typed!(
            hash,
        )
    }
}

/// ## Return value



pub fn network_get_primary_clan_data_pending_safe(
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xB5074DB804E28CE7u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xB5074DB804E28CE7u64;
        
        let result = invoke_raw!(
            hash,
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn network_get_primary_clan_data_pending_raw(
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xB5074DB804E28CE7u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xB5074DB804E28CE7u64;

        invoke_raw_typed!(
            hash,
        )
    }
}

/// ## Return value



pub fn network_is_tunable_cloud_request_pending_safe(
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x0467C11ED88B7D28u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x0467C11ED88B7D28u64;
        
        let result = invoke_raw!(
            hash,
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn network_is_tunable_cloud_request_pending_raw(
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x0467C11ED88B7D28u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x0467C11ED88B7D28u64;

        invoke_raw_typed!(
            hash,
        )
    }
}

/// To remove, see: [`NETWORK_REMOVE_ENTITY_AREA`](#_0x93CF869BAA0C4874).

See [`IS_POINT_IN_ANGLED_AREA`](#_0x2A70BAE8883E4C81) for the definition of an angled area.



pub fn network_add_entity_angled_area_safe(
        
        
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
        , 
        
        
            width: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x376C6375BA60293Au32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x376C6375BA60293Au64;
        
        let result = invoke_raw!(
            hash,
                x1, 
                y1, 
                z1, 
                x2, 
                y2, 
                z2, 
                width
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn network_add_entity_angled_area_raw(
        x1: , 
        y1: , 
        z1: , 
        x2: , 
        y2: , 
        z2: , 
        width: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x376C6375BA60293Au32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x376C6375BA60293Au64;

        invoke_raw_typed!(
            hash,
                x1, 
                y1, 
                z1, 
                x2, 
                y2, 
                z2, 
                width
        )
    }
}

/// ```
Same as NETWORK_CAN_COMMUNICATE_WITH_GAMER

NETWORK_CAN_*
```



pub fn _network_can_communicate_with_gamer_2_safe(
        
        
            networkHandle: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x8F5D1AD832AEB06Cu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x8F5D1AD832AEB06Cu64;
        
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
pub fn _network_can_communicate_with_gamer_2_raw(
        networkHandle: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x8F5D1AD832AEB06Cu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x8F5D1AD832AEB06Cu64;

        invoke_raw_typed!(
            hash,
                networkHandle
        )
    }
}

/// ```
Internal logging string: SCRIPT_RESERVING_LOCAL_OBJECTS
```

```
NativeDB Introduced: v1290
```



pub fn _reserve_network_local_objects_safe(
        
        
            amount: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x797F9C5E661D920Eu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x797F9C5E661D920Eu64;
        
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
pub fn _reserve_network_local_objects_raw(
        amount: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x797F9C5E661D920Eu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x797F9C5E661D920Eu64;

        invoke_raw_typed!(
            hash,
                amount
        )
    }
}

/// ```
Returns a NetworkHandle* from the specified member ID and stores it in a given buffer.  
* Currently unknown struct  
```



pub fn network_handle_from_member_id_safe(
        
        
            memberId: 
        , 
        
        
            networkHandle: 
        , 
        
        
            bufferSize: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xA0FD21BED61E5C4Cu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xA0FD21BED61E5C4Cu64;
        
        let result = invoke_raw!(
            hash,
                memberId, 
                networkHandle, 
                bufferSize
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn network_handle_from_member_id_raw(
        memberId: , 
        networkHandle: , 
        bufferSize: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xA0FD21BED61E5C4Cu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xA0FD21BED61E5C4Cu64;

        invoke_raw_typed!(
            hash,
                memberId, 
                networkHandle, 
                bufferSize
        )
    }
}

/// ```
NativeDB Introduced: v323
```



pub fn _network_get_average_latency_for_player_safe(
        
        
            player: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xD414BE129BB81B32u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xD414BE129BB81B32u64;
        
        let result = invoke_raw!(
            hash,
                player
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn _network_get_average_latency_for_player_raw(
        player: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xD414BE129BB81B32u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xD414BE129BB81B32u64;

        invoke_raw_typed!(
            hash,
                player
        )
    }
}

/// ## Parameters
*



pub fn network_get_currently_selected_gamer_handle_from_invite_menu_safe(
        
        
            p0: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x74881E6BCAE2327Cu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x74881E6BCAE2327Cu64;
        
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
pub fn network_get_currently_selected_gamer_handle_from_invite_menu_raw(
        p0: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x74881E6BCAE2327Cu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x74881E6BCAE2327Cu64;

        invoke_raw_typed!(
            hash,
                p0
        )
    }
}

/// ```
In scripts R* calls 'NETWORK_GET_FRIEND_NAME' in this param.  
```



pub fn network_is_friend_in_same_title_safe(
        
        
            friendName: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x2EA9A3BEDF3F17B8u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x2EA9A3BEDF3F17B8u64;
        
        let result = invoke_raw!(
            hash,
                friendName
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn network_is_friend_in_same_title_raw(
        friendName: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x2EA9A3BEDF3F17B8u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x2EA9A3BEDF3F17B8u64;

        invoke_raw_typed!(
            hash,
                friendName
        )
    }
}

/// ```
NETWORK_*

NativeDB Introduced: v323
```



pub fn _network_get_unreliable_resend_count_for_player_safe(
        
        
            player: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x3765C3A3E8192E10u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x3765C3A3E8192E10u64;
        
        let result = invoke_raw!(
            hash,
                player
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn _network_get_unreliable_resend_count_for_player_raw(
        player: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x3765C3A3E8192E10u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x3765C3A3E8192E10u64;

        invoke_raw_typed!(
            hash,
                player
        )
    }
}

/// ## Parameters
*



pub fn network_clan_release_emblem_safe(
        
        
            p0: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x113E6E3E50E286B0u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x113E6E3E50E286B0u64;
        
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
pub fn network_clan_release_emblem_raw(
        p0: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x113E6E3E50E286B0u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x113E6E3E50E286B0u64;

        invoke_raw_typed!(
            hash,
                p0
        )
    }
}

/// ## Return value



pub fn network_get_script_status_safe(
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x57D158647A6BFABFu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x57D158647A6BFABFu64;
        
        let result = invoke_raw!(
            hash,
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn network_get_script_status_raw(
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x57D158647A6BFABFu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x57D158647A6BFABFu64;

        invoke_raw_typed!(
            hash,
        )
    }
}

/// ## Parameters
*



pub fn network_disable_leave_remote_ped_behind_safe(
        
        
            toggle: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xC505036A35AFD01Bu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xC505036A35AFD01Bu64;
        
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
pub fn network_disable_leave_remote_ped_behind_raw(
        toggle: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xC505036A35AFD01Bu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xC505036A35AFD01Bu64;

        invoke_raw_typed!(
            hash,
                toggle
        )
    }
}

/// ```
p2 is true 3/4 of the occurrences I found.  
'players' is the number of players for a session. On PS3/360 it's always 18. On PC it's 32.  
```



pub fn network_do_transition_to_freemode_safe(
        
        
            p0: 
        , 
        
        
            p1: 
        , 
        
        
            p2: 
        , 
        
        
            players: 
        , 
        
        
            p4: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x3AAD8B2FCA1E289Fu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x3AAD8B2FCA1E289Fu64;
        
        let result = invoke_raw!(
            hash,
                p0, 
                p1, 
                p2, 
                players, 
                p4
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn network_do_transition_to_freemode_raw(
        p0: , 
        p1: , 
        p2: , 
        players: , 
        p4: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x3AAD8B2FCA1E289Fu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x3AAD8B2FCA1E289Fu64;

        invoke_raw_typed!(
            hash,
                p0, 
                p1, 
                p2, 
                players, 
                p4
        )
    }
}

/// ## Parameters
*



pub fn network_has_control_of_entity_safe(
        
        
            entity: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x01BF60A500E28887u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x01BF60A500E28887u64;
        
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
pub fn network_has_control_of_entity_raw(
        entity: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x01BF60A500E28887u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x01BF60A500E28887u64;

        invoke_raw_typed!(
            hash,
                entity
        )
    }
}

/// ## Parameters
*



pub fn network_do_transition_to_new_freemode_safe(
        
        
            p0: 
        , 
        
        
            p1: 
        , 
        
        
            players: 
        , 
        
        
            p3: 
        , 
        
        
            p4: 
        , 
        
        
            p5: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x9E80A5BA8109F974u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x9E80A5BA8109F974u64;
        
        let result = invoke_raw!(
            hash,
                p0, 
                p1, 
                players, 
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
pub fn network_do_transition_to_new_freemode_raw(
        p0: , 
        p1: , 
        players: , 
        p3: , 
        p4: , 
        p5: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x9E80A5BA8109F974u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x9E80A5BA8109F974u64;

        invoke_raw_typed!(
            hash,
                p0, 
                p1, 
                players, 
                p3, 
                p4, 
                p5
        )
    }
}

/// ## Return value



pub fn ugc_has_modify_finished_safe(
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x299EF3C576773506u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x299EF3C576773506u64;
        
        let result = invoke_raw!(
            hash,
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn ugc_has_modify_finished_raw(
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x299EF3C576773506u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x299EF3C576773506u64;

        invoke_raw_typed!(
            hash,
        )
    }
}

/// ## Parameters
*



pub fn ugc_get_my_content_safe(
        
        
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
        let hash = 0x3195F8DD0D531052u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x3195F8DD0D531052u64;
        
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
pub fn ugc_get_my_content_raw(
        p0: , 
        p1: , 
        p2: , 
        p3: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x3195F8DD0D531052u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x3195F8DD0D531052u64;

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



pub fn network_has_invited_gamer_to_transition_safe(
        
        
            p0: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x7284A47B3540E6CFu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x7284A47B3540E6CFu64;
        
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
pub fn network_has_invited_gamer_to_transition_raw(
        p0: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x7284A47B3540E6CFu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x7284A47B3540E6CFu64;

        invoke_raw_typed!(
            hash,
                p0
        )
    }
}

/// ## Return value



pub fn network_get_age_group_safe(
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x9614B71F8ADB982Bu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x9614B71F8ADB982Bu64;
        
        let result = invoke_raw!(
            hash,
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn network_get_age_group_raw(
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x9614B71F8ADB982Bu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x9614B71F8ADB982Bu64;

        invoke_raw_typed!(
            hash,
        )
    }
}

/// ```
p0 is always 0. p1 is pointing to a global.  
```



pub fn _0xfb1f9381e80fa13f_safe(
        
        
            p0: 
        , 
        
        
            p1: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xFB1F9381E80FA13Fu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xFB1F9381E80FA13Fu64;
        
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
pub fn _0xfb1f9381e80fa13f_raw(
        p0: , 
        p1: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xFB1F9381E80FA13Fu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xFB1F9381E80FA13Fu64;

        invoke_raw_typed!(
            hash,
                p0, 
                p1
        )
    }
}

/// ## Parameters
*



pub fn network_accept_presence_invite_safe(
        
        
            p0: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xFA91550DF9318B22u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xFA91550DF9318B22u64;
        
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
pub fn network_accept_presence_invite_raw(
        p0: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xFA91550DF9318B22u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xFA91550DF9318B22u64;

        invoke_raw_typed!(
            hash,
                p0
        )
    }
}

/// ## Parameters
*



pub fn _0xcfeb46dcd7d8d5eb_safe(
        
        
            p0: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xCFEB46DCD7D8D5EBu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xCFEB46DCD7D8D5EBu64;
        
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
pub fn _0xcfeb46dcd7d8d5eb_raw(
        p0: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xCFEB46DCD7D8D5EBu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xCFEB46DCD7D8D5EBu64;

        invoke_raw_typed!(
            hash,
                p0
        )
    }
}

/// ```
Subtracts the second argument from the first, then returns whether the result is negative.  
```



pub fn is_time_less_than_safe(
        
        
            timeA: 
        , 
        
        
            timeB: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xCB2CF5148012C8D0u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xCB2CF5148012C8D0u64;
        
        let result = invoke_raw!(
            hash,
                timeA, 
                timeB
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn is_time_less_than_raw(
        timeA: , 
        timeB: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xCB2CF5148012C8D0u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xCB2CF5148012C8D0u64;

        invoke_raw_typed!(
            hash,
                timeA, 
                timeB
        )
    }
}

/// ## Parameters
*



pub fn network_add_entity_area_safe(
        
        
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
        let hash = 0x494C8FB299290269u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x494C8FB299290269u64;
        
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
pub fn network_add_entity_area_raw(
        p0: , 
        p1: , 
        p2: , 
        p3: , 
        p4: , 
        p5: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x494C8FB299290269u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x494C8FB299290269u64;

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

/// ## Return value



pub fn network_is_game_in_progress_safe(
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x10FAB35428CCC9D7u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x10FAB35428CCC9D7u64;
        
        let result = invoke_raw!(
            hash,
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn network_is_game_in_progress_raw(
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x10FAB35428CCC9D7u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x10FAB35428CCC9D7u64;

        invoke_raw_typed!(
            hash,
        )
    }
}

/// ```
NativeDB Added Parameter 1: int p0
NativeDB Added Parameter 2: int p1
NativeDB Added Parameter 3: int p2
```



pub fn network_bail_transition_safe(
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xEAA572036990CD1Bu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xEAA572036990CD1Bu64;
        
        let result = invoke_raw!(
            hash,
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn network_bail_transition_raw(
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xEAA572036990CD1Bu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xEAA572036990CD1Bu64;

        invoke_raw_typed!(
            hash,
        )
    }
}

/// Overrides the game clock time for the local player, allowing for manipulation of the in-game time. This native is effective in both multiplayer and singleplayer modes.



pub fn network_override_clock_time_safe(
        
        
            hours: 
        , 
        
        
            minutes: 
        , 
        
        
            seconds: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xE679E3E06E363892u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xE679E3E06E363892u64;
        
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
pub fn network_override_clock_time_raw(
        hours: , 
        minutes: , 
        seconds: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xE679E3E06E363892u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xE679E3E06E363892u64;

        invoke_raw_typed!(
            hash,
                hours, 
                minutes, 
                seconds
        )
    }
}

/// Sets the alpha value used by [`_SET_LOCAL_PLAYER_AS_GHOST`](#_0x5FFE9B4144F9712F), [`SET_NETWORK_VEHICLE_AS_GHOST`](#_0x6274C4712850841E), and [`_NETWORK_SET_ENTITY_GHOSTED_WITH_OWNER`](#_0x4BA166079D658ED4).



pub fn _reset_ghosted_entity_alpha_safe(
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x17330EBF2F2124A8u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x17330EBF2F2124A8u64;
        
        let result = invoke_raw!(
            hash,
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn _reset_ghosted_entity_alpha_raw(
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x17330EBF2F2124A8u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x17330EBF2F2124A8u64;

        invoke_raw_typed!(
            hash,
        )
    }
}

/// ## Return value



pub fn is_commerce_data_valid_safe(
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xEA14EEF5B7CD2C30u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xEA14EEF5B7CD2C30u64;
        
        let result = invoke_raw!(
            hash,
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn is_commerce_data_valid_raw(
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xEA14EEF5B7CD2C30u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xEA14EEF5B7CD2C30u64;

        invoke_raw_typed!(
            hash,
        )
    }
}

/// ## Return value



pub fn _0x26f07dd83a5f7f98_safe(
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x26F07DD83A5F7F98u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x26F07DD83A5F7F98u64;
        
        let result = invoke_raw!(
            hash,
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn _0x26f07dd83a5f7f98_raw(
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x26F07DD83A5F7F98u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x26F07DD83A5F7F98u64;

        invoke_raw_typed!(
            hash,
        )
    }
}

/// Disconnects you from the session, and starts loading single player, however you still remain connected to the server (only if you're the host, if you're not then you also (most likely) get disconnected from the server) and other players will not be able to join until you exit the game.

If you're already in SP then it'll re-load singleplayer.

You might need to DoScreenFadeIn and ShutdownLoadingScreen otherwise you probably won't end up loading into SP at all.

Somewhat related note: opening the pause menu after loading into this 'singleplayer' mode crashes the game.



pub fn _shutdown_and_load_most_recent_save_safe(
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x9ECA15ADFE141431u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x9ECA15ADFE141431u64;
        
        let result = invoke_raw!(
            hash,
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn _shutdown_and_load_most_recent_save_raw(
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x9ECA15ADFE141431u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x9ECA15ADFE141431u64;

        invoke_raw_typed!(
            hash,
        )
    }
}

/// ## Parameters
*



pub fn set_network_id_always_exists_for_player_safe(
        
        
            netId: 
        , 
        
        
            player: 
        , 
        
        
            toggle: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xA8A024587329F36Au32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xA8A024587329F36Au64;
        
        let result = invoke_raw!(
            hash,
                netId, 
                player, 
                toggle
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn set_network_id_always_exists_for_player_raw(
        netId: , 
        player: , 
        toggle: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xA8A024587329F36Au32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xA8A024587329F36Au64;

        invoke_raw_typed!(
            hash,
                netId, 
                player, 
                toggle
        )
    }
}

/// **This native does absolutely nothing, just a nullsub**



pub fn _0xb606e6cc59664972_safe(
        
        
            p0: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xB606E6CC59664972u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xB606E6CC59664972u64;
        
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
pub fn _0xb606e6cc59664972_raw(
        p0: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xB606E6CC59664972u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xB606E6CC59664972u64;

        invoke_raw_typed!(
            hash,
                p0
        )
    }
}

/// ## Parameters
*



pub fn network_apply_ped_scar_data_safe(
        
        
            ped: 
        , 
        
        
            p1: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xE66C690248F11150u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xE66C690248F11150u64;
        
        let result = invoke_raw!(
            hash,
                ped, 
                p1
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn network_apply_ped_scar_data_raw(
        ped: , 
        p1: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xE66C690248F11150u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xE66C690248F11150u64;

        invoke_raw_typed!(
            hash,
                ped, 
                p1
        )
    }
}

/// The Native returns a hash of the session id as string from the specific invite index!



pub fn network_get_presence_invite_session_id_safe(
        
        
            inviteIndex: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x26E1CD96B0903D60u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x26E1CD96B0903D60u64;
        
        let result = invoke_raw!(
            hash,
                inviteIndex
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn network_get_presence_invite_session_id_raw(
        inviteIndex: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x26E1CD96B0903D60u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x26E1CD96B0903D60u64;

        invoke_raw_typed!(
            hash,
                inviteIndex
        )
    }
}

/// ## Return value



pub fn ugc_has_create_finished_safe(
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x5E24341A7F92A74Bu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x5E24341A7F92A74Bu64;
        
        let result = invoke_raw!(
            hash,
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn ugc_has_create_finished_raw(
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x5E24341A7F92A74Bu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x5E24341A7F92A74Bu64;

        invoke_raw_typed!(
            hash,
        )
    }
}

/// _0x1153FA02A659051C native function



pub fn _0x1153fa02a659051c_safe(
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x1153FA02A659051Cu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x1153FA02A659051Cu64;
        
        let result = invoke_raw!(
            hash,
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn _0x1153fa02a659051c_raw(
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x1153FA02A659051Cu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x1153FA02A659051Cu64;

        invoke_raw_typed!(
            hash,
        )
    }
}

/// ```
Returns a NetworkHandle* from the specified user ID and stores it in a given buffer.  
* Currently unknown struct  
```



pub fn network_handle_from_user_id_safe(
        
        
            userId: 
        , 
        
        
            networkHandle: 
        , 
        
        
            bufferSize: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xDCD51DD8F87AEC5Cu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xDCD51DD8F87AEC5Cu64;
        
        let result = invoke_raw!(
            hash,
                userId, 
                networkHandle, 
                bufferSize
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn network_handle_from_user_id_raw(
        userId: , 
        networkHandle: , 
        bufferSize: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xDCD51DD8F87AEC5Cu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xDCD51DD8F87AEC5Cu64;

        invoke_raw_typed!(
            hash,
                userId, 
                networkHandle, 
                bufferSize
        )
    }
}

/// ## Return value



pub fn _0x45e816772e93a9db_safe(
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x45E816772E93A9DBu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x45E816772E93A9DBu64;
        
        let result = invoke_raw!(
            hash,
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn _0x45e816772e93a9db_raw(
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x45E816772E93A9DBu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x45E816772E93A9DBu64;

        invoke_raw_typed!(
            hash,
        )
    }
}

/// ## Parameters
*



pub fn network_set_this_script_is_network_script_safe(
        
        
            maxNumMissionParticipants: 
        , 
        
        
            p1: 
        , 
        
        
            instanceId: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x1CA59E306ECB80A5u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x1CA59E306ECB80A5u64;
        
        let result = invoke_raw!(
            hash,
                maxNumMissionParticipants, 
                p1, 
                instanceId
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn network_set_this_script_is_network_script_raw(
        maxNumMissionParticipants: , 
        p1: , 
        instanceId: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x1CA59E306ECB80A5u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x1CA59E306ECB80A5u64;

        invoke_raw_typed!(
            hash,
                maxNumMissionParticipants, 
                p1, 
                instanceId
        )
    }
}

/// ```c
enum eMpSettingSpawn
{
	MP_SETTING_SPAWN_NULL = 0,
	MP_SETTING_SPAWN_PROPERTY = 1,
	MP_SETTING_SPAWN_LAST_POSITION = 2,
	MP_SETTING_SPAWN_GARAGE = 3,
	MP_SETTING_SPAWN_RANDOM = 4,
	MP_SETTING_SPAWN_PRIVATE_YACHT = 5,
	MP_SETTING_SPAWN_OFFICE = 6,
	MP_SETTING_SPAWN_CLUBHOUSE = 7,
	MP_SETTING_SPAWN_IE_WAREHOUSE = 8,
	MP_SETTING_SPAWN_BUNKER = 9,
	MP_SETTING_SPAWN_HANGAR = 10,
	MP_SETTING_SPAWN_DEFUNCT_BASE = 11,
	MP_SETTING_SPAWN_NIGHTCLUB = 12,
	MP_SETTING_SPAWN_ARENA_GARAGE = 13,
	MP_SETTING_SPAWN_CASINO_APARTMENT = 14,
	MP_SETTING_SPAWN_ARCADE = 15,
	MP_SETTING_SPAWN_SUBMARINE = 16,
	MP_SETTING_SPAWN_CAR_MEET = 17,
	MP_SETTING_SPAWN_AUTO_SHOP = 18,
	MP_SETTING_SPAWN_FIXER_HQ = 19,
	MP_SETTING_SPAWN_MAX = 20,
};
```

```
NativeDB Introduced: v2699
```



pub fn _network_set_current_spawn_setting_safe(
        
        
            mpSettingSpawn: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xAA6D5451DC3448B6u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xAA6D5451DC3448B6u64;
        
        let result = invoke_raw!(
            hash,
                mpSettingSpawn
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn _network_set_current_spawn_setting_raw(
        mpSettingSpawn: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xAA6D5451DC3448B6u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xAA6D5451DC3448B6u64;

        invoke_raw_typed!(
            hash,
                mpSettingSpawn
        )
    }
}

/// ## Parameters
*



pub fn network_set_transition_creator_handle_safe(
        
        
            p0: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xEF26739BCD9907D5u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xEF26739BCD9907D5u64;
        
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
pub fn network_set_transition_creator_handle_raw(
        p0: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xEF26739BCD9907D5u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xEF26739BCD9907D5u64;

        invoke_raw_typed!(
            hash,
                p0
        )
    }
}

/// ```
NativeDB Introduced: v1734
```



pub fn _0xdb663cc9ff3407a9_safe(
        
        
            player: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xDB663CC9FF3407A9u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xDB663CC9FF3407A9u64;
        
        let result = invoke_raw!(
            hash,
                player
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn _0xdb663cc9ff3407a9_raw(
        player: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xDB663CC9FF3407A9u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xDB663CC9FF3407A9u64;

        invoke_raw_typed!(
            hash,
                player
        )
    }
}

/// ## Parameters
*



pub fn network_set_friendly_fire_option_safe(
        
        
            toggle: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xF808475FA571D823u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xF808475FA571D823u64;
        
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
pub fn network_set_friendly_fire_option_raw(
        toggle: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xF808475FA571D823u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xF808475FA571D823u64;

        invoke_raw_typed!(
            hash,
                toggle
        )
    }
}

/// ## Parameters
*



pub fn network_override_team_restrictions_safe(
        
        
            team: 
        , 
        
        
            toggle: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x6F697A66CE78674Eu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x6F697A66CE78674Eu64;
        
        let result = invoke_raw!(
            hash,
                team, 
                toggle
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn network_override_team_restrictions_raw(
        team: , 
        toggle: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x6F697A66CE78674Eu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x6F697A66CE78674Eu64;

        invoke_raw_typed!(
            hash,
                team, 
                toggle
        )
    }
}

/// Sets the provided entity not visible for yourself for the current frame.



pub fn set_entity_locally_invisible_safe(
        
        
            entity: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xE135A9FF3F5D05D8u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xE135A9FF3F5D05D8u64;
        
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
pub fn set_entity_locally_invisible_raw(
        entity: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xE135A9FF3F5D05D8u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xE135A9FF3F5D05D8u64;

        invoke_raw_typed!(
            hash,
                entity
        )
    }
}

/// ```
NativeDB Introduced: v2372
```



pub fn _network_get_position_hash_of_this_script_safe(
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x257ED0FADF750BCFu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x257ED0FADF750BCFu64;
        
        let result = invoke_raw!(
            hash,
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn _network_get_position_hash_of_this_script_raw(
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x257ED0FADF750BCFu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x257ED0FADF750BCFu64;

        invoke_raw_typed!(
            hash,
        )
    }
}

/// ```
NETWORK_GET_*

NativeDB Introduced: v323
```



pub fn _network_get_average_packet_loss_for_player_safe(
        
        
            player: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x350C23949E43686Cu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x350C23949E43686Cu64;
        
        let result = invoke_raw!(
            hash,
                player
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn _network_get_average_packet_loss_for_player_raw(
        player: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x350C23949E43686Cu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x350C23949E43686Cu64;

        invoke_raw_typed!(
            hash,
                player
        )
    }
}

/// ```
NETWORK_SET_*
```



pub fn _0x0d77a82dc2d0da59_safe(
        
        
            p0: 
        , 
        
        
            p1: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x0D77A82DC2D0DA59u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x0D77A82DC2D0DA59u64;
        
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
pub fn _0x0d77a82dc2d0da59_raw(
        p0: , 
        p1: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x0D77A82DC2D0DA59u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x0D77A82DC2D0DA59u64;

        invoke_raw_typed!(
            hash,
                p0, 
                p1
        )
    }
}

/// **This native does absolutely nothing, just a nullsub**



pub fn _0x6bff5f84102df80a_safe(
        
        
            player: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x6BFF5F84102DF80Au32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x6BFF5F84102DF80Au64;
        
        let result = invoke_raw!(
            hash,
                player
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn _0x6bff5f84102df80a_raw(
        player: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x6BFF5F84102DF80Au32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x6BFF5F84102DF80Au64;

        invoke_raw_typed!(
            hash,
                player
        )
    }
}

/// ## Parameters
*



pub fn _0x77faddcbe3499df7_safe(
        
        
            p0: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x77FADDCBE3499DF7u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x77FADDCBE3499DF7u64;
        
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
pub fn _0x77faddcbe3499df7_raw(
        p0: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x77FADDCBE3499DF7u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x77FADDCBE3499DF7u64;

        invoke_raw_typed!(
            hash,
                p0
        )
    }
}

/// ## Parameters
*



pub fn network_check_user_content_privileges_safe(
        
        
            p0: 
        , 
        
        
            p1: 
        , 
        
        
            p2: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x595F028698072DD9u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x595F028698072DD9u64;
        
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
pub fn network_check_user_content_privileges_raw(
        p0: , 
        p1: , 
        p2: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x595F028698072DD9u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x595F028698072DD9u64;

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



pub fn _0x1398582b7f72b3ed_safe(
        
        
            p0: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x1398582B7F72B3EDu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x1398582B7F72B3EDu64;
        
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
pub fn _0x1398582b7f72b3ed_raw(
        p0: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x1398582B7F72B3EDu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x1398582B7F72B3EDu64;

        invoke_raw_typed!(
            hash,
                p0
        )
    }
}

/// ```
NativeDB Introduced: v1493
```



pub fn _0x906ca41a4b74eca4_safe(
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x906CA41A4B74ECA4u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x906CA41A4B74ECA4u64;
        
        let result = invoke_raw!(
            hash,
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn _0x906ca41a4b74eca4_raw(
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x906CA41A4B74ECA4u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x906CA41A4B74ECA4u64;

        invoke_raw_typed!(
            hash,
        )
    }
}

/// ## Parameters
*



pub fn network_is_transition_host_from_handle_safe(
        
        
            networkHandle: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x6B5C83BA3EFE6A10u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x6B5C83BA3EFE6A10u64;
        
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
pub fn network_is_transition_host_from_handle_raw(
        networkHandle: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x6B5C83BA3EFE6A10u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x6B5C83BA3EFE6A10u64;

        invoke_raw_typed!(
            hash,
                networkHandle
        )
    }
}

/// ## Parameters
*



pub fn ugc_get_content_description_hash_safe(
        
        
            p0: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x7CF0448787B23758u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x7CF0448787B23758u64;
        
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
pub fn ugc_get_content_description_hash_raw(
        p0: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x7CF0448787B23758u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x7CF0448787B23758u64;

        invoke_raw_typed!(
            hash,
                p0
        )
    }
}

/// ## Parameters
*



pub fn network_get_entity_is_local_safe(
        
        
            entity: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x0991549DE4D64762u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x0991549DE4D64762u64;
        
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
pub fn network_get_entity_is_local_raw(
        entity: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x0991549DE4D64762u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x0991549DE4D64762u64;

        invoke_raw_typed!(
            hash,
                entity
        )
    }
}

/// ```
NativeDB Introduced: v2699
```



pub fn _network_set_current_mission_id_safe(
        
        
            missionId: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x2C863ACDCD12B3DBu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x2C863ACDCD12B3DBu64;
        
        let result = invoke_raw!(
            hash,
                missionId
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn _network_set_current_mission_id_raw(
        missionId: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x2C863ACDCD12B3DBu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x2C863ACDCD12B3DBu64;

        invoke_raw_typed!(
            hash,
                missionId
        )
    }
}

/// ## Parameters
*



pub fn refresh_player_list_stats_safe(
        
        
            p0: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xE26CCFF8094D8C74u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xE26CCFF8094D8C74u64;
        
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
pub fn refresh_player_list_stats_raw(
        p0: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xE26CCFF8094D8C74u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xE26CCFF8094D8C74u64;

        invoke_raw_typed!(
            hash,
                p0
        )
    }
}

/// ## Parameters
*



pub fn network_get_background_loading_recipients_safe(
        
        
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
        let hash = 0x97A770BEEF227E2Bu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x97A770BEEF227E2Bu64;
        
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
pub fn network_get_background_loading_recipients_raw(
        p0: , 
        p1: , 
        p2: , 
        p3: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x97A770BEEF227E2Bu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x97A770BEEF227E2Bu64;

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
NETWORK_RE*
```



pub fn _0xf083835b70ba9bfe_safe(
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xF083835B70BA9BFEu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xF083835B70BA9BFEu64;
        
        let result = invoke_raw!(
            hash,
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn _0xf083835b70ba9bfe_raw(
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xF083835B70BA9BFEu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xF083835B70BA9BFEu64;

        invoke_raw_typed!(
            hash,
        )
    }
}

/// ```
Returns whether the game is not in offline mode.  
seemed not to work for some ppl  
```



pub fn network_is_signed_online_safe(
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x1077788E268557C2u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x1077788E268557C2u64;
        
        let result = invoke_raw!(
            hash,
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn network_is_signed_online_raw(
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x1077788E268557C2u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x1077788E268557C2u64;

        invoke_raw_typed!(
            hash,
        )
    }
}

/// ## Return value



pub fn network_get_host_of_this_script_safe(
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xC7B4D79B01FA7A5Cu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xC7B4D79B01FA7A5Cu64;
        
        let result = invoke_raw!(
            hash,
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn network_get_host_of_this_script_raw(
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xC7B4D79B01FA7A5Cu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xC7B4D79B01FA7A5Cu64;

        invoke_raw_typed!(
            hash,
        )
    }
}

/// _0x83FE8D7229593017 native function



pub fn _0x83fe8d7229593017_safe(
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x83FE8D7229593017u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x83FE8D7229593017u64;
        
        let result = invoke_raw!(
            hash,
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn _0x83fe8d7229593017_raw(
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x83FE8D7229593017u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x83FE8D7229593017u64;

        invoke_raw_typed!(
            hash,
        )
    }
}

/// ## Parameters
*



pub fn ugc_query_by_content_id_safe(
        
        
            contentId: 
        , 
        
        
            latestVersion: 
        , 
        
        
            contentTypeName: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x158EC424F35EC469u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x158EC424F35EC469u64;
        
        let result = invoke_raw!(
            hash,
                contentId, 
                latestVersion, 
                contentTypeName
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn ugc_query_by_content_id_raw(
        contentId: , 
        latestVersion: , 
        contentTypeName: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x158EC424F35EC469u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x158EC424F35EC469u64;

        invoke_raw_typed!(
            hash,
                contentId, 
                latestVersion, 
                contentTypeName
        )
    }
}

/// ## Return value
Hard-coded to always return 0.



pub fn _0xa0fa4ec6a05da44e_safe(
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xA0FA4EC6A05DA44Eu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xA0FA4EC6A05DA44Eu64;
        
        let result = invoke_raw!(
            hash,
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn _0xa0fa4ec6a05da44e_raw(
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xA0FA4EC6A05DA44Eu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xA0FA4EC6A05DA44Eu64;

        invoke_raw_typed!(
            hash,
        )
    }
}

/// ```
NativeDB Introduced: v2060
```



pub fn _0x17c9e241111a674d_safe(
        
        
            p0: 
        , 
        
        
            p1: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x17C9E241111A674Du32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x17C9E241111A674Du64;
        
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
pub fn _0x17c9e241111a674d_raw(
        p0: , 
        p1: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x17C9E241111A674Du32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x17C9E241111A674Du64;

        invoke_raw_typed!(
            hash,
                p0, 
                p1
        )
    }
}

/// ## Return value



pub fn network_is_transition_closed_friends_safe(
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x6512765E3BE78C50u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x6512765E3BE78C50u64;
        
        let result = invoke_raw!(
            hash,
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn network_is_transition_closed_friends_raw(
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x6512765E3BE78C50u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x6512765E3BE78C50u64;

        invoke_raw_typed!(
            hash,
        )
    }
}

/// ## Return value



pub fn _0xb5d3453c98456528_safe(
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xB5D3453C98456528u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xB5D3453C98456528u64;
        
        let result = invoke_raw!(
            hash,
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn _0xb5d3453c98456528_raw(
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xB5D3453C98456528u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xB5D3453C98456528u64;

        invoke_raw_typed!(
            hash,
        )
    }
}

/// ## Parameters
*



pub fn reserve_network_mission_vehicles_safe(
        
        
            amount: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x76B02E21ED27A469u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x76B02E21ED27A469u64;
        
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
pub fn reserve_network_mission_vehicles_raw(
        amount: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x76B02E21ED27A469u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x76B02E21ED27A469u64;

        invoke_raw_typed!(
            hash,
                amount
        )
    }
}

/// ```
NativeDB Introduced: v1604
```



pub fn _0xa6fceccf4721d679_safe(
        
        
            p0: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xA6FCECCF4721D679u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xA6FCECCF4721D679u64;
        
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
pub fn _0xa6fceccf4721d679_raw(
        p0: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xA6FCECCF4721D679u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xA6FCECCF4721D679u64;

        invoke_raw_typed!(
            hash,
                p0
        )
    }
}

/// ```
NativeDB Introduced: v1180
```



pub fn _0x1f7bc3539f9e0224_safe(
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x1F7BC3539F9E0224u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x1F7BC3539F9E0224u64;
        
        let result = invoke_raw!(
            hash,
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn _0x1f7bc3539f9e0224_raw(
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x1F7BC3539F9E0224u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x1F7BC3539F9E0224u64;

        invoke_raw_typed!(
            hash,
        )
    }
}

/// ## Parameters
*



pub fn network_clan_download_membership_pending_safe(
        
        
            p0: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x5B9E023DC6EBEDC0u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x5B9E023DC6EBEDC0u64;
        
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
pub fn network_clan_download_membership_pending_raw(
        p0: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x5B9E023DC6EBEDC0u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x5B9E023DC6EBEDC0u64;

        invoke_raw_typed!(
            hash,
                p0
        )
    }
}

/// ## Return value



pub fn network_has_social_club_account_safe(
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x67A5589628E0CFF6u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x67A5589628E0CFF6u64;
        
        let result = invoke_raw!(
            hash,
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn network_has_social_club_account_raw(
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x67A5589628E0CFF6u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x67A5589628E0CFF6u64;

        invoke_raw_typed!(
            hash,
        )
    }
}

/// ## Parameters
*



pub fn network_session_voice_respond_to_request_safe(
        
        
            p0: 
        , 
        
        
            p1: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x7F8413B7FC2AA6B9u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x7F8413B7FC2AA6B9u64;
        
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
pub fn network_session_voice_respond_to_request_raw(
        p0: , 
        p1: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x7F8413B7FC2AA6B9u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x7F8413B7FC2AA6B9u64;

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



pub fn _0x9d724b400a7e8ffc_safe(
        
        
            p0: 
        , 
        
        
            p1: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x9D724B400A7E8FFCu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x9D724B400A7E8FFCu64;
        
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
pub fn _0x9d724b400a7e8ffc_raw(
        p0: , 
        p1: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x9D724B400A7E8FFCu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x9D724B400A7E8FFCu64;

        invoke_raw_typed!(
            hash,
                p0, 
                p1
        )
    }
}

/// ## Parameters
*



pub fn can_register_mission_vehicles_safe(
        
        
            amount: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x7277F1F2E085EE74u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x7277F1F2E085EE74u64;
        
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
pub fn can_register_mission_vehicles_raw(
        amount: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x7277F1F2E085EE74u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x7277F1F2E085EE74u64;

        invoke_raw_typed!(
            hash,
                amount
        )
    }
}

/// ## Parameters
*



pub fn network_session_mark_visible_safe(
        
        
            toggle: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x271CC6AB59EBF9A5u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x271CC6AB59EBF9A5u64;
        
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
pub fn network_session_mark_visible_raw(
        toggle: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x271CC6AB59EBF9A5u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x271CC6AB59EBF9A5u64;

        invoke_raw_typed!(
            hash,
                toggle
        )
    }
}

/// ## Parameters
*



pub fn network_has_invited_gamer_safe(
        
        
            p0: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x4D86CD31E8976ECEu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x4D86CD31E8976ECEu64;
        
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
pub fn network_has_invited_gamer_raw(
        p0: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x4D86CD31E8976ECEu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x4D86CD31E8976ECEu64;

        invoke_raw_typed!(
            hash,
                p0
        )
    }
}

/// ## Parameters
*



pub fn network_set_transition_activity_id_safe(
        
        
            p0: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x30DE938B516F0AD2u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x30DE938B516F0AD2u64;
        
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
pub fn network_set_transition_activity_id_raw(
        p0: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x30DE938B516F0AD2u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x30DE938B516F0AD2u64;

        invoke_raw_typed!(
            hash,
                p0
        )
    }
}

/// Returns the Name of the inviter of the specific selected Invite.



pub fn network_get_presence_invite_inviter_safe(
        
        
            inviteIndex: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x4962CC4AA2F345B7u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x4962CC4AA2F345B7u64;
        
        let result = invoke_raw!(
            hash,
                inviteIndex
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn network_get_presence_invite_inviter_raw(
        inviteIndex: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x4962CC4AA2F345B7u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x4962CC4AA2F345B7u64;

        invoke_raw_typed!(
            hash,
                inviteIndex
        )
    }
}

/// ## Parameters
*



pub fn ugc_get_crew_content_safe(
        
        
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
        let hash = 0x9F6E2821885CAEE2u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x9F6E2821885CAEE2u64;
        
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
pub fn ugc_get_crew_content_raw(
        p0: , 
        p1: , 
        p2: , 
        p3: , 
        p4: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x9F6E2821885CAEE2u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x9F6E2821885CAEE2u64;

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



pub fn network_request_cloud_background_scripts_safe(
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x924426BFFD82E915u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x924426BFFD82E915u64;
        
        let result = invoke_raw!(
            hash,
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn network_request_cloud_background_scripts_raw(
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x924426BFFD82E915u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x924426BFFD82E915u64;

        invoke_raw_typed!(
            hash,
        )
    }
}

/// ```
NativeDB Added Parameter 7: Any p6
NativeDB Added Parameter 8: Any p7
```



pub fn network_do_transition_quickmatch_with_group_safe(
        
        
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
        let hash = 0x9C4AB58491FDC98Au32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x9C4AB58491FDC98Au64;
        
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
pub fn network_do_transition_quickmatch_with_group_raw(
        p0: , 
        p1: , 
        p2: , 
        p3: , 
        p4: , 
        p5: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x9C4AB58491FDC98Au32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x9C4AB58491FDC98Au64;

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



pub fn _0xebfa8d50addc54c4_safe(
        
        
            p0: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xEBFA8D50ADDC54C4u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xEBFA8D50ADDC54C4u64;
        
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
pub fn _0xebfa8d50addc54c4_raw(
        p0: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xEBFA8D50ADDC54C4u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xEBFA8D50ADDC54C4u64;

        invoke_raw_typed!(
            hash,
                p0
        )
    }
}

/// Returns true if the specified network id is controlled by someone else.



pub fn _network_is_network_id_a_clone_safe(
        
        
            netId: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x7242F8B741CE1086u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x7242F8B741CE1086u64;
        
        let result = invoke_raw!(
            hash,
                netId
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn _network_is_network_id_a_clone_raw(
        netId: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x7242F8B741CE1086u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x7242F8B741CE1086u64;

        invoke_raw_typed!(
            hash,
                netId
        )
    }
}

/// ## Parameters
*



pub fn network_get_player_owns_waypoint_safe(
        
        
            player: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x82377B65E943F72Du32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x82377B65E943F72Du64;
        
        let result = invoke_raw!(
            hash,
                player
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn network_get_player_owns_waypoint_raw(
        player: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x82377B65E943F72Du32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x82377B65E943F72Du64;

        invoke_raw_typed!(
            hash,
                player
        )
    }
}

/// ```
Same as GET_RANDOM_INT_IN_RANGE
```



pub fn network_get_random_int_ranged_safe(
        
        
            rangeStart: 
        , 
        
        
            rangeEnd: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xE30CF56F1EFA5F43u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xE30CF56F1EFA5F43u64;
        
        let result = invoke_raw!(
            hash,
                rangeStart, 
                rangeEnd
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn network_get_random_int_ranged_raw(
        rangeStart: , 
        rangeEnd: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xE30CF56F1EFA5F43u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xE30CF56F1EFA5F43u64;

        invoke_raw_typed!(
            hash,
                rangeStart, 
                rangeEnd
        )
    }
}

/// ## Parameters
*



pub fn network_session_friend_matchmaking_safe(
        
        
            p0: 
        , 
        
        
            p1: 
        , 
        
        
            maxPlayers: 
        , 
        
        
            p3: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x2CFC76E0D087C994u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x2CFC76E0D087C994u64;
        
        let result = invoke_raw!(
            hash,
                p0, 
                p1, 
                maxPlayers, 
                p3
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn network_session_friend_matchmaking_raw(
        p0: , 
        p1: , 
        maxPlayers: , 
        p3: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x2CFC76E0D087C994u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x2CFC76E0D087C994u64;

        invoke_raw_typed!(
            hash,
                p0, 
                p1, 
                maxPlayers, 
                p3
        )
    }
}

/// ## Parameters
*



pub fn _0x5324a0e3e4ce3570_safe(
        
        
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
        let hash = 0x5324A0E3E4CE3570u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x5324A0E3E4CE3570u64;
        
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
pub fn _0x5324a0e3e4ce3570_raw(
        p0: , 
        p1: , 
        p2: , 
        p3: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x5324A0E3E4CE3570u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x5324A0E3E4CE3570u64;

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
NativeDB Introduced: v2699
```



pub fn _network_bail_transition_quickmatch_safe(
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x023782EFC70585EEu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x023782EFC70585EEu64;
        
        let result = invoke_raw!(
            hash,
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn _network_bail_transition_quickmatch_raw(
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x023782EFC70585EEu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x023782EFC70585EEu64;

        invoke_raw_typed!(
            hash,
        )
    }
}

/// ```
Note according to IDA TU27 X360(Console),  
This native & 'NETWORK_IS_PARTY_MEMBER' both jump to the same location.  
Side note: This location just stops where it's at once jumped to.  
Screenshot for side note,   
h t t p ://i.imgur.com/m2ci1mF.png  
h t t p://i.imgur.com/Z0Wx2B6.png  
```



pub fn network_is_in_party_safe(
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x966C2BC2A7FE3F30u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x966C2BC2A7FE3F30u64;
        
        let result = invoke_raw!(
            hash,
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn network_is_in_party_raw(
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x966C2BC2A7FE3F30u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x966C2BC2A7FE3F30u64;

        invoke_raw_typed!(
            hash,
        )
    }
}

/// ## Parameters
*



pub fn _network_block_kicked_players_safe(
        
        
            p0: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x6B07B9CE4D390375u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x6B07B9CE4D390375u64;
        
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
pub fn _network_block_kicked_players_raw(
        p0: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x6B07B9CE4D390375u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x6B07B9CE4D390375u64;

        invoke_raw_typed!(
            hash,
                p0
        )
    }
}

/// ## Parameters
*



pub fn network_is_player_active_safe(
        
        
            player: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xB8DFD30D6973E135u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xB8DFD30D6973E135u64;
        
        let result = invoke_raw!(
            hash,
                player
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn network_is_player_active_raw(
        player: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xB8DFD30D6973E135u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xB8DFD30D6973E135u64;

        invoke_raw_typed!(
            hash,
                player
        )
    }
}

/// ## Parameters
*



pub fn ugc_is_language_supported_safe(
        
        
            p0: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xF53E48461B71EECBu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xF53E48461B71EECBu64;
        
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
pub fn ugc_is_language_supported_raw(
        p0: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xF53E48461B71EECBu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xF53E48461B71EECBu64;

        invoke_raw_typed!(
            hash,
                p0
        )
    }
}

/// ## Parameters
*



pub fn is_damage_tracker_active_on_network_id_safe(
        
        
            netID: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x6E192E33AD436366u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x6E192E33AD436366u64;
        
        let result = invoke_raw!(
            hash,
                netID
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn is_damage_tracker_active_on_network_id_raw(
        netID: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x6E192E33AD436366u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x6E192E33AD436366u64;

        invoke_raw_typed!(
            hash,
                netID
        )
    }
}

/// ## Parameters
*



pub fn network_is_gamer_in_my_session_safe(
        
        
            networkHandle: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x0F10B05DDF8D16E9u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x0F10B05DDF8D16E9u64;
        
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
pub fn network_is_gamer_in_my_session_raw(
        networkHandle: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x0F10B05DDF8D16E9u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x0F10B05DDF8D16E9u64;

        invoke_raw_typed!(
            hash,
                networkHandle
        )
    }
}

/// ```
Used by Metric VEHICLE_DIST_DRIVEN
```

```
NativeDB Introduced: v2699
```



pub fn _network_set_vehicle_test_drive_safe(
        
        
            toggle: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x8C70252FC40F320Bu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x8C70252FC40F320Bu64;
        
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
pub fn _network_set_vehicle_test_drive_raw(
        toggle: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x8C70252FC40F320Bu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x8C70252FC40F320Bu64;

        invoke_raw_typed!(
            hash,
                toggle
        )
    }
}

/// ```
NETWORK_IS_TRANSITION_*
```



pub fn _0xc571d0e77d8bbc29_safe(
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xC571D0E77D8BBC29u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xC571D0E77D8BBC29u64;
        
        let result = invoke_raw!(
            hash,
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn _0xc571d0e77d8bbc29_raw(
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xC571D0E77D8BBC29u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xC571D0E77D8BBC29u64;

        invoke_raw_typed!(
            hash,
        )
    }
}

/// ```
NETWORK_IS_*

NativeDB Introduced: v323
```



pub fn _network_is_connection_endpoint_relay_server_safe(
        
        
            player: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x16D3D49902F697BBu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x16D3D49902F697BBu64;
        
        let result = invoke_raw!(
            hash,
                player
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn _network_is_connection_endpoint_relay_server_raw(
        player: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x16D3D49902F697BBu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x16D3D49902F697BBu64;

        invoke_raw_typed!(
            hash,
                player
        )
    }
}

/// ## Parameters
*



pub fn _network_get_friend_name_from_index_safe(
        
        
            friendIndex: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x4164F227D052E293u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x4164F227D052E293u64;
        
        let result = invoke_raw!(
            hash,
                friendIndex
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn _network_get_friend_name_from_index_raw(
        friendIndex: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x4164F227D052E293u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x4164F227D052E293u64;

        invoke_raw_typed!(
            hash,
                friendIndex
        )
    }
}

/// ```
NETWORK_IS_*
```



pub fn _0x14922ed3e38761f0_safe(
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x14922ED3E38761F0u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x14922ED3E38761F0u64;
        
        let result = invoke_raw!(
            hash,
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn _0x14922ed3e38761f0_raw(
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x14922ED3E38761F0u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x14922ED3E38761F0u64;

        invoke_raw_typed!(
            hash,
        )
    }
}

/// ## Parameters
*



pub fn network_get_destroyer_of_network_id_safe(
        
        
            netId: 
        , 
        
        
            weaponHash: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x7A1ADEEF01740A24u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x7A1ADEEF01740A24u64;
        
        let result = invoke_raw!(
            hash,
                netId, 
                weaponHash
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn network_get_destroyer_of_network_id_raw(
        netId: , 
        weaponHash: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x7A1ADEEF01740A24u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x7A1ADEEF01740A24u64;

        invoke_raw_typed!(
            hash,
                netId, 
                weaponHash
        )
    }
}

/// ## Parameters
*



pub fn _0x4811bbac21c5fcd5_safe(
        
        
            p0: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x4811BBAC21C5FCD5u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x4811BBAC21C5FCD5u64;
        
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
pub fn _0x4811bbac21c5fcd5_raw(
        p0: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x4811BBAC21C5FCD5u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x4811BBAC21C5FCD5u64;

        invoke_raw_typed!(
            hash,
                p0
        )
    }
}

/// ```
Hardcoded to not work in SP.  
```



pub fn fade_out_local_player_safe(
        
        
            p0: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x416DBD4CD6ED8DD2u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x416DBD4CD6ED8DD2u64;
        
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
pub fn fade_out_local_player_raw(
        p0: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x416DBD4CD6ED8DD2u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x416DBD4CD6ED8DD2u64;

        invoke_raw_typed!(
            hash,
                p0
        )
    }
}

/// ## Parameters
*



pub fn network_is_player_in_mp_cutscene_safe(
        
        
            player: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x63F9EE203C3619F2u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x63F9EE203C3619F2u64;
        
        let result = invoke_raw!(
            hash,
                player
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn network_is_player_in_mp_cutscene_raw(
        player: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x63F9EE203C3619F2u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x63F9EE203C3619F2u64;

        invoke_raw_typed!(
            hash,
                player
        )
    }
}

/// Sets the provided entity visible for yourself for the current frame.



pub fn set_entity_locally_visible_safe(
        
        
            entity: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x241E289B5C059EDCu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x241E289B5C059EDCu64;
        
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
pub fn set_entity_locally_visible_raw(
        entity: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x241E289B5C059EDCu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x241E289B5C059EDCu64;

        invoke_raw_typed!(
            hash,
                entity
        )
    }
}

/// ## Return value



pub fn get_num_commerce_items_safe(
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xF2EAC213D5EA0623u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xF2EAC213D5EA0623u64;
        
        let result = invoke_raw!(
            hash,
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn get_num_commerce_items_raw(
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xF2EAC213D5EA0623u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xF2EAC213D5EA0623u64;

        invoke_raw_typed!(
            hash,
        )
    }
}

/// ## Parameters
*



pub fn network_access_tunable_bool_safe(
        
        
            tunableContext: 
        , 
        
        
            tunableName: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xAA6A47A573ABB75Au32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xAA6A47A573ABB75Au64;
        
        let result = invoke_raw!(
            hash,
                tunableContext, 
                tunableName
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn network_access_tunable_bool_raw(
        tunableContext: , 
        tunableName: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xAA6A47A573ABB75Au32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xAA6A47A573ABB75Au64;

        invoke_raw_typed!(
            hash,
                tunableContext, 
                tunableName
        )
    }
}

/// _0x9465E683B12D3F6B native function



pub fn _0x9465e683b12d3f6b_safe(
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x9465E683B12D3F6Bu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x9465E683B12D3F6Bu64;
        
        let result = invoke_raw!(
            hash,
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn _0x9465e683b12d3f6b_raw(
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x9465E683B12D3F6Bu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x9465E683B12D3F6Bu64;

        invoke_raw_typed!(
            hash,
        )
    }
}

/// NETWORK_CLEAR_CLOCK_TIME_OVERRIDE native function



pub fn network_clear_clock_time_override_safe(
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xD972DF67326F966Eu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xD972DF67326F966Eu64;
        
        let result = invoke_raw!(
            hash,
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn network_clear_clock_time_override_raw(
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xD972DF67326F966Eu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xD972DF67326F966Eu64;

        invoke_raw_typed!(
            hash,
        )
    }
}

/// ## Parameters
*



pub fn network_is_player_a_participant_on_script_safe(
        
        
            player1: 
        , 
        
        
            script: 
        , 
        
        
            player2: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x1AD5B71586B94820u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x1AD5B71586B94820u64;
        
        let result = invoke_raw!(
            hash,
                player1, 
                script, 
                player2
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn network_is_player_a_participant_on_script_raw(
        player1: , 
        script: , 
        player2: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x1AD5B71586B94820u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x1AD5B71586B94820u64;

        invoke_raw_typed!(
            hash,
                player1, 
                script, 
                player2
        )
    }
}

/// ```
Loads up the map that is loaded when beeing in mission creator  
Player gets placed in a mix between online/offline mode  
p0 is always 2 in R* scripts.  
Appears to be patched in gtav b757 (game gets terminated) alonside with most other network natives to prevent online modding ~ghost30812  
```



pub fn network_session_host_single_player_safe(
        
        
            p0: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xC74C33FCA52856D5u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xC74C33FCA52856D5u64;
        
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
pub fn network_session_host_single_player_raw(
        p0: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xC74C33FCA52856D5u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xC74C33FCA52856D5u64;

        invoke_raw_typed!(
            hash,
                p0
        )
    }
}

/// ```
Returns whether the signed-in user has valid Rockstar Online Services (ROS) credentials.
```



pub fn network_has_valid_ros_credentials_safe(
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x85443FF4C328F53Bu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x85443FF4C328F53Bu64;
        
        let result = invoke_raw!(
            hash,
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn network_has_valid_ros_credentials_raw(
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x85443FF4C328F53Bu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x85443FF4C328F53Bu64;

        invoke_raw_typed!(
            hash,
        )
    }
}

/// ## Parameters
*



pub fn network_session_crew_matchmaking_safe(
        
        
            p0: 
        , 
        
        
            p1: 
        , 
        
        
            p2: 
        , 
        
        
            maxPlayers: 
        , 
        
        
            p4: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x94BC51E9449D917Fu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x94BC51E9449D917Fu64;
        
        let result = invoke_raw!(
            hash,
                p0, 
                p1, 
                p2, 
                maxPlayers, 
                p4
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn network_session_crew_matchmaking_raw(
        p0: , 
        p1: , 
        p2: , 
        maxPlayers: , 
        p4: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x94BC51E9449D917Fu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x94BC51E9449D917Fu64;

        invoke_raw_typed!(
            hash,
                p0, 
                p1, 
                p2, 
                maxPlayers, 
                p4
        )
    }
}

/// ## Parameters
*



pub fn _0xfb680d403909dc70_safe(
        
        
            p0: 
        , 
        
        
            p1: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xFB680D403909DC70u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xFB680D403909DC70u64;
        
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
pub fn _0xfb680d403909dc70_raw(
        p0: , 
        p1: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xFB680D403909DC70u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xFB680D403909DC70u64;

        invoke_raw_typed!(
            hash,
                p0, 
                p1
        )
    }
}

/// NETWORK_SESSION_CANCEL_INVITE native function



pub fn network_session_cancel_invite_safe(
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x2FBF47B1B36D36F9u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x2FBF47B1B36D36F9u64;
        
        let result = invoke_raw!(
            hash,
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn network_session_cancel_invite_raw(
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x2FBF47B1B36D36F9u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x2FBF47B1B36D36F9u64;

        invoke_raw_typed!(
            hash,
        )
    }
}

/// ## Parameters
*



pub fn _network_register_tunable_float_hash_safe(
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x1950DAE9848A4739u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x1950DAE9848A4739u64;
        
        let result = invoke_raw!(
            hash,
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn _network_register_tunable_float_hash_raw(
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x1950DAE9848A4739u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x1950DAE9848A4739u64;

        invoke_raw_typed!(
            hash,
        )
    }
}

/// NETWORK_CLEAR_FOLLOWERS native function



pub fn network_clear_followers_safe(
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x058F43EC59A8631Au32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x058F43EC59A8631Au64;
        
        let result = invoke_raw!(
            hash,
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn network_clear_followers_raw(
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x058F43EC59A8631Au32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x058F43EC59A8631Au64;

        invoke_raw_typed!(
            hash,
        )
    }
}

/// ## Return value



pub fn ugc_has_get_finished_safe(
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x02ADA21EA2F6918Fu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x02ADA21EA2F6918Fu64;
        
        let result = invoke_raw!(
            hash,
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn ugc_has_get_finished_raw(
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x02ADA21EA2F6918Fu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x02ADA21EA2F6918Fu64;

        invoke_raw_typed!(
            hash,
        )
    }
}

/// ## Return value



pub fn _network_is_psn_available_safe(
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x8D11E61A4ABF49CCu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x8D11E61A4ABF49CCu64;
        
        let result = invoke_raw!(
            hash,
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn _network_is_psn_available_raw(
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x8D11E61A4ABF49CCu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x8D11E61A4ABF49CCu64;

        invoke_raw_typed!(
            hash,
        )
    }
}

/// ```
NativeDB Introduced: v2189
```



pub fn _0x76b3f29d3f967692_safe(
        
        
            p0: 
        , 
        
        
            p1: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x76B3F29D3F967692u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x76B3F29D3F967692u64;
        
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
pub fn _0x76b3f29d3f967692_raw(
        p0: , 
        p1: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x76B3F29D3F967692u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x76B3F29D3F967692u64;

        invoke_raw_typed!(
            hash,
                p0, 
                p1
        )
    }
}

/// ## Return value



pub fn network_is_tutorial_session_change_pending_safe(
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x35F0B98A8387274Du32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x35F0B98A8387274Du64;
        
        let result = invoke_raw!(
            hash,
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn network_is_tutorial_session_change_pending_raw(
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x35F0B98A8387274Du32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x35F0B98A8387274Du64;

        invoke_raw_typed!(
            hash,
        )
    }
}

/// ## Return value
Hard-coded to always return 0.



pub fn _0x8b0c2964ba471961_safe(
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x8B0C2964BA471961u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x8B0C2964BA471961u64;
        
        let result = invoke_raw!(
            hash,
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn _0x8b0c2964ba471961_raw(
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x8B0C2964BA471961u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x8B0C2964BA471961u64;

        invoke_raw_typed!(
            hash,
        )
    }
}

/// p8, p9, p10 is another coordinate, or zero, often related to ``GET_BLIP_COORDS`` in the decompiled scripts.



pub fn network_start_respawn_search_in_angled_area_for_player_safe(
        
        
            player: 
        , 
        
        
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
        , 
        
        
            width: 
        , 
        
        
            p8: 
        , 
        
        
            p9: 
        , 
        
        
            p10: 
        , 
        
        
            flags: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x4BA92A18502BCA61u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x4BA92A18502BCA61u64;
        
        let result = invoke_raw!(
            hash,
                player, 
                x1, 
                y1, 
                z1, 
                x2, 
                y2, 
                z2, 
                width, 
                p8, 
                p9, 
                p10, 
                flags
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn network_start_respawn_search_in_angled_area_for_player_raw(
        player: , 
        x1: , 
        y1: , 
        z1: , 
        x2: , 
        y2: , 
        z2: , 
        width: , 
        p8: , 
        p9: , 
        p10: , 
        flags: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x4BA92A18502BCA61u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x4BA92A18502BCA61u64;

        invoke_raw_typed!(
            hash,
                player, 
                x1, 
                y1, 
                z1, 
                x2, 
                y2, 
                z2, 
                width, 
                p8, 
                p9, 
                p10, 
                flags
        )
    }
}

/// ## Parameters
*



pub fn _0x742b58f723233ed9_safe(
        
        
            p0: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x742B58F723233ED9u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x742B58F723233ED9u64;
        
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
pub fn _0x742b58f723233ed9_raw(
        p0: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x742B58F723233ED9u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x742B58F723233ED9u64;

        invoke_raw_typed!(
            hash,
                p0
        )
    }
}

/// _0xFA2888E3833C8E96 native function



pub fn _0xfa2888e3833c8e96_safe(
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xFA2888E3833C8E96u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xFA2888E3833C8E96u64;
        
        let result = invoke_raw!(
            hash,
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn _0xfa2888e3833c8e96_raw(
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xFA2888E3833C8E96u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xFA2888E3833C8E96u64;

        invoke_raw_typed!(
            hash,
        )
    }
}

/// ## Parameters
*



pub fn _0x1171a97a3d3981b6_safe(
        
        
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
        let hash = 0x1171A97A3D3981B6u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x1171A97A3D3981B6u64;
        
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
pub fn _0x1171a97a3d3981b6_raw(
        p0: , 
        p1: , 
        p2: , 
        p3: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x1171A97A3D3981B6u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x1171A97A3D3981B6u64;

        invoke_raw_typed!(
            hash,
                p0, 
                p1, 
                p2, 
                p3
        )
    }
}

/// ## Return value



pub fn _network_get_ros_privilege_25_safe(
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x91B87C55093DE351u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x91B87C55093DE351u64;
        
        let result = invoke_raw!(
            hash,
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn _network_get_ros_privilege_25_raw(
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x91B87C55093DE351u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x91B87C55093DE351u64;

        invoke_raw_typed!(
            hash,
        )
    }
}

/// ## Return value



pub fn network_get_max_friends_safe(
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xAFEBB0D5D8F687D2u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xAFEBB0D5D8F687D2u64;
        
        let result = invoke_raw!(
            hash,
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn network_get_max_friends_raw(
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xAFEBB0D5D8F687D2u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xAFEBB0D5D8F687D2u64;

        invoke_raw_typed!(
            hash,
        )
    }
}

/// ## Parameters
*



pub fn network_is_script_active_safe(
        
        
            scriptName: 
        , 
        
        
            player: 
        , 
        
        
            p2: 
        , 
        
        
            p3: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x9D40DF90FAD26098u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x9D40DF90FAD26098u64;
        
        let result = invoke_raw!(
            hash,
                scriptName, 
                player, 
                p2, 
                p3
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn network_is_script_active_raw(
        scriptName: , 
        player: , 
        p2: , 
        p3: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x9D40DF90FAD26098u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x9D40DF90FAD26098u64;

        invoke_raw_typed!(
            hash,
                scriptName, 
                player, 
                p2, 
                p3
        )
    }
}

/// ## Parameters
*



pub fn ugc_texture_download_request_safe(
        
        
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
        let hash = 0x308F96458B7087CCu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x308F96458B7087CCu64;
        
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
pub fn ugc_texture_download_request_raw(
        p0: , 
        p1: , 
        p2: , 
        p3: , 
        p4: , 
        p5: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x308F96458B7087CCu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x308F96458B7087CCu64;

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
Has something to do with a host request.

NETWORK_RE*
```



pub fn _0x741a3d8380319a81_safe(
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x741A3D8380319A81u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x741A3D8380319A81u64;
        
        let result = invoke_raw!(
            hash,
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn _0x741a3d8380319a81_raw(
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x741A3D8380319A81u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x741A3D8380319A81u64;

        invoke_raw_typed!(
            hash,
        )
    }
}

/// ## Parameters
*



pub fn network_get_presence_invite_content_id_safe(
        
        
            p0: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x24409FC4C55CB22Du32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x24409FC4C55CB22Du64;
        
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
pub fn network_get_presence_invite_content_id_raw(
        p0: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x24409FC4C55CB22Du32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x24409FC4C55CB22Du64;

        invoke_raw_typed!(
            hash,
                p0
        )
    }
}

/// UGC_CLEAR_MODIFY_RESULT native function



pub fn ugc_clear_modify_result_safe(
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xA1E5E0204A6FCC70u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xA1E5E0204A6FCC70u64;
        
        let result = invoke_raw!(
            hash,
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn ugc_clear_modify_result_raw(
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xA1E5E0204A6FCC70u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xA1E5E0204A6FCC70u64;

        invoke_raw_typed!(
            hash,
        )
    }
}

/// ```
NativeDB Introduced: v1734
```



pub fn _0x38b7c51ab1edc7d8_safe(
        
        
            entity: 
        , 
        
        
            toggle: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x38B7C51AB1EDC7D8u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x38B7C51AB1EDC7D8u64;
        
        let result = invoke_raw!(
            hash,
                entity, 
                toggle
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn _0x38b7c51ab1edc7d8_raw(
        entity: , 
        toggle: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x38B7C51AB1EDC7D8u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x38B7C51AB1EDC7D8u64;

        invoke_raw_typed!(
            hash,
                entity, 
                toggle
        )
    }
}

/// ## Parameters
*



pub fn network_get_participant_index_safe(
        
        
            index: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x1B84DF6AF2A46938u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x1B84DF6AF2A46938u64;
        
        let result = invoke_raw!(
            hash,
                index
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn network_get_participant_index_raw(
        index: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x1B84DF6AF2A46938u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x1B84DF6AF2A46938u64;

        invoke_raw_typed!(
            hash,
                index
        )
    }
}

/// ```
This function is hard-coded to always return 0.
```



pub fn network_is_pending_friend_safe(
        
        
            p0: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x0BE73DA6984A6E33u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x0BE73DA6984A6E33u64;
        
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
pub fn network_is_pending_friend_raw(
        p0: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x0BE73DA6984A6E33u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x0BE73DA6984A6E33u64;

        invoke_raw_typed!(
            hash,
                p0
        )
    }
}

/// ## Parameters
*



pub fn network_get_respawn_result_flags_safe(
        
        
            p0: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x6C34F1208B8923FDu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x6C34F1208B8923FDu64;
        
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
pub fn network_get_respawn_result_flags_raw(
        p0: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x6C34F1208B8923FDu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x6C34F1208B8923FDu64;

        invoke_raw_typed!(
            hash,
                p0
        )
    }
}

/// ```
Used in am_mp_property_ext and am_mp_property_int  
```

```
NativeDB Added Parameter 2: Ped ped
```



pub fn remove_all_sticky_bombs_from_entity_safe(
        
        
            entity: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x715135F4B82AC90Du32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x715135F4B82AC90Du64;
        
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
pub fn remove_all_sticky_bombs_from_entity_raw(
        entity: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x715135F4B82AC90Du32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x715135F4B82AC90Du64;

        invoke_raw_typed!(
            hash,
                entity
        )
    }
}

/// ```
Allow vehicle wheels to be destructible even when the Vehicle entity is invincible.
```

```
NativeDB Introduced: v1365
```



pub fn _network_set_vehicle_wheels_destructible_safe(
        
        
            vehicle: 
        , 
        
        
            toggle: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x890E2C5ABED7236Du32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x890E2C5ABED7236Du64;
        
        let result = invoke_raw!(
            hash,
                vehicle, 
                toggle
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn _network_set_vehicle_wheels_destructible_raw(
        vehicle: , 
        toggle: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x890E2C5ABED7236Du32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x890E2C5ABED7236Du64;

        invoke_raw_typed!(
            hash,
                vehicle, 
                toggle
        )
    }
}

/// ## Parameters
*



pub fn network_handle_from_friend_safe(
        
        
            friendIndex: 
        , 
        
        
            networkHandle: 
        , 
        
        
            bufferSize: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xD45CB817D7E177D2u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xD45CB817D7E177D2u64;
        
        let result = invoke_raw!(
            hash,
                friendIndex, 
                networkHandle, 
                bufferSize
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn network_handle_from_friend_raw(
        friendIndex: , 
        networkHandle: , 
        bufferSize: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xD45CB817D7E177D2u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xD45CB817D7E177D2u64;

        invoke_raw_typed!(
            hash,
                friendIndex, 
                networkHandle, 
                bufferSize
        )
    }
}

/// ## Return value



pub fn network_get_random_int_safe(
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x599E4FA1F87EB5FFu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x599E4FA1F87EB5FFu64;
        
        let result = invoke_raw!(
            hash,
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn network_get_random_int_raw(
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x599E4FA1F87EB5FFu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x599E4FA1F87EB5FFu64;

        invoke_raw_typed!(
            hash,
        )
    }
}

/// ## Parameters
*



pub fn network_set_gamer_invited_to_transition_safe(
        
        
            networkHandle: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xCA2C8073411ECDB6u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xCA2C8073411ECDB6u64;
        
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
pub fn network_set_gamer_invited_to_transition_raw(
        networkHandle: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xCA2C8073411ECDB6u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xCA2C8073411ECDB6u64;

        invoke_raw_typed!(
            hash,
                networkHandle
        )
    }
}

/// ```
_NETWORK_CAN_VIEW_* - _NETWORK_CAN_SESSION*

NETWORK_CAN_PLAY_GAMER_USER_CONTENT?
```

```
NativeDB Introduced: v2699
```



pub fn _0x559ebf901a8c68e0_safe(
        
        
            gamerHandle: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x559EBF901A8C68E0u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x559EBF901A8C68E0u64;
        
        let result = invoke_raw!(
            hash,
                gamerHandle
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn _0x559ebf901a8c68e0_raw(
        gamerHandle: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x559EBF901A8C68E0u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x559EBF901A8C68E0u64;

        invoke_raw_typed!(
            hash,
                gamerHandle
        )
    }
}

/// ## Return value



pub fn network_get_gamer_status_from_queue_safe(
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x2CC848A861D01493u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x2CC848A861D01493u64;
        
        let result = invoke_raw!(
            hash,
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn network_get_gamer_status_from_queue_raw(
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x2CC848A861D01493u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x2CC848A861D01493u64;

        invoke_raw_typed!(
            hash,
        )
    }
}

/// ## Parameters
*



pub fn network_add_friend_safe(
        
        
            networkHandle: 
        , 
        
        
            message: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x8E02D73914064223u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x8E02D73914064223u64;
        
        let result = invoke_raw!(
            hash,
                networkHandle, 
                message
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn network_add_friend_raw(
        networkHandle: , 
        message: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x8E02D73914064223u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x8E02D73914064223u64;

        invoke_raw_typed!(
            hash,
                networkHandle, 
                message
        )
    }
}

/// ## Parameters
*



pub fn network_block_join_queue_invites_safe(
        
        
            toggle: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xCFEB8AF24FC1D0BBu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xCFEB8AF24FC1D0BBu64;
        
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
pub fn network_block_join_queue_invites_raw(
        toggle: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xCFEB8AF24FC1D0BBu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xCFEB8AF24FC1D0BBu64;

        invoke_raw_typed!(
            hash,
                toggle
        )
    }
}

/// ```
Checks some commerce stuff

NativeDB Introduced: v1290
```



pub fn _0x155467aca0f55705_safe(
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x155467ACA0F55705u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x155467ACA0F55705u64;
        
        let result = invoke_raw!(
            hash,
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn _0x155467aca0f55705_raw(
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x155467ACA0F55705u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x155467ACA0F55705u64;

        invoke_raw_typed!(
            hash,
        )
    }
}

/// ## Parameters
*



pub fn _0xe16aa70ce9beedc3_safe(
        
        
            p0: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xE16AA70CE9BEEDC3u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xE16AA70CE9BEEDC3u64;
        
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
pub fn _0xe16aa70ce9beedc3_raw(
        p0: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xE16AA70CE9BEEDC3u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xE16AA70CE9BEEDC3u64;

        invoke_raw_typed!(
            hash,
                p0
        )
    }
}

/// ## Parameters
*



pub fn network_do_transition_to_new_game_safe(
        
        
            p0: 
        , 
        
        
            maxPlayers: 
        , 
        
        
            p2: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x4665F51EFED00034u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x4665F51EFED00034u64;
        
        let result = invoke_raw!(
            hash,
                p0, 
                maxPlayers, 
                p2
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn network_do_transition_to_new_game_raw(
        p0: , 
        maxPlayers: , 
        p2: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x4665F51EFED00034u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x4665F51EFED00034u64;

        invoke_raw_typed!(
            hash,
                p0, 
                maxPlayers, 
                p2
        )
    }
}

/// ```
Used by NetBlender
```



pub fn _network_get_last_velocity_received_safe(
        
        
            entity: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x33DE49EDF4DDE77Au32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x33DE49EDF4DDE77Au64;
        
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
pub fn _network_get_last_velocity_received_raw(
        entity: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x33DE49EDF4DDE77Au32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x33DE49EDF4DDE77Au64;

        invoke_raw_typed!(
            hash,
                entity
        )
    }
}

/// ## Parameters
*



pub fn network_get_transition_host_safe(
        
        
            networkHandle: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x65042B9774C4435Eu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x65042B9774C4435Eu64;
        
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
pub fn network_get_transition_host_raw(
        networkHandle: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x65042B9774C4435Eu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x65042B9774C4435Eu64;

        invoke_raw_typed!(
            hash,
                networkHandle
        )
    }
}

/// ```
IS_COMMERCE_*
```



pub fn _0x1d4dc17c38feaff0_safe(
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x1D4DC17C38FEAFF0u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x1D4DC17C38FEAFF0u64;
        
        let result = invoke_raw!(
            hash,
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn _0x1d4dc17c38feaff0_raw(
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x1D4DC17C38FEAFF0u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x1D4DC17C38FEAFF0u64;

        invoke_raw_typed!(
            hash,
        )
    }
}

/// ## Return value



pub fn network_action_follow_invite_safe(
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xC88156EBB786F8D5u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xC88156EBB786F8D5u64;
        
        let result = invoke_raw!(
            hash,
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn network_action_follow_invite_raw(
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xC88156EBB786F8D5u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xC88156EBB786F8D5u64;

        invoke_raw_typed!(
            hash,
        )
    }
}

/// ## Parameters
*



pub fn texture_download_has_failed_safe(
        
        
            p0: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x5776ED562C134687u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x5776ED562C134687u64;
        
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
pub fn texture_download_has_failed_raw(
        p0: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x5776ED562C134687u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x5776ED562C134687u64;

        invoke_raw_typed!(
            hash,
                p0
        )
    }
}

/// ## Parameters
*



pub fn _0xcfeb8af24fc1d0bb_safe(
        
        
            p0: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xCFEB8AF24FC1D0BBu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xCFEB8AF24FC1D0BBu64;
        
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
pub fn _0xcfeb8af24fc1d0bb_raw(
        p0: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xCFEB8AF24FC1D0BBu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xCFEB8AF24FC1D0BBu64;

        invoke_raw_typed!(
            hash,
                p0
        )
    }
}

/// ## Parameters
*



pub fn reserve_network_mission_objects_safe(
        
        
            amount: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x4E5C93BD0C32FBF8u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x4E5C93BD0C32FBF8u64;
        
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
pub fn reserve_network_mission_objects_raw(
        amount: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x4E5C93BD0C32FBF8u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x4E5C93BD0C32FBF8u64;

        invoke_raw_typed!(
            hash,
                amount
        )
    }
}

/// ## Parameters
*



pub fn _network_can_view_gamer_user_content_safe(
        
        
            networkHandle: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xB57A49545BA53CE7u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xB57A49545BA53CE7u64;
        
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
pub fn _network_can_view_gamer_user_content_raw(
        networkHandle: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xB57A49545BA53CE7u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xB57A49545BA53CE7u64;

        invoke_raw_typed!(
            hash,
                networkHandle
        )
    }
}

/// ## Parameters
*



pub fn network_are_handles_the_same_safe(
        
        
            netHandle1: 
        , 
        
        
            netHandle2: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x57DBA049E110F217u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x57DBA049E110F217u64;
        
        let result = invoke_raw!(
            hash,
                netHandle1, 
                netHandle2
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn network_are_handles_the_same_raw(
        netHandle1: , 
        netHandle2: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x57DBA049E110F217u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x57DBA049E110F217u64;

        invoke_raw_typed!(
            hash,
                netHandle1, 
                netHandle2
        )
    }
}

/// ## Parameters
*



pub fn network_remove_presence_invite_safe(
        
        
            p0: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xF0210268DB0974B1u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xF0210268DB0974B1u64;
        
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
pub fn network_remove_presence_invite_raw(
        p0: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xF0210268DB0974B1u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xF0210268DB0974B1u64;

        invoke_raw_typed!(
            hash,
                p0
        )
    }
}

/// ```
NativeDB Introduced: v2060
```



pub fn _0xb9351a07a0d458b1_safe(
        
        
            p0: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xB9351A07A0D458B1u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xB9351A07A0D458B1u64;
        
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
pub fn _0xb9351a07a0d458b1_raw(
        p0: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xB9351A07A0D458B1u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xB9351A07A0D458B1u64;

        invoke_raw_typed!(
            hash,
                p0
        )
    }
}

/// UGC_CLEAR_OFFLINE_QUERY native function



pub fn ugc_clear_offline_query_safe(
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x61A885D3F7CFEE9Au32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x61A885D3F7CFEE9Au64;
        
        let result = invoke_raw!(
            hash,
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn ugc_clear_offline_query_raw(
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x61A885D3F7CFEE9Au32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x61A885D3F7CFEE9Au64;

        invoke_raw_typed!(
            hash,
        )
    }
}

/// ## Return value



pub fn network_player_is_cheater_safe(
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x655B91F1495A9090u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x655B91F1495A9090u64;
        
        let result = invoke_raw!(
            hash,
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn network_player_is_cheater_raw(
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x655B91F1495A9090u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x655B91F1495A9090u64;

        invoke_raw_typed!(
            hash,
        )
    }
}

/// ## Return value



pub fn ugc_get_create_result_safe(
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xFBC5E768C7A77A6Au32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xFBC5E768C7A77A6Au64;
        
        let result = invoke_raw!(
            hash,
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn ugc_get_create_result_raw(
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xFBC5E768C7A77A6Au32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xFBC5E768C7A77A6Au64;

        invoke_raw_typed!(
            hash,
        )
    }
}

/// ## Return value



pub fn network_can_session_end_safe(
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x4EEBC3694E49C572u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x4EEBC3694E49C572u64;
        
        let result = invoke_raw!(
            hash,
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn network_can_session_end_raw(
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x4EEBC3694E49C572u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x4EEBC3694E49C572u64;

        invoke_raw_typed!(
            hash,
        )
    }
}

/// ## Parameters
*



pub fn network_get_found_gamer_safe(
        
        
            p0: 
        , 
        
        
            p1: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x9DCFF2AFB68B3476u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x9DCFF2AFB68B3476u64;
        
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
pub fn network_get_found_gamer_raw(
        p0: , 
        p1: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x9DCFF2AFB68B3476u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x9DCFF2AFB68B3476u64;

        invoke_raw_typed!(
            hash,
                p0, 
                p1
        )
    }
}

/// UGC_CANCEL_QUERY native function



pub fn ugc_cancel_query_safe(
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xE9B99B6853181409u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xE9B99B6853181409u64;
        
        let result = invoke_raw!(
            hash,
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn ugc_cancel_query_raw(
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xE9B99B6853181409u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xE9B99B6853181409u64;

        invoke_raw_typed!(
            hash,
        )
    }
}

/// ## Return value



pub fn network_did_find_gamers_succeed_safe(
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xF9B83B77929D8863u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xF9B83B77929D8863u64;
        
        let result = invoke_raw!(
            hash,
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn network_did_find_gamers_succeed_raw(
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xF9B83B77929D8863u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xF9B83B77929D8863u64;

        invoke_raw_typed!(
            hash,
        )
    }
}

/// ```
if set to true other network players can't see it  
if set to false other network player can see it  
=========================================  
^^ I attempted this by grabbing an object with GET_ENTITY_PLAYER_IS_FREE_AIMING_AT and setting this naive no matter the toggle he could still see it.  
pc or last gen?  
^^ last-gen  
```



pub fn _network_set_entity_invisible_to_network_safe(
        
        
            entity: 
        , 
        
        
            toggle: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xF1CA12B18AEF5298u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xF1CA12B18AEF5298u64;
        
        let result = invoke_raw!(
            hash,
                entity, 
                toggle
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn _network_set_entity_invisible_to_network_raw(
        entity: , 
        toggle: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xF1CA12B18AEF5298u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xF1CA12B18AEF5298u64;

        invoke_raw_typed!(
            hash,
                entity, 
                toggle
        )
    }
}

/// NETWORK_CLEAR_VOICE_CHANNEL native function



pub fn network_clear_voice_channel_safe(
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xE036A705F989E049u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xE036A705F989E049u64;
        
        let result = invoke_raw!(
            hash,
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn network_clear_voice_channel_raw(
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xE036A705F989E049u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xE036A705F989E049u64;

        invoke_raw_typed!(
            hash,
        )
    }
}

/// ## Parameters
*



pub fn network_is_player_fading_safe(
        
        
            player: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x631DC5DFF4B110E3u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x631DC5DFF4B110E3u64;
        
        let result = invoke_raw!(
            hash,
                player
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn network_is_player_fading_raw(
        player: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x631DC5DFF4B110E3u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x631DC5DFF4B110E3u64;

        invoke_raw_typed!(
            hash,
                player
        )
    }
}

/// ## Parameters
*



pub fn network_clan_remote_memberships_are_in_cache_safe(
        
        
            p0: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xBB6E6FEE99D866B2u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xBB6E6FEE99D866B2u64;
        
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
pub fn network_clan_remote_memberships_are_in_cache_raw(
        p0: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xBB6E6FEE99D866B2u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xBB6E6FEE99D866B2u64;

        invoke_raw_typed!(
            hash,
                p0
        )
    }
}

/// ## Parameters
*



pub fn _0x3855fb5eb2c5e8b2_safe(
        
        
            p0: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x3855FB5EB2C5E8B2u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x3855FB5EB2C5E8B2u64;
        
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
pub fn _0x3855fb5eb2c5e8b2_raw(
        p0: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x3855FB5EB2C5E8B2u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x3855FB5EB2C5E8B2u64;

        invoke_raw_typed!(
            hash,
                p0
        )
    }
}

/// ```
Lets objects spawn online simply do it like this:  
int createdObject = OBJ_TO_NET(CREATE_OBJECT_NO_OFFSET(oball, pCoords.x, pCoords.y, pCoords.z, 1, 0, 0));  
```



pub fn obj_to_net_safe(
        
        
            object: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x99BFDC94A603E541u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x99BFDC94A603E541u64;
        
        let result = invoke_raw!(
            hash,
                object
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn obj_to_net_raw(
        object: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x99BFDC94A603E541u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x99BFDC94A603E541u64;

        invoke_raw_typed!(
            hash,
                object
        )
    }
}

/// ## Parameters
*



pub fn network_is_activity_spectator_from_handle_safe(
        
        
            networkHandle: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x2763BBAA72A7BCB9u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x2763BBAA72A7BCB9u64;
        
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
pub fn network_is_activity_spectator_from_handle_raw(
        networkHandle: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x2763BBAA72A7BCB9u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x2763BBAA72A7BCB9u64;

        invoke_raw_typed!(
            hash,
                networkHandle
        )
    }
}

/// ## Return value



pub fn network_session_is_closed_friends_safe(
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xFBCFA2EA2E206890u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xFBCFA2EA2E206890u64;
        
        let result = invoke_raw!(
            hash,
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn network_session_is_closed_friends_raw(
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xFBCFA2EA2E206890u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xFBCFA2EA2E206890u64;

        invoke_raw_typed!(
            hash,
        )
    }
}

/// ## Parameters
*



pub fn request_commerce_item_image_safe(
        
        
            index: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xA2F952104FC6DD4Bu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xA2F952104FC6DD4Bu64;
        
        let result = invoke_raw!(
            hash,
                index
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn request_commerce_item_image_raw(
        index: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xA2F952104FC6DD4Bu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xA2F952104FC6DD4Bu64;

        invoke_raw_typed!(
            hash,
                index
        )
    }
}

/// NETWORK_CLOSE_TRANSITION_MATCHMAKING native function



pub fn network_close_transition_matchmaking_safe(
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x43F4DBA69710E01Eu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x43F4DBA69710E01Eu64;
        
        let result = invoke_raw!(
            hash,
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn network_close_transition_matchmaking_raw(
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x43F4DBA69710E01Eu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x43F4DBA69710E01Eu64;

        invoke_raw_typed!(
            hash,
        )
    }
}

/// ## Parameters
*



pub fn network_has_entity_been_registered_with_this_thread_safe(
        
        
            entity: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xB07D3185E11657A5u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xB07D3185E11657A5u64;
        
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
pub fn network_has_entity_been_registered_with_this_thread_raw(
        entity: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xB07D3185E11657A5u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xB07D3185E11657A5u64;

        invoke_raw_typed!(
            hash,
                entity
        )
    }
}

/// ## Return value



pub fn network_clear_follow_invite_safe(
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x439BFDE3CD0610F6u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x439BFDE3CD0610F6u64;
        
        let result = invoke_raw!(
            hash,
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn network_clear_follow_invite_raw(
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x439BFDE3CD0610F6u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x439BFDE3CD0610F6u64;

        invoke_raw_typed!(
            hash,
        )
    }
}

/// UGC_CLEAR_CREATE_RESULT native function



pub fn ugc_clear_create_result_safe(
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x17440AA15D1D3739u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x17440AA15D1D3739u64;
        
        let result = invoke_raw!(
            hash,
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn ugc_clear_create_result_raw(
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x17440AA15D1D3739u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x17440AA15D1D3739u64;

        invoke_raw_typed!(
            hash,
        )
    }
}

/// ```
p0 appears to be for MP  
```

```
NativeDB Added Parameter 2: Any p1
```



pub fn get_num_reserved_mission_objects_safe(
        
        
            p0: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xAA81B5F10BC43AC2u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xAA81B5F10BC43AC2u64;
        
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
pub fn get_num_reserved_mission_objects_raw(
        p0: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xAA81B5F10BC43AC2u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xAA81B5F10BC43AC2u64;

        invoke_raw_typed!(
            hash,
                p0
        )
    }
}

/// ## Parameters
*



pub fn network_remove_entity_area_safe(
        
        
            p0: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x93CF869BAA0C4874u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x93CF869BAA0C4874u64;
        
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
pub fn network_remove_entity_area_raw(
        p0: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x93CF869BAA0C4874u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x93CF869BAA0C4874u64;

        invoke_raw_typed!(
            hash,
                p0
        )
    }
}

/// ## Parameters
*



pub fn network_explode_heli_safe(
        
        
            heli: 
        , 
        
        
            isAudible: 
        , 
        
        
            isInvisible: 
        , 
        
        
            netScriptEntityId: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x2A5E0621DD815A9Au32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x2A5E0621DD815A9Au64;
        
        let result = invoke_raw!(
            hash,
                heli, 
                isAudible, 
                isInvisible, 
                netScriptEntityId
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn network_explode_heli_raw(
        heli: , 
        isAudible: , 
        isInvisible: , 
        netScriptEntityId: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x2A5E0621DD815A9Au32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x2A5E0621DD815A9Au64;

        invoke_raw_typed!(
            hash,
                heli, 
                isAudible, 
                isInvisible, 
                netScriptEntityId
        )
    }
}

/// Adds an entity to a network synchronised scene.



pub fn network_add_entity_to_synchronised_scene_safe(
        
        
            entity: 
        , 
        
        
            netScene: 
        , 
        
        
            animDict: 
        , 
        
        
            animName: 
        , 
        
        
            blendIn: 
        , 
        
        
            blendOut: 
        , 
        
        
            flag: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xF2404D68CBC855FAu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xF2404D68CBC855FAu64;
        
        let result = invoke_raw!(
            hash,
                entity, 
                netScene, 
                animDict, 
                animName, 
                blendIn, 
                blendOut, 
                flag
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn network_add_entity_to_synchronised_scene_raw(
        entity: , 
        netScene: , 
        animDict: , 
        animName: , 
        blendIn: , 
        blendOut: , 
        flag: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xF2404D68CBC855FAu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xF2404D68CBC855FAu64;

        invoke_raw_typed!(
            hash,
                entity, 
                netScene, 
                animDict, 
                animName, 
                blendIn, 
                blendOut, 
                flag
        )
    }
}

/// ```
0 = succeeded
1 = pending
2 = failed
```



pub fn get_status_of_texture_download_safe(
        
        
            p0: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x8BD6C6DEA20E82C6u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x8BD6C6DEA20E82C6u64;
        
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
pub fn get_status_of_texture_download_raw(
        p0: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x8BD6C6DEA20E82C6u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x8BD6C6DEA20E82C6u64;

        invoke_raw_typed!(
            hash,
                p0
        )
    }
}

/// ## Return value



pub fn get_max_num_network_objects_safe(
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xC7BE335216B5EC7Cu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xC7BE335216B5EC7Cu64;
        
        let result = invoke_raw!(
            hash,
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn get_max_num_network_objects_raw(
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xC7BE335216B5EC7Cu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xC7BE335216B5EC7Cu64;

        invoke_raw_typed!(
            hash,
        )
    }
}

/// ```
Return the local Participant ID  
```



pub fn participant_id_safe(
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x90986E8876CE0A83u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x90986E8876CE0A83u64;
        
        let result = invoke_raw!(
            hash,
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn participant_id_raw(
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x90986E8876CE0A83u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x90986E8876CE0A83u64;

        invoke_raw_typed!(
            hash,
        )
    }
}

/// ## Parameters
*



pub fn network_check_communication_privileges_safe(
        
        
            p0: 
        , 
        
        
            p1: 
        , 
        
        
            p2: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x83F28CE49FBBFFBAu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x83F28CE49FBBFFBAu64;
        
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
pub fn network_check_communication_privileges_raw(
        p0: , 
        p1: , 
        p2: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x83F28CE49FBBFFBAu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x83F28CE49FBBFFBAu64;

        invoke_raw_typed!(
            hash,
                p0, 
                p1, 
                p2
        )
    }
}

/// ```
NativeDB Introduced: v1604
```



pub fn _0x560b423d73015e77_safe(
        
        
            p0: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x560B423D73015E77u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x560B423D73015E77u64;
        
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
pub fn _0x560b423d73015e77_raw(
        p0: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x560B423D73015E77u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x560B423D73015E77u64;

        invoke_raw_typed!(
            hash,
                p0
        )
    }
}

/// ```
rage::netBlenderLinInterp::GetPositionMaxForUpdateLevel
```



pub fn _set_network_vehicle_position_update_multiplier_safe(
        
        
            vehicle: 
        , 
        
        
            multiplier: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xA2A707979FE754DCu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xA2A707979FE754DCu64;
        
        let result = invoke_raw!(
            hash,
                vehicle, 
                multiplier
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn _set_network_vehicle_position_update_multiplier_raw(
        vehicle: , 
        multiplier: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xA2A707979FE754DCu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xA2A707979FE754DCu64;

        invoke_raw_typed!(
            hash,
                vehicle, 
                multiplier
        )
    }
}

/// ## Parameters
*



pub fn network_clan_request_emblem_safe(
        
        
            p0: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x13518FF1C6B28938u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x13518FF1C6B28938u64;
        
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
pub fn network_clan_request_emblem_raw(
        p0: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x13518FF1C6B28938u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x13518FF1C6B28938u64;

        invoke_raw_typed!(
            hash,
                p0
        )
    }
}

/// ```
Hardcoded to return false.
```



pub fn network_is_in_platform_party_safe(
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x2FC5650B0271CB57u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x2FC5650B0271CB57u64;
        
        let result = invoke_raw!(
            hash,
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn network_is_in_platform_party_raw(
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x2FC5650B0271CB57u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x2FC5650B0271CB57u64;

        invoke_raw_typed!(
            hash,
        )
    }
}

/// ## Return value



pub fn network_is_local_player_invincible_safe(
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x8A8694B48715B000u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x8A8694B48715B000u64;
        
        let result = invoke_raw!(
            hash,
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn network_is_local_player_invincible_raw(
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x8A8694B48715B000u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x8A8694B48715B000u64;

        invoke_raw_typed!(
            hash,
        )
    }
}

/// ```
Only one occurence in the scripts:
auto sub_cb43(auto a_0, auto a_1) {
    if (g_2594CB._f1) {
        if (NETWORK::_855BC38818F6F684()) {
            NETWORK::_ABD5E88B8A2D3DB2(&a_0._fB93);
            g_2594CB._f14/*{13}*/ = a_0._fB93;
            g_2594CB._f4/*"64"*/ = a_1;
            return 1;
        }
    }
    return 0;
}
other:
looks like it passes a player in the paramater
Contains string "NETWORK_VOICE_CONNECT_TO_PLAYER" in ida
```



pub fn network_session_voice_connect_to_player_safe(
        
        
            globalPtr: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xABD5E88B8A2D3DB2u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xABD5E88B8A2D3DB2u64;
        
        let result = invoke_raw!(
            hash,
                globalPtr
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn network_session_voice_connect_to_player_raw(
        globalPtr: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xABD5E88B8A2D3DB2u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xABD5E88B8A2D3DB2u64;

        invoke_raw_typed!(
            hash,
                globalPtr
        )
    }
}

/// Fade the given entity back in, usually used after the entity has been faded out with [NETWORK_FADE_OUT_ENTITY](#_0xDE564951F95E09ED)

When used on a entity which isn't invisible or faded out then the native will still work, it will just instanly make the ped invisible before fading in.



pub fn network_fade_in_entity_safe(
        
        
            entity: 
        , 
        
        
            bNetwork: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x1F4ED342ACEFE62Du32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x1F4ED342ACEFE62Du64;
        
        let result = invoke_raw!(
            hash,
                entity, 
                bNetwork
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn network_fade_in_entity_raw(
        entity: , 
        bNetwork: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x1F4ED342ACEFE62Du32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x1F4ED342ACEFE62Du64;

        invoke_raw_typed!(
            hash,
                entity, 
                bNetwork
        )
    }
}

/// ## Parameters
*



pub fn _0x7d395ea61622e116_safe(
        
        
            p0: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x7D395EA61622E116u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x7D395EA61622E116u64;
        
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
pub fn _0x7d395ea61622e116_raw(
        p0: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x7D395EA61622E116u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x7D395EA61622E116u64;

        invoke_raw_typed!(
            hash,
                p0
        )
    }
}

/// ## Return value



pub fn network_is_transition_busy_safe(
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x520F3282A53D26B7u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x520F3282A53D26B7u64;
        
        let result = invoke_raw!(
            hash,
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn network_is_transition_busy_raw(
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x520F3282A53D26B7u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x520F3282A53D26B7u64;

        invoke_raw_typed!(
            hash,
        )
    }
}

/// ## Parameters
*



pub fn network_query_respawn_results_safe(
        
        
            p0: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x3C891A251567DFCEu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x3C891A251567DFCEu64;
        
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
pub fn network_query_respawn_results_raw(
        p0: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x3C891A251567DFCEu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x3C891A251567DFCEu64;

        invoke_raw_typed!(
            hash,
                p0
        )
    }
}

/// ## Parameters
*



pub fn ugc_get_content_is_published_safe(
        
        
            p0: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x3054F114121C21EAu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x3054F114121C21EAu64;
        
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
pub fn ugc_get_content_is_published_raw(
        p0: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x3054F114121C21EAu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x3054F114121C21EAu64;

        invoke_raw_typed!(
            hash,
                p0
        )
    }
}

/// ```
NativeDB Introduced: v2060
```



pub fn _0x4c9034162368e206_safe(
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x4C9034162368E206u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x4C9034162368E206u64;
        
        let result = invoke_raw!(
            hash,
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn _0x4c9034162368e206_raw(
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x4C9034162368E206u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x4C9034162368E206u64;

        invoke_raw_typed!(
            hash,
        )
    }
}

/// ## Return value



pub fn _0xf287f506767cc8a9_safe(
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xF287F506767CC8A9u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xF287F506767CC8A9u64;
        
        let result = invoke_raw!(
            hash,
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn _0xf287f506767cc8a9_raw(
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xF287F506767CC8A9u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xF287F506767CC8A9u64;

        invoke_raw_typed!(
            hash,
        )
    }
}

/// _0x6CE50E47F5543D0C native function



pub fn _0x6ce50e47f5543d0c_safe(
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x6CE50E47F5543D0Cu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x6CE50E47F5543D0Cu64;
        
        let result = invoke_raw!(
            hash,
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn _0x6ce50e47f5543d0c_raw(
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x6CE50E47F5543D0Cu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x6CE50E47F5543D0Cu64;

        invoke_raw_typed!(
            hash,
        )
    }
}

/// ## Return value
Hard-coded to always return 1.



pub fn _0xbd545d44cce70597_safe(
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xBD545D44CCE70597u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xBD545D44CCE70597u64;
        
        let result = invoke_raw!(
            hash,
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn _0xbd545d44cce70597_raw(
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xBD545D44CCE70597u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xBD545D44CCE70597u64;

        invoke_raw_typed!(
            hash,
        )
    }
}

/// ## Return value



pub fn network_is_transition_started_safe(
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x53FA83401D9C07FEu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x53FA83401D9C07FEu64;
        
        let result = invoke_raw!(
            hash,
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn network_is_transition_started_raw(
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x53FA83401D9C07FEu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x53FA83401D9C07FEu64;

        invoke_raw_typed!(
            hash,
        )
    }
}

/// ## Parameters
*



pub fn ugc_get_content_path_safe(
        
        
            p0: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xBAF6BABF9E7CCC13u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xBAF6BABF9E7CCC13u64;
        
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
pub fn ugc_get_content_path_raw(
        p0: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xBAF6BABF9E7CCC13u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xBAF6BABF9E7CCC13u64;

        invoke_raw_typed!(
            hash,
                p0
        )
    }
}

/// ## Parameters
*



pub fn _0x5e3aa4ca2b6fb0ee_safe(
        
        
            p0: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x5E3AA4CA2B6FB0EEu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x5E3AA4CA2B6FB0EEu64;
        
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
pub fn _0x5e3aa4ca2b6fb0ee_raw(
        p0: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x5E3AA4CA2B6FB0EEu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x5E3AA4CA2B6FB0EEu64;

        invoke_raw_typed!(
            hash,
                p0
        )
    }
}

/// ## Return value



pub fn _0x793ff272d5b365f4_safe(
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x793FF272D5B365F4u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x793FF272D5B365F4u64;
        
        let result = invoke_raw!(
            hash,
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn _0x793ff272d5b365f4_raw(
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x793FF272D5B365F4u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x793FF272D5B365F4u64;

        invoke_raw_typed!(
            hash,
        )
    }
}

/// ```
NETWORK_GET_*
```



pub fn _0x64d779659bc37b19_safe(
        
        
            entity: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x64D779659BC37B19u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x64D779659BC37B19u64;
        
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
pub fn _0x64d779659bc37b19_raw(
        entity: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x64D779659BC37B19u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x64D779659BC37B19u64;

        invoke_raw_typed!(
            hash,
                entity
        )
    }
}

/// ## Return value



pub fn _0xc87e740d9f3872cc_safe(
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xC87E740D9F3872CCu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xC87E740D9F3872CCu64;
        
        let result = invoke_raw!(
            hash,
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn _0xc87e740d9f3872cc_raw(
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xC87E740D9F3872CCu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xC87E740D9F3872CCu64;

        invoke_raw_typed!(
            hash,
        )
    }
}

/// **This native does absolutely nothing, just a nullsub**



pub fn _0x4a9fde3a5a6d0437_safe(
        
        
            toggle: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x4A9FDE3A5A6D0437u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x4A9FDE3A5A6D0437u64;
        
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
pub fn _0x4a9fde3a5a6d0437_raw(
        toggle: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x4A9FDE3A5A6D0437u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x4A9FDE3A5A6D0437u64;

        invoke_raw_typed!(
            hash,
                toggle
        )
    }
}

/// ## Parameters
*



pub fn _0x2b1c623823db0d9d_safe(
        
        
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
        let hash = 0x2B1C623823DB0D9Du32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x2B1C623823DB0D9Du64;
        
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
pub fn _0x2b1c623823db0d9d_raw(
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
        let hash = 0x2B1C623823DB0D9Du32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x2B1C623823DB0D9Du64;

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



pub fn network_am_i_muted_by_gamer_safe(
        
        
            networkHandle: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xDF02A2C93F1F26DAu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xDF02A2C93F1F26DAu64;
        
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
pub fn network_am_i_muted_by_gamer_raw(
        networkHandle: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xDF02A2C93F1F26DAu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xDF02A2C93F1F26DAu64;

        invoke_raw_typed!(
            hash,
                networkHandle
        )
    }
}

/// ## Parameters
*



pub fn network_is_friend_online_safe(
        
        
            name: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x425A44533437B64Du32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x425A44533437B64Du64;
        
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
pub fn network_is_friend_online_raw(
        name: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x425A44533437B64Du32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x425A44533437B64Du64;

        invoke_raw_typed!(
            hash,
                name
        )
    }
}

/// ## Return value



pub fn ugc_get_modify_result_safe(
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x5A0A3D1A186A5508u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x5A0A3D1A186A5508u64;
        
        let result = invoke_raw!(
            hash,
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn ugc_get_modify_result_raw(
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x5A0A3D1A186A5508u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x5A0A3D1A186A5508u64;

        invoke_raw_typed!(
            hash,
        )
    }
}

/// ```
Could possibly bypass being muted or automatically muted  
```



pub fn network_override_chat_restrictions_safe(
        
        
            player: 
        , 
        
        
            toggle: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x3039AE5AD2C9C0C4u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x3039AE5AD2C9C0C4u64;
        
        let result = invoke_raw!(
            hash,
                player, 
                toggle
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn network_override_chat_restrictions_raw(
        player: , 
        toggle: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x3039AE5AD2C9C0C4u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x3039AE5AD2C9C0C4u64;

        invoke_raw_typed!(
            hash,
                player, 
                toggle
        )
    }
}

/// ```
p0 appears to be for MP  
```

```
NativeDB Added Parameter 2: Any p1
```



pub fn get_num_reserved_mission_peds_safe(
        
        
            p0: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x1F13D5AE5CB17E17u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x1F13D5AE5CB17E17u64;
        
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
pub fn get_num_reserved_mission_peds_raw(
        p0: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x1F13D5AE5CB17E17u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x1F13D5AE5CB17E17u64;

        invoke_raw_typed!(
            hash,
                p0
        )
    }
}

/// ## Parameters
*



pub fn network_has_cached_player_head_blend_data_safe(
        
        
            player: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x237D5336A9A54108u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x237D5336A9A54108u64;
        
        let result = invoke_raw!(
            hash,
                player
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn network_has_cached_player_head_blend_data_raw(
        player: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x237D5336A9A54108u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x237D5336A9A54108u64;

        invoke_raw_typed!(
            hash,
                player
        )
    }
}

/// _0x283B6062A2C01E9B native function



pub fn _0x283b6062a2c01e9b_safe(
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x283B6062A2C01E9Bu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x283B6062A2C01E9Bu64;
        
        let result = invoke_raw!(
            hash,
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn _0x283b6062a2c01e9b_raw(
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x283B6062A2C01E9Bu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x283B6062A2C01E9Bu64;

        invoke_raw_typed!(
            hash,
        )
    }
}

/// ## Return value



pub fn network_has_headset_safe(
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xE870F9F1F7B4F1FAu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xE870F9F1F7B4F1FAu64;
        
        let result = invoke_raw!(
            hash,
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn network_has_headset_raw(
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xE870F9F1F7B4F1FAu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xE870F9F1F7B4F1FAu64;

        invoke_raw_typed!(
            hash,
        )
    }
}

/// ```
Example:  
int playerHandle;	  
NETWORK_HANDLE_FROM_PLAYER(selectedPlayer, &playerHandle, 13);  
NETWORK_SHOW_PROFILE_UI(&playerHandle);  
```



pub fn network_show_profile_ui_safe(
        
        
            networkHandle: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x859ED1CEA343FCA8u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x859ED1CEA343FCA8u64;
        
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
pub fn network_show_profile_ui_raw(
        networkHandle: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x859ED1CEA343FCA8u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x859ED1CEA343FCA8u64;

        invoke_raw_typed!(
            hash,
                networkHandle
        )
    }
}

/// ## Parameters
*



pub fn _network_can_play_multiplayer_with_gamer_safe(
        
        
            networkHandle: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x07DD29D5E22763F1u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x07DD29D5E22763F1u64;
        
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
pub fn _network_can_play_multiplayer_with_gamer_raw(
        networkHandle: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x07DD29D5E22763F1u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x07DD29D5E22763F1u64;

        invoke_raw_typed!(
            hash,
                networkHandle
        )
    }
}

/// ```
Returns defaultValue if the tunable doesn't exist.
```



pub fn network_try_access_tunable_bool_hash_safe(
        
        
            tunableContext: 
        , 
        
        
            tunableName: 
        , 
        
        
            defaultValue: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xC7420099936CE286u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xC7420099936CE286u64;
        
        let result = invoke_raw!(
            hash,
                tunableContext, 
                tunableName, 
                defaultValue
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn network_try_access_tunable_bool_hash_raw(
        tunableContext: , 
        tunableName: , 
        defaultValue: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xC7420099936CE286u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xC7420099936CE286u64;

        invoke_raw_typed!(
            hash,
                tunableContext, 
                tunableName, 
                defaultValue
        )
    }
}

/// NETWORK_GET_PRIMARY_CLAN_DATA_CANCEL native function



pub fn network_get_primary_clan_data_cancel_safe(
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x042E4B70B93E6054u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x042E4B70B93E6054u64;
        
        let result = invoke_raw!(
            hash,
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn network_get_primary_clan_data_cancel_raw(
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x042E4B70B93E6054u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x042E4B70B93E6054u64;

        invoke_raw_typed!(
            hash,
        )
    }
}

/// ## Parameters
*



pub fn _network_get_destroyer_of_entity_safe(
        
        
            p0: 
        , 
        
        
            p1: 
        , 
        
        
            weaponHash: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x4CACA84440FA26F6u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x4CACA84440FA26F6u64;
        
        let result = invoke_raw!(
            hash,
                p0, 
                p1, 
                weaponHash
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn _network_get_destroyer_of_entity_raw(
        p0: , 
        p1: , 
        weaponHash: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x4CACA84440FA26F6u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x4CACA84440FA26F6u64;

        invoke_raw_typed!(
            hash,
                p0, 
                p1, 
                weaponHash
        )
    }
}

/// ## Return value



pub fn network_launch_transition_safe(
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x2DCF46CB1A4F0884u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x2DCF46CB1A4F0884u64;
        
        let result = invoke_raw!(
            hash,
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn network_launch_transition_raw(
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x2DCF46CB1A4F0884u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x2DCF46CB1A4F0884u64;

        invoke_raw_typed!(
            hash,
        )
    }
}

/// ## Parameters
*



pub fn network_gamer_has_headset_safe(
        
        
            networkHandle: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xF2FD55CB574BCC55u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xF2FD55CB574BCC55u64;
        
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
pub fn network_gamer_has_headset_raw(
        networkHandle: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xF2FD55CB574BCC55u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xF2FD55CB574BCC55u64;

        invoke_raw_typed!(
            hash,
                networkHandle
        )
    }
}

/// ## Parameters
*



pub fn get_num_created_mission_peds_safe(
        
        
            p0: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xCB215C4B56A7FAE7u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xCB215C4B56A7FAE7u64;
        
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
pub fn get_num_created_mission_peds_raw(
        p0: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xCB215C4B56A7FAE7u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xCB215C4B56A7FAE7u64;

        invoke_raw_typed!(
            hash,
                p0
        )
    }
}

/// ```
Getter for SET_NETWORK_CUTSCENE_ENTITIES.
```

```
NativeDB Introduced: v2699
```



pub fn _network_are_cutscene_entities_safe(
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x66D6A5E9C511214Au32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x66D6A5E9C511214Au64;
        
        let result = invoke_raw!(
            hash,
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn _network_are_cutscene_entities_raw(
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x66D6A5E9C511214Au32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x66D6A5E9C511214Au64;

        invoke_raw_typed!(
            hash,
        )
    }
}

/// ## Parameters
*



pub fn ugc_get_cached_description_safe(
        
        
            p0: 
        , 
        
        
            p1: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x40F7E66472DF3E5Cu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x40F7E66472DF3E5Cu64;
        
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
pub fn ugc_get_cached_description_raw(
        p0: , 
        p1: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x40F7E66472DF3E5Cu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x40F7E66472DF3E5Cu64;

        invoke_raw_typed!(
            hash,
                p0, 
                p1
        )
    }
}

/// ## Return value



pub fn network_is_in_session_safe(
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xCA97246103B63917u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xCA97246103B63917u64;
        
        let result = invoke_raw!(
            hash,
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn network_is_in_session_raw(
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xCA97246103B63917u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xCA97246103B63917u64;

        invoke_raw_typed!(
            hash,
        )
    }
}

/// ```
Returns a handle to networkHandle* from the specified player handle and stores it in a given buffer.  
* Currently unknown struct  
Example:  
std::vector<UINT64> GetPlayerNetworkHandle(Player player) {  
    const int size = 13;  
    uint64_t *buffer = std::make_unique<uint64_t[]>(size).get();  
    NETWORK::NETWORK_HANDLE_FROM_PLAYER(player, reinterpret_cast<int *>(buffer), 13);  
    for (int i = 0; i < size; i++) {  
        Log::Msg("networkhandle[%i]: %llx", i, buffer[i]);  
    }  
    std::vector<UINT64> result(buffer, buffer + sizeof(buffer));  
    return result;  
}  
```



pub fn network_handle_from_player_safe(
        
        
            player: 
        , 
        
        
            networkHandle: 
        , 
        
        
            bufferSize: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x388EB2B86C73B6B3u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x388EB2B86C73B6B3u64;
        
        let result = invoke_raw!(
            hash,
                player, 
                networkHandle, 
                bufferSize
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn network_handle_from_player_raw(
        player: , 
        networkHandle: , 
        bufferSize: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x388EB2B86C73B6B3u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x388EB2B86C73B6B3u64;

        invoke_raw_typed!(
            hash,
                player, 
                networkHandle, 
                bufferSize
        )
    }
}

/// ```
gets the entity id of a network id  
```



pub fn net_to_ent_safe(
        
        
            netHandle: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xBFFEAB45A9A9094Au32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xBFFEAB45A9A9094Au64;
        
        let result = invoke_raw!(
            hash,
                netHandle
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn net_to_ent_raw(
        netHandle: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xBFFEAB45A9A9094Au32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xBFFEAB45A9A9094Au64;

        invoke_raw_typed!(
            hash,
                netHandle
        )
    }
}

/// ## Return value



pub fn get_max_num_network_peds_safe(
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x0C1F7D49C39D2289u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x0C1F7D49C39D2289u64;
        
        let result = invoke_raw!(
            hash,
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn get_max_num_network_peds_raw(
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x0C1F7D49C39D2289u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x0C1F7D49C39D2289u64;

        invoke_raw_typed!(
            hash,
        )
    }
}

/// ```
Does nothing (it's a nullsub).

NativeDB Introduced: v323
```



pub fn _0xaedf1bc1c133d6e3_safe(
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xAEDF1BC1C133D6E3u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xAEDF1BC1C133D6E3u64;
        
        let result = invoke_raw!(
            hash,
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn _0xaedf1bc1c133d6e3_raw(
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xAEDF1BC1C133D6E3u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xAEDF1BC1C133D6E3u64;

        invoke_raw_typed!(
            hash,
        )
    }
}

/// ## Parameters
*



pub fn fillout_pm_player_list_with_names_safe(
        
        
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
        let hash = 0x716B6DB9D1886106u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x716B6DB9D1886106u64;
        
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
pub fn fillout_pm_player_list_with_names_raw(
        p0: , 
        p1: , 
        p2: , 
        p3: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x716B6DB9D1886106u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x716B6DB9D1886106u64;

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



pub fn network_send_invite_via_presence_safe(
        
        
            networkHandle: 
        , 
        
        
            p1: 
        , 
        
        
            p2: 
        , 
        
        
            p3: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xC3C7A6AFDB244624u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xC3C7A6AFDB244624u64;
        
        let result = invoke_raw!(
            hash,
                networkHandle, 
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
pub fn network_send_invite_via_presence_raw(
        networkHandle: , 
        p1: , 
        p2: , 
        p3: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xC3C7A6AFDB244624u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xC3C7A6AFDB244624u64;

        invoke_raw_typed!(
            hash,
                networkHandle, 
                p1, 
                p2, 
                p3
        )
    }
}

/// ```
the first arg seems to be the network player handle (&handle) and the second var is pretty much always "" and the third seems to be a number between 0 and ~10 and the 4th is is something like 0 to 5 and I guess the 5th is a bool cuz it is always 0 or 1  
does this send an invite to a player?  
```



pub fn network_send_transition_gamer_instruction_safe(
        
        
            networkHandle: 
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
        let hash = 0x31D1D2B858D25E6Bu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x31D1D2B858D25E6Bu64;
        
        let result = invoke_raw!(
            hash,
                networkHandle, 
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
pub fn network_send_transition_gamer_instruction_raw(
        networkHandle: , 
        p1: , 
        p2: , 
        p3: , 
        p4: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x31D1D2B858D25E6Bu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x31D1D2B858D25E6Bu64;

        invoke_raw_typed!(
            hash,
                networkHandle, 
                p1, 
                p2, 
                p3, 
                p4
        )
    }
}

/// ## Return value



pub fn _0x4d02279c83be69fe_safe(
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x4D02279C83BE69FEu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x4D02279C83BE69FEu64;
        
        let result = invoke_raw!(
            hash,
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn _0x4d02279c83be69fe_raw(
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x4D02279C83BE69FEu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x4D02279C83BE69FEu64;

        invoke_raw_typed!(
            hash,
        )
    }
}

/// Get the local entity handle of the given network id

Through this native you can get back the entity that you previously converted to netid with [NetworkGetNetworkIdFromEntity](#_0x9E35DAB6) or with the `ToNet` natives



pub fn network_get_entity_from_network_id_safe(
        
        
            netId: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xCE4E5D9B0A4FF560u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xCE4E5D9B0A4FF560u64;
        
        let result = invoke_raw!(
            hash,
                netId
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn network_get_entity_from_network_id_raw(
        netId: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xCE4E5D9B0A4FF560u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xCE4E5D9B0A4FF560u64;

        invoke_raw_typed!(
            hash,
                netId
        )
    }
}

/// Creates a networked synchronized scene.
Be sure to actually start the scene with [`NETWORK_START_SYNCHRONISED_SCENE`](#_0x9A1B3FCDB36C8697) after you're done adding peds or entities to the scene.



pub fn network_create_synchronised_scene_safe(
        
        
            x: 
        , 
        
        
            y: 
        , 
        
        
            z: 
        , 
        
        
            xRot: 
        , 
        
        
            yRot: 
        , 
        
        
            zRot: 
        , 
        
        
            rotationOrder: 
        , 
        
        
            holdLastFrame: 
        , 
        
        
            looped: 
        , 
        
        
            phaseToStopScene: 
        , 
        
        
            phaseToStartScene: 
        , 
        
        
            animSpeed: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x7CD6BC4C2BBDD526u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x7CD6BC4C2BBDD526u64;
        
        let result = invoke_raw!(
            hash,
                x, 
                y, 
                z, 
                xRot, 
                yRot, 
                zRot, 
                rotationOrder, 
                holdLastFrame, 
                looped, 
                phaseToStopScene, 
                phaseToStartScene, 
                animSpeed
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn network_create_synchronised_scene_raw(
        x: , 
        y: , 
        z: , 
        xRot: , 
        yRot: , 
        zRot: , 
        rotationOrder: , 
        holdLastFrame: , 
        looped: , 
        phaseToStopScene: , 
        phaseToStartScene: , 
        animSpeed: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x7CD6BC4C2BBDD526u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x7CD6BC4C2BBDD526u64;

        invoke_raw_typed!(
            hash,
                x, 
                y, 
                z, 
                xRot, 
                yRot, 
                zRot, 
                rotationOrder, 
                holdLastFrame, 
                looped, 
                phaseToStopScene, 
                phaseToStartScene, 
                animSpeed
        )
    }
}

/// ## Parameters
*



pub fn network_get_player_from_gamer_handle_safe(
        
        
            networkHandle: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xCE5F689CF5A0A49Du32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xCE5F689CF5A0A49Du64;
        
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
pub fn network_get_player_from_gamer_handle_raw(
        networkHandle: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xCE5F689CF5A0A49Du32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xCE5F689CF5A0A49Du64;

        invoke_raw_typed!(
            hash,
                networkHandle
        )
    }
}

/// ## Parameters
*



pub fn ugc_get_content_is_verified_safe(
        
        
            p0: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xA9240A96C74CCA13u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xA9240A96C74CCA13u64;
        
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
pub fn ugc_get_content_is_verified_raw(
        p0: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xA9240A96C74CCA13u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xA9240A96C74CCA13u64;

        invoke_raw_typed!(
            hash,
                p0
        )
    }
}

/// ## Parameters
*



pub fn _0x265559da40b3f327_safe(
        
        
            p0: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x265559DA40B3F327u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x265559DA40B3F327u64;
        
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
pub fn _0x265559da40b3f327_raw(
        p0: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x265559DA40B3F327u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x265559DA40B3F327u64;

        invoke_raw_typed!(
            hash,
                p0
        )
    }
}

/// ## Return value



pub fn network_is_getting_gamer_status_safe(
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x94A8394D150B013Au32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x94A8394D150B013Au64;
        
        let result = invoke_raw!(
            hash,
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn network_is_getting_gamer_status_raw(
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x94A8394D150B013Au32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x94A8394D150B013Au64;

        invoke_raw_typed!(
            hash,
        )
    }
}

/// ## Parameters
*



pub fn ugc_get_content_has_player_record_safe(
        
        
            p0: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x70EA8DA57840F9BEu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x70EA8DA57840F9BEu64;
        
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
pub fn ugc_get_content_has_player_record_raw(
        p0: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x70EA8DA57840F9BEu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x70EA8DA57840F9BEu64;

        invoke_raw_typed!(
            hash,
                p0
        )
    }
}

/// ## Return value



pub fn network_is_session_busy_safe(
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xF4435D66A8E2905Eu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xF4435D66A8E2905Eu64;
        
        let result = invoke_raw!(
            hash,
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn network_is_session_busy_raw(
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xF4435D66A8E2905Eu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xF4435D66A8E2905Eu64;

        invoke_raw_typed!(
            hash,
        )
    }
}

/// ```
NETWORK_SESSION_IS_*
```



pub fn _0xbdb6f89c729cf388_safe(
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xBDB6F89C729CF388u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xBDB6F89C729CF388u64;
        
        let result = invoke_raw!(
            hash,
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn _0xbdb6f89c729cf388_raw(
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xBDB6F89C729CF388u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xBDB6F89C729CF388u64;

        invoke_raw_typed!(
            hash,
        )
    }
}

/// NETWORK_SESSION_VOICE_LEAVE native function



pub fn network_session_voice_leave_safe(
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x6793E42BE02B575Du32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x6793E42BE02B575Du64;
        
        let result = invoke_raw!(
            hash,
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn network_session_voice_leave_raw(
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x6793E42BE02B575Du32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x6793E42BE02B575Du64;

        invoke_raw_typed!(
            hash,
        )
    }
}

/// ## Return value



pub fn network_get_num_presence_invites_safe(
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xCEFA968912D0F78Du32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xCEFA968912D0F78Du64;
        
        let result = invoke_raw!(
            hash,
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn network_get_num_presence_invites_raw(
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xCEFA968912D0F78Du32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xCEFA968912D0F78Du64;

        invoke_raw_typed!(
            hash,
        )
    }
}

/// ```
IS_*
```



pub fn _0x59328eb08c5ceb2b_safe(
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x59328EB08C5CEB2Bu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x59328EB08C5CEB2Bu64;
        
        let result = invoke_raw!(
            hash,
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn _0x59328eb08c5ceb2b_raw(
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x59328EB08C5CEB2Bu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x59328EB08C5CEB2Bu64;

        invoke_raw_typed!(
            hash,
        )
    }
}

/// ```
Hardcoded to return -1.
```



pub fn _network_displaynames_from_handles_start_safe(
        
        
            p0: 
        , 
        
        
            p1: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xD66C9E72B3CC4982u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xD66C9E72B3CC4982u64;
        
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
pub fn _network_displaynames_from_handles_start_raw(
        p0: , 
        p1: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xD66C9E72B3CC4982u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xD66C9E72B3CC4982u64;

        invoke_raw_typed!(
            hash,
                p0, 
                p1
        )
    }
}

/// ## Parameters
*



pub fn can_register_mission_peds_safe(
        
        
            amount: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xBCBF4FEF9FA5D781u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xBCBF4FEF9FA5D781u64;
        
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
pub fn can_register_mission_peds_raw(
        amount: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xBCBF4FEF9FA5D781u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xBCBF4FEF9FA5D781u64;

        invoke_raw_typed!(
            hash,
                amount
        )
    }
}

/// ## Parameters
*



pub fn network_hash_from_gamer_handle_safe(
        
        
            networkHandle: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x58575AC3CF2CA8ECu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x58575AC3CF2CA8ECu64;
        
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
pub fn network_hash_from_gamer_handle_raw(
        networkHandle: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x58575AC3CF2CA8ECu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x58575AC3CF2CA8ECu64;

        invoke_raw_typed!(
            hash,
                networkHandle
        )
    }
}

/// ## Parameters
*



pub fn _0x041c7f2a6c9894e6_safe(
        
        
            p0: 
        , 
        
        
            p1: 
        , 
        
        
            p2: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x041C7F2A6C9894E6u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x041C7F2A6C9894E6u64;
        
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
pub fn _0x041c7f2a6c9894e6_raw(
        p0: , 
        p1: , 
        p2: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x041C7F2A6C9894E6u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x041C7F2A6C9894E6u64;

        invoke_raw_typed!(
            hash,
                p0, 
                p1, 
                p2
        )
    }
}

/// ## Return value



pub fn network_can_enter_multiplayer_safe(
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x7E782A910C362C25u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x7E782A910C362C25u64;
        
        let result = invoke_raw!(
            hash,
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn network_can_enter_multiplayer_raw(
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x7E782A910C362C25u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x7E782A910C362C25u64;

        invoke_raw_typed!(
            hash,
        )
    }
}

/// Adds a ped to a networked synchronised scene.

Synchronized scene playback flags (Also works in other `NETWORK_ADD_*_TO_SYNCHRONISED_SCENE` natives):
| Value     |                  Name                     |                                                             Notes                                                                |
| :-------: | :---------------------------------------: | :------------------------------------------------------------------------------------------------------------------------------: |
| `0`       |  None                                     | No flag set.                                                                                                                     |
| `1`       | USE_PHYSICS                               | Allows the ped to have physics during the scene.                                                                                 |
| `2`       | TAG_SYNC_OUT                              | The task will do a tag synchronized blend out with the movement behaviour of the ped.                                            |
| `4`       | DONT_INTERRUPT                            | The scene will not be interrupted by external events.                                                                            |
| `8`       | ON_ABORT_STOP_SCENE                       | The scene will be stopped if the scripted task is aborted.                                                                       |
| `16`      | ABORT_ON_WEAPON_DAMAGE                    | The scene will be stopped if the ped is damaged by a weapon.                                                                     |
| `32`      | BLOCK_MOVER_UPDATE                        | The task will not update the mover.                                                                                              |
| `64`      | LOOP_WITHIN_SCENE                         | Animations within this scene will be looped until the scene is finished.                                                         |
| `128`     | PRESERVE_VELOCITY                         | The task will keep it's velocity when the scene is cleaned up/stopped. Do note that the `USE_PHYSICS` flag must also be present. |
| `256`     | EXPAND_PED_CAPSULE_FROM_SKELETON          | The task will apply the `ExpandPedCapsuleFromSkeleton` reset flag to the ped (See [`SET_PED_RESET_FLAG`](#_0xC1E8A365BF3B29F2)). |
| `512`     | ACTIVATE_RAGDOLL_ON_COLLISION             | The ped will be ragdoll if it comes in contact with an object.                                                                   |
| `1024`    | HIDE_WEAPON                               | The ped's current weapon will be hidden during the scene.                                                                        |
| `2048`    | ABORT_ON_DEATH                            | The synchronised scene will be aborted if the ped dies.                                                                          |
| `4096`    | VEHICLE_ABORT_ON_LARGE_IMPACT             | If the scene is running on a vehicle, then it will be aborted if the vehicle takes a heavy collision with another vehicle.       |
| `8192`    | VEHICLE_ALLOW_PLAYER_ENTRY                | If the scene is on a vehicle, it allows players to enter it.                                                                     |
| `16384`   | PROCESS_ATTACHMENTS_ON_START              | Attachments will be processed at the start of the scene.                                                                         |
| `32768`   | NET_ON_EARLY_NON_PED_STOP_RETURN_TO_START | A non-ped entity will be returned to their starting position if the scene finishes early.                                        |
| `65536`   | SET_PED_OUT_OF_VEHICLE_AT_START           | If the ped is in a vehicle when the scene starts, it will be set out of the vehicle.                                             |
| `131072`  | NET_DISREGARD_ATTACHMENT_CHECKS           | Attachment checks will be disregarded when the scene is running.                                                                 |

These flags can be combined with the `|` operator.



pub fn network_add_ped_to_synchronised_scene_safe(
        
        
            ped: 
        , 
        
        
            netScene: 
        , 
        
        
            animDict: 
        , 
        
        
            animClip: 
        , 
        
        
            blendInSpeed: 
        , 
        
        
            blendOutSpeed: 
        , 
        
        
            syncedSceneFlags: 
        , 
        
        
            ragdollFlags: 
        , 
        
        
            moverBlendInDelta: 
        , 
        
        
            ikFlags: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x742A637471BCECD9u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x742A637471BCECD9u64;
        
        let result = invoke_raw!(
            hash,
                ped, 
                netScene, 
                animDict, 
                animClip, 
                blendInSpeed, 
                blendOutSpeed, 
                syncedSceneFlags, 
                ragdollFlags, 
                moverBlendInDelta, 
                ikFlags
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn network_add_ped_to_synchronised_scene_raw(
        ped: , 
        netScene: , 
        animDict: , 
        animClip: , 
        blendInSpeed: , 
        blendOutSpeed: , 
        syncedSceneFlags: , 
        ragdollFlags: , 
        moverBlendInDelta: , 
        ikFlags: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x742A637471BCECD9u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x742A637471BCECD9u64;

        invoke_raw_typed!(
            hash,
                ped, 
                netScene, 
                animDict, 
                animClip, 
                blendInSpeed, 
                blendOutSpeed, 
                syncedSceneFlags, 
                ragdollFlags, 
                moverBlendInDelta, 
                ikFlags
        )
    }
}

/// ## Parameters
*



pub fn network_set_in_mp_cutscene_safe(
        
        
            p0: 
        , 
        
        
            p1: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x9CA5DE655269FEC4u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x9CA5DE655269FEC4u64;
        
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
pub fn network_set_in_mp_cutscene_raw(
        p0: , 
        p1: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x9CA5DE655269FEC4u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x9CA5DE655269FEC4u64;

        invoke_raw_typed!(
            hash,
                p0, 
                p1
        )
    }
}

/// Adds a map entity to a network synchronized scene. This native function is utilized only once as of game build 2944 within the casino_slots script.
Please note that it's only possible to add a single map entity to synchronised scenes.

It's advisable to initially locate the object and retrieve its actual coordinates using [`GET_CLOSEST_OBJECT_OF_TYPE`](#_0xE143FA2249364369).

```
NativeDB Introduced: v1734
```



pub fn network_add_map_entity_to_synchronised_scene_safe(
        
        
            netScene: 
        , 
        
        
            modelHash: 
        , 
        
        
            x: 
        , 
        
        
            y: 
        , 
        
        
            z: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x45F35C0EDC33B03Bu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x45F35C0EDC33B03Bu64;
        
        let result = invoke_raw!(
            hash,
                netScene, 
                modelHash, 
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
pub fn network_add_map_entity_to_synchronised_scene_raw(
        netScene: , 
        modelHash: , 
        x: , 
        y: , 
        z: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x45F35C0EDC33B03Bu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x45F35C0EDC33B03Bu64;

        invoke_raw_typed!(
            hash,
                netScene, 
                modelHash, 
                x, 
                y, 
                z
        )
    }
}

/// ## Parameters
*



pub fn network_queue_gamer_for_status_safe(
        
        
            p0: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x85A0EF54A500882Cu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x85A0EF54A500882Cu64;
        
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
pub fn network_queue_gamer_for_status_raw(
        p0: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x85A0EF54A500882Cu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x85A0EF54A500882Cu64;

        invoke_raw_typed!(
            hash,
                p0
        )
    }
}

/// ```
NativeDB Introduced: v2372
```



pub fn _network_ped_force_game_state_update_safe(
        
        
            ped: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xF0BC9BCD24A511D5u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xF0BC9BCD24A511D5u64;
        
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
pub fn _network_ped_force_game_state_update_raw(
        ped: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xF0BC9BCD24A511D5u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xF0BC9BCD24A511D5u64;

        invoke_raw_typed!(
            hash,
                ped
        )
    }
}

/// ## Return value
Hard-coded to always return 0.



pub fn _0x88b588b41ff7868e_safe(
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x88B588B41FF7868Eu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x88B588B41FF7868Eu64;
        
        let result = invoke_raw!(
            hash,
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn _0x88b588b41ff7868e_raw(
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x88B588B41FF7868Eu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x88B588B41FF7868Eu64;

        invoke_raw_typed!(
            hash,
        )
    }
}

/// ## Parameters
*



pub fn _0x4348bfda56023a2f_safe(
        
        
            p0: 
        , 
        
        
            p1: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x4348BFDA56023A2Fu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x4348BFDA56023A2Fu64;
        
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
pub fn _0x4348bfda56023a2f_raw(
        p0: , 
        p1: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x4348BFDA56023A2Fu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x4348BFDA56023A2Fu64;

        invoke_raw_typed!(
            hash,
                p0, 
                p1
        )
    }
}

/// ## Return value



pub fn network_is_local_talking_safe(
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xC0D2AF00BCC234CAu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xC0D2AF00BCC234CAu64;
        
        let result = invoke_raw!(
            hash,
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn network_is_local_talking_raw(
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xC0D2AF00BCC234CAu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xC0D2AF00BCC234CAu64;

        invoke_raw_typed!(
            hash,
        )
    }
}

/// ## Parameters
*



pub fn _network_get_displaynames_from_handles_safe(
        
        
            p0: 
        , 
        
        
            p1: 
        , 
        
        
            p2: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x58CC181719256197u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x58CC181719256197u64;
        
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
pub fn _network_get_displaynames_from_handles_raw(
        p0: , 
        p1: , 
        p2: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x58CC181719256197u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x58CC181719256197u64;

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



pub fn network_get_player_tutorial_session_instance_safe(
        
        
            player: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x3B39236746714134u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x3B39236746714134u64;
        
        let result = invoke_raw!(
            hash,
                player
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn network_get_player_tutorial_session_instance_raw(
        player: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x3B39236746714134u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x3B39236746714134u64;

        invoke_raw_typed!(
            hash,
                player
        )
    }
}

/// ## Parameters
*



pub fn network_set_choice_migrate_options_safe(
        
        
            toggle: 
        , 
        
        
            player: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x5C707A667DF8B9FAu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x5C707A667DF8B9FAu64;
        
        let result = invoke_raw!(
            hash,
                toggle, 
                player
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn network_set_choice_migrate_options_raw(
        toggle: , 
        player: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x5C707A667DF8B9FAu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x5C707A667DF8B9FAu64;

        invoke_raw_typed!(
            hash,
                toggle, 
                player
        )
    }
}

/// ```
Starts a new singleplayer game (at the prologue).  
```



pub fn shutdown_and_launch_single_player_game_safe(
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x593850C16A36B692u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x593850C16A36B692u64;
        
        let result = invoke_raw!(
            hash,
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn shutdown_and_launch_single_player_game_raw(
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x593850C16A36B692u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x593850C16A36B692u64;

        invoke_raw_typed!(
            hash,
        )
    }
}

/// ## Parameters
*



pub fn network_session_validate_join_safe(
        
        
            p0: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xC19F6C8E7865A6FFu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xC19F6C8E7865A6FFu64;
        
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
pub fn network_session_validate_join_raw(
        p0: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xC19F6C8E7865A6FFu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xC19F6C8E7865A6FFu64;

        invoke_raw_typed!(
            hash,
                p0
        )
    }
}

/// ## Parameters
*



pub fn _0x7fcc39c46c3c03bd_safe(
        
        
            p0: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x7FCC39C46C3C03BDu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x7FCC39C46C3C03BDu64;
        
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
pub fn _0x7fcc39c46c3c03bd_raw(
        p0: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x7FCC39C46C3C03BDu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x7FCC39C46C3C03BDu64;

        invoke_raw_typed!(
            hash,
                p0
        )
    }
}

/// ## Return value



pub fn network_join_group_activity_safe(
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xA06509A691D12BE4u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xA06509A691D12BE4u64;
        
        let result = invoke_raw!(
            hash,
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn network_join_group_activity_raw(
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xA06509A691D12BE4u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xA06509A691D12BE4u64;

        invoke_raw_typed!(
            hash,
        )
    }
}

/// ## Parameters
*



pub fn _0x2b51edbefc301339_safe(
        
        
            p0: 
        , 
        
        
            p1: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x2B51EDBEFC301339u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x2B51EDBEFC301339u64;
        
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
pub fn _0x2b51edbefc301339_raw(
        p0: , 
        p1: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x2B51EDBEFC301339u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x2B51EDBEFC301339u64;

        invoke_raw_typed!(
            hash,
                p0, 
                p1
        )
    }
}

/// ## Parameters
*



pub fn ugc_request_cached_description_safe(
        
        
            p0: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x5E0165278F6339EEu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x5E0165278F6339EEu64;
        
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
pub fn ugc_request_cached_description_raw(
        p0: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x5E0165278F6339EEu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x5E0165278F6339EEu64;

        invoke_raw_typed!(
            hash,
                p0
        )
    }
}

/// ## Parameters
*



pub fn network_set_local_player_sync_look_at_safe(
        
        
            toggle: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x524FF0AEFF9C3973u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x524FF0AEFF9C3973u64;
        
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
pub fn network_set_local_player_sync_look_at_raw(
        toggle: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x524FF0AEFF9C3973u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x524FF0AEFF9C3973u64;

        invoke_raw_typed!(
            hash,
                toggle
        )
    }
}

/// ## Parameters
*



pub fn network_disable_invincible_flashing_safe(
        
        
            player: 
        , 
        
        
            toggle: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x9DD368BF06983221u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x9DD368BF06983221u64;
        
        let result = invoke_raw!(
            hash,
                player, 
                toggle
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn network_disable_invincible_flashing_raw(
        player: , 
        toggle: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x9DD368BF06983221u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x9DD368BF06983221u64;

        invoke_raw_typed!(
            hash,
                player, 
                toggle
        )
    }
}

/// NETWORK_S*

```
NativeDB Introduced: v1734
```



pub fn _0xca59ccae5d01e4ce_safe(
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xCA59CCAE5D01E4CEu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xCA59CCAE5D01E4CEu64;
        
        let result = invoke_raw!(
            hash,
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn _0xca59ccae5d01e4ce_raw(
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xCA59CCAE5D01E4CEu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xCA59CCAE5D01E4CEu64;

        invoke_raw_typed!(
            hash,
        )
    }
}

/// ## Parameters
*



pub fn cloud_has_request_completed_safe(
        
        
            handle: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x4C61B39930D045DAu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x4C61B39930D045DAu64;
        
        let result = invoke_raw!(
            hash,
                handle
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn cloud_has_request_completed_raw(
        handle: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x4C61B39930D045DAu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x4C61B39930D045DAu64;

        invoke_raw_typed!(
            hash,
                handle
        )
    }
}

/// ## Parameters
*



pub fn network_apply_voice_proximity_override_safe(
        
        
            x: 
        , 
        
        
            y: 
        , 
        
        
            z: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xDBD2056652689917u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xDBD2056652689917u64;
        
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
pub fn network_apply_voice_proximity_override_raw(
        x: , 
        y: , 
        z: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xDBD2056652689917u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xDBD2056652689917u64;

        invoke_raw_typed!(
            hash,
                x, 
                y, 
                z
        )
    }
}

/// NETWORK_OPEN_TRANSITION_MATCHMAKING native function



pub fn network_open_transition_matchmaking_safe(
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x2B3A8F7CA3A38FDEu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x2B3A8F7CA3A38FDEu64;
        
        let result = invoke_raw!(
            hash,
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn network_open_transition_matchmaking_raw(
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x2B3A8F7CA3A38FDEu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x2B3A8F7CA3A38FDEu64;

        invoke_raw_typed!(
            hash,
        )
    }
}

/// ```
Returns true if the NAT type is Strict (3) and a certain number of connections have failed.
```



pub fn _network_should_show_connectivity_troubleshooting_safe(
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x82A2B386716608F1u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x82A2B386716608F1u64;
        
        let result = invoke_raw!(
            hash,
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn _network_should_show_connectivity_troubleshooting_raw(
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x82A2B386716608F1u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x82A2B386716608F1u64;

        invoke_raw_typed!(
            hash,
        )
    }
}

/// ## Parameters
*



pub fn network_suppress_invite_safe(
        
        
            toggle: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xA0682D67EF1FBA3Du32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xA0682D67EF1FBA3Du64;
        
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
pub fn network_suppress_invite_raw(
        toggle: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xA0682D67EF1FBA3Du32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xA0682D67EF1FBA3Du64;

        invoke_raw_typed!(
            hash,
                toggle
        )
    }
}

/// ## Parameters
*



pub fn network_set_activity_player_max_safe(
        
        
            playerCount: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x0E4F77F7B9D74D84u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x0E4F77F7B9D74D84u64;
        
        let result = invoke_raw!(
            hash,
                playerCount
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn network_set_activity_player_max_raw(
        playerCount: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x0E4F77F7B9D74D84u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x0E4F77F7B9D74D84u64;

        invoke_raw_typed!(
            hash,
                playerCount
        )
    }
}

/// ## Return value



pub fn _0x24e4e51fc16305f9_safe(
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x24E4E51FC16305F9u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x24E4E51FC16305F9u64;
        
        let result = invoke_raw!(
            hash,
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn _0x24e4e51fc16305f9_raw(
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x24E4E51FC16305F9u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x24E4E51FC16305F9u64;

        invoke_raw_typed!(
            hash,
        )
    }
}

/// ## Parameters
*



pub fn network_access_tunable_int_hash_safe(
        
        
            tunableContext: 
        , 
        
        
            tunableName: 
        , 
        
        
            value: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x40FCE03E50E8DBE8u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x40FCE03E50E8DBE8u64;
        
        let result = invoke_raw!(
            hash,
                tunableContext, 
                tunableName, 
                value
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn network_access_tunable_int_hash_raw(
        tunableContext: , 
        tunableName: , 
        value: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x40FCE03E50E8DBE8u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x40FCE03E50E8DBE8u64;

        invoke_raw_typed!(
            hash,
                tunableContext, 
                tunableName, 
                value
        )
    }
}

/// ```
NativeDB Added Parameter 3: int p2
```



pub fn get_commerce_item_texturename_safe(
        
        
            index: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x722F5D28B61C5EA8u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x722F5D28B61C5EA8u64;
        
        let result = invoke_raw!(
            hash,
                index
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn get_commerce_item_texturename_raw(
        index: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x722F5D28B61C5EA8u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x722F5D28B61C5EA8u64;

        invoke_raw_typed!(
            hash,
                index
        )
    }
}

/// ## Parameters
*



pub fn _0xd7b6c73cad419bcf_safe(
        
        
            p0: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xD7B6C73CAD419BCFu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xD7B6C73CAD419BCFu64;
        
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
pub fn _0xd7b6c73cad419bcf_raw(
        p0: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xD7B6C73CAD419BCFu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xD7B6C73CAD419BCFu64;

        invoke_raw_typed!(
            hash,
                p0
        )
    }
}

/// ## Parameters
*



pub fn set_entity_visible_in_cutscene_safe(
        
        
            p0: 
        , 
        
        
            p1: 
        , 
        
        
            p2: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xE0031D3C8F36AB82u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xE0031D3C8F36AB82u64;
        
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
pub fn set_entity_visible_in_cutscene_raw(
        p0: , 
        p1: , 
        p2: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xE0031D3C8F36AB82u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xE0031D3C8F36AB82u64;

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



pub fn set_player_visible_locally_safe(
        
        
            player: 
        , 
        
        
            toggle: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xFAA10F1FAFB11AF2u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xFAA10F1FAFB11AF2u64;
        
        let result = invoke_raw!(
            hash,
                player, 
                toggle
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn set_player_visible_locally_raw(
        player: , 
        toggle: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xFAA10F1FAFB11AF2u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xFAA10F1FAFB11AF2u64;

        invoke_raw_typed!(
            hash,
                player, 
                toggle
        )
    }
}

/// _NETWORK_UPDATE_PLAYER_SCARS native function



pub fn _network_update_player_scars_safe(
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xB7C7F6AD6424304Bu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xB7C7F6AD6424304Bu64;
        
        let result = invoke_raw!(
            hash,
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn _network_update_player_scars_raw(
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xB7C7F6AD6424304Bu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xB7C7F6AD6424304Bu64;

        invoke_raw_typed!(
            hash,
        )
    }
}

/// Returns the coordinates of another player.

Does not work if you enter your own player id as p0 (will return `(0.0, 0.0, 0.0)` in that case).



pub fn _network_get_player_coords_safe(
        
        
            player: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x125E6D638B8605D4u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x125E6D638B8605D4u64;
        
        let result = invoke_raw!(
            hash,
                player
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn _network_get_player_coords_raw(
        player: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x125E6D638B8605D4u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x125E6D638B8605D4u64;

        invoke_raw_typed!(
            hash,
                player
        )
    }
}

/// ## Parameters
*



pub fn _0x367ef5e2f439b4c6_safe(
        
        
            p0: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x367EF5E2F439B4C6u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x367EF5E2F439B4C6u64;
        
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
pub fn _0x367ef5e2f439b4c6_raw(
        p0: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x367EF5E2F439B4C6u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x367EF5E2F439B4C6u64;

        invoke_raw_typed!(
            hash,
                p0
        )
    }
}

/// ## Parameters
*



pub fn network_apply_transition_parameter_safe(
        
        
            p0: 
        , 
        
        
            p1: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x521638ADA1BA0D18u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x521638ADA1BA0D18u64;
        
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
pub fn network_apply_transition_parameter_raw(
        p0: , 
        p1: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x521638ADA1BA0D18u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x521638ADA1BA0D18u64;

        invoke_raw_typed!(
            hash,
                p0, 
                p1
        )
    }
}

/// ## Parameters
*



pub fn network_apply_transition_parameter_string_safe(
        
        
            p0: 
        , 
        
        
            string: 
        , 
        
        
            p2: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xEBEFC2E77084F599u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xEBEFC2E77084F599u64;
        
        let result = invoke_raw!(
            hash,
                p0, 
                string, 
                p2
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn network_apply_transition_parameter_string_raw(
        p0: , 
        string: , 
        p2: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xEBEFC2E77084F599u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xEBEFC2E77084F599u64;

        invoke_raw_typed!(
            hash,
                p0, 
                string, 
                p2
        )
    }
}

/// ## Return value



pub fn network_is_transition_visibility_locked_safe(
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xD0A484CB2F829FBEu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xD0A484CB2F829FBEu64;
        
        let result = invoke_raw!(
            hash,
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn network_is_transition_visibility_locked_raw(
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xD0A484CB2F829FBEu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xD0A484CB2F829FBEu64;

        invoke_raw_typed!(
            hash,
        )
    }
}

/// ```
related to: 
NETWORK_BAIL  
NETWORK_BAIL_TRANSITION  
NETWORK_JOIN_GROUP_ACTIVITY  
NETWORK_JOIN_TRANSITION  
NETWORK_LAUNCH_TRANSITION  
NETWORK_SESSION_HOST  
NETWORK_SESSION_HOST_CLOSED  
NETWORK_SESSION_HOST_FRIENDS_ONLY  
NETWORK_SESSION_HOST_SINGLE_PLAYER  
NETWORK_SESSION_VOICE_LEAVE  
```



pub fn _0x444c4525ece0a4b9_safe(
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x444C4525ECE0A4B9u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x444C4525ECE0A4B9u64;
        
        let result = invoke_raw!(
            hash,
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn _0x444c4525ece0a4b9_raw(
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x444C4525ECE0A4B9u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x444C4525ECE0A4B9u64;

        invoke_raw_typed!(
            hash,
        )
    }
}

/// ```
..  
```



pub fn network_add_followers_safe(
        
        
            p0: 
        , 
        
        
            p1: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x236406F60CF216D6u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x236406F60CF216D6u64;
        
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
pub fn network_add_followers_raw(
        p0: , 
        p1: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x236406F60CF216D6u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x236406F60CF216D6u64;

        invoke_raw_typed!(
            hash,
                p0, 
                p1
        )
    }
}

/// Forces the "Are you sure you want to quit Grand Theft Auto V?" warning message (Same as when you Alt+F4) to show.
Doesn't work in singleplayer.



pub fn network_quit_mp_to_desktop_safe(
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x45A83257ED02D9BCu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x45A83257ED02D9BCu64;
        
        let result = invoke_raw!(
            hash,
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn network_quit_mp_to_desktop_raw(
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x45A83257ED02D9BCu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x45A83257ED02D9BCu64;

        invoke_raw_typed!(
            hash,
        )
    }
}

/// ## Parameters
*



pub fn ugc_set_bookmarked_safe(
        
        
            contentId: 
        , 
        
        
            bookmarked: 
        , 
        
        
            contentTypeName: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x274A1519DFC1094Fu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x274A1519DFC1094Fu64;
        
        let result = invoke_raw!(
            hash,
                contentId, 
                bookmarked, 
                contentTypeName
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn ugc_set_bookmarked_raw(
        contentId: , 
        bookmarked: , 
        contentTypeName: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x274A1519DFC1094Fu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x274A1519DFC1094Fu64;

        invoke_raw_typed!(
            hash,
                contentId, 
                bookmarked, 
                contentTypeName
        )
    }
}

/// ## Return value
Hard-coded to always return 1.



pub fn _0x7808619f31ff22db_safe(
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x7808619F31FF22DBu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x7808619F31FF22DBu64;
        
        let result = invoke_raw!(
            hash,
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn _0x7808619f31ff22db_raw(
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x7808619F31FF22DBu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x7808619F31FF22DBu64;

        invoke_raw_typed!(
            hash,
        )
    }
}

/// ## Parameters
*



pub fn network_set_voice_active_safe(
        
        
            toggle: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xBABEC9E69A91C57Bu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xBABEC9E69A91C57Bu64;
        
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
pub fn network_set_voice_active_raw(
        toggle: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xBABEC9E69A91C57Bu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xBABEC9E69A91C57Bu64;

        invoke_raw_typed!(
            hash,
                toggle
        )
    }
}

/// ## Return value



pub fn network_get_primary_clan_data_success_safe(
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x5B4F04F19376A0BAu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x5B4F04F19376A0BAu64;
        
        let result = invoke_raw!(
            hash,
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn network_get_primary_clan_data_success_raw(
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x5B4F04F19376A0BAu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x5B4F04F19376A0BAu64;

        invoke_raw_typed!(
            hash,
        )
    }
}

/// ## Return value



pub fn _network_get_targeting_mode_safe(
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xDFFA5BE8381C3314u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xDFFA5BE8381C3314u64;
        
        let result = invoke_raw!(
            hash,
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn _network_get_targeting_mode_raw(
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xDFFA5BE8381C3314u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xDFFA5BE8381C3314u64;

        invoke_raw_typed!(
            hash,
        )
    }
}

/// ## Parameters
*



pub fn _network_transition_track_safe(
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xC3BFED92026A2AADu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xC3BFED92026A2AADu64;
        
        let result = invoke_raw!(
            hash,
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn _network_transition_track_raw(
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xC3BFED92026A2AADu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xC3BFED92026A2AADu64;

        invoke_raw_typed!(
            hash,
        )
    }
}

/// ## Parameters
*



pub fn network_start_synchronised_scene_safe(
        
        
            netScene: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x9A1B3FCDB36C8697u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x9A1B3FCDB36C8697u64;
        
        let result = invoke_raw!(
            hash,
                netScene
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn network_start_synchronised_scene_raw(
        netScene: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x9A1B3FCDB36C8697u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x9A1B3FCDB36C8697u64;

        invoke_raw_typed!(
            hash,
                netScene
        )
    }
}

/// ## Return value



pub fn _0x53c10c8bd774f2c9_safe(
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x53C10C8BD774F2C9u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x53C10C8BD774F2C9u64;
        
        let result = invoke_raw!(
            hash,
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn _0x53c10c8bd774f2c9_raw(
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x53C10C8BD774F2C9u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x53C10C8BD774F2C9u64;

        invoke_raw_typed!(
            hash,
        )
    }
}

/// ```
NativeDB Introduced: v1868
```



pub fn _0xd6d7478ca62b8d41_safe(
        
        
            p0: 
        , 
        
        
            p1: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xD6D7478CA62B8D41u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xD6D7478CA62B8D41u64;
        
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
pub fn _0xd6d7478ca62b8d41_raw(
        p0: , 
        p1: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xD6D7478CA62B8D41u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xD6D7478CA62B8D41u64;

        invoke_raw_typed!(
            hash,
                p0, 
                p1
        )
    }
}

/// ```
Sets some voice chat related value.

NETWORK_SET_*
```



pub fn _0x3c5c1e2c2ff814b1_safe(
        
        
            toggle: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x3C5C1E2C2FF814B1u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x3C5C1E2C2FF814B1u64;
        
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
pub fn _0x3c5c1e2c2ff814b1_raw(
        toggle: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x3C5C1E2C2FF814B1u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x3C5C1E2C2FF814B1u64;

        invoke_raw_typed!(
            hash,
                toggle
        )
    }
}

/// ## Parameters
*



pub fn network_am_i_blocked_by_player_safe(
        
        
            player: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x87F395D957D4353Du32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x87F395D957D4353Du64;
        
        let result = invoke_raw!(
            hash,
                player
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn network_am_i_blocked_by_player_raw(
        player: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x87F395D957D4353Du32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x87F395D957D4353Du64;

        invoke_raw_typed!(
            hash,
                player
        )
    }
}

/// ```
Sets some voice chat related value.

NETWORK_SET_*
```



pub fn _0x9d7afcbf21c51712_safe(
        
        
            toggle: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x9D7AFCBF21C51712u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x9D7AFCBF21C51712u64;
        
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
pub fn _0x9d7afcbf21c51712_raw(
        toggle: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x9D7AFCBF21C51712u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x9D7AFCBF21C51712u64;

        invoke_raw_typed!(
            hash,
                toggle
        )
    }
}

/// ## Parameters
*



pub fn network_invite_gamers_to_transition_safe(
        
        
            p0: 
        , 
        
        
            p1: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x4A595C32F77DFF76u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x4A595C32F77DFF76u64;
        
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
pub fn network_invite_gamers_to_transition_raw(
        p0: , 
        p1: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x4A595C32F77DFF76u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x4A595C32F77DFF76u64;

        invoke_raw_typed!(
            hash,
                p0, 
                p1
        )
    }
}

/// ## Parameters
*



pub fn network_set_activity_spectator_max_safe(
        
        
            maxSpectators: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x9D277B76D1D12222u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x9D277B76D1D12222u64;
        
        let result = invoke_raw!(
            hash,
                maxSpectators
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn network_set_activity_spectator_max_raw(
        maxSpectators: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x9D277B76D1D12222u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x9D277B76D1D12222u64;

        invoke_raw_typed!(
            hash,
                maxSpectators
        )
    }
}

/// ```
NativeDB Added Parameter 3: Any p2
NativeDB Added Parameter 4: Any p3
```



pub fn set_network_vehicle_respot_timer_safe(
        
        
            netId: 
        , 
        
        
            time: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xEC51713AB6EC36E8u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xEC51713AB6EC36E8u64;
        
        let result = invoke_raw!(
            hash,
                netId, 
                time
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn set_network_vehicle_respot_timer_raw(
        netId: , 
        time: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xEC51713AB6EC36E8u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xEC51713AB6EC36E8u64;

        invoke_raw_typed!(
            hash,
                netId, 
                time
        )
    }
}

/// ```
NativeDB Introduced: v1868
```



pub fn _0xea8c0ddb10e2822a_safe(
        
        
            p0: 
        , 
        
        
            p1: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xEA8C0DDB10E2822Au32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xEA8C0DDB10E2822Au64;
        
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
pub fn _0xea8c0ddb10e2822a_raw(
        p0: , 
        p1: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xEA8C0DDB10E2822Au32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xEA8C0DDB10E2822Au64;

        invoke_raw_typed!(
            hash,
                p0, 
                p1
        )
    }
}

/// ```
Downloads prod.cloud.rockstargames.com/titles/gta5/[platform]/check.json
```



pub fn cloud_check_availability_safe(
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x4F18196C8D38768Du32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x4F18196C8D38768Du64;
        
        let result = invoke_raw!(
            hash,
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn cloud_check_availability_raw(
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x4F18196C8D38768Du32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x4F18196C8D38768Du64;

        invoke_raw_typed!(
            hash,
        )
    }
}

/// ## Return value



pub fn _network_allocate_tunables_registration_data_map_safe(
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xFAFC23AEE23868DBu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xFAFC23AEE23868DBu64;
        
        let result = invoke_raw!(
            hash,
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn _network_allocate_tunables_registration_data_map_raw(
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xFAFC23AEE23868DBu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xFAFC23AEE23868DBu64;

        invoke_raw_typed!(
            hash,
        )
    }
}

/// ## Parameters
*



pub fn title_texture_download_request_safe(
        
        
            FilePath: 
        , 
        
        
            Name: 
        , 
        
        
            p2: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x0B203B4AFDE53A4Fu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x0B203B4AFDE53A4Fu64;
        
        let result = invoke_raw!(
            hash,
                FilePath, 
                Name, 
                p2
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn title_texture_download_request_raw(
        FilePath: , 
        Name: , 
        p2: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x0B203B4AFDE53A4Fu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x0B203B4AFDE53A4Fu64;

        invoke_raw_typed!(
            hash,
                FilePath, 
                Name, 
                p2
        )
    }
}

/// ```
This would be nice to see if someone is in party chat, but 2 sad notes.  
1) It only becomes true if said person is speaking in that party at the time.  
2) It will never, become true unless you are in that party with said person.  
```



pub fn network_is_chatting_in_platform_party_safe(
        
        
            networkHandle: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x8DE9945BCC9AEC52u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x8DE9945BCC9AEC52u64;
        
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
pub fn network_is_chatting_in_platform_party_raw(
        networkHandle: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x8DE9945BCC9AEC52u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x8DE9945BCC9AEC52u64;

        invoke_raw_typed!(
            hash,
                networkHandle
        )
    }
}

/// ## Return value



pub fn network_did_get_gamer_status_succeed_safe(
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x5AE17C6B0134B7F1u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x5AE17C6B0134B7F1u64;
        
        let result = invoke_raw!(
            hash,
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn network_did_get_gamer_status_succeed_raw(
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x5AE17C6B0134B7F1u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x5AE17C6B0134B7F1u64;

        invoke_raw_typed!(
            hash,
        )
    }
}

/// ## Parameters
*



pub fn network_get_presence_invite_handle_safe(
        
        
            p0: 
        , 
        
        
            p1: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x38D5B0FEBB086F75u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x38D5B0FEBB086F75u64;
        
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
pub fn network_get_presence_invite_handle_raw(
        p0: , 
        p1: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x38D5B0FEBB086F75u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x38D5B0FEBB086F75u64;

        invoke_raw_typed!(
            hash,
                p0, 
                p1
        )
    }
}

/// ## Return value



pub fn network_get_timeout_time_safe(
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x5ED0356A0CE3A34Fu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x5ED0356A0CE3A34Fu64;
        
        let result = invoke_raw!(
            hash,
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn network_get_timeout_time_raw(
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x5ED0356A0CE3A34Fu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x5ED0356A0CE3A34Fu64;

        invoke_raw_typed!(
            hash,
        )
    }
}

/// ## Parameters
*



pub fn texture_download_release_safe(
        
        
            p0: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x487EB90B98E9FB19u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x487EB90B98E9FB19u64;
        
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
pub fn texture_download_release_raw(
        p0: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x487EB90B98E9FB19u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x487EB90B98E9FB19u64;

        invoke_raw_typed!(
            hash,
                p0
        )
    }
}

/// ## Parameters
*



pub fn _0x78321bea235fd8cd_safe(
        
        
            p0: 
        , 
        
        
            p1: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x78321BEA235FD8CDu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x78321BEA235FD8CDu64;
        
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
pub fn _0x78321bea235fd8cd_raw(
        p0: , 
        p1: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x78321BEA235FD8CDu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x78321BEA235FD8CDu64;

        invoke_raw_typed!(
            hash,
                p0, 
                p1
        )
    }
}

/// ## Parameters
*



pub fn network_access_tunable_bool_hash_safe(
        
        
            tunableContext: 
        , 
        
        
            tunableName: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xEA16B69D93D71A45u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xEA16B69D93D71A45u64;
        
        let result = invoke_raw!(
            hash,
                tunableContext, 
                tunableName
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn network_access_tunable_bool_hash_raw(
        tunableContext: , 
        tunableName: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xEA16B69D93D71A45u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xEA16B69D93D71A45u64;

        invoke_raw_typed!(
            hash,
                tunableContext, 
                tunableName
        )
    }
}

/// ## Parameters
*



pub fn set_network_vehicle_as_ghost_safe(
        
        
            vehicle: 
        , 
        
        
            toggle: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x6274C4712850841Eu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x6274C4712850841Eu64;
        
        let result = invoke_raw!(
            hash,
                vehicle, 
                toggle
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn set_network_vehicle_as_ghost_raw(
        vehicle: , 
        toggle: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x6274C4712850841Eu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x6274C4712850841Eu64;

        invoke_raw_typed!(
            hash,
                vehicle, 
                toggle
        )
    }
}

/// ## Parameters
*



pub fn network_clan_get_membership_desc_safe(
        
        
            memberDesc: 
        , 
        
        
            p1: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x48DE78AF2C8885B8u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x48DE78AF2C8885B8u64;
        
        let result = invoke_raw!(
            hash,
                memberDesc, 
                p1
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn network_clan_get_membership_desc_raw(
        memberDesc: , 
        p1: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x48DE78AF2C8885B8u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x48DE78AF2C8885B8u64;

        invoke_raw_typed!(
            hash,
                memberDesc, 
                p1
        )
    }
}

/// ## Parameters
*



pub fn network_set_in_spectator_mode_extended_safe(
        
        
            toggle: 
        , 
        
        
            playerPed: 
        , 
        
        
            p2: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x419594E137637120u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x419594E137637120u64;
        
        let result = invoke_raw!(
            hash,
                toggle, 
                playerPed, 
                p2
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn network_set_in_spectator_mode_extended_raw(
        toggle: , 
        playerPed: , 
        p2: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x419594E137637120u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x419594E137637120u64;

        invoke_raw_typed!(
            hash,
                toggle, 
                playerPed, 
                p2
        )
    }
}

/// Hardcoded to return false.

```
NativeDB Introduced: v1734
```



pub fn _0x64e5c4cc82847b73_safe(
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x64E5C4CC82847B73u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x64E5C4CC82847B73u64;
        
        let result = invoke_raw!(
            hash,
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn _0x64e5c4cc82847b73_raw(
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x64E5C4CC82847B73u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x64E5C4CC82847B73u64;

        invoke_raw_typed!(
            hash,
        )
    }
}

/// ## Return value



pub fn _facebook_is_sending_data_safe(
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x62B9FEC9A11F10EFu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x62B9FEC9A11F10EFu64;
        
        let result = invoke_raw!(
            hash,
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn _facebook_is_sending_data_raw(
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x62B9FEC9A11F10EFu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x62B9FEC9A11F10EFu64;

        invoke_raw_typed!(
            hash,
        )
    }
}

/// _0x25D990F8E0E3F13C native function



pub fn _0x25d990f8e0e3f13c_safe(
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x25D990F8E0E3F13Cu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x25D990F8E0E3F13Cu64;
        
        let result = invoke_raw!(
            hash,
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn _0x25d990f8e0e3f13c_raw(
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x25D990F8E0E3F13Cu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x25D990F8E0E3F13Cu64;

        invoke_raw_typed!(
            hash,
        )
    }
}

/// Retrieves a membership for a player, from the cache (i.e. downloaded via NETWORK_CLAN_DOWNLOAD_MEMBERSHIP).

Test C++ code:
https://pastebin.com/CD8wni4C



pub fn network_clan_get_membership_safe(
        
        
            networkHandle: 
        , 
        
        
            clanMembership: 
        , 
        
        
            membershipIndex: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xC8BC2011F67B3411u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xC8BC2011F67B3411u64;
        
        let result = invoke_raw!(
            hash,
                networkHandle, 
                clanMembership, 
                membershipIndex
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn network_clan_get_membership_raw(
        networkHandle: , 
        clanMembership: , 
        membershipIndex: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xC8BC2011F67B3411u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xC8BC2011F67B3411u64;

        invoke_raw_typed!(
            hash,
                networkHandle, 
                clanMembership, 
                membershipIndex
        )
    }
}

/// _0xEBF8284D8CADEB53 native function



pub fn _0xebf8284d8cadeb53_safe(
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xEBF8284D8CADEB53u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xEBF8284D8CADEB53u64;
        
        let result = invoke_raw!(
            hash,
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn _0xebf8284d8cadeb53_raw(
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xEBF8284D8CADEB53u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xEBF8284D8CADEB53u64;

        invoke_raw_typed!(
            hash,
        )
    }
}

/// ## Parameters
*



pub fn network_is_entity_fading_safe(
        
        
            entity: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x422F32CC7E56ABADu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x422F32CC7E56ABADu64;
        
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
pub fn network_is_entity_fading_raw(
        entity: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x422F32CC7E56ABADu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x422F32CC7E56ABADu64;

        invoke_raw_typed!(
            hash,
                entity
        )
    }
}

/// ## Return value



pub fn _0xc42dd763159f3461_safe(
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xC42DD763159F3461u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xC42DD763159F3461u64;
        
        let result = invoke_raw!(
            hash,
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn _0xc42dd763159f3461_raw(
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xC42DD763159F3461u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xC42DD763159F3461u64;

        invoke_raw_typed!(
            hash,
        )
    }
}

/// ## Return value



pub fn ugc_get_creator_num_safe(
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x597F8DBA9B206FC7u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x597F8DBA9B206FC7u64;
        
        let result = invoke_raw!(
            hash,
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn ugc_get_creator_num_raw(
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x597F8DBA9B206FC7u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x597F8DBA9B206FC7u64;

        invoke_raw_typed!(
            hash,
        )
    }
}

/// ## Parameters
*



pub fn network_block_invites_safe(
        
        
            toggle: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x34F9E9049454A7A0u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x34F9E9049454A7A0u64;
        
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
pub fn network_block_invites_raw(
        toggle: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x34F9E9049454A7A0u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x34F9E9049454A7A0u64;

        invoke_raw_typed!(
            hash,
                toggle
        )
    }
}

/// ## Parameters
*



pub fn network_seed_random_number_generator_safe(
        
        
            seed: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xF1B84178F8674195u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xF1B84178F8674195u64;
        
        let result = invoke_raw!(
            hash,
                seed
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn network_seed_random_number_generator_raw(
        seed: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xF1B84178F8674195u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xF1B84178F8674195u64;

        invoke_raw_typed!(
            hash,
                seed
        )
    }
}

/// ```
NativeDB Added Parameter 1: int p0
NativeDB Added Parameter 2: int p1
NativeDB Added Parameter 3: int p2
```



pub fn network_bail_safe(
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x95914459A87EBA28u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x95914459A87EBA28u64;
        
        let result = invoke_raw!(
            hash,
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn network_bail_raw(
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x95914459A87EBA28u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x95914459A87EBA28u64;

        invoke_raw_typed!(
            hash,
        )
    }
}

/// NETWORK_SESSION_VOICE_HOST native function



pub fn network_session_voice_host_safe(
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x9C1556705F864230u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x9C1556705F864230u64;
        
        let result = invoke_raw!(
            hash,
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn network_session_voice_host_raw(
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x9C1556705F864230u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x9C1556705F864230u64;

        invoke_raw_typed!(
            hash,
        )
    }
}

/// ## Parameters
*



pub fn activate_damage_tracker_on_network_id_safe(
        
        
            netID: 
        , 
        
        
            toggle: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xD45B1FFCCD52FF19u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xD45B1FFCCD52FF19u64;
        
        let result = invoke_raw!(
            hash,
                netID, 
                toggle
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn activate_damage_tracker_on_network_id_raw(
        netID: , 
        toggle: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xD45B1FFCCD52FF19u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xD45B1FFCCD52FF19u64;

        invoke_raw_typed!(
            hash,
                netID, 
                toggle
        )
    }
}

/// ## Parameters
*



pub fn ugc_get_content_name_safe(
        
        
            p0: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xBF09786A7FCAB582u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xBF09786A7FCAB582u64;
        
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
pub fn ugc_get_content_name_raw(
        p0: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xBF09786A7FCAB582u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xBF09786A7FCAB582u64;

        invoke_raw_typed!(
            hash,
                p0
        )
    }
}

/// ```
p1 = 6
```



pub fn _trigger_script_crc_check_on_player_safe(
        
        
            player: 
        , 
        
        
            p1: 
        , 
        
        
            scriptHash: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x46FB3ED415C7641Cu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x46FB3ED415C7641Cu64;
        
        let result = invoke_raw!(
            hash,
                player, 
                p1, 
                scriptHash
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn _trigger_script_crc_check_on_player_raw(
        player: , 
        p1: , 
        scriptHash: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x46FB3ED415C7641Cu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x46FB3ED415C7641Cu64;

        invoke_raw_typed!(
            hash,
                player, 
                p1, 
                scriptHash
        )
    }
}

/// ## Parameters
*



pub fn network_find_gamers_in_crew_safe(
        
        
            p0: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xE532D6811B3A4D2Au32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xE532D6811B3A4D2Au64;
        
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
pub fn network_find_gamers_in_crew_raw(
        p0: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xE532D6811B3A4D2Au32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xE532D6811B3A4D2Au64;

        invoke_raw_typed!(
            hash,
                p0
        )
    }
}

/// ## Return value



pub fn _0xc32ea7a2f6ca7557_safe(
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xC32EA7A2F6CA7557u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xC32EA7A2F6CA7557u64;
        
        let result = invoke_raw!(
            hash,
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn _0xc32ea7a2f6ca7557_raw(
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xC32EA7A2F6CA7557u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xC32EA7A2F6CA7557u64;

        invoke_raw_typed!(
            hash,
        )
    }
}

/// NETWORK_FINISH_BROADCASTING_DATA native function



pub fn network_finish_broadcasting_data_safe(
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x64F62AFB081E260Du32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x64F62AFB081E260Du64;
        
        let result = invoke_raw!(
            hash,
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn network_finish_broadcasting_data_raw(
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x64F62AFB081E260Du32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x64F62AFB081E260Du64;

        invoke_raw_typed!(
            hash,
        )
    }
}

/// ## Parameters
*



pub fn _0x39917e1b4cb0f911_safe(
        
        
            p0: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x39917E1B4CB0F911u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x39917E1B4CB0F911u64;
        
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
pub fn _0x39917e1b4cb0f911_raw(
        p0: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x39917E1B4CB0F911u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x39917E1B4CB0F911u64;

        invoke_raw_typed!(
            hash,
                p0
        )
    }
}

/// ## Parameters
*



pub fn network_get_player_index_safe(
        
        
            player: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x24FB80D107371267u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x24FB80D107371267u64;
        
        let result = invoke_raw!(
            hash,
                player
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn network_get_player_index_raw(
        player: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x24FB80D107371267u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x24FB80D107371267u64;

        invoke_raw_typed!(
            hash,
                player
        )
    }
}

/// ## Parameters
*



pub fn network_does_entity_exist_with_network_id_safe(
        
        
            netId: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x18A47D074708FD68u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x18A47D074708FD68u64;
        
        let result = invoke_raw!(
            hash,
                netId
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn network_does_entity_exist_with_network_id_raw(
        netId: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x18A47D074708FD68u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x18A47D074708FD68u64;

        invoke_raw_typed!(
            hash,
                netId
        )
    }
}

/// ```
Returns the name of a given player. Returns "**Invalid**" if CPlayerInfo of the given player cannot be retrieved or the player doesn't exist.
```



pub fn network_player_get_name_safe(
        
        
            player: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x7718D2E2060837D2u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x7718D2E2060837D2u64;
        
        let result = invoke_raw!(
            hash,
                player
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn network_player_get_name_raw(
        player: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x7718D2E2060837D2u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x7718D2E2060837D2u64;

        invoke_raw_typed!(
            hash,
                player
        )
    }
}

/// NETWORK_REMOVE_ALL_TRANSITION_INVITE native function



pub fn network_remove_all_transition_invite_safe(
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x726E0375C7A26368u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x726E0375C7A26368u64;
        
        let result = invoke_raw!(
            hash,
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn network_remove_all_transition_invite_raw(
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x726E0375C7A26368u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x726E0375C7A26368u64;

        invoke_raw_typed!(
            hash,
        )
    }
}

/// ## Return value



pub fn network_is_adding_friend_safe(
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x6EA101606F6E4D81u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x6EA101606F6E4D81u64;
        
        let result = invoke_raw!(
            hash,
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn network_is_adding_friend_raw(
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x6EA101606F6E4D81u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x6EA101606F6E4D81u64;

        invoke_raw_typed!(
            hash,
        )
    }
}

/// NETWORK_CACHE_LOCAL_PLAYER_HEAD_BLEND_DATA native function



pub fn network_cache_local_player_head_blend_data_safe(
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xBD0BE0BFC927EAC1u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xBD0BE0BFC927EAC1u64;
        
        let result = invoke_raw!(
            hash,
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn network_cache_local_player_head_blend_data_raw(
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xBD0BE0BFC927EAC1u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xBD0BE0BFC927EAC1u64;

        invoke_raw_typed!(
            hash,
        )
    }
}

/// ## Return value



pub fn is_commerce_store_open_safe(
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x2EAC52B4019E2782u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x2EAC52B4019E2782u64;
        
        let result = invoke_raw!(
            hash,
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn is_commerce_store_open_raw(
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x2EAC52B4019E2782u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x2EAC52B4019E2782u64;

        invoke_raw_typed!(
            hash,
        )
    }
}

/// ## Parameters
*



pub fn network_add_entity_displayed_boundaries_safe(
        
        
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
        let hash = 0x25B99872D588A101u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x25B99872D588A101u64;
        
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
pub fn network_add_entity_displayed_boundaries_raw(
        p0: , 
        p1: , 
        p2: , 
        p3: , 
        p4: , 
        p5: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x25B99872D588A101u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x25B99872D588A101u64;

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



pub fn ugc_query_by_content_ids_safe(
        
        
            data: 
        , 
        
        
            count: 
        , 
        
        
            latestVersion: 
        , 
        
        
            contentTypeName: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xC7397A83F7A2A462u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xC7397A83F7A2A462u64;
        
        let result = invoke_raw!(
            hash,
                data, 
                count, 
                latestVersion, 
                contentTypeName
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn ugc_query_by_content_ids_raw(
        data: , 
        count: , 
        latestVersion: , 
        contentTypeName: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xC7397A83F7A2A462u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xC7397A83F7A2A462u64;

        invoke_raw_typed!(
            hash,
                data, 
                count, 
                latestVersion, 
                contentTypeName
        )
    }
}

/// ## Parameters
*



pub fn _0x0ede326d47cd0f3e_safe(
        
        
            ped: 
        , 
        
        
            player: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x0EDE326D47CD0F3Eu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x0EDE326D47CD0F3Eu64;
        
        let result = invoke_raw!(
            hash,
                ped, 
                player
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn _0x0ede326d47cd0f3e_raw(
        ped: , 
        player: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x0EDE326D47CD0F3Eu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x0EDE326D47CD0F3Eu64;

        invoke_raw_typed!(
            hash,
                ped, 
                player
        )
    }
}

/// ## Return value



pub fn _network_have_online_privilege_2_safe(
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x5EA784D197556507u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x5EA784D197556507u64;
        
        let result = invoke_raw!(
            hash,
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn _network_have_online_privilege_2_raw(
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x5EA784D197556507u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x5EA784D197556507u64;

        invoke_raw_typed!(
            hash,
        )
    }
}

/// ```
int handle[76];  
NETWORK_HANDLE_FROM_FRIEND(iSelectedPlayer, &handle[0], 13);  
Player uVar2 = NETWORK_GET_PLAYER_FROM_GAMER_HANDLE(&handle[0]);  
NETWORK_JOIN_TRANSITION(uVar2);  
		nothing doin.  
```



pub fn network_join_transition_safe(
        
        
            player: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x9D060B08CD63321Au32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x9D060B08CD63321Au64;
        
        let result = invoke_raw!(
            hash,
                player
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn network_join_transition_raw(
        player: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x9D060B08CD63321Au32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x9D060B08CD63321Au64;

        invoke_raw_typed!(
            hash,
                player
        )
    }
}

/// ```
Whether or not another player is allowed to take control of the entity  
```



pub fn set_network_id_can_migrate_safe(
        
        
            netId: 
        , 
        
        
            toggle: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x299EEB23175895FCu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x299EEB23175895FCu64;
        
        let result = invoke_raw!(
            hash,
                netId, 
                toggle
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn set_network_id_can_migrate_raw(
        netId: , 
        toggle: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x299EEB23175895FCu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x299EEB23175895FCu64;

        invoke_raw_typed!(
            hash,
                netId, 
                toggle
        )
    }
}

/// ## Return value



pub fn _0x2bf66d2e7414f686_safe(
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x2BF66D2E7414F686u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x2BF66D2E7414F686u64;
        
        let result = invoke_raw!(
            hash,
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn _0x2bf66d2e7414f686_raw(
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x2BF66D2E7414F686u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x2BF66D2E7414F686u64;

        invoke_raw_typed!(
            hash,
        )
    }
}

/// ## Parameters
*



pub fn _0x4ad490ae1536933b_safe(
        
        
            p0: 
        , 
        
        
            p1: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x4AD490AE1536933Bu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x4AD490AE1536933Bu64;
        
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
pub fn _0x4ad490ae1536933b_raw(
        p0: , 
        p1: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x4AD490AE1536933Bu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x4AD490AE1536933Bu64;

        invoke_raw_typed!(
            hash,
                p0, 
                p1
        )
    }
}

/// ```
normal - transition like when your coming out of LSC  
slow - transition like when you walk into a mission  
```



pub fn network_fade_out_entity_safe(
        
        
            entity: 
        , 
        
        
            normal: 
        , 
        
        
            slow: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xDE564951F95E09EDu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xDE564951F95E09EDu64;
        
        let result = invoke_raw!(
            hash,
                entity, 
                normal, 
                slow
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn network_fade_out_entity_raw(
        entity: , 
        normal: , 
        slow: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xDE564951F95E09EDu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xDE564951F95E09EDu64;

        invoke_raw_typed!(
            hash,
                entity, 
                normal, 
                slow
        )
    }
}

/// ## Parameters
*



pub fn network_player_has_headset_safe(
        
        
            player: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x3FB99A8B08D18FD6u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x3FB99A8B08D18FD6u64;
        
        let result = invoke_raw!(
            hash,
                player
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn network_player_has_headset_raw(
        player: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x3FB99A8B08D18FD6u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x3FB99A8B08D18FD6u64;

        invoke_raw_typed!(
            hash,
                player
        )
    }
}

/// ```
p4 and p5 are always 0 in scripts  
```



pub fn _network_respawn_coords_safe(
        
        
            player: 
        , 
        
        
            x: 
        , 
        
        
            y: 
        , 
        
        
            z: 
        , 
        
        
            p4: 
        , 
        
        
            p5: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x9769F811D1785B03u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x9769F811D1785B03u64;
        
        let result = invoke_raw!(
            hash,
                player, 
                x, 
                y, 
                z, 
                p4, 
                p5
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn _network_respawn_coords_raw(
        player: , 
        x: , 
        y: , 
        z: , 
        p4: , 
        p5: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x9769F811D1785B03u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x9769F811D1785B03u64;

        invoke_raw_typed!(
            hash,
                player, 
                x, 
                y, 
                z, 
                p4, 
                p5
        )
    }
}

/// ## Parameters
*



pub fn set_balance_add_machines_safe(
        
        
            data: 
        , 
        
        
            dataCount: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xB8322EEB38BE7C26u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xB8322EEB38BE7C26u64;
        
        let result = invoke_raw!(
            hash,
                data, 
                dataCount
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn set_balance_add_machines_raw(
        data: , 
        dataCount: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xB8322EEB38BE7C26u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xB8322EEB38BE7C26u64;

        invoke_raw_typed!(
            hash,
                data, 
                dataCount
        )
    }
}

/// ## Return value
Hard-coded to always return 0.



pub fn _0x67fc09bc554a75e5_safe(
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x67FC09BC554A75E5u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x67FC09BC554A75E5u64;
        
        let result = invoke_raw!(
            hash,
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn _0x67fc09bc554a75e5_raw(
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x67FC09BC554A75E5u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x67FC09BC554A75E5u64;

        invoke_raw_typed!(
            hash,
        )
    }
}

/// ```
Same as _NETWORK_GET_AVERAGE_LATENCY_FOR_PLAYER (0xD414BE129BB81B32)
```

```
NativeDB Introduced: v323
```



pub fn _network_get_average_latency_for_player_2_safe(
        
        
            player: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x0E3A041ED6AC2B45u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x0E3A041ED6AC2B45u64;
        
        let result = invoke_raw!(
            hash,
                player
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn _network_get_average_latency_for_player_2_raw(
        player: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x0E3A041ED6AC2B45u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x0E3A041ED6AC2B45u64;

        invoke_raw_typed!(
            hash,
                player
        )
    }
}

/// ## Parameters
*



pub fn ugc_get_content_category_safe(
        
        
            p0: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xA7BAB11E7C9C6C5Au32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xA7BAB11E7C9C6C5Au64;
        
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
pub fn ugc_get_content_category_raw(
        p0: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xA7BAB11E7C9C6C5Au32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xA7BAB11E7C9C6C5Au64;

        invoke_raw_typed!(
            hash,
                p0
        )
    }
}

/// ## Return value



pub fn network_is_offline_invite_pending_safe(
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x74698374C45701D2u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x74698374C45701D2u64;
        
        let result = invoke_raw!(
            hash,
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn network_is_offline_invite_pending_raw(
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x74698374C45701D2u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x74698374C45701D2u64;

        invoke_raw_typed!(
            hash,
        )
    }
}

/// ## Return value
True if there are any pending downloads, false otherwise.



pub fn network_clan_any_download_membership_pending_safe(
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xB3F64A6A91432477u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xB3F64A6A91432477u64;
        
        let result = invoke_raw!(
            hash,
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn network_clan_any_download_membership_pending_raw(
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xB3F64A6A91432477u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xB3F64A6A91432477u64;

        invoke_raw_typed!(
            hash,
        )
    }
}

/// ## Parameters
*



pub fn bad_sport_player_left_detected_safe(
        
        
            networkHandle: 
        , 
        
        
            event: 
        , 
        
        
            amountReceived: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xEC5E3AF5289DCA81u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xEC5E3AF5289DCA81u64;
        
        let result = invoke_raw!(
            hash,
                networkHandle, 
                event, 
                amountReceived
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn bad_sport_player_left_detected_raw(
        networkHandle: , 
        event: , 
        amountReceived: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xEC5E3AF5289DCA81u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xEC5E3AF5289DCA81u64;

        invoke_raw_typed!(
            hash,
                networkHandle, 
                event, 
                amountReceived
        )
    }
}

/// ## Parameters
*



pub fn network_get_presence_invite_from_admin_safe(
        
        
            p0: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x3DBF2DF0AEB7D289u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x3DBF2DF0AEB7D289u64;
        
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
pub fn network_get_presence_invite_from_admin_raw(
        p0: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x3DBF2DF0AEB7D289u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x3DBF2DF0AEB7D289u64;

        invoke_raw_typed!(
            hash,
                p0
        )
    }
}

/// ## Parameters
*



pub fn set_network_cutscene_entities_safe(
        
        
            toggle: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xAAA553E7DD28A457u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xAAA553E7DD28A457u64;
        
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
pub fn set_network_cutscene_entities_raw(
        toggle: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xAAA553E7DD28A457u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xAAA553E7DD28A457u64;

        invoke_raw_typed!(
            hash,
                toggle
        )
    }
}

/// ## Parameters
*



pub fn set_balance_add_machine_safe(
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x815E5E3073DA1D67u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x815E5E3073DA1D67u64;
        
        let result = invoke_raw!(
            hash,
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn set_balance_add_machine_raw(
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x815E5E3073DA1D67u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x815E5E3073DA1D67u64;

        invoke_raw_typed!(
            hash,
        )
    }
}

/// ## Parameters
*



pub fn ugc_get_bookmarked_content_safe(
        
        
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
        let hash = 0xD5A4B59980401588u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xD5A4B59980401588u64;
        
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
pub fn ugc_get_bookmarked_content_raw(
        p0: , 
        p1: , 
        p2: , 
        p3: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xD5A4B59980401588u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xD5A4B59980401588u64;

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



pub fn ugc_get_content_file_version_safe(
        
        
            p0: 
        , 
        
        
            p1: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x37025B27D9B658B1u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x37025B27D9B658B1u64;
        
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
pub fn ugc_get_content_file_version_raw(
        p0: , 
        p1: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x37025B27D9B658B1u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x37025B27D9B658B1u64;

        invoke_raw_typed!(
            hash,
                p0, 
                p1
        )
    }
}

/// ## Parameters
*



pub fn network_is_handle_valid_safe(
        
        
            networkHandle: 
        , 
        
        
            bufferSize: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x6F79B93B0A8E4133u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x6F79B93B0A8E4133u64;
        
        let result = invoke_raw!(
            hash,
                networkHandle, 
                bufferSize
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn network_is_handle_valid_raw(
        networkHandle: , 
        bufferSize: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x6F79B93B0A8E4133u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x6F79B93B0A8E4133u64;

        invoke_raw_typed!(
            hash,
                networkHandle, 
                bufferSize
        )
    }
}

/// NETWORK_START_SOLO_TUTORIAL_SESSION native function



pub fn network_start_solo_tutorial_session_safe(
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x17E0198B3882C2CBu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x17E0198B3882C2CBu64;
        
        let result = invoke_raw!(
            hash,
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn network_start_solo_tutorial_session_raw(
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x17E0198B3882C2CBu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x17E0198B3882C2CBu64;

        invoke_raw_typed!(
            hash,
        )
    }
}

/// ```
Takes the specified time and writes it to the structure specified in the second argument.  
struct date_time  
{ 
    alignas(8) int year;  
    alignas(8) int month;  
    alignas(8) int day;  
    alignas(8) int hour;  
    alignas(8) int minute;  
    alignas(8) int second;  
};
```



pub fn convert_posix_time_safe(
        
        
            posixTime: 
        , 
        
        
            timeStructure: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xAC97AF97FA68E5D5u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xAC97AF97FA68E5D5u64;
        
        let result = invoke_raw!(
            hash,
                posixTime, 
                timeStructure
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn convert_posix_time_raw(
        posixTime: , 
        timeStructure: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xAC97AF97FA68E5D5u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xAC97AF97FA68E5D5u64;

        invoke_raw_typed!(
            hash,
                posixTime, 
                timeStructure
        )
    }
}

/// ```
NativeDB Added Parameter 5: Any p4
NativeDB Added Parameter 6: Any p5
```



pub fn network_do_transition_quickmatch_async_safe(
        
        
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
        let hash = 0xA091A5E44F0072E5u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xA091A5E44F0072E5u64;
        
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
pub fn network_do_transition_quickmatch_async_raw(
        p0: , 
        p1: , 
        p2: , 
        p3: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xA091A5E44F0072E5u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xA091A5E44F0072E5u64;

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



pub fn network_is_player_blocked_by_me_safe(
        
        
            player: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x57AF1F8E27483721u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x57AF1F8E27483721u64;
        
        let result = invoke_raw!(
            hash,
                player
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn network_is_player_blocked_by_me_raw(
        player: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x57AF1F8E27483721u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x57AF1F8E27483721u64;

        invoke_raw_typed!(
            hash,
                player
        )
    }
}

/// ## Parameters
*



pub fn network_set_in_spectator_mode_safe(
        
        
            toggle: 
        , 
        
        
            playerPed: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x423DE3854BB50894u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x423DE3854BB50894u64;
        
        let result = invoke_raw!(
            hash,
                toggle, 
                playerPed
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn network_set_in_spectator_mode_raw(
        toggle: , 
        playerPed: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x423DE3854BB50894u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x423DE3854BB50894u64;

        invoke_raw_typed!(
            hash,
                toggle, 
                playerPed
        )
    }
}

/// ## Return value



pub fn _facebook_set_create_character_complete_safe(
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xDC48473142545431u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xDC48473142545431u64;
        
        let result = invoke_raw!(
            hash,
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn _facebook_set_create_character_complete_raw(
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xDC48473142545431u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xDC48473142545431u64;

        invoke_raw_typed!(
            hash,
        )
    }
}

/// ## Parameters
*



pub fn can_register_mission_objects_safe(
        
        
            amount: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x800DD4721A8B008Bu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x800DD4721A8B008Bu64;
        
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
pub fn can_register_mission_objects_raw(
        amount: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x800DD4721A8B008Bu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x800DD4721A8B008Bu64;

        invoke_raw_typed!(
            hash,
                amount
        )
    }
}

/// ## Parameters
*



pub fn network_get_gamer_status_result_safe(
        
        
            p0: 
        , 
        
        
            p1: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x02A8BEC6FD9AF660u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x02A8BEC6FD9AF660u64;
        
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
pub fn network_get_gamer_status_result_raw(
        p0: , 
        p1: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x02A8BEC6FD9AF660u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x02A8BEC6FD9AF660u64;

        invoke_raw_typed!(
            hash,
                p0, 
                p1
        )
    }
}

/// ## Parameters
*



pub fn network_clan_player_is_active_safe(
        
        
            networkHandle: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xB124B57F571D8F18u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xB124B57F571D8F18u64;
        
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
pub fn network_clan_player_is_active_raw(
        networkHandle: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xB124B57F571D8F18u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xB124B57F571D8F18u64;

        invoke_raw_typed!(
            hash,
                networkHandle
        )
    }
}

/// ```
If you are host, returns true else returns false.
```



pub fn network_is_host_safe(
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x8DB296B814EDDA07u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x8DB296B814EDDA07u64;
        
        let result = invoke_raw!(
            hash,
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn network_is_host_raw(
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x8DB296B814EDDA07u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x8DB296B814EDDA07u64;

        invoke_raw_typed!(
            hash,
        )
    }
}

/// ## Parameters
*



pub fn network_force_local_use_of_synced_scene_camera_safe(
        
        
            sceneId: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xC9B43A33D09CADA7u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xC9B43A33D09CADA7u64;
        
        let result = invoke_raw!(
            hash,
                sceneId
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn network_force_local_use_of_synced_scene_camera_raw(
        sceneId: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xC9B43A33D09CADA7u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xC9B43A33D09CADA7u64;

        invoke_raw_typed!(
            hash,
                sceneId
        )
    }
}

/// ## Return value



pub fn network_gamertag_from_handle_pending_safe(
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xB071E27958EF4CF0u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xB071E27958EF4CF0u64;
        
        let result = invoke_raw!(
            hash,
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn network_gamertag_from_handle_pending_raw(
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xB071E27958EF4CF0u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xB071E27958EF4CF0u64;

        invoke_raw_typed!(
            hash,
        )
    }
}

/// ```
Message is limited to 64 characters.
```



pub fn network_send_text_message_safe(
        
        
            message: 
        , 
        
        
            networkHandle: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x3A214F2EC889B100u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x3A214F2EC889B100u64;
        
        let result = invoke_raw!(
            hash,
                message, 
                networkHandle
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn network_send_text_message_raw(
        message: , 
        networkHandle: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x3A214F2EC889B100u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x3A214F2EC889B100u64;

        invoke_raw_typed!(
            hash,
                message, 
                networkHandle
        )
    }
}

/// ## Return value



pub fn network_is_in_mp_cutscene_safe(
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x6CC27C9FA2040220u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x6CC27C9FA2040220u64;
        
        let result = invoke_raw!(
            hash,
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn network_is_in_mp_cutscene_raw(
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x6CC27C9FA2040220u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x6CC27C9FA2040220u64;

        invoke_raw_typed!(
            hash,
        )
    }
}

/// ## Return value



pub fn network_get_friend_count_safe(
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x203F1CFD823B27A4u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x203F1CFD823B27A4u64;
        
        let result = invoke_raw!(
            hash,
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn network_get_friend_count_raw(
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x203F1CFD823B27A4u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x203F1CFD823B27A4u64;

        invoke_raw_typed!(
            hash,
        )
    }
}

/// ## Parameters
*



pub fn ugc_get_content_has_player_bookmarked_safe(
        
        
            p0: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x993CBE59D350D225u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x993CBE59D350D225u64;
        
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
pub fn ugc_get_content_has_player_bookmarked_raw(
        p0: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x993CBE59D350D225u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x993CBE59D350D225u64;

        invoke_raw_typed!(
            hash,
                p0
        )
    }
}

/// ## Parameters
*



pub fn network_disable_proximity_migration_safe(
        
        
            netID: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x407091CF6037118Eu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x407091CF6037118Eu64;
        
        let result = invoke_raw!(
            hash,
                netID
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn network_disable_proximity_migration_raw(
        netID: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x407091CF6037118Eu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x407091CF6037118Eu64;

        invoke_raw_typed!(
            hash,
                netID
        )
    }
}

/// ## Parameters
*



pub fn _0xc434133d9ba52777_safe(
        
        
            p0: 
        , 
        
        
            TypeOfWeapon: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xC434133D9BA52777u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xC434133D9BA52777u64;
        
        let result = invoke_raw!(
            hash,
                p0, 
                TypeOfWeapon
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn _0xc434133d9ba52777_raw(
        p0: , 
        TypeOfWeapon: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xC434133D9BA52777u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xC434133D9BA52777u64;

        invoke_raw_typed!(
            hash,
                p0, 
                TypeOfWeapon
        )
    }
}

/// ## Parameters
*



pub fn _0x2d5dc831176d0114_safe(
        
        
            p0: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x2D5DC831176D0114u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x2D5DC831176D0114u64;
        
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
pub fn _0x2d5dc831176d0114_raw(
        p0: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x2D5DC831176D0114u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x2D5DC831176D0114u64;

        invoke_raw_typed!(
            hash,
                p0
        )
    }
}

/// NETWORK_CLEAR_GET_GAMER_STATUS native function



pub fn network_clear_get_gamer_status_safe(
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x86E0660E4F5C956Du32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x86E0660E4F5C956Du64;
        
        let result = invoke_raw!(
            hash,
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn network_clear_get_gamer_status_raw(
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x86E0660E4F5C956Du32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x86E0660E4F5C956Du64;

        invoke_raw_typed!(
            hash,
        )
    }
}

/// ## Parameters
*



pub fn network_override_send_restrictions_all_safe(
        
        
            toggle: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x57B192B4D4AD23D5u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x57B192B4D4AD23D5u64;
        
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
pub fn network_override_send_restrictions_all_raw(
        toggle: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x57B192B4D4AD23D5u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x57B192B4D4AD23D5u64;

        invoke_raw_typed!(
            hash,
                toggle
        )
    }
}

/// ## Parameters
*



pub fn ugc_set_deleted_safe(
        
        
            p0: 
        , 
        
        
            p1: 
        , 
        
        
            p2: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xD05D1A6C74DA3498u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xD05D1A6C74DA3498u64;
        
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
pub fn ugc_set_deleted_raw(
        p0: , 
        p1: , 
        p2: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xD05D1A6C74DA3498u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xD05D1A6C74DA3498u64;

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



pub fn network_mark_transition_gamer_as_fully_joined_safe(
        
        
            p0: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x5728BB6D63E3FF1Du32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x5728BB6D63E3FF1Du64;
        
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
pub fn network_mark_transition_gamer_as_fully_joined_raw(
        p0: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x5728BB6D63E3FF1Du32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x5728BB6D63E3FF1Du64;

        invoke_raw_typed!(
            hash,
                p0
        )
    }
}

/// ## Parameters
*



pub fn is_player_in_cutscene_safe(
        
        
            player: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xE73092F4157CD126u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xE73092F4157CD126u64;
        
        let result = invoke_raw!(
            hash,
                player
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn is_player_in_cutscene_raw(
        player: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xE73092F4157CD126u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xE73092F4157CD126u64;

        invoke_raw_typed!(
            hash,
                player
        )
    }
}

/// ## Parameters
*



pub fn network_is_friend_index_online_safe(
        
        
            friendIndex: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xBAD8F2A42B844821u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xBAD8F2A42B844821u64;
        
        let result = invoke_raw!(
            hash,
                friendIndex
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn network_is_friend_index_online_raw(
        friendIndex: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xBAD8F2A42B844821u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xBAD8F2A42B844821u64;

        invoke_raw_typed!(
            hash,
                friendIndex
        )
    }
}

/// ## Return value



pub fn has_network_time_started_safe(
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x46718ACEEDEAFC84u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x46718ACEEDEAFC84u64;
        
        let result = invoke_raw!(
            hash,
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn has_network_time_started_raw(
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x46718ACEEDEAFC84u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x46718ACEEDEAFC84u64;

        invoke_raw_typed!(
            hash,
        )
    }
}

/// ## Return value



pub fn network_have_online_privileges_safe(
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x25CB5A9F37BFD063u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x25CB5A9F37BFD063u64;
        
        let result = invoke_raw!(
            hash,
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn network_have_online_privileges_raw(
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x25CB5A9F37BFD063u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x25CB5A9F37BFD063u64;

        invoke_raw_typed!(
            hash,
        )
    }
}

/// ## Parameters
*



pub fn _0x973d76aa760a6cb6_safe(
        
        
            p0: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x973D76AA760A6CB6u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x973D76AA760A6CB6u64;
        
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
pub fn _0x973d76aa760a6cb6_raw(
        p0: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x973D76AA760A6CB6u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x973D76AA760A6CB6u64;

        invoke_raw_typed!(
            hash,
                p0
        )
    }
}

/// ## Parameters
*



pub fn network_does_tunable_exist_safe(
        
        
            tunableContext: 
        , 
        
        
            tunableName: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x85E5F8B9B898B20Au32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x85E5F8B9B898B20Au64;
        
        let result = invoke_raw!(
            hash,
                tunableContext, 
                tunableName
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn network_does_tunable_exist_raw(
        tunableContext: , 
        tunableName: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x85E5F8B9B898B20Au32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x85E5F8B9B898B20Au64;

        invoke_raw_typed!(
            hash,
                tunableContext, 
                tunableName
        )
    }
}

/// ## Parameters
*



pub fn ugc_get_content_user_name_safe(
        
        
            p0: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x703F12425ECA8BF5u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x703F12425ECA8BF5u64;
        
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
pub fn ugc_get_content_user_name_raw(
        p0: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x703F12425ECA8BF5u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x703F12425ECA8BF5u64;

        invoke_raw_typed!(
            hash,
                p0
        )
    }
}

/// ## Parameters
*



pub fn set_network_id_exists_on_all_machines_safe(
        
        
            netId: 
        , 
        
        
            toggle: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xE05E81A888FA63C8u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xE05E81A888FA63C8u64;
        
        let result = invoke_raw!(
            hash,
                netId, 
                toggle
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn set_network_id_exists_on_all_machines_raw(
        netId: , 
        toggle: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xE05E81A888FA63C8u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xE05E81A888FA63C8u64;

        invoke_raw_typed!(
            hash,
                netId, 
                toggle
        )
    }
}

/// ## Parameters
*



pub fn _activate_damage_tracker_on_player_safe(
        
        
            player: 
        , 
        
        
            toggle: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xBEC0816FF5ACBCDAu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xBEC0816FF5ACBCDAu64;
        
        let result = invoke_raw!(
            hash,
                player, 
                toggle
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn _activate_damage_tracker_on_player_raw(
        player: , 
        toggle: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xBEC0816FF5ACBCDAu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xBEC0816FF5ACBCDAu64;

        invoke_raw_typed!(
            hash,
                player, 
                toggle
        )
    }
}

/// ## Parameters
*



pub fn _network_set_entity_ghosted_with_owner_safe(
        
        
            entity: 
        , 
        
        
            p1: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x4BA166079D658ED4u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x4BA166079D658ED4u64;
        
        let result = invoke_raw!(
            hash,
                entity, 
                p1
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn _network_set_entity_ghosted_with_owner_raw(
        entity: , 
        p1: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x4BA166079D658ED4u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x4BA166079D658ED4u64;

        invoke_raw_typed!(
            hash,
                entity, 
                p1
        )
    }
}

/// ## Parameters
*



pub fn texture_download_get_name_safe(
        
        
            p0: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x3448505B6E35262Du32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x3448505B6E35262Du64;
        
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
pub fn texture_download_get_name_raw(
        p0: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x3448505B6E35262Du32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x3448505B6E35262Du64;

        invoke_raw_typed!(
            hash,
                p0
        )
    }
}

/// ## Parameters
*



pub fn _0x07eab372c8841d99_safe(
        
        
            p0: 
        , 
        
        
            p1: 
        , 
        
        
            p2: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x07EAB372C8841D99u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x07EAB372C8841D99u64;
        
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
pub fn _0x07eab372c8841d99_raw(
        p0: , 
        p1: , 
        p2: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x07EAB372C8841D99u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x07EAB372C8841D99u64;

        invoke_raw_typed!(
            hash,
                p0, 
                p1, 
                p2
        )
    }
}

/// ## Return value



pub fn network_session_is_solo_safe(
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xF3929C2379B60CCEu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xF3929C2379B60CCEu64;
        
        let result = invoke_raw!(
            hash,
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn network_session_is_solo_raw(
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xF3929C2379B60CCEu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xF3929C2379B60CCEu64;

        invoke_raw_typed!(
            hash,
        )
    }
}

/// ## Parameters
*



pub fn ugc_copy_content_safe(
        
        
            p0: 
        , 
        
        
            p1: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x152D90E4C1B4738Au32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x152D90E4C1B4738Au64;
        
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
pub fn ugc_copy_content_raw(
        p0: , 
        p1: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x152D90E4C1B4738Au32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x152D90E4C1B4738Au64;

        invoke_raw_typed!(
            hash,
                p0, 
                p1
        )
    }
}

/// Checks if the networkHandle is the same as any other user that is signed in on the local machine.
For example, if your console has two or more users signed in (on different controllers), the profile that is not controlling the game would be "inactive".



pub fn network_is_inactive_profile_safe(
        
        
            networkHandle: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x7E58745504313A2Eu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x7E58745504313A2Eu64;
        
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
pub fn network_is_inactive_profile_raw(
        networkHandle: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x7E58745504313A2Eu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x7E58745504313A2Eu64;

        invoke_raw_typed!(
            hash,
                networkHandle
        )
    }
}

/// ## Parameters
*



pub fn _network_register_tunable_bool_hash_safe(
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x697F508861875B42u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x697F508861875B42u64;
        
        let result = invoke_raw!(
            hash,
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn _network_register_tunable_bool_hash_raw(
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x697F508861875B42u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x697F508861875B42u64;

        invoke_raw_typed!(
            hash,
        )
    }
}

/// ## Parameters
*



pub fn network_is_gamer_blocked_by_me_safe(
        
        
            networkHandle: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xE944C4F5AF1B5883u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xE944C4F5AF1B5883u64;
        
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
pub fn network_is_gamer_blocked_by_me_raw(
        networkHandle: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xE944C4F5AF1B5883u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xE944C4F5AF1B5883u64;

        invoke_raw_typed!(
            hash,
                networkHandle
        )
    }
}

/// ## Parameters
*



pub fn network_set_currently_selected_gamer_handle_from_invite_menu_safe(
        
        
            p0: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x7206F674F2A3B1BBu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x7206F674F2A3B1BBu64;
        
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
pub fn network_set_currently_selected_gamer_handle_from_invite_menu_raw(
        p0: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x7206F674F2A3B1BBu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x7206F674F2A3B1BBu64;

        invoke_raw_typed!(
            hash,
                p0
        )
    }
}

/// ## Return value



pub fn _0xa12d3a5a3753cc23_safe(
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xA12D3A5A3753CC23u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xA12D3A5A3753CC23u64;
        
        let result = invoke_raw!(
            hash,
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn _0xa12d3a5a3753cc23_raw(
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xA12D3A5A3753CC23u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xA12D3A5A3753CC23u64;

        invoke_raw_typed!(
            hash,
        )
    }
}

/// NETWORK_CANCEL_RESPAWN_SEARCH native function



pub fn network_cancel_respawn_search_safe(
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xFB8F2A6F3DF08CBEu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xFB8F2A6F3DF08CBEu64;
        
        let result = invoke_raw!(
            hash,
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn network_cancel_respawn_search_raw(
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xFB8F2A6F3DF08CBEu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xFB8F2A6F3DF08CBEu64;

        invoke_raw_typed!(
            hash,
        )
    }
}

/// ```
bufferSize is 35 in the scripts.  
```



pub fn network_clan_is_rockstar_clan_safe(
        
        
            clanDesc: 
        , 
        
        
            bufferSize: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x7543BB439F63792Bu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x7543BB439F63792Bu64;
        
        let result = invoke_raw!(
            hash,
                clanDesc, 
                bufferSize
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn network_clan_is_rockstar_clan_raw(
        clanDesc: , 
        bufferSize: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x7543BB439F63792Bu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x7543BB439F63792Bu64;

        invoke_raw_typed!(
            hash,
                clanDesc, 
                bufferSize
        )
    }
}

/// ## Return value



pub fn network_get_num_found_gamers_safe(
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xA1B043EE79A916FBu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xA1B043EE79A916FBu64;
        
        let result = invoke_raw!(
            hash,
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn network_get_num_found_gamers_raw(
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xA1B043EE79A916FBu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xA1B043EE79A916FBu64;

        invoke_raw_typed!(
            hash,
        )
    }
}

/// ## Parameters
*



pub fn network_get_player_loudness_safe(
        
        
            player: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x21A1684A25C2867Fu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x21A1684A25C2867Fu64;
        
        let result = invoke_raw!(
            hash,
                player
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn network_get_player_loudness_raw(
        player: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x21A1684A25C2867Fu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x21A1684A25C2867Fu64;

        invoke_raw_typed!(
            hash,
                player
        )
    }
}

/// ## Return value



pub fn _0xb37e4e6a2388ca7b_safe(
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xB37E4E6A2388CA7Bu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xB37E4E6A2388CA7Bu64;
        
        let result = invoke_raw!(
            hash,
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn _0xb37e4e6a2388ca7b_raw(
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xB37E4E6A2388CA7Bu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xB37E4E6A2388CA7Bu64;

        invoke_raw_typed!(
            hash,
        )
    }
}

/// ```
index is always 18 in scripts
```



pub fn network_has_ros_privilege_safe(
        
        
            index: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xA699957E60D80214u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xA699957E60D80214u64;
        
        let result = invoke_raw!(
            hash,
                index
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn network_has_ros_privilege_raw(
        index: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xA699957E60D80214u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xA699957E60D80214u64;

        invoke_raw_typed!(
            hash,
                index
        )
    }
}

/// ## Parameters
*



pub fn ugc_policies_make_private_safe(
        
        
            p0: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x5CAE833B0EE0C500u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x5CAE833B0EE0C500u64;
        
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
pub fn ugc_policies_make_private_raw(
        p0: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x5CAE833B0EE0C500u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x5CAE833B0EE0C500u64;

        invoke_raw_typed!(
            hash,
                p0
        )
    }
}

/// ## Parameters
*



pub fn is_sphere_visible_to_another_machine_safe(
        
        
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
        let hash = 0xD82CF8E64C8729D8u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xD82CF8E64C8729D8u64;
        
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
pub fn is_sphere_visible_to_another_machine_raw(
        p0: , 
        p1: , 
        p2: , 
        p3: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xD82CF8E64C8729D8u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xD82CF8E64C8729D8u64;

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
IS_*
```



pub fn _0x7ef7649b64d7ff10_safe(
        
        
            entity: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x7EF7649B64D7FF10u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x7EF7649B64D7FF10u64;
        
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
pub fn _0x7ef7649b64d7ff10_raw(
        entity: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x7EF7649B64D7FF10u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x7EF7649B64D7FF10u64;

        invoke_raw_typed!(
            hash,
                entity
        )
    }
}

/// ```
gets the ped id of a network id  
```



pub fn net_to_ped_safe(
        
        
            netHandle: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xBDCD95FC216A8B3Eu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xBDCD95FC216A8B3Eu64;
        
        let result = invoke_raw!(
            hash,
                netHandle
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn net_to_ped_raw(
        netHandle: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xBDCD95FC216A8B3Eu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xBDCD95FC216A8B3Eu64;

        invoke_raw_typed!(
            hash,
                netHandle
        )
    }
}

/// ## Parameters
*



pub fn _network_get_entity_net_script_id_safe(
        
        
            entity: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x815F18AD865F057Fu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x815F18AD865F057Fu64;
        
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
pub fn _network_get_entity_net_script_id_raw(
        entity: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x815F18AD865F057Fu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x815F18AD865F057Fu64;

        invoke_raw_typed!(
            hash,
                entity
        )
    }
}

/// _NETWORK_REPORT_MYSELF native function



pub fn _network_report_myself_safe(
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x5626D9D6810730D5u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x5626D9D6810730D5u64;
        
        let result = invoke_raw!(
            hash,
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn _network_report_myself_raw(
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x5626D9D6810730D5u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x5626D9D6810730D5u64;

        invoke_raw_typed!(
            hash,
        )
    }
}

/// ## Return value



pub fn ugc_get_content_total_safe(
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x769951E2455E2EB5u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x769951E2455E2EB5u64;
        
        let result = invoke_raw!(
            hash,
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn ugc_get_content_total_raw(
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x769951E2455E2EB5u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x769951E2455E2EB5u64;

        invoke_raw_typed!(
            hash,
        )
    }
}

/// NETWORK_END_TUTORIAL_SESSION native function



pub fn network_end_tutorial_session_safe(
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xD0AFAFF5A51D72F7u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xD0AFAFF5A51D72F7u64;
        
        let result = invoke_raw!(
            hash,
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn network_end_tutorial_session_raw(
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xD0AFAFF5A51D72F7u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xD0AFAFF5A51D72F7u64;

        invoke_raw_typed!(
            hash,
        )
    }
}

/// ## Parameters
*



pub fn network_player_index_is_cheater_safe(
        
        
            player: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x565E430DB3B05BECu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x565E430DB3B05BECu64;
        
        let result = invoke_raw!(
            hash,
                player
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn network_player_index_is_cheater_raw(
        player: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x565E430DB3B05BECu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x565E430DB3B05BECu64;

        invoke_raw_typed!(
            hash,
                player
        )
    }
}

/// Returns the same value as [`GetNetworkTime`](#_0x7A5487FE9FAA6B48) in freemode, but as opposed to `GetNetworkTime` it always gets the most recent time, instead of once per tick.

Could be used for benchmarking since it can return times in ticks.



pub fn get_network_time_accurate_safe(
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x89023FBBF9200E9Fu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x89023FBBF9200E9Fu64;
        
        let result = invoke_raw!(
            hash,
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn get_network_time_accurate_raw(
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x89023FBBF9200E9Fu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x89023FBBF9200E9Fu64;

        invoke_raw_typed!(
            hash,
        )
    }
}

/// ```
NativeDB Introduced: v2189
```



pub fn _network_override_clock_milliseconds_per_game_minute_safe(
        
        
            ms: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x42BF1D2E723B6D7Eu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x42BF1D2E723B6D7Eu64;
        
        let result = invoke_raw!(
            hash,
                ms
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn _network_override_clock_milliseconds_per_game_minute_raw(
        ms: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x42BF1D2E723B6D7Eu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x42BF1D2E723B6D7Eu64;

        invoke_raw_typed!(
            hash,
                ms
        )
    }
}

/// ```
p0 is always false and p1 varies.  
NETWORK_SESSION_END(0, 1)  
NETWORK_SESSION_END(0, 0)  
Results in: "Connection to session lost due to an unknown network error. Please return to Grand Theft Auto V and try again later."  
```



pub fn network_session_end_safe(
        
        
            p0: 
        , 
        
        
            p1: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xA02E59562D711006u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xA02E59562D711006u64;
        
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
pub fn network_session_end_raw(
        p0: , 
        p1: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xA02E59562D711006u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xA02E59562D711006u64;

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



pub fn _0x0cf6cc51aa18f0f8_safe(
        
        
            p0: 
        , 
        
        
            p1: 
        , 
        
        
            p2: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x0CF6CC51AA18F0F8u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x0CF6CC51AA18F0F8u64;
        
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
pub fn _0x0cf6cc51aa18f0f8_raw(
        p0: , 
        p1: , 
        p2: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x0CF6CC51AA18F0F8u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x0CF6CC51AA18F0F8u64;

        invoke_raw_typed!(
            hash,
                p0, 
                p1, 
                p2
        )
    }
}

/// ## Return value



pub fn network_get_talker_proximity_safe(
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x84F0F13120B4E098u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x84F0F13120B4E098u64;
        
        let result = invoke_raw!(
            hash,
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn network_get_talker_proximity_raw(
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x84F0F13120B4E098u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x84F0F13120B4E098u64;

        invoke_raw_typed!(
            hash,
        )
    }
}

/// ## Return value



pub fn _network_get_platform_party_unk_safe(
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x01ABCE5E7CBDA196u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x01ABCE5E7CBDA196u64;
        
        let result = invoke_raw!(
            hash,
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn _network_get_platform_party_unk_raw(
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x01ABCE5E7CBDA196u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x01ABCE5E7CBDA196u64;

        invoke_raw_typed!(
            hash,
        )
    }
}

/// ## Parameters
*



pub fn network_session_get_kick_vote_safe(
        
        
            player: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xD6D09A6F32F49EF1u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xD6D09A6F32F49EF1u64;
        
        let result = invoke_raw!(
            hash,
                player
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn network_session_get_kick_vote_raw(
        player: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xD6D09A6F32F49EF1u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xD6D09A6F32F49EF1u64;

        invoke_raw_typed!(
            hash,
                player
        )
    }
}

/// ## Parameters
*



pub fn network_register_host_broadcast_variables_safe(
        
        
            vars: 
        , 
        
        
            numVars: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x3E9B2F01C50DF595u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x3E9B2F01C50DF595u64;
        
        let result = invoke_raw!(
            hash,
                vars, 
                numVars
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn network_register_host_broadcast_variables_raw(
        vars: , 
        numVars: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x3E9B2F01C50DF595u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x3E9B2F01C50DF595u64;

        invoke_raw_typed!(
            hash,
                vars, 
                numVars
        )
    }
}

/// This is what R* uses to hide players in MP interiors.

To manage player visibility with NetworkConcealPlayer, here’s a solid approach:



pub fn network_conceal_player_safe(
        
        
            player: 
        , 
        
        
            toggle: 
        , 
        
        
            bAllowDamagingWhileConcealed: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xBBDF066252829606u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xBBDF066252829606u64;
        
        let result = invoke_raw!(
            hash,
                player, 
                toggle, 
                bAllowDamagingWhileConcealed
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn network_conceal_player_raw(
        player: , 
        toggle: , 
        bAllowDamagingWhileConcealed: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xBBDF066252829606u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xBBDF066252829606u64;

        invoke_raw_typed!(
            hash,
                player, 
                toggle, 
                bAllowDamagingWhileConcealed
        )
    }
}

/// ## Parameters
*



pub fn network_session_activity_quickmatch_safe(
        
        
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
        let hash = 0xBE3E347A87ACEB82u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xBE3E347A87ACEB82u64;
        
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
pub fn network_session_activity_quickmatch_raw(
        p0: , 
        p1: , 
        p2: , 
        p3: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xBE3E347A87ACEB82u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xBE3E347A87ACEB82u64;

        invoke_raw_typed!(
            hash,
                p0, 
                p1, 
                p2, 
                p3
        )
    }
}

/// nullsub, doesn't do anything

```
NativeDB Introduced: v1868
```



pub fn _0x3fc795691834481d_safe(
        
        
            p0: 
        , 
        
        
            p1: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x3FC795691834481Du32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x3FC795691834481Du64;
        
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
pub fn _0x3fc795691834481d_raw(
        p0: , 
        p1: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x3FC795691834481Du32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x3FC795691834481Du64;

        invoke_raw_typed!(
            hash,
                p0, 
                p1
        )
    }
}

/// ## Parameters
*



pub fn network_is_player_connected_safe(
        
        
            player: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x93DC1BE4E1ABE9D1u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x93DC1BE4E1ABE9D1u64;
        
        let result = invoke_raw!(
            hash,
                player
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn network_is_player_connected_raw(
        player: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x93DC1BE4E1ABE9D1u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x93DC1BE4E1ABE9D1u64;

        invoke_raw_typed!(
            hash,
                player
        )
    }
}

/// ## Parameters
*



pub fn get_num_created_mission_vehicles_safe(
        
        
            p0: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x0CD9AB83489430EAu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x0CD9AB83489430EAu64;
        
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
pub fn get_num_created_mission_vehicles_raw(
        p0: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x0CD9AB83489430EAu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x0CD9AB83489430EAu64;

        invoke_raw_typed!(
            hash,
                p0
        )
    }
}

/// ## Parameters
*



pub fn network_register_entity_as_networked_safe(
        
        
            entity: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x06FAACD625D80CAAu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x06FAACD625D80CAAu64;
        
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
pub fn network_register_entity_as_networked_raw(
        entity: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x06FAACD625D80CAAu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x06FAACD625D80CAAu64;

        invoke_raw_typed!(
            hash,
                entity
        )
    }
}

/// ## Parameters
*



pub fn ugc_request_content_data_from_index_safe(
        
        
            p0: 
        , 
        
        
            p1: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x171DF6A0C07FB3DCu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x171DF6A0C07FB3DCu64;
        
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
pub fn ugc_request_content_data_from_index_raw(
        p0: , 
        p1: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x171DF6A0C07FB3DCu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x171DF6A0C07FB3DCu64;

        invoke_raw_typed!(
            hash,
                p0, 
                p1
        )
    }
}

/// ## Parameters
*



pub fn _0x2e0bf682cc778d49_safe(
        
        
            p0: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x2E0BF682CC778D49u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x2E0BF682CC778D49u64;
        
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
pub fn _0x2e0bf682cc778d49_raw(
        p0: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x2E0BF682CC778D49u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x2E0BF682CC778D49u64;

        invoke_raw_typed!(
            hash,
                p0
        )
    }
}

/// ```
Returns the Player associated to a given Ped when in an online session.  
```



pub fn network_get_player_index_from_ped_safe(
        
        
            ped: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x6C0E2E0125610278u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x6C0E2E0125610278u64;
        
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
pub fn network_get_player_index_from_ped_raw(
        ped: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x6C0E2E0125610278u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x6C0E2E0125610278u64;

        invoke_raw_typed!(
            hash,
                ped
        )
    }
}

/// ## Parameters
*



pub fn _0xa7862bc5ed1dfd7e_safe(
        
        
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
        let hash = 0xA7862BC5ED1DFD7Eu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xA7862BC5ED1DFD7Eu64;
        
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
pub fn _0xa7862bc5ed1dfd7e_raw(
        p0: , 
        p1: , 
        p2: , 
        p3: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xA7862BC5ED1DFD7Eu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xA7862BC5ED1DFD7Eu64;

        invoke_raw_typed!(
            hash,
                p0, 
                p1, 
                p2, 
                p3
        )
    }
}

/// ## Return value



pub fn network_are_social_club_policies_current_safe(
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xBA9775570DB788CFu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xBA9775570DB788CFu64;
        
        let result = invoke_raw!(
            hash,
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn network_are_social_club_policies_current_raw(
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xBA9775570DB788CFu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xBA9775570DB788CFu64;

        invoke_raw_typed!(
            hash,
        )
    }
}

/// ## Parameters
*



pub fn get_commerce_item_num_cats_safe(
        
        
            index: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x2A7776C709904AB0u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x2A7776C709904AB0u64;
        
        let result = invoke_raw!(
            hash,
                index
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn get_commerce_item_num_cats_raw(
        index: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x2A7776C709904AB0u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x2A7776C709904AB0u64;

        invoke_raw_typed!(
            hash,
                index
        )
    }
}

/// ## Parameters
*



pub fn _0x94538037ee44f5cf_safe(
        
        
            p0: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x94538037EE44F5CFu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x94538037EE44F5CFu64;
        
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
pub fn _0x94538037ee44f5cf_raw(
        p0: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x94538037EE44F5CFu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x94538037EE44F5CFu64;

        invoke_raw_typed!(
            hash,
                p0
        )
    }
}

/// ## Parameters
*



pub fn network_session_get_matchmaking_group_free_safe(
        
        
            p0: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x56CE820830EF040Bu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x56CE820830EF040Bu64;
        
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
pub fn network_session_get_matchmaking_group_free_raw(
        p0: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x56CE820830EF040Bu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x56CE820830EF040Bu64;

        invoke_raw_typed!(
            hash,
                p0
        )
    }
}

/// ## Return value



pub fn network_is_activity_spectator_safe(
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x12103B9E0C9F92FBu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x12103B9E0C9F92FBu64;
        
        let result = invoke_raw!(
            hash,
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn network_is_activity_spectator_raw(
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x12103B9E0C9F92FBu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x12103B9E0C9F92FBu64;

        invoke_raw_typed!(
            hash,
        )
    }
}

/// ## Return value



pub fn network_have_ros_multiplayer_priv_safe(
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x5F91D5D0B36AA310u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x5F91D5D0B36AA310u64;
        
        let result = invoke_raw!(
            hash,
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn network_have_ros_multiplayer_priv_raw(
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x5F91D5D0B36AA310u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x5F91D5D0B36AA310u64;

        invoke_raw_typed!(
            hash,
        )
    }
}

/// ```
Hardcoded to return zero.
==== PS4 specific info ====
Returns some sort of unavailable reason:
-1 = REASON_INVALID
 0 = REASON_OTHER
 1 = REASON_SYSTEM_UPDATE
 2 = REASON_GAME_UPDATE
 3 = REASON_SIGNED_OUT
 4 = REASON_AGE
 5 = REASON_CONNECTION
=================================
```



pub fn _0x74fb3e29e6d10fa9_safe(
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x74FB3E29E6D10FA9u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x74FB3E29E6D10FA9u64;
        
        let result = invoke_raw!(
            hash,
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn _0x74fb3e29e6d10fa9_raw(
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x74FB3E29E6D10FA9u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x74FB3E29E6D10FA9u64;

        invoke_raw_typed!(
            hash,
        )
    }
}

/// ## Parameters
*



pub fn _network_has_view_gamer_user_content_result_safe(
        
        
            networkHandle: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xCCA4318E1AB03F1Fu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xCCA4318E1AB03F1Fu64;
        
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
pub fn _network_has_view_gamer_user_content_result_raw(
        networkHandle: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xCCA4318E1AB03F1Fu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xCCA4318E1AB03F1Fu64;

        invoke_raw_typed!(
            hash,
                networkHandle
        )
    }
}

/// This native does absolutely nothing, just a nullsub



pub fn network_set_rich_presence_string_safe(
        
        
            p0: 
        , 
        
        
            textLabel: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x3E200C2BCF4164EBu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x3E200C2BCF4164EBu64;
        
        let result = invoke_raw!(
            hash,
                p0, 
                textLabel
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn network_set_rich_presence_string_raw(
        p0: , 
        textLabel: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x3E200C2BCF4164EBu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x3E200C2BCF4164EBu64;

        invoke_raw_typed!(
            hash,
                p0, 
                textLabel
        )
    }
}

/// NETWORK_REQUEST_CLOUD_TUNABLES native function



pub fn network_request_cloud_tunables_safe(
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x42FB3B532D526E6Cu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x42FB3B532D526E6Cu64;
        
        let result = invoke_raw!(
            hash,
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn network_request_cloud_tunables_raw(
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x42FB3B532D526E6Cu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x42FB3B532D526E6Cu64;

        invoke_raw_typed!(
            hash,
        )
    }
}

/// ## Parameters
*



pub fn _0xf6f4383b7c92f11a_safe(
        
        
            p0: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xF6F4383B7C92F11Au32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xF6F4383B7C92F11Au64;
        
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
pub fn _0xf6f4383b7c92f11a_raw(
        p0: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xF6F4383B7C92F11Au32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xF6F4383B7C92F11Au64;

        invoke_raw_typed!(
            hash,
                p0
        )
    }
}

/// ## Parameters
*



pub fn _0x2ce9d95e4051aecd_safe(
        
        
            p0: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x2CE9D95E4051AECDu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x2CE9D95E4051AECDu64;
        
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
pub fn _0x2ce9d95e4051aecd_raw(
        p0: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x2CE9D95E4051AECDu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x2CE9D95E4051AECDu64;

        invoke_raw_typed!(
            hash,
                p0
        )
    }
}

/// _0x140E6A44870A11CE native function



pub fn _0x140e6a44870a11ce_safe(
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x140E6A44870A11CEu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x140E6A44870A11CEu64;
        
        let result = invoke_raw!(
            hash,
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn _0x140e6a44870a11ce_raw(
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x140E6A44870A11CEu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x140E6A44870A11CEu64;

        invoke_raw_typed!(
            hash,
        )
    }
}

/// ## Parameters
*



pub fn get_commerce_item_name_safe(
        
        
            index: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xB4271092CA7EDF48u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xB4271092CA7EDF48u64;
        
        let result = invoke_raw!(
            hash,
                index
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn get_commerce_item_name_raw(
        index: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xB4271092CA7EDF48u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xB4271092CA7EDF48u64;

        invoke_raw_typed!(
            hash,
                index
        )
    }
}

/// ## Return value



pub fn ugc_get_query_result_safe(
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xEDF7F927136C224Bu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xEDF7F927136C224Bu64;
        
        let result = invoke_raw!(
            hash,
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn ugc_get_query_result_raw(
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xEDF7F927136C224Bu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xEDF7F927136C224Bu64;

        invoke_raw_typed!(
            hash,
        )
    }
}

/// ## Return value



pub fn cloud_get_availability_check_result_safe(
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x0B0CC10720653F3Bu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x0B0CC10720653F3Bu64;
        
        let result = invoke_raw!(
            hash,
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn cloud_get_availability_check_result_raw(
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x0B0CC10720653F3Bu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x0B0CC10720653F3Bu64;

        invoke_raw_typed!(
            hash,
        )
    }
}

/// ## Return value



pub fn network_player_get_cheater_reason_safe(
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x172F75B6EE2233BAu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x172F75B6EE2233BAu64;
        
        let result = invoke_raw!(
            hash,
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn network_player_get_cheater_reason_raw(
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x172F75B6EE2233BAu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x172F75B6EE2233BAu64;

        invoke_raw_typed!(
            hash,
        )
    }
}

/// Get the network id of the given entity

An entity network id represents a given entity for other clients, it's a handle shared between clients, can be reconverted to a client entity handle.  
These (entity network IDs) can and will be reused, the network id will not change.
If you need to refer to an entity across machines (clients, or the server), you should use its network id.

Read more at [Network and local IDs](https://docs.fivem.net/docs/scripting-manual/networking/ids/)



pub fn network_get_network_id_from_entity_safe(
        
        
            entity: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xA11700682F3AD45Cu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xA11700682F3AD45Cu64;
        
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
pub fn network_get_network_id_from_entity_raw(
        entity: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xA11700682F3AD45Cu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xA11700682F3AD45Cu64;

        invoke_raw_typed!(
            hash,
                entity
        )
    }
}

/// ```
Hardcoded to return false.
```



pub fn network_is_in_platform_party_chat_safe(
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xFD8B834A8BA05048u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xFD8B834A8BA05048u64;
        
        let result = invoke_raw!(
            hash,
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn network_is_in_platform_party_chat_raw(
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xFD8B834A8BA05048u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xFD8B834A8BA05048u64;

        invoke_raw_typed!(
            hash,
        )
    }
}

/// ```
NativeDB Added Parameter 5: Any p4
NativeDB Added Parameter 6: Any p5
```



pub fn network_do_transition_quickmatch_safe(
        
        
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
        let hash = 0x71FB0EBCD4915D56u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x71FB0EBCD4915D56u64;
        
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
pub fn network_do_transition_quickmatch_raw(
        p0: , 
        p1: , 
        p2: , 
        p3: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x71FB0EBCD4915D56u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x71FB0EBCD4915D56u64;

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



pub fn network_session_change_slots_safe(
        
        
            p0: 
        , 
        
        
            p1: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xB4AB419E0D86ACAEu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xB4AB419E0D86ACAEu64;
        
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
pub fn network_session_change_slots_raw(
        p0: , 
        p1: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xB4AB419E0D86ACAEu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xB4AB419E0D86ACAEu64;

        invoke_raw_typed!(
            hash,
                p0, 
                p1
        )
    }
}

/// ## Return value



pub fn _network_get_ros_privilege_9_safe(
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x66B59CFFD78467AFu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x66B59CFFD78467AFu64;
        
        let result = invoke_raw!(
            hash,
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn _network_get_ros_privilege_9_raw(
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x66B59CFFD78467AFu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x66B59CFFD78467AFu64;

        invoke_raw_typed!(
            hash,
        )
    }
}

/// ## Return value



pub fn network_is_transition_private_safe(
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x5A6AA44FF8E931E6u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x5A6AA44FF8E931E6u64;
        
        let result = invoke_raw!(
            hash,
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn network_is_transition_private_raw(
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x5A6AA44FF8E931E6u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x5A6AA44FF8E931E6u64;

        invoke_raw_typed!(
            hash,
        )
    }
}

/// ```
playerTypes:
0 = regular joiner
4 = spectator
8 = unknown
```



pub fn network_session_set_matchmaking_group_max_safe(
        
        
            playerType: 
        , 
        
        
            playerCount: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x8B6A4DD0AF9CE215u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x8B6A4DD0AF9CE215u64;
        
        let result = invoke_raw!(
            hash,
                playerType, 
                playerCount
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn network_session_set_matchmaking_group_max_raw(
        playerType: , 
        playerCount: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x8B6A4DD0AF9CE215u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x8B6A4DD0AF9CE215u64;

        invoke_raw_typed!(
            hash,
                playerType, 
                playerCount
        )
    }
}

/// ## Return value



pub fn get_max_num_network_vehicles_safe(
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x0AFCE529F69B21FFu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x0AFCE529F69B21FFu64;
        
        let result = invoke_raw!(
            hash,
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn get_max_num_network_vehicles_raw(
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x0AFCE529F69B21FFu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x0AFCE529F69B21FFu64;

        invoke_raw_typed!(
            hash,
        )
    }
}

/// ## Parameters
*



pub fn _0xfae628f1e9adb239_safe(
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xFAE628F1E9ADB239u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xFAE628F1E9ADB239u64;
        
        let result = invoke_raw!(
            hash,
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn _0xfae628f1e9adb239_raw(
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xFAE628F1E9ADB239u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xFAE628F1E9ADB239u64;

        invoke_raw_typed!(
            hash,
        )
    }
}

/// ```
One of the first things it does is get the players ped.  
Then it calls a function that is used in some tasks and ped based functions.  
```

p5, p6, p7 is another coordinate (or zero), often related to ``GET_BLIP_COORDS``, in the decompiled scripts.



pub fn network_start_respawn_search_for_player_safe(
        
        
            player: 
        , 
        
        
            x: 
        , 
        
        
            y: 
        , 
        
        
            z: 
        , 
        
        
            radius: 
        , 
        
        
            p5: 
        , 
        
        
            p6: 
        , 
        
        
            p7: 
        , 
        
        
            flags: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x5A6FFA2433E2F14Cu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x5A6FFA2433E2F14Cu64;
        
        let result = invoke_raw!(
            hash,
                player, 
                x, 
                y, 
                z, 
                radius, 
                p5, 
                p6, 
                p7, 
                flags
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn network_start_respawn_search_for_player_raw(
        player: , 
        x: , 
        y: , 
        z: , 
        radius: , 
        p5: , 
        p6: , 
        p7: , 
        flags: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x5A6FFA2433E2F14Cu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x5A6FFA2433E2F14Cu64;

        invoke_raw_typed!(
            hash,
                player, 
                x, 
                y, 
                z, 
                radius, 
                p5, 
                p6, 
                p7, 
                flags
        )
    }
}

/// _0x4C2A9FDC22377075 native function



pub fn _0x4c2a9fdc22377075_safe(
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x4C2A9FDC22377075u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x4C2A9FDC22377075u64;
        
        let result = invoke_raw!(
            hash,
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn _0x4c2a9fdc22377075_raw(
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x4C2A9FDC22377075u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x4C2A9FDC22377075u64;

        invoke_raw_typed!(
            hash,
        )
    }
}

/// ```
In the console script dumps, this is only referenced once.   
NETWORK::NETWORK_EXPLODE_VEHICLE(vehicle, 1, 0, 0);  
^^^^^ That must be PC script dumps? In X360 Script Dumps it is reference a few times with 2 differences in the parameters.  
Which as you see below is 1, 0, 0 + 1, 1, 0 + 1, 0, and a *param?  
am_plane_takedown.c   
network_explode_vehicle(net_to_veh(Local_40.imm_2), 1, 1, 0);  
armenian2.c   
network_explode_vehicle(Local_80[6 <2>], 1, 0, 0);  
fm_horde_controler.c  
network_explode_vehicle(net_to_veh(*uParam0), 1, 0, *uParam0);  
fm_mission_controller.c, has 6 hits so not going to list them.  
Side note, setting the first parameter to 0 seems to mute sound or so?  
Seems it's like ADD_EXPLOSION, etc. the first 2 params. The 3rd atm no need to worry since it always seems to be 0.  
```



pub fn network_explode_vehicle_safe(
        
        
            vehicle: 
        , 
        
        
            isAudible: 
        , 
        
        
            isInvisible: 
        , 
        
        
            p3: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x301A42153C9AD707u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x301A42153C9AD707u64;
        
        let result = invoke_raw!(
            hash,
                vehicle, 
                isAudible, 
                isInvisible, 
                p3
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn network_explode_vehicle_raw(
        vehicle: , 
        isAudible: , 
        isInvisible: , 
        p3: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x301A42153C9AD707u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x301A42153C9AD707u64;

        invoke_raw_typed!(
            hash,
                vehicle, 
                isAudible, 
                isInvisible, 
                p3
        )
    }
}

/// Formerly incorrectly named `USE_PLAYER_COLOUR_INSTEAD_OF_TEAM_COLOUR` due to incorrect treatment of console vs. PC native registration.

Native name guessed through ordering.

```
NativeDB Added Parameter 2: BOOL p1
```



pub fn _set_local_player_as_ghost_safe(
        
        
            toggle: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x5FFE9B4144F9712Fu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x5FFE9B4144F9712Fu64;
        
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
pub fn _set_local_player_as_ghost_raw(
        toggle: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x5FFE9B4144F9712Fu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x5FFE9B4144F9712Fu64;

        invoke_raw_typed!(
            hash,
                toggle
        )
    }
}

/// ## Return value



pub fn network_is_multiplayer_disabled_safe(
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x9747292807126EDAu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x9747292807126EDAu64;
        
        let result = invoke_raw!(
            hash,
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn network_is_multiplayer_disabled_raw(
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x9747292807126EDAu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x9747292807126EDAu64;

        invoke_raw_typed!(
            hash,
        )
    }
}

/// ## Parameters
*



pub fn _0x702bc4d605522539_safe(
        
        
            p0: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x702BC4D605522539u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x702BC4D605522539u64;
        
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
pub fn _0x702bc4d605522539_raw(
        p0: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x702BC4D605522539u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x702BC4D605522539u64;

        invoke_raw_typed!(
            hash,
                p0
        )
    }
}

/// ## Return value



pub fn network_player_is_badsport_safe(
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x19D8DA0E5A68045Au32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x19D8DA0E5A68045Au64;
        
        let result = invoke_raw!(
            hash,
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn network_player_is_badsport_raw(
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x19D8DA0E5A68045Au32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x19D8DA0E5A68045Au64;

        invoke_raw_typed!(
            hash,
        )
    }
}

/// NETWORK_SESSION_LEAVE_SINGLE_PLAYER native function



pub fn network_session_leave_single_player_safe(
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x3442775428FD2DAAu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x3442775428FD2DAAu64;
        
        let result = invoke_raw!(
            hash,
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn network_session_leave_single_player_raw(
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x3442775428FD2DAAu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x3442775428FD2DAAu64;

        invoke_raw_typed!(
            hash,
        )
    }
}

/// ```
Retrieves the local player's NetworkHandle* and stores it in the given buffer.  
* Currently unknown struct  
```



pub fn network_get_local_handle_safe(
        
        
            networkHandle: 
        , 
        
        
            bufferSize: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xE86051786B66CD8Eu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xE86051786B66CD8Eu64;
        
        let result = invoke_raw!(
            hash,
                networkHandle, 
                bufferSize
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn network_get_local_handle_raw(
        networkHandle: , 
        bufferSize: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xE86051786B66CD8Eu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xE86051786B66CD8Eu64;

        invoke_raw_typed!(
            hash,
                networkHandle, 
                bufferSize
        )
    }
}

/// ## Return value



pub fn network_have_ros_banned_priv_safe(
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x8020A73847E0CA7Du32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x8020A73847E0CA7Du64;
        
        let result = invoke_raw!(
            hash,
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn network_have_ros_banned_priv_raw(
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x8020A73847E0CA7Du32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x8020A73847E0CA7Du64;

        invoke_raw_typed!(
            hash,
        )
    }
}

/// ```
NETWORK_GET_*

NativeDB Introduced: v323
```



pub fn _network_get_oldest_resend_count_for_player_safe(
        
        
            player: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x52C1EADAF7B10302u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x52C1EADAF7B10302u64;
        
        let result = invoke_raw!(
            hash,
                player
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn _network_get_oldest_resend_count_for_player_raw(
        player: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x52C1EADAF7B10302u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x52C1EADAF7B10302u64;

        invoke_raw_typed!(
            hash,
                player
        )
    }
}

/// ## Parameters
*



pub fn is_sphere_visible_to_player_safe(
        
        
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
        let hash = 0xDC3A310219E5DA62u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xDC3A310219E5DA62u64;
        
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
pub fn is_sphere_visible_to_player_raw(
        p0: , 
        p1: , 
        p2: , 
        p3: , 
        p4: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xDC3A310219E5DA62u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xDC3A310219E5DA62u64;

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
gets 2 floats from the CNetGamePlayer of p0 and stores them in p1 and p2.  
Possibly waypoint?  
```



pub fn _0xadb57e5b663cca8b_safe(
        
        
            p0: 
        , 
        
        
            p1: 
        , 
        
        
            p2: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xADB57E5B663CCA8Bu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xADB57E5B663CCA8Bu64;
        
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
pub fn _0xadb57e5b663cca8b_raw(
        p0: , 
        p1: , 
        p2: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xADB57E5B663CCA8Bu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xADB57E5B663CCA8Bu64;

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



pub fn network_find_matched_gamers_safe(
        
        
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
        let hash = 0xF7B2CFDE5C9F700Du32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xF7B2CFDE5C9F700Du64;
        
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
pub fn network_find_matched_gamers_raw(
        p0: , 
        p1: , 
        p2: , 
        p3: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xF7B2CFDE5C9F700Du32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xF7B2CFDE5C9F700Du64;

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



pub fn network_is_friend_safe(
        
        
            networkHandle: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x1A24A179F9B31654u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x1A24A179F9B31654u64;
        
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
pub fn network_is_friend_raw(
        networkHandle: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x1A24A179F9B31654u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x1A24A179F9B31654u64;

        invoke_raw_typed!(
            hash,
                networkHandle
        )
    }
}

/// ## Return value



pub fn _facebook_is_available_safe(
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x43865688AE10F0D7u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x43865688AE10F0D7u64;
        
        let result = invoke_raw!(
            hash,
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn _facebook_is_available_raw(
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x43865688AE10F0D7u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x43865688AE10F0D7u64;

        invoke_raw_typed!(
            hash,
        )
    }
}

/// ## Return value



pub fn _facebook_do_unk_check_safe(
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xA75E2B6733DA5142u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xA75E2B6733DA5142u64;
        
        let result = invoke_raw!(
            hash,
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn _facebook_do_unk_check_raw(
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xA75E2B6733DA5142u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xA75E2B6733DA5142u64;

        invoke_raw_typed!(
            hash,
        )
    }
}

/// ## Parameters
*



pub fn get_commerce_product_price_safe(
        
        
            index: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xCA94551B50B4932Cu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xCA94551B50B4932Cu64;
        
        let result = invoke_raw!(
            hash,
                index
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn get_commerce_product_price_raw(
        index: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xCA94551B50B4932Cu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xCA94551B50B4932Cu64;

        invoke_raw_typed!(
            hash,
                index
        )
    }
}

/// ## Parameters
*



pub fn network_does_network_id_exist_safe(
        
        
            netId: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x38CE16C96BD11344u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x38CE16C96BD11344u64;
        
        let result = invoke_raw!(
            hash,
                netId
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn network_does_network_id_exist_raw(
        netId: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x38CE16C96BD11344u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x38CE16C96BD11344u64;

        invoke_raw_typed!(
            hash,
                netId
        )
    }
}

/// ## Parameters
*



pub fn network_get_friend_name_safe(
        
        
            friendIndex: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xE11EBBB2A783FE8Bu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xE11EBBB2A783FE8Bu64;
        
        let result = invoke_raw!(
            hash,
                friendIndex
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn network_get_friend_name_raw(
        friendIndex: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xE11EBBB2A783FE8Bu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xE11EBBB2A783FE8Bu64;

        invoke_raw_typed!(
            hash,
                friendIndex
        )
    }
}

/// ## Parameters
*



pub fn network_change_transition_slots_safe(
        
        
            p0: 
        , 
        
        
            p1: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xEEEDA5E6D7080987u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xEEEDA5E6D7080987u64;
        
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
pub fn network_change_transition_slots_raw(
        p0: , 
        p1: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xEEEDA5E6D7080987u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xEEEDA5E6D7080987u64;

        invoke_raw_typed!(
            hash,
                p0, 
                p1
        )
    }
}

/// NETWORK_SET_*

```
NativeDB Introduced: v1734
```



pub fn _0x8ef52acaecc51d9c_safe(
        
        
            toggle: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x8EF52ACAECC51D9Cu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x8EF52ACAECC51D9Cu64;
        
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
pub fn _0x8ef52acaecc51d9c_raw(
        toggle: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x8EF52ACAECC51D9Cu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x8EF52ACAECC51D9Cu64;

        invoke_raw_typed!(
            hash,
                toggle
        )
    }
}

/// ```
Appears to set whether a transition should be started when the session is migrating.

NETWORK_SET_*
```



pub fn _0xa2e9c1ab8a92e8cd_safe(
        
        
            toggle: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xA2E9C1AB8A92E8CDu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xA2E9C1AB8A92E8CDu64;
        
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
pub fn _0xa2e9c1ab8a92e8cd_raw(
        toggle: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xA2E9C1AB8A92E8CDu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xA2E9C1AB8A92E8CDu64;

        invoke_raw_typed!(
            hash,
                toggle
        )
    }
}

/// ## Parameters
*



pub fn set_local_player_invisible_locally_safe(
        
        
            p0: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xE5F773C1A1D9D168u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xE5F773C1A1D9D168u64;
        
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
pub fn set_local_player_invisible_locally_raw(
        p0: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xE5F773C1A1D9D168u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xE5F773C1A1D9D168u64;

        invoke_raw_typed!(
            hash,
                p0
        )
    }
}

/// ## Parameters
*



pub fn network_set_talker_proximity_safe(
        
        
            value: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xCBF12D65F95AD686u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xCBF12D65F95AD686u64;
        
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
pub fn network_set_talker_proximity_raw(
        value: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xCBF12D65F95AD686u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xCBF12D65F95AD686u64;

        invoke_raw_typed!(
            hash,
                value
        )
    }
}

/// ## Parameters
*



pub fn network_clan_get_emblem_txd_name_safe(
        
        
            netHandle: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x5835D9CD92E83184u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x5835D9CD92E83184u64;
        
        let result = invoke_raw!(
            hash,
                netHandle
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn network_clan_get_emblem_txd_name_raw(
        netHandle: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x5835D9CD92E83184u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x5835D9CD92E83184u64;

        invoke_raw_typed!(
            hash,
                netHandle
        )
    }
}

/// ```
gets the network id of a ped  
```



pub fn ped_to_net_safe(
        
        
            ped: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x0EDEC3C276198689u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x0EDEC3C276198689u64;
        
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
pub fn ped_to_net_raw(
        ped: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x0EDEC3C276198689u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x0EDEC3C276198689u64;

        invoke_raw_typed!(
            hash,
                ped
        )
    }
}

/// ## Return value



pub fn network_session_was_invited_safe(
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x23DFB504655D0CE4u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x23DFB504655D0CE4u64;
        
        let result = invoke_raw!(
            hash,
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn network_session_was_invited_raw(
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x23DFB504655D0CE4u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x23DFB504655D0CE4u64;

        invoke_raw_typed!(
            hash,
        )
    }
}

/// ## Parameters
*



pub fn network_session_set_matchmaking_group_safe(
        
        
            matchmakingGroup: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x49EC8030F5015F8Bu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x49EC8030F5015F8Bu64;
        
        let result = invoke_raw!(
            hash,
                matchmakingGroup
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn network_session_set_matchmaking_group_raw(
        matchmakingGroup: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x49EC8030F5015F8Bu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x49EC8030F5015F8Bu64;

        invoke_raw_typed!(
            hash,
                matchmakingGroup
        )
    }
}

/// ## Return value



pub fn network_is_transition_matchmaking_safe(
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x292564C735375EDFu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x292564C735375EDFu64;
        
        let result = invoke_raw!(
            hash,
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn network_is_transition_matchmaking_raw(
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x292564C735375EDFu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x292564C735375EDFu64;

        invoke_raw_typed!(
            hash,
        )
    }
}

/// ## Parameters
*



pub fn network_set_team_only_chat_safe(
        
        
            toggle: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xD5B4883AC32F24C3u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xD5B4883AC32F24C3u64;
        
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
pub fn network_set_team_only_chat_raw(
        toggle: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xD5B4883AC32F24C3u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xD5B4883AC32F24C3u64;

        invoke_raw_typed!(
            hash,
                toggle
        )
    }
}

/// ## Parameters
*



pub fn network_override_transition_chat_safe(
        
        
            p0: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xAF66059A131AA269u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xAF66059A131AA269u64;
        
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
pub fn network_override_transition_chat_raw(
        p0: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xAF66059A131AA269u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xAF66059A131AA269u64;

        invoke_raw_typed!(
            hash,
                p0
        )
    }
}

/// Checks if the friendDataIndex in the friend data manager contains the data for the specified networkHandle



pub fn network_check_data_manager_succeeded_for_handle_safe(
        
        
            friendDataIndex: 
        , 
        
        
            networkHandle: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x44B37CDCAE765AAEu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x44B37CDCAE765AAEu64;
        
        let result = invoke_raw!(
            hash,
                friendDataIndex, 
                networkHandle
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn network_check_data_manager_succeeded_for_handle_raw(
        friendDataIndex: , 
        networkHandle: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x44B37CDCAE765AAEu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x44B37CDCAE765AAEu64;

        invoke_raw_typed!(
            hash,
                friendDataIndex, 
                networkHandle
        )
    }
}

/// ## Return value



pub fn ugc_get_content_hash_safe(
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x3A17A27D75C74887u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x3A17A27D75C74887u64;
        
        let result = invoke_raw!(
            hash,
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn ugc_get_content_hash_raw(
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x3A17A27D75C74887u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x3A17A27D75C74887u64;

        invoke_raw_typed!(
            hash,
        )
    }
}

/// ## Parameters
*



pub fn network_can_communicate_with_gamer_safe(
        
        
            networkHandle: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xA150A4F065806B1Fu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xA150A4F065806B1Fu64;
        
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
pub fn network_can_communicate_with_gamer_raw(
        networkHandle: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xA150A4F065806B1Fu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xA150A4F065806B1Fu64;

        invoke_raw_typed!(
            hash,
                networkHandle
        )
    }
}

/// Returns POSIX timestamp.

Renamed from `_GET_POSIX_TIME` to `GET_CLOUD_TIME_AS_INT` because of conflicting native names ([`0xDA488F299A5B164E`](#_0xDA488F299A5B164E))



pub fn get_cloud_time_as_int_safe(
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x9A73240B49945C76u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x9A73240B49945C76u64;
        
        let result = invoke_raw!(
            hash,
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn get_cloud_time_as_int_raw(
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x9A73240B49945C76u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x9A73240B49945C76u64;

        invoke_raw_typed!(
            hash,
        )
    }
}

/// ## Return value



pub fn _0x4237e822315d8ba9_safe(
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x4237E822315D8BA9u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x4237E822315D8BA9u64;
        
        let result = invoke_raw!(
            hash,
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn _0x4237e822315d8ba9_raw(
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x4237E822315D8BA9u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x4237E822315D8BA9u64;

        invoke_raw_typed!(
            hash,
        )
    }
}

/// ```
On PC it's a nullsub which means it does absolutely nothing.  
Now that Discord supports Rich Presence, R* might finally implement this for PC. Or maybe in future games like RDR2, GTA VI...  
```



pub fn network_set_rich_presence_safe(
        
        
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
        let hash = 0x1DCCACDCFC569362u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x1DCCACDCFC569362u64;
        
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
pub fn network_set_rich_presence_raw(
        p0: , 
        p1: , 
        p2: , 
        p3: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x1DCCACDCFC569362u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x1DCCACDCFC569362u64;

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



pub fn _network_conceal_entity_safe(
        
        
            entity: 
        , 
        
        
            toggle: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x1632BE0AC1E62876u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x1632BE0AC1E62876u64;
        
        let result = invoke_raw!(
            hash,
                entity, 
                toggle
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn _network_conceal_entity_raw(
        entity: , 
        toggle: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x1632BE0AC1E62876u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x1632BE0AC1E62876u64;

        invoke_raw_typed!(
            hash,
                entity, 
                toggle
        )
    }
}

/// ```
Checks if a specific value (BYTE) in CPlayerInfo is nonzero.  
Returns always false in Singleplayer.  
No longer used for dev checks since first mods were released on PS3 & 360.  
R* now checks with the is_dlc_present native for the dlc hash 2532323046,  
if that is present it will unlock dev stuff.  
```



pub fn network_player_is_rockstar_dev_safe(
        
        
            player: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x544ABDDA3B409B6Du32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x544ABDDA3B409B6Du64;
        
        let result = invoke_raw!(
            hash,
                player
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn network_player_is_rockstar_dev_raw(
        player: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x544ABDDA3B409B6Du32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x544ABDDA3B409B6Du64;

        invoke_raw_typed!(
            hash,
                player
        )
    }
}

/// ```
Does nothing (it's a nullsub).

NativeDB Introduced: v323
```



pub fn _0x2555cf7da5473794_safe(
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x2555CF7DA5473794u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x2555CF7DA5473794u64;
        
        let result = invoke_raw!(
            hash,
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn _0x2555cf7da5473794_raw(
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x2555CF7DA5473794u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x2555CF7DA5473794u64;

        invoke_raw_typed!(
            hash,
        )
    }
}

/// ## Parameters
*



pub fn network_request_control_of_network_id_safe(
        
        
            netId: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xA670B3662FAFFBD0u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xA670B3662FAFFBD0u64;
        
        let result = invoke_raw!(
            hash,
                netId
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn network_request_control_of_network_id_raw(
        netId: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xA670B3662FAFFBD0u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xA670B3662FAFFBD0u64;

        invoke_raw_typed!(
            hash,
                netId
        )
    }
}

/// ## Return value



pub fn network_get_num_participants_safe(
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x18D0456E86604654u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x18D0456E86604654u64;
        
        let result = invoke_raw!(
            hash,
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn network_get_num_participants_raw(
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x18D0456E86604654u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x18D0456E86604654u64;

        invoke_raw_typed!(
            hash,
        )
    }
}

/// ## Return value



pub fn network_is_in_transition_safe(
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x68049AEFF83D8F0Au32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x68049AEFF83D8F0Au64;
        
        let result = invoke_raw!(
            hash,
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn network_is_in_transition_raw(
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x68049AEFF83D8F0Au32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x68049AEFF83D8F0Au64;

        invoke_raw_typed!(
            hash,
        )
    }
}

/// ```
Checks some commerce stuff

NativeDB Introduced: v1290
```



pub fn _0x754615490a029508_safe(
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x754615490A029508u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x754615490A029508u64;
        
        let result = invoke_raw!(
            hash,
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn _0x754615490a029508_raw(
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x754615490A029508u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x754615490A029508u64;

        invoke_raw_typed!(
            hash,
        )
    }
}

/// ## Parameters
*



pub fn ugc_get_content_language_safe(
        
        
            p0: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x32DD916F3F7C9672u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x32DD916F3F7C9672u64;
        
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
pub fn ugc_get_content_language_raw(
        p0: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x32DD916F3F7C9672u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x32DD916F3F7C9672u64;

        invoke_raw_typed!(
            hash,
                p0
        )
    }
}

/// ## Parameters
*



pub fn _is_damage_tracker_active_on_player_safe(
        
        
            player: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xB2092A1EAA7FD45Fu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xB2092A1EAA7FD45Fu64;
        
        let result = invoke_raw!(
            hash,
                player
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn _is_damage_tracker_active_on_player_raw(
        player: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xB2092A1EAA7FD45Fu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xB2092A1EAA7FD45Fu64;

        invoke_raw_typed!(
            hash,
                player
        )
    }
}

/// ## Parameters
*



pub fn _network_is_this_script_marked_safe(
        
        
            p0: 
        , 
        
        
            p1: 
        , 
        
        
            p2: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xD1110739EEADB592u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xD1110739EEADB592u64;
        
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
pub fn _network_is_this_script_marked_raw(
        p0: , 
        p1: , 
        p2: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xD1110739EEADB592u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xD1110739EEADB592u64;

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



pub fn network_has_control_of_network_id_safe(
        
        
            netId: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x4D36070FE0215186u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x4D36070FE0215186u64;
        
        let result = invoke_raw!(
            hash,
                netId
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn network_has_control_of_network_id_raw(
        netId: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x4D36070FE0215186u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x4D36070FE0215186u64;

        invoke_raw_typed!(
            hash,
                netId
        )
    }
}

/// ## Parameters
*



pub fn set_network_id_visible_in_cutscene_safe(
        
        
            netId: 
        , 
        
        
            p1: 
        , 
        
        
            p2: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xA6928482543022B4u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xA6928482543022B4u64;
        
        let result = invoke_raw!(
            hash,
                netId, 
                p1, 
                p2
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn set_network_id_visible_in_cutscene_raw(
        netId: , 
        p1: , 
        p2: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xA6928482543022B4u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xA6928482543022B4u64;

        invoke_raw_typed!(
            hash,
                netId, 
                p1, 
                p2
        )
    }
}

/// ## Return value



pub fn _0xa8acb6459542a8c8_safe(
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xA8ACB6459542A8C8u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xA8ACB6459542A8C8u64;
        
        let result = invoke_raw!(
            hash,
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn _0xa8acb6459542a8c8_raw(
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xA8ACB6459542A8C8u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xA8ACB6459542A8C8u64;

        invoke_raw_typed!(
            hash,
        )
    }
}

/// ## Parameters
*



pub fn network_session_add_active_matchmaking_group_safe(
        
        
            groupId: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xCAE55F48D3D7875Cu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xCAE55F48D3D7875Cu64;
        
        let result = invoke_raw!(
            hash,
                groupId
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn network_session_add_active_matchmaking_group_raw(
        groupId: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xCAE55F48D3D7875Cu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xCAE55F48D3D7875Cu64;

        invoke_raw_typed!(
            hash,
                groupId
        )
    }
}

/// NETWORK_RESET_BODY_TRACKER native function



pub fn network_reset_body_tracker_safe(
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x72433699B4E6DD64u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x72433699B4E6DD64u64;
        
        let result = invoke_raw!(
            hash,
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn network_reset_body_tracker_raw(
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x72433699B4E6DD64u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x72433699B4E6DD64u64;

        invoke_raw_typed!(
            hash,
        )
    }
}

/// ## Parameters
*



pub fn get_time_as_string_safe(
        
        
            time: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x9E23B1777A927DADu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x9E23B1777A927DADu64;
        
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
pub fn get_time_as_string_raw(
        time: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x9E23B1777A927DADu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x9E23B1777A927DADu64;

        invoke_raw_typed!(
            hash,
                time
        )
    }
}

/// ## Parameters
*



pub fn network_register_player_broadcast_variables_safe(
        
        
            vars: 
        , 
        
        
            numVars: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x3364AA97340CA215u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x3364AA97340CA215u64;
        
        let result = invoke_raw!(
            hash,
                vars, 
                numVars
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn network_register_player_broadcast_variables_raw(
        vars: , 
        numVars: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x3364AA97340CA215u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x3364AA97340CA215u64;

        invoke_raw_typed!(
            hash,
                vars, 
                numVars
        )
    }
}

/// ## Parameters
*



pub fn cloud_delete_member_file_safe(
        
        
            p0: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xC64DED7EF0D2FE37u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xC64DED7EF0D2FE37u64;
        
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
pub fn cloud_delete_member_file_raw(
        p0: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xC64DED7EF0D2FE37u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xC64DED7EF0D2FE37u64;

        invoke_raw_typed!(
            hash,
                p0
        )
    }
}

/// ```
unknown params  
p0 = 0, 2, or 999 (The global is 999 by default.)  
p1 = 0 (Always in every script it's found in atleast.)  
p2 = 0, 3, or 4 (Based on a var that is determined by a function.)  
p3 = maxPlayers (It's obvious in x360 scripts it's always 18)  
p4 = 0 (Always in every script it's found in atleast.)  
p5 = 0 or 1. (1 if network_can_enter_multiplayer, but set to 0 if other checks after that are passed.)  
p5 is reset to 0 if,  
Global_1315318 = 0 or Global_1315323 = 9 or 12 or (Global_1312629 = 0 && Global_1312631 = true/1) those are passed.  
```



pub fn network_session_enter_safe(
        
        
            p0: 
        , 
        
        
            p1: 
        , 
        
        
            p2: 
        , 
        
        
            maxPlayers: 
        , 
        
        
            p4: 
        , 
        
        
            p5: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x330ED4D05491934Fu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x330ED4D05491934Fu64;
        
        let result = invoke_raw!(
            hash,
                p0, 
                p1, 
                p2, 
                maxPlayers, 
                p4, 
                p5
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn network_session_enter_raw(
        p0: , 
        p1: , 
        p2: , 
        maxPlayers: , 
        p4: , 
        p5: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x330ED4D05491934Fu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x330ED4D05491934Fu64;

        invoke_raw_typed!(
            hash,
                p0, 
                p1, 
                p2, 
                maxPlayers, 
                p4, 
                p5
        )
    }
}

/// ```
Has a 3rd param (int) since patch [???].  
```

```
NativeDB Added Parameter 3: int p2
```



pub fn open_commerce_store_safe(
        
        
            p0: 
        , 
        
        
            p1: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x58C21165F6545892u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x58C21165F6545892u64;
        
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
pub fn open_commerce_store_raw(
        p0: , 
        p1: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x58C21165F6545892u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x58C21165F6545892u64;

        invoke_raw_typed!(
            hash,
                p0, 
                p1
        )
    }
}

/// ```
Return the root content id of a job.
```



pub fn ugc_get_root_content_id_safe(
        
        
            p0: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xC0173D6BFF4E0348u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xC0173D6BFF4E0348u64;
        
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
pub fn ugc_get_root_content_id_raw(
        p0: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xC0173D6BFF4E0348u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xC0173D6BFF4E0348u64;

        invoke_raw_typed!(
            hash,
                p0
        )
    }
}

/// ## Parameters
*



pub fn texture_download_request_safe(
        
        
            PlayerHandle: 
        , 
        
        
            FilePath: 
        , 
        
        
            Name: 
        , 
        
        
            p3: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x16160DA74A8E74A2u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x16160DA74A8E74A2u64;
        
        let result = invoke_raw!(
            hash,
                PlayerHandle, 
                FilePath, 
                Name, 
                p3
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn texture_download_request_raw(
        PlayerHandle: , 
        FilePath: , 
        Name: , 
        p3: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x16160DA74A8E74A2u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x16160DA74A8E74A2u64;

        invoke_raw_typed!(
            hash,
                PlayerHandle, 
                FilePath, 
                Name, 
                p3
        )
    }
}

/// ```
Seems to always return 0, but it's used in quite a few loops.
for (num3 = 0; num3 < NETWORK::0xCCD8C02D(); num3++)
    {
        if (NETWORK::NETWORK_IS_PARTICIPANT_ACTIVE(PLAYER::0x98F3B274(num3)) != 0)
        {
            var num5 = NETWORK::NETWORK_GET_PLAYER_INDEX(PLAYER::0x98F3B274(num3));
```



pub fn network_get_max_num_participants_safe(
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xA6C90FBC38E395EEu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xA6C90FBC38E395EEu64;
        
        let result = invoke_raw!(
            hash,
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn network_get_max_num_participants_raw(
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xA6C90FBC38E395EEu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xA6C90FBC38E395EEu64;

        invoke_raw_typed!(
            hash,
        )
    }
}

/// ## Parameters
*



pub fn ugc_get_content_rating_negative_count_safe(
        
        
            p0: 
        , 
        
        
            p1: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x4E548C0D7AE39FF9u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x4E548C0D7AE39FF9u64;
        
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
pub fn ugc_get_content_rating_negative_count_raw(
        p0: , 
        p1: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x4E548C0D7AE39FF9u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x4E548C0D7AE39FF9u64;

        invoke_raw_typed!(
            hash,
                p0, 
                p1
        )
    }
}

/// ```
Actually returns the version (TUNABLE_VERSION)
```



pub fn network_get_tunable_cloud_crc_safe(
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x10BD227A753B0D84u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x10BD227A753B0D84u64;
        
        let result = invoke_raw!(
            hash,
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn network_get_tunable_cloud_crc_raw(
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x10BD227A753B0D84u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x10BD227A753B0D84u64;

        invoke_raw_typed!(
            hash,
        )
    }
}

/// ## Return value



pub fn ugc_is_getting_safe(
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xD53ACDBEF24A46E8u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xD53ACDBEF24A46E8u64;
        
        let result = invoke_raw!(
            hash,
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn ugc_is_getting_raw(
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xD53ACDBEF24A46E8u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xD53ACDBEF24A46E8u64;

        invoke_raw_typed!(
            hash,
        )
    }
}

/// ## Parameters
*



pub fn _0x2e4c123d1c8a710e_safe(
        
        
            retPlayerIds: 
        , 
        
        
            retNumber: 
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
        let hash = 0x2E4C123D1C8A710Eu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x2E4C123D1C8A710Eu64;
        
        let result = invoke_raw!(
            hash,
                retPlayerIds, 
                retNumber, 
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
pub fn _0x2e4c123d1c8a710e_raw(
        retPlayerIds: , 
        retNumber: , 
        p2: , 
        p3: , 
        p4: , 
        p5: , 
        p6: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x2E4C123D1C8A710Eu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x2E4C123D1C8A710Eu64;

        invoke_raw_typed!(
            hash,
                retPlayerIds, 
                retNumber, 
                p2, 
                p3, 
                p4, 
                p5, 
                p6
        )
    }
}

/// ```
Returns true if profile setting 901 is set to true and sets it to false.

NETWORK_C*
```



pub fn _0x60edd13eb3ac1ff3_safe(
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x60EDD13EB3AC1FF3u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x60EDD13EB3AC1FF3u64;
        
        let result = invoke_raw!(
            hash,
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn _0x60edd13eb3ac1ff3_raw(
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x60EDD13EB3AC1FF3u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x60EDD13EB3AC1FF3u64;

        invoke_raw_typed!(
            hash,
        )
    }
}

/// ## Parameters
*



pub fn _0xe6717e652b8c8d8a_safe(
        
        
            p0: 
        , 
        
        
            p1: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xE6717E652B8C8D8Au32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xE6717E652B8C8D8Au64;
        
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
pub fn _0xe6717e652b8c8d8a_raw(
        p0: , 
        p1: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xE6717E652B8C8D8Au32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xE6717E652B8C8D8Au64;

        invoke_raw_typed!(
            hash,
                p0, 
                p1
        )
    }
}

/// ```
Same as GET_CLOUD_TIME_AS_INT but returns the value as a hex string (%I64X).
```



pub fn _get_cloud_time_as_string_safe(
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xF12E6CD06C73D69Eu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xF12E6CD06C73D69Eu64;
        
        let result = invoke_raw!(
            hash,
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn _get_cloud_time_as_string_raw(
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xF12E6CD06C73D69Eu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xF12E6CD06C73D69Eu64;

        invoke_raw_typed!(
            hash,
        )
    }
}

/// ## Parameters
*



pub fn network_get_entity_killer_of_player_safe(
        
        
            player: 
        , 
        
        
            weaponHash: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x42B2DAA6B596F5F8u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x42B2DAA6B596F5F8u64;
        
        let result = invoke_raw!(
            hash,
                player, 
                weaponHash
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn network_get_entity_killer_of_player_raw(
        player: , 
        weaponHash: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x42B2DAA6B596F5F8u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x42B2DAA6B596F5F8u64;

        invoke_raw_typed!(
            hash,
                player, 
                weaponHash
        )
    }
}

/// ## Parameters
*



pub fn _0x36391f397731595d_safe(
        
        
            p0: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x36391F397731595Du32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x36391F397731595Du64;
        
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
pub fn _0x36391f397731595d_raw(
        p0: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x36391F397731595Du32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x36391F397731595Du64;

        invoke_raw_typed!(
            hash,
                p0
        )
    }
}

/// ## Return value



pub fn network_is_in_spectator_mode_safe(
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x048746E388762E11u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x048746E388762E11u64;
        
        let result = invoke_raw!(
            hash,
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn network_is_in_spectator_mode_raw(
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x048746E388762E11u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x048746E388762E11u64;

        invoke_raw_typed!(
            hash,
        )
    }
}

/// ```
p0 appears to be for MP  
```

```
NativeDB Added Parameter 2: Any p1
```



pub fn get_num_reserved_mission_vehicles_safe(
        
        
            p0: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xCF3A965906452031u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xCF3A965906452031u64;
        
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
pub fn get_num_reserved_mission_vehicles_raw(
        p0: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xCF3A965906452031u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xCF3A965906452031u64;

        invoke_raw_typed!(
            hash,
                p0
        )
    }
}

/// ```
Does nothing in online but in offline it will cause the screen to fade to black. Nothing happens past then, the screen will sit at black until you restart GTA. Other stuff must be needed to actually host a session.  
```



pub fn network_session_host_friends_only_safe(
        
        
            p0: 
        , 
        
        
            maxPlayers: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xB9CFD27A5D578D83u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xB9CFD27A5D578D83u64;
        
        let result = invoke_raw!(
            hash,
                p0, 
                maxPlayers
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn network_session_host_friends_only_raw(
        p0: , 
        maxPlayers: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xB9CFD27A5D578D83u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xB9CFD27A5D578D83u64;

        invoke_raw_typed!(
            hash,
                p0, 
                maxPlayers
        )
    }
}

/// ## Parameters
*



pub fn network_set_override_spectator_mode_safe(
        
        
            toggle: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x70DA3BF8DACD3210u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x70DA3BF8DACD3210u64;
        
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
pub fn network_set_override_spectator_mode_raw(
        toggle: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x70DA3BF8DACD3210u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x70DA3BF8DACD3210u64;

        invoke_raw_typed!(
            hash,
                toggle
        )
    }
}

/// ## Parameters
*



pub fn _0x2da41ed6e1fcd7a5_safe(
        
        
            p0: 
        , 
        
        
            TypeOfWeapon: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x2DA41ED6E1FCD7A5u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x2DA41ED6E1FCD7A5u64;
        
        let result = invoke_raw!(
            hash,
                p0, 
                TypeOfWeapon
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn _0x2da41ed6e1fcd7a5_raw(
        p0: , 
        TypeOfWeapon: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x2DA41ED6E1FCD7A5u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x2DA41ED6E1FCD7A5u64;

        invoke_raw_typed!(
            hash,
                p0, 
                TypeOfWeapon
        )
    }
}

/// Note: This only works for vehicles, which appears to be a bug (since the setter _does_ work for every entity type and the name is 99% correct).



pub fn _network_is_entity_concealed_safe(
        
        
            entity: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x71302EC70689052Au32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x71302EC70689052Au64;
        
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
pub fn _network_is_entity_concealed_raw(
        entity: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x71302EC70689052Au32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x71302EC70689052Au64;

        invoke_raw_typed!(
            hash,
                entity
        )
    }
}

/// ## Parameters
*



pub fn network_get_platform_party_members_safe(
        
        
            data: 
        , 
        
        
            dataSize: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x120364DE2845DAF8u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x120364DE2845DAF8u64;
        
        let result = invoke_raw!(
            hash,
                data, 
                dataSize
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn network_get_platform_party_members_raw(
        data: , 
        dataSize: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x120364DE2845DAF8u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x120364DE2845DAF8u64;

        invoke_raw_typed!(
            hash,
                data, 
                dataSize
        )
    }
}

/// ## Parameters
*



pub fn network_invite_gamers_safe(
        
        
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
        let hash = 0x9D80CD1D0E6327DEu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x9D80CD1D0E6327DEu64;
        
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
pub fn network_invite_gamers_raw(
        p0: , 
        p1: , 
        p2: , 
        p3: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x9D80CD1D0E6327DEu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x9D80CD1D0E6327DEu64;

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



pub fn network_is_door_networked_safe(
        
        
            doorHash: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xC01E93FAC20C3346u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xC01E93FAC20C3346u64;
        
        let result = invoke_raw!(
            hash,
                doorHash
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn network_is_door_networked_raw(
        doorHash: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xC01E93FAC20C3346u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xC01E93FAC20C3346u64;

        invoke_raw_typed!(
            hash,
                doorHash
        )
    }
}

/// ## Parameters
*



pub fn ugc_query_my_content_safe(
        
        
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
        let hash = 0x9BF438815F5D96EAu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x9BF438815F5D96EAu64;
        
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
pub fn ugc_query_my_content_raw(
        p0: , 
        p1: , 
        p2: , 
        p3: , 
        p4: , 
        p5: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x9BF438815F5D96EAu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x9BF438815F5D96EAu64;

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
NativeDB Introduced: v1493
```



pub fn _0x023acab2dc9dc4a4_safe(
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x023ACAB2DC9DC4A4u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x023ACAB2DC9DC4A4u64;
        
        let result = invoke_raw!(
            hash,
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn _0x023acab2dc9dc4a4_raw(
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x023ACAB2DC9DC4A4u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x023ACAB2DC9DC4A4u64;

        invoke_raw_typed!(
            hash,
        )
    }
}

/// ## Return value



pub fn network_can_bail_safe(
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x580CE4438479CC61u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x580CE4438479CC61u64;
        
        let result = invoke_raw!(
            hash,
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn network_can_bail_raw(
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x580CE4438479CC61u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x580CE4438479CC61u64;

        invoke_raw_typed!(
            hash,
        )
    }
}

/// ## Return value



pub fn network_clan_get_local_memberships_count_safe(
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x1F471B79ACC90BEFu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x1F471B79ACC90BEFu64;
        
        let result = invoke_raw!(
            hash,
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn network_clan_get_local_memberships_count_raw(
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x1F471B79ACC90BEFu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x1F471B79ACC90BEFu64;

        invoke_raw_typed!(
            hash,
        )
    }
}

/// ```
NETWORK_RE*

Triggers a CEventNetworkInviteConfirmed event
```



pub fn _network_accept_invite_safe(
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x62A0296C1BB1CEB3u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x62A0296C1BB1CEB3u64;
        
        let result = invoke_raw!(
            hash,
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn _network_accept_invite_raw(
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x62A0296C1BB1CEB3u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x62A0296C1BB1CEB3u64;

        invoke_raw_typed!(
            hash,
        )
    }
}

/// ```
Adds the first argument to the second.  
```



pub fn get_time_offset_safe(
        
        
            timeA: 
        , 
        
        
            timeB: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x017008CCDAD48503u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x017008CCDAD48503u64;
        
        let result = invoke_raw!(
            hash,
                timeA, 
                timeB
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn get_time_offset_raw(
        timeA: , 
        timeB: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x017008CCDAD48503u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x017008CCDAD48503u64;

        invoke_raw_typed!(
            hash,
                timeA, 
                timeB
        )
    }
}

/// ## Parameters
*



pub fn _0x261e97ad7bcf3d40_safe(
        
        
            p0: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x261E97AD7BCF3D40u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x261E97AD7BCF3D40u64;
        
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
pub fn _0x261e97ad7bcf3d40_raw(
        p0: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x261E97AD7BCF3D40u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x261E97AD7BCF3D40u64;

        invoke_raw_typed!(
            hash,
                p0
        )
    }
}

/// ## Parameters
*



pub fn network_add_synchronised_scene_camera_safe(
        
        
            netScene: 
        , 
        
        
            animDict: 
        , 
        
        
            animName: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xCF8BD3B0BD6D42D7u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xCF8BD3B0BD6D42D7u64;
        
        let result = invoke_raw!(
            hash,
                netScene, 
                animDict, 
                animName
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn network_add_synchronised_scene_camera_raw(
        netScene: , 
        animDict: , 
        animName: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xCF8BD3B0BD6D42D7u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xCF8BD3B0BD6D42D7u64;

        invoke_raw_typed!(
            hash,
                netScene, 
                animDict, 
                animName
        )
    }
}

/// ## Parameters
*



pub fn network_gamertag_from_handle_start_safe(
        
        
            networkHandle: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x9F0C0A981D73FA56u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x9F0C0A981D73FA56u64;
        
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
pub fn network_gamertag_from_handle_start_raw(
        networkHandle: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x9F0C0A981D73FA56u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x9F0C0A981D73FA56u64;

        invoke_raw_typed!(
            hash,
                networkHandle
        )
    }
}

/// ```
gets the object id of a network id  
```



pub fn net_to_obj_safe(
        
        
            netHandle: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xD8515F5FEA14CB3Fu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xD8515F5FEA14CB3Fu64;
        
        let result = invoke_raw!(
            hash,
                netHandle
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn net_to_obj_raw(
        netHandle: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xD8515F5FEA14CB3Fu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xD8515F5FEA14CB3Fu64;

        invoke_raw_typed!(
            hash,
                netHandle
        )
    }
}

/// ## Parameters
*



pub fn network_set_script_ready_for_events_safe(
        
        
            toggle: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x7AC752103856FB20u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x7AC752103856FB20u64;
        
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
pub fn network_set_script_ready_for_events_raw(
        toggle: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x7AC752103856FB20u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x7AC752103856FB20u64;

        invoke_raw_typed!(
            hash,
                toggle
        )
    }
}

/// ## Parameters
*



pub fn _0xb309ebea797e001f_safe(
        
        
            p0: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xB309EBEA797E001Fu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xB309EBEA797E001Fu64;
        
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
pub fn _0xb309ebea797e001f_raw(
        p0: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xB309EBEA797E001Fu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xB309EBEA797E001Fu64;

        invoke_raw_typed!(
            hash,
                p0
        )
    }
}

/// ## Parameters
*



pub fn ugc_get_content_user_id_safe(
        
        
            p0: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xCD67AD041A394C9Cu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xCD67AD041A394C9Cu64;
        
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
pub fn ugc_get_content_user_id_raw(
        p0: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xCD67AD041A394C9Cu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xCD67AD041A394C9Cu64;

        invoke_raw_typed!(
            hash,
                p0
        )
    }
}

/// ## Parameters
*



pub fn network_set_player_is_passive_safe(
        
        
            toggle: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x1B857666604B1A74u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x1B857666604B1A74u64;
        
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
pub fn network_set_player_is_passive_raw(
        toggle: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x1B857666604B1A74u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x1B857666604B1A74u64;

        invoke_raw_typed!(
            hash,
                toggle
        )
    }
}

/// ## Return value



pub fn network_join_previously_failed_session_safe(
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x59DF79317F85A7E0u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x59DF79317F85A7E0u64;
        
        let result = invoke_raw!(
            hash,
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn network_join_previously_failed_session_raw(
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x59DF79317F85A7E0u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x59DF79317F85A7E0u64;

        invoke_raw_typed!(
            hash,
        )
    }
}

/// ## Parameters
*



pub fn network_clan_get_membership_count_safe(
        
        
            p0: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xAAB11F6C4ADBC2C1u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xAAB11F6C4ADBC2C1u64;
        
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
pub fn network_clan_get_membership_count_raw(
        p0: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xAAB11F6C4ADBC2C1u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xAAB11F6C4ADBC2C1u64;

        invoke_raw_typed!(
            hash,
                p0
        )
    }
}

/// ## Parameters
*



pub fn network_hash_from_player_handle_safe(
        
        
            player: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xBC1D768F2F5D6C05u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xBC1D768F2F5D6C05u64;
        
        let result = invoke_raw!(
            hash,
                player
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn network_hash_from_player_handle_raw(
        player: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xBC1D768F2F5D6C05u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xBC1D768F2F5D6C05u64;

        invoke_raw_typed!(
            hash,
                player
        )
    }
}

/// ```
Based on scripts such as in freemode.c how they call their vars vVar and fVar the 2nd and 3rd param it a Vector3 and Float, but the first is based on get_random_int_in_range..  
```



pub fn network_get_respawn_result_safe(
        
        
            randomInt: 
        , 
        
        
            coordinates: 
        , 
        
        
            heading: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x371EA43692861CF1u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x371EA43692861CF1u64;
        
        let result = invoke_raw!(
            hash,
                randomInt, 
                coordinates, 
                heading
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn network_get_respawn_result_raw(
        randomInt: , 
        coordinates: , 
        heading: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x371EA43692861CF1u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x371EA43692861CF1u64;

        invoke_raw_typed!(
            hash,
                randomInt, 
                coordinates, 
                heading
        )
    }
}

/// Revives our local player who was previously dead.



pub fn network_resurrect_local_player_safe(
        
        
            x: 
        , 
        
        
            y: 
        , 
        
        
            z: 
        , 
        
        
            heading: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xEA23C49EAA83ACFBu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xEA23C49EAA83ACFBu64;
        
        let result = invoke_raw!(
            hash,
                x, 
                y, 
                z, 
                heading
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn network_resurrect_local_player_raw(
        x: , 
        y: , 
        z: , 
        heading: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xEA23C49EAA83ACFBu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xEA23C49EAA83ACFBu64;

        invoke_raw_typed!(
            hash,
                x, 
                y, 
                z, 
                heading
        )
    }
}

/// ## Parameters
*



pub fn _0x59d421683d31835a_safe(
        
        
            p0: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x59D421683D31835Au32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x59D421683D31835Au64;
        
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
pub fn _0x59d421683d31835a_raw(
        p0: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x59D421683D31835Au32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x59D421683D31835Au64;

        invoke_raw_typed!(
            hash,
                p0
        )
    }
}

/// ## Parameters
*



pub fn network_is_gamer_talking_safe(
        
        
            networkHandle: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x71C33B22606CD88Au32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x71C33B22606CD88Au64;
        
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
pub fn network_is_gamer_talking_raw(
        networkHandle: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x71C33B22606CD88Au32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x71C33B22606CD88Au64;

        invoke_raw_typed!(
            hash,
                networkHandle
        )
    }
}

/// _0xB13E88E655E5A3BC native function



pub fn _0xb13e88e655e5a3bc_safe(
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xB13E88E655E5A3BCu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xB13E88E655E5A3BCu64;
        
        let result = invoke_raw!(
            hash,
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn _0xb13e88e655e5a3bc_raw(
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xB13E88E655E5A3BCu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xB13E88E655E5A3BCu64;

        invoke_raw_typed!(
            hash,
        )
    }
}

/// ## Parameters
*



pub fn fillout_pm_player_list_safe(
        
        
            networkHandle: 
        , 
        
        
            p1: 
        , 
        
        
            p2: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xCBBD7C4991B64809u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xCBBD7C4991B64809u64;
        
        let result = invoke_raw!(
            hash,
                networkHandle, 
                p1, 
                p2
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn fillout_pm_player_list_raw(
        networkHandle: , 
        p1: , 
        p2: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xCBBD7C4991B64809u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xCBBD7C4991B64809u64;

        invoke_raw_typed!(
            hash,
                networkHandle, 
                p1, 
                p2
        )
    }
}

/// ## Parameters
*



pub fn network_am_i_muted_by_player_safe(
        
        
            player: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x9D6981DFC91A8604u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x9D6981DFC91A8604u64;
        
        let result = invoke_raw!(
            hash,
                player
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn network_am_i_muted_by_player_raw(
        player: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x9D6981DFC91A8604u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x9D6981DFC91A8604u64;

        invoke_raw_typed!(
            hash,
                player
        )
    }
}

/// ## Parameters
*



pub fn ugc_get_content_rating_positive_count_safe(
        
        
            p0: 
        , 
        
        
            p1: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x87E5C46C187FE0AEu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x87E5C46C187FE0AEu64;
        
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
pub fn ugc_get_content_rating_positive_count_raw(
        p0: , 
        p1: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x87E5C46C187FE0AEu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x87E5C46C187FE0AEu64;

        invoke_raw_typed!(
            hash,
                p0, 
                p1
        )
    }
}

/// ## Parameters
*



pub fn network_am_i_blocked_by_gamer_safe(
        
        
            networkHandle: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x15337C7C268A27B2u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x15337C7C268A27B2u64;
        
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
pub fn network_am_i_blocked_by_gamer_raw(
        networkHandle: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x15337C7C268A27B2u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x15337C7C268A27B2u64;

        invoke_raw_typed!(
            hash,
                networkHandle
        )
    }
}

/// ## Parameters
*



pub fn network_use_logarithmic_blending_this_frame_safe(
        
        
            entity: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xCD71A4ECAB22709Eu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xCD71A4ECAB22709Eu64;
        
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
pub fn network_use_logarithmic_blending_this_frame_raw(
        entity: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xCD71A4ECAB22709Eu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xCD71A4ECAB22709Eu64;

        invoke_raw_typed!(
            hash,
                entity
        )
    }
}

/// ## Return value



pub fn network_session_get_private_slots_safe(
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x53AFD64C6758F2F9u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x53AFD64C6758F2F9u64;
        
        let result = invoke_raw!(
            hash,
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn network_session_get_private_slots_raw(
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x53AFD64C6758F2F9u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x53AFD64C6758F2F9u64;

        invoke_raw_typed!(
            hash,
        )
    }
}

/// ## Parameters
*



pub fn network_get_presence_invite_playlist_length_safe(
        
        
            p0: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xD39B3FFF8FFDD5BFu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xD39B3FFF8FFDD5BFu64;
        
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
pub fn network_get_presence_invite_playlist_length_raw(
        p0: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xD39B3FFF8FFDD5BFu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xD39B3FFF8FFDD5BFu64;

        invoke_raw_typed!(
            hash,
                p0
        )
    }
}

/// ## Parameters
*



pub fn _0x83660b734994124d_safe(
        
        
            p0: 
        , 
        
        
            p1: 
        , 
        
        
            damageDealt: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x83660B734994124Du32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x83660B734994124Du64;
        
        let result = invoke_raw!(
            hash,
                p0, 
                p1, 
                damageDealt
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn _0x83660b734994124d_raw(
        p0: , 
        p1: , 
        damageDealt: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x83660B734994124Du32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x83660B734994124Du64;

        invoke_raw_typed!(
            hash,
                p0, 
                p1, 
                damageDealt
        )
    }
}

/// ```
NativeDB Introduced: v2372
```



pub fn _network_ugc_nav_safe(
        
        
            p0: 
        , 
        
        
            p1: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xC1447451DDB512F0u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xC1447451DDB512F0u64;
        
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
pub fn _network_ugc_nav_raw(
        p0: , 
        p1: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xC1447451DDB512F0u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xC1447451DDB512F0u64;

        invoke_raw_typed!(
            hash,
                p0, 
                p1
        )
    }
}

/// ```
NativeDB Added Parameter 1: Entity entity
```



pub fn _0xaa5fafcd2c5f5e47_safe(
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xAA5FAFCD2C5F5E47u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xAA5FAFCD2C5F5E47u64;
        
        let result = invoke_raw!(
            hash,
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn _0xaa5fafcd2c5f5e47_raw(
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xAA5FAFCD2C5F5E47u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xAA5FAFCD2C5F5E47u64;

        invoke_raw_typed!(
            hash,
        )
    }
}

/// ## Return value



pub fn network_session_is_in_voice_session_safe(
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x855BC38818F6F684u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x855BC38818F6F684u64;
        
        let result = invoke_raw!(
            hash,
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn network_session_is_in_voice_session_raw(
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x855BC38818F6F684u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x855BC38818F6F684u64;

        invoke_raw_typed!(
            hash,
        )
    }
}

/// ## Return value



pub fn network_gamertag_from_handle_succeeded_safe(
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xFD00798DBA7523DDu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xFD00798DBA7523DDu64;
        
        let result = invoke_raw!(
            hash,
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn network_gamertag_from_handle_succeeded_raw(
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xFD00798DBA7523DDu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xFD00798DBA7523DDu64;

        invoke_raw_typed!(
            hash,
        )
    }
}

/// ## Parameters
*



pub fn network_has_ros_privilege_end_date_safe(
        
        
            privilege: 
        , 
        
        
            banType: 
        , 
        
        
            timeData: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xC22912B1D85F26B1u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xC22912B1D85F26B1u64;
        
        let result = invoke_raw!(
            hash,
                privilege, 
                banType, 
                timeData
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn network_has_ros_privilege_end_date_raw(
        privilege: , 
        banType: , 
        timeData: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xC22912B1D85F26B1u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xC22912B1D85F26B1u64;

        invoke_raw_typed!(
            hash,
                privilege, 
                banType, 
                timeData
        )
    }
}

/// Sets whether or not an object (created using `CREATE_OBJECT`, or similar) should have its position/rotation synchronized,
even if it is a 'static' object (for example, having flag 32 - Static - set in its archetype definition).

This has to be called during the same frame the object is created/registered for network, as otherwise it may already
have a remote clone created.

Once a remote clone is created, changing this value will not have any effect on said clone.



pub fn _network_set_object_force_static_blend_safe(
        
        
            object: 
        , 
        
        
            enabled: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x0379DAF89BA09AA5u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x0379DAF89BA09AA5u64;
        
        let result = invoke_raw!(
            hash,
                object, 
                enabled
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn _network_set_object_force_static_blend_raw(
        object: , 
        enabled: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x0379DAF89BA09AA5u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x0379DAF89BA09AA5u64;

        invoke_raw_typed!(
            hash,
                object, 
                enabled
        )
    }
}

/// ```
Sets some voice chat related value.
NETWORK_SET_*
```



pub fn _0x6a5d89d7769a40d8_safe(
        
        
            toggle: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x6A5D89D7769A40D8u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x6A5D89D7769A40D8u64;
        
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
pub fn _0x6a5d89d7769a40d8_raw(
        toggle: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x6A5D89D7769A40D8u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x6A5D89D7769A40D8u64;

        invoke_raw_typed!(
            hash,
                toggle
        )
    }
}

/// ```
Access to the store for shark cards etc...  
```



pub fn set_store_enabled_safe(
        
        
            toggle: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x9641A9FF718E9C5Eu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x9641A9FF718E9C5Eu64;
        
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
pub fn set_store_enabled_raw(
        toggle: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x9641A9FF718E9C5Eu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x9641A9FF718E9C5Eu64;

        invoke_raw_typed!(
            hash,
                toggle
        )
    }
}

/// ## Parameters
*



pub fn network_unregister_networked_entity_safe(
        
        
            entity: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x7368E683BB9038D6u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x7368E683BB9038D6u64;
        
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
pub fn network_unregister_networked_entity_raw(
        entity: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x7368E683BB9038D6u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x7368E683BB9038D6u64;

        invoke_raw_typed!(
            hash,
                entity
        )
    }
}

/// ## Parameters
*



pub fn reserve_network_mission_peds_safe(
        
        
            amount: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xB60FEBA45333D36Fu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xB60FEBA45333D36Fu64;
        
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
pub fn reserve_network_mission_peds_raw(
        amount: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xB60FEBA45333D36Fu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xB60FEBA45333D36Fu64;

        invoke_raw_typed!(
            hash,
                amount
        )
    }
}

/// ## Parameters
*



pub fn network_has_invite_been_acked_safe(
        
        
            networkHandle: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x71DC455F5CD1C2B1u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x71DC455F5CD1C2B1u64;
        
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
pub fn network_has_invite_been_acked_raw(
        networkHandle: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x71DC455F5CD1C2B1u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x71DC455F5CD1C2B1u64;

        invoke_raw_typed!(
            hash,
                networkHandle
        )
    }
}

/// ## Parameters
*



pub fn network_is_gamer_muted_by_me_safe(
        
        
            networkHandle: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xCE60DE011B6C7978u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xCE60DE011B6C7978u64;
        
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
pub fn network_is_gamer_muted_by_me_raw(
        networkHandle: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xCE60DE011B6C7978u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xCE60DE011B6C7978u64;

        invoke_raw_typed!(
            hash,
                networkHandle
        )
    }
}

/// ## Return value



pub fn network_session_is_visible_safe(
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xBA416D68C631496Au32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xBA416D68C631496Au64;
        
        let result = invoke_raw!(
            hash,
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn network_session_is_visible_raw(
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xBA416D68C631496Au32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xBA416D68C631496Au64;

        invoke_raw_typed!(
            hash,
        )
    }
}

/// ## Parameters
*



pub fn ugc_get_content_rating_safe(
        
        
            p0: 
        , 
        
        
            p1: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x1ACCFBA3D8DAB2EEu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x1ACCFBA3D8DAB2EEu64;
        
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
pub fn ugc_get_content_rating_raw(
        p0: , 
        p1: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x1ACCFBA3D8DAB2EEu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x1ACCFBA3D8DAB2EEu64;

        invoke_raw_typed!(
            hash,
                p0, 
                p1
        )
    }
}

/// ```
Old name: _NETWORK_SET_NETWORK_ID_DYNAMIC
```



pub fn network_use_high_precision_blending_safe(
        
        
            netID: 
        , 
        
        
            toggle: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x2B1813ABA29016C5u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x2B1813ABA29016C5u64;
        
        let result = invoke_raw!(
            hash,
                netID, 
                toggle
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn network_use_high_precision_blending_raw(
        netID: , 
        toggle: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x2B1813ABA29016C5u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x2B1813ABA29016C5u64;

        invoke_raw_typed!(
            hash,
                netID, 
                toggle
        )
    }
}

/// ## Parameters
*



pub fn _0x8b4ffc790ca131ef_safe(
        
        
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
        let hash = 0x8B4FFC790CA131EFu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x8B4FFC790CA131EFu64;
        
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
pub fn _0x8b4ffc790ca131ef_raw(
        p0: , 
        p1: , 
        p2: , 
        p3: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x8B4FFC790CA131EFu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x8B4FFC790CA131EFu64;

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
Returns whether the player is signed into Social Club.  
```



pub fn network_is_signed_in_safe(
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x054354A99211EB96u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x054354A99211EB96u64;
        
        let result = invoke_raw!(
            hash,
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn network_is_signed_in_raw(
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x054354A99211EB96u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x054354A99211EB96u64;

        invoke_raw_typed!(
            hash,
        )
    }
}

/// ```
p0 is always false in scripts.
```



pub fn network_override_receive_restrictions_all_safe(
        
        
            toggle: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x0FF2862B61A58AF9u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x0FF2862B61A58AF9u64;
        
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
pub fn network_override_receive_restrictions_all_raw(
        toggle: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x0FF2862B61A58AF9u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x0FF2862B61A58AF9u64;

        invoke_raw_typed!(
            hash,
                toggle
        )
    }
}

/// ## Parameters
*



pub fn set_local_player_visible_locally_safe(
        
        
            p0: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x7619364C82D3BF14u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x7619364C82D3BF14u64;
        
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
pub fn set_local_player_visible_locally_raw(
        p0: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x7619364C82D3BF14u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x7619364C82D3BF14u64;

        invoke_raw_typed!(
            hash,
                p0
        )
    }
}

/// ```
NETWORK_SESSION_IS_*
```



pub fn _0xd313de83394af134_safe(
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xD313DE83394AF134u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xD313DE83394AF134u64;
        
        let result = invoke_raw!(
            hash,
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn _0xd313de83394af134_raw(
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xD313DE83394AF134u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xD313DE83394AF134u64;

        invoke_raw_typed!(
            hash,
        )
    }
}

/// ## Return value



pub fn _network_has_age_restricted_profile_safe(
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x1353F87E89946207u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x1353F87E89946207u64;
        
        let result = invoke_raw!(
            hash,
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn _network_has_age_restricted_profile_raw(
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x1353F87E89946207u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x1353F87E89946207u64;

        invoke_raw_typed!(
            hash,
        )
    }
}

/// ## Parameters
*



pub fn network_get_gamertag_from_handle_safe(
        
        
            networkHandle: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x426141162EBE5CDBu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x426141162EBE5CDBu64;
        
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
pub fn network_get_gamertag_from_handle_raw(
        networkHandle: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x426141162EBE5CDBu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x426141162EBE5CDBu64;

        invoke_raw_typed!(
            hash,
                networkHandle
        )
    }
}

/// ## Parameters
*



pub fn _network_can_gamer_play_multiplayer_with_me_safe(
        
        
            networkHandle: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x135F9B7B7ADD2185u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x135F9B7B7ADD2185u64;
        
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
pub fn _network_can_gamer_play_multiplayer_with_me_raw(
        networkHandle: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x135F9B7B7ADD2185u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x135F9B7B7ADD2185u64;

        invoke_raw_typed!(
            hash,
                networkHandle
        )
    }
}

/// ```
This checks if player is playing on gta online or not.  
Please add an if and block your mod if this is "true".  
```



pub fn network_is_session_started_safe(
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x9DE624D2FC4B603Fu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x9DE624D2FC4B603Fu64;
        
        let result = invoke_raw!(
            hash,
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn network_is_session_started_raw(
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x9DE624D2FC4B603Fu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x9DE624D2FC4B603Fu64;

        invoke_raw_typed!(
            hash,
        )
    }
}

/// ## Parameters
*



pub fn _0x600f8cb31c7aab6e_safe(
        
        
            p0: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x600F8CB31C7AAB6Eu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x600F8CB31C7AAB6Eu64;
        
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
pub fn _0x600f8cb31c7aab6e_raw(
        p0: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x600F8CB31C7AAB6Eu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x600F8CB31C7AAB6Eu64;

        invoke_raw_typed!(
            hash,
                p0
        )
    }
}

/// ## Parameters
*



pub fn _is_entity_ghosted_to_local_player_safe(
        
        
            entity: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x21D04D7BC538C146u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x21D04D7BC538C146u64;
        
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
pub fn _is_entity_ghosted_to_local_player_raw(
        entity: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x21D04D7BC538C146u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x21D04D7BC538C146u64;

        invoke_raw_typed!(
            hash,
                entity
        )
    }
}

/// ```
NativeDB Introduced: v323
```



pub fn _network_get_num_unacked_for_player_safe(
        
        
            player: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xFF8FCF9FFC458A1Cu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xFF8FCF9FFC458A1Cu64;
        
        let result = invoke_raw!(
            hash,
                player
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn _network_get_num_unacked_for_player_raw(
        player: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xFF8FCF9FFC458A1Cu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xFF8FCF9FFC458A1Cu64;

        invoke_raw_typed!(
            hash,
                player
        )
    }
}

/// ## Return value



pub fn network_is_clock_time_overridden_safe(
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xD7C95D322FF57522u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xD7C95D322FF57522u64;
        
        let result = invoke_raw!(
            hash,
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn network_is_clock_time_overridden_raw(
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xD7C95D322FF57522u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xD7C95D322FF57522u64;

        invoke_raw_typed!(
            hash,
        )
    }
}

