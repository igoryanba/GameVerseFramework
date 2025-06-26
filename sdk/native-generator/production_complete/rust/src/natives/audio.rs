//! AUDIO native functions
//! 
//! Functions for the audio category (local)

use crate::types::*;
use crate::errors::{NativeError, NativeResult, validation};
use nalgebra::Vector3;
use tracing::{debug, warn};

/// This native is marked as deprecated internally, please use [REQUEST_SCRIPT_AUDIO_BANK](#_0x2F844A8B08D76685)

This native has a new argument on newer game builds:
*



pub fn request_mission_audio_bank_safe(
        
        
            bankName: 
        , 
        
        
            bOverNetwork: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x7345BDD95E62E0F2u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x7345BDD95E62E0F2u64;
        
        let result = invoke_raw!(
            hash,
                bankName, 
                bOverNetwork
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn request_mission_audio_bank_raw(
        bankName: , 
        bOverNetwork: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x7345BDD95E62E0F2u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x7345BDD95E62E0F2u64;

        invoke_raw_typed!(
            hash,
                bankName, 
                bOverNetwork
        )
    }
}

/// ```
NativeDB Introduced: v1290
```



pub fn _0x9bd7bd55e4533183_safe(
        
        
            p0: 
        , 
        
        
            p1: 
        , 
        
        
            p2: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x9BD7BD55E4533183u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x9BD7BD55E4533183u64;
        
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
pub fn _0x9bd7bd55e4533183_raw(
        p0: , 
        p1: , 
        p2: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x9BD7BD55E4533183u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x9BD7BD55E4533183u64;

        invoke_raw_typed!(
            hash,
                p0, 
                p1, 
                p2
        )
    }
}

/// ```
All music event names found in the b617d scripts: pastebin.com/GnYt0R3P
```



pub fn cancel_music_event_safe(
        
        
            eventName: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x5B17A90291133DA5u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x5B17A90291133DA5u64;
        
        let result = invoke_raw!(
            hash,
                eventName
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn cancel_music_event_raw(
        eventName: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x5B17A90291133DA5u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x5B17A90291133DA5u64;

        invoke_raw_typed!(
            hash,
                eventName
        )
    }
}

/// ## Parameters
*



pub fn stop_scripted_conversation_safe(
        
        
            finishCurrentLine: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xD79DEEFB53455EBAu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xD79DEEFB53455EBAu64;
        
        let result = invoke_raw!(
            hash,
                finishCurrentLine
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn stop_scripted_conversation_raw(
        finishCurrentLine: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xD79DEEFB53455EBAu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xD79DEEFB53455EBAu64;

        invoke_raw_typed!(
            hash,
                finishCurrentLine
        )
    }
}

/// Stops the named mixing scene (which was previously started by this script)



pub fn stop_audio_scene_safe(
        
        
            sceneName: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xDFE8422B3B94E688u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xDFE8422B3B94E688u64;
        
        let result = invoke_raw!(
            hash,
                sceneName
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn stop_audio_scene_raw(
        sceneName: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xDFE8422B3B94E688u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xDFE8422B3B94E688u64;

        invoke_raw_typed!(
            hash,
                sceneName
        )
    }
}

/// ```
NativeDB Introduced: v463
```



pub fn _0x2dd39bf3e2f9c47f_safe(
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x2DD39BF3E2F9C47Fu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x2DD39BF3E2F9C47Fu64;
        
        let result = invoke_raw!(
            hash,
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn _0x2dd39bf3e2f9c47f_raw(
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x2DD39BF3E2F9C47Fu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x2DD39BF3E2F9C47Fu64;

        invoke_raw_typed!(
            hash,
        )
    }
}

/// Sets the radio to tune up. (changes radio station)



pub fn set_radio_retune_up_safe(
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xFF266D1D0EB1195Du32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xFF266D1D0EB1195Du64;
        
        let result = invoke_raw!(
            hash,
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn set_radio_retune_up_raw(
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xFF266D1D0EB1195Du32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xFF266D1D0EB1195Du64;

        invoke_raw_typed!(
            hash,
        )
    }
}

/// Removes an entity from its current mix group.



pub fn remove_entity_from_audio_mix_group_safe(
        
        
            entity: 
        , 
        
        
            fadeOut: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x18EB48CFC41F2EA0u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x18EB48CFC41F2EA0u64;
        
        let result = invoke_raw!(
            hash,
                entity, 
                fadeOut
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn remove_entity_from_audio_mix_group_raw(
        entity: , 
        fadeOut: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x18EB48CFC41F2EA0u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x18EB48CFC41F2EA0u64;

        invoke_raw_typed!(
            hash,
                entity, 
                fadeOut
        )
    }
}

/// Clears the previously queued custom track lost for the given radio station.



pub fn clear_custom_radio_track_list_safe(
        
        
            radioStation: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x1654F24A88A8E3FEu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x1654F24A88A8E3FEu64;
        
        let result = invoke_raw!(
            hash,
                radioStation
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn clear_custom_radio_track_list_raw(
        radioStation: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x1654F24A88A8E3FEu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x1654F24A88A8E3FEu64;

        invoke_raw_typed!(
            hash,
                radioStation
        )
    }
}

/// This should be called once a sound has finished being manipulated by the script so that its SoundId can be released and re-used.



pub fn release_sound_id_safe(
        
        
            soundId: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x353FC880830B88FAu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x353FC880830B88FAu64;
        
        let result = invoke_raw!(
            hash,
                soundId
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn release_sound_id_raw(
        soundId: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x353FC880830B88FAu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x353FC880830B88FAu64;

        invoke_raw_typed!(
            hash,
                soundId
        )
    }
}

/// ## Parameters
*



pub fn set_ped_race_and_voice_group_safe(
        
        
            ped: 
        , 
        
        
            pedRace: 
        , 
        
        
            pvgHash: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x1B7ABE26CBCBF8C7u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x1B7ABE26CBCBF8C7u64;
        
        let result = invoke_raw!(
            hash,
                ped, 
                pedRace, 
                pvgHash
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn set_ped_race_and_voice_group_raw(
        ped: , 
        pedRace: , 
        pvgHash: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x1B7ABE26CBCBF8C7u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x1B7ABE26CBCBF8C7u64;

        invoke_raw_typed!(
            hash,
                ped, 
                pedRace, 
                pvgHash
        )
    }
}

/// See [`PLAY_PED_AMBIENT_SPEECH_NATIVE`](#_0x8E04FEDD28D42462) for parameter specifications.

```
NativeDB Added Parameter 4: Any p3
```



pub fn play_ped_ambient_speech_and_clone_native_safe(
        
        
            ped: 
        , 
        
        
            speechName: 
        , 
        
        
            speechParam: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xC6941B4A3A8FBBB9u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xC6941B4A3A8FBBB9u64;
        
        let result = invoke_raw!(
            hash,
                ped, 
                speechName, 
                speechParam
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn play_ped_ambient_speech_and_clone_native_raw(
        ped: , 
        speechName: , 
        speechParam: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xC6941B4A3A8FBBB9u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xC6941B4A3A8FBBB9u64;

        invoke_raw_typed!(
            hash,
                ped, 
                speechName, 
                speechParam
        )
    }
}

/// Checks if the context exists for the ped, searching through the voices in its PedVoiceGroup.

The final argument can be set to true to allow searching in backup PVGs



pub fn does_context_exist_for_this_ped_safe(
        
        
            ped: 
        , 
        
        
            speechName: 
        , 
        
        
            allowBackupPVGs: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x49B99BF3FDA89A7Au32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x49B99BF3FDA89A7Au64;
        
        let result = invoke_raw!(
            hash,
                ped, 
                speechName, 
                allowBackupPVGs
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn does_context_exist_for_this_ped_raw(
        ped: , 
        speechName: , 
        allowBackupPVGs: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x49B99BF3FDA89A7Au32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x49B99BF3FDA89A7Au64;

        invoke_raw_typed!(
            hash,
                ped, 
                speechName, 
                allowBackupPVGs
        )
    }
}

/// ## Parameters
*



pub fn is_radio_station_favourited_safe(
        
        
            radioStation: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x2B1784DB08AFEA79u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x2B1784DB08AFEA79u64;
        
        let result = invoke_raw!(
            hash,
                radioStation
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn is_radio_station_favourited_raw(
        radioStation: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x2B1784DB08AFEA79u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x2B1784DB08AFEA79u64;

        invoke_raw_typed!(
            hash,
                radioStation
        )
    }
}

/// Resets the list of ambients zones enabled/disabled status to its value before it was modified by this script.

Default behaviour is that any state change only gets applied once the player leaves the zone.



pub fn clear_ambient_zone_list_state_safe(
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x120C48C614909FA4u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x120C48C614909FA4u64;
        
        let result = invoke_raw!(
            hash,
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn clear_ambient_zone_list_state_raw(
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x120C48C614909FA4u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x120C48C614909FA4u64;

        invoke_raw_typed!(
            hash,
        )
    }
}

/// Forces the chosen station to paly the given music track list. All other music track lists will be locked.



pub fn force_music_track_list_safe(
        
        
            radioStation: 
        , 
        
        
            trackListName: 
        , 
        
        
            timeOffsetMilliseconds: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x4E0AF9114608257Cu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x4E0AF9114608257Cu64;
        
        let result = invoke_raw!(
            hash,
                radioStation, 
                trackListName, 
                timeOffsetMilliseconds
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn force_music_track_list_raw(
        radioStation: , 
        trackListName: , 
        timeOffsetMilliseconds: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x4E0AF9114608257Cu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x4E0AF9114608257Cu64;

        invoke_raw_typed!(
            hash,
                radioStation, 
                trackListName, 
                timeOffsetMilliseconds
        )
    }
}

/// ```
This is the same as PLAY_PED_AMBIENT_SPEECH_NATIVE and PLAY_PED_AMBIENT_SPEECH_AND_CLONE_NATIVE but it will allow you to play a speech file from a specific voice file. It works on players and all peds, even animals.
EX (C#):
GTA.Native.Function.Call(Hash._0x3523634255FC3318, Game.Player.Character, "GENERIC_INSULT_HIGH", "s_m_y_sheriff_01_white_full_01", "SPEECH_PARAMS_FORCE_SHOUTED", 0);
The first param is the ped you want to play it on, the second is the speech name, the third is the voice name, the fourth is the speech param, and the last param is usually always 0.
```



pub fn play_ped_ambient_speech_with_voice_native_safe(
        
        
            ped: 
        , 
        
        
            speechName: 
        , 
        
        
            voiceName: 
        , 
        
        
            speechParam: 
        , 
        
        
            p4: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x3523634255FC3318u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x3523634255FC3318u64;
        
        let result = invoke_raw!(
            hash,
                ped, 
                speechName, 
                voiceName, 
                speechParam, 
                p4
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn play_ped_ambient_speech_with_voice_native_raw(
        ped: , 
        speechName: , 
        voiceName: , 
        speechParam: , 
        p4: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x3523634255FC3318u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x3523634255FC3318u64;

        invoke_raw_typed!(
            hash,
                ped, 
                speechName, 
                voiceName, 
                speechParam, 
                p4
        )
    }
}

/// ## Parameters
*



pub fn set_horn_enabled_safe(
        
        
            vehicle: 
        , 
        
        
            toggle: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x76D683C108594D0Eu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x76D683C108594D0Eu64;
        
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
pub fn set_horn_enabled_raw(
        vehicle: , 
        toggle: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x76D683C108594D0Eu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x76D683C108594D0Eu64;

        invoke_raw_typed!(
            hash,
                vehicle, 
                toggle
        )
    }
}

/// Sets the footstep tuning modes



pub fn override_player_ground_material_safe(
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xD2CC78CD3D0B50F9u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xD2CC78CD3D0B50F9u64;
        
        let result = invoke_raw!(
            hash,
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn override_player_ground_material_raw(
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xD2CC78CD3D0B50F9u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xD2CC78CD3D0B50F9u64;

        invoke_raw_typed!(
            hash,
        )
    }
}

/// This native has been marked as deprecated internally, please use [RELEASE_SCRIPT_AUDIO_BANK](#_0x7A2D8AD0A9EB9C3F) instead.



pub fn release_ambient_audio_bank_safe(
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x65475A218FFAA93Du32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x65475A218FFAA93Du64;
        
        let result = invoke_raw!(
            hash,
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn release_ambient_audio_bank_raw(
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x65475A218FFAA93Du32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x65475A218FFAA93Du64;

        invoke_raw_typed!(
            hash,
        )
    }
}

/// ## Parameters
*



pub fn get_ambient_voice_name_hash_safe(
        
        
            ped: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x5E203DA2BA15D436u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x5E203DA2BA15D436u64;
        
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
pub fn get_ambient_voice_name_hash_raw(
        ped: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x5E203DA2BA15D436u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x5E203DA2BA15D436u64;

        invoke_raw_typed!(
            hash,
                ped
        )
    }
}

/// Hints that this bank would be good to load if there are free slots.
Does not guarentee loading of the bank, [REQUEST_SCRIPT_AUDIO_BANK](#_0xFE02FFBED8CA9D99) MUST be used as normal before triggering sounds"

This native has a new argument on newer game builds:
*



pub fn hint_script_audio_bank_safe(
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xFB380A29641EC31Au32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xFB380A29641EC31Au64;
        
        let result = invoke_raw!(
            hash,
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn hint_script_audio_bank_raw(
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xFB380A29641EC31Au32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xFB380A29641EC31Au64;

        invoke_raw_typed!(
            hash,
        )
    }
}

/// Determines if any ped-independent, positionally-based scripted speech is currently active. This typically includes speech events triggered using [`PLAY_AMBIENT_SPEECH_FROM_POSITION_NATIVE`](#_0xED640017ED337E45).

```
NativeDB Introduced: v2189
```



pub fn is_any_positional_speech_playing_safe(
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x30CA2EF91D15ADF8u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x30CA2EF91D15ADF8u64;
        
        let result = invoke_raw!(
            hash,
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn is_any_positional_speech_playing_raw(
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x30CA2EF91D15ADF8u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x30CA2EF91D15ADF8u64;

        invoke_raw_typed!(
            hash,
        )
    }
}

/// Counterpart: [`GET_NETWORK_ID_FROM_SOUND_ID`](#_0x2DE3F0A134FFBC0D).



pub fn get_sound_id_from_network_id_safe(
        
        
            netId: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x75262FD12D0A1C84u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x75262FD12D0A1C84u64;
        
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
pub fn get_sound_id_from_network_id_raw(
        netId: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x75262FD12D0A1C84u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x75262FD12D0A1C84u64;

        invoke_raw_typed!(
            hash,
                netId
        )
    }
}

/// ## Parameters
*



pub fn get_current_track_sound_name_safe(
        
        
            radioStationName: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x34D66BC058019CE0u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x34D66BC058019CE0u64;
        
        let result = invoke_raw!(
            hash,
                radioStationName
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn get_current_track_sound_name_raw(
        radioStationName: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x34D66BC058019CE0u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x34D66BC058019CE0u64;

        invoke_raw_typed!(
            hash,
                radioStationName
        )
    }
}

/// ## Parameters
*



pub fn set_variable_on_sound_safe(
        
        
            soundId: 
        , 
        
        
            variableName: 
        , 
        
        
            value: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xAD6B3148A78AE9B6u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xAD6B3148A78AE9B6u64;
        
        let result = invoke_raw!(
            hash,
                soundId, 
                variableName, 
                value
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn set_variable_on_sound_raw(
        soundId: , 
        variableName: , 
        value: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xAD6B3148A78AE9B6u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xAD6B3148A78AE9B6u64;

        invoke_raw_typed!(
            hash,
                soundId, 
                variableName, 
                value
        )
    }
}

/// Sets the ped so they sound drunk



pub fn set_ped_is_drunk_safe(
        
        
            ped: 
        , 
        
        
            toggle: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x95D2D383D5396B8Au32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x95D2D383D5396B8Au64;
        
        let result = invoke_raw!(
            hash,
                ped, 
                toggle
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn set_ped_is_drunk_raw(
        ped: , 
        toggle: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x95D2D383D5396B8Au32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x95D2D383D5396B8Au64;

        invoke_raw_typed!(
            hash,
                ped, 
                toggle
        )
    }
}

/// ```
Example:
AUDIO::SET_STATIC_EMITTER_ENABLED((Any*)"LOS_SANTOS_VANILLA_UNICORN_01_STAGE", false);    AUDIO::SET_STATIC_EMITTER_ENABLED((Any*)"LOS_SANTOS_VANILLA_UNICORN_02_MAIN_ROOM", false);    AUDIO::SET_STATIC_EMITTER_ENABLED((Any*)"LOS_SANTOS_VANILLA_UNICORN_03_BACK_ROOM", false);
This turns off surrounding sounds not connected directly to peds.
```



pub fn set_static_emitter_enabled_safe(
        
        
            emitterName: 
        , 
        
        
            toggle: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x399D2D3B33F1B8EBu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x399D2D3B33F1B8EBu64;
        
        let result = invoke_raw!(
            hash,
                emitterName, 
                toggle
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn set_static_emitter_enabled_raw(
        emitterName: , 
        toggle: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x399D2D3B33F1B8EBu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x399D2D3B33F1B8EBu64;

        invoke_raw_typed!(
            hash,
                emitterName, 
                toggle
        )
    }
}

/// ## Parameters
*



pub fn get_next_audible_beat_safe(
        
        
            timeInSeconds: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xC64A06D939F826F5u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xC64A06D939F826F5u64;
        
        let result = invoke_raw!(
            hash,
                timeInSeconds
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn get_next_audible_beat_raw(
        timeInSeconds: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xC64A06D939F826F5u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xC64A06D939F826F5u64;

        invoke_raw_typed!(
            hash,
                timeInSeconds
        )
    }
}

/// Plays a siren blip from the vehicle when you double press the horn key.

This only works on vehicles with sirens.



pub fn blip_siren_safe(
        
        
            vehicle: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x1B9025BDA76822B6u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x1B9025BDA76822B6u64;
        
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
pub fn blip_siren_raw(
        vehicle: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x1B9025BDA76822B6u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x1B9025BDA76822B6u64;

        invoke_raw_typed!(
            hash,
                vehicle
        )
    }
}

/// ## Return value
Returns true if the mission complete audio is playing



pub fn is_mission_complete_playing_safe(
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x19A30C23F5827F8Au32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x19A30C23F5827F8Au64;
        
        let result = invoke_raw!(
            hash,
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn is_mission_complete_playing_raw(
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x19A30C23F5827F8Au32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x19A30C23F5827F8Au64;

        invoke_raw_typed!(
            hash,
        )
    }
}

/// This native has a new argument on newer game builds:
*



pub fn request_script_audio_bank_safe(
        
        
            bankName: 
        , 
        
        
            bOverNetwork: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x2F844A8B08D76685u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x2F844A8B08D76685u64;
        
        let result = invoke_raw!(
            hash,
                bankName, 
                bOverNetwork
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn request_script_audio_bank_raw(
        bankName: , 
        bOverNetwork: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x2F844A8B08D76685u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x2F844A8B08D76685u64;

        invoke_raw_typed!(
            hash,
                bankName, 
                bOverNetwork
        )
    }
}

/// Unlocks any available DJ radio tracks based on the tuneable status

```	
NativeDB Introduced: v1493	
```



pub fn update_unlockable_dj_radio_tracks_safe(
        
        
            allowTrackReprioritization: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x47AED84213A47510u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x47AED84213A47510u64;
        
        let result = invoke_raw!(
            hash,
                allowTrackReprioritization
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn update_unlockable_dj_radio_tracks_raw(
        allowTrackReprioritization: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x47AED84213A47510u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x47AED84213A47510u64;

        invoke_raw_typed!(
            hash,
                allowTrackReprioritization
        )
    }
}

/// ## Parameters
*



pub fn stop_all_alarms_safe(
        
        
            instantStop: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x2F794A877ADD4C92u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x2F794A877ADD4C92u64;
        
        let result = invoke_raw!(
            hash,
                instantStop
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn stop_all_alarms_raw(
        instantStop: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x2F794A877ADD4C92u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x2F794A877ADD4C92u64;

        invoke_raw_typed!(
            hash,
                instantStop
        )
    }
}

/// ## Parameters
*



pub fn can_vehicle_receive_cb_radio_safe(
        
        
            vehicle: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x032A116663A4D5ACu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x032A116663A4D5ACu64;
        
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
pub fn can_vehicle_receive_cb_radio_raw(
        vehicle: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x032A116663A4D5ACu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x032A116663A4D5ACu64;

        invoke_raw_typed!(
            hash,
                vehicle
        )
    }
}

/// ## Parameters
*



pub fn _0x11579d940949c49e_safe(
        
        
            p0: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x11579D940949C49Eu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x11579D940949C49Eu64;
        
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
pub fn _0x11579d940949c49e_raw(
        p0: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x11579D940949C49Eu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x11579D940949C49Eu64;

        invoke_raw_typed!(
            hash,
                p0
        )
    }
}

/// Load in named stream. Optionally can specify a sound set which contains the sound specified by name.



pub fn load_stream_with_start_offset_safe(
        
        
            streamName: 
        , 
        
        
            startOffset: 
        , 
        
        
            soundSet: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x59C16B79F53B3712u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x59C16B79F53B3712u64;
        
        let result = invoke_raw!(
            hash,
                streamName, 
                startOffset, 
                soundSet
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn load_stream_with_start_offset_raw(
        streamName: , 
        startOffset: , 
        soundSet: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x59C16B79F53B3712u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x59C16B79F53B3712u64;

        invoke_raw_typed!(
            hash,
                streamName, 
                startOffset, 
                soundSet
        )
    }
}

/// ## Parameters
*



pub fn is_any_speech_playing_safe(
        
        
            ped: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x729072355FA39EC9u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x729072355FA39EC9u64;
        
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
pub fn is_any_speech_playing_raw(
        ped: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x729072355FA39EC9u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x729072355FA39EC9u64;

        invoke_raw_typed!(
            hash,
                ped
        )
    }
}

/// ## Parameters
*



pub fn play_synchronized_audio_event_safe(
        
        
            sceneId: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x8B2FD4560E55DD2Du32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x8B2FD4560E55DD2Du64;
        
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
pub fn play_synchronized_audio_event_raw(
        sceneId: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x8B2FD4560E55DD2Du32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x8B2FD4560E55DD2Du64;

        invoke_raw_typed!(
            hash,
                sceneId
        )
    }
}

/// ## Parameters
*



pub fn disable_ped_pain_audio_safe(
        
        
            ped: 
        , 
        
        
            shouldDisable: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xA9A41C1E940FB0E8u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xA9A41C1E940FB0E8u64;
        
        let result = invoke_raw!(
            hash,
                ped, 
                shouldDisable
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn disable_ped_pain_audio_raw(
        ped: , 
        shouldDisable: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xA9A41C1E940FB0E8u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xA9A41C1E940FB0E8u64;

        invoke_raw_typed!(
            hash,
                ped, 
                shouldDisable
        )
    }
}

/// ```
NativeDB Introduced: v1493
```



pub fn _set_radio_track_mix_safe(
        
        
            radioStationName: 
        , 
        
        
            mixName: 
        , 
        
        
            p2: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x2CB0075110BE1E56u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x2CB0075110BE1E56u64;
        
        let result = invoke_raw!(
            hash,
                radioStationName, 
                mixName, 
                p2
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn _set_radio_track_mix_raw(
        radioStationName: , 
        mixName: , 
        p2: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x2CB0075110BE1E56u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x2CB0075110BE1E56u64;

        invoke_raw_typed!(
            hash,
                radioStationName, 
                mixName, 
                p2
        )
    }
}

/// ```cpp
enum eAudibility {
	AUD_AUDIBILITY_NORMAL = 0,
	AUD_AUDIBILITY_CLEAR = 1,
	AUD_AUDIBILITY_CRITICAL = 2,
	AUD_AUDIBILITY_LEAD_IN = 3
}
```



pub fn add_line_to_conversation_safe(
        
        
            speakerConversationIndex: 
        , 
        
        
            context: 
        , 
        
        
            subtitle: 
        , 
        
        
            listenerNumber: 
        , 
        
        
            volumeType: 
        , 
        
        
            isRandom: 
        , 
        
        
            interruptible: 
        , 
        
        
            ducksRadio: 
        , 
        
        
            ducksScore: 
        , 
        
        
            audibility: 
        , 
        
        
            headset: 
        , 
        
        
            dontInterruptForSpecialAbility: 
        , 
        
        
            isPadSpeakerRoute: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xC5EF963405593646u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xC5EF963405593646u64;
        
        let result = invoke_raw!(
            hash,
                speakerConversationIndex, 
                context, 
                subtitle, 
                listenerNumber, 
                volumeType, 
                isRandom, 
                interruptible, 
                ducksRadio, 
                ducksScore, 
                audibility, 
                headset, 
                dontInterruptForSpecialAbility, 
                isPadSpeakerRoute
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn add_line_to_conversation_raw(
        speakerConversationIndex: , 
        context: , 
        subtitle: , 
        listenerNumber: , 
        volumeType: , 
        isRandom: , 
        interruptible: , 
        ducksRadio: , 
        ducksScore: , 
        audibility: , 
        headset: , 
        dontInterruptForSpecialAbility: , 
        isPadSpeakerRoute: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xC5EF963405593646u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xC5EF963405593646u64;

        invoke_raw_typed!(
            hash,
                speakerConversationIndex, 
                context, 
                subtitle, 
                listenerNumber, 
                volumeType, 
                isRandom, 
                interruptible, 
                ducksRadio, 
                ducksScore, 
                audibility, 
                headset, 
                dontInterruptForSpecialAbility, 
                isPadSpeakerRoute
        )
    }
}

/// Prepares the specified music event. Preparing it in advance will preload any required data so that it's ready to play immediately.



pub fn prepare_music_event_safe(
        
        
            eventName: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x1E5185B72EF5158Au32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x1E5185B72EF5158Au64;
        
        let result = invoke_raw!(
            hash,
                eventName
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn prepare_music_event_raw(
        eventName: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x1E5185B72EF5158Au32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x1E5185B72EF5158Au64;

        invoke_raw_typed!(
            hash,
                eventName
        )
    }
}

/// Updates a playing sounds absolute position.



pub fn update_sound_coord_safe(
        
        
            soundId: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x7EC3C679D0E7E46Bu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x7EC3C679D0E7E46Bu64;
        
        let result = invoke_raw!(
            hash,
                soundId
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn update_sound_coord_raw(
        soundId: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x7EC3C679D0E7E46Bu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x7EC3C679D0E7E46Bu64;

        invoke_raw_typed!(
            hash,
                soundId
        )
    }
}

/// Stops a ped's ringtone from playing



pub fn stop_ped_ringtone_safe(
        
        
            ped: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x6C5AE23EFA885092u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x6C5AE23EFA885092u64;
        
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
pub fn stop_ped_ringtone_raw(
        ped: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x6C5AE23EFA885092u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x6C5AE23EFA885092u64;

        invoke_raw_typed!(
            hash,
                ped
        )
    }
}

/// ... When not in a vehicle

```
NativeDB Introduced: v1290
```



pub fn _trigger_siren_safe(
        
        
            vehicle: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x66C3FB05206041BAu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x66C3FB05206041BAu64;
        
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
pub fn _trigger_siren_raw(
        vehicle: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x66C3FB05206041BAu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x66C3FB05206041BAu64;

        invoke_raw_typed!(
            hash,
                vehicle
        )
    }
}

/// This native is marked as deprecated internally, please use [REQUEST_SCRIPT_AUDIO_BANK](#_0x2F844A8B08D76685)

This native has a new argument on newer game builds:
*



pub fn request_ambient_audio_bank_safe(
        
        
            bankName: 
        , 
        
        
            bOverNetwork: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xFE02FFBED8CA9D99u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xFE02FFBED8CA9D99u64;
        
        let result = invoke_raw!(
            hash,
                bankName, 
                bOverNetwork
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn request_ambient_audio_bank_raw(
        bankName: , 
        bOverNetwork: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xFE02FFBED8CA9D99u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xFE02FFBED8CA9D99u64;

        invoke_raw_typed!(
            hash,
                bankName, 
                bOverNetwork
        )
    }
}

/// Allows the radio to be played in the frontend.



pub fn set_frontend_radio_active_safe(
        
        
            active: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xF7F26C6E9CC9EBB8u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xF7F26C6E9CC9EBB8u64;
        
        let result = invoke_raw!(
            hash,
                active
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn set_frontend_radio_active_raw(
        active: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xF7F26C6E9CC9EBB8u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xF7F26C6E9CC9EBB8u64;

        invoke_raw_typed!(
            hash,
                active
        )
    }
}

/// Request that we preload the required audio bank for a given vehicle model.


```
NativeDB Introduced: v1180
```



pub fn preload_vehicle_audio_bank_safe(
        
        
            model: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xCA4CEA6AE0000A7Eu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xCA4CEA6AE0000A7Eu64;
        
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
pub fn preload_vehicle_audio_bank_raw(
        model: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xCA4CEA6AE0000A7Eu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xCA4CEA6AE0000A7Eu64;

        invoke_raw_typed!(
            hash,
                model
        )
    }
}

/// Allows script to freeze the microphone for a single frame, mantaining its current transform/settings.
This native should be called every frame you want to keep the microphone frozen, when you stop calling it it will automatically unfreeze



pub fn freeze_microphone_safe(
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xD57AAAE0E2214D11u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xD57AAAE0E2214D11u64;
        
        let result = invoke_raw!(
            hash,
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn freeze_microphone_raw(
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xD57AAAE0E2214D11u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xD57AAAE0E2214D11u64;

        invoke_raw_typed!(
            hash,
        )
    }
}

/// ## Return value
Returns true of a mobile phone call is currently happening.



pub fn is_mobile_phone_call_ongoing_safe(
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x7497D2CE2C30D24Cu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x7497D2CE2C30D24Cu64;
        
        let result = invoke_raw!(
            hash,
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn is_mobile_phone_call_ongoing_raw(
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x7497D2CE2C30D24Cu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x7497D2CE2C30D24Cu64;

        invoke_raw_typed!(
            hash,
        )
    }
}

/// ## Parameters
*



pub fn set_ambient_zone_state_persistent_safe(
        
        
            zoneName: 
        , 
        
        
            enabled: 
        , 
        
        
            forceUpdate: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x1D6650420CEC9D3Bu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x1D6650420CEC9D3Bu64;
        
        let result = invoke_raw!(
            hash,
                zoneName, 
                enabled, 
                forceUpdate
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn set_ambient_zone_state_persistent_raw(
        zoneName: , 
        enabled: , 
        forceUpdate: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x1D6650420CEC9D3Bu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x1D6650420CEC9D3Bu64;

        invoke_raw_typed!(
            hash,
                zoneName, 
                enabled, 
                forceUpdate
        )
    }
}

/// ```c
enum eAudAnimalType {
	AUD_ANIMAL_NONE = -1,
	AUD_ANIMAL_BOAR = 0,
	AUD_ANIMAL_CHICKEN = 1,
	AUD_ANIMAL_DOG = 2,
	AUD_ANIMAL_DOG_ROTTWEILER = 3,
	AUD_ANIMAL_HORSE = 4,
	AUD_NUM_ANIMALS = 5
}
```



pub fn play_animal_vocalization_safe(
        
        
            pedHandle: 
        , 
        
        
            animalType: 
        , 
        
        
            speechName: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xEE066C7006C49C0Au32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xEE066C7006C49C0Au64;
        
        let result = invoke_raw!(
            hash,
                pedHandle, 
                animalType, 
                speechName
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn play_animal_vocalization_raw(
        pedHandle: , 
        animalType: , 
        speechName: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xEE066C7006C49C0Au32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xEE066C7006C49C0Au64;

        invoke_raw_typed!(
            hash,
                pedHandle, 
                animalType, 
                speechName
        )
    }
}

/// Clears the override set by [OVERRIDE_TREVOR_RAGE](#_0x13AD665062541A7E)



pub fn reset_trevor_rage_safe(
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xE78503B10C4314E0u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xE78503B10C4314E0u64;
        
        let result = invoke_raw!(
            hash,
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn reset_trevor_rage_raw(
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xE78503B10C4314E0u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xE78503B10C4314E0u64;

        invoke_raw_typed!(
            hash,
        )
    }
}

/// SKIP_TO_NEXT_SCRIPTED_CONVERSATION_LINE native function



pub fn skip_to_next_scripted_conversation_line_safe(
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x9663FE6B7A61EB00u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x9663FE6B7A61EB00u64;
        
        let result = invoke_raw!(
            hash,
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn skip_to_next_scripted_conversation_line_raw(
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x9663FE6B7A61EB00u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x9663FE6B7A61EB00u64;

        invoke_raw_typed!(
            hash,
        )
    }
}

/// ```
Called 38 times in the scripts. There are 5 different audioNames used.
 One unknown removed below.
AUDIO::PLAY_MISSION_COMPLETE_AUDIO("DEAD");
AUDIO::PLAY_MISSION_COMPLETE_AUDIO("FRANKLIN_BIG_01");
AUDIO::PLAY_MISSION_COMPLETE_AUDIO("GENERIC_FAILED");
AUDIO::PLAY_MISSION_COMPLETE_AUDIO("TREVOR_SMALL_01");
```



pub fn play_mission_complete_audio_safe(
        
        
            audioName: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xB138AAB8A70D3C69u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xB138AAB8A70D3C69u64;
        
        let result = invoke_raw!(
            hash,
                audioName
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn play_mission_complete_audio_raw(
        audioName: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xB138AAB8A70D3C69u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xB138AAB8A70D3C69u64;

        invoke_raw_typed!(
            hash,
                audioName
        )
    }
}

/// Stops all smoke grenade sounds



pub fn stop_smoke_grenade_explosion_sounds_safe(
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xE4E6DD5566D28C82u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xE4E6DD5566D28C82u64;
        
        let result = invoke_raw!(
            hash,
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn stop_smoke_grenade_explosion_sounds_raw(
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xE4E6DD5566D28C82u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xE4E6DD5566D28C82u64;

        invoke_raw_typed!(
            hash,
        )
    }
}

/// ## Parameters
*



pub fn get_variation_chosen_for_scripted_line_safe(
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xAA19F5572C38B564u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xAA19F5572C38B564u64;
        
        let result = invoke_raw!(
            hash,
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn get_variation_chosen_for_scripted_line_raw(
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xAA19F5572C38B564u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xAA19F5572C38B564u64;

        invoke_raw_typed!(
            hash,
        )
    }
}

/// ## Return value



pub fn is_scripted_conversation_ongoing_safe(
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x16754C556D2EDE3Du32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x16754C556D2EDE3Du64;
        
        let result = invoke_raw!(
            hash,
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn is_scripted_conversation_ongoing_raw(
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x16754C556D2EDE3Du32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x16754C556D2EDE3Du64;

        invoke_raw_typed!(
            hash,
        )
    }
}

/// ## Parameters
*



pub fn set_initial_player_station_safe(
        
        
            radioStation: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x88795F13FACDA88Du32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x88795F13FACDA88Du64;
        
        let result = invoke_raw!(
            hash,
                radioStation
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn set_initial_player_station_raw(
        radioStation: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x88795F13FACDA88Du32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x88795F13FACDA88Du64;

        invoke_raw_typed!(
            hash,
                radioStation
        )
    }
}

/// ## Parameters
*



pub fn set_mobile_radio_enabled_during_gameplay_safe(
        
        
            toggle: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x1098355A16064BB3u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x1098355A16064BB3u64;
        
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
pub fn set_mobile_radio_enabled_during_gameplay_raw(
        toggle: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x1098355A16064BB3u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x1098355A16064BB3u64;

        invoke_raw_typed!(
            hash,
                toggle
        )
    }
}

/// Restarts a conversation that was previously paused with [PAUSE_SCRIPTED_CONVERSATION](#_0x8530AD776CD72B12)



pub fn restart_scripted_conversation_safe(
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x9AEB285D1818C9ACu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x9AEB285D1818C9ACu64;
        
        let result = invoke_raw!(
            hash,
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn restart_scripted_conversation_raw(
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x9AEB285D1818C9ACu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x9AEB285D1818C9ACu64;

        invoke_raw_typed!(
            hash,
        )
    }
}

/// Similar to [START_SCRIPT_CONVERSATION](#_0x6B17C62C9635D2DC), except that is starts the conversation off paused.

A scripter can then kick off the conversation by calling [START_PRELOADED_CONVERSATION](#_0x23641AFE870AF385).

If they want to check that the conversation is done preloading, they can use [GET_IS_PRELOADED_CONVERSATION_READY](#_0xE73364DB90778FFA)



pub fn preload_script_conversation_safe(
        
        
            displaySubtitles: 
        , 
        
        
            addToBriefScreen: 
        , 
        
        
            cloneConversation: 
        , 
        
        
            interruptible: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x3B3CAD6166916D87u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x3B3CAD6166916D87u64;
        
        let result = invoke_raw!(
            hash,
                displaySubtitles, 
                addToBriefScreen, 
                cloneConversation, 
                interruptible
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn preload_script_conversation_raw(
        displaySubtitles: , 
        addToBriefScreen: , 
        cloneConversation: , 
        interruptible: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x3B3CAD6166916D87u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x3B3CAD6166916D87u64;

        invoke_raw_typed!(
            hash,
                displaySubtitles, 
                addToBriefScreen, 
                cloneConversation, 
                interruptible
        )
    }
}

/// Load in named stream. Optionally can specify a sound set which contains the sound specified by name.

Names for the streams can be found [here](https://gist.github.com/4mmonium/2bd2c9c54d6ca5cbdb7b156a82a3a85a
), the list will be updated as more are found.



pub fn load_stream_safe(
        
        
            streamName: 
        , 
        
        
            soundSet: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x1F1F957154EC51DFu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x1F1F957154EC51DFu64;
        
        let result = invoke_raw!(
            hash,
                streamName, 
                soundSet
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn load_stream_raw(
        streamName: , 
        soundSet: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x1F1F957154EC51DFu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x1F1F957154EC51DFu64;

        invoke_raw_typed!(
            hash,
                streamName, 
                soundSet
        )
    }
}

/// ```
Sets radio station by index.  
```



pub fn set_radio_to_station_index_safe(
        
        
            radioStation: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xA619B168B8A8570Fu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xA619B168B8A8570Fu64;
        
        let result = invoke_raw!(
            hash,
                radioStation
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn set_radio_to_station_index_raw(
        radioStation: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xA619B168B8A8570Fu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xA619B168B8A8570Fu64;

        invoke_raw_typed!(
            hash,
                radioStation
        )
    }
}

/// ## Parameters
*



pub fn get_network_id_from_sound_id_safe(
        
        
            soundId: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x2DE3F0A134FFBC0Du32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x2DE3F0A134FFBC0Du64;
        
        let result = invoke_raw!(
            hash,
                soundId
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn get_network_id_from_sound_id_raw(
        soundId: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x2DE3F0A134FFBC0Du32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x2DE3F0A134FFBC0Du64;

        invoke_raw_typed!(
            hash,
                soundId
        )
    }
}

/// Sets the specified ped to use a specific voice different to the one associated with their model.



pub fn set_ambient_voice_name_hash_safe(
        
        
            ped: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x9A53DED9921DE990u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x9A53DED9921DE990u64;
        
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
pub fn set_ambient_voice_name_hash_raw(
        ped: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x9A53DED9921DE990u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x9A53DED9921DE990u64;

        invoke_raw_typed!(
            hash,
                ped
        )
    }
}

/// This native has been marked as deprecated internally, please use [RELEASE_SCRIPT_AUDIO_BANK](#_0x7A2D8AD0A9EB9C3F) instead.



pub fn release_mission_audio_bank_safe(
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x0EC92A1BF0857187u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x0EC92A1BF0857187u64;
        
        let result = invoke_raw!(
            hash,
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn release_mission_audio_bank_raw(
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x0EC92A1BF0857187u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x0EC92A1BF0857187u64;

        invoke_raw_typed!(
            hash,
        )
    }
}

/// Plays a preloaded stream back from the specified ped.



pub fn play_stream_from_ped_safe(
        
        
            ped: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x89049DD63C08B5D1u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x89049DD63C08B5D1u64;
        
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
pub fn play_stream_from_ped_raw(
        ped: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x89049DD63C08B5D1u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x89049DD63C08B5D1u64;

        invoke_raw_typed!(
            hash,
                ped
        )
    }
}

/// ## Parameters
*



pub fn set_gps_active_safe(
        
        
            active: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x3BD3F52BA9B1E4E8u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x3BD3F52BA9B1E4E8u64;
        
        let result = invoke_raw!(
            hash,
                active
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn set_gps_active_raw(
        active: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x3BD3F52BA9B1E4E8u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x3BD3F52BA9B1E4E8u64;

        invoke_raw_typed!(
            hash,
                active
        )
    }
}

/// Needs to be called every frame.

```c
enum eAudSpecialEffectMode
{
	kSpecialEffectModeNormal = 0,
	kSpecialEffectModeUnderwater = 1,
	kSpecialEffectModeStoned = 2,
	kSpecialEffectModePauseMenu = 3,
	kSpecialEffectModeSlowMotion = 4,
	kSpecialEffectModeDrunkStage01 = 5,
	kSpecialEffectModeDrunkStage02 = 6,
	kSpecialEffectModeDrunkStage03 = 7,
	NUM_AUDSPECIALEFFECTMODE
};
```



pub fn set_audio_special_effect_mode_safe(
        
        
            mode: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x12561FCBB62D5B9Cu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x12561FCBB62D5B9Cu64;
        
        let result = invoke_raw!(
            hash,
                mode
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn set_audio_special_effect_mode_raw(
        mode: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x12561FCBB62D5B9Cu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x12561FCBB62D5B9Cu64;

        invoke_raw_typed!(
            hash,
                mode
        )
    }
}

/// ```
NativeDB Introduced: v1365
```



pub fn get_vehicle_horn_sound_index_safe(
        
        
            vehicle: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xD53F3A29BCE2580Eu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xD53F3A29BCE2580Eu64;
        
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
pub fn get_vehicle_horn_sound_index_raw(
        vehicle: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xD53F3A29BCE2580Eu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xD53F3A29BCE2580Eu64;

        invoke_raw_typed!(
            hash,
                vehicle
        )
    }
}

/// ## Return value
Returns true if the radio is tuning to a station



pub fn is_radio_retuning_safe(
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xA151A7394A214E65u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xA151A7394A214E65u64;
        
        let result = invoke_raw!(
            hash,
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn is_radio_retuning_raw(
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xA151A7394A214E65u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xA151A7394A214E65u64;

        invoke_raw_typed!(
            hash,
        )
    }
}

/// Enable the stunt jump audio detection code

This native is meant to be called per-frame for as long as detection is wanted.



pub fn enable_stunt_jump_audio_safe(
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xB81CF134AEB56FFBu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xB81CF134AEB56FFBu64;
        
        let result = invoke_raw!(
            hash,
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn enable_stunt_jump_audio_raw(
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xB81CF134AEB56FFBu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xB81CF134AEB56FFBu64;

        invoke_raw_typed!(
            hash,
        )
    }
}

/// ## Parameters
*



pub fn get_vehicle_default_horn_safe(
        
        
            vehicle: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x02165D55000219ACu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x02165D55000219ACu64;
        
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
pub fn get_vehicle_default_horn_raw(
        vehicle: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x02165D55000219ACu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x02165D55000219ACu64;

        invoke_raw_typed!(
            hash,
                vehicle
        )
    }
}

/// ```
NativeDB Introduced: v1868
```



pub fn _0x55ecf4d13d9903b0_safe(
        
        
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
        let hash = 0x55ECF4D13D9903B0u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x55ECF4D13D9903B0u64;
        
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
pub fn _0x55ecf4d13d9903b0_raw(
        p0: , 
        p1: , 
        p2: , 
        p3: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x55ECF4D13D9903B0u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x55ECF4D13D9903B0u64;

        invoke_raw_typed!(
            hash,
                p0, 
                p1, 
                p2, 
                p3
        )
    }
}

/// Stops all mixed scenes which were previously started by this script



pub fn stop_audio_scenes_safe(
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xBAC7FC81A75EC1A1u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xBAC7FC81A75EC1A1u64;
        
        let result = invoke_raw!(
            hash,
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn stop_audio_scenes_raw(
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xBAC7FC81A75EC1A1u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xBAC7FC81A75EC1A1u64;

        invoke_raw_typed!(
            hash,
        )
    }
}

/// ## Return value
Returns true if the radio is currently faded out



pub fn is_radio_faded_out_safe(
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x0626A247D2405330u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x0626A247D2405330u64;
        
        let result = invoke_raw!(
            hash,
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn is_radio_faded_out_raw(
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x0626A247D2405330u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x0626A247D2405330u64;

        invoke_raw_typed!(
            hash,
        )
    }
}

/// **This native does absolutely nothing, just a nullsub**



pub fn register_script_with_audio_safe(
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xC6ED9D5092438D91u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xC6ED9D5092438D91u64;
        
        let result = invoke_raw!(
            hash,
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn register_script_with_audio_raw(
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xC6ED9D5092438D91u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xC6ED9D5092438D91u64;

        invoke_raw_typed!(
            hash,
        )
    }
}

/// _0x9AC92EED5E4793AB native function



pub fn _0x9ac92eed5e4793ab_safe(
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x9AC92EED5E4793ABu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x9AC92EED5E4793ABu64;
        
        let result = invoke_raw!(
            hash,
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn _0x9ac92eed5e4793ab_raw(
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x9AC92EED5E4793ABu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x9AC92EED5E4793ABu64;

        invoke_raw_typed!(
            hash,
        )
    }
}

/// ## Parameters
*



pub fn unfreeze_radio_station_safe(
        
        
            radioStation: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xFC00454CF60B91DDu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xFC00454CF60B91DDu64;
        
        let result = invoke_raw!(
            hash,
                radioStation
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn unfreeze_radio_station_raw(
        radioStation: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xFC00454CF60B91DDu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xFC00454CF60B91DDu64;

        invoke_raw_typed!(
            hash,
                radioStation
        )
    }
}

/// ## Parameters
*



pub fn lock_radio_station_track_list_safe(
        
        
            radioStation: 
        , 
        
        
            trackListName: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xFF5E5EA2DCEEACF3u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xFF5E5EA2DCEEACF3u64;
        
        let result = invoke_raw!(
            hash,
                radioStation, 
                trackListName
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn lock_radio_station_track_list_raw(
        radioStation: , 
        trackListName: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xFF5E5EA2DCEEACF3u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xFF5E5EA2DCEEACF3u64;

        invoke_raw_typed!(
            hash,
                radioStation, 
                trackListName
        )
    }
}

/// ## Parameters
*



pub fn enable_vehicle_fanbelt_damage_safe(
        
        
            vehicle: 
        , 
        
        
            enableFanbeltDamage: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x1C073274E065C6D2u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x1C073274E065C6D2u64;
        
        let result = invoke_raw!(
            hash,
                vehicle, 
                enableFanbeltDamage
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn enable_vehicle_fanbelt_damage_raw(
        vehicle: , 
        enableFanbeltDamage: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x1C073274E065C6D2u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x1C073274E065C6D2u64;

        invoke_raw_typed!(
            hash,
                vehicle, 
                enableFanbeltDamage
        )
    }
}

/// ```
SET_VEHICLE_BOOST_ACTIVE(vehicle, 1, 0);  
SET_VEHICLE_BOOST_ACTIVE(vehicle, 0, 0);   
Will give a boost-soundeffect.  
```



pub fn set_vehicle_boost_active_safe(
        
        
            vehicle: 
        , 
        
        
            toggle: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x4A04DE7CAB2739A1u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x4A04DE7CAB2739A1u64;
        
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
pub fn set_vehicle_boost_active_raw(
        vehicle: , 
        toggle: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x4A04DE7CAB2739A1u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x4A04DE7CAB2739A1u64;

        invoke_raw_typed!(
            hash,
                vehicle, 
                toggle
        )
    }
}

/// ## Parameters
*



pub fn is_ambient_zone_enabled_safe(
        
        
            ambientZone: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x01E2817A479A7F9Bu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x01E2817A479A7F9Bu64;
        
        let result = invoke_raw!(
            hash,
                ambientZone
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn is_ambient_zone_enabled_raw(
        ambientZone: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x01E2817A479A7F9Bu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x01E2817A479A7F9Bu64;

        invoke_raw_typed!(
            hash,
                ambientZone
        )
    }
}

/// Find the radio station list [here](https://gist.github.com/4mmonium/b47d6512a2d992cbf4eea15d9038b581)



pub fn set_radio_to_station_name_safe(
        
        
            stationName: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xC69EDA28699D5107u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xC69EDA28699D5107u64;
        
        let result = invoke_raw!(
            hash,
                stationName
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn set_radio_to_station_name_raw(
        stationName: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xC69EDA28699D5107u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xC69EDA28699D5107u64;

        invoke_raw_typed!(
            hash,
                stationName
        )
    }
}

/// This native enables the audio flag "TrevorRageIsOverridden" and sets the voice effect to `voiceEffect`

To clear the override use [RESET_TREVOR_RAGE](#_0xE78503B10C4314E0)



pub fn override_trevor_rage_safe(
        
        
            voiceEffect: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x13AD665062541A7Eu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x13AD665062541A7Eu64;
        
        let result = invoke_raw!(
            hash,
                voiceEffect
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn override_trevor_rage_raw(
        voiceEffect: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x13AD665062541A7Eu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x13AD665062541A7Eu64;

        invoke_raw_typed!(
            hash,
                voiceEffect
        )
    }
}

/// ```
All found occurrences in b617d, sorted alphabetically and identical lines removed: pastebin.com/RFb4GTny  
AUDIO::PLAY_PED_RINGTONE("Remote_Ring", PLAYER::PLAYER_PED_ID(), 1);  
AUDIO::PLAY_PED_RINGTONE("Dial_and_Remote_Ring", PLAYER::PLAYER_PED_ID(), 1);  
```



pub fn play_ped_ringtone_safe(
        
        
            ringtoneName: 
        , 
        
        
            ped: 
        , 
        
        
            p2: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xF9E56683CA8E11A5u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xF9E56683CA8E11A5u64;
        
        let result = invoke_raw!(
            hash,
                ringtoneName, 
                ped, 
                p2
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn play_ped_ringtone_raw(
        ringtoneName: , 
        ped: , 
        p2: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xF9E56683CA8E11A5u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xF9E56683CA8E11A5u64;

        invoke_raw_typed!(
            hash,
                ringtoneName, 
                ped, 
                p2
        )
    }
}

/// ## Parameters
*



pub fn is_horn_active_safe(
        
        
            vehicle: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x9D6BFC12B05C6121u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x9D6BFC12B05C6121u64;
        
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
pub fn is_horn_active_raw(
        vehicle: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x9D6BFC12B05C6121u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x9D6BFC12B05C6121u64;

        invoke_raw_typed!(
            hash,
                vehicle
        )
    }
}

/// ## Parameters
*



pub fn _0x8bf907833be275de_safe(
        
        
            p0: 
        , 
        
        
            p1: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x8BF907833BE275DEu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x8BF907833BE275DEu64;
        
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
pub fn _0x8bf907833be275de_raw(
        p0: , 
        p1: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x8BF907833BE275DEu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x8BF907833BE275DEu64;

        invoke_raw_typed!(
            hash,
                p0, 
                p1
        )
    }
}

/// ## Parameters
*



pub fn is_ped_ringtone_playing_safe(
        
        
            ped: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x1E8E5E20937E3137u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x1E8E5E20937E3137u64;
        
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
pub fn is_ped_ringtone_playing_raw(
        ped: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x1E8E5E20937E3137u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x1E8E5E20937E3137u64;

        invoke_raw_typed!(
            hash,
                ped
        )
    }
}

/// To resume the conversation use [RESTART_SCRIPTED_CONVERSATION](#_0x9AEB285D1818C9AC)



pub fn pause_scripted_conversation_safe(
        
        
            finishCurrentLine: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x8530AD776CD72B12u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x8530AD776CD72B12u64;
        
        let result = invoke_raw!(
            hash,
                finishCurrentLine
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn pause_scripted_conversation_raw(
        finishCurrentLine: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x8530AD776CD72B12u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x8530AD776CD72B12u64;

        invoke_raw_typed!(
            hash,
                finishCurrentLine
        )
    }
}

/// Unloads the specified audioBank



pub fn release_named_script_audio_bank_safe(
        
        
            audioBank: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x77ED170667F50170u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x77ED170667F50170u64;
        
        let result = invoke_raw!(
            hash,
                audioBank
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn release_named_script_audio_bank_raw(
        audioBank: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x77ED170667F50170u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x77ED170667F50170u64;

        invoke_raw_typed!(
            hash,
                audioBank
        )
    }
}

/// This disables the radio station completely - it won't be selectable on the radio wheel or ever be heard coming from a vehicle/ambient emitter

```
NativeDB Introduced: v1493
```



pub fn lock_radio_station_safe(
        
        
            radioStationName: 
        , 
        
        
            toggle: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x477D9DB48F889591u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x477D9DB48F889591u64;
        
        let result = invoke_raw!(
            hash,
                radioStationName, 
                toggle
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn lock_radio_station_raw(
        radioStationName: , 
        toggle: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x477D9DB48F889591u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x477D9DB48F889591u64;

        invoke_raw_typed!(
            hash,
                radioStationName, 
                toggle
        )
    }
}

/// You should call [PREPARE_ALARM](#_0x9D74AE343DB65533) and wait for its value to be true before using this



pub fn start_alarm_safe(
        
        
            alarmName: 
        , 
        
        
            skipStartup: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x0355EF116C4C97B2u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x0355EF116C4C97B2u64;
        
        let result = invoke_raw!(
            hash,
                alarmName, 
                skipStartup
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn start_alarm_raw(
        alarmName: , 
        skipStartup: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x0355EF116C4C97B2u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x0355EF116C4C97B2u64;

        invoke_raw_typed!(
            hash,
                alarmName, 
                skipStartup
        )
    }
}

/// ## Parameters
*



pub fn is_mission_news_story_unlocked_safe(
        
        
            newsStory: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x66E49BF55B4B1874u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x66E49BF55B4B1874u64;
        
        let result = invoke_raw!(
            hash,
                newsStory
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn is_mission_news_story_unlocked_raw(
        newsStory: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x66E49BF55B4B1874u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x66E49BF55B4B1874u64;

        invoke_raw_typed!(
            hash,
                newsStory
        )
    }
}

/// ## Return value
Returns the number of radio stations currently unlocked in the game. This can change as a result of DLC and/or script calls.



pub fn get_num_unlocked_radio_stations_safe(
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xF1620ECB50E01DE7u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xF1620ECB50E01DE7u64;
        
        let result = invoke_raw!(
            hash,
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn get_num_unlocked_radio_stations_raw(
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xF1620ECB50E01DE7u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xF1620ECB50E01DE7u64;

        invoke_raw_typed!(
            hash,
        )
    }
}

/// Enable or disable the plane stall warning sounds



pub fn enable_stall_warning_sounds_safe(
        
        
            vehicle: 
        , 
        
        
            enable: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xC15907D667F7CFB2u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xC15907D667F7CFB2u64;
        
        let result = invoke_raw!(
            hash,
                vehicle, 
                enable
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn enable_stall_warning_sounds_raw(
        vehicle: , 
        enable: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xC15907D667F7CFB2u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xC15907D667F7CFB2u64;

        invoke_raw_typed!(
            hash,
                vehicle, 
                enable
        )
    }
}

/// Retunes a named static emitter to the specified station



pub fn set_emitter_radio_station_safe(
        
        
            emitterName: 
        , 
        
        
            radioStation: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xACF57305B12AF907u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xACF57305B12AF907u64;
        
        let result = invoke_raw!(
            hash,
                emitterName, 
                radioStation
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn set_emitter_radio_station_raw(
        emitterName: , 
        radioStation: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xACF57305B12AF907u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xACF57305B12AF907u64;

        invoke_raw_typed!(
            hash,
                emitterName, 
                radioStation
        )
    }
}

/// This native sets the audio to the specific vehicle hash's audioNameHash.



pub fn force_use_audio_game_object_safe(
        
        
            vehicle: 
        , 
        
        
            gameObjectName: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x4F0C413926060B38u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x4F0C413926060B38u64;
        
        let result = invoke_raw!(
            hash,
                vehicle, 
                gameObjectName
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn force_use_audio_game_object_raw(
        vehicle: , 
        gameObjectName: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x4F0C413926060B38u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x4F0C413926060B38u64;

        invoke_raw_typed!(
            hash,
                vehicle, 
                gameObjectName
        )
    }
}

/// ## Parameters
*



pub fn start_script_phone_conversation_safe(
        
        
            displaySubtitles: 
        , 
        
        
            addToBriefScreen: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x252E5F915EABB675u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x252E5F915EABB675u64;
        
        let result = invoke_raw!(
            hash,
                displaySubtitles, 
                addToBriefScreen
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn start_script_phone_conversation_raw(
        displaySubtitles: , 
        addToBriefScreen: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x252E5F915EABB675u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x252E5F915EABB675u64;

        invoke_raw_typed!(
            hash,
                displaySubtitles, 
                addToBriefScreen
        )
    }
}

/// ## Return value



pub fn get_current_scripted_conversation_line_safe(
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x480357EE890C295Au32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x480357EE890C295Au64;
        
        let result = invoke_raw!(
            hash,
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn get_current_scripted_conversation_line_raw(
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x480357EE890C295Au32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x480357EE890C295Au64;

        invoke_raw_typed!(
            hash,
        )
    }
}

/// ## Return value
Returns the genre of the players current radio station



pub fn get_player_radio_station_genre_safe(
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xA571991A7FE6CCEBu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xA571991A7FE6CCEBu64;
        
        let result = invoke_raw!(
            hash,
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn get_player_radio_station_genre_raw(
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xA571991A7FE6CCEBu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xA571991A7FE6CCEBu64;

        invoke_raw_typed!(
            hash,
        )
    }
}

/// Vehicle will make a 'rattling' noise when decelerating



pub fn set_vehicle_audio_body_damage_factor_safe(
        
        
            vehicle: 
        , 
        
        
            intensity: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x01BB4D577D38BD9Eu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x01BB4D577D38BD9Eu64;
        
        let result = invoke_raw!(
            hash,
                vehicle, 
                intensity
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn set_vehicle_audio_body_damage_factor_raw(
        vehicle: , 
        intensity: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x01BB4D577D38BD9Eu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x01BB4D577D38BD9Eu64;

        invoke_raw_typed!(
            hash,
                vehicle, 
                intensity
        )
    }
}

/// ## Parameters
*



pub fn set_audio_scene_variable_safe(
        
        
            scene: 
        , 
        
        
            variableName: 
        , 
        
        
            value: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xEF21A9EF089A2668u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xEF21A9EF089A2668u64;
        
        let result = invoke_raw!(
            hash,
                scene, 
                variableName, 
                value
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn set_audio_scene_variable_raw(
        scene: , 
        variableName: , 
        value: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xEF21A9EF089A2668u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xEF21A9EF089A2668u64;

        invoke_raw_typed!(
            hash,
                scene, 
                variableName, 
                value
        )
    }
}

/// Generic interface to toggle audio functionality, with auto-reset on script termination and support for multiple script threads

Flags used in game scripts:
| Flag Name | Description of Usage | 
|



pub fn set_audio_flag_safe(
        
        
            flagName: 
        , 
        
        
            toggle: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xB9EFD5C25018725Au32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xB9EFD5C25018725Au64;
        
        let result = invoke_raw!(
            hash,
                flagName, 
                toggle
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn set_audio_flag_raw(
        flagName: , 
        toggle: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xB9EFD5C25018725Au32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xB9EFD5C25018725Au64;

        invoke_raw_typed!(
            hash,
                flagName, 
                toggle
        )
    }
}

/// ## Parameters
*



pub fn set_vehicle_radio_loud_safe(
        
        
            vehicle: 
        , 
        
        
            loud: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xBB6F1CAEC68B0BCEu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xBB6F1CAEC68B0BCEu64;
        
        let result = invoke_raw!(
            hash,
                vehicle, 
                loud
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn set_vehicle_radio_loud_raw(
        vehicle: , 
        loud: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xBB6F1CAEC68B0BCEu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xBB6F1CAEC68B0BCEu64;

        invoke_raw_typed!(
            hash,
                vehicle, 
                loud
        )
    }
}

/// Sets a player ped to use his ANGRY speech contexts



pub fn set_player_angry_safe(
        
        
            ped: 
        , 
        
        
            isAngry: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xEA241BB04110F091u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xEA241BB04110F091u64;
        
        let result = invoke_raw!(
            hash,
                ped, 
                isAngry
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn set_player_angry_raw(
        ped: , 
        isAngry: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xEA241BB04110F091u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xEA241BB04110F091u64;

        invoke_raw_typed!(
            hash,
                ped, 
                isAngry
        )
    }
}

/// ```
NativeDB Introduced: v2372
```



pub fn _set_siren_keep_on_safe(
        
        
            vehicle: 
        , 
        
        
            toggle: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xF584CF8529B51434u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xF584CF8529B51434u64;
        
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
pub fn _set_siren_keep_on_raw(
        vehicle: , 
        toggle: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xF584CF8529B51434u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xF584CF8529B51434u64;

        invoke_raw_typed!(
            hash,
                vehicle, 
                toggle
        )
    }
}

/// Makes pedestrians sound their horn longer, faster and more agressive when they use their horn.



pub fn set_aggressive_horns_safe(
        
        
            toggle: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x395BF71085D1B1D9u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x395BF71085D1B1D9u64;
        
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
pub fn set_aggressive_horns_raw(
        toggle: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x395BF71085D1B1D9u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x395BF71085D1B1D9u64;

        invoke_raw_typed!(
            hash,
                toggle
        )
    }
}

/// Sets the specified ped to use a specific voice different to the one associated with their model.



pub fn set_ambient_voice_name_safe(
        
        
            ped: 
        , 
        
        
            voiceName: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x6C8065A3B780185Bu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x6C8065A3B780185Bu64;
        
        let result = invoke_raw!(
            hash,
                ped, 
                voiceName
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn set_ambient_voice_name_raw(
        ped: , 
        voiceName: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x6C8065A3B780185Bu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x6C8065A3B780185Bu64;

        invoke_raw_typed!(
            hash,
                ped, 
                voiceName
        )
    }
}

/// ```
From the scripts:
AUDIO::_SET_PED_VOICE_GROUP(PLAYER::PLAYER_PED_ID(), MISC::GET_HASH_KEY("PAIGE_PVG"));
AUDIO::_SET_PED_VOICE_GROUP(PLAYER::PLAYER_PED_ID(), MISC::GET_HASH_KEY("TALINA_PVG"));
AUDIO::_SET_PED_VOICE_GROUP(PLAYER::PLAYER_PED_ID(), MISC::GET_HASH_KEY("FEMALE_LOST_BLACK_PVG"));
AUDIO::_SET_PED_VOICE_GROUP(PLAYER::PLAYER_PED_ID(), MISC::GET_HASH_KEY("FEMALE_LOST_WHITE_PVG"));
```



pub fn _set_ped_voice_group_safe(
        
        
            ped: 
        , 
        
        
            voiceGroupHash: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x7CDC8C3B89F661B3u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x7CDC8C3B89F661B3u64;
        
        let result = invoke_raw!(
            hash,
                ped, 
                voiceGroupHash
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn _set_ped_voice_group_raw(
        ped: , 
        voiceGroupHash: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x7CDC8C3B89F661B3u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x7CDC8C3B89F661B3u64;

        invoke_raw_typed!(
            hash,
                ped, 
                voiceGroupHash
        )
    }
}

/// **This native does absolutely nothing, just a nullsub**

```
On last-gen this just runs blr and this func is called by several other functions other then the native's table.  
```



pub fn unregister_script_with_audio_safe(
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xA8638BE228D4751Au32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xA8638BE228D4751Au64;
        
        let result = invoke_raw!(
            hash,
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn unregister_script_with_audio_raw(
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xA8638BE228D4751Au32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xA8638BE228D4751Au64;

        invoke_raw_typed!(
            hash,
        )
    }
}

/// ## Parameters
*



pub fn _0x43fa0dfc5df87815_safe(
        
        
            vehicle: 
        , 
        
        
            p1: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x43FA0DFC5DF87815u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x43FA0DFC5DF87815u64;
        
        let result = invoke_raw!(
            hash,
                vehicle, 
                p1
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn _0x43fa0dfc5df87815_raw(
        vehicle: , 
        p1: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x43FA0DFC5DF87815u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x43FA0DFC5DF87815u64;

        invoke_raw_typed!(
            hash,
                vehicle, 
                p1
        )
    }
}

/// ```
Dat151RelType == 29
```

```
NativeDB Introduced: v2699
```



pub fn _set_ped_voice_group_race_safe(
        
        
            ped: 
        , 
        
        
            voiceGroupHash: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x0BABC1345ABBFB16u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x0BABC1345ABBFB16u64;
        
        let result = invoke_raw!(
            hash,
                ped, 
                voiceGroupHash
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn _set_ped_voice_group_race_raw(
        ped: , 
        voiceGroupHash: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x0BABC1345ABBFB16u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x0BABC1345ABBFB16u64;

        invoke_raw_typed!(
            hash,
                ped, 
                voiceGroupHash
        )
    }
}

/// ## Return value
Returns true if the script is currently playing a stream.



pub fn is_stream_playing_safe(
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xD11FA52EB849D978u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xD11FA52EB849D978u64;
        
        let result = invoke_raw!(
            hash,
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn is_stream_playing_raw(
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xD11FA52EB849D978u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xD11FA52EB849D978u64;

        invoke_raw_typed!(
            hash,
        )
    }
}

/// ```
 Found in the b617d scripts, duplicates removed:
 AUDIO::_B4BBFD9CD8B3922B("V_CARSHOWROOM_PS_WINDOW_UNBROKEN");
 AUDIO::_B4BBFD9CD8B3922B("V_CIA_PS_WINDOW_UNBROKEN");
 AUDIO::_B4BBFD9CD8B3922B("V_DLC_HEIST_APARTMENT_DOOR_CLOSED");
 AUDIO::_B4BBFD9CD8B3922B("V_FINALEBANK_PS_VAULT_INTACT");
 AUDIO::_B4BBFD9CD8B3922B("V_MICHAEL_PS_BATHROOM_WITH_WINDOW");
```

For events like cars driving through windows, allows script to unocclude that window



pub fn remove_portal_settings_override_safe(
        
        
            portalSettingsName: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xB4BBFD9CD8B3922Bu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xB4BBFD9CD8B3922Bu64;
        
        let result = invoke_raw!(
            hash,
                portalSettingsName
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn remove_portal_settings_override_raw(
        portalSettingsName: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xB4BBFD9CD8B3922Bu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xB4BBFD9CD8B3922Bu64;

        invoke_raw_typed!(
            hash,
                portalSettingsName
        )
    }
}

/// ```
All found occurrences in b617d, sorted alphabetically and identical lines removed: pastebin.com/f2A7vTj0   
No changes made in b678d.  
gtaforums.com/topic/795622-audio-for-mods  
```



pub fn play_sound_from_entity_safe(
        
        
            soundId: 
        , 
        
        
            audioName: 
        , 
        
        
            entity: 
        , 
        
        
            audioRef: 
        , 
        
        
            isNetwork: 
        , 
        
        
            p5: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xE65F427EB70AB1EDu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xE65F427EB70AB1EDu64;
        
        let result = invoke_raw!(
            hash,
                soundId, 
                audioName, 
                entity, 
                audioRef, 
                isNetwork, 
                p5
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn play_sound_from_entity_raw(
        soundId: , 
        audioName: , 
        entity: , 
        audioRef: , 
        isNetwork: , 
        p5: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xE65F427EB70AB1EDu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xE65F427EB70AB1EDu64;

        invoke_raw_typed!(
            hash,
                soundId, 
                audioName, 
                entity, 
                audioRef, 
                isNetwork, 
                p5
        )
    }
}

/// Deactivates the named slowmo mode.



pub fn deactivate_audio_slowmo_mode_safe(
        
        
            mode: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xDDC635D5B3262C56u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xDDC635D5B3262C56u64;
        
        let result = invoke_raw!(
            hash,
                mode
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn deactivate_audio_slowmo_mode_raw(
        mode: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xDDC635D5B3262C56u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xDDC635D5B3262C56u64;

        invoke_raw_typed!(
            hash,
                mode
        )
    }
}

/// ## Parameters
*



pub fn play_ambient_speech_from_position_native_safe(
        
        
            speechName: 
        , 
        
        
            voiceName: 
        , 
        
        
            x: 
        , 
        
        
            y: 
        , 
        
        
            z: 
        , 
        
        
            speechParam: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xED640017ED337E45u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xED640017ED337E45u64;
        
        let result = invoke_raw!(
            hash,
                speechName, 
                voiceName, 
                x, 
                y, 
                z, 
                speechParam
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn play_ambient_speech_from_position_native_raw(
        speechName: , 
        voiceName: , 
        x: , 
        y: , 
        z: , 
        speechParam: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xED640017ED337E45u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xED640017ED337E45u64;

        invoke_raw_typed!(
            hash,
                speechName, 
                voiceName, 
                x, 
                y, 
                z, 
                speechParam
        )
    }
}

/// ## Parameters
*



pub fn freeze_radio_station_safe(
        
        
            radioStation: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x344F393B027E38C3u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x344F393B027E38C3u64;
        
        let result = invoke_raw!(
            hash,
                radioStation
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn freeze_radio_station_raw(
        radioStation: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x344F393B027E38C3u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x344F393B027E38C3u64;

        invoke_raw_typed!(
            hash,
                radioStation
        )
    }
}

/// Stops currently playing speech (Pain, ambient, scripted, breathing).



pub fn stop_current_playing_speech_safe(
        
        
            ped: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x7A73D05A607734C7u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x7A73D05A607734C7u64;
        
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
pub fn stop_current_playing_speech_raw(
        ped: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x7A73D05A607734C7u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x7A73D05A607734C7u64;

        invoke_raw_typed!(
            hash,
                ped
        )
    }
}

/// Overrides wind elevation sounds



pub fn script_overrides_wind_elevation_safe(
        
        
            override: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x70B8EC8FC108A634u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x70B8EC8FC108A634u64;
        
        let result = invoke_raw!(
            hash,
                override
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn script_overrides_wind_elevation_raw(
        override: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x70B8EC8FC108A634u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x70B8EC8FC108A634u64;

        invoke_raw_typed!(
            hash,
                override
        )
    }
}

/// PLAY_STREAM_FRONTEND native function



pub fn play_stream_frontend_safe(
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x58FCE43488F9F5F4u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x58FCE43488F9F5F4u64;
        
        let result = invoke_raw!(
            hash,
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn play_stream_frontend_raw(
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x58FCE43488F9F5F4u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x58FCE43488F9F5F4u64;

        invoke_raw_typed!(
            hash,
        )
    }
}

/// Creates a broken glass area



pub fn record_broken_glass_safe(
        
        
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
        let hash = 0xFBE20329593DEC9Du32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xFBE20329593DEC9Du64;
        
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
pub fn record_broken_glass_raw(
        x: , 
        y: , 
        z: , 
        radius: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xFBE20329593DEC9Du32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xFBE20329593DEC9Du64;

        invoke_raw_typed!(
            hash,
                x, 
                y, 
                z, 
                radius
        )
    }
}

/// CLEAR_ALL_BROKEN_GLASS native function



pub fn clear_all_broken_glass_safe(
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xB32209EFFDC04913u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xB32209EFFDC04913u64;
        
        let result = invoke_raw!(
            hash,
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn clear_all_broken_glass_raw(
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xB32209EFFDC04913u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xB32209EFFDC04913u64;

        invoke_raw_typed!(
            hash,
        )
    }
}

/// ```
Plays the given police radio message.
All found occurrences in b617d, sorted alphabetically and identical lines removed: pastebin.com/GBnsQ5hr
```



pub fn play_police_report_safe(
        
        
            name: 
        , 
        
        
            p1: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xDFEBD56D9BD1EB16u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xDFEBD56D9BD1EB16u64;
        
        let result = invoke_raw!(
            hash,
                name, 
                p1
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn play_police_report_raw(
        name: , 
        p1: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xDFEBD56D9BD1EB16u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xDFEBD56D9BD1EB16u64;

        invoke_raw_typed!(
            hash,
                name, 
                p1
        )
    }
}

/// Sets radio to tune down. (Changes radio station)



pub fn set_radio_retune_down_safe(
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xDD6BCF9E94425DF9u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xDD6BCF9E94425DF9u64;
        
        let result = invoke_raw!(
            hash,
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn set_radio_retune_down_raw(
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xDD6BCF9E94425DF9u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xDD6BCF9E94425DF9u64;

        invoke_raw_typed!(
            hash,
        )
    }
}

/// Sets whether or not scripted conversation flow should be controlled by anim triggers



pub fn set_conversation_audio_controlled_by_anim_safe(
        
        
            enable: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x0B568201DD99F0EBu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x0B568201DD99F0EBu64;
        
        let result = invoke_raw!(
            hash,
                enable
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn set_conversation_audio_controlled_by_anim_raw(
        enable: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x0B568201DD99F0EBu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x0B568201DD99F0EBu64;

        invoke_raw_typed!(
            hash,
                enable
        )
    }
}

/// ## Return value
Returns the name of the players radio station. This serves as the text label.



pub fn get_player_radio_station_name_safe(
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xF6D733C32076AD03u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xF6D733C32076AD03u64;
        
        let result = invoke_raw!(
            hash,
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn get_player_radio_station_name_raw(
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xF6D733C32076AD03u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xF6D733C32076AD03u64;

        invoke_raw_typed!(
            hash,
        )
    }
}

/// ## Return value
Returns true if a one shot is currently playing or prepared



pub fn is_music_oneshot_playing_safe(
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xA097AB275061FB21u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xA097AB275061FB21u64;
        
        let result = invoke_raw!(
            hash,
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn is_music_oneshot_playing_raw(
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xA097AB275061FB21u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xA097AB275061FB21u64;

        invoke_raw_typed!(
            hash,
        )
    }
}

/// ## Parameters
*



pub fn set_user_radio_control_enabled_safe(
        
        
            toggle: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x19F21E63AE6EAE4Eu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x19F21E63AE6EAE4Eu64;
        
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
pub fn set_user_radio_control_enabled_raw(
        toggle: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x19F21E63AE6EAE4Eu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x19F21E63AE6EAE4Eu64;

        invoke_raw_typed!(
            hash,
                toggle
        )
    }
}

/// ## Parameters
*



pub fn is_animal_vocalization_playing_safe(
        
        
            pedHandle: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xC265DF9FB44A9FBDu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xC265DF9FB44A9FBDu64;
        
        let result = invoke_raw!(
            hash,
                pedHandle
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn is_animal_vocalization_playing_raw(
        pedHandle: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xC265DF9FB44A9FBDu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xC265DF9FB44A9FBDu64;

        invoke_raw_typed!(
            hash,
                pedHandle
        )
    }
}

/// ## Parameters
*



pub fn is_scripted_speech_playing_safe(
        
        
            ped: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xCC9AA18DCC7084F4u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xCC9AA18DCC7084F4u64;
        
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
pub fn is_scripted_speech_playing_raw(
        ped: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xCC9AA18DCC7084F4u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xCC9AA18DCC7084F4u64;

        invoke_raw_typed!(
            hash,
                ped
        )
    }
}

/// ## Parameters
*



pub fn get_vehicle_default_horn_ignore_mods_safe(
        
        
            vehicle: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xACB5DCCA1EC76840u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xACB5DCCA1EC76840u64;
        
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
pub fn get_vehicle_default_horn_ignore_mods_raw(
        vehicle: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xACB5DCCA1EC76840u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xACB5DCCA1EC76840u64;

        invoke_raw_typed!(
            hash,
                vehicle
        )
    }
}

/// ## Parameters
*



pub fn init_synch_scene_audio_with_position_safe(
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xC8EDE9BDBCCBA6D4u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xC8EDE9BDBCCBA6D4u64;
        
        let result = invoke_raw!(
            hash,
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn init_synch_scene_audio_with_position_raw(
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xC8EDE9BDBCCBA6D4u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xC8EDE9BDBCCBA6D4u64;

        invoke_raw_typed!(
            hash,
        )
    }
}

/// SKIP_RADIO_FORWARD native function



pub fn skip_radio_forward_safe(
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x6DDBBDD98E2E9C25u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x6DDBBDD98E2E9C25u64;
        
        let result = invoke_raw!(
            hash,
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn skip_radio_forward_raw(
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x6DDBBDD98E2E9C25u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x6DDBBDD98E2E9C25u64;

        invoke_raw_typed!(
            hash,
        )
    }
}

/// This doesn't stop a piece of dialogue that has been triggered.

This stops the ability to force ambient dialogue if set to true - however setting it to false, then triggering a context, then setting it to true again will allow this.

The ped will also be prevented from speaking on remote machines. Use [STOP_PED_SPEAKING](#_0x9D64D7405520E3D3) if you just want to affect the local machine.



pub fn stop_ped_speaking_synced_safe(
        
        
            ped: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xAB6781A5F3101470u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xAB6781A5F3101470u64;
        
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
pub fn stop_ped_speaking_synced_raw(
        ped: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xAB6781A5F3101470u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xAB6781A5F3101470u64;

        invoke_raw_typed!(
            hash,
                ped
        )
    }
}

/// Enable or disable exhaust pops on the given vehicle.



pub fn enable_vehicle_exhaust_pops_safe(
        
        
            vehicle: 
        , 
        
        
            toggle: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x2BE4BC731D039D5Au32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x2BE4BC731D039D5Au64;
        
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
pub fn enable_vehicle_exhaust_pops_raw(
        vehicle: , 
        toggle: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x2BE4BC731D039D5Au32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x2BE4BC731D039D5Au64;

        invoke_raw_typed!(
            hash,
                vehicle, 
                toggle
        )
    }
}

/// Links a static emitter to the given entity



pub fn link_static_emitter_to_entity_safe(
        
        
            emitterName: 
        , 
        
        
            entity: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x651D3228960D08AFu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x651D3228960D08AFu64;
        
        let result = invoke_raw!(
            hash,
                emitterName, 
                entity
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn link_static_emitter_to_entity_raw(
        emitterName: , 
        entity: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x651D3228960D08AFu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x651D3228960D08AFu64;

        invoke_raw_typed!(
            hash,
                emitterName, 
                entity
        )
    }
}

/// Sets audio flag "OverrideMicrophoneSettings"

Allows the script to ovverride the current microphone settings



pub fn override_microphone_settings_safe(
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x75773E11BA459E90u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x75773E11BA459E90u64;
        
        let result = invoke_raw!(
            hash,
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn override_microphone_settings_raw(
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x75773E11BA459E90u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x75773E11BA459E90u64;

        invoke_raw_typed!(
            hash,
        )
    }
}

/// Overrides the vehicle's startup engine rev sound.

You can reset this with [RESET_VEHICLE_STARTUP_REV_SOUND](#_0xD2DCCD8E16E20997)



pub fn set_vehicle_startup_rev_sound_safe(
        
        
            vehicle: 
        , 
        
        
            soundName: 
        , 
        
        
            setName: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xF1F8157B8C3F171Cu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xF1F8157B8C3F171Cu64;
        
        let result = invoke_raw!(
            hash,
                vehicle, 
                soundName, 
                setName
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn set_vehicle_startup_rev_sound_raw(
        vehicle: , 
        soundName: , 
        setName: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xF1F8157B8C3F171Cu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xF1F8157B8C3F171Cu64;

        invoke_raw_typed!(
            hash,
                vehicle, 
                soundName, 
                setName
        )
    }
}

/// ## Parameters
*



pub fn set_script_update_door_audio_safe(
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x06C0023BED16DD6Bu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x06C0023BED16DD6Bu64;
        
        let result = invoke_raw!(
            hash,
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn set_script_update_door_audio_raw(
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x06C0023BED16DD6Bu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x06C0023BED16DD6Bu64;

        invoke_raw_typed!(
            hash,
        )
    }
}

/// ## Return value



pub fn does_player_veh_have_radio_safe(
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x109697E2FFBAC8A1u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x109697E2FFBAC8A1u64;
        
        let result = invoke_raw!(
            hash,
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn does_player_veh_have_radio_raw(
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x109697E2FFBAC8A1u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x109697E2FFBAC8A1u64;

        invoke_raw_typed!(
            hash,
        )
    }
}

/// ```
Used to prepare a scene where the surrounding sound is muted or a bit changed. This does not play any sound.  
List of all usable scene names found in b617d. Sorted alphabetically and identical names removed: pastebin.com/MtM9N9CC  
```



pub fn start_audio_scene_safe(
        
        
            scene: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x013A80FC08F6E4F2u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x013A80FC08F6E4F2u64;
        
        let result = invoke_raw!(
            hash,
                scene
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn start_audio_scene_raw(
        scene: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x013A80FC08F6E4F2u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x013A80FC08F6E4F2u64;

        invoke_raw_typed!(
            hash,
                scene
        )
    }
}

/// Unloads all audio banks requested by this script.



pub fn release_script_audio_bank_safe(
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x7A2D8AD0A9EB9C3Fu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x7A2D8AD0A9EB9C3Fu64;
        
        let result = invoke_raw!(
            hash,
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn release_script_audio_bank_raw(
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x7A2D8AD0A9EB9C3Fu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x7A2D8AD0A9EB9C3Fu64;

        invoke_raw_typed!(
            hash,
        )
    }
}

/// ## Parameters
*



pub fn set_portal_settings_override_safe(
        
        
            oldPortalSettingsName: 
        , 
        
        
            newPortalSettingsName: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x044DBAD7A7FA2BE5u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x044DBAD7A7FA2BE5u64;
        
        let result = invoke_raw!(
            hash,
                oldPortalSettingsName, 
                newPortalSettingsName
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn set_portal_settings_override_raw(
        oldPortalSettingsName: , 
        newPortalSettingsName: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x044DBAD7A7FA2BE5u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x044DBAD7A7FA2BE5u64;

        invoke_raw_typed!(
            hash,
                oldPortalSettingsName, 
                newPortalSettingsName
        )
    }
}

/// ## Return value
Returns the index of the current players radio station, or 255 if the radio is off



pub fn get_player_radio_station_index_safe(
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xE8AF77C4C06ADC93u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xE8AF77C4C06ADC93u64;
        
        let result = invoke_raw!(
            hash,
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn get_player_radio_station_index_raw(
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xE8AF77C4C06ADC93u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xE8AF77C4C06ADC93u64;

        invoke_raw_typed!(
            hash,
        )
    }
}

/// ## Parameters
*



pub fn prepare_synchronized_audio_event_safe(
        
        
            audioEvent: 
        , 
        
        
            startOffsetMs: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xC7ABCACA4985A766u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xC7ABCACA4985A766u64;
        
        let result = invoke_raw!(
            hash,
                audioEvent, 
                startOffsetMs
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn prepare_synchronized_audio_event_raw(
        audioEvent: , 
        startOffsetMs: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xC7ABCACA4985A766u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xC7ABCACA4985A766u64;

        invoke_raw_typed!(
            hash,
                audioEvent, 
                startOffsetMs
        )
    }
}

/// ## Parameters
*



pub fn preload_script_phone_conversation_safe(
        
        
            displaySubtitles: 
        , 
        
        
            addToBriefScreen: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x6004BCB0E226AAEAu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x6004BCB0E226AAEAu64;
        
        let result = invoke_raw!(
            hash,
                displaySubtitles, 
                addToBriefScreen
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn preload_script_phone_conversation_raw(
        displaySubtitles: , 
        addToBriefScreen: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x6004BCB0E226AAEAu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x6004BCB0E226AAEAu64;

        invoke_raw_typed!(
            hash,
                displaySubtitles, 
                addToBriefScreen
        )
    }
}

/// ```
NativeDB Introduced: v1365
```



pub fn _set_vehicle_horn_variation_safe(
        
        
            vehicle: 
        , 
        
        
            value: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x0350E7E17BA767D0u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x0350E7E17BA767D0u64;
        
        let result = invoke_raw!(
            hash,
                vehicle, 
                value
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn _set_vehicle_horn_variation_raw(
        vehicle: , 
        value: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x0350E7E17BA767D0u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x0350E7E17BA767D0u64;

        invoke_raw_typed!(
            hash,
                vehicle, 
                value
        )
    }
}

/// ## Return value
Returns true if the script should draw the mission complete UI to be in sync with audio.



pub fn is_mission_complete_ready_for_ui_safe(
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x6F259F82D873B8B8u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x6F259F82D873B8B8u64;
        
        let result = invoke_raw!(
            hash,
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn is_mission_complete_ready_for_ui_raw(
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x6F259F82D873B8B8u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x6F259F82D873B8B8u64;

        invoke_raw_typed!(
            hash,
        )
    }
}

/// ## Parameters
*



pub fn play_end_credits_music_safe(
        
        
            bActive: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xCD536C4D33DCC900u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xCD536C4D33DCC900u64;
        
        let result = invoke_raw!(
            hash,
                bActive
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn play_end_credits_music_raw(
        bActive: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xCD536C4D33DCC900u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xCD536C4D33DCC900u64;

        invoke_raw_typed!(
            hash,
                bActive
        )
    }
}

/// ## Parameters
*



pub fn is_vehicle_radio_on_safe(
        
        
            vehicle: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x0BE4BE946463F917u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x0BE4BE946463F917u64;
        
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
pub fn is_vehicle_radio_on_raw(
        vehicle: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x0BE4BE946463F917u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x0BE4BE946463F917u64;

        invoke_raw_typed!(
            hash,
                vehicle
        )
    }
}

/// Sets the global radio signal level, lower value will cause radio static. 
Used only a handful of times in scripts.



pub fn set_global_radio_signal_level_safe(
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x159B7318403A1CD8u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x159B7318403A1CD8u64;
        
        let result = invoke_raw!(
            hash,
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn set_global_radio_signal_level_raw(
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x159B7318403A1CD8u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x159B7318403A1CD8u64;

        invoke_raw_typed!(
            hash,
        )
    }
}

/// Refreshes the closest shoreline using the nearest road position.



pub fn refresh_closest_ocean_shoreline_safe(
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x5D2BFAAB8D956E0Eu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x5D2BFAAB8D956E0Eu64;
        
        let result = invoke_raw!(
            hash,
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn refresh_closest_ocean_shoreline_raw(
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x5D2BFAAB8D956E0Eu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x5D2BFAAB8D956E0Eu64;

        invoke_raw_typed!(
            hash,
        )
    }
}

/// ## Parameters
*



pub fn has_sound_finished_safe(
        
        
            soundId: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xFCBDCE714A7C88E5u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xFCBDCE714A7C88E5u64;
        
        let result = invoke_raw!(
            hash,
                soundId
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn has_sound_finished_raw(
        soundId: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xFCBDCE714A7C88E5u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xFCBDCE714A7C88E5u64;

        invoke_raw_typed!(
            hash,
                soundId
        )
    }
}

/// ```
Enables/disables ped's "loud" footstep sound.
```

```
NativeDB Introduced: v1493
```



pub fn _set_ped_audio_footstep_loud_safe(
        
        
            ped: 
        , 
        
        
            toggle: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x0653B735BFBDFE87u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x0653B735BFBDFE87u64;
        
        let result = invoke_raw!(
            hash,
                ped, 
                toggle
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn _set_ped_audio_footstep_loud_raw(
        ped: , 
        toggle: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x0653B735BFBDFE87u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x0653B735BFBDFE87u64;

        invoke_raw_typed!(
            hash,
                ped, 
                toggle
        )
    }
}

/// ## Parameters
*



pub fn is_ambient_speech_playing_safe(
        
        
            ped: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x9072C8B49907BFADu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x9072C8B49907BFADu64;
        
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
pub fn is_ambient_speech_playing_raw(
        ped: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x9072C8B49907BFADu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x9072C8B49907BFADu64;

        invoke_raw_typed!(
            hash,
                ped
        )
    }
}

/// ```
All found occurrences in b678d:
pastebin.com/ceu67jz8
```



pub fn add_entity_to_audio_mix_group_safe(
        
        
            entity: 
        , 
        
        
            groupName: 
        , 
        
        
            fadeIn: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x153973AB99FE8980u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x153973AB99FE8980u64;
        
        let result = invoke_raw!(
            hash,
                entity, 
                groupName, 
                fadeIn
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn add_entity_to_audio_mix_group_raw(
        entity: , 
        groupName: , 
        fadeIn: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x153973AB99FE8980u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x153973AB99FE8980u64;

        invoke_raw_typed!(
            hash,
                entity, 
                groupName, 
                fadeIn
        )
    }
}

/// ## Parameters
*



pub fn add_ped_to_conversation_safe(
        
        
            speakerConversationIndex: 
        , 
        
        
            ped: 
        , 
        
        
            voiceName: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x95D9F4BC443956E7u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x95D9F4BC443956E7u64;
        
        let result = invoke_raw!(
            hash,
                speakerConversationIndex, 
                ped, 
                voiceName
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn add_ped_to_conversation_raw(
        speakerConversationIndex: , 
        ped: , 
        voiceName: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x95D9F4BC443956E7u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x95D9F4BC443956E7u64;

        invoke_raw_typed!(
            hash,
                speakerConversationIndex, 
                ped, 
                voiceName
        )
    }
}

/// Plays ambient speech; see also [`PLAY_PED_AMBIENT_SPEECH_AND_CLONE_NATIVE`](#_0xC6941B4A3A8FBBB9).

```
speechParam: Can be one of the following:
SPEECH_PARAMS_STANDARD
SPEECH_PARAMS_ALLOW_REPEAT
SPEECH_PARAMS_BEAT
SPEECH_PARAMS_FORCE
SPEECH_PARAMS_FORCE_FRONTEND
SPEECH_PARAMS_FORCE_NO_REPEAT_FRONTEND
SPEECH_PARAMS_FORCE_NORMAL
SPEECH_PARAMS_FORCE_NORMAL_CLEAR
SPEECH_PARAMS_FORCE_NORMAL_CRITICAL
SPEECH_PARAMS_FORCE_SHOUTED
SPEECH_PARAMS_FORCE_SHOUTED_CLEAR
SPEECH_PARAMS_FORCE_SHOUTED_CRITICAL
SPEECH_PARAMS_FORCE_PRELOAD_ONLY
SPEECH_PARAMS_MEGAPHONE
SPEECH_PARAMS_HELI
SPEECH_PARAMS_FORCE_MEGAPHONE
SPEECH_PARAMS_FORCE_HELI
SPEECH_PARAMS_INTERRUPT
SPEECH_PARAMS_INTERRUPT_SHOUTED
SPEECH_PARAMS_INTERRUPT_SHOUTED_CLEAR
SPEECH_PARAMS_INTERRUPT_SHOUTED_CRITICAL
SPEECH_PARAMS_INTERRUPT_NO_FORCE
SPEECH_PARAMS_INTERRUPT_FRONTEND
SPEECH_PARAMS_INTERRUPT_NO_FORCE_FRONTEND
SPEECH_PARAMS_ADD_BLIP
SPEECH_PARAMS_ADD_BLIP_ALLOW_REPEAT
SPEECH_PARAMS_ADD_BLIP_FORCE
SPEECH_PARAMS_ADD_BLIP_SHOUTED
SPEECH_PARAMS_ADD_BLIP_SHOUTED_FORCE
SPEECH_PARAMS_ADD_BLIP_INTERRUPT
SPEECH_PARAMS_ADD_BLIP_INTERRUPT_FORCE
SPEECH_PARAMS_FORCE_PRELOAD_ONLY_SHOUTED
SPEECH_PARAMS_FORCE_PRELOAD_ONLY_SHOUTED_CLEAR
SPEECH_PARAMS_FORCE_PRELOAD_ONLY_SHOUTED_CRITICAL
SPEECH_PARAMS_SHOUTED
SPEECH_PARAMS_SHOUTED_CLEAR
SPEECH_PARAMS_SHOUTED_CRITICAL
Note: A list of Name and Parameters can be found here pastebin.com/1GZS5dCL
```

```
NativeDB Added Parameter 4: Any p3
```



pub fn play_ped_ambient_speech_native_safe(
        
        
            ped: 
        , 
        
        
            speechName: 
        , 
        
        
            speechParam: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x8E04FEDD28D42462u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x8E04FEDD28D42462u64;
        
        let result = invoke_raw!(
            hash,
                ped, 
                speechName, 
                speechParam
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn play_ped_ambient_speech_native_raw(
        ped: , 
        speechName: , 
        speechParam: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x8E04FEDD28D42462u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x8E04FEDD28D42462u64;

        invoke_raw_typed!(
            hash,
                ped, 
                speechName, 
                speechParam
        )
    }
}

/// ## Return value
Returns true of mobile phone interference is currently happening



pub fn is_mobile_interference_active_safe(
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xC8B1B2425604CDD0u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xC8B1B2425604CDD0u64;
        
        let result = invoke_raw!(
            hash,
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn is_mobile_interference_active_raw(
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xC8B1B2425604CDD0u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xC8B1B2425604CDD0u64;

        invoke_raw_typed!(
            hash,
        )
    }
}

/// ## Parameters
*



pub fn set_entity_for_null_conv_ped_safe(
        
        
            speakerConversationIndex: 
        , 
        
        
            entity: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x892B6AB8F33606F5u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x892B6AB8F33606F5u64;
        
        let result = invoke_raw!(
            hash,
                speakerConversationIndex, 
                entity
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn set_entity_for_null_conv_ped_raw(
        speakerConversationIndex: , 
        entity: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x892B6AB8F33606F5u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x892B6AB8F33606F5u64;

        invoke_raw_typed!(
            hash,
                speakerConversationIndex, 
                entity
        )
    }
}

/// ## Parameters
*



pub fn play_vehicle_door_close_sound_safe(
        
        
            vehicle: 
        , 
        
        
            doorIndex: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x62A456AA4769EF34u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x62A456AA4769EF34u64;
        
        let result = invoke_raw!(
            hash,
                vehicle, 
                doorIndex
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn play_vehicle_door_close_sound_raw(
        vehicle: , 
        doorIndex: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x62A456AA4769EF34u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x62A456AA4769EF34u64;

        invoke_raw_typed!(
            hash,
                vehicle, 
                doorIndex
        )
    }
}

/// ```
can't seem to enable radio on cop cars etc  
```



pub fn set_vehicle_radio_enabled_safe(
        
        
            vehicle: 
        , 
        
        
            toggle: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x3B988190C0AA6C0Bu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x3B988190C0AA6C0Bu64;
        
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
pub fn set_vehicle_radio_enabled_raw(
        vehicle: , 
        toggle: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x3B988190C0AA6C0Bu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x3B988190C0AA6C0Bu64;

        invoke_raw_typed!(
            hash,
                vehicle, 
                toggle
        )
    }
}

/// ```
NativeDB Introduced: v323
```



pub fn release_weapon_audio_safe(
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xCE4AC0439F607045u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xCE4AC0439F607045u64;
        
        let result = invoke_raw!(
            hash,
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn release_weapon_audio_raw(
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xCE4AC0439F607045u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xCE4AC0439F607045u64;

        invoke_raw_typed!(
            hash,
        )
    }
}

/// Enable player vehicle specific alarm disarm/arm sound triggering



pub fn set_player_vehicle_alarm_audio_active_safe(
        
        
            vehicle: 
        , 
        
        
            active: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x6FDDAD856E36988Au32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x6FDDAD856E36988Au64;
        
        let result = invoke_raw!(
            hash,
                vehicle, 
                active
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn set_player_vehicle_alarm_audio_active_raw(
        vehicle: , 
        active: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x6FDDAD856E36988Au32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x6FDDAD856E36988Au64;

        invoke_raw_typed!(
            hash,
                vehicle, 
                active
        )
    }
}

/// Forces the ambient peds into their panic walla state



pub fn force_ped_panic_walla_safe(
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x062D5EAD4DA2FA6Au32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x062D5EAD4DA2FA6Au64;
        
        let result = invoke_raw!(
            hash,
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn force_ped_panic_walla_raw(
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x062D5EAD4DA2FA6Au32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x062D5EAD4DA2FA6Au64;

        invoke_raw_typed!(
            hash,
        )
    }
}

/// ```c
enum eAudContextBlockTarget {
	AUD_CONTEXT_BLOCK_PLAYER = 0,
	AUD_CONTEXT_BLOCK_NPCS = 1,
	AUD_CONTEXT_BLOCK_BUDDYS = 2,
	AUD_CONTEXT_BLOCK_EVERYONE = 3,

	AUD_CONTEXT_BLOCK_TARGETS_TOTAL
}
```

Stop a certain group of peds from using a certain group of speech contexts.

Note that the block will be automatically removed when the calling script finishes



pub fn block_speech_context_group_safe(
        
        
            groupName: 
        , 
        
        
            contextBlockTarget: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xA8A7D434AFB4B97Bu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xA8A7D434AFB4B97Bu64;
        
        let result = invoke_raw!(
            hash,
                groupName, 
                contextBlockTarget
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn block_speech_context_group_raw(
        groupName: , 
        contextBlockTarget: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xA8A7D434AFB4B97Bu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xA8A7D434AFB4B97Bu64;

        invoke_raw_typed!(
            hash,
                groupName, 
                contextBlockTarget
        )
    }
}

/// Set a delay in milliseconds for the audio to be cleaned up when the script finishes.



pub fn set_audio_script_cleanup_time_safe(
        
        
            timeMs: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xA5F377B175A699C5u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xA5F377B175A699C5u64;
        
        let result = invoke_raw!(
            hash,
                timeMs
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn set_audio_script_cleanup_time_raw(
        timeMs: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xA5F377B175A699C5u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xA5F377B175A699C5u64;

        invoke_raw_typed!(
            hash,
                timeMs
        )
    }
}

/// ## Return value
Returns the currently playing stream's play time



pub fn get_stream_play_time_safe(
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x4E72BBDBCA58A3DBu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x4E72BBDBCA58A3DBu64;
        
        let result = invoke_raw!(
            hash,
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn get_stream_play_time_raw(
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x4E72BBDBCA58A3DBu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x4E72BBDBCA58A3DBu64;

        invoke_raw_typed!(
            hash,
        )
    }
}

/// ## Parameters
*



pub fn set_ambient_zone_state_safe(
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xBDA07E5950085E46u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xBDA07E5950085E46u64;
        
        let result = invoke_raw!(
            hash,
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn set_ambient_zone_state_raw(
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xBDA07E5950085E46u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xBDA07E5950085E46u64;

        invoke_raw_typed!(
            hash,
        )
    }
}

/// ## Parameters
*



pub fn block_death_jingle_safe(
        
        
            blocked: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xF154B8D1775B2DECu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xF154B8D1775B2DECu64;
        
        let result = invoke_raw!(
            hash,
                blocked
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn block_death_jingle_raw(
        blocked: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xF154B8D1775B2DECu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xF154B8D1775B2DECu64;

        invoke_raw_typed!(
            hash,
                blocked
        )
    }
}

/// ```
Hardcoded to return 1  
```



pub fn is_game_in_control_of_music_safe(
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x6D28DC1671E334FDu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x6D28DC1671E334FDu64;
        
        let result = invoke_raw!(
            hash,
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn is_game_in_control_of_music_raw(
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x6D28DC1671E334FDu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x6D28DC1671E334FDu64;

        invoke_raw_typed!(
            hash,
        )
    }
}

/// CANCEL_ALL_POLICE_REPORTS native function



pub fn cancel_all_police_reports_safe(
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xB4F90FAF7670B16Fu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xB4F90FAF7670B16Fu64;
        
        let result = invoke_raw!(
            hash,
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn cancel_all_police_reports_raw(
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xB4F90FAF7670B16Fu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xB4F90FAF7670B16Fu64;

        invoke_raw_typed!(
            hash,
        )
    }
}

/// Handles conversation interrupts, using the code-side system for improved timing and to minimize unfriendly logic interactions.



pub fn interrupt_conversation_safe(
        
        
            interrupterPed: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xA018A12E5C5C2FA6u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xA018A12E5C5C2FA6u64;
        
        let result = invoke_raw!(
            hash,
                interrupterPed
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn interrupt_conversation_raw(
        interrupterPed: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xA018A12E5C5C2FA6u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xA018A12E5C5C2FA6u64;

        invoke_raw_typed!(
            hash,
                interrupterPed
        )
    }
}

/// Stops audio for the current cutscene.



pub fn stop_cutscene_audio_safe(
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x806058BBDC136E06u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x806058BBDC136E06u64;
        
        let result = invoke_raw!(
            hash,
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn stop_cutscene_audio_raw(
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x806058BBDC136E06u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x806058BBDC136E06u64;

        invoke_raw_typed!(
            hash,
        )
    }
}

/// CREATE_NEW_SCRIPTED_CONVERSATION native function



pub fn create_new_scripted_conversation_safe(
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xD2C91A0B572AAE56u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xD2C91A0B572AAE56u64;
        
        let result = invoke_raw!(
            hash,
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn create_new_scripted_conversation_raw(
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xD2C91A0B572AAE56u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xD2C91A0B572AAE56u64;

        invoke_raw_typed!(
            hash,
        )
    }
}

/// ## Parameters
*



pub fn set_radio_frontend_fade_time_safe(
        
        
            fadeTime: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x2C96CDB04FCA358Eu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x2C96CDB04FCA358Eu64;
        
        let result = invoke_raw!(
            hash,
                fadeTime
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn set_radio_frontend_fade_time_raw(
        fadeTime: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x2C96CDB04FCA358Eu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x2C96CDB04FCA358Eu64;

        invoke_raw_typed!(
            hash,
                fadeTime
        )
    }
}

/// This doesn't stop a piece of dialogue that has been triggered.

This stops the ability to force ambient dialogue if set to true - however setting it to false, then triggering a context, then setting it to true again will allow this.

Nb. This does not sync over the network, it will only affect peds locally. Use [STOP_PED_SPEAKING_SYNCED](#_0xAB6781A5F3101470) if you need to affect peds on other machines too.



pub fn stop_ped_speaking_safe(
        
        
            ped: 
        , 
        
        
            shouldDisable: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x9D64D7405520E3D3u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x9D64D7405520E3D3u64;
        
        let result = invoke_raw!(
            hash,
                ped, 
                shouldDisable
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn stop_ped_speaking_raw(
        ped: , 
        shouldDisable: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x9D64D7405520E3D3u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x9D64D7405520E3D3u64;

        invoke_raw_typed!(
            hash,
                ped, 
                shouldDisable
        )
    }
}

/// ```
If this is the correct name, what microphone? I know your TV isn't going to reach out and adjust your headset so..  
```



pub fn set_microphone_position_safe(
        
        
            p0: 
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
        
        
            x3: 
        , 
        
        
            y3: 
        , 
        
        
            z3: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xB6AE90EDDE95C762u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xB6AE90EDDE95C762u64;
        
        let result = invoke_raw!(
            hash,
                p0, 
                x1, 
                y1, 
                z1, 
                x2, 
                y2, 
                z2, 
                x3, 
                y3, 
                z3
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn set_microphone_position_raw(
        p0: , 
        x1: , 
        y1: , 
        z1: , 
        x2: , 
        y2: , 
        z2: , 
        x3: , 
        y3: , 
        z3: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xB6AE90EDDE95C762u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xB6AE90EDDE95C762u64;

        invoke_raw_typed!(
            hash,
                p0, 
                x1, 
                y1, 
                z1, 
                x2, 
                y2, 
                z2, 
                x3, 
                y3, 
                z3
        )
    }
}

/// Queues up a custom track list on the specified radio station. The content in the track list will be played as soon as possible.
The station does not have to be frozen.



pub fn set_custom_radio_track_list_safe(
        
        
            radioStation: 
        , 
        
        
            trackListName: 
        , 
        
        
            forceNow: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x4E404A9361F75BB2u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x4E404A9361F75BB2u64;
        
        let result = invoke_raw!(
            hash,
                radioStation, 
                trackListName, 
                forceNow
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn set_custom_radio_track_list_raw(
        radioStation: , 
        trackListName: , 
        forceNow: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x4E404A9361F75BB2u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x4E404A9361F75BB2u64;

        invoke_raw_typed!(
            hash,
                radioStation, 
                trackListName, 
                forceNow
        )
    }
}

/// Used to determine whether conversation should use robot speech or not



pub fn set_conversation_audio_placeholder_safe(
        
        
            isPlaceHolder: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x61631F5DF50D1C34u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x61631F5DF50D1C34u64;
        
        let result = invoke_raw!(
            hash,
                isPlaceHolder
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn set_conversation_audio_placeholder_raw(
        isPlaceHolder: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x61631F5DF50D1C34u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x61631F5DF50D1C34u64;

        invoke_raw_typed!(
            hash,
                isPlaceHolder
        )
    }
}

/// ## Return value
Returns true if the audio for the Multiplayer data set has loaded



pub fn has_loaded_mp_data_set_safe(
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x544810ED9DB6BBE6u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x544810ED9DB6BBE6u64;
        
        let result = invoke_raw!(
            hash,
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn has_loaded_mp_data_set_raw(
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x544810ED9DB6BBE6u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x544810ED9DB6BBE6u64;

        invoke_raw_typed!(
            hash,
        )
    }
}

/// Find the radio station list [here](https://gist.github.com/4mmonium/b47d6512a2d992cbf4eea15d9038b581)



pub fn set_veh_radio_station_safe(
        
        
            vehicle: 
        , 
        
        
            radioStation: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x1B9C0099CB942AC6u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x1B9C0099CB942AC6u64;
        
        let result = invoke_raw!(
            hash,
                vehicle, 
                radioStation
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn set_veh_radio_station_raw(
        vehicle: , 
        radioStation: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x1B9C0099CB942AC6u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x1B9C0099CB942AC6u64;

        invoke_raw_typed!(
            hash,
                vehicle, 
                radioStation
        )
    }
}

/// ## Return value
Returns true if the audio for the Single Player data set has loaded



pub fn has_loaded_sp_data_set_safe(
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x5B50ABB1FE3746F4u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x5B50ABB1FE3746F4u64;
        
        let result = invoke_raw!(
            hash,
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn has_loaded_sp_data_set_raw(
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x5B50ABB1FE3746F4u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x5B50ABB1FE3746F4u64;

        invoke_raw_typed!(
            hash,
        )
    }
}

/// ## Return value
Returns true if the preloaded conversation has finished preparing.



pub fn get_is_preloaded_conversation_ready_safe(
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xE73364DB90778FFAu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xE73364DB90778FFAu64;
        
        let result = invoke_raw!(
            hash,
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn get_is_preloaded_conversation_ready_raw(
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xE73364DB90778FFAu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xE73364DB90778FFAu64;

        invoke_raw_typed!(
            hash,
        )
    }
}

/// ```
All found occurrences in b617d, sorted alphabetically and identical lines removed: pastebin.com/eeFc5DiW  
gtaforums.com/topic/795622-audio-for-mods  
```



pub fn play_sound_from_coord_safe(
        
        
            soundId: 
        , 
        
        
            audioName: 
        , 
        
        
            x: 
        , 
        
        
            y: 
        , 
        
        
            z: 
        , 
        
        
            audioRef: 
        , 
        
        
            isNetwork: 
        , 
        
        
            range: 
        , 
        
        
            p8: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x8D8686B622B88120u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x8D8686B622B88120u64;
        
        let result = invoke_raw!(
            hash,
                soundId, 
                audioName, 
                x, 
                y, 
                z, 
                audioRef, 
                isNetwork, 
                range, 
                p8
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn play_sound_from_coord_raw(
        soundId: , 
        audioName: , 
        x: , 
        y: , 
        z: , 
        audioRef: , 
        isNetwork: , 
        range: , 
        p8: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x8D8686B622B88120u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x8D8686B622B88120u64;

        invoke_raw_typed!(
            hash,
                soundId, 
                audioName, 
                x, 
                y, 
                z, 
                audioRef, 
                isNetwork, 
                range, 
                p8
        )
    }
}

/// ```
AUDIO::SET_VARIABLE_ON_UNDER_WATER_STREAM("inTunnel", 1.0);
AUDIO::SET_VARIABLE_ON_UNDER_WATER_STREAM("inTunnel", 0.0);
```



pub fn set_variable_on_under_water_stream_safe(
        
        
            variableName: 
        , 
        
        
            value: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x733ADF241531E5C2u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x733ADF241531E5C2u64;
        
        let result = invoke_raw!(
            hash,
                variableName, 
                value
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn set_variable_on_under_water_stream_raw(
        variableName: , 
        value: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x733ADF241531E5C2u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x733ADF241531E5C2u64;

        invoke_raw_typed!(
            hash,
                variableName, 
                value
        )
    }
}

/// ```
All occurrences found in b617d, sorted alphabetically and identical lines removed:
AUDIO::SET_CUTSCENE_AUDIO_OVERRIDE("_AK");
AUDIO::SET_CUTSCENE_AUDIO_OVERRIDE("_CUSTOM");
AUDIO::SET_CUTSCENE_AUDIO_OVERRIDE("_TOOTHLESS");
```

Add a suffix to the cutscene audio name. Call before loading the cutscene.



pub fn set_cutscene_audio_override_safe(
        
        
            name: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x3B4BF5F0859204D9u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x3B4BF5F0859204D9u64;
        
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
pub fn set_cutscene_audio_override_raw(
        name: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x3B4BF5F0859204D9u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x3B4BF5F0859204D9u64;

        invoke_raw_typed!(
            hash,
                name
        )
    }
}

/// Allows script to trigger a sweetener footstep sound



pub fn use_footstep_script_sweeteners_safe(
        
        
            ped: 
        , 
        
        
            useSweetner: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xBF4DC1784BE94DFAu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xBF4DC1784BE94DFAu64;
        
        let result = invoke_raw!(
            hash,
                ped, 
                useSweetner
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn use_footstep_script_sweeteners_raw(
        ped: , 
        useSweetner: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xBF4DC1784BE94DFAu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xBF4DC1784BE94DFAu64;

        invoke_raw_typed!(
            hash,
                ped, 
                useSweetner
        )
    }
}

/// ## Parameters
*



pub fn is_alarm_playing_safe(
        
        
            alarmName: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x226435CB96CCFC8Cu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x226435CB96CCFC8Cu64;
        
        let result = invoke_raw!(
            hash,
                alarmName
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn is_alarm_playing_raw(
        alarmName: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x226435CB96CCFC8Cu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x226435CB96CCFC8Cu64;

        invoke_raw_typed!(
            hash,
                alarmName
        )
    }
}

/// ```c
enum eAudAnimalMood {
	AUD_ANIMAL_MOOD_ANGRY = 0,
	AUD_ANIMAL_MOOD_PLAYFUL = 1,

	AUD_ANIMAL_MOOD_NUM_MOODS = 2
}
```



pub fn set_animal_mood_safe(
        
        
            animal: 
        , 
        
        
            mood: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xCC97B29285B1DC3Bu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xCC97B29285B1DC3Bu64;
        
        let result = invoke_raw!(
            hash,
                animal, 
                mood
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn set_animal_mood_raw(
        animal: , 
        mood: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xCC97B29285B1DC3Bu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xCC97B29285B1DC3Bu64;

        invoke_raw_typed!(
            hash,
                animal, 
                mood
        )
    }
}

/// ## Return value
Returns true if the first batch of lines for the currently requested conversation have loaded successfully.



pub fn is_scripted_conversation_loaded_safe(
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xDF0D54BE7A776737u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xDF0D54BE7A776737u64;
        
        let result = invoke_raw!(
            hash,
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn is_scripted_conversation_loaded_raw(
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xDF0D54BE7A776737u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xDF0D54BE7A776737u64;

        invoke_raw_typed!(
            hash,
        )
    }
}

/// Toggles fake distant siren sounds



pub fn distant_cop_car_sirens_safe(
        
        
            shouldPlay: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x552369F549563AD5u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x552369F549563AD5u64;
        
        let result = invoke_raw!(
            hash,
                shouldPlay
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn distant_cop_car_sirens_raw(
        shouldPlay: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x552369F549563AD5u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x552369F549563AD5u64;

        invoke_raw_typed!(
            hash,
                shouldPlay
        )
    }
}

/// Toggles the incoming missile warning system for specified vehicle.



pub fn set_vehicle_missile_warning_enabled_safe(
        
        
            vehicle: 
        , 
        
        
            toggle: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xF3365489E0DD50F9u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xF3365489E0DD50F9u64;
        
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
pub fn set_vehicle_missile_warning_enabled_raw(
        vehicle: , 
        toggle: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xF3365489E0DD50F9u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xF3365489E0DD50F9u64;

        invoke_raw_typed!(
            hash,
                vehicle, 
                toggle
        )
    }
}

/// ## Return value
Returns true if the mobile phone radio is active



pub fn is_mobile_phone_radio_active_safe(
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xB35CE999E8EF317Eu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xB35CE999E8EF317Eu64;
        
        let result = invoke_raw!(
            hash,
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn is_mobile_phone_radio_active_raw(
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xB35CE999E8EF317Eu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xB35CE999E8EF317Eu64;

        invoke_raw_typed!(
            hash,
        )
    }
}

/// ## Parameters
*



pub fn _0x58bb377bec7cd5f4_safe(
        
        
            p0: 
        , 
        
        
            p1: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x58BB377BEC7CD5F4u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x58BB377BEC7CD5F4u64;
        
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
pub fn _0x58bb377bec7cd5f4_raw(
        p0: , 
        p1: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x58BB377BEC7CD5F4u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x58BB377BEC7CD5F4u64;

        invoke_raw_typed!(
            hash,
                p0, 
                p1
        )
    }
}

/// ## Return value
Returns the current volume slider position from 0 to 10



pub fn get_music_vol_slider_safe(
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x3A48AB4445D499BEu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x3A48AB4445D499BEu64;
        
        let result = invoke_raw!(
            hash,
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn get_music_vol_slider_raw(
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x3A48AB4445D499BEu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x3A48AB4445D499BEu64;

        invoke_raw_typed!(
            hash,
        )
    }
}

/// **This native does absolutely nothing, just a nullsub**



pub fn _0x02e93c796abd3a97_safe(
        
        
            p0: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x02E93C796ABD3A97u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x02E93C796ABD3A97u64;
        
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
pub fn _0x02e93c796abd3a97_raw(
        p0: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x02E93C796ABD3A97u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x02E93C796ABD3A97u64;

        invoke_raw_typed!(
            hash,
                p0
        )
    }
}

/// ## Parameters
*



pub fn stop_alarm_safe(
        
        
            alarmName: 
        , 
        
        
            instantStop: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xA1CADDCD98415A41u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xA1CADDCD98415A41u64;
        
        let result = invoke_raw!(
            hash,
                alarmName, 
                instantStop
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn stop_alarm_raw(
        alarmName: , 
        instantStop: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xA1CADDCD98415A41u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xA1CADDCD98415A41u64;

        invoke_raw_typed!(
            hash,
                alarmName, 
                instantStop
        )
    }
}

/// ## Parameters
*



pub fn set_position_for_null_conv_ped_safe(
        
        
            speakerConversationIndex: 
        , 
        
        
            x: 
        , 
        
        
            y: 
        , 
        
        
            z: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x33E3C6C6F2F0B506u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x33E3C6C6F2F0B506u64;
        
        let result = invoke_raw!(
            hash,
                speakerConversationIndex, 
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
pub fn set_position_for_null_conv_ped_raw(
        speakerConversationIndex: , 
        x: , 
        y: , 
        z: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x33E3C6C6F2F0B506u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x33E3C6C6F2F0B506u64;

        invoke_raw_typed!(
            hash,
                speakerConversationIndex, 
                x, 
                y, 
                z
        )
    }
}

/// ## Parameters
*



pub fn set_ambient_zone_list_state_safe(
        
        
            zoneListName: 
        , 
        
        
            enabled: 
        , 
        
        
            forceUpdate: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x9748FA4DE50CCE3Eu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x9748FA4DE50CCE3Eu64;
        
        let result = invoke_raw!(
            hash,
                zoneListName, 
                enabled, 
                forceUpdate
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn set_ambient_zone_list_state_raw(
        zoneListName: , 
        enabled: , 
        forceUpdate: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x9748FA4DE50CCE3Eu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x9748FA4DE50CCE3Eu64;

        invoke_raw_typed!(
            hash,
                zoneListName, 
                enabled, 
                forceUpdate
        )
    }
}

/// ```
SET_VEH*
```



pub fn _0xc1805d05e6d4fe10_safe(
        
        
            vehicle: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xC1805D05E6D4FE10u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xC1805D05E6D4FE10u64;
        
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
pub fn _0xc1805d05e6d4fe10_raw(
        vehicle: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xC1805D05E6D4FE10u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xC1805D05E6D4FE10u64;

        invoke_raw_typed!(
            hash,
                vehicle
        )
    }
}

/// This native is used alongside with [`SET_VEHICLE_TYRE_BURST`](#_0xEC6A202EE4960385).

```
NativeDB Introduced: v3258
```



pub fn _force_vehicle_engine_synth_safe(
        
        
            vehicle: 
        , 
        
        
            force: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xEB7D0E1FCC8FE17Au32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xEB7D0E1FCC8FE17Au64;
        
        let result = invoke_raw!(
            hash,
                vehicle, 
                force
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn _force_vehicle_engine_synth_raw(
        vehicle: , 
        force: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xEB7D0E1FCC8FE17Au32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xEB7D0E1FCC8FE17Au64;

        invoke_raw_typed!(
            hash,
                vehicle, 
                force
        )
    }
}

/// ```
NativeDB Introduced: v1493
```

Removes all instances of a given context block.



pub fn unblock_speech_context_group_safe(
        
        
            groupName: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x2ACABED337622DF2u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x2ACABED337622DF2u64;
        
        let result = invoke_raw!(
            hash,
                groupName
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn unblock_speech_context_group_raw(
        groupName: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x2ACABED337622DF2u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x2ACABED337622DF2u64;

        invoke_raw_typed!(
            hash,
                groupName
        )
    }
}

/// ## Parameters
*



pub fn _0xb542de8c3d1cb210_safe(
        
        
            p0: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xB542DE8C3D1CB210u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xB542DE8C3D1CB210u64;
        
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
pub fn _0xb542de8c3d1cb210_raw(
        p0: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xB542DE8C3D1CB210u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xB542DE8C3D1CB210u64;

        invoke_raw_typed!(
            hash,
                p0
        )
    }
}

/// ## Parameters
*



pub fn start_script_conversation_safe(
        
        
            displaySubtitles: 
        , 
        
        
            addToBriefScreen: 
        , 
        
        
            cloneConversation: 
        , 
        
        
            interruptible: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x6B17C62C9635D2DCu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x6B17C62C9635D2DCu64;
        
        let result = invoke_raw!(
            hash,
                displaySubtitles, 
                addToBriefScreen, 
                cloneConversation, 
                interruptible
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn start_script_conversation_raw(
        displaySubtitles: , 
        addToBriefScreen: , 
        cloneConversation: , 
        interruptible: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x6B17C62C9635D2DCu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x6B17C62C9635D2DCu64;

        invoke_raw_typed!(
            hash,
                displaySubtitles, 
                addToBriefScreen, 
                cloneConversation, 
                interruptible
        )
    }
}

/// ```
NativeDB Introduced: v2372
```



pub fn _0x97ffb4adeed08066_safe(
        
        
            p0: 
        , 
        
        
            p1: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x97FFB4ADEED08066u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x97FFB4ADEED08066u64;
        
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
pub fn _0x97ffb4adeed08066_raw(
        p0: , 
        p1: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x97FFB4ADEED08066u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x97FFB4ADEED08066u64;

        invoke_raw_typed!(
            hash,
                p0, 
                p1
        )
    }
}

/// Below is a list of modes and their respective hashes.

| Mode (string)               | Hash               |
|-----------------------------|--------------------|
| SLOWMO_BIG_SCORE_JUMP       | 0x2B981B0C         |
| JSH_EXIT_TUNNEL_SLOWMO      | 0x2562AA6          |
| SLOW_MO_METH_HOUSE_RAYFIRE  | 0xDB9E1909         |
| SLOWMO_FIB4_TRUCK_SMASH     | 0x9E144347         |
| SLOWMO_PROLOGUE_VAULT       | 0xEA2E68E1         |
| SLOWMO_T1_RAYFIRE_EXPLOSION | 0xD6D358F3         |
| SLOWMO_T1_TRAILER_SMASH     | 0xBE607345         |
| BARRY_01_SLOWMO             | 0xD59540D4         |
| BARRY_02_SLOWMO             | 0x12F140B3         |
| SLOWMO_EXTREME_04           | 0xF562EA50         |
| NIGEL_02_SLOWMO_SETTING     | 0x384689B0         |



pub fn activate_audio_slowmo_mode_safe(
        
        
            mode: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xD01005D2BA2EB778u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xD01005D2BA2EB778u64;
        
        let result = invoke_raw!(
            hash,
                mode
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn activate_audio_slowmo_mode_raw(
        mode: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xD01005D2BA2EB778u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xD01005D2BA2EB778u64;

        invoke_raw_typed!(
            hash,
                mode
        )
    }
}

/// If a playback function has a soundId field but the sound doesn't need to be altered after triggering then pass a value of -1 for fire-and-forget playback, rather than getting a soundId.

SoundId's can be reused, without needing to release them and grab a new one.

If a sound's finished playing, you can reuse its SoundId to kick off another one.

If the sound's not finished playing, it'll be stopped first (fading out or whatever is set up in RAVE by the sound designer), and the new one kicked off; usually it is safer to just get a new SoundId.

SoundId's are not automatically cleaned up, you must use [RELEASE_SOUND_ID](#_0x353FC880830B88FA) after you've finished using them to allow the engine to recycle the sound id.



pub fn get_sound_id_safe(
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x430386FE9BF80B45u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x430386FE9BF80B45u64;
        
        let result = invoke_raw!(
            hash,
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn get_sound_id_raw(
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x430386FE9BF80B45u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x430386FE9BF80B45u64;

        invoke_raw_typed!(
            hash,
        )
    }
}

/// ## Return value



pub fn get_audible_music_track_text_id_safe(
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x50B196FC9ED6545Bu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x50B196FC9ED6545Bu64;
        
        let result = invoke_raw!(
            hash,
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn get_audible_music_track_text_id_raw(
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x50B196FC9ED6545Bu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x50B196FC9ED6545Bu64;

        invoke_raw_typed!(
            hash,
        )
    }
}

/// ```
6 calls in the b617d scripts, removed identical lines:
AUDIO::SET_RADIO_STATION_MUSIC_ONLY("RADIO_01_CLASS_ROCK", 1);
AUDIO::SET_RADIO_STATION_MUSIC_ONLY(AUDIO::GET_RADIO_STATION_NAME(10), 0);
AUDIO::SET_RADIO_STATION_MUSIC_ONLY(AUDIO::GET_RADIO_STATION_NAME(10), 1);
```



pub fn set_radio_station_music_only_safe(
        
        
            radioStation: 
        , 
        
        
            toggle: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x774BD811F656A122u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x774BD811F656A122u64;
        
        let result = invoke_raw!(
            hash,
                radioStation, 
                toggle
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn set_radio_station_music_only_raw(
        radioStation: , 
        toggle: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x774BD811F656A122u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x774BD811F656A122u64;

        invoke_raw_typed!(
            hash,
                radioStation, 
                toggle
        )
    }
}

/// ## Parameters
*



pub fn prepare_synchronized_audio_event_for_scene_safe(
        
        
            sceneId: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x029FE7CD1B7E2E75u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x029FE7CD1B7E2E75u64;
        
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
pub fn prepare_synchronized_audio_event_for_scene_raw(
        sceneId: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x029FE7CD1B7E2E75u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x029FE7CD1B7E2E75u64;

        invoke_raw_typed!(
            hash,
                sceneId
        )
    }
}

/// ## Parameters
*



pub fn set_ambient_zone_list_state_persistent_safe(
        
        
            ambientZone: 
        , 
        
        
            enabled: 
        , 
        
        
            forceUpdate: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xF3638DAE8C4045E1u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xF3638DAE8C4045E1u64;
        
        let result = invoke_raw!(
            hash,
                ambientZone, 
                enabled, 
                forceUpdate
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn set_ambient_zone_list_state_persistent_raw(
        ambientZone: , 
        enabled: , 
        forceUpdate: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xF3638DAE8C4045E1u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xF3638DAE8C4045E1u64;

        invoke_raw_typed!(
            hash,
                ambientZone, 
                enabled, 
                forceUpdate
        )
    }
}

/// ```
SET_H*
```



pub fn _0x9d3af56e94c9ae98_safe(
        
        
            vehicle: 
        , 
        
        
            p1: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x9D3AF56E94C9AE98u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x9D3AF56E94C9AE98u64;
        
        let result = invoke_raw!(
            hash,
                vehicle, 
                p1
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn _0x9d3af56e94c9ae98_raw(
        vehicle: , 
        p1: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x9D3AF56E94C9AE98u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x9D3AF56E94C9AE98u64;

        invoke_raw_typed!(
            hash,
                vehicle, 
                p1
        )
    }
}

/// ```
AUDIO::UNLOCK_RADIO_STATION_TRACK_LIST("RADIO_16_SILVERLAKE", "MIRRORPARK_LOCKED");
```



pub fn unlock_radio_station_track_list_safe(
        
        
            radioStation: 
        , 
        
        
            trackListName: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x031ACB6ABA18C729u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x031ACB6ABA18C729u64;
        
        let result = invoke_raw!(
            hash,
                radioStation, 
                trackListName
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn unlock_radio_station_track_list_raw(
        radioStation: , 
        trackListName: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x031ACB6ABA18C729u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x031ACB6ABA18C729u64;

        invoke_raw_typed!(
            hash,
                radioStation, 
                trackListName
        )
    }
}

/// ```
All found occurrences in b617d, sorted alphabetically and identical lines removed: pastebin.com/A8Ny8AHZ  
```



pub fn play_sound_safe(
        
        
            soundId: 
        , 
        
        
            audioName: 
        , 
        
        
            audioRef: 
        , 
        
        
            p3: 
        , 
        
        
            p4: 
        , 
        
        
            p5: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x7FF4944CC209192Du32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x7FF4944CC209192Du64;
        
        let result = invoke_raw!(
            hash,
                soundId, 
                audioName, 
                audioRef, 
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
pub fn play_sound_raw(
        soundId: , 
        audioName: , 
        audioRef: , 
        p3: , 
        p4: , 
        p5: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x7FF4944CC209192Du32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x7FF4944CC209192Du64;

        invoke_raw_typed!(
            hash,
                soundId, 
                audioName, 
                audioRef, 
                p3, 
                p4, 
                p5
        )
    }
}

/// Handles conversation interrupts and pauses, using the code-side system for improved timing and to minimize unfriendly logic interactions.



pub fn interrupt_conversation_and_pause_safe(
        
        
            interrupterPed: 
        , 
        
        
            context: 
        , 
        
        
            voiceName: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x8A694D7A68F8DC38u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x8A694D7A68F8DC38u64;
        
        let result = invoke_raw!(
            hash,
                interrupterPed, 
                context, 
                voiceName
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn interrupt_conversation_and_pause_raw(
        interrupterPed: , 
        context: , 
        voiceName: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x8A694D7A68F8DC38u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x8A694D7A68F8DC38u64;

        invoke_raw_typed!(
            hash,
                interrupterPed, 
                context, 
                voiceName
        )
    }
}

/// ## Parameters
*



pub fn set_siren_with_no_driver_safe(
        
        
            vehicle: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x1FEF0683B96EBCF2u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x1FEF0683B96EBCF2u64;
        
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
pub fn set_siren_with_no_driver_raw(
        vehicle: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x1FEF0683B96EBCF2u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x1FEF0683B96EBCF2u64;

        invoke_raw_typed!(
            hash,
                vehicle
        )
    }
}

/// ## Parameters
*



pub fn is_vehicle_audibly_damaged_safe(
        
        
            vehicle: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x5DB8010EE71FDEF2u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x5DB8010EE71FDEF2u64;
        
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
pub fn is_vehicle_audibly_damaged_raw(
        vehicle: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x5DB8010EE71FDEF2u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x5DB8010EE71FDEF2u64;

        invoke_raw_typed!(
            hash,
                vehicle
        )
    }
}

/// ## Parameters
*



pub fn reset_ped_audio_flags_safe(
        
        
            ped: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xF54BB7B61036F335u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xF54BB7B61036F335u64;
        
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
pub fn reset_ped_audio_flags_raw(
        ped: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xF54BB7B61036F335u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xF54BB7B61036F335u64;

        invoke_raw_typed!(
            hash,
                ped
        )
    }
}

/// This native is marked as a deprecated native internally, use [HINT_SCRIPT_AUDIO_BANK](#_0xFB380A29641EC31A) instead



pub fn hint_ambient_audio_bank_safe(
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x8F8C0E370AE62F5Cu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x8F8C0E370AE62F5Cu64;
        
        let result = invoke_raw!(
            hash,
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn hint_ambient_audio_bank_raw(
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x8F8C0E370AE62F5Cu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x8F8C0E370AE62F5Cu64;

        invoke_raw_typed!(
            hash,
        )
    }
}

/// ```
NativeDB Introduced: v1493
```



pub fn set_ped_cloth_events_enabled_safe(
        
        
            ped: 
        , 
        
        
            enabled: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x29DA3CA8D8B2692Du32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x29DA3CA8D8B2692Du64;
        
        let result = invoke_raw!(
            hash,
                ped, 
                enabled
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn set_ped_cloth_events_enabled_raw(
        ped: , 
        enabled: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x29DA3CA8D8B2692Du32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x29DA3CA8D8B2692Du64;

        invoke_raw_typed!(
            hash,
                ped, 
                enabled
        )
    }
}

/// For use with [PRELOAD_SCRIPT_CONVERSATION](#_0x3B3CAD6166916D87) and [GET_IS_PRELOADED_CONVERSATION_READY](#_0xE73364DB90778FFA)



pub fn start_preloaded_conversation_safe(
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x23641AFE870AF385u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x23641AFE870AF385u64;
        
        let result = invoke_raw!(
            hash,
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn start_preloaded_conversation_raw(
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x23641AFE870AF385u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x23641AFE870AF385u64;

        invoke_raw_typed!(
            hash,
        )
    }
}

/// Plays a preloaded stream back from the specified object.



pub fn play_stream_from_object_safe(
        
        
            object: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xEBAA9B64D76356FDu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xEBAA9B64D76356FDu64;
        
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
pub fn play_stream_from_object_raw(
        object: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xEBAA9B64D76356FDu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xEBAA9B64D76356FDu64;

        invoke_raw_typed!(
            hash,
                object
        )
    }
}

/// ```
SET_*
```



pub fn _sound_vehicle_horn_this_frame_safe(
        
        
            vehicle: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x9C11908013EA4715u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x9C11908013EA4715u64;
        
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
pub fn _sound_vehicle_horn_this_frame_raw(
        vehicle: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x9C11908013EA4715u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x9C11908013EA4715u64;

        invoke_raw_typed!(
            hash,
                vehicle
        )
    }
}

/// ```
From the scripts, p0:  
"ArmWrestlingIntensity",  
"INOUT",  
"Monkey_Stream",  
"ZoomLevel"  
```



pub fn set_variable_on_stream_safe(
        
        
            p0: 
        , 
        
        
            p1: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x2F9D3834AEB9EF79u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x2F9D3834AEB9EF79u64;
        
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
pub fn set_variable_on_stream_raw(
        p0: , 
        p1: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x2F9D3834AEB9EF79u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x2F9D3834AEB9EF79u64;

        invoke_raw_typed!(
            hash,
                p0, 
                p1
        )
    }
}

/// ## Parameters
*



pub fn _0x5b9853296731e88d_safe(
        
        
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
        let hash = 0x5B9853296731E88Du32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x5B9853296731E88Du64;
        
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
pub fn _0x5b9853296731e88d_raw(
        p0: , 
        p1: , 
        p2: , 
        p3: , 
        p4: , 
        p5: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x5B9853296731E88Du32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x5B9853296731E88Du64;

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
Return the playback time (in milliseconds) of the radio stations current track. 

NativeDB Introduced: v1493
```



pub fn _get_current_radio_track_playback_time_safe(
        
        
            radioStationName: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x3E65CDE5215832C1u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x3E65CDE5215832C1u64;
        
        let result = invoke_raw!(
            hash,
                radioStationName
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn _get_current_radio_track_playback_time_raw(
        radioStationName: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x3E65CDE5215832C1u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x3E65CDE5215832C1u64;

        invoke_raw_typed!(
            hash,
                radioStationName
        )
    }
}

/// Blocks *all* speech playing on the given ped, including speech triggered by natives such as [PLAY_PED_AMBIENT_SPEECH_WITH_VOICE_NATIVE](#_0x3523634255FC3318)

The flag itself is not synced, it must be called on each machine that wishes to suppress the speech.

The `SuppressOutgoingNetworkSpeech` flag can be set to `false` if you want speech triggered locally through `PLAY_PED_AMBIENT_SPEECH_*` related native calls to still be audible on remote machines, even though it was blocked on the local one.



pub fn block_all_speech_from_ped_safe(
        
        
            ped: 
        , 
        
        
            shouldBlock: 
        , 
        
        
            suppressOutgoingNetworkSpeech: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xF8AD2EED7C47E8FEu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xF8AD2EED7C47E8FEu64;
        
        let result = invoke_raw!(
            hash,
                ped, 
                shouldBlock, 
                suppressOutgoingNetworkSpeech
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn block_all_speech_from_ped_raw(
        ped: , 
        shouldBlock: , 
        suppressOutgoingNetworkSpeech: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xF8AD2EED7C47E8FEu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xF8AD2EED7C47E8FEu64;

        invoke_raw_typed!(
            hash,
                ped, 
                shouldBlock, 
                suppressOutgoingNetworkSpeech
        )
    }
}

/// ## Parameters
*



pub fn stop_synchronized_audio_event_safe(
        
        
            p0: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x92D6A88E64A94430u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x92D6A88E64A94430u64;
        
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
pub fn stop_synchronized_audio_event_raw(
        p0: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x92D6A88E64A94430u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x92D6A88E64A94430u64;

        invoke_raw_typed!(
            hash,
                p0
        )
    }
}

/// ```
List: https://pastebin.com/DCeRiaLJ
All occurrences as of Cayo Perico Heist DLC (b2189), sorted alphabetically and identical lines removed: https://git.io/JtLxM
```



pub fn play_sound_frontend_safe(
        
        
            soundId: 
        , 
        
        
            audioName: 
        , 
        
        
            audioRef: 
        , 
        
        
            p3: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x67C540AA08E4A6F5u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x67C540AA08E4A6F5u64;
        
        let result = invoke_raw!(
            hash,
                soundId, 
                audioName, 
                audioRef, 
                p3
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn play_sound_frontend_raw(
        soundId: , 
        audioName: , 
        audioRef: , 
        p3: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x67C540AA08E4A6F5u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x67C540AA08E4A6F5u64;

        invoke_raw_typed!(
            hash,
                soundId, 
                audioName, 
                audioRef, 
                p3
        )
    }
}

/// Unloads tennis vocalization banks loaded with [`REQUEST_TENNIS_BANKS`](#_0x4ADA3F19BE4A6047).



pub fn unrequest_tennis_banks_safe(
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x0150B6FF25A9E2E5u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x0150B6FF25A9E2E5u64;
        
        let result = invoke_raw!(
            hash,
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn unrequest_tennis_banks_raw(
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x0150B6FF25A9E2E5u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x0150B6FF25A9E2E5u64;

        invoke_raw_typed!(
            hash,
        )
    }
}

/// This native had a 4th parameter added in newer game builds
`syncOverNetwork` creates a `CPedPlayPainEvent` when set to true, by default this variable is false.

You won't be able to use this for clones (remote pedestrians that are not owned by you) or migrating peds if `syncOverNetwork` is set to true; it simply won't execute. 

The `ped` should also have speech for this to work.

```c
enum eAudDamageReason {
	AUD_DAMAGE_REASON_DEFAULT = 0,
	AUD_DAMAGE_REASON_FALLING = 1,
	AUD_DAMAGE_REASON_SUPER_FALLING = 2,
	AUD_DAMAGE_REASON_SCREAM_PANIC = 3,
	AUD_DAMAGE_REASON_SCREAM_PANIC_SHORT = 4,
	AUD_DAMAGE_REASON_SCREAM_SCARED = 5,
	AUD_DAMAGE_REASON_SCREAM_SHOCKED = 6,
	AUD_DAMAGE_REASON_SCREAM_TERROR = 7,
	AUD_DAMAGE_REASON_ON_FIRE = 8,
	AUD_DAMAGE_REASON_DROWNING = 9,
	// drowning on the surface of water, after we time out
	AUD_DAMAGE_REASON_SURFACE_DROWNING = 10,
	AUD_DAMAGE_REASON_INHALE = 11,
	AUD_DAMAGE_REASON_EXHALE = 12,
	AUD_DAMAGE_REASON_POST_FALL_GRUNT = 13,
	AUD_DAMAGE_REASON_ENTERING_RAGDOLL_DEATH = 14,
	AUD_DAMAGE_REASON_EXPLOSION = 15,
	AUD_DAMAGE_REASON_MELEE = 16,
	AUD_DAMAGE_REASON_SHOVE = 17,
	AUD_DAMAGE_REASON_WHEEZE = 18,
	AUD_DAMAGE_REASON_COUGH = 19,
	AUD_DAMAGE_REASON_TAZER = 20,
	AUD_DAMAGE_REASON_EXHAUSTION = 21,
	AUD_DAMAGE_REASON_CLIMB_LARGE = 22,
	AUD_DAMAGE_REASON_CLIMB_SMALL = 23,
	AUD_DAMAGE_REASON_JUMP = 24,
	AUD_DAMAGE_REASON_COWER = 25,
	AUD_DAMAGE_REASON_WHIMPER = 26,
	AUD_DAMAGE_REASON_DYING_MOAN = 27,
	AUD_DAMAGE_REASON_CYCLING_EXHALE = 28,
	AUD_DAMAGE_REASON_PAIN_RAPIDS = 29,
	AUD_DAMAGE_REASON_SNEEZE = 30,
	AUD_DAMAGE_REASON_MELEE_SMALL_GRUNT = 31,
	AUD_DAMAGE_REASON_MELEE_LARGE_GRUNT = 32,
	AUD_DAMAGE_REASON_POST_FALL_GRUNT_LOW = 33
}
```



pub fn play_pain_safe(
        
        
            ped: 
        , 
        
        
            damageReason: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xBC9AE166038A5CECu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xBC9AE166038A5CECu64;
        
        let result = invoke_raw!(
            hash,
                ped, 
                damageReason
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn play_pain_raw(
        ped: , 
        damageReason: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xBC9AE166038A5CECu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xBC9AE166038A5CECu64;

        invoke_raw_typed!(
            hash,
                ped, 
                damageReason
        )
    }
}

/// ## Parameters
*



pub fn is_audio_scene_active_safe(
        
        
            scene: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xB65B60556E2A9225u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xB65B60556E2A9225u64;
        
        let result = invoke_raw!(
            hash,
                scene
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn is_audio_scene_active_raw(
        scene: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xB65B60556E2A9225u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xB65B60556E2A9225u64;

        invoke_raw_typed!(
            hash,
                scene
        )
    }
}

/// ## Parameters
*



pub fn set_radio_auto_unfreeze_safe(
        
        
            toggle: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xC1AA9F53CE982990u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xC1AA9F53CE982990u64;
        
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
pub fn set_radio_auto_unfreeze_raw(
        toggle: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xC1AA9F53CE982990u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xC1AA9F53CE982990u64;

        invoke_raw_typed!(
            hash,
                toggle
        )
    }
}

/// ## Return value



pub fn audio_is_scripted_music_playing_safe(
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x845FFC3A4FEEFA3Eu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x845FFC3A4FEEFA3Eu64;
        
        let result = invoke_raw!(
            hash,
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn audio_is_scripted_music_playing_raw(
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x845FFC3A4FEEFA3Eu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x845FFC3A4FEEFA3Eu64;

        invoke_raw_typed!(
            hash,
        )
    }
}

/// ```
List of all usable event names found in b617d used with this native. Sorted alphabetically and identical names removed: pastebin.com/RzDFmB1W  
All music event names found in the b617d scripts: pastebin.com/GnYt0R3P  
```



pub fn trigger_music_event_safe(
        
        
            eventName: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x706D57B0F50DA710u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x706D57B0F50DA710u64;
        
        let result = invoke_raw!(
            hash,
                eventName
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn trigger_music_event_raw(
        eventName: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x706D57B0F50DA710u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x706D57B0F50DA710u64;

        invoke_raw_typed!(
            hash,
                eventName
        )
    }
}

/// _0x19AF7ED9B9D23058 native function



pub fn _0x19af7ed9b9d23058_safe(
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x19AF7ED9B9D23058u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x19AF7ED9B9D23058u64;
        
        let result = invoke_raw!(
            hash,
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn _0x19af7ed9b9d23058_raw(
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x19AF7ED9B9D23058u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x19AF7ED9B9D23058u64;

        invoke_raw_typed!(
            hash,
        )
    }
}

/// ## Parameters
*



pub fn get_radio_station_name_safe(
        
        
            stationIndex: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xB28ECA15046CA8B9u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xB28ECA15046CA8B9u64;
        
        let result = invoke_raw!(
            hash,
                stationIndex
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn get_radio_station_name_raw(
        stationIndex: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xB28ECA15046CA8B9u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xB28ECA15046CA8B9u64;

        invoke_raw_typed!(
            hash,
                stationIndex
        )
    }
}

/// ## Return value



pub fn is_player_veh_radio_enable_safe(
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x5F43D83FD6738741u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x5F43D83FD6738741u64;
        
        let result = invoke_raw!(
            hash,
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn is_player_veh_radio_enable_raw(
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x5F43D83FD6738741u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x5F43D83FD6738741u64;

        invoke_raw_typed!(
            hash,
        )
    }
}

/// Prepares any banks required to play the given alarm



pub fn prepare_alarm_safe(
        
        
            alarmName: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x9D74AE343DB65533u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x9D74AE343DB65533u64;
        
        let result = invoke_raw!(
            hash,
                alarmName
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn prepare_alarm_raw(
        alarmName: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x9D74AE343DB65533u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x9D74AE343DB65533u64;

        invoke_raw_typed!(
            hash,
                alarmName
        )
    }
}

/// ## Parameters
*



pub fn _set_ped_audio_gender_safe(
        
        
            ped: 
        , 
        
        
            p1: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xA5342D390CDA41D6u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xA5342D390CDA41D6u64;
        
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
pub fn _set_ped_audio_gender_raw(
        ped: , 
        p1: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xA5342D390CDA41D6u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xA5342D390CDA41D6u64;

        invoke_raw_typed!(
            hash,
                ped, 
                p1
        )
    }
}

/// ## Return value
Returns the play time in milliseconds of the current score track.



pub fn get_music_playtime_safe(
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xE7A0D23DC414507Bu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xE7A0D23DC414507Bu64;
        
        let result = invoke_raw!(
            hash,
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn get_music_playtime_raw(
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xE7A0D23DC414507Bu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xE7A0D23DC414507Bu64;

        invoke_raw_typed!(
            hash,
        )
    }
}

/// ## Parameters
*



pub fn play_deferred_sound_frontend_safe(
        
        
            soundName: 
        , 
        
        
            soundsetName: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xCADA5A0D0702381Eu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xCADA5A0D0702381Eu64;
        
        let result = invoke_raw!(
            hash,
                soundName, 
                soundsetName
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn play_deferred_sound_frontend_raw(
        soundName: , 
        soundsetName: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xCADA5A0D0702381Eu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xCADA5A0D0702381Eu64;

        invoke_raw_typed!(
            hash,
                soundName, 
                soundsetName
        )
    }
}

/// ## Parameters
*



pub fn set_vehicle_audio_engine_damage_factor_safe(
        
        
            vehicle: 
        , 
        
        
            damageFactor: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x59E7B488451F4D3Au32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x59E7B488451F4D3Au64;
        
        let result = invoke_raw!(
            hash,
                vehicle, 
                damageFactor
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn set_vehicle_audio_engine_damage_factor_raw(
        vehicle: , 
        damageFactor: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x59E7B488451F4D3Au32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x59E7B488451F4D3Au64;

        invoke_raw_typed!(
            hash,
                vehicle, 
                damageFactor
        )
    }
}

/// Overrides the calculated ped density that is used to modulate the ambient ped walla sounds (in exteriors only)

If you want to use this for interiors, use [SET_PED_INTERIOR_WALLA_DENSITY](#_0x8BF907833BE275DE)



pub fn set_ped_walla_density_safe(
        
        
            density: 
        , 
        
        
            applyValue: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x149AEE66F0CB3A99u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x149AEE66F0CB3A99u64;
        
        let result = invoke_raw!(
            hash,
                density, 
                applyValue
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn set_ped_walla_density_raw(
        density: , 
        applyValue: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x149AEE66F0CB3A99u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x149AEE66F0CB3A99u64;

        invoke_raw_typed!(
            hash,
                density, 
                applyValue
        )
    }
}

/// Stops the sound from currently playing, there isn't a way to resume a sound
after stopping it.



pub fn stop_sound_safe(
        
        
            soundId: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xA3B0C41BA5CC0BB5u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xA3B0C41BA5CC0BB5u64;
        
        let result = invoke_raw!(
            hash,
                soundId
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn stop_sound_raw(
        soundId: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xA3B0C41BA5CC0BB5u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xA3B0C41BA5CC0BB5u64;

        invoke_raw_typed!(
            hash,
                soundId
        )
    }
}

/// This native allows a scripter to override the current underwater stream.
It needs to be called before going into the water

It needs to also be called with OVERRIDE_UNDERWATER_STREAM("", false) in order to stop overriding.



pub fn override_underwater_stream_safe(
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xF2A9CDABCEA04BD6u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xF2A9CDABCEA04BD6u64;
        
        let result = invoke_raw!(
            hash,
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn override_underwater_stream_raw(
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xF2A9CDABCEA04BD6u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xF2A9CDABCEA04BD6u64;

        invoke_raw_typed!(
            hash,
        )
    }
}

/// Plays a preloaded stream back from the specified ped vehicle



pub fn play_stream_from_vehicle_safe(
        
        
            vehicle: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xB70374A758007DFAu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xB70374A758007DFAu64;
        
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
pub fn play_stream_from_vehicle_raw(
        vehicle: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xB70374A758007DFAu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xB70374A758007DFAu64;

        invoke_raw_typed!(
            hash,
                vehicle
        )
    }
}

/// Calls the same internal function [`_SET_PED_VOICE_GROUP`](#_0x7CDC8C3B89F661B3) calls, but passes `voiceGroupHash` (defined as a parameter in the referenced native) as `0`.



pub fn set_ped_voice_full_safe(
        
        
            ped: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x40CF0D12D142A9E8u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x40CF0D12D142A9E8u64;
        
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
pub fn set_ped_voice_full_raw(
        ped: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x40CF0D12D142A9E8u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x40CF0D12D142A9E8u64;

        invoke_raw_typed!(
            hash,
                ped
        )
    }
}

/// ## Parameters
*



pub fn init_synch_scene_audio_with_entity_safe(
        
        
            audioName: 
        , 
        
        
            entity: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x950A154B8DAB6185u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x950A154B8DAB6185u64;
        
        let result = invoke_raw!(
            hash,
                audioName, 
                entity
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn init_synch_scene_audio_with_entity_raw(
        audioName: , 
        entity: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x950A154B8DAB6185u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x950A154B8DAB6185u64;

        invoke_raw_typed!(
            hash,
                audioName, 
                entity
        )
    }
}

/// ## Parameters
*



pub fn use_siren_as_horn_safe(
        
        
            vehicle: 
        , 
        
        
            sirenAsHorn: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xFA932DE350266EF8u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xFA932DE350266EF8u64;
        
        let result = invoke_raw!(
            hash,
                vehicle, 
                sirenAsHorn
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn use_siren_as_horn_raw(
        vehicle: , 
        sirenAsHorn: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xFA932DE350266EF8u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xFA932DE350266EF8u64;

        invoke_raw_typed!(
            hash,
                vehicle, 
                sirenAsHorn
        )
    }
}

/// Doesn't have an effect in Story Mode.

```
NativeDB Introduced: v2372
```



pub fn _set_radio_station_is_visible_safe(
        
        
            radioStation: 
        , 
        
        
            toggle: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x4CAFEBFA21EC188Du32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x4CAFEBFA21EC188Du64;
        
        let result = invoke_raw!(
            hash,
                radioStation, 
                toggle
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn _set_radio_station_is_visible_raw(
        radioStation: , 
        toggle: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x4CAFEBFA21EC188Du32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x4CAFEBFA21EC188Du64;

        invoke_raw_typed!(
            hash,
                radioStation, 
                toggle
        )
    }
}

/// This is marked as a deprecated function internally, please use [HINT_SCRIPT_AUDIO_BANK](#_0xFB380A29641EC31A) instead.



pub fn hint_mission_audio_bank_safe(
        
        
            bankName: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x40763EA7B9B783E7u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x40763EA7B9B783E7u64;
        
        let result = invoke_raw!(
            hash,
                bankName
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn hint_mission_audio_bank_raw(
        bankName: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x40763EA7B9B783E7u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x40763EA7B9B783E7u64;

        invoke_raw_typed!(
            hash,
                bankName
        )
    }
}

/// ## Parameters
*



pub fn is_ambient_speech_disabled_safe(
        
        
            ped: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x932C2D096A2C3FFFu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x932C2D096A2C3FFFu64;
        
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
pub fn is_ambient_speech_disabled_raw(
        ped: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x932C2D096A2C3FFFu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x932C2D096A2C3FFFu64;

        invoke_raw_typed!(
            hash,
                ped
        )
    }
}

/// ## Parameters
*



pub fn unlock_mission_news_story_safe(
        
        
            newsStory: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xB165AB7C248B2DC1u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xB165AB7C248B2DC1u64;
        
        let result = invoke_raw!(
            hash,
                newsStory
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn unlock_mission_news_story_raw(
        newsStory: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xB165AB7C248B2DC1u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xB165AB7C248B2DC1u64;

        invoke_raw_typed!(
            hash,
                newsStory
        )
    }
}

/// ```
SET_VARIABLE_ON_*
```



pub fn _set_variable_on_cutscene_audio_safe(
        
        
            variableName: 
        , 
        
        
            value: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xBCC29F935ED07688u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xBCC29F935ED07688u64;
        
        let result = invoke_raw!(
            hash,
                variableName, 
                value
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn _set_variable_on_cutscene_audio_raw(
        variableName: , 
        value: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xBCC29F935ED07688u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xBCC29F935ED07688u64;

        invoke_raw_typed!(
            hash,
                variableName, 
                value
        )
    }
}

/// ## Parameters
*



pub fn _0xbef34b1d9624d5dd_safe(
        
        
            p0: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xBEF34B1D9624D5DDu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xBEF34B1D9624D5DDu64;
        
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
pub fn _0xbef34b1d9624d5dd_raw(
        p0: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xBEF34B1D9624D5DDu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xBEF34B1D9624D5DDu64;

        invoke_raw_typed!(
            hash,
                p0
        )
    }
}

/// Sets the priority for the given vehicle. This is a hint for the audio system as to what LOD the
vehicle should use.

'High' priority will bump up the activation range significantly and prevent it
from dropping when the vehicle is not within the view frustrum.

'Max' will attempt to keep the vehicle at maximum LOD regardless of how far it is from the listener or what it is currently doing. Be careful with this!

There is a hard limit of 5 simulataneous granular cars (including the player) so we are quite limited on the number
we can play at once, so setting vehicles to max priority will reduce the number of engines availble for regular NPC vehicles


```c
enum eAudVehiclePriority {
	AUDIO_VEHICLE_PRIORITY_NORMAL = 0,
	AUDIO_VEHICLE_PRIORITY_MEDIUM = 1,
	AUDIO_VEHICLE_PRIORITY_MAX = 2,
	AUDIO_VEHICLE_PRIORITY_HIGH = 3,
}
```



pub fn set_audio_vehicle_priority_safe(
        
        
            vehicle: 
        , 
        
        
            priority: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xE5564483E407F914u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xE5564483E407F914u64;
        
        let result = invoke_raw!(
            hash,
                vehicle, 
                priority
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn set_audio_vehicle_priority_raw(
        vehicle: , 
        priority: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xE5564483E407F914u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xE5564483E407F914u64;

        invoke_raw_typed!(
            hash,
                vehicle, 
                priority
        )
    }
}

/// Resets the ambient zone enabled/disabled status to its value before it was modified by this script

Default behaviour is that any state change only gets applied once the player leaves the zone.



pub fn clear_ambient_zone_state_safe(
        
        
            zoneName: 
        , 
        
        
            forceUpdate: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x218DD44AAAC964FFu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x218DD44AAAC964FFu64;
        
        let result = invoke_raw!(
            hash,
                zoneName, 
                forceUpdate
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn clear_ambient_zone_state_raw(
        zoneName: , 
        forceUpdate: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x218DD44AAAC964FFu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x218DD44AAAC964FFu64;

        invoke_raw_typed!(
            hash,
                zoneName, 
                forceUpdate
        )
    }
}

/// ```
Only found this one in the decompiled scripts:  
AUDIO::SET_RADIO_TRACK("RADIO_03_HIPHOP_NEW", "ARM1_RADIO_STARTS");  
```



pub fn set_radio_track_safe(
        
        
            radioStation: 
        , 
        
        
            radioTrack: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xB39786F201FEE30Bu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xB39786F201FEE30Bu64;
        
        let result = invoke_raw!(
            hash,
                radioStation, 
                radioTrack
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn set_radio_track_raw(
        radioStation: , 
        radioTrack: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xB39786F201FEE30Bu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xB39786F201FEE30Bu64;

        invoke_raw_typed!(
            hash,
                radioStation, 
                radioTrack
        )
    }
}

/// ## Parameters
*



pub fn is_ped_in_current_conversation_safe(
        
        
            ped: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x049E937F18F4020Cu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x049E937F18F4020Cu64;
        
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
pub fn is_ped_in_current_conversation_raw(
        ped: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x049E937F18F4020Cu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x049E937F18F4020Cu64;

        invoke_raw_typed!(
            hash,
                ped
        )
    }
}

/// Resets the override for [SET_VEHICLE_STARTUP_REV_SOUND](#_0xF1F8157B8C3F171C)



pub fn reset_vehicle_startup_rev_sound_safe(
        
        
            vehicle: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xD2DCCD8E16E20997u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xD2DCCD8E16E20997u64;
        
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
pub fn reset_vehicle_startup_rev_sound_raw(
        vehicle: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xD2DCCD8E16E20997u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xD2DCCD8E16E20997u64;

        invoke_raw_typed!(
            hash,
                vehicle
        )
    }
}

/// ## Parameters
*



pub fn set_positioned_player_vehicle_radio_emitter_enabled_safe(
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xDA07819E452FFE8Fu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xDA07819E452FFE8Fu64;
        
        let result = invoke_raw!(
            hash,
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn set_positioned_player_vehicle_radio_emitter_enabled_raw(
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xDA07819E452FFE8Fu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xDA07819E452FFE8Fu64;

        invoke_raw_typed!(
            hash,
        )
    }
}

/// Plays a preloaded stream back from the specified Vector3.



pub fn play_stream_from_position_safe(
        
        
            x: 
        , 
        
        
            y: 
        , 
        
        
            z: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x21442F412E8DE56Bu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x21442F412E8DE56Bu64;
        
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
pub fn play_stream_from_position_raw(
        x: , 
        y: , 
        z: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x21442F412E8DE56Bu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x21442F412E8DE56Bu64;

        invoke_raw_typed!(
            hash,
                x, 
                y, 
                z
        )
    }
}

/// Overrides the vehicle's horn hash.


When changing this hash on a vehicle, [`_GET_VEHICLE_HORN_HASH`](#_0xACB5DCCA1EC76840) will



pub fn override_veh_horn_safe(
        
        
            vehicle: 
        , 
        
        
            override: 
        , 
        
        
            hornHash: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x3CDC1E622CCE0356u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x3CDC1E622CCE0356u64;
        
        let result = invoke_raw!(
            hash,
                vehicle, 
                override, 
                hornHash
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn override_veh_horn_raw(
        vehicle: , 
        override: , 
        hornHash: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x3CDC1E622CCE0356u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x3CDC1E622CCE0356u64;

        invoke_raw_typed!(
            hash,
                vehicle, 
                override, 
                hornHash
        )
    }
}

/// ## Parameters
*



pub fn play_vehicle_door_open_sound_safe(
        
        
            vehicle: 
        , 
        
        
            doorIndex: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x3A539D52857EA82Du32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x3A539D52857EA82Du64;
        
        let result = invoke_raw!(
            hash,
                vehicle, 
                doorIndex
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn play_vehicle_door_open_sound_raw(
        vehicle: , 
        doorIndex: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x3A539D52857EA82Du32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x3A539D52857EA82Du64;

        invoke_raw_typed!(
            hash,
                vehicle, 
                doorIndex
        )
    }
}

/// STOP_STREAM native function



pub fn stop_stream_safe(
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xA4718A1419D18151u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xA4718A1419D18151u64;
        
        let result = invoke_raw!(
            hash,
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn stop_stream_raw(
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xA4718A1419D18151u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xA4718A1419D18151u64;

        invoke_raw_typed!(
            hash,
        )
    }
}

/// Stops currently playing ambient speech.



pub fn stop_current_playing_ambient_speech_safe(
        
        
            ped: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xB8BEC0CA6F0EDB0Fu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xB8BEC0CA6F0EDB0Fu64;
        
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
pub fn stop_current_playing_ambient_speech_raw(
        ped: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xB8BEC0CA6F0EDB0Fu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xB8BEC0CA6F0EDB0Fu64;

        invoke_raw_typed!(
            hash,
                ped
        )
    }
}

/// ## Parameters
*



pub fn set_mobile_phone_radio_state_safe(
        
        
            state: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xBF286C554784F3DFu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xBF286C554784F3DFu64;
        
        let result = invoke_raw!(
            hash,
                state
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn set_mobile_phone_radio_state_raw(
        state: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xBF286C554784F3DFu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xBF286C554784F3DFu64;

        invoke_raw_typed!(
            hash,
                state
        )
    }
}

/// Loads the tennis vocalization banks into a couple animal slots.



pub fn request_tennis_banks_safe(
        
        
            opponentPed: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x4ADA3F19BE4A6047u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x4ADA3F19BE4A6047u64;
        
        let result = invoke_raw!(
            hash,
                opponentPed
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn request_tennis_banks_raw(
        opponentPed: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x4ADA3F19BE4A6047u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x4ADA3F19BE4A6047u64;

        invoke_raw_typed!(
            hash,
                opponentPed
        )
    }
}

/// ## Parameters
*



pub fn find_radio_station_index_safe(
        
        
            stationNameHash: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x8D67489793FF428Bu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x8D67489793FF428Bu64;
        
        let result = invoke_raw!(
            hash,
                stationNameHash
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn find_radio_station_index_raw(
        stationNameHash: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x8D67489793FF428Bu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x8D67489793FF428Bu64;

        invoke_raw_typed!(
            hash,
                stationNameHash
        )
    }
}

/// ```
NativeDB Introduced: v2372
```



pub fn _set_veh_has_radio_override_safe(
        
        
            vehicle: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x3E45765F3FBB582Fu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x3E45765F3FBB582Fu64;
        
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
pub fn _set_veh_has_radio_override_raw(
        vehicle: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x3E45765F3FBB582Fu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x3E45765F3FBB582Fu64;

        invoke_raw_typed!(
            hash,
                vehicle
        )
    }
}

