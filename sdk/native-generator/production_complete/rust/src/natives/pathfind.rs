//! PATHFIND native functions
//! 
//! Functions for the pathfind category (local)

use crate::types::*;
use crate::errors::{NativeError, NativeResult, validation};
use nalgebra::Vector3;
use tracing::{debug, warn};

/// Clears a disabled GPS route area from a certain index previously set using [`SET_GPS_DISABLED_ZONE_AT_INDEX`](#_0xD0BC1C6FB18EE154).



pub fn clear_gps_disabled_zone_at_index_safe(
        
        
            index: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x2801D0012266DF07u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x2801D0012266DF07u64;
        
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
pub fn clear_gps_disabled_zone_at_index_raw(
        index: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x2801D0012266DF07u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x2801D0012266DF07u64;

        invoke_raw_typed!(
            hash,
                index
        )
    }
}

/// ```cpp
enum eSafePositionFlags {
    // Only navmesh polygons marked as pavement
    GSC_FLAG_ONLY_PAVEMENT = 1,
    // Only navmesh polygons not marked as "isolated"
    GSC_FLAG_NOT_ISOLATED = 2,
    // No navmesh polygons created from interiors
    GSC_FLAG_NOT_INTERIOR = 4,
    // No navmesh polygons marked as water
    GSC_FLAG_NOT_WATER = 8,
    // Only navmesh polygons marked as "network spawn candidate"
    GSC_FLAG_ONLY_NETWORK_SPAWN = 16,
    // Specify whether to use a flood-fill from the starting position, as opposed to scanning all polygons within the search volume
    GSC_FLAG_USE_FLOOD_FILL = 32
}
```



pub fn get_safe_coord_for_ped_safe(
        
        
            x: 
        , 
        
        
            y: 
        , 
        
        
            z: 
        , 
        
        
            onlyOnPavement: 
        , 
        
        
            outPosition: 
        , 
        
        
            flags: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xB61C8E878A4199CAu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xB61C8E878A4199CAu64;
        
        let result = invoke_raw!(
            hash,
                x, 
                y, 
                z, 
                onlyOnPavement, 
                outPosition, 
                flags
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn get_safe_coord_for_ped_raw(
        x: , 
        y: , 
        z: , 
        onlyOnPavement: , 
        outPosition: , 
        flags: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xB61C8E878A4199CAu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xB61C8E878A4199CAu64;

        invoke_raw_typed!(
            hash,
                x, 
                y, 
                z, 
                onlyOnPavement, 
                outPosition, 
                flags
        )
    }
}

/// ```
Returns whether navmesh for the region is loaded. The region is a rectangular prism defined by it's top left deepest corner to it's bottom right shallowest corner.  
If you can re-word this so it makes more sense, please do. I'm horrible with words sometimes...  
```



pub fn is_navmesh_loaded_in_area_safe(
        
        
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
        let hash = 0xF813C7E63F9062A5u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xF813C7E63F9062A5u64;
        
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
pub fn is_navmesh_loaded_in_area_raw(
        x1: , 
        y1: , 
        z1: , 
        x2: , 
        y2: , 
        z2: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xF813C7E63F9062A5u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xF813C7E63F9062A5u64;

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

/// ```
Returns CGameWorldHeightMap's maximum Z among all grid nodes that intersect with the specified rectangle.
```



pub fn _get_heightmap_top_z_for_area_safe(
        
        
            x1: 
        , 
        
        
            y1: 
        , 
        
        
            x2: 
        , 
        
        
            y2: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x8ABE8608576D9CE3u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x8ABE8608576D9CE3u64;
        
        let result = invoke_raw!(
            hash,
                x1, 
                y1, 
                x2, 
                y2
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn _get_heightmap_top_z_for_area_raw(
        x1: , 
        y1: , 
        x2: , 
        y2: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x8ABE8608576D9CE3u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x8ABE8608576D9CE3u64;

        invoke_raw_typed!(
            hash,
                x1, 
                y1, 
                x2, 
                y2
        )
    }
}

/// unknown3 is related to ``SEND_SCRIPT_WORLD_STATE_EVENT > CNetworkRoadNodeWorldStateData`` in networked environments.

See [`IS_POINT_IN_ANGLED_AREA`](#_0x2A70BAE8883E4C81) for the definition of an angled area.



pub fn set_roads_in_angled_area_safe(
        
        
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
        
        
            unknown1: 
        , 
        
        
            unknown2: 
        , 
        
        
            unknown3: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x1A5AA1208AF5DB59u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x1A5AA1208AF5DB59u64;
        
        let result = invoke_raw!(
            hash,
                x1, 
                y1, 
                z1, 
                x2, 
                y2, 
                z2, 
                width, 
                unknown1, 
                unknown2, 
                unknown3
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn set_roads_in_angled_area_raw(
        x1: , 
        y1: , 
        z1: , 
        x2: , 
        y2: , 
        z2: , 
        width: , 
        unknown1: , 
        unknown2: , 
        unknown3: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x1A5AA1208AF5DB59u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x1A5AA1208AF5DB59u64;

        invoke_raw_typed!(
            hash,
                x1, 
                y1, 
                z1, 
                x2, 
                y2, 
                z2, 
                width, 
                unknown1, 
                unknown2, 
                unknown3
        )
    }
}

/// ```
Returns CGameWorldHeightMap's minimum Z value at specified point (grid node).
```



pub fn _get_heightmap_bottom_z_for_position_safe(
        
        
            x: 
        , 
        
        
            y: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x336511A34F2E5185u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x336511A34F2E5185u64;
        
        let result = invoke_raw!(
            hash,
                x, 
                y
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn _get_heightmap_bottom_z_for_position_raw(
        x: , 
        y: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x336511A34F2E5185u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x336511A34F2E5185u64;

        invoke_raw_typed!(
            hash,
                x, 
                y
        )
    }
}

/// Creates a navmesh blocking object, vehicles will avoid driving through this area. 

Only 32 blocking objects may exist at a given time and must be manually managed. See [`REMOVE_NAVMESH_BLOCKING_OBJECT`](#_0x46399A7895957C0E) and [`onResourceStop`](https://docs.fivem.net/docs/scripting-reference/events/list/onResourceStop/)

```c
enum eBlockingObjectFlags {
    // Default Flag
    BLOCKING_OBJECT_DEFAULT = 0,
    // Blocking object will block wander paths
    BLOCKING_OBJECT_WANDERPATH = 1,
    // Blocking object will block (regular) shortest-paths
    BLOCKING_OBJECT_SHORTESTPATH = 2,
    // Blocking object will block flee paths
    BLOCKING_OBJECT_FLEEPATH = 4,
    // Blocking object will block all paths
    BLOCKING_OBJECT_ALLPATHS = 7,
}
```



pub fn add_navmesh_blocking_object_safe(
        
        
            x: 
        , 
        
        
            y: 
        , 
        
        
            z: 
        , 
        
        
            width: 
        , 
        
        
            length: 
        , 
        
        
            height: 
        , 
        
        
            heading: 
        , 
        
        
            bPermanent: 
        , 
        
        
            flags: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xFCD5C8E06E502F5Au32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xFCD5C8E06E502F5Au64;
        
        let result = invoke_raw!(
            hash,
                x, 
                y, 
                z, 
                width, 
                length, 
                height, 
                heading, 
                bPermanent, 
                flags
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn add_navmesh_blocking_object_raw(
        x: , 
        y: , 
        z: , 
        width: , 
        length: , 
        height: , 
        heading: , 
        bPermanent: , 
        flags: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xFCD5C8E06E502F5Au32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xFCD5C8E06E502F5Au64;

        invoke_raw_typed!(
            hash,
                x, 
                y, 
                z, 
                width, 
                length, 
                height, 
                heading, 
                bPermanent, 
                flags
        )
    }
}

/// Get the nth closest vehicle node with its heading and total lane count. If you need specific forward and backward lane counts use [`GET_CLOSEST_ROAD`](#_0x132F52BBA570FE92).



pub fn get_nth_closest_vehicle_node_with_heading_safe(
        
        
            x: 
        , 
        
        
            y: 
        , 
        
        
            z: 
        , 
        
        
            nthClosest: 
        , 
        
        
            outPosition: 
        , 
        
        
            outHeading: 
        , 
        
        
            totalLanes: 
        , 
        
        
            nodeFlags: 
        , 
        
        
            zMeasureMult: 
        , 
        
        
            zTolerance: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x80CA6A8B6C094CC4u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x80CA6A8B6C094CC4u64;
        
        let result = invoke_raw!(
            hash,
                x, 
                y, 
                z, 
                nthClosest, 
                outPosition, 
                outHeading, 
                totalLanes, 
                nodeFlags, 
                zMeasureMult, 
                zTolerance
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn get_nth_closest_vehicle_node_with_heading_raw(
        x: , 
        y: , 
        z: , 
        nthClosest: , 
        outPosition: , 
        outHeading: , 
        totalLanes: , 
        nodeFlags: , 
        zMeasureMult: , 
        zTolerance: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x80CA6A8B6C094CC4u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x80CA6A8B6C094CC4u64;

        invoke_raw_typed!(
            hash,
                x, 
                y, 
                z, 
                nthClosest, 
                outPosition, 
                outHeading, 
                totalLanes, 
                nodeFlags, 
                zMeasureMult, 
                zTolerance
        )
    }
}

/// ## Parameters
*



pub fn _get_point_on_road_side_safe(
        
        
            x: 
        , 
        
        
            y: 
        , 
        
        
            z: 
        , 
        
        
            p3: 
        , 
        
        
            outPosition: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x16F46FB18C8009E4u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x16F46FB18C8009E4u64;
        
        let result = invoke_raw!(
            hash,
                x, 
                y, 
                z, 
                p3, 
                outPosition
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn _get_point_on_road_side_raw(
        x: , 
        y: , 
        z: , 
        p3: , 
        outPosition: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x16F46FB18C8009E4u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x16F46FB18C8009E4u64;

        invoke_raw_typed!(
            hash,
                x, 
                y, 
                z, 
                p3, 
                outPosition
        )
    }
}

/// When this is set to false, all nodes in the area get disabled.

`GET_VEHICLE_NODE_IS_SWITCHED_OFF` returns true afterwards.

If it's true,

`GET_VEHICLE_NODE_IS_SWITCHED_OFF` returns false.



pub fn set_roads_in_area_safe(
        
        
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
        
        
            nodeEnabled: 
        , 
        
        
            unknown2: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xBF1A602B5BA52FEEu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xBF1A602B5BA52FEEu64;
        
        let result = invoke_raw!(
            hash,
                x1, 
                y1, 
                z1, 
                x2, 
                y2, 
                z2, 
                nodeEnabled, 
                unknown2
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn set_roads_in_area_raw(
        x1: , 
        y1: , 
        z1: , 
        x2: , 
        y2: , 
        z2: , 
        nodeEnabled: , 
        unknown2: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xBF1A602B5BA52FEEu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xBF1A602B5BA52FEEu64;

        invoke_raw_typed!(
            hash,
                x1, 
                y1, 
                z1, 
                x2, 
                y2, 
                z2, 
                nodeEnabled, 
                unknown2
        )
    }
}

/// ```
Used internally for long range tasks
```



pub fn _request_paths_prefer_accurate_boundingstruct_safe(
        
        
            x1: 
        , 
        
        
            y1: 
        , 
        
        
            x2: 
        , 
        
        
            y2: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x07FB139B592FA687u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x07FB139B592FA687u64;
        
        let result = invoke_raw!(
            hash,
                x1, 
                y1, 
                x2, 
                y2
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn _request_paths_prefer_accurate_boundingstruct_raw(
        x1: , 
        y1: , 
        x2: , 
        y2: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x07FB139B592FA687u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x07FB139B592FA687u64;

        invoke_raw_typed!(
            hash,
                x1, 
                y1, 
                x2, 
                y2
        )
    }
}

/// ## Parameters
*



pub fn get_num_navmeshes_existing_in_area_safe(
        
        
            posMinX: 
        , 
        
        
            posMinY: 
        , 
        
        
            posMinZ: 
        , 
        
        
            posMaxX: 
        , 
        
        
            posMaxY: 
        , 
        
        
            posMaxZ: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x01708E8DD3FF8C65u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x01708E8DD3FF8C65u64;
        
        let result = invoke_raw!(
            hash,
                posMinX, 
                posMinY, 
                posMinZ, 
                posMaxX, 
                posMaxY, 
                posMaxZ
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn get_num_navmeshes_existing_in_area_raw(
        posMinX: , 
        posMinY: , 
        posMinZ: , 
        posMaxX: , 
        posMaxY: , 
        posMaxZ: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x01708E8DD3FF8C65u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x01708E8DD3FF8C65u64;

        invoke_raw_typed!(
            hash,
                posMinX, 
                posMinY, 
                posMinZ, 
                posMaxX, 
                posMaxY, 
                posMaxZ
        )
    }
}

/// ```
Returns false for nodes that aren't used for GPS routes.
Example:
Nodes in Fort Zancudo and LSIA are false
```



pub fn get_vehicle_node_is_gps_allowed_safe(
        
        
            nodeID: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xA2AE5C478B96E3B6u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xA2AE5C478B96E3B6u64;
        
        let result = invoke_raw!(
            hash,
                nodeID
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn get_vehicle_node_is_gps_allowed_raw(
        nodeID: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xA2AE5C478B96E3B6u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xA2AE5C478B96E3B6u64;

        invoke_raw_typed!(
            hash,
                nodeID
        )
    }
}

/// ```
This native has been removed in v1180.  
```



pub fn load_all_path_nodes_safe(
        
        
            keepInMemory: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x80E4A6EDDB0BE8D9u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x80E4A6EDDB0BE8D9u64;
        
        let result = invoke_raw!(
            hash,
                keepInMemory
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn load_all_path_nodes_raw(
        keepInMemory: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x80E4A6EDDB0BE8D9u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x80E4A6EDDB0BE8D9u64;

        invoke_raw_typed!(
            hash,
                keepInMemory
        )
    }
}

/// ## Parameters
*



pub fn get_random_vehicle_node_safe(
        
        
            x: 
        , 
        
        
            y: 
        , 
        
        
            z: 
        , 
        
        
            radius: 
        , 
        
        
            p4: 
        , 
        
        
            p5: 
        , 
        
        
            p6: 
        , 
        
        
            outPosition: 
        , 
        
        
            nodeId: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x93E0DB8440B73A7Du32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x93E0DB8440B73A7Du64;
        
        let result = invoke_raw!(
            hash,
                x, 
                y, 
                z, 
                radius, 
                p4, 
                p5, 
                p6, 
                outPosition, 
                nodeId
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn get_random_vehicle_node_raw(
        x: , 
        y: , 
        z: , 
        radius: , 
        p4: , 
        p5: , 
        p6: , 
        outPosition: , 
        nodeId: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x93E0DB8440B73A7Du32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x93E0DB8440B73A7Du64;

        invoke_raw_typed!(
            hash,
                x, 
                y, 
                z, 
                radius, 
                p4, 
                p5, 
                p6, 
                outPosition, 
                nodeId
        )
    }
}

/// ## Parameters
*



pub fn are_nodes_loaded_for_area_safe(
        
        
            x1: 
        , 
        
        
            y1: 
        , 
        
        
            x2: 
        , 
        
        
            y2: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xF7B79A50B905A30Du32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xF7B79A50B905A30Du64;
        
        let result = invoke_raw!(
            hash,
                x1, 
                y1, 
                x2, 
                y2
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn are_nodes_loaded_for_area_raw(
        x1: , 
        y1: , 
        x2: , 
        y2: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xF7B79A50B905A30Du32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xF7B79A50B905A30Du64;

        invoke_raw_typed!(
            hash,
                x1, 
                y1, 
                x2, 
                y2
        )
    }
}

/// ```
Gets a value indicating whether the specified position is on a road.  
The vehicle parameter is not implemented (ignored).  
```



pub fn is_point_on_road_safe(
        
        
            x: 
        , 
        
        
            y: 
        , 
        
        
            z: 
        , 
        
        
            vehicle: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x125BF4ABFC536B09u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x125BF4ABFC536B09u64;
        
        let result = invoke_raw!(
            hash,
                x, 
                y, 
                z, 
                vehicle
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn is_point_on_road_raw(
        x: , 
        y: , 
        z: , 
        vehicle: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x125BF4ABFC536B09u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x125BF4ABFC536B09u64;

        invoke_raw_typed!(
            hash,
                x, 
                y, 
                z, 
                vehicle
        )
    }
}

/// ```
See: SET_BLIP_ROUTE
```



pub fn _set_ignore_secondary_route_nodes_safe(
        
        
            toggle: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x1FC289A0C3FF470Fu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x1FC289A0C3FF470Fu64;
        
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
pub fn _set_ignore_secondary_route_nodes_raw(
        toggle: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x1FC289A0C3FF470Fu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x1FC289A0C3FF470Fu64;

        invoke_raw_typed!(
            hash,
                toggle
        )
    }
}

/// ```c
enum eGetClosestNodeFlags {
    GCNF_INCLUDE_SWITCHED_OFF_NODES = 1,
    GCNF_INCLUDE_BOAT_NODES = 2,
    GCNF_IGNORE_SLIPLANES = 4,
    GCNF_IGNORE_SWITCHED_OFF_DEADENDS = 8,
    GCNF_GET_HEADING = 256,
    GCNF_FAVOUR_FACING = 512
}
```



pub fn get_closest_vehicle_node_safe(
        
        
            x: 
        , 
        
        
            y: 
        , 
        
        
            z: 
        , 
        
        
            outPosition: 
        , 
        
        
            nodeFlags: 
        , 
        
        
            zMeasureMult: 
        , 
        
        
            zTolerance: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x240A18690AE96513u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x240A18690AE96513u64;
        
        let result = invoke_raw!(
            hash,
                x, 
                y, 
                z, 
                outPosition, 
                nodeFlags, 
                zMeasureMult, 
                zTolerance
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn get_closest_vehicle_node_raw(
        x: , 
        y: , 
        z: , 
        outPosition: , 
        nodeFlags: , 
        zMeasureMult: , 
        zTolerance: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x240A18690AE96513u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x240A18690AE96513u64;

        invoke_raw_typed!(
            hash,
                x, 
                y, 
                z, 
                outPosition, 
                nodeFlags, 
                zMeasureMult, 
                zTolerance
        )
    }
}

/// ## Parameters
*



pub fn set_ambient_ped_range_multiplier_this_frame_safe(
        
        
            multiplier: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x0B919E1FB47CC4E0u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x0B919E1FB47CC4E0u64;
        
        let result = invoke_raw!(
            hash,
                multiplier
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn set_ambient_ped_range_multiplier_this_frame_raw(
        multiplier: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x0B919E1FB47CC4E0u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x0B919E1FB47CC4E0u64;

        invoke_raw_typed!(
            hash,
                multiplier
        )
    }
}

/// ## Parameters
*



pub fn get_nth_closest_vehicle_node_id_safe(
        
        
            x: 
        , 
        
        
            y: 
        , 
        
        
            z: 
        , 
        
        
            nthClosest: 
        , 
        
        
            nodeFlags: 
        , 
        
        
            zMeasureMult: 
        , 
        
        
            zTolerance: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x22D7275A79FE8215u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x22D7275A79FE8215u64;
        
        let result = invoke_raw!(
            hash,
                x, 
                y, 
                z, 
                nthClosest, 
                nodeFlags, 
                zMeasureMult, 
                zTolerance
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn get_nth_closest_vehicle_node_id_raw(
        x: , 
        y: , 
        z: , 
        nthClosest: , 
        nodeFlags: , 
        zMeasureMult: , 
        zTolerance: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x22D7275A79FE8215u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x22D7275A79FE8215u64;

        invoke_raw_typed!(
            hash,
                x, 
                y, 
                z, 
                nthClosest, 
                nodeFlags, 
                zMeasureMult, 
                zTolerance
        )
    }
}

/// ## Return value



pub fn get_gps_blip_route_length_safe(
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xBBB45C3CF5C8AA85u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xBBB45C3CF5C8AA85u64;
        
        let result = invoke_raw!(
            hash,
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn get_gps_blip_route_length_raw(
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xBBB45C3CF5C8AA85u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xBBB45C3CF5C8AA85u64;

        invoke_raw_typed!(
            hash,
        )
    }
}

/// ## Parameters
*



pub fn get_nth_closest_vehicle_node_id_with_heading_safe(
        
        
            x: 
        , 
        
        
            y: 
        , 
        
        
            z: 
        , 
        
        
            nthClosest: 
        , 
        
        
            outPosition: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x6448050E9C2A7207u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x6448050E9C2A7207u64;
        
        let result = invoke_raw!(
            hash,
                x, 
                y, 
                z, 
                nthClosest, 
                outPosition
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn get_nth_closest_vehicle_node_id_with_heading_raw(
        x: , 
        y: , 
        z: , 
        nthClosest: , 
        outPosition: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x6448050E9C2A7207u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x6448050E9C2A7207u64;

        invoke_raw_typed!(
            hash,
                x, 
                y, 
                z, 
                nthClosest, 
                outPosition
        )
    }
}

/// ```
Returns CGameWorldHeightMap's minimum Z among all grid nodes that intersect with the specified rectangle.
```



pub fn _get_heightmap_bottom_z_for_area_safe(
        
        
            x1: 
        , 
        
        
            y1: 
        , 
        
        
            x2: 
        , 
        
        
            y2: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x3599D741C9AC6310u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x3599D741C9AC6310u64;
        
        let result = invoke_raw!(
            hash,
                x1, 
                y1, 
                x2, 
                y2
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn _get_heightmap_bottom_z_for_area_raw(
        x1: , 
        y1: , 
        x2: , 
        y2: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x3599D741C9AC6310u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x3599D741C9AC6310u64;

        invoke_raw_typed!(
            hash,
                x1, 
                y1, 
                x2, 
                y2
        )
    }
}

/// REMOVE_NAVMESH_REQUIRED_REGIONS native function



pub fn remove_navmesh_required_regions_safe(
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x916F0A3CDEC3445Eu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x916F0A3CDEC3445Eu64;
        
        let result = invoke_raw!(
            hash,
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn remove_navmesh_required_regions_raw(
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x916F0A3CDEC3445Eu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x916F0A3CDEC3445Eu64;

        invoke_raw_typed!(
            hash,
        )
    }
}

/// Native to get a position along current player GPS route using supplied slot.
This native was previously named `GET_GPS_WAYPOINT_ROUTE_END`, but its named changed.

```c
enum eGpsSlotType {
	GPS_SLOT_WAYPOINT = 0,
	GPS_SLOT_RADAR_BLIP = 1,
	GPS_SLOT_DISCRETE = 2
}
```



pub fn get_pos_along_gps_type_route_safe(
        
        
            result: 
        , 
        
        
            bStartAtPlayerPos: 
        , 
        
        
            fDistanceAlongRoute: 
        , 
        
        
            slotType: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xF3162836C28F9DA5u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xF3162836C28F9DA5u64;
        
        let result = invoke_raw!(
            hash,
                result, 
                bStartAtPlayerPos, 
                fDistanceAlongRoute, 
                slotType
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn get_pos_along_gps_type_route_raw(
        result: , 
        bStartAtPlayerPos: , 
        fDistanceAlongRoute: , 
        slotType: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xF3162836C28F9DA5u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xF3162836C28F9DA5u64;

        invoke_raw_typed!(
            hash,
                result, 
                bStartAtPlayerPos, 
                fDistanceAlongRoute, 
                slotType
        )
    }
}

/// ## Parameters
*



pub fn remove_navmesh_blocking_object_safe(
        
        
            p0: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x46399A7895957C0Eu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x46399A7895957C0Eu64;
        
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
pub fn remove_navmesh_blocking_object_raw(
        p0: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x46399A7895957C0Eu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x46399A7895957C0Eu64;

        invoke_raw_typed!(
            hash,
                p0
        )
    }
}

/// ## Parameters
*



pub fn _0xaa76052dda9bfc3e_safe(
        
        
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
        let hash = 0xAA76052DDA9BFC3Eu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xAA76052DDA9BFC3Eu64;
        
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
pub fn _0xaa76052dda9bfc3e_raw(
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
        let hash = 0xAA76052DDA9BFC3Eu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xAA76052DDA9BFC3Eu64;

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

/// Same as [`GET_CLOSEST_VEHICLE_NODE`](#_0x240A18690AE96513), but with the node flag `GCNF_GET_HEADING` set, causing the native to also return the heading.



pub fn get_closest_vehicle_node_with_heading_safe(
        
        
            x: 
        , 
        
        
            y: 
        , 
        
        
            z: 
        , 
        
        
            outPosition: 
        , 
        
        
            outHeading: 
        , 
        
        
            nodeFlags: 
        , 
        
        
            zMeasureMult: 
        , 
        
        
            zTolerance: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xFF071FB798B803B0u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xFF071FB798B803B0u64;
        
        let result = invoke_raw!(
            hash,
                x, 
                y, 
                z, 
                outPosition, 
                outHeading, 
                nodeFlags, 
                zMeasureMult, 
                zTolerance
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn get_closest_vehicle_node_with_heading_raw(
        x: , 
        y: , 
        z: , 
        outPosition: , 
        outHeading: , 
        nodeFlags: , 
        zMeasureMult: , 
        zTolerance: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xFF071FB798B803B0u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xFF071FB798B803B0u64;

        invoke_raw_typed!(
            hash,
                x, 
                y, 
                z, 
                outPosition, 
                outHeading, 
                nodeFlags, 
                zMeasureMult, 
                zTolerance
        )
    }
}

/// ## Parameters
*



pub fn does_navmesh_blocking_object_exist_safe(
        
        
            p0: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x0EAEB0DB4B132399u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x0EAEB0DB4B132399u64;
        
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
pub fn does_navmesh_blocking_object_exist_raw(
        p0: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x0EAEB0DB4B132399u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x0EAEB0DB4B132399u64;

        invoke_raw_typed!(
            hash,
                p0
        )
    }
}

/// ## Parameters
*



pub fn get_road_boundary_using_heading_safe(
        
        
            x: 
        , 
        
        
            y: 
        , 
        
        
            z: 
        , 
        
        
            heading: 
        , 
        
        
            outPosition: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xA0F8A7517A273C05u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xA0F8A7517A273C05u64;
        
        let result = invoke_raw!(
            hash,
                x, 
                y, 
                z, 
                heading, 
                outPosition
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn get_road_boundary_using_heading_raw(
        x: , 
        y: , 
        z: , 
        heading: , 
        outPosition: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xA0F8A7517A273C05u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xA0F8A7517A273C05u64;

        invoke_raw_typed!(
            hash,
                x, 
                y, 
                z, 
                heading, 
                outPosition
        )
    }
}

/// Calculates the travel distance between a set of points.
Doesn't seem to correlate with distance on gps sometimes.

This function returns the value 100000.0 over long distances, seems to be a failure mode result, potentially occurring when not all path nodes are loaded into pathfind.



pub fn calculate_travel_distance_between_points_safe(
        
        
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
        let hash = 0xADD95C7005C4A197u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xADD95C7005C4A197u64;
        
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
pub fn calculate_travel_distance_between_points_raw(
        x1: , 
        y1: , 
        z1: , 
        x2: , 
        y2: , 
        z2: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xADD95C7005C4A197u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xADD95C7005C4A197u64;

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

/// Like [`GET_CLOSEST_VEHICLE_NODE_WITH_HEADING`](#_0xFF071FB798B803B0), but returns the nth closest node instead of the first.



pub fn get_nth_closest_vehicle_node_favour_direction_safe(
        
        
            x: 
        , 
        
        
            y: 
        , 
        
        
            z: 
        , 
        
        
            desiredX: 
        , 
        
        
            desiredY: 
        , 
        
        
            desiredZ: 
        , 
        
        
            nthClosest: 
        , 
        
        
            outPosition: 
        , 
        
        
            outHeading: 
        , 
        
        
            nodeFlags: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x45905BE8654AE067u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x45905BE8654AE067u64;
        
        let result = invoke_raw!(
            hash,
                x, 
                y, 
                z, 
                desiredX, 
                desiredY, 
                desiredZ, 
                nthClosest, 
                outPosition, 
                outHeading, 
                nodeFlags
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn get_nth_closest_vehicle_node_favour_direction_raw(
        x: , 
        y: , 
        z: , 
        desiredX: , 
        desiredY: , 
        desiredZ: , 
        nthClosest: , 
        outPosition: , 
        outHeading: , 
        nodeFlags: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x45905BE8654AE067u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x45905BE8654AE067u64;

        invoke_raw_typed!(
            hash,
                x, 
                y, 
                z, 
                desiredX, 
                desiredY, 
                desiredZ, 
                nthClosest, 
                outPosition, 
                outHeading, 
                nodeFlags
        )
    }
}

/// ```
Returns true if the id is non zero.  
```



pub fn is_vehicle_node_id_valid_safe(
        
        
            vehicleNodeId: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x1EAF30FCFBF5AF74u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x1EAF30FCFBF5AF74u64;
        
        let result = invoke_raw!(
            hash,
                vehicleNodeId
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn is_vehicle_node_id_valid_raw(
        vehicleNodeId: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x1EAF30FCFBF5AF74u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x1EAF30FCFBF5AF74u64;

        invoke_raw_typed!(
            hash,
                vehicleNodeId
        )
    }
}

/// ```
Returns CGameWorldHeightMap's maximum Z value at specified point (grid node).
```



pub fn _get_heightmap_top_z_for_position_safe(
        
        
            x: 
        , 
        
        
            y: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x29C24BFBED8AB8FBu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x29C24BFBED8AB8FBu64;
        
        let result = invoke_raw!(
            hash,
                x, 
                y
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn _get_heightmap_top_z_for_position_raw(
        x: , 
        y: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x29C24BFBED8AB8FBu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x29C24BFBED8AB8FBu64;

        invoke_raw_typed!(
            hash,
                x, 
                y
        )
    }
}

/// ```
NativeDB Added Parameter 7: Any p6
```



pub fn set_ped_paths_back_to_original_safe(
        
        
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
        let hash = 0xE04B48F2CC926253u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xE04B48F2CC926253u64;
        
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
pub fn set_ped_paths_back_to_original_raw(
        p0: , 
        p1: , 
        p2: , 
        p3: , 
        p4: , 
        p5: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xE04B48F2CC926253u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xE04B48F2CC926253u64;

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



pub fn set_ignore_no_gps_flag_safe(
        
        
            toggle: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x72751156E7678833u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x72751156E7678833u64;
        
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
pub fn set_ignore_no_gps_flag_raw(
        toggle: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x72751156E7678833u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x72751156E7678833u64;

        invoke_raw_typed!(
            hash,
                toggle
        )
    }
}

/// Use this if you want to completely disable a large area of navmesh.
For smaller areas, use [`ADD_NAVMESH_BLOCKING_OBJECT`](#_0xFCD5C8E06E502F5A) instead.



pub fn disable_navmesh_in_area_safe(
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x4C8872D8CDBE1B8Bu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x4C8872D8CDBE1B8Bu64;
        
        let result = invoke_raw!(
            hash,
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn disable_navmesh_in_area_raw(
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x4C8872D8CDBE1B8Bu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x4C8872D8CDBE1B8Bu64;

        invoke_raw_typed!(
            hash,
        )
    }
}

/// ```
missing a last parameter int p6  
```

```
NativeDB Added Parameter 7: Any p6
```



pub fn set_roads_back_to_original_safe(
        
        
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
        let hash = 0x1EE7063B80FFC77Cu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x1EE7063B80FFC77Cu64;
        
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
pub fn set_roads_back_to_original_raw(
        p0: , 
        p1: , 
        p2: , 
        p3: , 
        p4: , 
        p5: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x1EE7063B80FFC77Cu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x1EE7063B80FFC77Cu64;

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



pub fn update_navmesh_blocking_object_safe(
        
        
            object: 
        , 
        
        
            posX: 
        , 
        
        
            posY: 
        , 
        
        
            posZ: 
        , 
        
        
            scaleX: 
        , 
        
        
            scaleY: 
        , 
        
        
            scaleZ: 
        , 
        
        
            heading: 
        , 
        
        
            flags: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x109E99373F290687u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x109E99373F290687u64;
        
        let result = invoke_raw!(
            hash,
                object, 
                posX, 
                posY, 
                posZ, 
                scaleX, 
                scaleY, 
                scaleZ, 
                heading, 
                flags
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn update_navmesh_blocking_object_raw(
        object: , 
        posX: , 
        posY: , 
        posZ: , 
        scaleX: , 
        scaleY: , 
        scaleZ: , 
        heading: , 
        flags: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x109E99373F290687u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x109E99373F290687u64;

        invoke_raw_typed!(
            hash,
                object, 
                posX, 
                posY, 
                posZ, 
                scaleX, 
                scaleY, 
                scaleZ, 
                heading, 
                flags
        )
    }
}

/// Disables the GPS route displayed on the minimap while within a certain zone (area). When in a disabled zone and creating a waypoint, the GPS route is not shown on the minimap until you are outside of the zone. When disabled, the direct distance is shown on minimap opposed to distance to travel. Seems to only work before setting a waypoint.

You can clear the disabled zone with CLEAR_GPS_DISABLED_ZONE_AT_INDEX.



pub fn set_gps_disabled_zone_at_index_safe(
        
        
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
        
        
            index: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xD0BC1C6FB18EE154u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xD0BC1C6FB18EE154u64;
        
        let result = invoke_raw!(
            hash,
                x1, 
                y1, 
                z1, 
                x2, 
                y2, 
                z2, 
                index
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn set_gps_disabled_zone_at_index_raw(
        x1: , 
        y1: , 
        z1: , 
        x2: , 
        y2: , 
        z2: , 
        index: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xD0BC1C6FB18EE154u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xD0BC1C6FB18EE154u64;

        invoke_raw_typed!(
            hash,
                x1, 
                y1, 
                z1, 
                x2, 
                y2, 
                z2, 
                index
        )
    }
}

/// Activates Cayo Perico path nodes if passed `1`. GPS navigation will start working, maybe more stuff will change, not sure. It seems if you try to unload (pass `0`) when close to the island, your game might crash.

```
NativeDB Introduced: v2189
```



pub fn _set_ai_global_path_nodes_type_safe(
        
        
            type: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xF74B1FFA4A15FBEAu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xF74B1FFA4A15FBEAu64;
        
        let result = invoke_raw!(
            hash,
                type
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn _set_ai_global_path_nodes_type_raw(
        type: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xF74B1FFA4A15FBEAu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xF74B1FFA4A15FBEAu64;

        invoke_raw_typed!(
            hash,
                type
        )
    }
}

/// Finds an edge (node connection to another node) that satisfies the specified criteria.



pub fn get_closest_road_safe(
        
        
            x: 
        , 
        
        
            y: 
        , 
        
        
            z: 
        , 
        
        
            minimumEdgeLength: 
        , 
        
        
            minimumLaneCount: 
        , 
        
        
            srcNode: 
        , 
        
        
            targetNode: 
        , 
        
        
            laneCountForward: 
        , 
        
        
            laneCountBackward: 
        , 
        
        
            width: 
        , 
        
        
            onlyMajorRoads: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x132F52BBA570FE92u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x132F52BBA570FE92u64;
        
        let result = invoke_raw!(
            hash,
                x, 
                y, 
                z, 
                minimumEdgeLength, 
                minimumLaneCount, 
                srcNode, 
                targetNode, 
                laneCountForward, 
                laneCountBackward, 
                width, 
                onlyMajorRoads
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn get_closest_road_raw(
        x: , 
        y: , 
        z: , 
        minimumEdgeLength: , 
        minimumLaneCount: , 
        srcNode: , 
        targetNode: , 
        laneCountForward: , 
        laneCountBackward: , 
        width: , 
        onlyMajorRoads: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x132F52BBA570FE92u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x132F52BBA570FE92u64;

        invoke_raw_typed!(
            hash,
                x, 
                y, 
                z, 
                minimumEdgeLength, 
                minimumLaneCount, 
                srcNode, 
                targetNode, 
                laneCountForward, 
                laneCountBackward, 
                width, 
                onlyMajorRoads
        )
    }
}

/// ## Return value



pub fn get_gps_blip_route_found_safe(
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x869DAACBBE9FA006u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x869DAACBBE9FA006u64;
        
        let result = invoke_raw!(
            hash,
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn get_gps_blip_route_found_raw(
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x869DAACBBE9FA006u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x869DAACBBE9FA006u64;

        invoke_raw_typed!(
            hash,
        )
    }
}

/// Gets the next zone that has been disabled using SET_GPS_DISABLED_ZONE_AT_INDEX.

```
NativeDB Removed Parameter 1: int index
```



pub fn get_next_gps_disabled_zone_index_safe(
        
        
            index: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xD3A6A0EF48823A8Cu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xD3A6A0EF48823A8Cu64;
        
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
pub fn get_next_gps_disabled_zone_index_raw(
        index: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xD3A6A0EF48823A8Cu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xD3A6A0EF48823A8Cu64;

        invoke_raw_typed!(
            hash,
                index
        )
    }
}

/// Same as [`GET_CLOSEST_VEHICLE_NODE`](#_0x240A18690AE96513), but returns the nth closest node instead of the first.



pub fn get_nth_closest_vehicle_node_safe(
        
        
            x: 
        , 
        
        
            y: 
        , 
        
        
            z: 
        , 
        
        
            nthClosest: 
        , 
        
        
            outPosition: 
        , 
        
        
            nodeFlags: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xE50E52416CCF948Bu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xE50E52416CCF948Bu64;
        
        let result = invoke_raw!(
            hash,
                x, 
                y, 
                z, 
                nthClosest, 
                outPosition, 
                nodeFlags
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn get_nth_closest_vehicle_node_raw(
        x: , 
        y: , 
        z: , 
        nthClosest: , 
        outPosition: , 
        nodeFlags: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xE50E52416CCF948Bu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xE50E52416CCF948Bu64;

        invoke_raw_typed!(
            hash,
                x, 
                y, 
                z, 
                nthClosest, 
                outPosition, 
                nodeFlags
        )
    }
}

/// Gets the density and flags of the closest node to the specified position.  
Density is a value between 0 and 15, indicating how busy the road is.  

```c
enum eVehicleNodeProperties {
	OFF_ROAD = 1 << 0,
	ON_PLAYERS_ROAD =  1 << 1,
	NO_BIG_VEHICLES = 1 << 2,
	SWITCHED_OFF = 1 << 3,
	TUNNEL_OR_INTERIOR = 1 << 4,
	LEADS_TO_DEAD_END = 1 << 5,
	HIGHWAY = 1 << 6,
	JUNCTION = 1 << 7,
	TRAFFIC_LIGHT = 1 << 8,
	GIVE_WAY = 1 << 9,
	WATER = 1 << 10,
}
```



pub fn get_vehicle_node_properties_safe(
        
        
            x: 
        , 
        
        
            y: 
        , 
        
        
            z: 
        , 
        
        
            density: 
        , 
        
        
            flags: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x0568566ACBB5DEDCu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x0568566ACBB5DEDCu64;
        
        let result = invoke_raw!(
            hash,
                x, 
                y, 
                z, 
                density, 
                flags
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn get_vehicle_node_properties_raw(
        x: , 
        y: , 
        z: , 
        density: , 
        flags: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x0568566ACBB5DEDCu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x0568566ACBB5DEDCu64;

        invoke_raw_typed!(
            hash,
                x, 
                y, 
                z, 
                density, 
                flags
        )
    }
}

/// Same as [`GET_CLOSEST_VEHICLE_NODE`](#_0x240A18690AE96513), but with the node flag `GCNF_INCLUDE_SWITCHED_OFF_NODES` set.



pub fn get_closest_major_vehicle_node_safe(
        
        
            x: 
        , 
        
        
            y: 
        , 
        
        
            z: 
        , 
        
        
            outPosition: 
        , 
        
        
            zMeasureMult: 
        , 
        
        
            zTolerance: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x2EABE3B06F58C1BEu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x2EABE3B06F58C1BEu64;
        
        let result = invoke_raw!(
            hash,
                x, 
                y, 
                z, 
                outPosition, 
                zMeasureMult, 
                zTolerance
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn get_closest_major_vehicle_node_raw(
        x: , 
        y: , 
        z: , 
        outPosition: , 
        zMeasureMult: , 
        zTolerance: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x2EABE3B06F58C1BEu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x2EABE3B06F58C1BEu64;

        invoke_raw_typed!(
            hash,
                x, 
                y, 
                z, 
                outPosition, 
                zMeasureMult, 
                zTolerance
        )
    }
}

/// ```
Returns true when the node is Offroad. Alleys, some dirt roads, and carparks return true.
Normal roads where plenty of Peds spawn will return false
```



pub fn get_vehicle_node_is_switched_off_safe(
        
        
            nodeID: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x4F5070AA58F69279u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x4F5070AA58F69279u64;
        
        let result = invoke_raw!(
            hash,
                nodeID
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn get_vehicle_node_is_switched_off_raw(
        nodeID: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x4F5070AA58F69279u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x4F5070AA58F69279u64;

        invoke_raw_typed!(
            hash,
                nodeID
        )
    }
}

/// ```
Determines the name of the street which is the closest to the given coordinates.
x,y,z - the coordinates of the street
streetName - returns a hash to the name of the street the coords are on
crossingRoad - if the coordinates are on an intersection, a hash to the name of the crossing road
Note: the names are returned as hashes, the strings can be returned using the function HUD::GET_STREET_NAME_FROM_HASH_KEY.
```



pub fn get_street_name_at_coord_safe(
        
        
            x: 
        , 
        
        
            y: 
        , 
        
        
            z: 
        , 
        
        
            streetName: 
        , 
        
        
            crossingRoad: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x2EB41072B4C1E4C0u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x2EB41072B4C1E4C0u64;
        
        let result = invoke_raw!(
            hash,
                x, 
                y, 
                z, 
                streetName, 
                crossingRoad
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn get_street_name_at_coord_raw(
        x: , 
        y: , 
        z: , 
        streetName: , 
        crossingRoad: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x2EB41072B4C1E4C0u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x2EB41072B4C1E4C0u64;

        invoke_raw_typed!(
            hash,
                x, 
                y, 
                z, 
                streetName, 
                crossingRoad
        )
    }
}

/// ## Parameters
*



pub fn set_gps_disabled_zone_safe(
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xDC20483CD3DD5201u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xDC20483CD3DD5201u64;
        
        let result = invoke_raw!(
            hash,
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn set_gps_disabled_zone_raw(
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xDC20483CD3DD5201u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xDC20483CD3DD5201u64;

        invoke_raw_typed!(
            hash,
        )
    }
}

/// See [`IS_POINT_IN_ANGLED_AREA`](#_0x2A70BAE8883E4C81) for the definition of an angled area.

```
NativeDB Added Parameter 8: Any p7

bool p7 - always 1  
```



pub fn set_roads_back_to_original_in_angled_area_safe(
        
        
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
        let hash = 0x0027501B9F3B407Eu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x0027501B9F3B407Eu64;
        
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
pub fn set_roads_back_to_original_in_angled_area_raw(
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
        let hash = 0x0027501B9F3B407Eu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x0027501B9F3B407Eu64;

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
NativeDB Added Parameter 8: Any p7
```



pub fn set_ped_paths_in_area_safe(
        
        
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
        
        
            unknown: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x34F060F4BF92E018u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x34F060F4BF92E018u64;
        
        let result = invoke_raw!(
            hash,
                x1, 
                y1, 
                z1, 
                x2, 
                y2, 
                z2, 
                unknown
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn set_ped_paths_in_area_raw(
        x1: , 
        y1: , 
        z1: , 
        x2: , 
        y2: , 
        z2: , 
        unknown: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x34F060F4BF92E018u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x34F060F4BF92E018u64;

        invoke_raw_typed!(
            hash,
                x1, 
                y1, 
                z1, 
                x2, 
                y2, 
                z2, 
                unknown
        )
    }
}

/// ```
Toggles a global boolean, name is probably a hash collision but describes its functionality.
```



pub fn _set_all_paths_cache_boundingstruct_safe(
        
        
            toggle: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x228E5C6AD4D74BFDu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x228E5C6AD4D74BFDu64;
        
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
pub fn _set_all_paths_cache_boundingstruct_raw(
        toggle: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x228E5C6AD4D74BFDu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x228E5C6AD4D74BFDu64;

        invoke_raw_typed!(
            hash,
                toggle
        )
    }
}

/// ```
IS_*
```



pub fn _is_navmesh_required_region_owned_by_any_thread_safe(
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x705A844002B39DC0u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x705A844002B39DC0u64;
        
        let result = invoke_raw!(
            hash,
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn _is_navmesh_required_region_owned_by_any_thread_raw(
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x705A844002B39DC0u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x705A844002B39DC0u64;

        invoke_raw_typed!(
            hash,
        )
    }
}

/// ```
p3 is 0 in the only game script occurrence (trevor3) but 1 doesn't seem to make a difference
distToNxJunction seems to be the distance in metres * 10.0f
direction:
0 = This happens randomly during the drive for seemingly no reason but if you consider that this native is only used in trevor3, it seems to mean "Next frame, stop whatever's being said and tell the player the direction."
1 = Route is being calculated or the player is going in the wrong direction
2 = Please Proceed the Highlighted Route
3 = In (distToNxJunction) Turn Left
4 = In (distToNxJunction) Turn Right
5 = In (distToNxJunction) Keep Straight
6 = In (distToNxJunction) Turn Sharply To The Left
7 = In (distToNxJunction) Turn Sharply To The Right
8 = Route is being recalculated or the navmesh is confusing. This happens randomly during the drive but consistently at {2044.0358, 2996.6116, 44.9717} if you face towards the bar and the route needs you to turn right. In that particular case, it could be a bug with how the turn appears to be 270 deg. CCW instead of "right." Either way, this seems to be the engine saying "I don't know the route right now."
return value set to 0 always
```



pub fn generate_directions_to_coord_safe(
        
        
            x: 
        , 
        
        
            y: 
        , 
        
        
            z: 
        , 
        
        
            p3: 
        , 
        
        
            direction: 
        , 
        
        
            vehicle: 
        , 
        
        
            distToNxJunction: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xF90125F1F79ECDF8u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xF90125F1F79ECDF8u64;
        
        let result = invoke_raw!(
            hash,
                x, 
                y, 
                z, 
                p3, 
                direction, 
                vehicle, 
                distToNxJunction
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn generate_directions_to_coord_raw(
        x: , 
        y: , 
        z: , 
        p3: , 
        direction: , 
        vehicle: , 
        distToNxJunction: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xF90125F1F79ECDF8u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xF90125F1F79ECDF8u64;

        invoke_raw_typed!(
            hash,
                x, 
                y, 
                z, 
                p3, 
                direction, 
                vehicle, 
                distToNxJunction
        )
    }
}

/// ```
Calling this with an invalid node id, will crash the game.
Note that IS_VEHICLE_NODE_ID_VALID simply checks if nodeId is not zero. It does not actually ensure that the id is valid.
Eg. IS_VEHICLE_NODE_ID_VALID(1) will return true, but will crash when calling GET_VEHICLE_NODE_POSITION().
```



pub fn get_vehicle_node_position_safe(
        
        
            nodeId: 
        , 
        
        
            outPosition: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x703123E5E7D429C2u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x703123E5E7D429C2u64;
        
        let result = invoke_raw!(
            hash,
                nodeId, 
                outPosition
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn get_vehicle_node_position_raw(
        nodeId: , 
        outPosition: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x703123E5E7D429C2u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x703123E5E7D429C2u64;

        invoke_raw_typed!(
            hash,
                nodeId, 
                outPosition
        )
    }
}

/// ## Return value



pub fn are_all_navmesh_regions_loaded_safe(
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x8415D95B194A3AEAu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x8415D95B194A3AEAu64;
        
        let result = invoke_raw!(
            hash,
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn are_all_navmesh_regions_loaded_raw(
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x8415D95B194A3AEAu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x8415D95B194A3AEAu64;

        invoke_raw_typed!(
            hash,
        )
    }
}

/// ## Parameters
*



pub fn add_navmesh_required_region_safe(
        
        
            x: 
        , 
        
        
            y: 
        , 
        
        
            radius: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x387EAD7EE42F6685u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x387EAD7EE42F6685u64;
        
        let result = invoke_raw!(
            hash,
                x, 
                y, 
                radius
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn add_navmesh_required_region_raw(
        x: , 
        y: , 
        radius: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x387EAD7EE42F6685u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x387EAD7EE42F6685u64;

        invoke_raw_typed!(
            hash,
                x, 
                y, 
                radius
        )
    }
}

