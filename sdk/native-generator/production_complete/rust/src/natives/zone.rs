//! ZONE native functions
//! 
//! Functions for the zone category (local)

use crate::types::*;
use crate::errors::{NativeError, NativeResult, validation};
use nalgebra::Vector3;
use tracing::{debug, warn};

/// ## Parameters
*



pub fn set_zone_enabled_safe(
        
        
            zoneId: 
        , 
        
        
            toggle: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xBA5ECEEA120E5611u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xBA5ECEEA120E5611u64;
        
        let result = invoke_raw!(
            hash,
                zoneId, 
                toggle
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn set_zone_enabled_raw(
        zoneId: , 
        toggle: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xBA5ECEEA120E5611u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xBA5ECEEA120E5611u64;

        invoke_raw_typed!(
            hash,
                zoneId, 
                toggle
        )
    }
}

/// ## Parameters
*



pub fn get_zone_popschedule_safe(
        
        
            zoneId: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x4334BC40AA0CB4BBu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x4334BC40AA0CB4BBu64;
        
        let result = invoke_raw!(
            hash,
                zoneId
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn get_zone_popschedule_raw(
        zoneId: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x4334BC40AA0CB4BBu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x4334BC40AA0CB4BBu64;

        invoke_raw_typed!(
            hash,
                zoneId
        )
    }
}

/// ```
Only used once in the decompiled scripts. Seems to be related to scripted vehicle generators.  
Modified example from "am_imp_exp.c4", line 6418:  
/* popSchedules[0] = ZONE::GET_ZONE_POPSCHEDULE(ZONE::GET_ZONE_AT_COORDS(891.3, 807.9, 188.1));  
etc.  
*/  
STREAMING::SET_MODEL_AS_NO_LONGER_NEEDED(vehicleHash);  
ZONE::CLEAR_POPSCHEDULE_OVERRIDE_VEHICLE_MODEL(popSchedules[index]);  
```



pub fn clear_popschedule_override_vehicle_model_safe(
        
        
            scheduleId: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x5C0DE367AA0D911Cu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x5C0DE367AA0D911Cu64;
        
        let result = invoke_raw!(
            hash,
                scheduleId
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn clear_popschedule_override_vehicle_model_raw(
        scheduleId: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x5C0DE367AA0D911Cu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x5C0DE367AA0D911Cu64;

        invoke_raw_typed!(
            hash,
                scheduleId
        )
    }
}

/// ```
Only used once in the decompiled scripts. Seems to be related to scripted vehicle generators.  
Modified example from "am_imp_exp.c4", line 6406:  
/* popSchedules[0] = ZONE::GET_ZONE_POPSCHEDULE(ZONE::GET_ZONE_AT_COORDS(891.3, 807.9, 188.1));  
etc.  
*/  
ZONE::OVERRIDE_POPSCHEDULE_VEHICLE_MODEL(popSchedules[index], vehicleHash);  
STREAMING::REQUEST_MODEL(vehicleHash);  
```



pub fn override_popschedule_vehicle_model_safe(
        
        
            scheduleId: 
        , 
        
        
            vehicleHash: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x5F7D596BAC2E7777u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x5F7D596BAC2E7777u64;
        
        let result = invoke_raw!(
            hash,
                scheduleId, 
                vehicleHash
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn override_popschedule_vehicle_model_raw(
        scheduleId: , 
        vehicleHash: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x5F7D596BAC2E7777u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x5F7D596BAC2E7777u64;

        invoke_raw_typed!(
            hash,
                scheduleId, 
                vehicleHash
        )
    }
}

/// ## Parameters
*



pub fn get_zone_at_coords_safe(
        
        
            x: 
        , 
        
        
            y: 
        , 
        
        
            z: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x27040C25DE6CB2F4u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x27040C25DE6CB2F4u64;
        
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
pub fn get_zone_at_coords_raw(
        x: , 
        y: , 
        z: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x27040C25DE6CB2F4u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x27040C25DE6CB2F4u64;

        invoke_raw_typed!(
            hash,
                x, 
                y, 
                z
        )
    }
}

/// ```
AIRP = Los Santos International Airport  
ALAMO = Alamo Sea  
ALTA = Alta  
ARMYB = Fort Zancudo  
BANHAMC = Banham Canyon Dr  
BANNING = Banning  
BEACH = Vespucci Beach  
BHAMCA = Banham Canyon  
BRADP = Braddock Pass  
BRADT = Braddock Tunnel  
BURTON = Burton  
CALAFB = Calafia Bridge  
CANNY = Raton Canyon  
CCREAK = Cassidy Creek  
CHAMH = Chamberlain Hills  
CHIL = Vinewood Hills  
CHU = Chumash  
CMSW = Chiliad Mountain State Wilderness  
CYPRE = Cypress Flats  
DAVIS = Davis  
DELBE = Del Perro Beach  
DELPE = Del Perro  
DELSOL = La Puerta  
DESRT = Grand Senora Desert  
DOWNT = Downtown  
DTVINE = Downtown Vinewood  
EAST_V = East Vinewood  
EBURO = El Burro Heights  
ELGORL = El Gordo Lighthouse  
ELYSIAN = Elysian Island  
GALFISH = Galilee  
GOLF = GWC and Golfing Society  
GRAPES = Grapeseed  
GREATC = Great Chaparral  
HARMO = Harmony  
HAWICK = Hawick  
HORS = Vinewood Racetrack  
HUMLAB = Humane Labs and Research  
JAIL = Bolingbroke Penitentiary  
KOREAT = Little Seoul  
LACT = Land Act Reservoir  
LAGO = Lago Zancudo  
LDAM = Land Act Dam  
LEGSQU = Legion Square  
LMESA = La Mesa  
LOSPUER = La Puerta  
MIRR = Mirror Park  
MORN = Morningwood  
MOVIE = Richards Majestic  
MTCHIL = Mount Chiliad  
MTGORDO = Mount Gordo  
MTJOSE = Mount Josiah  
MURRI = Murrieta Heights  
NCHU = North Chumash  
NOOSE = N.O.O.S.E  
OCEANA = Pacific Ocean  
PALCOV = Paleto Cove  
PALETO = Paleto Bay  
PALFOR = Paleto Forest  
PALHIGH = Palomino Highlands  
PALMPOW = Palmer-Taylor Power Station  
PBLUFF = Pacific Bluffs  
PBOX = Pillbox Hill  
PROCOB = Procopio Beach  
RANCHO = Rancho  
RGLEN = Richman Glen  
RICHM = Richman  
ROCKF = Rockford Hills  
RTRAK = Redwood Lights Track  
SANAND = San Andreas  
SANCHIA = San Chianski Mountain Range  
SANDY = Sandy Shores  
SKID = Mission Row  
SLAB = Stab City  
STAD = Maze Bank Arena  
STRAW = Strawberry  
TATAMO = Tataviam Mountains  
TERMINA = Terminal  
TEXTI = Textile City  
TONGVAH = Tongva Hills  
TONGVAV = Tongva Valley  
VCANA = Vespucci Canals  
VESP = Vespucci  
VINE = Vinewood  
WINDF = Ron Alternates Wind Farm  
WVINE = West Vinewood  
ZANCUDO = Zancudo River  
ZP_ORT = Port of South Los Santos  
ZQ_UAR = Davis Quartz  
PROL = Prologue / North Yankton
ISHeist = Cayo Perico Island
```



pub fn get_name_of_zone_safe(
        
        
            x: 
        , 
        
        
            y: 
        , 
        
        
            z: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xCD90657D4C30E1CAu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xCD90657D4C30E1CAu64;
        
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
pub fn get_name_of_zone_raw(
        x: , 
        y: , 
        z: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xCD90657D4C30E1CAu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xCD90657D4C30E1CAu64;

        invoke_raw_typed!(
            hash,
                x, 
                y, 
                z
        )
    }
}

/// ```
Returns a hash representing which part of the map the given coords are located.  
Possible return values:  
(Hash of) city -> -289320599  
(Hash of) countryside -> 2072609373  
C# Example :  
Ped player = Game.Player.Character;  
Hash h = Function.Call<Hash>(Hash.GET_HASH_OF_MAP_AREA_AT_COORDS, player.Position.X, player.Position.Y, player.Position.Z);  
```



pub fn get_hash_of_map_area_at_coords_safe(
        
        
            x: 
        , 
        
        
            y: 
        , 
        
        
            z: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x7EE64D51E8498728u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x7EE64D51E8498728u64;
        
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
pub fn get_hash_of_map_area_at_coords_raw(
        x: , 
        y: , 
        z: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x7EE64D51E8498728u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x7EE64D51E8498728u64;

        invoke_raw_typed!(
            hash,
                x, 
                y, 
                z
        )
    }
}

/// Gets the zone scumminess level, used to calculate the cellphone signal strength.

```c
enum eZoneScumminess
{
    SCUMMINESS_POSH = 0,
    SCUMMINESS_NICE = 1,
    SCUMMINESS_ABOVE_AVERAGE = 2,
    SCUMMINESS_BELOW_AVERAGE = 3,
    SCUMMINESS_CRAP = 4,
    SCUMMINESS_SCUM = 5
}
```



pub fn get_zone_scumminess_safe(
        
        
            zoneId: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x5F7B268D15BA0739u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x5F7B268D15BA0739u64;
        
        let result = invoke_raw!(
            hash,
                zoneId
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn get_zone_scumminess_raw(
        zoneId: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x5F7B268D15BA0739u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x5F7B268D15BA0739u64;

        invoke_raw_typed!(
            hash,
                zoneId
        )
    }
}

/// ```
'zoneName' corresponds to an entry in 'popzone.ipl'.  
AIRP = Los Santos International Airport  
ALAMO = Alamo Sea  
ALTA = Alta  
ARMYB = Fort Zancudo  
BANHAMC = Banham Canyon Dr  
BANNING = Banning  
BEACH = Vespucci Beach  
BHAMCA = Banham Canyon  
BRADP = Braddock Pass  
BRADT = Braddock Tunnel  
BURTON = Burton  
CALAFB = Calafia Bridge  
CANNY = Raton Canyon  
CCREAK = Cassidy Creek  
CHAMH = Chamberlain Hills  
CHIL = Vinewood Hills  
CHU = Chumash  
CMSW = Chiliad Mountain State Wilderness  
CYPRE = Cypress Flats  
DAVIS = Davis  
DELBE = Del Perro Beach  
DELPE = Del Perro  
DELSOL = La Puerta  
DESRT = Grand Senora Desert  
DOWNT = Downtown  
DTVINE = Downtown Vinewood  
EAST_V = East Vinewood  
EBURO = El Burro Heights  
ELGORL = El Gordo Lighthouse  
ELYSIAN = Elysian Island  
GALFISH = Galilee  
GOLF = GWC and Golfing Society  
GRAPES = Grapeseed  
GREATC = Great Chaparral  
HARMO = Harmony  
HAWICK = Hawick  
HORS = Vinewood Racetrack  
HUMLAB = Humane Labs and Research  
JAIL = Bolingbroke Penitentiary  
KOREAT = Little Seoul  
LACT = Land Act Reservoir  
LAGO = Lago Zancudo  
LDAM = Land Act Dam  
LEGSQU = Legion Square  
LMESA = La Mesa  
LOSPUER = La Puerta  
MIRR = Mirror Park  
MORN = Morningwood  
MOVIE = Richards Majestic  
MTCHIL = Mount Chiliad  
MTGORDO = Mount Gordo  
MTJOSE = Mount Josiah  
MURRI = Murrieta Heights  
NCHU = North Chumash  
NOOSE = N.O.O.S.E  
OCEANA = Pacific Ocean  
PALCOV = Paleto Cove  
PALETO = Paleto Bay  
PALFOR = Paleto Forest  
PALHIGH = Palomino Highlands  
PALMPOW = Palmer-Taylor Power Station  
PBLUFF = Pacific Bluffs  
PBOX = Pillbox Hill  
PROCOB = Procopio Beach  
RANCHO = Rancho  
RGLEN = Richman Glen  
RICHM = Richman  
ROCKF = Rockford Hills  
RTRAK = Redwood Lights Track  
SANAND = San Andreas  
SANCHIA = San Chianski Mountain Range  
SANDY = Sandy Shores  
SKID = Mission Row  
SLAB = Stab City  
STAD = Maze Bank Arena  
STRAW = Strawberry  
TATAMO = Tataviam Mountains  
TERMINA = Terminal  
TEXTI = Textile City  
TONGVAH = Tongva Hills  
TONGVAV = Tongva Valley  
VCANA = Vespucci Canals  
VESP = Vespucci  
VINE = Vinewood  
WINDF = Ron Alternates Wind Farm  
WVINE = West Vinewood  
ZANCUDO = Zancudo River  
ZP_ORT = Port of South Los Santos  
ZQ_UAR = Davis Quartz  
```



pub fn get_zone_from_name_id_safe(
        
        
            zoneName: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x98CD1D2934B76CC1u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x98CD1D2934B76CC1u64;
        
        let result = invoke_raw!(
            hash,
                zoneName
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn get_zone_from_name_id_raw(
        zoneName: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x98CD1D2934B76CC1u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x98CD1D2934B76CC1u64;

        invoke_raw_typed!(
            hash,
                zoneName
        )
    }
}

