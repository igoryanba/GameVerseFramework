//! TASK native functions
//! 
//! Functions for the task category (local)

use crate::types::*;
use crate::errors::{NativeError, NativeResult, validation};
use nalgebra::Vector3;
use tracing::{debug, warn};

/// ## Parameters
*



pub fn waypoint_playback_stop_aiming_or_shooting_safe(
        
        
            p0: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x47EFA040EBB8E2EAu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x47EFA040EBB8E2EAu64;
        
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
pub fn waypoint_playback_stop_aiming_or_shooting_raw(
        p0: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x47EFA040EBB8E2EAu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x47EFA040EBB8E2EAu64;

        invoke_raw_typed!(
            hash,
                p0
        )
    }
}

/// ## Parameters
*



pub fn is_mounted_weapon_task_underneath_driving_task_safe(
        
        
            ped: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xA320EF046186FA3Bu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xA320EF046186FA3Bu64;
        
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
pub fn is_mounted_weapon_task_underneath_driving_task_raw(
        ped: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xA320EF046186FA3Bu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xA320EF046186FA3Bu64;

        invoke_raw_typed!(
            hash,
                ped
        )
    }
}

/// See [`FORCE_PED_MOTION_STATE`](#_0xF28965D04F570DCA)



pub fn task_force_motion_state_safe(
        
        
            ped: 
        , 
        
        
            state: 
        , 
        
        
            p2: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x4F056E1AFFEF17ABu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x4F056E1AFFEF17ABu64;
        
        let result = invoke_raw!(
            hash,
                ped, 
                state, 
                p2
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn task_force_motion_state_raw(
        ped: , 
        state: , 
        p2: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x4F056E1AFFEF17ABu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x4F056E1AFFEF17ABu64;

        invoke_raw_typed!(
            hash,
                ped, 
                state, 
                p2
        )
    }
}

/// ```
NativeDB Introduced: v1290
```



pub fn _task_heli_escort_heli_safe(
        
        
            pilot: 
        , 
        
        
            heli1: 
        , 
        
        
            heli2: 
        , 
        
        
            p3: 
        , 
        
        
            p4: 
        , 
        
        
            p5: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xB385523325077210u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xB385523325077210u64;
        
        let result = invoke_raw!(
            hash,
                pilot, 
                heli1, 
                heli2, 
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
pub fn _task_heli_escort_heli_raw(
        pilot: , 
        heli1: , 
        heli2: , 
        p3: , 
        p4: , 
        p5: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xB385523325077210u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xB385523325077210u64;

        invoke_raw_typed!(
            hash,
                pilot, 
                heli1, 
                heli2, 
                p3, 
                p4, 
                p5
        )
    }
}

/// ## Parameters
*



pub fn waypoint_playback_get_is_paused_safe(
        
        
            p0: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x701375A7D43F01CBu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x701375A7D43F01CBu64;
        
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
pub fn waypoint_playback_get_is_paused_raw(
        p0: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x701375A7D43F01CBu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x701375A7D43F01CBu64;

        invoke_raw_typed!(
            hash,
                p0
        )
    }
}

/// ```
 TASK::TASK_SYNCHRONIZED_SCENE(ped, scene, "creatures@rottweiler@in_vehicle@std_car", "get_in", 1000.0, -8.0, 4, 0, 0x447a0000, 0);
```



pub fn task_synchronized_scene_safe(
        
        
            ped: 
        , 
        
        
            scene: 
        , 
        
        
            animDictionary: 
        , 
        
        
            animationName: 
        , 
        
        
            speed: 
        , 
        
        
            speedMultiplier: 
        , 
        
        
            duration: 
        , 
        
        
            flag: 
        , 
        
        
            playbackRate: 
        , 
        
        
            p9: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xEEA929141F699854u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xEEA929141F699854u64;
        
        let result = invoke_raw!(
            hash,
                ped, 
                scene, 
                animDictionary, 
                animationName, 
                speed, 
                speedMultiplier, 
                duration, 
                flag, 
                playbackRate, 
                p9
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn task_synchronized_scene_raw(
        ped: , 
        scene: , 
        animDictionary: , 
        animationName: , 
        speed: , 
        speedMultiplier: , 
        duration: , 
        flag: , 
        playbackRate: , 
        p9: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xEEA929141F699854u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xEEA929141F699854u64;

        invoke_raw_typed!(
            hash,
                ped, 
                scene, 
                animDictionary, 
                animationName, 
                speed, 
                speedMultiplier, 
                duration, 
                flag, 
                playbackRate, 
                p9
        )
    }
}

/// **This native does absolutely nothing, just a nullsub**



pub fn set_next_desired_move_state_safe(
        
        
            p0: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xF1B9F16E89E2C93Au32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xF1B9F16E89E2C93Au64;
        
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
pub fn set_next_desired_move_state_raw(
        p0: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xF1B9F16E89E2C93Au32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xF1B9F16E89E2C93Au64;

        invoke_raw_typed!(
            hash,
                p0
        )
    }
}

/// ## Parameters
*



pub fn task_climb_ladder_safe(
        
        
            ped: 
        , 
        
        
            p1: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xB6C987F9285A3814u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xB6C987F9285A3814u64;
        
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
pub fn task_climb_ladder_raw(
        ped: , 
        p1: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xB6C987F9285A3814u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xB6C987F9285A3814u64;

        invoke_raw_typed!(
            hash,
                ped, 
                p1
        )
    }
}

/// ```
Makes the specified ped shuffle to the next vehicle seat.  
The ped MUST be in a vehicle and the vehicle parameter MUST be the ped's current vehicle.  
```

```
NativeDB Added Parameter 3: Any p2
```



pub fn task_shuffle_to_next_vehicle_seat_safe(
        
        
            ped: 
        , 
        
        
            vehicle: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x7AA80209BDA643EBu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x7AA80209BDA643EBu64;
        
        let result = invoke_raw!(
            hash,
                ped, 
                vehicle
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn task_shuffle_to_next_vehicle_seat_raw(
        ped: , 
        vehicle: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x7AA80209BDA643EBu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x7AA80209BDA643EBu64;

        invoke_raw_typed!(
            hash,
                ped, 
                vehicle
        )
    }
}

/// ```
Occurrences in the b617d scripts:
"ARMY_GUARD",
"ARMY_HELI",
"Cinema_Downtown",
"Cinema_Morningwood",
"Cinema_Textile",
"City_Banks",
"Countryside_Banks",
"DEALERSHIP",
"GRAPESEED_PLANES",
"KORTZ_SECURITY",
"LOST_BIKERS",
"LSA_Planes",
"LSA_Planes",
"MP_POLICE",
"Observatory_Bikers",
"POLICE_POUND1",
"POLICE_POUND2",
"POLICE_POUND3",
"POLICE_POUND4",
"POLICE_POUND5"
"QUARRY",
"SANDY_PLANES",
"SCRAP_SECURITY",
"SEW_MACHINE",
"SOLOMON_GATE",
"Triathlon_1_Start",
"Triathlon_2_Start",
"Triathlon_3_Start"
Sometimes used with IS_SCENARIO_GROUP_ENABLED:
if (TASK::DOES_SCENARIO_GROUP_EXIST("Observatory_Bikers") && (!TASK::IS_SCENARIO_GROUP_ENABLED("Observatory_Bikers"))) {
else if (TASK::IS_SCENARIO_GROUP_ENABLED("BLIMP")) {
```



pub fn does_scenario_group_exist_safe(
        
        
            scenarioGroup: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xF9034C136C9E00D3u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xF9034C136C9E00D3u64;
        
        let result = invoke_raw!(
            hash,
                scenarioGroup
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn does_scenario_group_exist_raw(
        scenarioGroup: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xF9034C136C9E00D3u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xF9034C136C9E00D3u64;

        invoke_raw_typed!(
            hash,
                scenarioGroup
        )
    }
}

/// ```
Birds will try to reach the given height.  
```



pub fn set_global_min_bird_flight_height_safe(
        
        
            height: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x6C6B148586F934F7u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x6C6B148586F934F7u64;
        
        let result = invoke_raw!(
            hash,
                height
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn set_global_min_bird_flight_height_raw(
        height: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x6C6B148586F934F7u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x6C6B148586F934F7u64;

        invoke_raw_typed!(
            hash,
                height
        )
    }
}

/// ```
From re_prisonvanbreak:
TASK::TASK_GUARD_CURRENT_POSITION(l_DD, 35.0, 35.0, 1);
```



pub fn task_guard_current_position_safe(
        
        
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
        let hash = 0x4A58A47A72E3FCB4u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x4A58A47A72E3FCB4u64;
        
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
pub fn task_guard_current_position_raw(
        p0: , 
        p1: , 
        p2: , 
        p3: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x4A58A47A72E3FCB4u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x4A58A47A72E3FCB4u64;

        invoke_raw_typed!(
            hash,
                p0, 
                p1, 
                p2, 
                p3
        )
    }
}

/// All parameters except ped, vehicle, x, y, z and speed are optional; with `missionType` being only those that don't require a target entity.

If you don't want to use a parameter; pass `0` for int parameters, and `-1.0f` for the remaining float parameters.



pub fn task_vehicle_mission_coors_target_safe(
        
        
            ped: 
        , 
        
        
            vehicle: 
        , 
        
        
            x: 
        , 
        
        
            y: 
        , 
        
        
            z: 
        , 
        
        
            missionType: 
        , 
        
        
            speed: 
        , 
        
        
            drivingStyle: 
        , 
        
        
            radius: 
        , 
        
        
            straightLineDist: 
        , 
        
        
            DriveAgainstTraffic: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xF0AF20AA7731F8C3u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xF0AF20AA7731F8C3u64;
        
        let result = invoke_raw!(
            hash,
                ped, 
                vehicle, 
                x, 
                y, 
                z, 
                missionType, 
                speed, 
                drivingStyle, 
                radius, 
                straightLineDist, 
                DriveAgainstTraffic
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn task_vehicle_mission_coors_target_raw(
        ped: , 
        vehicle: , 
        x: , 
        y: , 
        z: , 
        missionType: , 
        speed: , 
        drivingStyle: , 
        radius: , 
        straightLineDist: , 
        DriveAgainstTraffic: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xF0AF20AA7731F8C3u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xF0AF20AA7731F8C3u64;

        invoke_raw_typed!(
            hash,
                ped, 
                vehicle, 
                x, 
                y, 
                z, 
                missionType, 
                speed, 
                drivingStyle, 
                radius, 
                straightLineDist, 
                DriveAgainstTraffic
        )
    }
}

/// ```
Actually has 3 params, not 2.  
p0: Ped  
p1: int (or bool?)  
p2: int  
```

```
NativeDB Added Parameter 3: Any p2
```



pub fn task_use_mobile_phone_safe(
        
        
            ped: 
        , 
        
        
            p1: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xBD2A8EC3AF4DE7DBu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xBD2A8EC3AF4DE7DBu64;
        
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
pub fn task_use_mobile_phone_raw(
        ped: , 
        p1: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xBD2A8EC3AF4DE7DBu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xBD2A8EC3AF4DE7DBu64;

        invoke_raw_typed!(
            hash,
                ped, 
                p1
        )
    }
}

/// ```
For a full list of the points, see here: goo.gl/wIH0vn
```



pub fn waypoint_recording_get_closest_waypoint_safe(
        
        
            name: 
        , 
        
        
            x: 
        , 
        
        
            y: 
        , 
        
        
            z: 
        , 
        
        
            point: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xB629A298081F876Fu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xB629A298081F876Fu64;
        
        let result = invoke_raw!(
            hash,
                name, 
                x, 
                y, 
                z, 
                point
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn waypoint_recording_get_closest_waypoint_raw(
        name: , 
        x: , 
        y: , 
        z: , 
        point: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xB629A298081F876Fu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xB629A298081F876Fu64;

        invoke_raw_typed!(
            hash,
                name, 
                x, 
                y, 
                z, 
                point
        )
    }
}

/// ## Parameters
*



pub fn task_follow_waypoint_recording_safe(
        
        
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
        let hash = 0x0759591819534F7Bu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x0759591819534F7Bu64;
        
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
pub fn task_follow_waypoint_recording_raw(
        p0: , 
        p1: , 
        p2: , 
        p3: , 
        p4: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x0759591819534F7Bu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x0759591819534F7Bu64;

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



pub fn _get_task_move_network_signal_float_safe(
        
        
            ped: 
        , 
        
        
            signalName: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x44AB0B3AFECCE242u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x44AB0B3AFECCE242u64;
        
        let result = invoke_raw!(
            hash,
                ped, 
                signalName
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn _get_task_move_network_signal_float_raw(
        ped: , 
        signalName: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x44AB0B3AFECCE242u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x44AB0B3AFECCE242u64;

        invoke_raw_typed!(
            hash,
                ped, 
                signalName
        )
    }
}

/// ## Parameters
*



pub fn set_anim_weight_safe(
        
        
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
        let hash = 0x207F1A47C0342F48u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x207F1A47C0342F48u64;
        
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
pub fn set_anim_weight_raw(
        p0: , 
        p1: , 
        p2: , 
        p3: , 
        p4: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x207F1A47C0342F48u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x207F1A47C0342F48u64;

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



pub fn get_waypoint_distance_along_route_safe(
        
        
            p0: 
        , 
        
        
            p1: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xA5B769058763E497u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xA5B769058763E497u64;
        
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
pub fn get_waypoint_distance_along_route_raw(
        p0: , 
        p1: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xA5B769058763E497u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xA5B769058763E497u64;

        invoke_raw_typed!(
            hash,
                p0, 
                p1
        )
    }
}

/// Clears the current point route. Call this before [TASK_EXTEND_ROUTE](#_0x1E7889778264843A) and [TASK_FOLLOW_POINT_ROUTE](#_0x595583281858626E).



pub fn task_flush_route_safe(
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x841142A1376E9006u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x841142A1376E9006u64;
        
        let result = invoke_raw!(
            hash,
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn task_flush_route_raw(
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x841142A1376E9006u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x841142A1376E9006u64;

        invoke_raw_typed!(
            hash,
        )
    }
}

/// ## Parameters
*



pub fn clear_ped_secondary_task_safe(
        
        
            ped: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x176CECF6F920D707u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x176CECF6F920D707u64;
        
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
pub fn clear_ped_secondary_task_raw(
        ped: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x176CECF6F920D707u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x176CECF6F920D707u64;

        invoke_raw_typed!(
            hash,
                ped
        )
    }
}

/// ```
The ped will walk or run towards goToLocation, aiming towards goToLocation or focusLocation (depending on the aimingFlag) and shooting if shootAtEnemies = true to any enemy in his path.
If the ped is closer than noRoadsDistance, the ped will ignore pathing/navmesh and go towards goToLocation directly. This could cause the ped to get stuck behind tall walls if the goToLocation is on the other side. To avoid this, use 0.0f and the ped will always use pathing/navmesh to reach his destination.
If the speed is set to 0.0f, the ped will just stand there while aiming, if set to 1.0f he will walk while aiming, 2.0f will run while aiming.
The ped will stop aiming when he is closer than distanceToStopAt to goToLocation.
I still can't figure out what unkTrue is used for. I don't notice any difference if I set it to false but in the decompiled scripts is always true.
I think that unkFlag, like the driving styles, could be a flag that "work as a list of 32 bits converted to a decimal integer. Each bit acts as a flag, and enables or disables a function". What leads me to this conclusion is the fact that in the decompiled scripts, unkFlag takes values like: 0, 1, 5 (101 in binary) and 4097 (4096 + 1 or 1000000000001 in binary). For now, I don't know what behavior enable or disable this possible flag so I leave it at 0.
Note: After some testing, using unkFlag = 16 (0x10) enables the use of sidewalks while moving towards goToLocation.
The aimingFlag takes 2 values: 0 to aim at the focusLocation, 1 to aim at where the ped is heading (goToLocation).
Example:
enum AimFlag
{
   AimAtFocusLocation,
   AimAtGoToLocation
};
Vector3 goToLocation1 = { 996.2867f, 0, -2143.044f, 0, 28.4763f, 0 }; // remember the padding.
Vector3 goToLocation2 = { 990.2867f, 0, -2140.044f, 0, 28.4763f, 0 }; // remember the padding.
Vector3 focusLocation = { 994.3478f, 0, -2136.118f, 0, 29.2463f, 0 }; // the coord z should be a little higher, around +1.0f to avoid aiming at the ground
// 1st example
TASK::TASK_GO_TO_COORD_AND_AIM_AT_HATED_ENTITIES_NEAR_COORD(pedHandle, goToLocation1.x, goToLocation1.y, goToLocation1.z, focusLocation.x, focusLocation.y, focusLocation.z, 2.0f /*run*/, true /*shoot*/, 3.0f /*stop at*/, 0.0f /*noRoadsDistance*/, true /*always true*/, 0 /*possible flag*/, AimFlag::AimAtGoToLocation, -957453492 /*FullAuto pattern*/);
// 2nd example
TASK::TASK_GO_TO_COORD_AND_AIM_AT_HATED_ENTITIES_NEAR_COORD(pedHandle, goToLocation2.x, goToLocation2.y, goToLocation2.z, focusLocation.x, focusLocation.y, focusLocation.z, 1.0f /*walk*/, false /*don't shoot*/, 3.0f /*stop at*/, 0.0f /*noRoadsDistance*/, true /*always true*/, 0 /*possible flag*/, AimFlag::AimAtFocusLocation, -957453492 /*FullAuto pattern*/);
1st example: The ped (pedhandle) will run towards goToLocation1. While running and aiming towards goToLocation1, the ped will shoot on sight to any enemy in his path, using "FullAuto" firing pattern. The ped will stop once he is closer than distanceToStopAt to goToLocation1.
2nd example: The ped will walk towards goToLocation2. This time, while walking towards goToLocation2 and aiming at focusLocation, the ped will point his weapon on sight to any enemy in his path without shooting. The ped will stop once he is closer than distanceToStopAt to goToLocation2.
```



pub fn task_go_to_coord_and_aim_at_hated_entities_near_coord_safe(
        
        
            pedHandle: 
        , 
        
        
            goToLocationX: 
        , 
        
        
            goToLocationY: 
        , 
        
        
            goToLocationZ: 
        , 
        
        
            focusLocationX: 
        , 
        
        
            focusLocationY: 
        , 
        
        
            focusLocationZ: 
        , 
        
        
            speed: 
        , 
        
        
            shootAtEnemies: 
        , 
        
        
            distanceToStopAt: 
        , 
        
        
            noRoadsDistance: 
        , 
        
        
            unkTrue: 
        , 
        
        
            unkFlag: 
        , 
        
        
            aimingFlag: 
        , 
        
        
            firingPattern: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xA55547801EB331FCu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xA55547801EB331FCu64;
        
        let result = invoke_raw!(
            hash,
                pedHandle, 
                goToLocationX, 
                goToLocationY, 
                goToLocationZ, 
                focusLocationX, 
                focusLocationY, 
                focusLocationZ, 
                speed, 
                shootAtEnemies, 
                distanceToStopAt, 
                noRoadsDistance, 
                unkTrue, 
                unkFlag, 
                aimingFlag, 
                firingPattern
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn task_go_to_coord_and_aim_at_hated_entities_near_coord_raw(
        pedHandle: , 
        goToLocationX: , 
        goToLocationY: , 
        goToLocationZ: , 
        focusLocationX: , 
        focusLocationY: , 
        focusLocationZ: , 
        speed: , 
        shootAtEnemies: , 
        distanceToStopAt: , 
        noRoadsDistance: , 
        unkTrue: , 
        unkFlag: , 
        aimingFlag: , 
        firingPattern: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xA55547801EB331FCu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xA55547801EB331FCu64;

        invoke_raw_typed!(
            hash,
                pedHandle, 
                goToLocationX, 
                goToLocationY, 
                goToLocationZ, 
                focusLocationX, 
                focusLocationY, 
                focusLocationZ, 
                speed, 
                shootAtEnemies, 
                distanceToStopAt, 
                noRoadsDistance, 
                unkTrue, 
                unkFlag, 
                aimingFlag, 
                firingPattern
        )
    }
}

/// ## Parameters
*



pub fn get_task_move_network_state_safe(
        
        
            ped: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x717E4D1F2048376Du32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x717E4D1F2048376Du64;
        
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
pub fn get_task_move_network_state_raw(
        ped: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x717E4D1F2048376Du32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x717E4D1F2048376Du64;

        invoke_raw_typed!(
            hash,
                ped
        )
    }
}

/// ## Parameters
*



pub fn set_drive_task_max_cruise_speed_safe(
        
        
            p0: 
        , 
        
        
            p1: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x404A5AA9B9F0B746u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x404A5AA9B9F0B746u64;
        
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
pub fn set_drive_task_max_cruise_speed_raw(
        p0: , 
        p1: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x404A5AA9B9F0B746u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x404A5AA9B9F0B746u64;

        invoke_raw_typed!(
            hash,
                p0, 
                p1
        )
    }
}

/// ```
Makes a ped run away from another ped (fleeTarget).  
distance = ped will flee this distance.  
fleeTime = ped will flee for this amount of time, set to "-1" to flee forever  
```



pub fn task_smart_flee_ped_safe(
        
        
            ped: 
        , 
        
        
            fleeTarget: 
        , 
        
        
            distance: 
        , 
        
        
            fleeTime: 
        , 
        
        
            p4: 
        , 
        
        
            p5: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x22B0D0E37CCB840Du32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x22B0D0E37CCB840Du64;
        
        let result = invoke_raw!(
            hash,
                ped, 
                fleeTarget, 
                distance, 
                fleeTime, 
                p4, 
                p5
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn task_smart_flee_ped_raw(
        ped: , 
        fleeTarget: , 
        distance: , 
        fleeTime: , 
        p4: , 
        p5: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x22B0D0E37CCB840Du32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x22B0D0E37CCB840Du64;

        invoke_raw_typed!(
            hash,
                ped, 
                fleeTarget, 
                distance, 
                fleeTime, 
                p4, 
                p5
        )
    }
}

/// ```
NativeDB Added Parameter 14: Any p13
```



pub fn task_go_to_coord_any_means_extra_params_with_cruise_speed_safe(
        
        
            ped: 
        , 
        
        
            x: 
        , 
        
        
            y: 
        , 
        
        
            z: 
        , 
        
        
            speed: 
        , 
        
        
            p5: 
        , 
        
        
            p6: 
        , 
        
        
            walkingStyle: 
        , 
        
        
            p8: 
        , 
        
        
            p9: 
        , 
        
        
            p10: 
        , 
        
        
            p11: 
        , 
        
        
            p12: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xB8ECD61F531A7B02u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xB8ECD61F531A7B02u64;
        
        let result = invoke_raw!(
            hash,
                ped, 
                x, 
                y, 
                z, 
                speed, 
                p5, 
                p6, 
                walkingStyle, 
                p8, 
                p9, 
                p10, 
                p11, 
                p12
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn task_go_to_coord_any_means_extra_params_with_cruise_speed_raw(
        ped: , 
        x: , 
        y: , 
        z: , 
        speed: , 
        p5: , 
        p6: , 
        walkingStyle: , 
        p8: , 
        p9: , 
        p10: , 
        p11: , 
        p12: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xB8ECD61F531A7B02u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xB8ECD61F531A7B02u64;

        invoke_raw_typed!(
            hash,
                ped, 
                x, 
                y, 
                z, 
                speed, 
                p5, 
                p6, 
                walkingStyle, 
                p8, 
                p9, 
                p10, 
                p11, 
                p12
        )
    }
}

/// ## Parameters
*



pub fn waypoint_playback_start_shooting_at_coord_safe(
        
        
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
        let hash = 0x057A25CFCC9DB671u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x057A25CFCC9DB671u64;
        
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
pub fn waypoint_playback_start_shooting_at_coord_raw(
        p0: , 
        p1: , 
        p2: , 
        p3: , 
        p4: , 
        p5: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x057A25CFCC9DB671u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x057A25CFCC9DB671u64;

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



pub fn is_waypoint_playback_going_on_for_vehicle_safe(
        
        
            vehicle: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xF5134943EA29868Cu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xF5134943EA29868Cu64;
        
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
pub fn is_waypoint_playback_going_on_for_vehicle_raw(
        vehicle: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xF5134943EA29868Cu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xF5134943EA29868Cu64;

        invoke_raw_typed!(
            hash,
                vehicle
        )
    }
}

/// [Animations list](https://alexguirre.github.io/animations-list/)



pub fn play_anim_on_running_scenario_safe(
        
        
            ped: 
        , 
        
        
            animDict: 
        , 
        
        
            animName: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x748040460F8DF5DCu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x748040460F8DF5DCu64;
        
        let result = invoke_raw!(
            hash,
                ped, 
                animDict, 
                animName
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn play_anim_on_running_scenario_raw(
        ped: , 
        animDict: , 
        animName: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x748040460F8DF5DCu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x748040460F8DF5DCu64;

        invoke_raw_typed!(
            hash,
                ped, 
                animDict, 
                animName
        )
    }
}

/// ## Parameters
*



pub fn does_scenario_exist_in_area_safe(
        
        
            x: 
        , 
        
        
            y: 
        , 
        
        
            z: 
        , 
        
        
            radius: 
        , 
        
        
            b: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x5A59271FFADD33C1u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x5A59271FFADD33C1u64;
        
        let result = invoke_raw!(
            hash,
                x, 
                y, 
                z, 
                radius, 
                b
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn does_scenario_exist_in_area_raw(
        x: , 
        y: , 
        z: , 
        radius: , 
        b: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x5A59271FFADD33C1u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x5A59271FFADD33C1u64;

        invoke_raw_typed!(
            hash,
                x, 
                y, 
                z, 
                radius, 
                b
        )
    }
}

/// ## Parameters
*



pub fn set_ped_path_may_enter_water_safe(
        
        
            ped: 
        , 
        
        
            mayEnterWater: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xF35425A4204367ECu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xF35425A4204367ECu64;
        
        let result = invoke_raw!(
            hash,
                ped, 
                mayEnterWater
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn set_ped_path_may_enter_water_raw(
        ped: , 
        mayEnterWater: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xF35425A4204367ECu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xF35425A4204367ECu64;

        invoke_raw_typed!(
            hash,
                ped, 
                mayEnterWater
        )
    }
}

/// ```
speed 1.0 = walk, 2.0 = run  
p5 1 = normal, 3 = teleport to vehicle, 8 = normal/carjack ped from seat, 16 = teleport directly into vehicle  
p6 is always 0  
```



pub fn task_enter_vehicle_safe(
        
        
            ped: 
        , 
        
        
            vehicle: 
        , 
        
        
            timeout: 
        , 
        
        
            seatIndex: 
        , 
        
        
            speed: 
        , 
        
        
            flag: 
        , 
        
        
            p6: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xC20E50AA46D09CA8u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xC20E50AA46D09CA8u64;
        
        let result = invoke_raw!(
            hash,
                ped, 
                vehicle, 
                timeout, 
                seatIndex, 
                speed, 
                flag, 
                p6
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn task_enter_vehicle_raw(
        ped: , 
        vehicle: , 
        timeout: , 
        seatIndex: , 
        speed: , 
        flag: , 
        p6: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xC20E50AA46D09CA8u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xC20E50AA46D09CA8u64;

        invoke_raw_typed!(
            hash,
                ped, 
                vehicle, 
                timeout, 
                seatIndex, 
                speed, 
                flag, 
                p6
        )
    }
}

/// ## Parameters
*



pub fn update_task_sweep_aim_position_safe(
        
        
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
        let hash = 0xBB106883F5201FC4u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xBB106883F5201FC4u64;
        
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
pub fn update_task_sweep_aim_position_raw(
        p0: , 
        p1: , 
        p2: , 
        p3: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xBB106883F5201FC4u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xBB106883F5201FC4u64;

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
For a full list of the points, see here: goo.gl/wIH0vn
```



pub fn waypoint_recording_get_num_points_safe(
        
        
            name: 
        , 
        
        
            points: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x5343532C01A07234u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x5343532C01A07234u64;
        
        let result = invoke_raw!(
            hash,
                name, 
                points
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn waypoint_recording_get_num_points_raw(
        name: , 
        points: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x5343532C01A07234u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x5343532C01A07234u64;

        invoke_raw_typed!(
            hash,
                name, 
                points
        )
    }
}

/// ## Parameters
*



pub fn assisted_movement_remove_route_safe(
        
        
            route: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x3548536485DD792Bu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x3548536485DD792Bu64;
        
        let result = invoke_raw!(
            hash,
                route
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn assisted_movement_remove_route_raw(
        route: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x3548536485DD792Bu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x3548536485DD792Bu64;

        invoke_raw_typed!(
            hash,
                route
        )
    }
}

/// ```
NativeDB Introduced: v1868
```



pub fn _0x9d252648778160df_safe(
        
        
            p0: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x9D252648778160DFu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x9D252648778160DFu64;
        
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
pub fn _0x9d252648778160df_raw(
        p0: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x9D252648778160DFu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x9D252648778160DFu64;

        invoke_raw_typed!(
            hash,
                p0
        )
    }
}

/// Gives the plane a mission (purpose or objective), the mission is passed onto the `iMissionIndex` parameter.

```c
enum eVehicleMission {
    MISSION_NONE = 0,
    MISSION_CRUISE, // 1
    MISSION_RAM, // 2
    MISSION_BLOCK, // 3
    MISSION_GOTO, // 4
    MISSION_STOP, // 5
    MISSION_ATTACK, // 6
    MISSION_FOLLOW, // 7
    MISSION_FLEE, // 8
    MISSION_CIRCLE, // 9
    MISSION_ESCORT_LEFT, // 10
    MISSION_ESCORT_RIGHT, // 11
    MISSION_ESCORT_REAR, // 12
    MISSION_ESCORT_FRONT, // 13
    MISSION_GOTO_RACING, // 14
    MISSION_FOLLOW_RECORDING, // 15
    MISSION_POLICE_BEHAVIOUR, // 16
    MISSION_PARK_PERPENDICULAR, // 17
    MISSION_PARK_PARALLEL, // 18
    MISSION_LAND, // 19
    MISSION_LAND_AND_WAIT, // 20
    MISSION_CRASH, // 21
    MISSION_PULL_OVER, // 22
    MISSION_PROTECT // 23
};
```



pub fn task_plane_mission_safe(
        
        
            ped: 
        , 
        
        
            vehicle: 
        , 
        
        
            targetVehicle: 
        , 
        
        
            targetPed: 
        , 
        
        
            fTargetCoordX: 
        , 
        
        
            fTargetCoordY: 
        , 
        
        
            fTargetCoordZ: 
        , 
        
        
            iMissionIndex: 
        , 
        
        
            fCruiseSpeed: 
        , 
        
        
            fTargetReachedDist: 
        , 
        
        
            fOrientation: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x23703CD154E83B88u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x23703CD154E83B88u64;
        
        let result = invoke_raw!(
            hash,
                ped, 
                vehicle, 
                targetVehicle, 
                targetPed, 
                fTargetCoordX, 
                fTargetCoordY, 
                fTargetCoordZ, 
                iMissionIndex, 
                fCruiseSpeed, 
                fTargetReachedDist, 
                fOrientation
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn task_plane_mission_raw(
        ped: , 
        vehicle: , 
        targetVehicle: , 
        targetPed: , 
        fTargetCoordX: , 
        fTargetCoordY: , 
        fTargetCoordZ: , 
        iMissionIndex: , 
        fCruiseSpeed: , 
        fTargetReachedDist: , 
        fOrientation: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x23703CD154E83B88u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x23703CD154E83B88u64;

        invoke_raw_typed!(
            hash,
                ped, 
                vehicle, 
                targetVehicle, 
                targetPed, 
                fTargetCoordX, 
                fTargetCoordY, 
                fTargetCoordZ, 
                iMissionIndex, 
                fCruiseSpeed, 
                fTargetReachedDist, 
                fOrientation
        )
    }
}

/// RESET_SCENARIO_TYPES_ENABLED native function



pub fn reset_scenario_types_enabled_safe(
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x0D40EE2A7F2B2D6Du32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x0D40EE2A7F2B2D6Du64;
        
        let result = invoke_raw!(
            hash,
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn reset_scenario_types_enabled_raw(
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x0D40EE2A7F2B2D6Du32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x0D40EE2A7F2B2D6Du64;

        invoke_raw_typed!(
            hash,
        )
    }
}

/// ```
Occurrences in the b617d scripts: pastebin.com/Tvg2PRHU  
```



pub fn set_scenario_group_enabled_safe(
        
        
            scenarioGroup: 
        , 
        
        
            p1: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x02C8E5B49848664Eu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x02C8E5B49848664Eu64;
        
        let result = invoke_raw!(
            hash,
                scenarioGroup, 
                p1
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn set_scenario_group_enabled_raw(
        scenarioGroup: , 
        p1: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x02C8E5B49848664Eu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x02C8E5B49848664Eu64;

        invoke_raw_typed!(
            hash,
                scenarioGroup, 
                p1
        )
    }
}

/// ## Parameters
*



pub fn waypoint_playback_override_speed_safe(
        
        
            p0: 
        , 
        
        
            p1: 
        , 
        
        
            p2: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x7D7D2B47FA788E85u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x7D7D2B47FA788E85u64;
        
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
pub fn waypoint_playback_override_speed_raw(
        p0: , 
        p1: , 
        p2: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x7D7D2B47FA788E85u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x7D7D2B47FA788E85u64;

        invoke_raw_typed!(
            hash,
                p0, 
                p1, 
                p2
        )
    }
}

/// ```
Occurrences in the b617d scripts:
"PROP_HUMAN_SEAT_CHAIR",
"WORLD_HUMAN_DRINKING",
"WORLD_HUMAN_HANG_OUT_STREET",
"WORLD_HUMAN_SMOKING",
"WORLD_MOUNTAIN_LION_WANDER",
"WORLD_HUMAN_DRINKING"
Sometimes used together with MISC::IS_STRING_NULL_OR_EMPTY in the scripts.
scenarioType could be the same as scenarioName, used in for example TASK::TASK_START_SCENARIO_AT_POSITION.
```



pub fn is_scenario_type_enabled_safe(
        
        
            scenarioType: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x3A815DB3EA088722u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x3A815DB3EA088722u64;
        
        let result = invoke_raw!(
            hash,
                scenarioType
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn is_scenario_type_enabled_raw(
        scenarioType: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x3A815DB3EA088722u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x3A815DB3EA088722u64;

        invoke_raw_typed!(
            hash,
                scenarioType
        )
    }
}

/// ```
Note: Look in decompiled scripts and the times that p1 and p2 aren't 0. They are filled with vars. If you look through out that script what other natives those vars are used in, you can tell p1 is a ped and p2 is a vehicle. Which most likely means if you want the mounted weapon to target a ped set targetVehicle to 0 or vice-versa.  
```

```
NativeDB Added Parameter 7: Any p6
NativeDB Added Parameter 8: Any p7
```



pub fn set_mounted_weapon_target_safe(
        
        
            shootingPed: 
        , 
        
        
            targetPed: 
        , 
        
        
            targetVehicle: 
        , 
        
        
            x: 
        , 
        
        
            y: 
        , 
        
        
            z: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xCCD892192C6D2BB9u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xCCD892192C6D2BB9u64;
        
        let result = invoke_raw!(
            hash,
                shootingPed, 
                targetPed, 
                targetVehicle, 
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
pub fn set_mounted_weapon_target_raw(
        shootingPed: , 
        targetPed: , 
        targetVehicle: , 
        x: , 
        y: , 
        z: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xCCD892192C6D2BB9u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xCCD892192C6D2BB9u64;

        invoke_raw_typed!(
            hash,
                shootingPed, 
                targetPed, 
                targetVehicle, 
                x, 
                y, 
                z
        )
    }
}

/// ## Parameters
*



pub fn vehicle_waypoint_playback_use_default_speed_safe(
        
        
            vehicle: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x5CEB25A7D2848963u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x5CEB25A7D2848963u64;
        
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
pub fn vehicle_waypoint_playback_use_default_speed_raw(
        vehicle: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x5CEB25A7D2848963u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x5CEB25A7D2848963u64;

        invoke_raw_typed!(
            hash,
                vehicle
        )
    }
}

/// ```
 Occurrences in the b617d scripts:
 "ARMY_GUARD",
 "ARMY_HELI",
 "BLIMP",
 "Cinema_Downtown",
 "Cinema_Morningwood",
 "Cinema_Textile",
 "City_Banks",
 "Countryside_Banks",
 "DEALERSHIP",
 "KORTZ_SECURITY",
 "LSA_Planes",
 "MP_POLICE",
 "Observatory_Bikers",
 "POLICE_POUND1",
 "POLICE_POUND2",
 "POLICE_POUND3",
 "POLICE_POUND4",
 "POLICE_POUND5",
 "Rampage1",
 "SANDY_PLANES",
 "SCRAP_SECURITY",
 "SEW_MACHINE",
 "SOLOMON_GATE"
Sometimes used with DOES_SCENARIO_GROUP_EXIST:
if (TASK::DOES_SCENARIO_GROUP_EXIST("Observatory_Bikers") &&   (!TASK::IS_SCENARIO_GROUP_ENABLED("Observatory_Bikers"))) {
else if (TASK::IS_SCENARIO_GROUP_ENABLED("BLIMP")) {
```



pub fn is_scenario_group_enabled_safe(
        
        
            scenarioGroup: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x367A09DED4E05B99u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x367A09DED4E05B99u64;
        
        let result = invoke_raw!(
            hash,
                scenarioGroup
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn is_scenario_group_enabled_raw(
        scenarioGroup: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x367A09DED4E05B99u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x367A09DED4E05B99u64;

        invoke_raw_typed!(
            hash,
                scenarioGroup
        )
    }
}

/// This native checks if a ped is on the ground, in pain from a (gunshot) wound.



pub fn is_ped_in_writhe_safe(
        
        
            ped: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xDEB6D52126E7D640u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xDEB6D52126E7D640u64;
        
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
pub fn is_ped_in_writhe_raw(
        ped: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xDEB6D52126E7D640u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xDEB6D52126E7D640u64;

        invoke_raw_typed!(
            hash,
                ped
        )
    }
}

/// Adds a new point to the current point route; a maximum of 8 points can be added.

Call [TASK_FLUSH_ROUTE](#_0x841142A1376E9006) before the first call to this. Call [TASK_FOLLOW_POINT_ROUTE](#_0x595583281858626E) to make the Ped go the route.



pub fn task_extend_route_safe(
        
        
            x: 
        , 
        
        
            y: 
        , 
        
        
            z: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x1E7889778264843Au32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x1E7889778264843Au64;
        
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
pub fn task_extend_route_raw(
        x: , 
        y: , 
        z: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x1E7889778264843Au32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x1E7889778264843Au64;

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



pub fn get_ped_waypoint_progress_safe(
        
        
            ped: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x2720AAA75001E094u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x2720AAA75001E094u64;
        
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
pub fn get_ped_waypoint_progress_raw(
        ped: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x2720AAA75001E094u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x2720AAA75001E094u64;

        invoke_raw_typed!(
            hash,
                ped
        )
    }
}

/// ```
CLEAR_*

NativeDB Introduced: v1290
```



pub fn _clear_vehicle_tasks_safe(
        
        
            vehicle: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xDBBC7A2432524127u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xDBBC7A2432524127u64;
        
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
pub fn _clear_vehicle_tasks_raw(
        vehicle: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xDBBC7A2432524127u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xDBBC7A2432524127u64;

        invoke_raw_typed!(
            hash,
                vehicle
        )
    }
}

/// ## Parameters
*



pub fn is_ped_sprinting_safe(
        
        
            ped: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x57E457CD2C0FC168u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x57E457CD2C0FC168u64;
        
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
pub fn is_ped_sprinting_raw(
        ped: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x57E457CD2C0FC168u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x57E457CD2C0FC168u64;

        invoke_raw_typed!(
            hash,
                ped
        )
    }
}

/// CREATE_PATROL_ROUTE native function



pub fn create_patrol_route_safe(
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xAF8A443CCC8018DCu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xAF8A443CCC8018DCu64;
        
        let result = invoke_raw!(
            hash,
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn create_patrol_route_raw(
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xAF8A443CCC8018DCu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xAF8A443CCC8018DCu64;

        invoke_raw_typed!(
            hash,
        )
    }
}

/// ## Parameters
*



pub fn task_seek_cover_from_pos_safe(
        
        
            ped: 
        , 
        
        
            x: 
        , 
        
        
            y: 
        , 
        
        
            z: 
        , 
        
        
            duration: 
        , 
        
        
            p5: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x75AC2B60386D89F2u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x75AC2B60386D89F2u64;
        
        let result = invoke_raw!(
            hash,
                ped, 
                x, 
                y, 
                z, 
                duration, 
                p5
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn task_seek_cover_from_pos_raw(
        ped: , 
        x: , 
        y: , 
        z: , 
        duration: , 
        p5: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x75AC2B60386D89F2u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x75AC2B60386D89F2u64;

        invoke_raw_typed!(
            hash,
                ped, 
                x, 
                y, 
                z, 
                duration, 
                p5
        )
    }
}

/// ```
//this part of the code is to determine at which entity the player is aiming, for example if you want to create a mod where you give orders to peds
Entity aimedentity;
Player player = PLAYER::PLAYER_ID();
PLAYER::_GET_AIMED_ENTITY(player, &aimedentity);
//bg is an array of peds
TASK::TASK_SHOOT_AT_ENTITY(bg[i], aimedentity, 5000, MISC::GET_HASH_KEY("FIRING_PATTERN_FULL_AUTO"));
in practical usage, getting the entity the player is aiming at and then task the peds to shoot at the entity, at a button press event would be better.
Firing Pattern Hash Information: https://pastebin.com/Px036isB
```



pub fn task_shoot_at_entity_safe(
        
        
            entity: 
        , 
        
        
            target: 
        , 
        
        
            duration: 
        , 
        
        
            firingPattern: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x08DA95E8298AE772u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x08DA95E8298AE772u64;
        
        let result = invoke_raw!(
            hash,
                entity, 
                target, 
                duration, 
                firingPattern
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn task_shoot_at_entity_raw(
        entity: , 
        target: , 
        duration: , 
        firingPattern: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x08DA95E8298AE772u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x08DA95E8298AE772u64;

        invoke_raw_typed!(
            hash,
                entity, 
                target, 
                duration, 
                firingPattern
        )
    }
}

/// Drive randomly with no destination set.



pub fn task_vehicle_drive_wander_safe(
        
        
            ped: 
        , 
        
        
            vehicle: 
        , 
        
        
            speed: 
        , 
        
        
            drivingStyle: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x480142959D337D00u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x480142959D337D00u64;
        
        let result = invoke_raw!(
            hash,
                ped, 
                vehicle, 
                speed, 
                drivingStyle
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn task_vehicle_drive_wander_raw(
        ped: , 
        vehicle: , 
        speed: , 
        drivingStyle: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x480142959D337D00u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x480142959D337D00u64;

        invoke_raw_typed!(
            hash,
                ped, 
                vehicle, 
                speed, 
                drivingStyle
        )
    }
}

/// ## Parameters
*



pub fn _0x1f351cf1c6475734_safe(
        
        
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
        let hash = 0x1F351CF1C6475734u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x1F351CF1C6475734u64;
        
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
pub fn _0x1f351cf1c6475734_raw(
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
        let hash = 0x1F351CF1C6475734u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x1F351CF1C6475734u64;

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

/// ```
info about driving modes: HTTP://gtaforums.com/topic/822314-guide-driving-styles/



pub fn task_vehicle_drive_to_coord_safe(
        
        
            ped: 
        , 
        
        
            vehicle: 
        , 
        
        
            x: 
        , 
        
        
            y: 
        , 
        
        
            z: 
        , 
        
        
            speed: 
        , 
        
        
            p6: 
        , 
        
        
            vehicleModel: 
        , 
        
        
            drivingMode: 
        , 
        
        
            stopRange: 
        , 
        
        
            p10: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xE2A2AA2F659D77A7u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xE2A2AA2F659D77A7u64;
        
        let result = invoke_raw!(
            hash,
                ped, 
                vehicle, 
                x, 
                y, 
                z, 
                speed, 
                p6, 
                vehicleModel, 
                drivingMode, 
                stopRange, 
                p10
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn task_vehicle_drive_to_coord_raw(
        ped: , 
        vehicle: , 
        x: , 
        y: , 
        z: , 
        speed: , 
        p6: , 
        vehicleModel: , 
        drivingMode: , 
        stopRange: , 
        p10: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xE2A2AA2F659D77A7u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xE2A2AA2F659D77A7u64;

        invoke_raw_typed!(
            hash,
                ped, 
                vehicle, 
                x, 
                y, 
                z, 
                speed, 
                p6, 
                vehicleModel, 
                drivingMode, 
                stopRange, 
                p10
        )
    }
}

/// ## Parameters
*



pub fn is_ped_running_arrest_task_safe(
        
        
            ped: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x3DC52677769B4AE0u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x3DC52677769B4AE0u64;
        
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
pub fn is_ped_running_arrest_task_raw(
        ped: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x3DC52677769B4AE0u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x3DC52677769B4AE0u64;

        invoke_raw_typed!(
            hash,
                ped
        )
    }
}

/// ## Parameters
*



pub fn task_perform_sequence_from_progress_safe(
        
        
            ped: 
        , 
        
        
            taskIndex: 
        , 
        
        
            progress1: 
        , 
        
        
            progress2: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x89221B16730234F0u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x89221B16730234F0u64;
        
        let result = invoke_raw!(
            hash,
                ped, 
                taskIndex, 
                progress1, 
                progress2
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn task_perform_sequence_from_progress_raw(
        ped: , 
        taskIndex: , 
        progress1: , 
        progress2: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x89221B16730234F0u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x89221B16730234F0u64;

        invoke_raw_typed!(
            hash,
                ped, 
                taskIndex, 
                progress1, 
                progress2
        )
    }
}

/// Makes the ped go on a point route.

```c
enum eFollowPointRouteMode {
	TICKET_SINGLE = 0,
	TICKET_RETURN = 1,
	TICKET_SEASON = 2,
	TICKET_LOOP = 3
}
```

This native is often times used with [`TASK_FLUSH_ROUTE`](#_0x841142A1376E9006) and [`TASK_EXTEND_ROUTE`](#_0x1E7889778264843A)



pub fn task_follow_point_route_safe(
        
        
            ped: 
        , 
        
        
            speed: 
        , 
        
        
            routeMode: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x595583281858626Eu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x595583281858626Eu64;
        
        let result = invoke_raw!(
            hash,
                ped, 
                speed, 
                routeMode
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn task_follow_point_route_raw(
        ped: , 
        speed: , 
        routeMode: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x595583281858626Eu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x595583281858626Eu64;

        invoke_raw_typed!(
            hash,
                ped, 
                speed, 
                routeMode
        )
    }
}

/// ## Parameters
*



pub fn get_is_waypoint_recording_loaded_safe(
        
        
            name: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xCB4E8BE8A0063C5Du32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xCB4E8BE8A0063C5Du64;
        
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
pub fn get_is_waypoint_recording_loaded_raw(
        name: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xCB4E8BE8A0063C5Du32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xCB4E8BE8A0063C5Du64;

        invoke_raw_typed!(
            hash,
                name
        )
    }
}

/// ## Parameters
*



pub fn task_aim_gun_scripted_safe(
        
        
            ped: 
        , 
        
        
            scriptTask: 
        , 
        
        
            bDisableBlockingClip: 
        , 
        
        
            bInstantBlendToAim: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x7A192BE16D373D00u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x7A192BE16D373D00u64;
        
        let result = invoke_raw!(
            hash,
                ped, 
                scriptTask, 
                bDisableBlockingClip, 
                bInstantBlendToAim
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn task_aim_gun_scripted_raw(
        ped: , 
        scriptTask: , 
        bDisableBlockingClip: , 
        bInstantBlendToAim: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x7A192BE16D373D00u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x7A192BE16D373D00u64;

        invoke_raw_typed!(
            hash,
                ped, 
                scriptTask, 
                bDisableBlockingClip, 
                bInstantBlendToAim
        )
    }
}

/// ## Parameters
*



pub fn task_vehicle_shoot_at_coord_safe(
        
        
            ped: 
        , 
        
        
            x: 
        , 
        
        
            y: 
        , 
        
        
            z: 
        , 
        
        
            p4: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x5190796ED39C9B6Du32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x5190796ED39C9B6Du64;
        
        let result = invoke_raw!(
            hash,
                ped, 
                x, 
                y, 
                z, 
                p4
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn task_vehicle_shoot_at_coord_raw(
        ped: , 
        x: , 
        y: , 
        z: , 
        p4: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x5190796ED39C9B6Du32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x5190796ED39C9B6Du64;

        invoke_raw_typed!(
            hash,
                ped, 
                x, 
                y, 
                z, 
                p4
        )
    }
}

/// ```
Checks if there is a cover point at position  
```



pub fn does_scripted_cover_point_exist_at_coords_safe(
        
        
            x: 
        , 
        
        
            y: 
        , 
        
        
            z: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xA98B8E3C088E5A31u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xA98B8E3C088E5A31u64;
        
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
pub fn does_scripted_cover_point_exist_at_coords_raw(
        x: , 
        y: , 
        z: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xA98B8E3C088E5A31u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xA98B8E3C088E5A31u64;

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



pub fn task_vehicle_aim_at_coord_safe(
        
        
            ped: 
        , 
        
        
            x: 
        , 
        
        
            y: 
        , 
        
        
            z: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x447C1E9EF844BC0Fu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x447C1E9EF844BC0Fu64;
        
        let result = invoke_raw!(
            hash,
                ped, 
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
pub fn task_vehicle_aim_at_coord_raw(
        ped: , 
        x: , 
        y: , 
        z: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x447C1E9EF844BC0Fu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x447C1E9EF844BC0Fu64;

        invoke_raw_typed!(
            hash,
                ped, 
                x, 
                y, 
                z
        )
    }
}

/// ```
x, y, z: offset in world coords from some entity.  
```



pub fn add_vehicle_subtask_attack_coord_safe(
        
        
            ped: 
        , 
        
        
            x: 
        , 
        
        
            y: 
        , 
        
        
            z: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x5CF0D8F9BBA0DD75u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x5CF0D8F9BBA0DD75u64;
        
        let result = invoke_raw!(
            hash,
                ped, 
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
pub fn add_vehicle_subtask_attack_coord_raw(
        ped: , 
        x: , 
        y: , 
        z: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x5CF0D8F9BBA0DD75u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x5CF0D8F9BBA0DD75u64;

        invoke_raw_typed!(
            hash,
                ped, 
                x, 
                y, 
                z
        )
    }
}

/// Instructs the ped to go to the entity with the given offset.

```c
enum eSeekEntityOffsetFlags {
    ESEEK_OFFSET_ORIENTATES_WITH_ENTITY = 1,
    ESEEK_KEEP_TO_PAVEMENTS = 2
};
```



pub fn task_goto_entity_offset_safe(
        
        
            ped: 
        , 
        
        
            entity: 
        , 
        
        
            duration: 
        , 
        
        
            seekRadius: 
        , 
        
        
            seekAngleDeg: 
        , 
        
        
            moveBlendRatio: 
        , 
        
        
            gotoEntityOffsetFlags: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xE39B4FF4FDEBDE27u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xE39B4FF4FDEBDE27u64;
        
        let result = invoke_raw!(
            hash,
                ped, 
                entity, 
                duration, 
                seekRadius, 
                seekAngleDeg, 
                moveBlendRatio, 
                gotoEntityOffsetFlags
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn task_goto_entity_offset_raw(
        ped: , 
        entity: , 
        duration: , 
        seekRadius: , 
        seekAngleDeg: , 
        moveBlendRatio: , 
        gotoEntityOffsetFlags: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xE39B4FF4FDEBDE27u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xE39B4FF4FDEBDE27u64;

        invoke_raw_typed!(
            hash,
                ped, 
                entity, 
                duration, 
                seekRadius, 
                seekAngleDeg, 
                moveBlendRatio, 
                gotoEntityOffsetFlags
        )
    }
}

/// Prevents a ped from playing ambient idle animations.



pub fn set_ped_can_play_ambient_idles_safe(
        
        
            ped: 
        , 
        
        
            bBlockIdleClips: 
        , 
        
        
            bRemoveIdleClipIfPlaying: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x8FD89A6240813FD0u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x8FD89A6240813FD0u64;
        
        let result = invoke_raw!(
            hash,
                ped, 
                bBlockIdleClips, 
                bRemoveIdleClipIfPlaying
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn set_ped_can_play_ambient_idles_raw(
        ped: , 
        bBlockIdleClips: , 
        bRemoveIdleClipIfPlaying: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x8FD89A6240813FD0u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x8FD89A6240813FD0u64;

        invoke_raw_typed!(
            hash,
                ped, 
                bBlockIdleClips, 
                bRemoveIdleClipIfPlaying
        )
    }
}

/// This tasks the ped to do nothing for the specified amount of miliseconds.
This is useful if you want to add a delay between tasks when using a sequence task.



pub fn task_pause_safe(
        
        
            ped: 
        , 
        
        
            ms: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xE73A266DB0CA9042u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xE73A266DB0CA9042u64;
        
        let result = invoke_raw!(
            hash,
                ped, 
                ms
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn task_pause_raw(
        ped: , 
        ms: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xE73A266DB0CA9042u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xE73A266DB0CA9042u64;

        invoke_raw_typed!(
            hash,
                ped, 
                ms
        )
    }
}

/// ```
Modes:  
0 - ignore heading  
1 - park forward  
2 - park backwards  
Depending on the angle of approach, the vehicle can park at the specified heading or at its exact opposite (-180) angle.  
Radius seems to define how close the vehicle has to be -after parking- to the position for this task considered completed. If the value is too small, the vehicle will try to park again until it's exactly where it should be. 20.0 Works well but lower values don't, like the radius is measured in centimeters or something.  
```



pub fn task_vehicle_park_safe(
        
        
            ped: 
        , 
        
        
            vehicle: 
        , 
        
        
            x: 
        , 
        
        
            y: 
        , 
        
        
            z: 
        , 
        
        
            heading: 
        , 
        
        
            mode: 
        , 
        
        
            radius: 
        , 
        
        
            keepEngineOn: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x0F3E34E968EA374Eu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x0F3E34E968EA374Eu64;
        
        let result = invoke_raw!(
            hash,
                ped, 
                vehicle, 
                x, 
                y, 
                z, 
                heading, 
                mode, 
                radius, 
                keepEngineOn
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn task_vehicle_park_raw(
        ped: , 
        vehicle: , 
        x: , 
        y: , 
        z: , 
        heading: , 
        mode: , 
        radius: , 
        keepEngineOn: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x0F3E34E968EA374Eu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x0F3E34E968EA374Eu64;

        invoke_raw_typed!(
            hash,
                ped, 
                vehicle, 
                x, 
                y, 
                z, 
                heading, 
                mode, 
                radius, 
                keepEngineOn
        )
    }
}

/// ## Parameters
*



pub fn play_entity_scripted_anim_safe(
        
        
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
        let hash = 0x77A1EEC547E7FCF1u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x77A1EEC547E7FCF1u64;
        
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
pub fn play_entity_scripted_anim_raw(
        p0: , 
        p1: , 
        p2: , 
        p3: , 
        p4: , 
        p5: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x77A1EEC547E7FCF1u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x77A1EEC547E7FCF1u64;

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
Makes ped walk around the area.  
set p1 to 10.0f and p2 to 10 if you want the ped to walk anywhere without a duration.  
```



pub fn task_wander_standard_safe(
        
        
            ped: 
        , 
        
        
            p1: 
        , 
        
        
            p2: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xBB9CE077274F6A1Bu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xBB9CE077274F6A1Bu64;
        
        let result = invoke_raw!(
            hash,
                ped, 
                p1, 
                p2
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn task_wander_standard_raw(
        ped: , 
        p1: , 
        p2: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xBB9CE077274F6A1Bu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xBB9CE077274F6A1Bu64;

        invoke_raw_typed!(
            hash,
                ped, 
                p1, 
                p2
        )
    }
}

/// ## Parameters
*



pub fn is_move_blend_ratio_running_safe(
        
        
            ped: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xD4D8636C0199A939u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xD4D8636C0199A939u64;
        
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
pub fn is_move_blend_ratio_running_raw(
        ped: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xD4D8636C0199A939u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xD4D8636C0199A939u64;

        invoke_raw_typed!(
            hash,
                ped
        )
    }
}

/// ## Parameters
*



pub fn task_vehicle_shoot_at_ped_safe(
        
        
            ped: 
        , 
        
        
            target: 
        , 
        
        
            p2: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x10AB107B887214D8u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x10AB107B887214D8u64;
        
        let result = invoke_raw!(
            hash,
                ped, 
                target, 
                p2
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn task_vehicle_shoot_at_ped_raw(
        ped: , 
        target: , 
        p2: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x10AB107B887214D8u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x10AB107B887214D8u64;

        invoke_raw_typed!(
            hash,
                ped, 
                target, 
                p2
        )
    }
}

/// ## Parameters
*



pub fn assisted_movement_override_load_distance_this_frame_safe(
        
        
            dist: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x13945951E16EF912u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x13945951E16EF912u64;
        
        let result = invoke_raw!(
            hash,
                dist
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn assisted_movement_override_load_distance_this_frame_raw(
        dist: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x13945951E16EF912u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x13945951E16EF912u64;

        invoke_raw_typed!(
            hash,
                dist
        )
    }
}

/// ```
Example:
TASK::TASK_MOVE_NETWORK_ADVANCED_BY_NAME(PLAYER::PLAYER_PED_ID(), "minigame_tattoo_michael_parts", 324.13f, 181.29f, 102.6f, 0.0f, 0.0f, 22.32f, 2, 0, false, 0, 0);
```



pub fn task_move_network_advanced_by_name_safe(
        
        
            ped: 
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
        , 
        
        
            p10: 
        , 
        
        
            animDict: 
        , 
        
        
            flags: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xD5B35BEA41919ACBu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xD5B35BEA41919ACBu64;
        
        let result = invoke_raw!(
            hash,
                ped, 
                p1, 
                p2, 
                p3, 
                p4, 
                p5, 
                p6, 
                p7, 
                p8, 
                p9, 
                p10, 
                animDict, 
                flags
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn task_move_network_advanced_by_name_raw(
        ped: , 
        p1: , 
        p2: , 
        p3: , 
        p4: , 
        p5: , 
        p6: , 
        p7: , 
        p8: , 
        p9: , 
        p10: , 
        animDict: , 
        flags: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xD5B35BEA41919ACBu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xD5B35BEA41919ACBu64;

        invoke_raw_typed!(
            hash,
                ped, 
                p1, 
                p2, 
                p3, 
                p4, 
                p5, 
                p6, 
                p7, 
                p8, 
                p9, 
                p10, 
                animDict, 
                flags
        )
    }
}

/// ## Parameters
*



pub fn waypoint_playback_start_aiming_at_ped_safe(
        
        
            p0: 
        , 
        
        
            p1: 
        , 
        
        
            p2: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x20E330937C399D29u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x20E330937C399D29u64;
        
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
pub fn waypoint_playback_start_aiming_at_ped_raw(
        p0: , 
        p1: , 
        p2: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x20E330937C399D29u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x20E330937C399D29u64;

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



pub fn assisted_movement_set_route_properties_safe(
        
        
            route: 
        , 
        
        
            props: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xD5002D78B7162E1Bu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xD5002D78B7162E1Bu64;
        
        let result = invoke_raw!(
            hash,
                route, 
                props
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn assisted_movement_set_route_properties_raw(
        route: , 
        props: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xD5002D78B7162E1Bu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xD5002D78B7162E1Bu64;

        invoke_raw_typed!(
            hash,
                route, 
                props
        )
    }
}

/// ## Parameters
*



pub fn set_anim_looped_safe(
        
        
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
        let hash = 0x70033C3CC29A1FF4u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x70033C3CC29A1FF4u64;
        
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
pub fn set_anim_looped_raw(
        p0: , 
        p1: , 
        p2: , 
        p3: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x70033C3CC29A1FF4u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x70033C3CC29A1FF4u64;

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
returned values:
0 to 7 = task that's currently in progress, 0 meaning the first one.
-1 no task sequence in progress.
```



pub fn get_sequence_progress_safe(
        
        
            ped: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x00A9010CFE1E3533u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x00A9010CFE1E3533u64;
        
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
pub fn get_sequence_progress_raw(
        ped: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x00A9010CFE1E3533u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x00A9010CFE1E3533u64;

        invoke_raw_typed!(
            hash,
                ped
        )
    }
}

/// Sometimes a path may not be able to be found. This could happen because there simply isn't any way to get there, or maybe a bunch of dynamic objects have blocked the way, 
or maybe the destination is too far away. In this case the ped will simply stand still.
To identify when this has happened, you can use GET_NAVMESH_ROUTE_RESULT. This will help you find situations where peds cannot get to their target.

```c
enum eNavScriptFlags {
    // Default flag
    ENAV_DEFAULT = 0,
    // Will ensure the ped continues to move whilst waiting for the path
    // to be found, and will not slow down at the end of their route.
    ENAV_NO_STOPPING = 1,
    // Performs a slide-to-coord at the end of the task. This requires that the
    // accompanying NAVDATA structure has the 'SlideToCoordHeading' member set correctly.
    ENAV_ADV_SLIDE_TO_COORD_AND_ACHIEVE_HEADING_AT_END = 2,
    // If the navmesh is not loaded in under the target position, then this will
    // cause the ped to get as close as is possible on whatever navmesh is loaded.
    // The navmesh must still be loaded at the path start.
    ENAV_GO_FAR_AS_POSSIBLE_IF_TARGET_NAVMESH_NOT_LOADED = 4,
    // Will allow navigation underwater - by default this is not allowed.
    ENAV_ALLOW_SWIMMING_UNDERWATER = 8,
    // Will only allow navigation on pavements. If the path starts or ends off
    // the pavement, the command will fail. Likewise if no pavement-only route
    // can be found even although the start and end are on pavement.
    ENAV_KEEP_TO_PAVEMENTS = 16,
    // Prevents the path from entering water at all.
    ENAV_NEVER_ENTER_WATER = 32,
    // Disables object-avoidance for this path. The ped may still make minor
    // steering adjustments to avoid objects, but will not pathfind around them.
    ENAV_DONT_AVOID_OBJECTS = 64,
    // Specifies that the navmesh route will only be able to traverse up slopes
    // which are under the angle specified, in the MaxSlopeNavigable member of the accompanying NAVDATA structure.
    ENAV_ADVANCED_USE_MAX_SLOPE_NAVIGABLE = 128,
    // Unused.
    ENAV_STOP_EXACTLY = 512,
    // The entity will look ahead in its path for a longer distance to make the
    // walk/run start go more in the right direction.
    ENAV_ACCURATE_WALKRUN_START = 1024,
    // Disables ped-avoidance for this path while we move.
    ENAV_DONT_AVOID_PEDS = 2048,
    // If target pos is inside the boundingbox of an object it will otherwise be pushed out.
    ENAV_DONT_ADJUST_TARGET_POSITION = 4096,
    // Turns off the default behaviour, which is to stop exactly at the target position.
    // Occasionally this can cause footsliding/skating problems.
    ENAV_SUPPRESS_EXACT_STOP = 8192,
    // Prevents the path-search from finding paths outside of this search distance.
    // This can be used to prevent peds from finding long undesired routes.
    ENAV_ADVANCED_USE_CLAMP_MAX_SEARCH_DISTANCE = 16384,
    // Pulls out the paths from edges at corners for a longer distance, to prevent peds walking into stuff.
    ENAV_PULL_FROM_EDGE_EXTRA = 32768
};
```



pub fn task_follow_nav_mesh_to_coord_safe(
        
        
            ped: 
        , 
        
        
            x: 
        , 
        
        
            y: 
        , 
        
        
            z: 
        , 
        
        
            moveBlendRatio: 
        , 
        
        
            time: 
        , 
        
        
            radius: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x15D3A79D4E44B913u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x15D3A79D4E44B913u64;
        
        let result = invoke_raw!(
            hash,
                ped, 
                x, 
                y, 
                z, 
                moveBlendRatio, 
                time, 
                radius
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn task_follow_nav_mesh_to_coord_raw(
        ped: , 
        x: , 
        y: , 
        z: , 
        moveBlendRatio: , 
        time: , 
        radius: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x15D3A79D4E44B913u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x15D3A79D4E44B913u64;

        invoke_raw_typed!(
            hash,
                ped, 
                x, 
                y, 
                z, 
                moveBlendRatio, 
                time, 
                radius
        )
    }
}

/// Related to [`_CLEAR_VEHICLE_TASKS`](#_0xDBBC7A2432524127) and requires more research (e.g., _CLEAR_VEHICLE_SECONDARY_TASKS).

```
CLEAR_*

NativeDB Introduced: v1290
```



pub fn _0x53ddc75bc3ac0a90_safe(
        
        
            vehicle: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x53DDC75BC3AC0A90u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x53DDC75BC3AC0A90u64;
        
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
pub fn _0x53ddc75bc3ac0a90_raw(
        vehicle: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x53DDC75BC3AC0A90u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x53DDC75BC3AC0A90u64;

        invoke_raw_typed!(
            hash,
                vehicle
        )
    }
}

/// ```
NativeDB Introduced: v1290
```



pub fn _task_plane_goto_precise_vtol_safe(
        
        
            ped: 
        , 
        
        
            vehicle: 
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
        let hash = 0xF7F9DCCA89E7505Bu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xF7F9DCCA89E7505Bu64;
        
        let result = invoke_raw!(
            hash,
                ped, 
                vehicle, 
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
pub fn _task_plane_goto_precise_vtol_raw(
        ped: , 
        vehicle: , 
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
        let hash = 0xF7F9DCCA89E7505Bu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xF7F9DCCA89E7505Bu64;

        invoke_raw_typed!(
            hash,
                ped, 
                vehicle, 
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



pub fn get_task_move_network_signal_bool_safe(
        
        
            ped: 
        , 
        
        
            signalName: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xA7FFBA498E4AAF67u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xA7FFBA498E4AAF67u64;
        
        let result = invoke_raw!(
            hash,
                ped, 
                signalName
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn get_task_move_network_signal_bool_raw(
        ped: , 
        signalName: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xA7FFBA498E4AAF67u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xA7FFBA498E4AAF67u64;

        invoke_raw_typed!(
            hash,
                ped, 
                signalName
        )
    }
}

/// ```
Makes the specified ped achieve the specified heading.  
pedHandle: The handle of the ped to assign the task to.  
heading: The desired heading.  
timeout: The time, in milliseconds, to allow the task to complete. If the task times out, it is cancelled, and the ped will stay at the heading it managed to reach in the time.  
```



pub fn task_achieve_heading_safe(
        
        
            ped: 
        , 
        
        
            heading: 
        , 
        
        
            timeout: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x93B93A37987F1F3Du32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x93B93A37987F1F3Du64;
        
        let result = invoke_raw!(
            hash,
                ped, 
                heading, 
                timeout
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn task_achieve_heading_raw(
        ped: , 
        heading: , 
        timeout: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x93B93A37987F1F3Du32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x93B93A37987F1F3Du64;

        invoke_raw_typed!(
            hash,
                ped, 
                heading, 
                timeout
        )
    }
}

/// ## Parameters
*



pub fn task_go_straight_to_coord_safe(
        
        
            ped: 
        , 
        
        
            x: 
        , 
        
        
            y: 
        , 
        
        
            z: 
        , 
        
        
            speed: 
        , 
        
        
            timeout: 
        , 
        
        
            targetHeading: 
        , 
        
        
            distanceToSlide: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xD76B57B44F1E6F8Bu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xD76B57B44F1E6F8Bu64;
        
        let result = invoke_raw!(
            hash,
                ped, 
                x, 
                y, 
                z, 
                speed, 
                timeout, 
                targetHeading, 
                distanceToSlide
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn task_go_straight_to_coord_raw(
        ped: , 
        x: , 
        y: , 
        z: , 
        speed: , 
        timeout: , 
        targetHeading: , 
        distanceToSlide: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xD76B57B44F1E6F8Bu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xD76B57B44F1E6F8Bu64;

        invoke_raw_typed!(
            hash,
                ped, 
                x, 
                y, 
                z, 
                speed, 
                timeout, 
                targetHeading, 
                distanceToSlide
        )
    }
}

/// All parameters except ped, vehicle, vehicleTarget and speed are optional; with `missionType` being only those that require a target entity.

If you don't want to use a parameter; pass `0` for int parameters, and `-1.0f` for the remaining float parameters.

```c
enum eVehicleMissionType
{
  None = 0,
  Cruise = 1,
  Ram = 2,
  Block = 3,
  GoTo = 4,
  Stop = 5,
  Attack = 6,
  Follow = 7,
  Flee = 8,
  Circle = 9,
  Escort = 12,
  GoToRacing = 14,
  FollowRecording = 15,
  PoliceBehaviour = 16,
  Land = 19,
  LandAndWait = 20,
  Crash = 21,
  PullOver = 22,
  HeliProtect = 23
}
```



pub fn task_vehicle_mission_safe(
        
        
            ped: 
        , 
        
        
            vehicle: 
        , 
        
        
            vehicleTarget: 
        , 
        
        
            missionType: 
        , 
        
        
            speed: 
        , 
        
        
            drivingStyle: 
        , 
        
        
            radius: 
        , 
        
        
            straightLineDist: 
        , 
        
        
            DriveAgainstTraffic: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x659427E0EF36BCDEu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x659427E0EF36BCDEu64;
        
        let result = invoke_raw!(
            hash,
                ped, 
                vehicle, 
                vehicleTarget, 
                missionType, 
                speed, 
                drivingStyle, 
                radius, 
                straightLineDist, 
                DriveAgainstTraffic
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn task_vehicle_mission_raw(
        ped: , 
        vehicle: , 
        vehicleTarget: , 
        missionType: , 
        speed: , 
        drivingStyle: , 
        radius: , 
        straightLineDist: , 
        DriveAgainstTraffic: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x659427E0EF36BCDEu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x659427E0EF36BCDEu64;

        invoke_raw_typed!(
            hash,
                ped, 
                vehicle, 
                vehicleTarget, 
                missionType, 
                speed, 
                drivingStyle, 
                radius, 
                straightLineDist, 
                DriveAgainstTraffic
        )
    }
}

/// ## Parameters
*



pub fn is_move_blend_ratio_sprinting_safe(
        
        
            ped: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x24A2AD74FA9814E2u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x24A2AD74FA9814E2u64;
        
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
pub fn is_move_blend_ratio_sprinting_raw(
        ped: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x24A2AD74FA9814E2u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x24A2AD74FA9814E2u64;

        invoke_raw_typed!(
            hash,
                ped
        )
    }
}

/// ## Parameters
*



pub fn task_aim_gun_at_coord_safe(
        
        
            ped: 
        , 
        
        
            x: 
        , 
        
        
            y: 
        , 
        
        
            z: 
        , 
        
        
            time: 
        , 
        
        
            bInstantBlendToAim: 
        , 
        
        
            bPlayAimIntro: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x6671F3EEC681BDA1u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x6671F3EEC681BDA1u64;
        
        let result = invoke_raw!(
            hash,
                ped, 
                x, 
                y, 
                z, 
                time, 
                bInstantBlendToAim, 
                bPlayAimIntro
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn task_aim_gun_at_coord_raw(
        ped: , 
        x: , 
        y: , 
        z: , 
        time: , 
        bInstantBlendToAim: , 
        bPlayAimIntro: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x6671F3EEC681BDA1u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x6671F3EEC681BDA1u64;

        invoke_raw_typed!(
            hash,
                ped, 
                x, 
                y, 
                z, 
                time, 
                bInstantBlendToAim, 
                bPlayAimIntro
        )
    }
}

/// [Animations list](https://alexguirre.github.io/animations-list/)



pub fn stop_anim_task_safe(
        
        
            ped: 
        , 
        
        
            animDictionary: 
        , 
        
        
            animationName: 
        , 
        
        
            animExitSpeed: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x97FF36A1D40EA00Au32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x97FF36A1D40EA00Au64;
        
        let result = invoke_raw!(
            hash,
                ped, 
                animDictionary, 
                animationName, 
                animExitSpeed
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn stop_anim_task_raw(
        ped: , 
        animDictionary: , 
        animationName: , 
        animExitSpeed: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x97FF36A1D40EA00Au32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x97FF36A1D40EA00Au64;

        invoke_raw_typed!(
            hash,
                ped, 
                animDictionary, 
                animationName, 
                animExitSpeed
        )
    }
}

/// ## Parameters
*



pub fn waypoint_playback_pause_safe(
        
        
            p0: 
        , 
        
        
            p1: 
        , 
        
        
            p2: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x0F342546AA06FED5u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x0F342546AA06FED5u64;
        
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
pub fn waypoint_playback_pause_raw(
        p0: , 
        p1: , 
        p2: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x0F342546AA06FED5u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x0F342546AA06FED5u64;

        invoke_raw_typed!(
            hash,
                p0, 
                p1, 
                p2
        )
    }
}

/// ```
param3: duration in ms, use -1 to look forever  
param4: using 2048 is fine  
param5: using 3 is fine  
```



pub fn task_look_at_entity_safe(
        
        
            ped: 
        , 
        
        
            lookAt: 
        , 
        
        
            duration: 
        , 
        
        
            unknown1: 
        , 
        
        
            unknown2: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x69F4BE8C8CC4796Cu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x69F4BE8C8CC4796Cu64;
        
        let result = invoke_raw!(
            hash,
                ped, 
                lookAt, 
                duration, 
                unknown1, 
                unknown2
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn task_look_at_entity_raw(
        ped: , 
        lookAt: , 
        duration: , 
        unknown1: , 
        unknown2: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x69F4BE8C8CC4796Cu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x69F4BE8C8CC4796Cu64;

        invoke_raw_typed!(
            hash,
                ped, 
                lookAt, 
                duration, 
                unknown1, 
                unknown2
        )
    }
}

/// ```
From the b617d scripts:
TASK::DELETE_PATROL_ROUTE("miss_merc0");
TASK::DELETE_PATROL_ROUTE("miss_merc1");
TASK::DELETE_PATROL_ROUTE("miss_merc2");
TASK::DELETE_PATROL_ROUTE("miss_dock");
```



pub fn delete_patrol_route_safe(
        
        
            patrolRoute: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x7767DD9D65E91319u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x7767DD9D65E91319u64;
        
        let result = invoke_raw!(
            hash,
                patrolRoute
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn delete_patrol_route_raw(
        patrolRoute: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x7767DD9D65E91319u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x7767DD9D65E91319u64;

        invoke_raw_typed!(
            hash,
                patrolRoute
        )
    }
}

/// ```
Despite its name, it only attacks ONE hated target. The one closest hated target.  
p2 seems to be always 0  
```



pub fn task_combat_hated_targets_around_ped_safe(
        
        
            ped: 
        , 
        
        
            radius: 
        , 
        
        
            p2: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x7BF835BB9E2698C8u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x7BF835BB9E2698C8u64;
        
        let result = invoke_raw!(
            hash,
                ped, 
                radius, 
                p2
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn task_combat_hated_targets_around_ped_raw(
        ped: , 
        radius: , 
        p2: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x7BF835BB9E2698C8u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x7BF835BB9E2698C8u64;

        invoke_raw_typed!(
            hash,
                ped, 
                radius, 
                p2
        )
    }
}

/// ```
p0 - Guessing PedID  
p1, p2, p3 - XYZ?  
p4 - ???  
p5 - Maybe the size of sphere from XYZ?  
p6 - ???  
p7, p8, p9 - XYZ again?  
p10 - Maybe the size of sphere from second XYZ?  
```



pub fn task_guard_sphere_defensive_area_safe(
        
        
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
        , 
        
        
            p10: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xC946FE14BE0EB5E2u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xC946FE14BE0EB5E2u64;
        
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
                p9, 
                p10
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn task_guard_sphere_defensive_area_raw(
        p0: , 
        p1: , 
        p2: , 
        p3: , 
        p4: , 
        p5: , 
        p6: , 
        p7: , 
        p8: , 
        p9: , 
        p10: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xC946FE14BE0EB5E2u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xC946FE14BE0EB5E2u64;

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
                p9, 
                p10
        )
    }
}

/// ## Parameters
*



pub fn task_exit_cover_safe(
        
        
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
        let hash = 0x79B258E397854D29u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x79B258E397854D29u64;
        
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
pub fn task_exit_cover_raw(
        p0: , 
        p1: , 
        p2: , 
        p3: , 
        p4: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x79B258E397854D29u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x79B258E397854D29u64;

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



pub fn vehicle_waypoint_playback_resume_safe(
        
        
            vehicle: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xDC04FCAA7839D492u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xDC04FCAA7839D492u64;
        
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
pub fn vehicle_waypoint_playback_resume_raw(
        vehicle: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xDC04FCAA7839D492u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xDC04FCAA7839D492u64;

        invoke_raw_typed!(
            hash,
                vehicle
        )
    }
}

/// ```
Only appears twice in the scripts.
TASK::TASK_RAPPEL_FROM_HELI(PLAYER::PLAYER_PED_ID(), 0x41200000);
TASK::TASK_RAPPEL_FROM_HELI(a_0, 0x41200000);
```



pub fn task_rappel_from_heli_safe(
        
        
            ped: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x09693B0312F91649u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x09693B0312F91649u64;
        
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
pub fn task_rappel_from_heli_raw(
        ped: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x09693B0312F91649u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x09693B0312F91649u64;

        invoke_raw_typed!(
            hash,
                ped
        )
    }
}

/// ## Parameters
*



pub fn set_ped_path_prefer_to_avoid_water_safe(
        
        
            ped: 
        , 
        
        
            avoidWater: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x38FE1EC73743793Cu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x38FE1EC73743793Cu64;
        
        let result = invoke_raw!(
            hash,
                ped, 
                avoidWater
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn set_ped_path_prefer_to_avoid_water_raw(
        ped: , 
        avoidWater: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x38FE1EC73743793Cu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x38FE1EC73743793Cu64;

        invoke_raw_typed!(
            hash,
                ped, 
                avoidWater
        )
    }
}

/// Gets the status of a spesifed script-assigned task on the given ped. The return value is always an int between 0-7.  

You can set taskHash to `SCRIPT_TASK_ANY` to check if any task is active, it will return 1 for active, 3 for no active. 
`SCRIPT_TASK_INVALID` can be similarly used, it returns 7 if there are any active task, and 3 if there are no active tasks.

taskHash list: https://alloc8or.re/gta5/doc/enums/eScriptTaskHash.txt  

Returns:  
```
0 = WAITING_TO_START_TASK
1 = PERFORMING_TASK
2 = DORMANT_TASK
3 = VACANT_STAGE
7 = TASK_FINISHED_OR_NOT_FOUND
```



pub fn get_script_task_status_safe(
        
        
            ped: 
        , 
        
        
            taskHash: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x77F1BEB8863288D5u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x77F1BEB8863288D5u64;
        
        let result = invoke_raw!(
            hash,
                ped, 
                taskHash
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn get_script_task_status_raw(
        ped: , 
        taskHash: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x77F1BEB8863288D5u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x77F1BEB8863288D5u64;

        invoke_raw_typed!(
            hash,
                ped, 
                taskHash
        )
    }
}

/// ## Parameters
*



pub fn set_task_move_network_signal_bool_safe(
        
        
            ped: 
        , 
        
        
            signalName: 
        , 
        
        
            value: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xB0A6CFD2C69C1088u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xB0A6CFD2C69C1088u64;
        
        let result = invoke_raw!(
            hash,
                ped, 
                signalName, 
                value
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn set_task_move_network_signal_bool_raw(
        ped: , 
        signalName: , 
        value: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xB0A6CFD2C69C1088u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xB0A6CFD2C69C1088u64;

        invoke_raw_typed!(
            hash,
                ped, 
                signalName, 
                value
        )
    }
}

/// ## Parameters
*



pub fn set_ped_path_can_use_climbovers_safe(
        
        
            ped: 
        , 
        
        
            Toggle: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x8E06A6FE76C9EFF4u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x8E06A6FE76C9EFF4u64;
        
        let result = invoke_raw!(
            hash,
                ped, 
                Toggle
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn set_ped_path_can_use_climbovers_raw(
        ped: , 
        Toggle: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x8E06A6FE76C9EFF4u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x8E06A6FE76C9EFF4u64;

        invoke_raw_typed!(
            hash,
                ped, 
                Toggle
        )
    }
}

/// Instructs the ped to go to the entity with the given offset.



pub fn task_goto_entity_offset_xy_safe(
        
        
            ped: 
        , 
        
        
            entity: 
        , 
        
        
            duration: 
        , 
        
        
            targetRadius: 
        , 
        
        
            offsetX: 
        , 
        
        
            offsetY: 
        , 
        
        
            moveBlendRatio: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x338E7EF52B6095A9u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x338E7EF52B6095A9u64;
        
        let result = invoke_raw!(
            hash,
                ped, 
                entity, 
                duration, 
                targetRadius, 
                offsetX, 
                offsetY, 
                moveBlendRatio
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn task_goto_entity_offset_xy_raw(
        ped: , 
        entity: , 
        duration: , 
        targetRadius: , 
        offsetX: , 
        offsetY: , 
        moveBlendRatio: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x338E7EF52B6095A9u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x338E7EF52B6095A9u64;

        invoke_raw_typed!(
            hash,
                ped, 
                entity, 
                duration, 
                targetRadius, 
                offsetX, 
                offsetY, 
                moveBlendRatio
        )
    }
}

/// Makes a ped wander/patrol around the specified area.

The ped will continue to wander after getting distracted, but only if this additional task is temporary, ie. killing a target, after killing the target it will continue to wander around.

Use `GetIsTaskActive(ped, 222)` to check if the ped is still wandering the area.



pub fn task_wander_in_area_safe(
        
        
            ped: 
        , 
        
        
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
        let hash = 0xE054346CA3A0F315u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xE054346CA3A0F315u64;
        
        let result = invoke_raw!(
            hash,
                ped, 
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
pub fn task_wander_in_area_raw(
        ped: , 
        x: , 
        y: , 
        z: , 
        radius: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xE054346CA3A0F315u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xE054346CA3A0F315u64;

        invoke_raw_typed!(
            hash,
                ped, 
                x, 
                y, 
                z, 
                radius
        )
    }
}

/// ```
* Flag 1: Aggressive ramming of suspect
* Flag 2: Ram attempts
* Flag 8: Medium-aggressive boxing tactic with a bit of PIT
* Flag 16: Ramming, seems to be slightly less aggressive than 1-2.
* Flag 32: Stay back from suspect, no tactical contact. Convoy-like.
```



pub fn set_task_vehicle_chase_behavior_flag_safe(
        
        
            ped: 
        , 
        
        
            flag: 
        , 
        
        
            set: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xCC665AAC360D31E7u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xCC665AAC360D31E7u64;
        
        let result = invoke_raw!(
            hash,
                ped, 
                flag, 
                set
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn set_task_vehicle_chase_behavior_flag_raw(
        ped: , 
        flag: , 
        set: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xCC665AAC360D31E7u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xCC665AAC360D31E7u64;

        invoke_raw_typed!(
            hash,
                ped, 
                flag, 
                set
        )
    }
}

/// ## Parameters
*



pub fn waypoint_recording_get_speed_at_point_safe(
        
        
            name: 
        , 
        
        
            point: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x005622AEBC33ACA9u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x005622AEBC33ACA9u64;
        
        let result = invoke_raw!(
            hash,
                name, 
                point
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn waypoint_recording_get_speed_at_point_raw(
        name: , 
        point: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x005622AEBC33ACA9u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x005622AEBC33ACA9u64;

        invoke_raw_typed!(
            hash,
                name, 
                point
        )
    }
}

/// ## Parameters
*



pub fn is_task_move_network_active_safe(
        
        
            ped: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x921CE12C489C4C41u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x921CE12C489C4C41u64;
        
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
pub fn is_task_move_network_active_raw(
        ped: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x921CE12C489C4C41u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x921CE12C489C4C41u64;

        invoke_raw_typed!(
            hash,
                ped
        )
    }
}

/// ## Parameters
*



pub fn clear_driveby_task_underneath_driving_task_safe(
        
        
            ped: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xC35B5CDB2824CF69u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xC35B5CDB2824CF69u64;
        
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
pub fn clear_driveby_task_underneath_driving_task_raw(
        ped: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xC35B5CDB2824CF69u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xC35B5CDB2824CF69u64;

        invoke_raw_typed!(
            hash,
                ped
        )
    }
}

/// ## Parameters
*



pub fn waypoint_playback_use_default_speed_safe(
        
        
            p0: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x6599D834B12D0800u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x6599D834B12D0800u64;
        
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
pub fn waypoint_playback_use_default_speed_raw(
        p0: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x6599D834B12D0800u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x6599D834B12D0800u64;

        invoke_raw_typed!(
            hash,
                p0
        )
    }
}

/// ## Parameters
*



pub fn set_high_fall_task_safe(
        
        
            ped: 
        , 
        
        
            duration: 
        , 
        
        
            p2: 
        , 
        
        
            p3: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x8C825BDC7741D37Cu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x8C825BDC7741D37Cu64;
        
        let result = invoke_raw!(
            hash,
                ped, 
                duration, 
                p2, 
                p3
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn set_high_fall_task_raw(
        ped: , 
        duration: , 
        p2: , 
        p3: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x8C825BDC7741D37Cu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x8C825BDC7741D37Cu64;

        invoke_raw_typed!(
            hash,
                ped, 
                duration, 
                p2, 
                p3
        )
    }
}

/// ## Parameters
*



pub fn vehicle_waypoint_playback_override_speed_safe(
        
        
            vehicle: 
        , 
        
        
            speed: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x121F0593E0A431D7u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x121F0593E0A431D7u64;
        
        let result = invoke_raw!(
            hash,
                vehicle, 
                speed
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn vehicle_waypoint_playback_override_speed_raw(
        vehicle: , 
        speed: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x121F0593E0A431D7u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x121F0593E0A431D7u64;

        invoke_raw_typed!(
            hash,
                vehicle, 
                speed
        )
    }
}

/// ## Parameters
*



pub fn is_ped_running_safe(
        
        
            ped: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xC5286FFC176F28A2u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xC5286FFC176F28A2u64;
        
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
pub fn is_ped_running_raw(
        ped: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xC5286FFC176F28A2u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xC5286FFC176F28A2u64;

        invoke_raw_typed!(
            hash,
                ped
        )
    }
}

/// ## Parameters
*



pub fn uncuff_ped_safe(
        
        
            ped: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x67406F2C8F87FC4Fu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x67406F2C8F87FC4Fu64;
        
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
pub fn uncuff_ped_raw(
        ped: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x67406F2C8F87FC4Fu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x67406F2C8F87FC4Fu64;

        invoke_raw_typed!(
            hash,
                ped
        )
    }
}

/// ```
NativeDB Introduced: v3407
```



pub fn clear_ped_script_task_if_running_threat_response_non_temp_task_safe(
        
        
            ped: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xF6DC48E56BE1243Au32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xF6DC48E56BE1243Au64;
        
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
pub fn clear_ped_script_task_if_running_threat_response_non_temp_task_raw(
        ped: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xF6DC48E56BE1243Au32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xF6DC48E56BE1243Au64;

        invoke_raw_typed!(
            hash,
                ped
        )
    }
}

/// [Animations list](https://alexguirre.github.io/animations-list/)  

```c
enum eScriptedAnimFlags
{
    AF_LOOPING = 1,
    AF_HOLD_LAST_FRAME = 2,
    AF_REPOSITION_WHEN_FINISHED = 4,
    AF_NOT_INTERRUPTABLE = 8,
    AF_UPPERBODY = 16,
    AF_SECONDARY = 32,
    AF_REORIENT_WHEN_FINISHED = 64,
    AF_ABORT_ON_PED_MOVEMENT = 128,
    AF_ADDITIVE = 256,
    AF_TURN_OFF_COLLISION = 512,
    AF_OVERRIDE_PHYSICS = 1024,
    AF_IGNORE_GRAVITY = 2048,
    AF_EXTRACT_INITIAL_OFFSET = 4096,
    AF_EXIT_AFTER_INTERRUPTED = 8192,
    AF_TAG_SYNC_IN = 16384,
    AF_TAG_SYNC_OUT = 32768,
    AF_TAG_SYNC_CONTINUOUS = 65536,
    AF_FORCE_START = 131072,
    AF_USE_KINEMATIC_PHYSICS = 262144,
    AF_USE_MOVER_EXTRACTION = 524288,
    AF_HIDE_WEAPON = 1048576,
    AF_ENDS_IN_DEAD_POSE = 2097152,
    AF_ACTIVATE_RAGDOLL_ON_COLLISION = 4194304,
    AF_DONT_EXIT_ON_DEATH = 8388608,
    AF_ABORT_ON_WEAPON_DAMAGE = 16777216,
    AF_DISABLE_FORCED_PHYSICS_UPDATE = 33554432,
    AF_PROCESS_ATTACHMENTS_ON_START = 67108864,
    AF_EXPAND_PED_CAPSULE_FROM_SKELETON = 134217728,
    AF_USE_ALTERNATIVE_FP_ANIM = 268435456,
    AF_BLENDOUT_WRT_LAST_FRAME = 536870912,
    AF_USE_FULL_BLENDING = 1073741824
}
```



pub fn task_play_anim_safe(
        
        
            ped: 
        , 
        
        
            animDictionary: 
        , 
        
        
            animationName: 
        , 
        
        
            blendInSpeed: 
        , 
        
        
            blendOutSpeed: 
        , 
        
        
            duration: 
        , 
        
        
            flag: 
        , 
        
        
            playbackRate: 
        , 
        
        
            lockX: 
        , 
        
        
            lockY: 
        , 
        
        
            lockZ: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xEA47FE3719165B94u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xEA47FE3719165B94u64;
        
        let result = invoke_raw!(
            hash,
                ped, 
                animDictionary, 
                animationName, 
                blendInSpeed, 
                blendOutSpeed, 
                duration, 
                flag, 
                playbackRate, 
                lockX, 
                lockY, 
                lockZ
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn task_play_anim_raw(
        ped: , 
        animDictionary: , 
        animationName: , 
        blendInSpeed: , 
        blendOutSpeed: , 
        duration: , 
        flag: , 
        playbackRate: , 
        lockX: , 
        lockY: , 
        lockZ: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xEA47FE3719165B94u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xEA47FE3719165B94u64;

        invoke_raw_typed!(
            hash,
                ped, 
                animDictionary, 
                animationName, 
                blendInSpeed, 
                blendOutSpeed, 
                duration, 
                flag, 
                playbackRate, 
                lockX, 
                lockY, 
                lockZ
        )
    }
}

/// ```
Definition is wrong. This has 4 parameters (Not sure when they were added. v350 has 2, v678 has 4).  
v350: Ped ped, bool unused  
v678: Ped ped, bool unused, bool flag1, bool flag2  
flag1 = super jump, flag2 = do nothing if flag1 is false and doubles super jump height if flag1 is true.  
```

```
NativeDB Added Parameter 3: Any p2
NativeDB Added Parameter 4: Any p3
```



pub fn task_jump_safe(
        
        
            ped: 
        , 
        
        
            unused: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x0AE4086104E067B1u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x0AE4086104E067B1u64;
        
        let result = invoke_raw!(
            hash,
                ped, 
                unused
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn task_jump_raw(
        ped: , 
        unused: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x0AE4086104E067B1u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x0AE4086104E067B1u64;

        invoke_raw_typed!(
            hash,
                ped, 
                unused
        )
    }
}

/// All parameters except ped and boat are optional, with `pedTarget`, `vehicleTarget`, `x`, `y`, `z` being dependent on `missionType` (ie. Attack/Flee mission types require a target ped/vehicle, whereas GoTo mission types require either `x`, `y`, `z` or a target ped/vehicle).

If you don't want to use a parameter; pass `0.0f` for `x`, `y` and `z`, `0` for `pedTarget`, `vehicleTarget` and other int parameters, and `-1.0f` for the remaining float parameters.

```c
enum eBoatMissionFlags
{
  None = 0,
  StopAtEnd = 1,
  StopAtShore = 2,
  AvoidShore = 4,
  PreferForward = 8,
  NeverStop = 16,
  NeverNavMesh = 32,
  NeverRoute = 64,
  ForceBeached = 128,
  UseWanderRoute = 256,
  UseFleeRoute = 512,
  NeverPause = 1024,
  // StopAtEnd | StopAtShore | AvoidShore
  DefaultSettings = 7,
  // StopAtEnd | StopAtShore | AvoidShore | PreferForward | NeverNavMesh | NeverRoute
  OpenOceanSettings = 111,
  // StopAtEnd | StopAtShore | AvoidShore | PreferForward | NeverNavMesh | NeverPause
  BoatTaxiSettings = 1071,
}
```



pub fn task_boat_mission_safe(
        
        
            ped: 
        , 
        
        
            boat: 
        , 
        
        
            vehicleTarget: 
        , 
        
        
            pedTarget: 
        , 
        
        
            x: 
        , 
        
        
            y: 
        , 
        
        
            z: 
        , 
        
        
            missionType: 
        , 
        
        
            speed: 
        , 
        
        
            drivingStyle: 
        , 
        
        
            radius: 
        , 
        
        
            missionFlags: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x15C86013127CE63Fu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x15C86013127CE63Fu64;
        
        let result = invoke_raw!(
            hash,
                ped, 
                boat, 
                vehicleTarget, 
                pedTarget, 
                x, 
                y, 
                z, 
                missionType, 
                speed, 
                drivingStyle, 
                radius, 
                missionFlags
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn task_boat_mission_raw(
        ped: , 
        boat: , 
        vehicleTarget: , 
        pedTarget: , 
        x: , 
        y: , 
        z: , 
        missionType: , 
        speed: , 
        drivingStyle: , 
        radius: , 
        missionFlags: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x15C86013127CE63Fu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x15C86013127CE63Fu64;

        invoke_raw_typed!(
            hash,
                ped, 
                boat, 
                vehicleTarget, 
                pedTarget, 
                x, 
                y, 
                z, 
                missionType, 
                speed, 
                drivingStyle, 
                radius, 
                missionFlags
        )
    }
}

/// ## Parameters
*



pub fn _task_agitated_action_safe(
        
        
            ped: 
        , 
        
        
            ped2: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x19D1B791CB3670FEu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x19D1B791CB3670FEu64;
        
        let result = invoke_raw!(
            hash,
                ped, 
                ped2
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn _task_agitated_action_raw(
        ped: , 
        ped2: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x19D1B791CB3670FEu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x19D1B791CB3670FEu64;

        invoke_raw_typed!(
            hash,
                ped, 
                ped2
        )
    }
}

/// ## Parameters
*



pub fn request_task_move_network_state_transition_safe(
        
        
            ped: 
        , 
        
        
            name: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xD01015C7316AE176u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xD01015C7316AE176u64;
        
        let result = invoke_raw!(
            hash,
                ped, 
                name
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn request_task_move_network_state_transition_raw(
        ped: , 
        name: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xD01015C7316AE176u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xD01015C7316AE176u64;

        invoke_raw_typed!(
            hash,
                ped, 
                name
        )
    }
}

/// ```
Example:
TASK::TASK_DRIVE_BY(l_467[1/*22*/], PLAYER::PLAYER_PED_ID(), 0, 0.0, 0.0, 2.0, 300.0, 100, 0, ${firing_pattern_burst_fire_driveby});
Needs working example. Doesn't seem to do anything.
I marked p2 as targetVehicle as all these shooting related tasks seem to have that in common.
I marked p6 as distanceToShoot as if you think of GTA's Logic with the native SET_VEHICLE_SHOOT natives, it won't shoot till it gets within a certain distance of the target.
I marked p7 as pedAccuracy as it seems it's mostly 100 (Completely Accurate), 75, 90, etc. Although this could be the ammo count within the gun, but I highly doubt it. I will change this comment once I find out if it's ammo count or not.
```



pub fn task_drive_by_safe(
        
        
            driverPed: 
        , 
        
        
            targetPed: 
        , 
        
        
            targetVehicle: 
        , 
        
        
            targetX: 
        , 
        
        
            targetY: 
        , 
        
        
            targetZ: 
        , 
        
        
            distanceToShoot: 
        , 
        
        
            pedAccuracy: 
        , 
        
        
            p8: 
        , 
        
        
            firingPattern: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x2F8AF0E82773A171u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x2F8AF0E82773A171u64;
        
        let result = invoke_raw!(
            hash,
                driverPed, 
                targetPed, 
                targetVehicle, 
                targetX, 
                targetY, 
                targetZ, 
                distanceToShoot, 
                pedAccuracy, 
                p8, 
                firingPattern
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn task_drive_by_raw(
        driverPed: , 
        targetPed: , 
        targetVehicle: , 
        targetX: , 
        targetY: , 
        targetZ: , 
        distanceToShoot: , 
        pedAccuracy: , 
        p8: , 
        firingPattern: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x2F8AF0E82773A171u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x2F8AF0E82773A171u64;

        invoke_raw_typed!(
            hash,
                driverPed, 
                targetPed, 
                targetVehicle, 
                targetX, 
                targetY, 
                targetZ, 
                distanceToShoot, 
                pedAccuracy, 
                p8, 
                firingPattern
        )
    }
}

/// ## Parameters
*



pub fn set_parachute_task_target_safe(
        
        
            ped: 
        , 
        
        
            x: 
        , 
        
        
            y: 
        , 
        
        
            z: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xC313379AF0FCEDA7u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xC313379AF0FCEDA7u64;
        
        let result = invoke_raw!(
            hash,
                ped, 
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
pub fn set_parachute_task_target_raw(
        ped: , 
        x: , 
        y: , 
        z: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xC313379AF0FCEDA7u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xC313379AF0FCEDA7u64;

        invoke_raw_typed!(
            hash,
                ped, 
                x, 
                y, 
                z
        )
    }
}

/// The given ped will try to drive the plane to the given coordinates and will then drive around the given coords (the plane will form 8s on the ground)



pub fn task_plane_taxi_safe(
        
        
            pilot: 
        , 
        
        
            aircraft: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x92C360B5F15D2302u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x92C360B5F15D2302u64;
        
        let result = invoke_raw!(
            hash,
                pilot, 
                aircraft
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn task_plane_taxi_raw(
        pilot: , 
        aircraft: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x92C360B5F15D2302u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x92C360B5F15D2302u64;

        invoke_raw_typed!(
            hash,
                pilot, 
                aircraft
        )
    }
}

/// ```
Used only once in the scripts (am_mp_nightclub)
```

```
Used only once in the scripts (am_mp_nightclub)

NativeDB Introduced: v1493
```



pub fn _task_move_network_by_name_with_init_params_safe(
        
        
            ped: 
        , 
        
        
            p1: 
        , 
        
        
            data: 
        , 
        
        
            p3: 
        , 
        
        
            p4: 
        , 
        
        
            animDict: 
        , 
        
        
            flags: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x3D45B0B355C5E0C9u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x3D45B0B355C5E0C9u64;
        
        let result = invoke_raw!(
            hash,
                ped, 
                p1, 
                data, 
                p3, 
                p4, 
                animDict, 
                flags
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn _task_move_network_by_name_with_init_params_raw(
        ped: , 
        p1: , 
        data: , 
        p3: , 
        p4: , 
        animDict: , 
        flags: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x3D45B0B355C5E0C9u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x3D45B0B355C5E0C9u64;

        invoke_raw_typed!(
            hash,
                ped, 
                p1, 
                data, 
                p3, 
                p4, 
                animDict, 
                flags
        )
    }
}

/// ## Parameters
*



pub fn is_waypoint_playback_going_on_for_ped_safe(
        
        
            p0: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xE03B3F2D3DC59B64u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xE03B3F2D3DC59B64u64;
        
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
pub fn is_waypoint_playback_going_on_for_ped_raw(
        p0: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xE03B3F2D3DC59B64u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xE03B3F2D3DC59B64u64;

        invoke_raw_typed!(
            hash,
                p0
        )
    }
}

/// ## Parameters
*



pub fn waypoint_playback_start_aiming_at_coord_safe(
        
        
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
        let hash = 0x8968400D900ED8B3u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x8968400D900ED8B3u64;
        
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
pub fn waypoint_playback_start_aiming_at_coord_raw(
        p0: , 
        p1: , 
        p2: , 
        p3: , 
        p4: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x8968400D900ED8B3u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x8968400D900ED8B3u64;

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
After looking at some scripts the second parameter seems to be an id of some kind. Here are some I found from some R* scripts:
"miss_Tower_01" (this went from 01 - 10)
"miss_Ass0" (0, 4, 6, 3)
"MISS_PATROL_8"
I think they're patrol routes, but I'm not sure. And I believe the 3rd parameter is a BOOL, but I can't confirm other than only seeing 0 and 1 being passed.
As far as I can see the patrol routes names such as "miss_Ass0" have been defined earlier in the scripts. This leads me to believe we can defined our own new patrol routes by following the same approach.
From the scripts
    TASK::OPEN_PATROL_ROUTE("miss_Ass0");
    TASK::ADD_PATROL_ROUTE_NODE(0, "WORLD_HUMAN_GUARD_STAND", l_738[0/*3*/], -139.4076690673828, -993.4732055664062, 26.2754, MISC::GET_RANDOM_INT_IN_RANGE(5000, 10000));
    TASK::ADD_PATROL_ROUTE_NODE(1, "WORLD_HUMAN_GUARD_STAND", l_738[1/*3*/], -116.1391830444336, -987.4984130859375, 26.38541030883789, MISC::GET_RANDOM_INT_IN_RANGE(5000, 10000));
    TASK::ADD_PATROL_ROUTE_NODE(2, "WORLD_HUMAN_GUARD_STAND", l_738[2/*3*/], -128.46847534179688, -979.0340576171875, 26.2754, MISC::GET_RANDOM_INT_IN_RANGE(5000, 10000));
    TASK::ADD_PATROL_ROUTE_LINK(0, 1);
    TASK::ADD_PATROL_ROUTE_LINK(1, 2);
    TASK::ADD_PATROL_ROUTE_LINK(2, 0);
    TASK::CLOSE_PATROL_ROUTE();
    TASK::CREATE_PATROL_ROUTE();
```



pub fn task_patrol_safe(
        
        
            ped: 
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
        let hash = 0xBDA5DF49D080FE4Eu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xBDA5DF49D080FE4Eu64;
        
        let result = invoke_raw!(
            hash,
                ped, 
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
pub fn task_patrol_raw(
        ped: , 
        p1: , 
        p2: , 
        p3: , 
        p4: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xBDA5DF49D080FE4Eu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xBDA5DF49D080FE4Eu64;

        invoke_raw_typed!(
            hash,
                ped, 
                p1, 
                p2, 
                p3, 
                p4
        )
    }
}

/// ## Parameters
*



pub fn task_seek_cover_to_cover_point_safe(
        
        
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
        let hash = 0xD43D95C7A869447Fu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xD43D95C7A869447Fu64;
        
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
pub fn task_seek_cover_to_cover_point_raw(
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
        let hash = 0xD43D95C7A869447Fu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xD43D95C7A869447Fu64;

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

/// For an example on how to use this please refer to [OPEN_SEQUENCE_TASK](#_0xE8854A4326B9E12B)



pub fn task_perform_sequence_safe(
        
        
            ped: 
        , 
        
        
            taskSequenceId: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x5ABA3986D90D8A3Bu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x5ABA3986D90D8A3Bu64;
        
        let result = invoke_raw!(
            hash,
                ped, 
                taskSequenceId
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn task_perform_sequence_raw(
        ped: , 
        taskSequenceId: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x5ABA3986D90D8A3Bu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x5ABA3986D90D8A3Bu64;

        invoke_raw_typed!(
            hash,
                ped, 
                taskSequenceId
        )
    }
}

/// ## Parameters
*



pub fn task_set_sphere_defensive_area_safe(
        
        
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
        let hash = 0x933C06518B52A9A4u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x933C06518B52A9A4u64;
        
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
pub fn task_set_sphere_defensive_area_raw(
        p0: , 
        p1: , 
        p2: , 
        p3: , 
        p4: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x933C06518B52A9A4u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x933C06518B52A9A4u64;

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



pub fn _0x8634cef2522d987b_safe(
        
        
            ped: 
        , 
        
        
            p1: 
        , 
        
        
            value: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x8634CEF2522D987Bu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x8634CEF2522D987Bu64;
        
        let result = invoke_raw!(
            hash,
                ped, 
                p1, 
                value
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn _0x8634cef2522d987b_raw(
        ped: , 
        p1: , 
        value: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x8634CEF2522D987Bu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x8634CEF2522D987Bu64;

        invoke_raw_typed!(
            hash,
                ped, 
                p1, 
                value
        )
    }
}

/// ```
p2 tend to be 16, 17 or 1  
p3 to p7 tend to be 0.0  
```



pub fn task_chat_to_ped_safe(
        
        
            ped: 
        , 
        
        
            target: 
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
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x8C338E0263E4FD19u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x8C338E0263E4FD19u64;
        
        let result = invoke_raw!(
            hash,
                ped, 
                target, 
                p2, 
                p3, 
                p4, 
                p5, 
                p6, 
                p7
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn task_chat_to_ped_raw(
        ped: , 
        target: , 
        p2: , 
        p3: , 
        p4: , 
        p5: , 
        p6: , 
        p7: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x8C338E0263E4FD19u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x8C338E0263E4FD19u64;

        invoke_raw_typed!(
            hash,
                ped, 
                target, 
                p2, 
                p3, 
                p4, 
                p5, 
                p6, 
                p7
        )
    }
}

/// For an example on how to use this please refer to [OPEN_SEQUENCE_TASK](#_0xE8854A4326B9E12B)



pub fn clear_sequence_task_safe(
        
        
            taskSequenceId: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x3841422E9C488D8Cu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x3841422E9C488D8Cu64;
        
        let result = invoke_raw!(
            hash,
                taskSequenceId
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn clear_sequence_task_raw(
        taskSequenceId: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x3841422E9C488D8Cu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x3841422E9C488D8Cu64;

        invoke_raw_typed!(
            hash,
                taskSequenceId
        )
    }
}

/// ```
scenarioName example: "WORLD_HUMAN_GUARD_STAND"  
```



pub fn task_stand_guard_safe(
        
        
            ped: 
        , 
        
        
            x: 
        , 
        
        
            y: 
        , 
        
        
            z: 
        , 
        
        
            heading: 
        , 
        
        
            scenarioName: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xAE032F8BBA959E90u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xAE032F8BBA959E90u64;
        
        let result = invoke_raw!(
            hash,
                ped, 
                x, 
                y, 
                z, 
                heading, 
                scenarioName
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn task_stand_guard_raw(
        ped: , 
        x: , 
        y: , 
        z: , 
        heading: , 
        scenarioName: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xAE032F8BBA959E90u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xAE032F8BBA959E90u64;

        invoke_raw_typed!(
            hash,
                ped, 
                x, 
                y, 
                z, 
                heading, 
                scenarioName
        )
    }
}

/// ```
Task index enum: https://alloc8or.re/gta5/doc/enums/eTaskTypeIndex.txt
```



pub fn get_is_task_active_safe(
        
        
            ped: 
        , 
        
        
            taskIndex: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xB0760331C7AA4155u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xB0760331C7AA4155u64;
        
        let result = invoke_raw!(
            hash,
                ped, 
                taskIndex
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn get_is_task_active_raw(
        ped: , 
        taskIndex: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xB0760331C7AA4155u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xB0760331C7AA4155u64;

        invoke_raw_typed!(
            hash,
                ped, 
                taskIndex
        )
    }
}

/// ## Parameters
*



pub fn add_vehicle_subtask_attack_ped_safe(
        
        
            ped: 
        , 
        
        
            ped2: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x85F462BADC7DA47Fu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x85F462BADC7DA47Fu64;
        
        let result = invoke_raw!(
            hash,
                ped, 
                ped2
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn add_vehicle_subtask_attack_ped_raw(
        ped: , 
        ped2: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x85F462BADC7DA47Fu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x85F462BADC7DA47Fu64;

        invoke_raw_typed!(
            hash,
                ped, 
                ped2
        )
    }
}

/// connects/links 2 [route nodes](#_0x8EDF950167586B7C)  
image representing the cyclic example below:  
![image](https://user-images.githubusercontent.com/55803068/188470866-c32c6a9f-a25d-4772-9b18-5be46e2c14a1.png)



pub fn add_patrol_route_link_safe(
        
        
            id1: 
        , 
        
        
            id2: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x23083260DEC3A551u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x23083260DEC3A551u64;
        
        let result = invoke_raw!(
            hash,
                id1, 
                id2
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn add_patrol_route_link_raw(
        id1: , 
        id2: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x23083260DEC3A551u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x23083260DEC3A551u64;

        invoke_raw_typed!(
            hash,
                id1, 
                id2
        )
    }
}

/// ```
For a full list of the points, see here: goo.gl/wIH0vn
Max number of loaded recordings is 32.
```



pub fn request_waypoint_recording_safe(
        
        
            name: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x9EEFB62EB27B5792u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x9EEFB62EB27B5792u64;
        
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
pub fn request_waypoint_recording_raw(
        name: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x9EEFB62EB27B5792u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x9EEFB62EB27B5792u64;

        invoke_raw_typed!(
            hash,
                name
        )
    }
}

/// ```
This function is hard-coded to always return 0.  
```



pub fn is_ped_being_arrested_safe(
        
        
            ped: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x90A09F3A45FED688u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x90A09F3A45FED688u64;
        
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
pub fn is_ped_being_arrested_raw(
        ped: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x90A09F3A45FED688u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x90A09F3A45FED688u64;

        invoke_raw_typed!(
            hash,
                ped
        )
    }
}

/// ## Parameters
*



pub fn get_vehicle_waypoint_progress_safe(
        
        
            vehicle: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x9824CFF8FC66E159u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x9824CFF8FC66E159u64;
        
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
pub fn get_vehicle_waypoint_progress_raw(
        vehicle: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x9824CFF8FC66E159u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x9824CFF8FC66E159u64;

        invoke_raw_typed!(
            hash,
                vehicle
        )
    }
}

/// ## Parameters
*



pub fn task_shocking_event_react_safe(
        
        
            ped: 
        , 
        
        
            eventHandle: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x452419CBD838065Bu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x452419CBD838065Bu64;
        
        let result = invoke_raw!(
            hash,
                ped, 
                eventHandle
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn task_shocking_event_react_raw(
        ped: , 
        eventHandle: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x452419CBD838065Bu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x452419CBD838065Bu64;

        invoke_raw_typed!(
            hash,
                ped, 
                eventHandle
        )
    }
}

/// ## Parameters
*



pub fn task_combat_ped_timed_safe(
        
        
            p0: 
        , 
        
        
            ped: 
        , 
        
        
            p2: 
        , 
        
        
            p3: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x944F30DCB7096BDEu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x944F30DCB7096BDEu64;
        
        let result = invoke_raw!(
            hash,
                p0, 
                ped, 
                p2, 
                p3
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn task_combat_ped_timed_raw(
        p0: , 
        ped: , 
        p2: , 
        p3: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x944F30DCB7096BDEu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x944F30DCB7096BDEu64;

        invoke_raw_typed!(
            hash,
                p0, 
                ped, 
                p2, 
                p3
        )
    }
}

/// ```
The patrol route name must starts with "miss_" to be properly created. 

 patrolRoutes found in the b617d scripts:
 "miss_Ass0",
 "miss_Ass1",
 "miss_Ass2",
 "miss_Ass3",
 "miss_Ass4",
 "miss_Ass5",
 "miss_Ass6",
 "MISS_PATROL_6",
 "MISS_PATROL_7",
 "MISS_PATROL_8",
 "MISS_PATROL_9",
 "miss_Tower_01",
 "miss_Tower_02",
 "miss_Tower_03",
 "miss_Tower_04",
 "miss_Tower_05",
 "miss_Tower_06",
 "miss_Tower_07",
 "miss_Tower_08",
 "miss_Tower_10"
```



pub fn open_patrol_route_safe(
        
        
            patrolRoute: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xA36BFB5EE89F3D82u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xA36BFB5EE89F3D82u64;
        
        let result = invoke_raw!(
            hash,
                patrolRoute
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn open_patrol_route_raw(
        patrolRoute: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xA36BFB5EE89F3D82u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xA36BFB5EE89F3D82u64;

        invoke_raw_typed!(
            hash,
                patrolRoute
        )
    }
}

/// REMOVE_ALL_COVER_BLOCKING_AREAS native function



pub fn remove_all_cover_blocking_areas_safe(
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xDB6708C0B46F56D8u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xDB6708C0B46F56D8u64;
        
        let result = invoke_raw!(
            hash,
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn remove_all_cover_blocking_areas_raw(
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xDB6708C0B46F56D8u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xDB6708C0B46F56D8u64;

        invoke_raw_typed!(
            hash,
        )
    }
}

/// See [`GET_NAVMESH_ROUTE_DISTANCE_REMAINING`](#_0xC6F5C0BCDC74D62D) for more details.



pub fn get_navmesh_route_result_safe(
        
        
            ped: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x632E831F382A0FA8u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x632E831F382A0FA8u64;
        
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
pub fn get_navmesh_route_result_raw(
        ped: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x632E831F382A0FA8u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x632E831F382A0FA8u64;

        invoke_raw_typed!(
            hash,
                ped
        )
    }
}

/// ## Parameters
*



pub fn is_move_blend_ratio_still_safe(
        
        
            ped: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x349CE7B56DAFD95Cu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x349CE7B56DAFD95Cu64;
        
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
pub fn is_move_blend_ratio_still_raw(
        ped: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x349CE7B56DAFD95Cu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x349CE7B56DAFD95Cu64;

        invoke_raw_typed!(
            hash,
                ped
        )
    }
}

/// ```
NativeDB Introduced: v1868
```



pub fn _task_wander_specific_safe(
        
        
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
        let hash = 0x6919A2F136426098u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x6919A2F136426098u64;
        
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
pub fn _task_wander_specific_raw(
        p0: , 
        p1: , 
        p2: , 
        p3: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x6919A2F136426098u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x6919A2F136426098u64;

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



pub fn task_plane_chase_safe(
        
        
            pilot: 
        , 
        
        
            entityToFollow: 
        , 
        
        
            x: 
        , 
        
        
            y: 
        , 
        
        
            z: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x2D2386F273FF7A25u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x2D2386F273FF7A25u64;
        
        let result = invoke_raw!(
            hash,
                pilot, 
                entityToFollow, 
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
pub fn task_plane_chase_raw(
        pilot: , 
        entityToFollow: , 
        x: , 
        y: , 
        z: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x2D2386F273FF7A25u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x2D2386F273FF7A25u64;

        invoke_raw_typed!(
            hash,
                pilot, 
                entityToFollow, 
                x, 
                y, 
                z
        )
    }
}

/// ## Parameters
*



pub fn is_move_blend_ratio_walking_safe(
        
        
            ped: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xF133BBBE91E1691Fu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xF133BBBE91E1691Fu64;
        
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
pub fn is_move_blend_ratio_walking_raw(
        ped: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xF133BBBE91E1691Fu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xF133BBBE91E1691Fu64;

        invoke_raw_typed!(
            hash,
                ped
        )
    }
}

/// ```
NativeDB Added Parameter 13: Any p12
```



pub fn task_go_to_coord_any_means_extra_params_safe(
        
        
            ped: 
        , 
        
        
            x: 
        , 
        
        
            y: 
        , 
        
        
            z: 
        , 
        
        
            speed: 
        , 
        
        
            p5: 
        , 
        
        
            p6: 
        , 
        
        
            walkingStyle: 
        , 
        
        
            p8: 
        , 
        
        
            p9: 
        , 
        
        
            p10: 
        , 
        
        
            p11: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x1DD45F9ECFDB1BC9u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x1DD45F9ECFDB1BC9u64;
        
        let result = invoke_raw!(
            hash,
                ped, 
                x, 
                y, 
                z, 
                speed, 
                p5, 
                p6, 
                walkingStyle, 
                p8, 
                p9, 
                p10, 
                p11
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn task_go_to_coord_any_means_extra_params_raw(
        ped: , 
        x: , 
        y: , 
        z: , 
        speed: , 
        p5: , 
        p6: , 
        walkingStyle: , 
        p8: , 
        p9: , 
        p10: , 
        p11: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x1DD45F9ECFDB1BC9u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x1DD45F9ECFDB1BC9u64;

        invoke_raw_typed!(
            hash,
                ped, 
                x, 
                y, 
                z, 
                speed, 
                p5, 
                p6, 
                walkingStyle, 
                p8, 
                p9, 
                p10, 
                p11
        )
    }
}

/// ```
Example from the scripts:
TASK::TASK_PLAY_PHONE_GESTURE_ANIMATION(PLAYER::PLAYER_PED_ID(), v_3, v_2, v_4, 0.25, 0.25, 0, 0);
=========================================================
^^ No offense, but Idk how that would really help anyone.
As for the animDict & animation, they're both store in a global in all 5 scripts. So if anyone would be so kind as to read that global and comment what strings they use. Thanks.
Known boneMaskTypes'
"BONEMASK_HEADONLY"
"BONEMASK_HEAD_NECK_AND_ARMS"
"BONEMASK_HEAD_NECK_AND_L_ARM"
"BONEMASK_HEAD_NECK_AND_R_ARM"
p4 known args - 0.0f, 0.5f, 0.25f
p5 known args - 0.0f, 0.25f
p6 known args - 1 if a global if check is passed.
p7 known args - 1 if a global if check is passed.
The values found above, I found within the 5 scripts this is ever called in. (fmmc_launcher, fm_deathmatch_controller, fm_impromptu_dm_controller, fm_mission_controller, and freemode).
=========================================================
```



pub fn task_play_phone_gesture_animation_safe(
        
        
            ped: 
        , 
        
        
            animDict: 
        , 
        
        
            animation: 
        , 
        
        
            boneMaskType: 
        , 
        
        
            p4: 
        , 
        
        
            p5: 
        , 
        
        
            p6: 
        , 
        
        
            p7: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x8FBB6758B3B3E9ECu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x8FBB6758B3B3E9ECu64;
        
        let result = invoke_raw!(
            hash,
                ped, 
                animDict, 
                animation, 
                boneMaskType, 
                p4, 
                p5, 
                p6, 
                p7
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn task_play_phone_gesture_animation_raw(
        ped: , 
        animDict: , 
        animation: , 
        boneMaskType: , 
        p4: , 
        p5: , 
        p6: , 
        p7: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x8FBB6758B3B3E9ECu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x8FBB6758B3B3E9ECu64;

        invoke_raw_typed!(
            hash,
                ped, 
                animDict, 
                animation, 
                boneMaskType, 
                p4, 
                p5, 
                p6, 
                p7
        )
    }
}

/// ```
REMOVE_*

NativeDB Introduced: v1493
```



pub fn _0xfa83ca6776038f64_safe(
        
        
            x: 
        , 
        
        
            y: 
        , 
        
        
            z: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xFA83CA6776038F64u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xFA83CA6776038F64u64;
        
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
pub fn _0xfa83ca6776038f64_raw(
        x: , 
        y: , 
        z: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xFA83CA6776038F64u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xFA83CA6776038F64u64;

        invoke_raw_typed!(
            hash,
                x, 
                y, 
                z
        )
    }
}

/// ```
p0 - PLAYER::PLAYER_PED_ID();
p1 - "Phase", "Wobble", "x_axis","y_axis","introphase","speed".
p2 - From what i can see it goes up to 1f (maybe).
-LcGamingHD
Example: TASK::_D5BB4025AE449A4E(PLAYER::PLAYER_PED_ID(), "Phase", 0.5);
```



pub fn set_task_move_network_signal_float_safe(
        
        
            ped: 
        , 
        
        
            signalName: 
        , 
        
        
            value: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xD5BB4025AE449A4Eu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xD5BB4025AE449A4Eu64;
        
        let result = invoke_raw!(
            hash,
                ped, 
                signalName, 
                value
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn set_task_move_network_signal_float_raw(
        ped: , 
        signalName: , 
        value: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xD5BB4025AE449A4Eu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xD5BB4025AE449A4Eu64;

        invoke_raw_typed!(
            hash,
                ped, 
                signalName, 
                value
        )
    }
}

/// ## Parameters
*



pub fn task_plant_bomb_safe(
        
        
            ped: 
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
        let hash = 0x965FEC691D55E9BFu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x965FEC691D55E9BFu64;
        
        let result = invoke_raw!(
            hash,
                ped, 
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
pub fn task_plant_bomb_raw(
        ped: , 
        x: , 
        y: , 
        z: , 
        heading: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x965FEC691D55E9BFu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x965FEC691D55E9BFu64;

        invoke_raw_typed!(
            hash,
                ped, 
                x, 
                y, 
                z, 
                heading
        )
    }
}

/// ## Parameters
*



pub fn waypoint_playback_resume_safe(
        
        
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
        let hash = 0x244F70C84C547D2Du32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x244F70C84C547D2Du64;
        
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
pub fn waypoint_playback_resume_raw(
        p0: , 
        p1: , 
        p2: , 
        p3: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x244F70C84C547D2Du32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x244F70C84C547D2Du64;

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
Forces the ped to use the mounted weapon.  
Returns false if task is not possible.  
```



pub fn control_mounted_weapon_safe(
        
        
            ped: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xDCFE42068FE0135Au32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xDCFE42068FE0135Au64;
        
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
pub fn control_mounted_weapon_raw(
        ped: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xDCFE42068FE0135Au32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xDCFE42068FE0135Au64;

        invoke_raw_typed!(
            hash,
                ped
        )
    }
}

/// ## Parameters
*



pub fn get_ped_desired_move_blend_ratio_safe(
        
        
            ped: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x8517D4A6CA8513EDu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x8517D4A6CA8513EDu64;
        
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
pub fn get_ped_desired_move_blend_ratio_raw(
        ped: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x8517D4A6CA8513EDu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x8517D4A6CA8513EDu64;

        invoke_raw_typed!(
            hash,
                ped
        )
    }
}

/// ## Parameters
*



pub fn is_task_move_network_ready_for_transition_safe(
        
        
            ped: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x30ED88D5E0C56A37u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x30ED88D5E0C56A37u64;
        
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
pub fn is_task_move_network_ready_for_transition_raw(
        ped: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x30ED88D5E0C56A37u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x30ED88D5E0C56A37u64;

        invoke_raw_typed!(
            hash,
                ped
        )
    }
}

/// ```
task_vehicle_follow_waypoint_recording(Ped p0, Vehicle p1, string p2, int p3, int p4, int p5, int p6, float.x p7, float.Y p8, float.Z p9, bool p10, int p11)
p2 = Waypoint recording string (found in update\update.rpf\x64\levels\gta5\waypointrec.rpf
p3 = 786468
p4 = 0
p5 = 16
p6 = -1 (angle?)
p7/8/9 = usually v3.zero
p10 = bool (repeat?)
p11 = 1073741824
-khorio
```



pub fn task_vehicle_follow_waypoint_recording_safe(
        
        
            ped: 
        , 
        
        
            vehicle: 
        , 
        
        
            WPRecording: 
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
        let hash = 0x3123FAA6DB1CF7EDu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x3123FAA6DB1CF7EDu64;
        
        let result = invoke_raw!(
            hash,
                ped, 
                vehicle, 
                WPRecording, 
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
pub fn task_vehicle_follow_waypoint_recording_raw(
        ped: , 
        vehicle: , 
        WPRecording: , 
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
        let hash = 0x3123FAA6DB1CF7EDu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x3123FAA6DB1CF7EDu64;

        invoke_raw_typed!(
            hash,
                ped, 
                vehicle, 
                WPRecording, 
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



pub fn remove_cover_point_safe(
        
        
            coverpoint: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xAE287C923D891715u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xAE287C923D891715u64;
        
        let result = invoke_raw!(
            hash,
                coverpoint
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn remove_cover_point_raw(
        coverpoint: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xAE287C923D891715u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xAE287C923D891715u64;

        invoke_raw_typed!(
            hash,
                coverpoint
        )
    }
}

/// ## Parameters
*



pub fn set_ped_path_avoid_fire_safe(
        
        
            ped: 
        , 
        
        
            avoidFire: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x4455517B28441E60u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x4455517B28441E60u64;
        
        let result = invoke_raw!(
            hash,
                ped, 
                avoidFire
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn set_ped_path_avoid_fire_raw(
        ped: , 
        avoidFire: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x4455517B28441E60u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x4455517B28441E60u64;

        invoke_raw_typed!(
            hash,
                ped, 
                avoidFire
        )
    }
}

/// ```
Firing Pattern Hash Information: https://pastebin.com/Px036isB
```



pub fn task_shoot_at_coord_safe(
        
        
            ped: 
        , 
        
        
            x: 
        , 
        
        
            y: 
        , 
        
        
            z: 
        , 
        
        
            duration: 
        , 
        
        
            firingPattern: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x46A6CC01E0826106u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x46A6CC01E0826106u64;
        
        let result = invoke_raw!(
            hash,
                ped, 
                x, 
                y, 
                z, 
                duration, 
                firingPattern
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn task_shoot_at_coord_raw(
        ped: , 
        x: , 
        y: , 
        z: , 
        duration: , 
        firingPattern: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x46A6CC01E0826106u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x46A6CC01E0826106u64;

        invoke_raw_typed!(
            hash,
                ped, 
                x, 
                y, 
                z, 
                duration, 
                firingPattern
        )
    }
}

/// ## Parameters
*



pub fn set_sequence_to_repeat_safe(
        
        
            taskSequenceId: 
        , 
        
        
            repeat: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x58C70CF3A41E4AE7u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x58C70CF3A41E4AE7u64;
        
        let result = invoke_raw!(
            hash,
                taskSequenceId, 
                repeat
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn set_sequence_to_repeat_raw(
        taskSequenceId: , 
        repeat: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x58C70CF3A41E4AE7u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x58C70CF3A41E4AE7u64;

        invoke_raw_typed!(
            hash,
                taskSequenceId, 
                repeat
        )
    }
}

/// ## Parameters
*



pub fn get_vehicle_waypoint_target_point_safe(
        
        
            vehicle: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x416B62AC8B9E5BBDu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x416B62AC8B9E5BBDu64;
        
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
pub fn get_vehicle_waypoint_target_point_raw(
        vehicle: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x416B62AC8B9E5BBDu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x416B62AC8B9E5BBDu64;

        invoke_raw_typed!(
            hash,
                vehicle
        )
    }
}

/// ```
The 2nd param (unused) is not implemented.



pub fn task_reload_weapon_safe(
        
        
            ped: 
        , 
        
        
            unused: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x62D2916F56B9CD2Du32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x62D2916F56B9CD2Du64;
        
        let result = invoke_raw!(
            hash,
                ped, 
                unused
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn task_reload_weapon_raw(
        ped: , 
        unused: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x62D2916F56B9CD2Du32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x62D2916F56B9CD2Du64;

        invoke_raw_typed!(
            hash,
                ped, 
                unused
        )
    }
}

/// The given ped will try to open the nearest door to 'seat'.

Example: telling the ped to open the door for the driver seat does not necessarily mean it will open the driver door, it may choose to open the passenger door instead if that one is closer.



pub fn task_open_vehicle_door_safe(
        
        
            ped: 
        , 
        
        
            vehicle: 
        , 
        
        
            timeOut: 
        , 
        
        
            seat: 
        , 
        
        
            speed: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x965791A9A488A062u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x965791A9A488A062u64;
        
        let result = invoke_raw!(
            hash,
                ped, 
                vehicle, 
                timeOut, 
                seat, 
                speed
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn task_open_vehicle_door_raw(
        ped: , 
        vehicle: , 
        timeOut: , 
        seat: , 
        speed: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x965791A9A488A062u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x965791A9A488A062u64;

        invoke_raw_typed!(
            hash,
                ped, 
                vehicle, 
                timeOut, 
                seat, 
                speed
        )
    }
}

/// ## Parameters
*



pub fn task_sweep_aim_position_safe(
        
        
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
        , 
        
        
            p10: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x7AFE8FDC10BC07D2u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x7AFE8FDC10BC07D2u64;
        
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
                p9, 
                p10
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn task_sweep_aim_position_raw(
        p0: , 
        p1: , 
        p2: , 
        p3: , 
        p4: , 
        p5: , 
        p6: , 
        p7: , 
        p8: , 
        p9: , 
        p10: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x7AFE8FDC10BC07D2u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x7AFE8FDC10BC07D2u64;

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
                p9, 
                p10
        )
    }
}

/// ```
Groups found in the scripts used with this native:  
"AMMUNATION",  
"QUARRY",  
"Triathlon_1",  
"Triathlon_2",  
"Triathlon_3"  
```



pub fn set_exclusive_scenario_group_safe(
        
        
            scenarioGroup: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x535E97E1F7FC0C6Au32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x535E97E1F7FC0C6Au64;
        
        let result = invoke_raw!(
            hash,
                scenarioGroup
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn set_exclusive_scenario_group_raw(
        scenarioGroup: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x535E97E1F7FC0C6Au32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x535E97E1F7FC0C6Au64;

        invoke_raw_typed!(
            hash,
                scenarioGroup
        )
    }
}

/// ## Parameters
*



pub fn task_use_nearest_scenario_to_coord_warp_safe(
        
        
            ped: 
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
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x58E2E0F23F6B76C3u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x58E2E0F23F6B76C3u64;
        
        let result = invoke_raw!(
            hash,
                ped, 
                x, 
                y, 
                z, 
                radius, 
                p5
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn task_use_nearest_scenario_to_coord_warp_raw(
        ped: , 
        x: , 
        y: , 
        z: , 
        radius: , 
        p5: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x58E2E0F23F6B76C3u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x58E2E0F23F6B76C3u64;

        invoke_raw_typed!(
            hash,
                ped, 
                x, 
                y, 
                z, 
                radius, 
                p5
        )
    }
}

/// ## Parameters
*



pub fn task_get_off_boat_safe(
        
        
            ped: 
        , 
        
        
            boat: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x9C00E77AF14B2DFFu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x9C00E77AF14B2DFFu64;
        
        let result = invoke_raw!(
            hash,
                ped, 
                boat
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn task_get_off_boat_raw(
        ped: , 
        boat: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x9C00E77AF14B2DFFu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x9C00E77AF14B2DFFu64;

        invoke_raw_typed!(
            hash,
                ped, 
                boat
        )
    }
}

/// ## Parameters
*



pub fn task_seek_cover_from_ped_safe(
        
        
            ped: 
        , 
        
        
            target: 
        , 
        
        
            duration: 
        , 
        
        
            p3: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x84D32B3BEC531324u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x84D32B3BEC531324u64;
        
        let result = invoke_raw!(
            hash,
                ped, 
                target, 
                duration, 
                p3
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn task_seek_cover_from_ped_raw(
        ped: , 
        target: , 
        duration: , 
        p3: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x84D32B3BEC531324u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x84D32B3BEC531324u64;

        invoke_raw_typed!(
            hash,
                ped, 
                target, 
                duration, 
                p3
        )
    }
}

/// ```
p1 is always GET_HASH_KEY("empty") in scripts, for the rare times this is used  
```



pub fn task_set_decision_maker_safe(
        
        
            ped: 
        , 
        
        
            p1: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xEB8517DDA73720DAu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xEB8517DDA73720DAu64;
        
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
pub fn task_set_decision_maker_raw(
        ped: , 
        p1: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xEB8517DDA73720DAu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xEB8517DDA73720DAu64;

        invoke_raw_typed!(
            hash,
                ped, 
                p1
        )
    }
}

/// ```
used in sequence task  
both parameters seems to be always 0  
```



pub fn task_toggle_duck_safe(
        
        
            p0: 
        , 
        
        
            p1: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xAC96609B9995EDF8u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xAC96609B9995EDF8u64;
        
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
pub fn task_toggle_duck_raw(
        p0: , 
        p1: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xAC96609B9995EDF8u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xAC96609B9995EDF8u64;

        invoke_raw_typed!(
            hash,
                p0, 
                p1
        )
    }
}

/// ```
Despite its name, it only attacks ONE hated target. The one closest to the specified position.  
```



pub fn task_combat_hated_targets_in_area_safe(
        
        
            ped: 
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
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x4CF5F55DAC3280A0u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x4CF5F55DAC3280A0u64;
        
        let result = invoke_raw!(
            hash,
                ped, 
                x, 
                y, 
                z, 
                radius, 
                p5
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn task_combat_hated_targets_in_area_raw(
        ped: , 
        x: , 
        y: , 
        z: , 
        radius: , 
        p5: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x4CF5F55DAC3280A0u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x4CF5F55DAC3280A0u64;

        invoke_raw_typed!(
            hash,
                ped, 
                x, 
                y, 
                z, 
                radius, 
                p5
        )
    }
}

/// This task warps a ped directly into a cover position closest to the specified point. This can be used to quickly place peds in strategic positions during gameplay.

```
NativeDB Introduced: 2545
```



pub fn task_warp_ped_directly_into_cover_safe(
        
        
            ped: 
        , 
        
        
            time: 
        , 
        
        
            canPeekAndAim: 
        , 
        
        
            forceInitialFacingDirection: 
        , 
        
        
            forceFaceLeft: 
        , 
        
        
            coverIndex: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x6E01E9E8D89F8276u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x6E01E9E8D89F8276u64;
        
        let result = invoke_raw!(
            hash,
                ped, 
                time, 
                canPeekAndAim, 
                forceInitialFacingDirection, 
                forceFaceLeft, 
                coverIndex
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn task_warp_ped_directly_into_cover_raw(
        ped: , 
        time: , 
        canPeekAndAim: , 
        forceInitialFacingDirection: , 
        forceFaceLeft: , 
        coverIndex: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x6E01E9E8D89F8276u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x6E01E9E8D89F8276u64;

        invoke_raw_typed!(
            hash,
                ped, 
                time, 
                canPeekAndAim, 
                forceInitialFacingDirection, 
                forceFaceLeft, 
                coverIndex
        )
    }
}

/// ```
chases targetEnt fast and aggressively  
--  
Makes ped (needs to be in vehicle) chase targetEnt.  
```



pub fn task_vehicle_chase_safe(
        
        
            driver: 
        , 
        
        
            targetEnt: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x3C08A8E30363B353u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x3C08A8E30363B353u64;
        
        let result = invoke_raw!(
            hash,
                driver, 
                targetEnt
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn task_vehicle_chase_raw(
        driver: , 
        targetEnt: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x3C08A8E30363B353u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x3C08A8E30363B353u64;

        invoke_raw_typed!(
            hash,
                driver, 
                targetEnt
        )
    }
}

/// ### NOTE
If this returns 0 that means it failed to get a sequence id.

If you fail to call [`CLOSE_SEQUENCE_TASK`](#_0x39E72BC99E6360CB) and [`CLEAR_SEQUENCE_TASK`](#_0x3841422E9C488D8C) the sequence system can get stuck in a broken state until you restart your client.



pub fn open_sequence_task_safe(
        
        
            taskSequenceId: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xE8854A4326B9E12Bu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xE8854A4326B9E12Bu64;
        
        let result = invoke_raw!(
            hash,
                taskSequenceId
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn open_sequence_task_raw(
        taskSequenceId: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xE8854A4326B9E12Bu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xE8854A4326B9E12Bu64;

        invoke_raw_typed!(
            hash,
                taskSequenceId
        )
    }
}

/// ```
Example:
TASK::TASK_MOVE_NETWORK_BY_NAME(PLAYER::PLAYER_PED_ID(), "arm_wrestling_sweep_paired_a_rev3", 0.0f, true, "mini@arm_wrestling", 0);
```



pub fn task_move_network_by_name_safe(
        
        
            ped: 
        , 
        
        
            task: 
        , 
        
        
            multiplier: 
        , 
        
        
            p3: 
        , 
        
        
            animDict: 
        , 
        
        
            flags: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x2D537BA194896636u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x2D537BA194896636u64;
        
        let result = invoke_raw!(
            hash,
                ped, 
                task, 
                multiplier, 
                p3, 
                animDict, 
                flags
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn task_move_network_by_name_raw(
        ped: , 
        task: , 
        multiplier: , 
        p3: , 
        animDict: , 
        flags: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x2D537BA194896636u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x2D537BA194896636u64;

        invoke_raw_typed!(
            hash,
                ped, 
                task, 
                multiplier, 
                p3, 
                animDict, 
                flags
        )
    }
}

/// ```
The entity will move towards the target until time is over (duration) or get in target's range (distance). p5 and p6 are unknown, but you could leave p5 = 1073741824 or 100 or even 0 (didn't see any difference but on the decompiled scripts, they use 1073741824 mostly) and p6 = 0
Note: I've only tested it on entity -> ped and target -> vehicle. It could work differently on other entities, didn't try it yet.
Example: TASK::TASK_GO_TO_ENTITY(pedHandle, vehicleHandle, 5000, 4.0, 100, 1073741824, 0)
Ped will run towards the vehicle for 5 seconds and stop when time is over or when he gets 4 meters(?) around the vehicle (with duration = -1, the task duration will be ignored).
```



pub fn task_go_to_entity_safe(
        
        
            entity: 
        , 
        
        
            target: 
        , 
        
        
            duration: 
        , 
        
        
            distance: 
        , 
        
        
            speed: 
        , 
        
        
            p5: 
        , 
        
        
            p6: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x6A071245EB0D1882u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x6A071245EB0D1882u64;
        
        let result = invoke_raw!(
            hash,
                entity, 
                target, 
                duration, 
                distance, 
                speed, 
                p5, 
                p6
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn task_go_to_entity_raw(
        entity: , 
        target: , 
        duration: , 
        distance: , 
        speed: , 
        p5: , 
        p6: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x6A071245EB0D1882u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x6A071245EB0D1882u64;

        invoke_raw_typed!(
            hash,
                entity, 
                target, 
                duration, 
                distance, 
                speed, 
                p5, 
                p6
        )
    }
}

/// Similar in functionality to [`TASK_PLAY_ANIM`](#_0xEA47FE3719165B94), except the position and rotation parameters let you specify the initial position and rotation of the task. The ped is teleported to the position specified.

[Animations list](https://alexguirre.github.io/animations-list/)



pub fn task_play_anim_advanced_safe(
        
        
            ped: 
        , 
        
        
            animDictionary: 
        , 
        
        
            animationName: 
        , 
        
        
            posX: 
        , 
        
        
            posY: 
        , 
        
        
            posZ: 
        , 
        
        
            rotX: 
        , 
        
        
            rotY: 
        , 
        
        
            rotZ: 
        , 
        
        
            blendInSpeed: 
        , 
        
        
            blendOutSpeed: 
        , 
        
        
            duration: 
        , 
        
        
            flag: 
        , 
        
        
            animTime: 
        , 
        
        
            p14: 
        , 
        
        
            p15: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x83CDB10EA29B370Bu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x83CDB10EA29B370Bu64;
        
        let result = invoke_raw!(
            hash,
                ped, 
                animDictionary, 
                animationName, 
                posX, 
                posY, 
                posZ, 
                rotX, 
                rotY, 
                rotZ, 
                blendInSpeed, 
                blendOutSpeed, 
                duration, 
                flag, 
                animTime, 
                p14, 
                p15
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn task_play_anim_advanced_raw(
        ped: , 
        animDictionary: , 
        animationName: , 
        posX: , 
        posY: , 
        posZ: , 
        rotX: , 
        rotY: , 
        rotZ: , 
        blendInSpeed: , 
        blendOutSpeed: , 
        duration: , 
        flag: , 
        animTime: , 
        p14: , 
        p15: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x83CDB10EA29B370Bu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x83CDB10EA29B370Bu64;

        invoke_raw_typed!(
            hash,
                ped, 
                animDictionary, 
                animationName, 
                posX, 
                posY, 
                posZ, 
                rotX, 
                rotY, 
                rotZ, 
                blendInSpeed, 
                blendOutSpeed, 
                duration, 
                flag, 
                animTime, 
                p14, 
                p15
        )
    }
}

/// ## Parameters
*



pub fn task_plane_land_safe(
        
        
            pilot: 
        , 
        
        
            plane: 
        , 
        
        
            runwayStartX: 
        , 
        
        
            runwayStartY: 
        , 
        
        
            runwayStartZ: 
        , 
        
        
            runwayEndX: 
        , 
        
        
            runwayEndY: 
        , 
        
        
            runwayEndZ: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xBF19721FA34D32C0u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xBF19721FA34D32C0u64;
        
        let result = invoke_raw!(
            hash,
                pilot, 
                plane, 
                runwayStartX, 
                runwayStartY, 
                runwayStartZ, 
                runwayEndX, 
                runwayEndY, 
                runwayEndZ
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn task_plane_land_raw(
        pilot: , 
        plane: , 
        runwayStartX: , 
        runwayStartY: , 
        runwayStartZ: , 
        runwayEndX: , 
        runwayEndY: , 
        runwayEndZ: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xBF19721FA34D32C0u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xBF19721FA34D32C0u64;

        invoke_raw_typed!(
            hash,
                pilot, 
                plane, 
                runwayStartX, 
                runwayStartY, 
                runwayStartZ, 
                runwayEndX, 
                runwayEndY, 
                runwayEndZ
        )
    }
}

/// ## Parameters
*



pub fn task_swap_weapon_safe(
        
        
            ped: 
        , 
        
        
            p1: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xA21C51255B205245u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xA21C51255B205245u64;
        
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
pub fn task_swap_weapon_raw(
        ped: , 
        p1: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xA21C51255B205245u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xA21C51255B205245u64;

        invoke_raw_typed!(
            hash,
                ped, 
                p1
        )
    }
}

/// ```
from armenian3.c4
TASK::TASK_PUT_PED_DIRECTLY_INTO_MELEE(PlayerPed, armenianPed, 0.0, -1.0, 0.0, 0);
```



pub fn task_put_ped_directly_into_melee_safe(
        
        
            ped: 
        , 
        
        
            meleeTarget: 
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
        let hash = 0x1C6CD14A876FFE39u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x1C6CD14A876FFE39u64;
        
        let result = invoke_raw!(
            hash,
                ped, 
                meleeTarget, 
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
pub fn task_put_ped_directly_into_melee_raw(
        ped: , 
        meleeTarget: , 
        p2: , 
        p3: , 
        p4: , 
        p5: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x1C6CD14A876FFE39u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x1C6CD14A876FFE39u64;

        invoke_raw_typed!(
            hash,
                ped, 
                meleeTarget, 
                p2, 
                p3, 
                p4, 
                p5
        )
    }
}

/// ```
Ped pilot should be in a heli.  
EntityToFollow can be a vehicle or Ped.  
x,y,z appear to be how close to the EntityToFollow the heli should be. Scripts use 0.0, 0.0, 80.0. Then the heli tries to position itself 80 units above the EntityToFollow. If you reduce it to -5.0, it tries to go below (if the EntityToFollow is a heli or plane)  
NOTE: If the pilot finds enemies, it will engage them, then remain there idle, not continuing to chase the Entity given.  
```



pub fn task_heli_chase_safe(
        
        
            pilot: 
        , 
        
        
            entityToFollow: 
        , 
        
        
            x: 
        , 
        
        
            y: 
        , 
        
        
            z: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xAC83B1DB38D0ADA0u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xAC83B1DB38D0ADA0u64;
        
        let result = invoke_raw!(
            hash,
                pilot, 
                entityToFollow, 
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
pub fn task_heli_chase_raw(
        pilot: , 
        entityToFollow: , 
        x: , 
        y: , 
        z: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xAC83B1DB38D0ADA0u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xAC83B1DB38D0ADA0u64;

        invoke_raw_typed!(
            hash,
                pilot, 
                entityToFollow, 
                x, 
                y, 
                z
        )
    }
}

/// ## Parameters
*



pub fn update_task_hands_up_duration_safe(
        
        
            ped: 
        , 
        
        
            duration: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xA98FCAFD7893C834u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xA98FCAFD7893C834u64;
        
        let result = invoke_raw!(
            hash,
                ped, 
                duration
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn update_task_hands_up_duration_raw(
        ped: , 
        duration: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xA98FCAFD7893C834u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xA98FCAFD7893C834u64;

        invoke_raw_typed!(
            hash,
                ped, 
                duration
        )
    }
}

/// ## Parameters
*



pub fn set_drive_task_cruise_speed_safe(
        
        
            driver: 
        , 
        
        
            cruiseSpeed: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x5C9B84BD7D31D908u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x5C9B84BD7D31D908u64;
        
        let result = invoke_raw!(
            hash,
                driver, 
                cruiseSpeed
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn set_drive_task_cruise_speed_raw(
        driver: , 
        cruiseSpeed: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x5C9B84BD7D31D908u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x5C9B84BD7D31D908u64;

        invoke_raw_typed!(
            hash,
                driver, 
                cruiseSpeed
        )
    }
}

/// You can let your character drive to the destination at the speed and driving style you set. You can use map marks to set the destination.

```c
enum eDriveBehaviorFlags {
  DF_StopForCars = 1,
  DF_StopForPeds = 2,
  DF_SwerveAroundAllCars = 4,
  DF_SteerAroundStationaryCars = 8,
  DF_SteerAroundPeds = 16,
  DF_SteerAroundObjects = 32,
  DF_DontSteerAroundPlayerPed = 64,
  DF_StopAtLights = 128,
  DF_GoOffRoadWhenAvoiding = 256,
  DF_DriveIntoOncomingTraffic = 512,
  DF_DriveInReverse = 1024,
  DF_UseWanderFallbackInsteadOfStraightLine = 2048,
  DF_AvoidRestrictedAreas = 4096,
  DF_PreventBackgroundPathfinding = 8192, //



pub fn task_vehicle_drive_to_coord_longrange_safe(
        
        
            ped: 
        , 
        
        
            vehicle: 
        , 
        
        
            x: 
        , 
        
        
            y: 
        , 
        
        
            z: 
        , 
        
        
            speed: 
        , 
        
        
            drivingStyle: 
        , 
        
        
            stopRange: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x158BB33F920D360Cu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x158BB33F920D360Cu64;
        
        let result = invoke_raw!(
            hash,
                ped, 
                vehicle, 
                x, 
                y, 
                z, 
                speed, 
                drivingStyle, 
                stopRange
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn task_vehicle_drive_to_coord_longrange_raw(
        ped: , 
        vehicle: , 
        x: , 
        y: , 
        z: , 
        speed: , 
        drivingStyle: , 
        stopRange: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x158BB33F920D360Cu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x158BB33F920D360Cu64;

        invoke_raw_typed!(
            hash,
                ped, 
                vehicle, 
                x, 
                y, 
                z, 
                speed, 
                drivingStyle, 
                stopRange
        )
    }
}

/// ## Parameters
*



pub fn get_scripted_cover_point_coords_safe(
        
        
            coverpoint: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x594A1028FC2A3E85u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x594A1028FC2A3E85u64;
        
        let result = invoke_raw!(
            hash,
                coverpoint
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn get_scripted_cover_point_coords_raw(
        coverpoint: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x594A1028FC2A3E85u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x594A1028FC2A3E85u64;

        invoke_raw_typed!(
            hash,
                coverpoint
        )
    }
}

/// ```
shootatEntity:  
If true, peds will shoot at Entity till it is dead.  
If false, peds will just walk till they reach the entity and will cease shooting.  
```



pub fn task_go_to_entity_while_aiming_at_entity_safe(
        
        
            ped: 
        , 
        
        
            entityToWalkTo: 
        , 
        
        
            entityToAimAt: 
        , 
        
        
            speed: 
        , 
        
        
            shootatEntity: 
        , 
        
        
            p5: 
        , 
        
        
            p6: 
        , 
        
        
            p7: 
        , 
        
        
            p8: 
        , 
        
        
            firingPattern: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x97465886D35210E9u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x97465886D35210E9u64;
        
        let result = invoke_raw!(
            hash,
                ped, 
                entityToWalkTo, 
                entityToAimAt, 
                speed, 
                shootatEntity, 
                p5, 
                p6, 
                p7, 
                p8, 
                firingPattern
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn task_go_to_entity_while_aiming_at_entity_raw(
        ped: , 
        entityToWalkTo: , 
        entityToAimAt: , 
        speed: , 
        shootatEntity: , 
        p5: , 
        p6: , 
        p7: , 
        p8: , 
        firingPattern: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x97465886D35210E9u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x97465886D35210E9u64;

        invoke_raw_typed!(
            hash,
                ped, 
                entityToWalkTo, 
                entityToAimAt, 
                speed, 
                shootatEntity, 
                p5, 
                p6, 
                p7, 
                p8, 
                firingPattern
        )
    }
}

/// ```
NativeDB Introduced: v1868
```



pub fn _0x29682e2ccf21e9b5_safe(
        
        
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
        , 
        
        
            p10: 
        , 
        
        
            p11: 
        , 
        
        
            p12: 
        , 
        
        
            p13: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x29682E2CCF21E9B5u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x29682E2CCF21E9B5u64;
        
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
                p9, 
                p10, 
                p11, 
                p12, 
                p13
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn _0x29682e2ccf21e9b5_raw(
        p0: , 
        p1: , 
        p2: , 
        p3: , 
        p4: , 
        p5: , 
        p6: , 
        p7: , 
        p8: , 
        p9: , 
        p10: , 
        p11: , 
        p12: , 
        p13: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x29682E2CCF21E9B5u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x29682E2CCF21E9B5u64;

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
                p9, 
                p10, 
                p11, 
                p12, 
                p13
        )
    }
}

/// ## Parameters
*



pub fn get_ped_waypoint_distance_safe(
        
        
            p0: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xE6A877C64CAF1BC5u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xE6A877C64CAF1BC5u64;
        
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
pub fn get_ped_waypoint_distance_raw(
        p0: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xE6A877C64CAF1BC5u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xE6A877C64CAF1BC5u64;

        invoke_raw_typed!(
            hash,
                p0
        )
    }
}

/// ```
In every case of this native, I've only seen the first parameter passed as 0, although I believe it's a Ped after seeing tasks around it using 0. That's because it's used in a Sequence Task.  
The last 3 parameters are definitely coordinates after seeing them passed in other scripts, and even being used straight from the player's coordinates.



pub fn task_throw_projectile_safe(
        
        
            ped: 
        , 
        
        
            x: 
        , 
        
        
            y: 
        , 
        
        
            z: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x7285951DBF6B5A51u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x7285951DBF6B5A51u64;
        
        let result = invoke_raw!(
            hash,
                ped, 
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
pub fn task_throw_projectile_raw(
        ped: , 
        x: , 
        y: , 
        z: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x7285951DBF6B5A51u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x7285951DBF6B5A51u64;

        invoke_raw_typed!(
            hash,
                ped, 
                x, 
                y, 
                z
        )
    }
}

/// All parameters except ped, vehicle, pedTarget and speed are optional; with `missionType` being only those that require a target entity.

If you don't want to use a parameter; pass `0` for int parameters, and `-1.0f` for the remaining float parameters.



pub fn task_vehicle_mission_ped_target_safe(
        
        
            ped: 
        , 
        
        
            vehicle: 
        , 
        
        
            pedTarget: 
        , 
        
        
            missionType: 
        , 
        
        
            speed: 
        , 
        
        
            drivingStyle: 
        , 
        
        
            radius: 
        , 
        
        
            straightLineDist: 
        , 
        
        
            DriveAgainstTraffic: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x9454528DF15D657Au32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x9454528DF15D657Au64;
        
        let result = invoke_raw!(
            hash,
                ped, 
                vehicle, 
                pedTarget, 
                missionType, 
                speed, 
                drivingStyle, 
                radius, 
                straightLineDist, 
                DriveAgainstTraffic
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn task_vehicle_mission_ped_target_raw(
        ped: , 
        vehicle: , 
        pedTarget: , 
        missionType: , 
        speed: , 
        drivingStyle: , 
        radius: , 
        straightLineDist: , 
        DriveAgainstTraffic: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x9454528DF15D657Au32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x9454528DF15D657Au64;

        invoke_raw_typed!(
            hash,
                ped, 
                vehicle, 
                pedTarget, 
                missionType, 
                speed, 
                drivingStyle, 
                radius, 
                straightLineDist, 
                DriveAgainstTraffic
        )
    }
}

/// Doesn't actually return anything.

```
NativeDB Introduced: v2060
```



pub fn _0x0ffb3c758e8c07b9_safe(
        
        
            ped: 
        , 
        
        
            p1: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x0FFB3C758E8C07B9u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x0FFB3C758E8C07B9u64;
        
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
pub fn _0x0ffb3c758e8c07b9_raw(
        ped: , 
        p1: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x0FFB3C758E8C07B9u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x0FFB3C758E8C07B9u64;

        invoke_raw_typed!(
            hash,
                ped, 
                p1
        )
    }
}

/// ## Parameters
*



pub fn set_ped_path_can_drop_from_height_safe(
        
        
            ped: 
        , 
        
        
            Toggle: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xE361C5C71C431A4Fu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xE361C5C71C431A4Fu64;
        
        let result = invoke_raw!(
            hash,
                ped, 
                Toggle
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn set_ped_path_can_drop_from_height_raw(
        ped: , 
        Toggle: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xE361C5C71C431A4Fu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xE361C5C71C431A4Fu64;

        invoke_raw_typed!(
            hash,
                ped, 
                Toggle
        )
    }
}

/// ## Parameters
*



pub fn set_ped_path_can_use_ladders_safe(
        
        
            ped: 
        , 
        
        
            Toggle: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x77A5B103C87F476Eu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x77A5B103C87F476Eu64;
        
        let result = invoke_raw!(
            hash,
                ped, 
                Toggle
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn set_ped_path_can_use_ladders_raw(
        ped: , 
        Toggle: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x77A5B103C87F476Eu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x77A5B103C87F476Eu64;

        invoke_raw_typed!(
            hash,
                ped, 
                Toggle
        )
    }
}

/// All parameters except ped, heli and speed are optional, with `pedTarget`, `vehicleTarget`, `x`, `y`, `z` being dependent on `missionType` (ie. Attack/Flee mission types require a target ped/vehicle, whereas GoTo mission types require either `x`, `y`, `z` or a target ped/vehicle).

If you don't want to use a parameter; pass `0.0f` for `x`, `y` and `z`, `0` for `pedTarget`, `vehicleTarget`, `0` for other int parameters, and `-1.0f` for the remaining float parameters.

```c
enum eHeliMissionFlags
{
  None = 0,
  AttainRequestedOrientation = 1,
  DontModifyOrientation = 2,
  DontModifyPitch = 4,
  DontModifyThrottle = 8,
  DontModifyRoll = 16,
  LandOnArrival = 32,
  DontDoAvoidance = 64,
  StartEngineImmediately = 128,
  ForceHeightMapAvoidance = 256,
  DontClampProbesToDestination = 512,
  EnableTimeslicingWhenPossible = 1024,
  CircleOppositeDirection = 2048,
  MaintainHeightAboveTerrain = 4096,
  IgnoreHiddenEntitiesDuringLand = 8192,
  DisableAllHeightMapAvoidance = 16384,
  // ForceHeightMapAvoidance | DontDoAvoidance
  HeightMapOnlyAvoidance = 320,
}
```



pub fn task_heli_mission_safe(
        
        
            ped: 
        , 
        
        
            heli: 
        , 
        
        
            vehicleTarget: 
        , 
        
        
            pedTarget: 
        , 
        
        
            x: 
        , 
        
        
            y: 
        , 
        
        
            z: 
        , 
        
        
            missionType: 
        , 
        
        
            speed: 
        , 
        
        
            radius: 
        , 
        
        
            heading: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xDAD029E187A2BEB4u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xDAD029E187A2BEB4u64;
        
        let result = invoke_raw!(
            hash,
                ped, 
                heli, 
                vehicleTarget, 
                pedTarget, 
                x, 
                y, 
                z, 
                missionType, 
                speed, 
                radius, 
                heading
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn task_heli_mission_raw(
        ped: , 
        heli: , 
        vehicleTarget: , 
        pedTarget: , 
        x: , 
        y: , 
        z: , 
        missionType: , 
        speed: , 
        radius: , 
        heading: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xDAD029E187A2BEB4u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xDAD029E187A2BEB4u64;

        invoke_raw_typed!(
            hash,
                ped, 
                heli, 
                vehicleTarget, 
                pedTarget, 
                x, 
                y, 
                z, 
                missionType, 
                speed, 
                radius, 
                heading
        )
    }
}

/// ```
Makes a ped in a vehicle follow an entity (ped, vehicle, etc.)
drivingStyle: http://gtaforums.com/topic/822314-guide-driving-styles/
```



pub fn task_vehicle_follow_safe(
        
        
            driver: 
        , 
        
        
            vehicle: 
        , 
        
        
            targetEntity: 
        , 
        
        
            speed: 
        , 
        
        
            drivingStyle: 
        , 
        
        
            minDistance: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xFC545A9F0626E3B6u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xFC545A9F0626E3B6u64;
        
        let result = invoke_raw!(
            hash,
                driver, 
                vehicle, 
                targetEntity, 
                speed, 
                drivingStyle, 
                minDistance
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn task_vehicle_follow_raw(
        driver: , 
        vehicle: , 
        targetEntity: , 
        speed: , 
        drivingStyle: , 
        minDistance: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xFC545A9F0626E3B6u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xFC545A9F0626E3B6u64;

        invoke_raw_typed!(
            hash,
                driver, 
                vehicle, 
                targetEntity, 
                speed, 
                drivingStyle, 
                minDistance
        )
    }
}

/// ```
Most probably plays a specific animation on vehicle. For example getting chop out of van etc...
Here's how its used -
TASK::TASK_VEHICLE_PLAY_ANIM(l_325, "rcmnigel1b", "idle_speedo");
TASK::TASK_VEHICLE_PLAY_ANIM(l_556[0/*1*/], "missfra0_chop_drhome", "InCar_GetOutofBack_Speedo");
FYI : Speedo is the name of van in which chop was put in the mission.
```



pub fn task_vehicle_play_anim_safe(
        
        
            vehicle: 
        , 
        
        
            animationSet: 
        , 
        
        
            animationName: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x69F5C3BD0F3EBD89u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x69F5C3BD0F3EBD89u64;
        
        let result = invoke_raw!(
            hash,
                vehicle, 
                animationSet, 
                animationName
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn task_vehicle_play_anim_raw(
        vehicle: , 
        animationSet: , 
        animationName: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x69F5C3BD0F3EBD89u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x69F5C3BD0F3EBD89u64;

        invoke_raw_typed!(
            hash,
                vehicle, 
                animationSet, 
                animationName
        )
    }
}

/// ```
NativeDB Introduced: v323
```

Warp a ped into a vehicle.



pub fn task_warp_ped_into_vehicle_safe(
        
        
            ped: 
        , 
        
        
            vehicle: 
        , 
        
        
            seatIndex: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x9A7D091411C5F684u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x9A7D091411C5F684u64;
        
        let result = invoke_raw!(
            hash,
                ped, 
                vehicle, 
                seatIndex
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn task_warp_ped_into_vehicle_raw(
        ped: , 
        vehicle: , 
        seatIndex: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x9A7D091411C5F684u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x9A7D091411C5F684u64;

        invoke_raw_typed!(
            hash,
                ped, 
                vehicle, 
                seatIndex
        )
    }
}

/// Attaches a ped to a rope and allows player control to rappel down a wall.
Disables all collisions while on the rope.

NativeDB Introduced: v1868



pub fn _task_rappel_down_wall_safe(
        
        
            ped: 
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
        
        
            minZ: 
        , 
        
        
            ropeId: 
        , 
        
        
            clipset: 
        , 
        
        
            p10: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xEAF66ACDDC794793u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xEAF66ACDDC794793u64;
        
        let result = invoke_raw!(
            hash,
                ped, 
                x1, 
                y1, 
                z1, 
                x2, 
                y2, 
                z2, 
                minZ, 
                ropeId, 
                clipset, 
                p10
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn _task_rappel_down_wall_raw(
        ped: , 
        x1: , 
        y1: , 
        z1: , 
        x2: , 
        y2: , 
        z2: , 
        minZ: , 
        ropeId: , 
        clipset: , 
        p10: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xEAF66ACDDC794793u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xEAF66ACDDC794793u64;

        invoke_raw_typed!(
            hash,
                ped, 
                x1, 
                y1, 
                z1, 
                x2, 
                y2, 
                z2, 
                minZ, 
                ropeId, 
                clipset, 
                p10
        )
    }
}

/// ```
What's strafing?  
```



pub fn is_ped_strafing_safe(
        
        
            ped: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xE45B7F222DE47E09u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xE45B7F222DE47E09u64;
        
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
pub fn is_ped_strafing_raw(
        ped: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xE45B7F222DE47E09u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xE45B7F222DE47E09u64;

        invoke_raw_typed!(
            hash,
                ped
        )
    }
}

/// ## Parameters
*



pub fn task_use_nearest_scenario_chain_to_coord_warp_safe(
        
        
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
        let hash = 0x97A28E63F0BA5631u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x97A28E63F0BA5631u64;
        
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
pub fn task_use_nearest_scenario_chain_to_coord_warp_raw(
        p0: , 
        p1: , 
        p2: , 
        p3: , 
        p4: , 
        p5: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x97A28E63F0BA5631u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x97A28E63F0BA5631u64;

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

/// Clear a ped's tasks. Stop animations and other tasks created by scripts.



pub fn clear_ped_tasks_safe(
        
        
            ped: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xE1EF3C1216AFF2CDu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xE1EF3C1216AFF2CDu64;
        
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
pub fn clear_ped_tasks_raw(
        ped: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xE1EF3C1216AFF2CDu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xE1EF3C1216AFF2CDu64;

        invoke_raw_typed!(
            hash,
                ped
        )
    }
}

/// ## Parameters
*



pub fn task_everyone_leave_vehicle_safe(
        
        
            vehicle: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x7F93691AB4B92272u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x7F93691AB4B92272u64;
        
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
pub fn task_everyone_leave_vehicle_raw(
        vehicle: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x7F93691AB4B92272u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x7F93691AB4B92272u64;

        invoke_raw_typed!(
            hash,
                vehicle
        )
    }
}

/// ## Parameters
*



pub fn set_anim_rate_safe(
        
        
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
        let hash = 0x032D49C5E359C847u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x032D49C5E359C847u64;
        
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
pub fn set_anim_rate_raw(
        p0: , 
        p1: , 
        p2: , 
        p3: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x032D49C5E359C847u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x032D49C5E359C847u64;

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



pub fn waypoint_playback_start_shooting_at_ped_safe(
        
        
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
        let hash = 0xE70BA7B90F8390DCu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xE70BA7B90F8390DCu64;
        
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
pub fn waypoint_playback_start_shooting_at_ped_raw(
        p0: , 
        p1: , 
        p2: , 
        p3: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xE70BA7B90F8390DCu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xE70BA7B90F8390DCu64;

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
```



pub fn task_stop_phone_gesture_animation_safe(
        
        
            ped: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x3FA00D4F4641BFAEu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x3FA00D4F4641BFAEu64;
        
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
pub fn task_stop_phone_gesture_animation_raw(
        ped: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x3FA00D4F4641BFAEu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x3FA00D4F4641BFAEu64;

        invoke_raw_typed!(
            hash,
                ped
        )
    }
}

/// ## Parameters
*



pub fn remove_waypoint_recording_safe(
        
        
            name: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xFF1B8B4AA1C25DC8u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xFF1B8B4AA1C25DC8u64;
        
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
pub fn remove_waypoint_recording_raw(
        name: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xFF1B8B4AA1C25DC8u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xFF1B8B4AA1C25DC8u64;

        invoke_raw_typed!(
            hash,
                name
        )
    }
}

/// ```
IS_*
```



pub fn _0x3e38e28a1d80ddf6_safe(
        
        
            ped: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x3E38E28A1D80DDF6u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x3E38E28A1D80DDF6u64;
        
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
pub fn _0x3e38e28a1d80ddf6_raw(
        ped: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x3E38E28A1D80DDF6u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x3E38E28A1D80DDF6u64;

        invoke_raw_typed!(
            hash,
                ped
        )
    }
}

/// ```
seems to enable/disable specific scenario-types from happening in the game world.
Here are some scenario types from the scripts:
"WORLD_MOUNTAIN_LION_REST"
"WORLD_MOUNTAIN_LION_WANDER"
"DRIVE"
"WORLD_VEHICLE_POLICE_BIKE"
"WORLD_VEHICLE_POLICE_CAR"
"WORLD_VEHICLE_POLICE_NEXT_TO_CAR"
"WORLD_VEHICLE_DRIVE_SOLO"
"WORLD_VEHICLE_BIKER"
"WORLD_VEHICLE_DRIVE_PASSENGERS"
"WORLD_VEHICLE_SALTON_DIRT_BIKE"
"WORLD_VEHICLE_BICYCLE_MOUNTAIN"
"PROP_HUMAN_SEAT_CHAIR"
"WORLD_VEHICLE_ATTRACTOR"
"WORLD_HUMAN_LEANING"
"WORLD_HUMAN_HANG_OUT_STREET"
"WORLD_HUMAN_DRINKING"
"WORLD_HUMAN_SMOKING"
"WORLD_HUMAN_GUARD_STAND"
"WORLD_HUMAN_CLIPBOARD"
"WORLD_HUMAN_HIKER"
"WORLD_VEHICLE_EMPTY"
"WORLD_VEHICLE_BIKE_OFF_ROAD_RACE"
"WORLD_HUMAN_PAPARAZZI"
"WORLD_VEHICLE_PARK_PERPENDICULAR_NOSE_IN"
"WORLD_VEHICLE_PARK_PARALLEL"
"WORLD_VEHICLE_CONSTRUCTION_SOLO"
"WORLD_VEHICLE_CONSTRUCTION_PASSENGERS"
"WORLD_VEHICLE_TRUCK_LOGS"
scenarioType could be the same as scenarioName, used in for example TASK::TASK_START_SCENARIO_AT_POSITION.
```



pub fn set_scenario_type_enabled_safe(
        
        
            scenarioType: 
        , 
        
        
            toggle: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xEB47EC4E34FB7EE1u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xEB47EC4E34FB7EE1u64;
        
        let result = invoke_raw!(
            hash,
                scenarioType, 
                toggle
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn set_scenario_type_enabled_raw(
        scenarioType: , 
        toggle: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xEB47EC4E34FB7EE1u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xEB47EC4E34FB7EE1u64;

        invoke_raw_typed!(
            hash,
                scenarioType, 
                toggle
        )
    }
}

/// ## Parameters
*



pub fn add_cover_point_safe(
        
        
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
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xD5C12A75C7B9497Fu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xD5C12A75C7B9497Fu64;
        
        let result = invoke_raw!(
            hash,
                p0, 
                p1, 
                p2, 
                p3, 
                p4, 
                p5, 
                p6, 
                p7
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn add_cover_point_raw(
        p0: , 
        p1: , 
        p2: , 
        p3: , 
        p4: , 
        p5: , 
        p6: , 
        p7: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xD5C12A75C7B9497Fu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xD5C12A75C7B9497Fu64;

        invoke_raw_typed!(
            hash,
                p0, 
                p1, 
                p2, 
                p3, 
                p4, 
                p5, 
                p6, 
                p7
        )
    }
}

/// ```
In the scripts, p3 was always -1.  
p3 seems to be duration or timeout of turn animation.  
Also facingPed can be 0 or -1 so ped will just raise hands up.  
```



pub fn task_hands_up_safe(
        
        
            ped: 
        , 
        
        
            duration: 
        , 
        
        
            facingPed: 
        , 
        
        
            p3: 
        , 
        
        
            p4: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xF2EAB31979A7F910u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xF2EAB31979A7F910u64;
        
        let result = invoke_raw!(
            hash,
                ped, 
                duration, 
                facingPed, 
                p3, 
                p4
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn task_hands_up_raw(
        ped: , 
        duration: , 
        facingPed: , 
        p3: , 
        p4: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xF2EAB31979A7F910u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xF2EAB31979A7F910u64;

        invoke_raw_typed!(
            hash,
                ped, 
                duration, 
                facingPed, 
                p3, 
                p4
        )
    }
}

/// ## Parameters
*



pub fn task_ped_slide_to_coord_safe(
        
        
            ped: 
        , 
        
        
            x: 
        , 
        
        
            y: 
        , 
        
        
            z: 
        , 
        
        
            heading: 
        , 
        
        
            duration: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xD04FE6765D990A06u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xD04FE6765D990A06u64;
        
        let result = invoke_raw!(
            hash,
                ped, 
                x, 
                y, 
                z, 
                heading, 
                duration
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn task_ped_slide_to_coord_raw(
        ped: , 
        x: , 
        y: , 
        z: , 
        heading: , 
        duration: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xD04FE6765D990A06u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xD04FE6765D990A06u64;

        invoke_raw_typed!(
            hash,
                ped, 
                x, 
                y, 
                z, 
                heading, 
                duration
        )
    }
}

/// ```
Makes the ped run to take cover  
```



pub fn task_stay_in_cover_safe(
        
        
            ped: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xE5DA8615A6180789u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xE5DA8615A6180789u64;
        
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
pub fn task_stay_in_cover_raw(
        ped: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xE5DA8615A6180789u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xE5DA8615A6180789u64;

        invoke_raw_typed!(
            hash,
                ped
        )
    }
}

/// ```
Makes the specified ped attack the target ped.  
p2 should be 0  
p3 should be 16  
```



pub fn task_combat_ped_safe(
        
        
            ped: 
        , 
        
        
            targetPed: 
        , 
        
        
            p2: 
        , 
        
        
            p3: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xF166E48407BAC484u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xF166E48407BAC484u64;
        
        let result = invoke_raw!(
            hash,
                ped, 
                targetPed, 
                p2, 
                p3
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn task_combat_ped_raw(
        ped: , 
        targetPed: , 
        p2: , 
        p3: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xF166E48407BAC484u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xF166E48407BAC484u64;

        invoke_raw_typed!(
            hash,
                ped, 
                targetPed, 
                p2, 
                p3
        )
    }
}

/// ## Parameters
*



pub fn set_task_vehicle_chase_ideal_pursuit_distance_safe(
        
        
            ped: 
        , 
        
        
            distance: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x639B642FACBE4EDDu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x639B642FACBE4EDDu64;
        
        let result = invoke_raw!(
            hash,
                ped, 
                distance
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn set_task_vehicle_chase_ideal_pursuit_distance_raw(
        ped: , 
        distance: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x639B642FACBE4EDDu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x639B642FACBE4EDDu64;

        invoke_raw_typed!(
            hash,
                ped, 
                distance
        )
    }
}

/// Sets the driving style for a ped currently performing a driving task.

Each flag in the `eVehicleDrivingFlags` enum can be combined to create a driving style, with each enabling or disabling a specific driving behavior. The driving style can be set to one of the predefined driving styles, or a custom driving style can be created by combining the flags. This can be done by using the bitwise OR operator (`|`) to combine the flags or by adding the decimal values of the flags together.

```c
enum eVehicleDrivingFlags
{
  None = 0,
  StopForVehicles = 1,
  StopForPeds = 2,
  SwerveAroundAllVehicles = 4,
  SteerAroundStationaryVehicles = 8,
  SteerAroundPeds = 16,
  SteerAroundObjects = 32,
  DontSteerAroundPlayerPed = 64,
  StopAtTrafficLights = 128,
  GoOffRoadWhenAvoiding = 256,
  AllowGoingWrongWay = 512,
  Reverse = 1024,
  UseWanderFallbackInsteadOfStraightLine = 2048,
  AvoidRestrictedAreas = 4096,
  PreventBackgroundPathfinding = 8192,
  AdjustCruiseSpeedBasedOnRoadSpeed = 16384,
  UseShortCutLinks = 262144,
  ChangeLanesAroundObstructions = 524288,
  UseSwitchedOffNodes = 2097152,
  PreferNavmeshRoute = 4194304,
  PlaneTaxiMode = 8388608,
  ForceStraightLine = 16777216,
  UseStringPullingAtJunctions = 33554432,
  TryToAvoidHighways = 536870912,
  ForceJoinInRoadDirection = 1073741824,
  StopAtDestination = 2147483648,
  // StopForVehicles | StopForPeds | SteerAroundObjects | SteerAroundStationaryVehicles | StopAtTrafficLights | UseShortCutLinks | ChangeLanesAroundObstructions
  DrivingModeStopForVehicles = 786603,
  // StopForVehicles | StopForPeds | StopAtTrafficLights | UseShortCutLinks
  DrivingModeStopForVehiclesStrict = 262275,
  // SwerveAroundAllVehicles | SteerAroundObjects | UseShortCutLinks | ChangeLanesAroundObstructions | StopForVehicles
  DrivingModeAvoidVehicles = 786469,
  // SwerveAroundAllVehicles | SteerAroundObjects | UseShortCutLinks | ChangeLanesAroundObstructions
  DrivingModeAvoidVehiclesReckless = 786468,
  // StopForVehicles | SteerAroundStationaryVehicles | StopForPeds | SteerAroundObjects | UseShortCutLinks | ChangeLanesAroundObstructions
  DrivingModeStopForVehiclesIgnoreLights = 786475,
  // SwerveAroundAllVehicles | StopAtTrafficLights | SteerAroundObjects | UseShortCutLinks | ChangeLanesAroundObstructions | StopForVehicles
  DrivingModeAvoidVehiclesObeyLights = 786597,
  // SwerveAroundAllVehicles | StopAtTrafficLights | StopForPeds | SteerAroundObjects | UseShortCutLinks | ChangeLanesAroundObstructions | StopForVehicles
  DrivingModeAvoidVehiclesStopForPedsObeyLights = 786599,
}
```



pub fn set_drive_task_driving_style_safe(
        
        
            ped: 
        , 
        
        
            drivingStyle: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xDACE1BE37D88AF67u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xDACE1BE37D88AF67u64;
        
        let result = invoke_raw!(
            hash,
                ped, 
                drivingStyle
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn set_drive_task_driving_style_raw(
        ped: , 
        drivingStyle: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xDACE1BE37D88AF67u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xDACE1BE37D88AF67u64;

        invoke_raw_typed!(
            hash,
                ped, 
                drivingStyle
        )
    }
}

/// ## Parameters
*



pub fn set_ped_waypoint_route_offset_safe(
        
        
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
        let hash = 0xED98E10B0AFCE4B4u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xED98E10B0AFCE4B4u64;
        
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
pub fn set_ped_waypoint_route_offset_raw(
        p0: , 
        p1: , 
        p2: , 
        p3: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xED98E10B0AFCE4B4u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xED98E10B0AFCE4B4u64;

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



pub fn update_task_aim_gun_scripted_target_safe(
        
        
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
        let hash = 0x9724FB59A3E72AD0u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x9724FB59A3E72AD0u64;
        
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
pub fn update_task_aim_gun_scripted_target_raw(
        p0: , 
        p1: , 
        p2: , 
        p3: , 
        p4: , 
        p5: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x9724FB59A3E72AD0u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x9724FB59A3E72AD0u64;

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



pub fn task_combat_hated_targets_around_ped_timed_safe(
        
        
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
        let hash = 0x2BBA30B854534A0Cu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x2BBA30B854534A0Cu64;
        
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
pub fn task_combat_hated_targets_around_ped_timed_raw(
        p0: , 
        p1: , 
        p2: , 
        p3: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x2BBA30B854534A0Cu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x2BBA30B854534A0Cu64;

        invoke_raw_typed!(
            hash,
                p0, 
                p1, 
                p2, 
                p3
        )
    }
}

/// x2,y2 and z2 are the coordinates to which the ped should look at



pub fn add_patrol_route_node_safe(
        
        
            id: 
        , 
        
        
            guardScenario: 
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
        
        
            waitTime: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x8EDF950167586B7Cu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x8EDF950167586B7Cu64;
        
        let result = invoke_raw!(
            hash,
                id, 
                guardScenario, 
                x1, 
                y1, 
                z1, 
                x2, 
                y2, 
                z2, 
                waitTime
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn add_patrol_route_node_raw(
        id: , 
        guardScenario: , 
        x1: , 
        y1: , 
        z1: , 
        x2: , 
        y2: , 
        z2: , 
        waitTime: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x8EDF950167586B7Cu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x8EDF950167586B7Cu64;

        invoke_raw_typed!(
            hash,
                id, 
                guardScenario, 
                x1, 
                y1, 
                z1, 
                x2, 
                y2, 
                z2, 
                waitTime
        )
    }
}

/// For an example on how to use this please refer to [OPEN_SEQUENCE_TASK](#_0xE8854A4326B9E12B)



pub fn close_sequence_task_safe(
        
        
            taskSequenceId: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x39E72BC99E6360CBu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x39E72BC99E6360CBu64;
        
        let result = invoke_raw!(
            hash,
                taskSequenceId
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn close_sequence_task_raw(
        taskSequenceId: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x39E72BC99E6360CBu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x39E72BC99E6360CBu64;

        invoke_raw_typed!(
            hash,
                taskSequenceId
        )
    }
}

/// The ped will act like NPC's involved in a gunfight. The ped will squat down with their heads held in place and look around.



pub fn task_cower_safe(
        
        
            ped: 
        , 
        
        
            duration: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x3EB1FE9E8E908E15u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x3EB1FE9E8E908E15u64;
        
        let result = invoke_raw!(
            hash,
                ped, 
                duration
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn task_cower_raw(
        ped: , 
        duration: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x3EB1FE9E8E908E15u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x3EB1FE9E8E908E15u64;

        invoke_raw_typed!(
            hash,
                ped, 
                duration
        )
    }
}

/// ```
I cant believe I have to define this, this is one of the best natives.  
It makes the ped ignore basically all shocking events around it. Occasionally the ped may comment or gesture, but other than that they just continue their daily activities. This includes shooting and wounding the ped. And - most importantly - they do not flee.  
Since it is a task, every time the native is called the ped will stop for a moment.  
```



pub fn task_set_blocking_of_non_temporary_events_safe(
        
        
            ped: 
        , 
        
        
            toggle: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x90D2156198831D69u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x90D2156198831D69u64;
        
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
pub fn task_set_blocking_of_non_temporary_events_raw(
        ped: , 
        toggle: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x90D2156198831D69u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x90D2156198831D69u64;

        invoke_raw_typed!(
            hash,
                ped, 
                toggle
        )
    }
}

/// ## Parameters
*



pub fn is_ped_still_safe(
        
        
            ped: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xAC29253EEF8F0180u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xAC29253EEF8F0180u64;
        
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
pub fn is_ped_still_raw(
        ped: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xAC29253EEF8F0180u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xAC29253EEF8F0180u64;

        invoke_raw_typed!(
            hash,
                ped
        )
    }
}

/// ```
Not clear what it actually does, but here's how script uses it -
if (OBJECT::HAS_PICKUP_BEEN_COLLECTED(...)
{
 if(ENTITY::DOES_ENTITY_EXIST(PLAYER::PLAYER_PED_ID()))
    {
     TASK::TASK_CLEAR_LOOK_AT(PLAYER::PLAYER_PED_ID());
  }
 ...
}
Another one where it doesn't "look" at current player -
TASK::TASK_PLAY_ANIM(l_3ED, "missheist_agency2aig_2", "look_at_phone_a", 1000.0, -2.0, -1, 48, v_2, 0, 0, 0);
PED::_2208438012482A1A(l_3ED, 0, 0);
TASK::TASK_CLEAR_LOOK_AT(l_3ED);
```



pub fn task_clear_look_at_safe(
        
        
            ped: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x0F804F1DB19B9689u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x0F804F1DB19B9689u64;
        
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
pub fn task_clear_look_at_raw(
        ped: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x0F804F1DB19B9689u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x0F804F1DB19B9689u64;

        invoke_raw_typed!(
            hash,
                ped
        )
    }
}

/// ```
For p1 & p2 (Ped, Vehicle). I could be wrong, as the only time this native is called in scripts is once and both are 0, but I assume this native will work like SET_MOUNTED_WEAPON_TARGET in which has the same exact amount of parameters and the 1st and last 3 parameters are right and the same for both natives.  
```



pub fn set_driveby_task_target_safe(
        
        
            shootingPed: 
        , 
        
        
            targetPed: 
        , 
        
        
            targetVehicle: 
        , 
        
        
            x: 
        , 
        
        
            y: 
        , 
        
        
            z: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xE5B302114D8162EEu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xE5B302114D8162EEu64;
        
        let result = invoke_raw!(
            hash,
                shootingPed, 
                targetPed, 
                targetVehicle, 
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
pub fn set_driveby_task_target_raw(
        shootingPed: , 
        targetPed: , 
        targetVehicle: , 
        x: , 
        y: , 
        z: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xE5B302114D8162EEu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xE5B302114D8162EEu64;

        invoke_raw_typed!(
            hash,
                shootingPed, 
                targetPed, 
                targetVehicle, 
                x, 
                y, 
                z
        )
    }
}

/// ```
From fm_mission_controller.c:  
reserve_network_mission_objects(get_num_reserved_mission_objects(0) + 1);  
	vVar28 = {0.094f, 0.02f, -0.005f};  
	vVar29 = {-92.24f, 63.64f, 150.24f};  
	func_253(&uVar30, joaat("prop_ld_case_01"), Global_1592429.imm_34757[iParam1 <268>], 1, 1, 0, 1);  
	set_entity_lod_dist(net_to_ent(uVar30), 500);  
	attach_entity_to_entity(net_to_ent(uVar30), iParam0, get_ped_bone_index(iParam0, 28422), vVar28, vVar29, 1, 0, 0, 0, 2, 1);  
	Var31.imm_4 = 1065353216;  
	Var31.imm_5 = 1065353216;  
	Var31.imm_9 = 1065353216;  
	Var31.imm_10 = 1065353216;  
	Var31.imm_14 = 1065353216;  
	Var31.imm_15 = 1065353216;  
	Var31.imm_17 = 1040187392;  
	Var31.imm_18 = 1040187392;  
	Var31.imm_19 = -1;  
	Var32.imm_4 = 1065353216;  
	Var32.imm_5 = 1065353216;  
	Var32.imm_9 = 1065353216;  
	Var32.imm_10 = 1065353216;  
	Var32.imm_14 = 1065353216;  
	Var32.imm_15 = 1065353216;  
	Var32.imm_17 = 1040187392;  
	Var32.imm_18 = 1040187392;  
	Var32.imm_19 = -1;  
	Var31 = 1;  
	Var31.imm_1 = "weapons@misc@jerrycan@mp_male";  
	Var31.imm_2 = "idle";  
	Var31.imm_20 = 1048633;  
	Var31.imm_4 = 0.5f;  
	Var31.imm_16 = get_hash_key("BONEMASK_ARMONLY_R");  
	task_scripted_animation(iParam0, &Var31, &Var32, &Var32, 0f, 0.25f);  
	set_model_as_no_longer_needed(joaat("prop_ld_case_01"));  
	remove_anim_dict("anim@heists@biolab@");  
```



pub fn task_scripted_animation_safe(
        
        
            ped: 
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
        let hash = 0x126EF75F1E17ABE5u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x126EF75F1E17ABE5u64;
        
        let result = invoke_raw!(
            hash,
                ped, 
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
pub fn task_scripted_animation_raw(
        ped: , 
        p1: , 
        p2: , 
        p3: , 
        p4: , 
        p5: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x126EF75F1E17ABE5u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x126EF75F1E17ABE5u64;

        invoke_raw_typed!(
            hash,
                ped, 
                p1, 
                p2, 
                p3, 
                p4, 
                p5
        )
    }
}

/// This is a stricter version of [`IS_PED_USING_ANY_SCENARIO`](#_0x57AB4A3080F85143). It only returns true if the ped is playing the ambient animations associated with the scenario.



pub fn is_ped_active_in_scenario_safe(
        
        
            ped: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xAA135F9482C82CC3u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xAA135F9482C82CC3u64;
        
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
pub fn is_ped_active_in_scenario_raw(
        ped: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xAA135F9482C82CC3u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xAA135F9482C82CC3u64;

        invoke_raw_typed!(
            hash,
                ped
        )
    }
}

/// RESET_SCENARIO_GROUPS_ENABLED native function



pub fn reset_scenario_groups_enabled_safe(
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xDD902D0349AFAD3Au32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xDD902D0349AFAD3Au64;
        
        let result = invoke_raw!(
            hash,
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn reset_scenario_groups_enabled_raw(
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xDD902D0349AFAD3Au32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xDD902D0349AFAD3Au64;

        invoke_raw_typed!(
            hash,
        )
    }
}

/// ```
Makes the specified ped flee the specified distance from the specified position.  
```



pub fn task_smart_flee_coord_safe(
        
        
            ped: 
        , 
        
        
            x: 
        , 
        
        
            y: 
        , 
        
        
            z: 
        , 
        
        
            distance: 
        , 
        
        
            time: 
        , 
        
        
            p6: 
        , 
        
        
            p7: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x94587F17E9C365D5u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x94587F17E9C365D5u64;
        
        let result = invoke_raw!(
            hash,
                ped, 
                x, 
                y, 
                z, 
                distance, 
                time, 
                p6, 
                p7
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn task_smart_flee_coord_raw(
        ped: , 
        x: , 
        y: , 
        z: , 
        distance: , 
        time: , 
        p6: , 
        p7: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x94587F17E9C365D5u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x94587F17E9C365D5u64;

        invoke_raw_typed!(
            hash,
                ped, 
                x, 
                y, 
                z, 
                distance, 
                time, 
                p6, 
                p7
        )
    }
}

/// The ped will move or warp to the position and heading given, then start the scenario passed. See [`TASK_START_SCENARIO_IN_PLACE`](#_0x142A02425FF02BD9) for a list of scenarios.



pub fn task_start_scenario_at_position_safe(
        
        
            ped: 
        , 
        
        
            scenarioName: 
        , 
        
        
            x: 
        , 
        
        
            y: 
        , 
        
        
            z: 
        , 
        
        
            heading: 
        , 
        
        
            timeToLeave: 
        , 
        
        
            playIntro: 
        , 
        
        
            warp: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xFA4EFC79F69D4F07u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xFA4EFC79F69D4F07u64;
        
        let result = invoke_raw!(
            hash,
                ped, 
                scenarioName, 
                x, 
                y, 
                z, 
                heading, 
                timeToLeave, 
                playIntro, 
                warp
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn task_start_scenario_at_position_raw(
        ped: , 
        scenarioName: , 
        x: , 
        y: , 
        z: , 
        heading: , 
        timeToLeave: , 
        playIntro: , 
        warp: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xFA4EFC79F69D4F07u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xFA4EFC79F69D4F07u64;

        invoke_raw_typed!(
            hash,
                ped, 
                scenarioName, 
                x, 
                y, 
                z, 
                heading, 
                timeToLeave, 
                playIntro, 
                warp
        )
    }
}

/// ## Parameters
*



pub fn task_go_to_entity_while_aiming_at_coord_safe(
        
        
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
        , 
        
        
            p10: 
        , 
        
        
            p11: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x04701832B739DCE5u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x04701832B739DCE5u64;
        
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
                p9, 
                p10, 
                p11
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn task_go_to_entity_while_aiming_at_coord_raw(
        p0: , 
        p1: , 
        p2: , 
        p3: , 
        p4: , 
        p5: , 
        p6: , 
        p7: , 
        p8: , 
        p9: , 
        p10: , 
        p11: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x04701832B739DCE5u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x04701832B739DCE5u64;

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
                p9, 
                p10, 
                p11
        )
    }
}

/// ## Parameters
*



pub fn task_use_nearest_scenario_chain_to_coord_safe(
        
        
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
        let hash = 0x9FDA1B3D7E7028B3u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x9FDA1B3D7E7028B3u64;
        
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
pub fn task_use_nearest_scenario_chain_to_coord_raw(
        p0: , 
        p1: , 
        p2: , 
        p3: , 
        p4: , 
        p5: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x9FDA1B3D7E7028B3u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x9FDA1B3D7E7028B3u64;

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



pub fn task_go_straight_to_coord_relative_to_entity_safe(
        
        
            entity1: 
        , 
        
        
            entity2: 
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
        let hash = 0x61E360B7E040D12Eu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x61E360B7E040D12Eu64;
        
        let result = invoke_raw!(
            hash,
                entity1, 
                entity2, 
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
pub fn task_go_straight_to_coord_relative_to_entity_raw(
        entity1: , 
        entity2: , 
        p2: , 
        p3: , 
        p4: , 
        p5: , 
        p6: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x61E360B7E040D12Eu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x61E360B7E040D12Eu64;

        invoke_raw_typed!(
            hash,
                entity1, 
                entity2, 
                p2, 
                p3, 
                p4, 
                p5, 
                p6
        )
    }
}

/// ```
p6 always -1  
p7 always 10.0  
p8 always 1  
```



pub fn task_follow_to_offset_of_entity_safe(
        
        
            ped: 
        , 
        
        
            entity: 
        , 
        
        
            offsetX: 
        , 
        
        
            offsetY: 
        , 
        
        
            offsetZ: 
        , 
        
        
            movementSpeed: 
        , 
        
        
            timeout: 
        , 
        
        
            stoppingRange: 
        , 
        
        
            persistFollowing: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x304AE42E357B8C7Eu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x304AE42E357B8C7Eu64;
        
        let result = invoke_raw!(
            hash,
                ped, 
                entity, 
                offsetX, 
                offsetY, 
                offsetZ, 
                movementSpeed, 
                timeout, 
                stoppingRange, 
                persistFollowing
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn task_follow_to_offset_of_entity_raw(
        ped: , 
        entity: , 
        offsetX: , 
        offsetY: , 
        offsetZ: , 
        movementSpeed: , 
        timeout: , 
        stoppingRange: , 
        persistFollowing: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x304AE42E357B8C7Eu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x304AE42E357B8C7Eu64;

        invoke_raw_typed!(
            hash,
                ped, 
                entity, 
                offsetX, 
                offsetY, 
                offsetZ, 
                movementSpeed, 
                timeout, 
                stoppingRange, 
                persistFollowing
        )
    }
}

/// ## Parameters
*



pub fn is_scenario_occupied_safe(
        
        
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
        let hash = 0x788756D73AC2E07Cu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x788756D73AC2E07Cu64;
        
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
pub fn is_scenario_occupied_raw(
        p0: , 
        p1: , 
        p2: , 
        p3: , 
        p4: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x788756D73AC2E07Cu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x788756D73AC2E07Cu64;

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



pub fn _set_task_move_network_signal_float_2_safe(
        
        
            ped: 
        , 
        
        
            signalName: 
        , 
        
        
            value: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x373EF409B82697A3u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x373EF409B82697A3u64;
        
        let result = invoke_raw!(
            hash,
                ped, 
                signalName, 
                value
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn _set_task_move_network_signal_float_2_raw(
        ped: , 
        signalName: , 
        value: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x373EF409B82697A3u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x373EF409B82697A3u64;

        invoke_raw_typed!(
            hash,
                ped, 
                signalName, 
                value
        )
    }
}

/// For an example on how to use this please refer to [OPEN_SEQUENCE_TASK](#_0xE8854A4326B9E12B



pub fn task_perform_sequence_locally_safe(
        
        
            ped: 
        , 
        
        
            taskSequenceId: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x8C33220C8D78CA0Du32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x8C33220C8D78CA0Du64;
        
        let result = invoke_raw!(
            hash,
                ped, 
                taskSequenceId
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn task_perform_sequence_locally_raw(
        ped: , 
        taskSequenceId: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x8C33220C8D78CA0Du32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x8C33220C8D78CA0Du64;

        invoke_raw_typed!(
            hash,
                ped, 
                taskSequenceId
        )
    }
}

/// ```
https://alloc8or.re/gta5/doc/enums/eVehicleMissionType.txt
```



pub fn get_active_vehicle_mission_type_safe(
        
        
            vehicle: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x534AEBA6E5ED4CABu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x534AEBA6E5ED4CABu64;
        
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
pub fn get_active_vehicle_mission_type_raw(
        vehicle: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x534AEBA6E5ED4CABu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x534AEBA6E5ED4CABu64;

        invoke_raw_typed!(
            hash,
                vehicle
        )
    }
}

/// ## Parameters
*



pub fn get_phone_gesture_anim_total_time_safe(
        
        
            ped: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x1EE0F68A7C25DEC6u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x1EE0F68A7C25DEC6u64;
        
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
pub fn get_phone_gesture_anim_total_time_raw(
        ped: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x1EE0F68A7C25DEC6u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x1EE0F68A7C25DEC6u64;

        invoke_raw_typed!(
            hash,
                ped
        )
    }
}

/// Flags are the same flags used in [`TASK_LEAVE_VEHICLE`](#_0xD3DBCE61A490BE02)



pub fn task_leave_any_vehicle_safe(
        
        
            ped: 
        , 
        
        
            p1: 
        , 
        
        
            flags: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x504D54DF3F6F2247u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x504D54DF3F6F2247u64;
        
        let result = invoke_raw!(
            hash,
                ped, 
                p1, 
                flags
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn task_leave_any_vehicle_raw(
        ped: , 
        p1: , 
        flags: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x504D54DF3F6F2247u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x504D54DF3F6F2247u64;

        invoke_raw_typed!(
            hash,
                ped, 
                p1, 
                flags
        )
    }
}

/// ## Parameters
*



pub fn add_cover_blocking_area_safe(
        
        
            playerX: 
        , 
        
        
            playerY: 
        , 
        
        
            playerZ: 
        , 
        
        
            radiusX: 
        , 
        
        
            radiusY: 
        , 
        
        
            radiusZ: 
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
        let hash = 0x45C597097DD7CB81u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x45C597097DD7CB81u64;
        
        let result = invoke_raw!(
            hash,
                playerX, 
                playerY, 
                playerZ, 
                radiusX, 
                radiusY, 
                radiusZ, 
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
pub fn add_cover_blocking_area_raw(
        playerX: , 
        playerY: , 
        playerZ: , 
        radiusX: , 
        radiusY: , 
        radiusZ: , 
        p6: , 
        p7: , 
        p8: , 
        p9: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x45C597097DD7CB81u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x45C597097DD7CB81u64;

        invoke_raw_typed!(
            hash,
                playerX, 
                playerY, 
                playerZ, 
                radiusX, 
                radiusY, 
                radiusZ, 
                p6, 
                p7, 
                p8, 
                p9
        )
    }
}

/// Will make the ped move to a coordinate while aiming (and optionally shooting) at given coordinates.



pub fn task_go_to_coord_while_aiming_at_coord_safe(
        
        
            ped: 
        , 
        
        
            x: 
        , 
        
        
            y: 
        , 
        
        
            z: 
        , 
        
        
            aimAtX: 
        , 
        
        
            aimAtY: 
        , 
        
        
            aimAtZ: 
        , 
        
        
            moveSpeed: 
        , 
        
        
            shoot: 
        , 
        
        
            p9: 
        , 
        
        
            p10: 
        , 
        
        
            p11: 
        , 
        
        
            flags: 
        , 
        
        
            p13: 
        , 
        
        
            firingPattern: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x11315AB3385B8AC0u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x11315AB3385B8AC0u64;
        
        let result = invoke_raw!(
            hash,
                ped, 
                x, 
                y, 
                z, 
                aimAtX, 
                aimAtY, 
                aimAtZ, 
                moveSpeed, 
                shoot, 
                p9, 
                p10, 
                p11, 
                flags, 
                p13, 
                firingPattern
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn task_go_to_coord_while_aiming_at_coord_raw(
        ped: , 
        x: , 
        y: , 
        z: , 
        aimAtX: , 
        aimAtY: , 
        aimAtZ: , 
        moveSpeed: , 
        shoot: , 
        p9: , 
        p10: , 
        p11: , 
        flags: , 
        p13: , 
        firingPattern: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x11315AB3385B8AC0u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x11315AB3385B8AC0u64;

        invoke_raw_typed!(
            hash,
                ped, 
                x, 
                y, 
                z, 
                aimAtX, 
                aimAtY, 
                aimAtZ, 
                moveSpeed, 
                shoot, 
                p9, 
                p10, 
                p11, 
                flags, 
                p13, 
                firingPattern
        )
    }
}

/// ## Parameters
*



pub fn update_task_sweep_aim_entity_safe(
        
        
            ped: 
        , 
        
        
            entity: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xE4973DBDBE6E44B3u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xE4973DBDBE6E44B3u64;
        
        let result = invoke_raw!(
            hash,
                ped, 
                entity
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn update_task_sweep_aim_entity_raw(
        ped: , 
        entity: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xE4973DBDBE6E44B3u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xE4973DBDBE6E44B3u64;

        invoke_raw_typed!(
            hash,
                ped, 
                entity
        )
    }
}

/// ```
NativeDB Added Parameter 5: Any p4
NativeDB Added Parameter 6: Any p5
```



pub fn task_writhe_safe(
        
        
            ped: 
        , 
        
        
            target: 
        , 
        
        
            time: 
        , 
        
        
            p3: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xCDDC2B77CE54AC6Eu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xCDDC2B77CE54AC6Eu64;
        
        let result = invoke_raw!(
            hash,
                ped, 
                target, 
                time, 
                p3
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn task_writhe_raw(
        ped: , 
        target: , 
        time: , 
        p3: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xCDDC2B77CE54AC6Eu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xCDDC2B77CE54AC6Eu64;

        invoke_raw_typed!(
            hash,
                ped, 
                target, 
                time, 
                p3
        )
    }
}

/// ## Parameters
*



pub fn is_ped_getting_up_safe(
        
        
            ped: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x2A74E1D5F2F00EECu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x2A74E1D5F2F00EECu64;
        
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
pub fn is_ped_getting_up_raw(
        ped: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x2A74E1D5F2F00EECu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x2A74E1D5F2F00EECu64;

        invoke_raw_typed!(
            hash,
                ped
        )
    }
}

/// Makes a shark ped circle around specified coordinates.

```
NativeDB Introduced: v3407
```



pub fn task_shark_circle_coord_safe(
        
        
            ped: 
        , 
        
        
            x: 
        , 
        
        
            y: 
        , 
        
        
            z: 
        , 
        
        
            moveBlendRatio: 
        , 
        
        
            radius: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x60A19CF85FF4CEFAu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x60A19CF85FF4CEFAu64;
        
        let result = invoke_raw!(
            hash,
                ped, 
                x, 
                y, 
                z, 
                moveBlendRatio, 
                radius
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn task_shark_circle_coord_raw(
        ped: , 
        x: , 
        y: , 
        z: , 
        moveBlendRatio: , 
        radius: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x60A19CF85FF4CEFAu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x60A19CF85FF4CEFAu64;

        invoke_raw_typed!(
            hash,
                ped, 
                x, 
                y, 
                z, 
                moveBlendRatio, 
                radius
        )
    }
}

/// ## Parameters
*



pub fn vehicle_waypoint_playback_pause_safe(
        
        
            vehicle: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x8A4E6AC373666BC5u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x8A4E6AC373666BC5u64;
        
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
pub fn vehicle_waypoint_playback_pause_raw(
        vehicle: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x8A4E6AC373666BC5u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x8A4E6AC373666BC5u64;

        invoke_raw_typed!(
            hash,
                vehicle
        )
    }
}

/// ```
duration: the amount of time in milliseconds to do the task. -1 will keep the task going until either another task is applied, or CLEAR_ALL_TASKS() is called with the ped  
```



pub fn task_turn_ped_to_face_entity_safe(
        
        
            ped: 
        , 
        
        
            entity: 
        , 
        
        
            duration: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x5AD23D40115353ACu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x5AD23D40115353ACu64;
        
        let result = invoke_raw!(
            hash,
                ped, 
                entity, 
                duration
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn task_turn_ped_to_face_entity_raw(
        ped: , 
        entity: , 
        duration: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x5AD23D40115353ACu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x5AD23D40115353ACu64;

        invoke_raw_typed!(
            hash,
                ped, 
                entity, 
                duration
        )
    }
}

/// Gives the vehicle a temporary action.



pub fn task_vehicle_temp_action_safe(
        
        
            driver: 
        , 
        
        
            vehicle: 
        , 
        
        
            action: 
        , 
        
        
            time: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xC429DCEEB339E129u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xC429DCEEB339E129u64;
        
        let result = invoke_raw!(
            hash,
                driver, 
                vehicle, 
                action, 
                time
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn task_vehicle_temp_action_raw(
        driver: , 
        vehicle: , 
        action: , 
        time: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xC429DCEEB339E129u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xC429DCEEB339E129u64;

        invoke_raw_typed!(
            hash,
                driver, 
                vehicle, 
                action, 
                time
        )
    }
}

/// ```
from michael2:
TASK::TASK_SEEK_COVER_TO_COORDS(ped, 967.5164794921875, -2121.603515625, 30.479299545288086, 978.94677734375, -2125.84130859375, 29.4752, -1, 1);
appears to be shorter variation
from michael3:
TASK::TASK_SEEK_COVER_TO_COORDS(ped, -2231.011474609375, 263.6326599121094, 173.60195922851562, -1, 0);
```



pub fn task_seek_cover_to_coords_safe(
        
        
            ped: 
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
        
        
            p7: 
        , 
        
        
            p8: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x39246A6958EF072Cu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x39246A6958EF072Cu64;
        
        let result = invoke_raw!(
            hash,
                ped, 
                x1, 
                y1, 
                z1, 
                x2, 
                y2, 
                z2, 
                p7, 
                p8
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn task_seek_cover_to_coords_raw(
        ped: , 
        x1: , 
        y1: , 
        z1: , 
        x2: , 
        y2: , 
        z2: , 
        p7: , 
        p8: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x39246A6958EF072Cu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x39246A6958EF072Cu64;

        invoke_raw_typed!(
            hash,
                ped, 
                x1, 
                y1, 
                z1, 
                x2, 
                y2, 
                z2, 
                p7, 
                p8
        )
    }
}

/// ```
Climbs or vaults the nearest thing.  
```



pub fn task_climb_safe(
        
        
            ped: 
        , 
        
        
            unused: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x89D9FCC2435112F1u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x89D9FCC2435112F1u64;
        
        let result = invoke_raw!(
            hash,
                ped, 
                unused
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn task_climb_raw(
        ped: , 
        unused: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x89D9FCC2435112F1u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x89D9FCC2435112F1u64;

        invoke_raw_typed!(
            hash,
                ped, 
                unused
        )
    }
}

/// ## Parameters
*



pub fn assisted_movement_is_route_loaded_safe(
        
        
            route: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x60F9A4393A21F741u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x60F9A4393A21F741u64;
        
        let result = invoke_raw!(
            hash,
                route
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn assisted_movement_is_route_loaded_raw(
        route: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x60F9A4393A21F741u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x60F9A4393A21F741u64;

        invoke_raw_typed!(
            hash,
                route
        )
    }
}

/// ```
NativeDB Introduced: v2372
```



pub fn set_anim_phase_safe(
        
        
            entity: 
        , 
        
        
            p1: 
        , 
        
        
            p2: 
        , 
        
        
            p3: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xDDF3CB5A0A4C0B49u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xDDF3CB5A0A4C0B49u64;
        
        let result = invoke_raw!(
            hash,
                entity, 
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
pub fn set_anim_phase_raw(
        entity: , 
        p1: , 
        p2: , 
        p3: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xDDF3CB5A0A4C0B49u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xDDF3CB5A0A4C0B49u64;

        invoke_raw_typed!(
            hash,
                entity, 
                p1, 
                p2, 
                p3
        )
    }
}

/// ```
Differs from TASK_VEHICLE_DRIVE_TO_COORDS in that it will pick the shortest possible road route without taking one-way streets and other "road laws" into consideration.  
WARNING:  
A behaviorFlag value of 0 will result in a clunky, stupid driver!  
Recommended settings:  
speed = 30.0f,  
behaviorFlag = 156,   
stoppingRange = 5.0f;  
If you simply want to have your driver move to a fixed location, call it only once, or, when necessary in the event of interruption.   
If using this to continually follow a Ped who is on foot:  You will need to run this in a tick loop.  Call it in with the Ped's updated coordinates every 20 ticks or so and you will have one hell of a smart, fast-reacting NPC driver -- provided he doesn't get stuck.  If your update frequency is too fast, the Ped may not have enough time to figure his way out of being stuck, and thus, remain stuck.  One way around this would be to implement an "anti-stuck" mechanism, which allows the driver to realize he's stuck, temporarily pause the tick, unstuck, then resume the tick.  
EDIT:  This is being discussed in more detail at http://gtaforums.com/topic/818504-any-idea-on-how-to-make-peds-clever-and-insanely-fast-c/  
```



pub fn task_vehicle_goto_navmesh_safe(
        
        
            ped: 
        , 
        
        
            vehicle: 
        , 
        
        
            x: 
        , 
        
        
            y: 
        , 
        
        
            z: 
        , 
        
        
            speed: 
        , 
        
        
            behaviorFlag: 
        , 
        
        
            stoppingRange: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x195AEEB13CEFE2EEu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x195AEEB13CEFE2EEu64;
        
        let result = invoke_raw!(
            hash,
                ped, 
                vehicle, 
                x, 
                y, 
                z, 
                speed, 
                behaviorFlag, 
                stoppingRange
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn task_vehicle_goto_navmesh_raw(
        ped: , 
        vehicle: , 
        x: , 
        y: , 
        z: , 
        speed: , 
        behaviorFlag: , 
        stoppingRange: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x195AEEB13CEFE2EEu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x195AEEB13CEFE2EEu64;

        invoke_raw_typed!(
            hash,
                ped, 
                vehicle, 
                x, 
                y, 
                z, 
                speed, 
                behaviorFlag, 
                stoppingRange
        )
    }
}

/// ## Parameters
*



pub fn is_ped_playing_base_clip_in_scenario_safe(
        
        
            ped: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x621C6E4729388E41u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x621C6E4729388E41u64;
        
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
pub fn is_ped_playing_base_clip_in_scenario_raw(
        ped: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x621C6E4729388E41u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x621C6E4729388E41u64;

        invoke_raw_typed!(
            hash,
                ped
        )
    }
}

/// Immediately stops the pedestrian from whatever it's doing. The difference between this and [CLEAR_PED_TASKS](#_0xE1EF3C1216AFF2CD) is that this one teleports the ped but does not change the position of the ped.



pub fn clear_ped_tasks_immediately_safe(
        
        
            ped: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xAAA34F8A7CB32098u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xAAA34F8A7CB32098u64;
        
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
pub fn clear_ped_tasks_immediately_raw(
        ped: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xAAA34F8A7CB32098u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xAAA34F8A7CB32098u64;

        invoke_raw_typed!(
            hash,
                ped
        )
    }
}

/// ```
Used only once in the scripts (fm_mission_controller) like so:

TASK::_0xAB13A5565480B6D9(iLocal_3160, "Cutting");

SET_*
```



pub fn _0xab13a5565480b6d9_safe(
        
        
            ped: 
        , 
        
        
            p1: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xAB13A5565480B6D9u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xAB13A5565480B6D9u64;
        
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
pub fn _0xab13a5565480b6d9_raw(
        ped: , 
        p1: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xAB13A5565480B6D9u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xAB13A5565480B6D9u64;

        invoke_raw_typed!(
            hash,
                ped, 
                p1
        )
    }
}

/// ```
duration: the amount of time in milliseconds to do the task.  -1 will keep the task going until either another task is applied, or CLEAR_ALL_TASKS() is called with the ped  
```



pub fn task_aim_gun_at_entity_safe(
        
        
            ped: 
        , 
        
        
            entity: 
        , 
        
        
            duration: 
        , 
        
        
            bInstantBlendToAim: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x9B53BB6E8943AF53u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x9B53BB6E8943AF53u64;
        
        let result = invoke_raw!(
            hash,
                ped, 
                entity, 
                duration, 
                bInstantBlendToAim
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn task_aim_gun_at_entity_raw(
        ped: , 
        entity: , 
        duration: , 
        bInstantBlendToAim: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x9B53BB6E8943AF53u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x9B53BB6E8943AF53u64;

        invoke_raw_typed!(
            hash,
                ped, 
                entity, 
                duration, 
                bInstantBlendToAim
        )
    }
}

/// ## Parameters
*



pub fn ped_has_use_scenario_task_safe(
        
        
            ped: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x295E3CCEC879CCD7u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x295E3CCEC879CCD7u64;
        
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
pub fn ped_has_use_scenario_task_raw(
        ped: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x295E3CCEC879CCD7u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x295E3CCEC879CCD7u64;

        invoke_raw_typed!(
            hash,
                ped
        )
    }
}

/// ## Parameters
*



pub fn is_playing_phone_gesture_anim_safe(
        
        
            ped: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xB8EBB1E9D3588C10u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xB8EBB1E9D3588C10u64;
        
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
pub fn is_playing_phone_gesture_anim_raw(
        ped: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xB8EBB1E9D3588C10u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xB8EBB1E9D3588C10u64;

        invoke_raw_typed!(
            hash,
                ped
        )
    }
}

/// ```
eg
 TASK::TASK_GOTO_ENTITY_AIMING(v_2, PLAYER::PLAYER_PED_ID(), 5.0, 25.0);
ped = Ped you want to perform this task.
target = the Entity they should aim at.
distanceToStopAt = distance from the target, where the ped should stop to aim.
StartAimingDist = distance where the ped should start to aim.
```



pub fn task_goto_entity_aiming_safe(
        
        
            ped: 
        , 
        
        
            target: 
        , 
        
        
            distanceToStopAt: 
        , 
        
        
            StartAimingDist: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xA9DA48FAB8A76C12u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xA9DA48FAB8A76C12u64;
        
        let result = invoke_raw!(
            hash,
                ped, 
                target, 
                distanceToStopAt, 
                StartAimingDist
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn task_goto_entity_aiming_raw(
        ped: , 
        target: , 
        distanceToStopAt: , 
        StartAimingDist: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xA9DA48FAB8A76C12u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xA9DA48FAB8A76C12u64;

        invoke_raw_typed!(
            hash,
                ped, 
                target, 
                distanceToStopAt, 
                StartAimingDist
        )
    }
}

/// ```
Default modifier is 1.0, minimum is 0.0 and maximum is 10.0.
```



pub fn set_ped_path_climb_cost_modifier_safe(
        
        
            ped: 
        , 
        
        
            modifier: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x88E32DB8C1A4AA4Bu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x88E32DB8C1A4AA4Bu64;
        
        let result = invoke_raw!(
            hash,
                ped, 
                modifier
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn set_ped_path_climb_cost_modifier_raw(
        ped: , 
        modifier: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x88E32DB8C1A4AA4Bu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x88E32DB8C1A4AA4Bu64;

        invoke_raw_typed!(
            hash,
                ped, 
                modifier
        )
    }
}

/// ## Parameters
*



pub fn task_aim_gun_scripted_with_target_safe(
        
        
            ped: 
        , 
        
        
            targetPed: 
        , 
        
        
            x: 
        , 
        
        
            y: 
        , 
        
        
            z: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x8605AF0DE8B3A5ACu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x8605AF0DE8B3A5ACu64;
        
        let result = invoke_raw!(
            hash,
                ped, 
                targetPed, 
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
pub fn task_aim_gun_scripted_with_target_raw(
        ped: , 
        targetPed: , 
        x: , 
        y: , 
        z: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x8605AF0DE8B3A5ACu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x8605AF0DE8B3A5ACu64;

        invoke_raw_typed!(
            hash,
                ped, 
                targetPed, 
                x, 
                y, 
                z
        )
    }
}

/// ```
duration in milliseconds  
```



pub fn task_turn_ped_to_face_coord_safe(
        
        
            ped: 
        , 
        
        
            x: 
        , 
        
        
            y: 
        , 
        
        
            z: 
        , 
        
        
            duration: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x1DDA930A0AC38571u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x1DDA930A0AC38571u64;
        
        let result = invoke_raw!(
            hash,
                ped, 
                x, 
                y, 
                z, 
                duration
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn task_turn_ped_to_face_coord_raw(
        ped: , 
        x: , 
        y: , 
        z: , 
        duration: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x1DDA930A0AC38571u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x1DDA930A0AC38571u64;

        invoke_raw_typed!(
            hash,
                ped, 
                x, 
                y, 
                z, 
                duration
        )
    }
}

/// ```
NativeDB Introduced: v1493
```



pub fn _0x8423541e8b3a1589_safe(
        
        
            p0: 
        , 
        
        
            p1: 
        , 
        
        
            p2: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x8423541E8B3A1589u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x8423541E8B3A1589u64;
        
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
pub fn _0x8423541e8b3a1589_raw(
        p0: , 
        p1: , 
        p2: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x8423541E8B3A1589u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x8423541E8B3A1589u64;

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



pub fn is_driveby_task_underneath_driving_task_safe(
        
        
            ped: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x8785E6E40C7A8818u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x8785E6E40C7A8818u64;
        
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
pub fn is_driveby_task_underneath_driving_task_raw(
        ped: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x8785E6E40C7A8818u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x8785E6E40C7A8818u64;

        invoke_raw_typed!(
            hash,
                ped
        )
    }
}

/// ```
Looks like p1 may be a flag, still need to do some research, though.
```



pub fn stop_anim_playback_safe(
        
        
            ped: 
        , 
        
        
            p1: 
        , 
        
        
            p2: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xEE08C992D238C5D1u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xEE08C992D238C5D1u64;
        
        let result = invoke_raw!(
            hash,
                ped, 
                p1, 
                p2
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn stop_anim_playback_raw(
        ped: , 
        p1: , 
        p2: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xEE08C992D238C5D1u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xEE08C992D238C5D1u64;

        invoke_raw_typed!(
            hash,
                ped, 
                p1, 
                p2
        )
    }
}

/// ## Parameters
*



pub fn is_ped_walking_safe(
        
        
            ped: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xDE4C184B2B9B071Au32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xDE4C184B2B9B071Au64;
        
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
pub fn is_ped_walking_raw(
        ped: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xDE4C184B2B9B071Au32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xDE4C184B2B9B071Au64;

        invoke_raw_typed!(
            hash,
                ped
        )
    }
}

/// ## Parameters
*



pub fn is_ped_cuffed_safe(
        
        
            ped: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x74E559B3BC910685u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x74E559B3BC910685u64;
        
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
pub fn is_ped_cuffed_raw(
        ped: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x74E559B3BC910685u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x74E559B3BC910685u64;

        invoke_raw_typed!(
            hash,
                ped
        )
    }
}

/// ```
Looks like the last parameter returns true if the path has been calculated, while the first returns the remaining distance to the end of the path.
Return value of native is the same as GET_NAVMESH_ROUTE_RESULT
Looks like the native returns an int for the path's state:
1 - ???
2 - ???
3 - Finished Generating
```



pub fn get_navmesh_route_distance_remaining_safe(
        
        
            ped: 
        , 
        
        
            distanceRemaining: 
        , 
        
        
            isPathReady: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xC6F5C0BCDC74D62Du32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xC6F5C0BCDC74D62Du64;
        
        let result = invoke_raw!(
            hash,
                ped, 
                distanceRemaining, 
                isPathReady
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn get_navmesh_route_distance_remaining_raw(
        ped: , 
        distanceRemaining: , 
        isPathReady: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xC6F5C0BCDC74D62Du32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xC6F5C0BCDC74D62Du64;

        invoke_raw_typed!(
            hash,
                ped, 
                distanceRemaining, 
                isPathReady
        )
    }
}

/// ## Parameters
*



pub fn task_look_at_coord_safe(
        
        
            entity: 
        , 
        
        
            x: 
        , 
        
        
            y: 
        , 
        
        
            z: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x6FA46612594F7973u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x6FA46612594F7973u64;
        
        let result = invoke_raw!(
            hash,
                entity, 
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
pub fn task_look_at_coord_raw(
        entity: , 
        x: , 
        y: , 
        z: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x6FA46612594F7973u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x6FA46612594F7973u64;

        invoke_raw_typed!(
            hash,
                entity, 
                x, 
                y, 
                z
        )
    }
}

/// ```
For a full list of the points, see here: goo.gl/wIH0vn
```



pub fn waypoint_recording_get_coord_safe(
        
        
            name: 
        , 
        
        
            point: 
        , 
        
        
            coord: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x2FB897405C90B361u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x2FB897405C90B361u64;
        
        let result = invoke_raw!(
            hash,
                name, 
                point, 
                coord
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn waypoint_recording_get_coord_raw(
        name: , 
        point: , 
        coord: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x2FB897405C90B361u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x2FB897405C90B361u64;

        invoke_raw_typed!(
            hash,
                name, 
                point, 
                coord
        )
    }
}

/// ## Parameters
*



pub fn set_ped_desired_move_blend_ratio_safe(
        
        
            ped: 
        , 
        
        
            p1: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x1E982AC8716912C5u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x1E982AC8716912C5u64;
        
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
pub fn set_ped_desired_move_blend_ratio_raw(
        ped: , 
        p1: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x1E982AC8716912C5u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x1E982AC8716912C5u64;

        invoke_raw_typed!(
            hash,
                ped, 
                p1
        )
    }
}

/// ```
Makes a ped follow the targetVehicle with <minDistance> in between.  
note: minDistance is ignored if drivingstyle is avoiding traffic, but Rushed is fine.  
Mode: The mode defines the relative position to the targetVehicle. The ped will try to position its vehicle there.  
-1 = behind  
0 = ahead  
1 = left  
2 = right  
3 = back left  
4 = back right  
if the target is closer than noRoadsDistance, the driver will ignore pathing/roads and follow you directly.  
Driving Styles guide: gtaforums.com/topic/822314-guide-driving-styles/  
```



pub fn task_vehicle_escort_safe(
        
        
            ped: 
        , 
        
        
            vehicle: 
        , 
        
        
            targetVehicle: 
        , 
        
        
            mode: 
        , 
        
        
            speed: 
        , 
        
        
            drivingStyle: 
        , 
        
        
            minDistance: 
        , 
        
        
            p7: 
        , 
        
        
            noRoadsDistance: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x0FA6E4B75F302400u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x0FA6E4B75F302400u64;
        
        let result = invoke_raw!(
            hash,
                ped, 
                vehicle, 
                targetVehicle, 
                mode, 
                speed, 
                drivingStyle, 
                minDistance, 
                p7, 
                noRoadsDistance
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn task_vehicle_escort_raw(
        ped: , 
        vehicle: , 
        targetVehicle: , 
        mode: , 
        speed: , 
        drivingStyle: , 
        minDistance: , 
        p7: , 
        noRoadsDistance: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x0FA6E4B75F302400u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x0FA6E4B75F302400u64;

        invoke_raw_typed!(
            hash,
                ped, 
                vehicle, 
                targetVehicle, 
                mode, 
                speed, 
                drivingStyle, 
                minDistance, 
                p7, 
                noRoadsDistance
        )
    }
}

/// RESET_EXCLUSIVE_SCENARIO_GROUP native function



pub fn reset_exclusive_scenario_group_safe(
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x4202BBCB8684563Du32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x4202BBCB8684563Du64;
        
        let result = invoke_raw!(
            hash,
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn reset_exclusive_scenario_group_raw(
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x4202BBCB8684563Du32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x4202BBCB8684563Du64;

        invoke_raw_typed!(
            hash,
        )
    }
}

/// ```
This function is called on peds in vehicles.  
anim: animation name  
p2, p3, p4: "sweep_low", "sweep_med" or "sweep_high"  
p5: no idea what it does but is usually -1  
```



pub fn task_sweep_aim_entity_safe(
        
        
            ped: 
        , 
        
        
            anim: 
        , 
        
        
            p2: 
        , 
        
        
            p3: 
        , 
        
        
            p4: 
        , 
        
        
            p5: 
        , 
        
        
            vehicle: 
        , 
        
        
            p7: 
        , 
        
        
            p8: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x2047C02158D6405Au32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x2047C02158D6405Au64;
        
        let result = invoke_raw!(
            hash,
                ped, 
                anim, 
                p2, 
                p3, 
                p4, 
                p5, 
                vehicle, 
                p7, 
                p8
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn task_sweep_aim_entity_raw(
        ped: , 
        anim: , 
        p2: , 
        p3: , 
        p4: , 
        p5: , 
        vehicle: , 
        p7: , 
        p8: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x2047C02158D6405Au32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x2047C02158D6405Au64;

        invoke_raw_typed!(
            hash,
                ped, 
                anim, 
                p2, 
                p3, 
                p4, 
                p5, 
                vehicle, 
                p7, 
                p8
        )
    }
}

/// ## Parameters
*



pub fn get_clip_set_for_scripted_gun_task_safe(
        
        
            p0: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x3A8CADC7D37AACC5u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x3A8CADC7D37AACC5u64;
        
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
pub fn get_clip_set_for_scripted_gun_task_raw(
        p0: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x3A8CADC7D37AACC5u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x3A8CADC7D37AACC5u64;

        invoke_raw_typed!(
            hash,
                p0
        )
    }
}

/// ## Parameters
*



pub fn does_scenario_of_type_exist_in_area_safe(
        
        
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
        let hash = 0x0A9D0C2A3BBC86C1u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x0A9D0C2A3BBC86C1u64;
        
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
pub fn does_scenario_of_type_exist_in_area_raw(
        p0: , 
        p1: , 
        p2: , 
        p3: , 
        p4: , 
        p5: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x0A9D0C2A3BBC86C1u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x0A9D0C2A3BBC86C1u64;

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
Stealth kill action name hashes:  
stealth kills can be found here: Grand Theft Auto V\common.rpf\data\action\stealth_kills.meta  
...  
{  
    "ACT_stealth_kill_a",  
    "ACT_stealth_kill_weapon",  
    "ACT_stealth_kill_b",  
    "ACT_stealth_kill_c",  
    "ACT_stealth_kill_d",  
    "ACT_stealth_kill_a_gardener"  
}  
Only known script using this native: fbi4_prep2  
EXAMPLE:  
ai::task_stealth_kill(iParam1, Local_252, gameplay::get_hash_key("AR_stealth_kill_a"), 1f, 0);ai::task_stealth_kill(iParam1, Local_252, gameplay::get_hash_key("AR_stealth_kill_knife"), 1f, 0);  
Also it may be important to note, that each time this task is called, it's followed by AI::CLEAR_PED_TASKS on the target  
```



pub fn task_stealth_kill_safe(
        
        
            killer: 
        , 
        
        
            target: 
        , 
        
        
            actionType: 
        , 
        
        
            p3: 
        , 
        
        
            p4: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xAA5DC05579D60BD9u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xAA5DC05579D60BD9u64;
        
        let result = invoke_raw!(
            hash,
                killer, 
                target, 
                actionType, 
                p3, 
                p4
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn task_stealth_kill_raw(
        killer: , 
        target: , 
        actionType: , 
        p3: , 
        p4: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xAA5DC05579D60BD9u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xAA5DC05579D60BD9u64;

        invoke_raw_typed!(
            hash,
                killer, 
                target, 
                actionType, 
                p3, 
                p4
        )
    }
}

/// ```
Makes the specified ped stand still for (time) milliseconds.  
```



pub fn task_stand_still_safe(
        
        
            ped: 
        , 
        
        
            time: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x919BE13EED931959u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x919BE13EED931959u64;
        
        let result = invoke_raw!(
            hash,
                ped, 
                time
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn task_stand_still_raw(
        ped: , 
        time: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x919BE13EED931959u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x919BE13EED931959u64;

        invoke_raw_typed!(
            hash,
                ped, 
                time
        )
    }
}

/// ## Parameters
*



pub fn use_waypoint_recording_as_assisted_movement_route_safe(
        
        
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
        let hash = 0x5A353B8E6B1095B5u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x5A353B8E6B1095B5u64;
        
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
pub fn use_waypoint_recording_as_assisted_movement_route_raw(
        name: , 
        p1: , 
        p2: , 
        p3: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x5A353B8E6B1095B5u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x5A353B8E6B1095B5u64;

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



pub fn task_use_mobile_phone_timed_safe(
        
        
            ped: 
        , 
        
        
            duration: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x5EE02954A14C69DBu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x5EE02954A14C69DBu64;
        
        let result = invoke_raw!(
            hash,
                ped, 
                duration
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn task_use_mobile_phone_timed_raw(
        ped: , 
        duration: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x5EE02954A14C69DBu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x5EE02954A14C69DBu64;

        invoke_raw_typed!(
            hash,
                ped, 
                duration
        )
    }
}

/// Will make the ped move to a coordinate while aiming (and optionally shooting) at the given entity.

```c
enum eFiringPatternHashes {
    FIRING_PATTERN_DEFAULT = 0,
    FIRING_PATTERN_BURST_FIRE = -687903391,
    FIRING_PATTERN_BURST_FIRE_DRIVEBY = -753768974,
    FIRING_PATTERN_FULL_AUTO = -957453492,
    FIRING_PATTERN_SINGLE_SHOT = 1566631136,
    FIRING_PATTERN_DELAY_FIRE_BY_ONE_SEC = 2055493265,
    FIRING_PATTERN_BURST_FIRE_HELI = -1857128337,
    FIRING_PATTERN_SHORT_BURSTS = 445831135,
    FIRING_PATTERN_BURST_FIRE_MICRO = 1122960381,
    FIRING_PATTERN_SLOW_FIRE_TANK = -490063247,
    FIRING_PATTERN_TAMPA_MORTAR = -1842093953
}
```



pub fn task_go_to_coord_while_aiming_at_entity_safe(
        
        
            ped: 
        , 
        
        
            x: 
        , 
        
        
            y: 
        , 
        
        
            z: 
        , 
        
        
            entityToAimAt: 
        , 
        
        
            moveSpeed: 
        , 
        
        
            shoot: 
        , 
        
        
            targetRadius: 
        , 
        
        
            slowDistance: 
        , 
        
        
            useNavMesh: 
        , 
        
        
            navFlags: 
        , 
        
        
            instantBlendAtAim: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xB2A16444EAD9AE47u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xB2A16444EAD9AE47u64;
        
        let result = invoke_raw!(
            hash,
                ped, 
                x, 
                y, 
                z, 
                entityToAimAt, 
                moveSpeed, 
                shoot, 
                targetRadius, 
                slowDistance, 
                useNavMesh, 
                navFlags, 
                instantBlendAtAim
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn task_go_to_coord_while_aiming_at_entity_raw(
        ped: , 
        x: , 
        y: , 
        z: , 
        entityToAimAt: , 
        moveSpeed: , 
        shoot: , 
        targetRadius: , 
        slowDistance: , 
        useNavMesh: , 
        navFlags: , 
        instantBlendAtAim: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xB2A16444EAD9AE47u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xB2A16444EAD9AE47u64;

        invoke_raw_typed!(
            hash,
                ped, 
                x, 
                y, 
                z, 
                entityToAimAt, 
                moveSpeed, 
                shoot, 
                targetRadius, 
                slowDistance, 
                useNavMesh, 
                navFlags, 
                instantBlendAtAim
        )
    }
}

/// ```
This function has a third parameter as well (bool).  
Second parameter is unused.  
seconds parameter was for jetpack in the early stages of gta and the hard coded code is now removed  
```

```
NativeDB Added Parameter 3: BOOL p2
```



pub fn task_parachute_safe(
        
        
            ped: 
        , 
        
        
            p1: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xD2F1C53C97EE81ABu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xD2F1C53C97EE81ABu64;
        
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
pub fn task_parachute_raw(
        ped: , 
        p1: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xD2F1C53C97EE81ABu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xD2F1C53C97EE81ABu64;

        invoke_raw_typed!(
            hash,
                ped, 
                p1
        )
    }
}

/// Used in am_vehicle_spawn.ysc and am_mp_submarine.ysc.
p0 is always 0, p5 is always 1
p1 is the vehicle handle of the submarine. Submarine must have a driver, but the ped handle is not passed to the native.
Speed can be set by calling SET_DRIVE_TASK_CRUISE_SPEED after

```
NativeDB Introduced: v2189
```



pub fn _task_submarine_goto_and_stop_safe(
        
        
            p0: 
        , 
        
        
            submarine: 
        , 
        
        
            x: 
        , 
        
        
            y: 
        , 
        
        
            z: 
        , 
        
        
            p5: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xC22B40579A498CA4u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xC22B40579A498CA4u64;
        
        let result = invoke_raw!(
            hash,
                p0, 
                submarine, 
                x, 
                y, 
                z, 
                p5
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn _task_submarine_goto_and_stop_raw(
        p0: , 
        submarine: , 
        x: , 
        y: , 
        z: , 
        p5: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xC22B40579A498CA4u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xC22B40579A498CA4u64;

        invoke_raw_typed!(
            hash,
                p0, 
                submarine, 
                x, 
                y, 
                z, 
                p5
        )
    }
}

/// ```
Example from "me_amanda1.ysc.c4":
TASK::TASK_ARREST_PED(l_19F /* This is a Ped */ , PLAYER::PLAYER_PED_ID());
Example from "armenian1.ysc.c4":
if (!PED::IS_PED_INJURED(l_B18[0/*1*/])) {
    TASK::TASK_ARREST_PED(l_B18[0/*1*/], PLAYER::PLAYER_PED_ID());
}
I would love to have time to experiment to see if a player Ped can arrest another Ped. Might make for a good cop mod.
Looks like only the player can be arrested this way. Peds react and try to arrest you if you task them, but the player charater doesn't do anything if tasked to arrest another ped.
```



pub fn task_arrest_ped_safe(
        
        
            ped: 
        , 
        
        
            target: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xF3B9A78A178572B1u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xF3B9A78A178572B1u64;
        
        let result = invoke_raw!(
            hash,
                ped, 
                target
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn task_arrest_ped_raw(
        ped: , 
        target: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xF3B9A78A178572B1u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xF3B9A78A178572B1u64;

        invoke_raw_typed!(
            hash,
                ped, 
                target
        )
    }
}

/// ## Parameters
*



pub fn task_react_and_flee_ped_safe(
        
        
            ped: 
        , 
        
        
            fleeTarget: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x72C896464915D1B1u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x72C896464915D1B1u64;
        
        let result = invoke_raw!(
            hash,
                ped, 
                fleeTarget
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn task_react_and_flee_ped_raw(
        ped: , 
        fleeTarget: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x72C896464915D1B1u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x72C896464915D1B1u64;

        invoke_raw_typed!(
            hash,
                ped, 
                fleeTarget
        )
    }
}

/// ```
pilot, vehicle and altitude are rather self-explanatory.  
p4: is unused variable in the function.  
entityToFollow: you can provide a Vehicle entity or a Ped entity, the heli will protect them.  
'targetSpeed':  The pilot will dip the nose AS MUCH AS POSSIBLE so as to reach this value AS FAST AS POSSIBLE.  As such, you'll want to modulate it as opposed to calling it via a hard-wired, constant #.  
'radius' isn't just "stop within radius of X of target" like with ground vehicles.  In this case, the pilot will fly an entire circle around 'radius' and continue to do so.  
NOT CONFIRMED:  p7 appears to be a FlyingStyle enum.  Still investigating it as of this writing, but playing around with values here appears to result in different -behavior- as opposed to offsetting coordinates, altitude, target speed, etc.  
NOTE: If the pilot finds enemies, it will engage them until it kills them, but will return to protect the ped/vehicle given shortly thereafter.  
```



pub fn task_vehicle_heli_protect_safe(
        
        
            pilot: 
        , 
        
        
            vehicle: 
        , 
        
        
            entityToFollow: 
        , 
        
        
            targetSpeed: 
        , 
        
        
            p4: 
        , 
        
        
            radius: 
        , 
        
        
            altitude: 
        , 
        
        
            p7: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x1E09C32048FEFD1Cu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x1E09C32048FEFD1Cu64;
        
        let result = invoke_raw!(
            hash,
                pilot, 
                vehicle, 
                entityToFollow, 
                targetSpeed, 
                p4, 
                radius, 
                altitude, 
                p7
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn task_vehicle_heli_protect_raw(
        pilot: , 
        vehicle: , 
        entityToFollow: , 
        targetSpeed: , 
        p4: , 
        radius: , 
        altitude: , 
        p7: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x1E09C32048FEFD1Cu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x1E09C32048FEFD1Cu64;

        invoke_raw_typed!(
            hash,
                pilot, 
                vehicle, 
                entityToFollow, 
                targetSpeed, 
                p4, 
                radius, 
                altitude, 
                p7
        )
    }
}

/// ## Parameters
*



pub fn task_follow_nav_mesh_to_coord_advanced_safe(
        
        
            ped: 
        , 
        
        
            x: 
        , 
        
        
            y: 
        , 
        
        
            z: 
        , 
        
        
            speed: 
        , 
        
        
            timeout: 
        , 
        
        
            unkFloat: 
        , 
        
        
            unkInt: 
        , 
        
        
            unkX: 
        , 
        
        
            unkY: 
        , 
        
        
            unkZ: 
        , 
        
        
            unk_40000f: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x17F58B88D085DBACu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x17F58B88D085DBACu64;
        
        let result = invoke_raw!(
            hash,
                ped, 
                x, 
                y, 
                z, 
                speed, 
                timeout, 
                unkFloat, 
                unkInt, 
                unkX, 
                unkY, 
                unkZ, 
                unk_40000f
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn task_follow_nav_mesh_to_coord_advanced_raw(
        ped: , 
        x: , 
        y: , 
        z: , 
        speed: , 
        timeout: , 
        unkFloat: , 
        unkInt: , 
        unkX: , 
        unkY: , 
        unkZ: , 
        unk_40000f: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x17F58B88D085DBACu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x17F58B88D085DBACu64;

        invoke_raw_typed!(
            hash,
                ped, 
                x, 
                y, 
                z, 
                speed, 
                timeout, 
                unkFloat, 
                unkInt, 
                unkX, 
                unkY, 
                unkZ, 
                unk_40000f
        )
    }
}

/// ```
Routes: "1_FIBStairs", "2_FIBStairs", "3_FIBStairs", "4_FIBStairs", "5_FIBStairs", "5_TowardsFire", "6a_FIBStairs", "7_FIBStairs", "8_FIBStairs", "Aprtmnt_1", "AssAfterLift", "ATM_1", "coroner2", "coroner_stairs", "f5_jimmy1", "fame1", "family5b", "family5c", "Family5d", "family5d", "FIB_Glass1", "FIB_Glass2", "FIB_Glass3", "finaBroute1A", "finalb1st", "finalB1sta", "finalbround", "finalbroute2", "Hairdresser1", "jan_foyet_ft_door", "Jo_3", "Lemar1", "Lemar2", "mansion_1", "Mansion_1", "pols_1", "pols_2", "pols_3", "pols_4", "pols_5", "pols_6", "pols_7", "pols_8", "Pro_S1", "Pro_S1a", "Pro_S2", "Towards_case", "trev_steps", "tunrs1", "tunrs2", "tunrs3", "Wave01457s"  
```



pub fn assisted_movement_request_route_safe(
        
        
            route: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x817268968605947Au32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x817268968605947Au64;
        
        let result = invoke_raw!(
            hash,
                route
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn assisted_movement_request_route_raw(
        route: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x817268968605947Au32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x817268968605947Au64;

        invoke_raw_typed!(
            hash,
                route
        )
    }
}

/// ## Parameters
*



pub fn task_put_ped_directly_into_cover_safe(
        
        
            ped: 
        , 
        
        
            x: 
        , 
        
        
            y: 
        , 
        
        
            z: 
        , 
        
        
            timeout: 
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
        , 
        
        
            p10: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x4172393E6BE1FECEu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x4172393E6BE1FECEu64;
        
        let result = invoke_raw!(
            hash,
                ped, 
                x, 
                y, 
                z, 
                timeout, 
                p5, 
                p6, 
                p7, 
                p8, 
                p9, 
                p10
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn task_put_ped_directly_into_cover_raw(
        ped: , 
        x: , 
        y: , 
        z: , 
        timeout: , 
        p5: , 
        p6: , 
        p7: , 
        p8: , 
        p9: , 
        p10: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x4172393E6BE1FECEu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x4172393E6BE1FECEu64;

        invoke_raw_typed!(
            hash,
                ped, 
                x, 
                y, 
                z, 
                timeout, 
                p5, 
                p6, 
                p7, 
                p8, 
                p9, 
                p10
        )
    }
}

/// ```
NativeDB Introduced: v2189
```



pub fn _0x6100b3cefd43452e_safe(
        
        
            p0: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x6100B3CEFD43452Eu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x6100B3CEFD43452Eu64;
        
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
pub fn _0x6100b3cefd43452e_raw(
        p0: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x6100B3CEFD43452Eu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x6100B3CEFD43452Eu64;

        invoke_raw_typed!(
            hash,
                p0
        )
    }
}

/// Puts the ped into the given scenario immediately at their current location. [List of scenario names](https://pastebin.com/6mrYTdQv) or in `update/update.rpf/common/data/ai/scenarios.meta`.



pub fn task_start_scenario_in_place_safe(
        
        
            ped: 
        , 
        
        
            scenarioName: 
        , 
        
        
            timeToLeave: 
        , 
        
        
            playIntroClip: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x142A02425FF02BD9u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x142A02425FF02BD9u64;
        
        let result = invoke_raw!(
            hash,
                ped, 
                scenarioName, 
                timeToLeave, 
                playIntroClip
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn task_start_scenario_in_place_raw(
        ped: , 
        scenarioName: , 
        timeToLeave: , 
        playIntroClip: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x142A02425FF02BD9u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x142A02425FF02BD9u64;

        invoke_raw_typed!(
            hash,
                ped, 
                scenarioName, 
                timeToLeave, 
                playIntroClip
        )
    }
}

/// ## Parameters
*



pub fn get_phone_gesture_anim_current_time_safe(
        
        
            ped: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x47619ABE8B268C60u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x47619ABE8B268C60u64;
        
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
pub fn get_phone_gesture_anim_current_time_raw(
        ped: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x47619ABE8B268C60u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x47619ABE8B268C60u64;

        invoke_raw_typed!(
            hash,
                ped
        )
    }
}

/// ```
makes ped parachute to coords x y z. Works well with PATHFIND::GET_SAFE_COORD_FOR_PED  
```



pub fn task_parachute_to_target_safe(
        
        
            ped: 
        , 
        
        
            x: 
        , 
        
        
            y: 
        , 
        
        
            z: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xB33E291AFA6BD03Au32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xB33E291AFA6BD03Au64;
        
        let result = invoke_raw!(
            hash,
                ped, 
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
pub fn task_parachute_to_target_raw(
        ped: , 
        x: , 
        y: , 
        z: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xB33E291AFA6BD03Au32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xB33E291AFA6BD03Au64;

        invoke_raw_typed!(
            hash,
                ped, 
                x, 
                y, 
                z
        )
    }
}

/// ```
Updated variables
An alternative to TASK::TASK_USE_NEAREST_SCENARIO_TO_COORD_WARP. Makes the ped walk to the scenario instead.
```



pub fn task_use_nearest_scenario_to_coord_safe(
        
        
            ped: 
        , 
        
        
            x: 
        , 
        
        
            y: 
        , 
        
        
            z: 
        , 
        
        
            distance: 
        , 
        
        
            duration: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x277F471BA9DB000Bu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x277F471BA9DB000Bu64;
        
        let result = invoke_raw!(
            hash,
                ped, 
                x, 
                y, 
                z, 
                distance, 
                duration
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn task_use_nearest_scenario_to_coord_raw(
        ped: , 
        x: , 
        y: , 
        z: , 
        distance: , 
        duration: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x277F471BA9DB000Bu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x277F471BA9DB000Bu64;

        invoke_raw_typed!(
            hash,
                ped, 
                x, 
                y, 
                z, 
                distance, 
                duration
        )
    }
}

/// ## Parameters
*



pub fn set_parachute_task_thrust_safe(
        
        
            ped: 
        , 
        
        
            thrust: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x0729BAC1B8C64317u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x0729BAC1B8C64317u64;
        
        let result = invoke_raw!(
            hash,
                ped, 
                thrust
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn set_parachute_task_thrust_raw(
        ped: , 
        thrust: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x0729BAC1B8C64317u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x0729BAC1B8C64317u64;

        invoke_raw_typed!(
            hash,
                ped, 
                thrust
        )
    }
}

/// ```
NativeDB Added Parameter 2: BOOL p1
```



pub fn task_sky_dive_safe(
        
        
            ped: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x601736CFE536B0A0u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x601736CFE536B0A0u64;
        
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
pub fn task_sky_dive_raw(
        ped: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x601736CFE536B0A0u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x601736CFE536B0A0u64;

        invoke_raw_typed!(
            hash,
                ped
        )
    }
}

/// ## Parameters
*



pub fn task_guard_assigned_defensive_area_safe(
        
        
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
        let hash = 0xD2A207EEBDF9889Bu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xD2A207EEBDF9889Bu64;
        
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
pub fn task_guard_assigned_defensive_area_raw(
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
        let hash = 0xD2A207EEBDF9889Bu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xD2A207EEBDF9889Bu64;

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
Flags from decompiled scripts:  
0 = normal exit and closes door.  
1 = normal exit and closes door.  
16 = teleports outside, door kept closed.  (This flag does not seem to work for the front seats in buses, NPCs continue to exit normally)
64 = normal exit and closes door, maybe a bit slower animation than 0.  
256 = normal exit but does not close the door.  
4160 = ped is throwing himself out, even when the vehicle is still.  
262144 = ped moves to passenger seat first, then exits normally  
Others to be tried out: 320, 512, 131072.  
```



pub fn task_leave_vehicle_safe(
        
        
            ped: 
        , 
        
        
            vehicle: 
        , 
        
        
            flags: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xD3DBCE61A490BE02u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xD3DBCE61A490BE02u64;
        
        let result = invoke_raw!(
            hash,
                ped, 
                vehicle, 
                flags
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn task_leave_vehicle_raw(
        ped: , 
        vehicle: , 
        flags: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xD3DBCE61A490BE02u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xD3DBCE61A490BE02u64;

        invoke_raw_typed!(
            hash,
                ped, 
                vehicle, 
                flags
        )
    }
}

/// ## Parameters
*



pub fn task_ped_slide_to_coord_hdg_rate_safe(
        
        
            ped: 
        , 
        
        
            x: 
        , 
        
        
            y: 
        , 
        
        
            z: 
        , 
        
        
            heading: 
        , 
        
        
            p5: 
        , 
        
        
            p6: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x5A4A6A6D3DC64F52u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x5A4A6A6D3DC64F52u64;
        
        let result = invoke_raw!(
            hash,
                ped, 
                x, 
                y, 
                z, 
                heading, 
                p5, 
                p6
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn task_ped_slide_to_coord_hdg_rate_raw(
        ped: , 
        x: , 
        y: , 
        z: , 
        heading: , 
        p5: , 
        p6: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x5A4A6A6D3DC64F52u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x5A4A6A6D3DC64F52u64;

        invoke_raw_typed!(
            hash,
                ped, 
                x, 
                y, 
                z, 
                heading, 
                p5, 
                p6
        )
    }
}

/// Tells a ped to go to a coord by any means.

```c
enum eDrivingMode {
  DF_StopForCars = 1,
  DF_StopForPeds = 2,
  DF_SwerveAroundAllCars = 4,
  DF_SteerAroundStationaryCars = 8,
  DF_SteerAroundPeds = 16,
  DF_SteerAroundObjects = 32,
  DF_DontSteerAroundPlayerPed = 64,
  DF_StopAtLights = 128,
  DF_GoOffRoadWhenAvoiding = 256,
  DF_DriveIntoOncomingTraffic = 512,
  DF_DriveInReverse = 1024,

  // If pathfinding fails, cruise randomly instead of going on a straight line
  DF_UseWanderFallbackInsteadOfStraightLine = 2048,

  DF_AvoidRestrictedAreas = 4096,

  // These only work on MISSION_CRUISE
  DF_PreventBackgroundPathfinding = 8192,
  DF_AdjustCruiseSpeedBasedOnRoadSpeed = 16384,

  DF_UseShortCutLinks =  262144,
  DF_ChangeLanesAroundObstructions = 524288,
  // cruise tasks ignore this anyway--only used for goto's
  DF_UseSwitchedOffNodes =  2097152,
  // if you're going to be primarily driving off road
  DF_PreferNavmeshRoute =  4194304,

  // Only works for planes using MISSION_GOTO, will cause them to drive along the ground instead of fly
  DF_PlaneTaxiMode =  8388608,

  DF_ForceStraightLine = 16777216,
  DF_UseStringPullingAtJunctions = 33554432,

  DF_AvoidHighways = 536870912,
  DF_ForceJoinInRoadDirection = 1073741824,

  // Standard driving mode. stops for cars, peds, and lights, goes around stationary obstructions
  DRIVINGMODE_STOPFORCARS = 786603, // DF_StopForCars|DF_StopForPeds|DF_SteerAroundObjects|DF_SteerAroundStationaryCars|DF_StopAtLights|DF_UseShortCutLinks|DF_ChangeLanesAroundObstructions,		// Obey lights too

  // Like the above, but doesn't steer around anything in its way - will only wait instead.
  DRIVINGMODE_STOPFORCARS_STRICT = 262275, // DF_StopForCars|DF_StopForPeds|DF_StopAtLights|DF_UseShortCutLinks, // Doesn't deviate an inch.

  // Default "alerted" driving mode. drives around everything, doesn't obey lights
  DRIVINGMODE_AVOIDCARS = 786469, // DF_SwerveAroundAllCars|DF_SteerAroundObjects|DF_UseShortCutLinks|DF_ChangeLanesAroundObstructions|DF_StopForCars,

  // Very erratic driving. difference between this and AvoidCars is that it doesn't use the brakes at ALL to help with steering
  DRIVINGMODE_AVOIDCARS_RECKLESS = 786468, // DF_SwerveAroundAllCars|DF_SteerAroundObjects|DF_UseShortCutLinks|DF_ChangeLanesAroundObstructions,

  // Smashes through everything
  DRIVINGMODE_PLOUGHTHROUGH = 262144, // DF_UseShortCutLinks

  // Drives normally except for the fact that it ignores lights
  DRIVINGMODE_STOPFORCARS_IGNORELIGHTS = 786475, // DF_StopForCars|DF_SteerAroundStationaryCars|DF_StopForPeds|DF_SteerAroundObjects|DF_UseShortCutLinks|DF_ChangeLanesAroundObstructions

  // Try to swerve around everything, but stop for lights if necessary
  DRIVINGMODE_AVOIDCARS_OBEYLIGHTS = 786597, // DF_SwerveAroundAllCars|DF_StopAtLights|DF_SteerAroundObjects|DF_UseShortCutLinks|DF_ChangeLanesAroundObstructions|DF_StopForCars

  // Swerve around cars, be careful around peds, and stop for lights
  DRIVINGMODE_AVOIDCARS_STOPFORPEDS_OBEYLIGHTS = 786599 // DF_SwerveAroundAllCars|DF_StopAtLights|DF_StopForPeds|DF_SteerAroundObjects|DF_UseShortCutLinks|DF_ChangeLanesAroundObstructions|DF_StopForCars
};
```



pub fn task_go_to_coord_any_means_safe(
        
        
            ped: 
        , 
        
        
            x: 
        , 
        
        
            y: 
        , 
        
        
            z: 
        , 
        
        
            fMoveBlendRatio: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x5BC448CB78FA3E88u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x5BC448CB78FA3E88u64;
        
        let result = invoke_raw!(
            hash,
                ped, 
                x, 
                y, 
                z, 
                fMoveBlendRatio
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn task_go_to_coord_any_means_raw(
        ped: , 
        x: , 
        y: , 
        z: , 
        fMoveBlendRatio: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x5BC448CB78FA3E88u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x5BC448CB78FA3E88u64;

        invoke_raw_typed!(
            hash,
                ped, 
                x, 
                y, 
                z, 
                fMoveBlendRatio
        )
    }
}

/// ## Parameters
*



pub fn task_vehicle_aim_at_ped_safe(
        
        
            ped: 
        , 
        
        
            target: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xE41885592B08B097u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xE41885592B08B097u64;
        
        let result = invoke_raw!(
            hash,
                ped, 
                target
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn task_vehicle_aim_at_ped_raw(
        ped: , 
        target: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xE41885592B08B097u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xE41885592B08B097u64;

        invoke_raw_typed!(
            hash,
                ped, 
                target
        )
    }
}

/// ## Parameters
*



pub fn get_task_move_network_event_safe(
        
        
            ped: 
        , 
        
        
            eventName: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xB4F47213DF45A64Cu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xB4F47213DF45A64Cu64;
        
        let result = invoke_raw!(
            hash,
                ped, 
                eventName
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn get_task_move_network_event_raw(
        ped: , 
        eventName: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xB4F47213DF45A64Cu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xB4F47213DF45A64Cu64;

        invoke_raw_typed!(
            hash,
                ped, 
                eventName
        )
    }
}

/// ## Parameters
*



pub fn task_clear_defensive_area_safe(
        
        
            p0: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x95A6C46A31D1917Du32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x95A6C46A31D1917Du64;
        
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
pub fn task_clear_defensive_area_raw(
        p0: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x95A6C46A31D1917Du32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x95A6C46A31D1917Du64;

        invoke_raw_typed!(
            hash,
                p0
        )
    }
}

/// CLOSE_PATROL_ROUTE native function



pub fn close_patrol_route_safe(
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xB043ECA801B8CBC1u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xB043ECA801B8CBC1u64;
        
        let result = invoke_raw!(
            hash,
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn close_patrol_route_raw(
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xB043ECA801B8CBC1u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xB043ECA801B8CBC1u64;

        invoke_raw_typed!(
            hash,
        )
    }
}

