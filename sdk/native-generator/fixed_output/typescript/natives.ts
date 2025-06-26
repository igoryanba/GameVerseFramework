// Auto-generated TypeScript definitions

/**
 * ```
 * On accelerating, spins the driven wheels with the others braked, so you don't go anywhere.  
 * ```
 * @param vehicle - 
 * @param toggle - 
 * @returns void
 * @remarks Hash: 0xFB8794444A7D60FB
 */
export function setVehicleBurnout(vehicle: number, toggle: number): void;

/**
 * ```
 * NativeDB Added Parameter 1: Vehicle vehicle
 * NativeDB Added Parameter 2: Any p1
 * ```
 * @returns void
 * @remarks Hash: 0xDCE97BDF8A0EABC8
 */
export function 0xdce97bdf8a0eabc8(): void;

/**
 * ```
 * Sets a vehicle on the ground on all wheels.  Returns whether or not the operation was successful.  
 * ```
 * 
 * ```
 * NativeDB Added Parameter 2: float p1
 * ```
 * @param vehicle - 
 * @returns number
 * @remarks Hash: 0x49733E92263139D1
 */
export function setVehicleOnGroundProperly(vehicle: number): number;

/**
 * ## Parameters
 * *
 * @param vehicle - 
 * @returns number
 * @remarks Hash: 0xAEA8FD591FAD4106
 */
export function isPlaybackUsingAiGoingOnForVehicle(vehicle: number): number;

/**
 * ```c
 * enum eVehicleWheelType
 * {
 *     VWT_SPORT = 0,
 *     VWT_MUSCLE = 1,
 *     VWT_LOWRIDER = 2,
 *     VWT_SUV = 3,
 *     VWT_OFFROAD = 4,
 *     VWT_TUNER = 5,
 *     VWT_BIKE = 6,
 *     VWT_HIEND = 7,
 *     // Benny's Original
 *     VWT_SUPERMOD1 = 8,
 *     // Benny's Bespoke
 *     VWT_SUPERMOD2 = 9,
 *     // Open Wheel
 *     VWT_SUPERMOD3 = 10,
 *     // Street
 *     VWT_SUPERMOD4 = 11,
 *     // Track
 *     VWT_SUPERMOD5 = 12,
 * };
 * ```
 * @param vehicle - 
 * @returns number
 * @remarks Hash: 0xB3ED1BFB4BE636DC
 */
export function getVehicleWheelType(vehicle: number): number;

/**
 * ## Parameters
 * *
 * @param multiplier - 
 * @returns void
 * @remarks Hash: 0xEAE6DCC7EEE3DB1D
 */
export function setParkedVehicleDensityMultiplierThisFrame(multiplier: number): void;

/**
 * ## Parameters
 * *
 * @param p0 - 
 * @param p1 - 
 * @param p2 - 
 * @param p3 - 
 * @param p4 - 
 * @param p5 - 
 * @param p6 - 
 * @returns void
 * @remarks Hash: 0x2FA9923062DD396C
 */
export function addVehicleStuckCheckWithWarp(p0: any, p1: number, p2: any, p3: number, p4: number, p5: number, p6: any): void;

/**
 * ## Parameters
 * *
 * @param vehicle - 
 * @returns number
 * @remarks Hash: 0x42BC05C27A946054
 */
export function getCurrentPlaybackForVehicle(vehicle: number): number;

/**
 * ```
 * From the driver's perspective, is the right headlight broken.  
 * ```
 * @param vehicle - 
 * @returns number
 * @remarks Hash: 0xA7ECB73355EB2F20
 */
export function getIsRightVehicleHeadlightDamaged(vehicle: number): number;

/**
 * ## Parameters
 * *
 * @param recording - 
 * @returns number
 * @remarks Hash: 0x0E48D1C262390950
 */
export function getTotalDurationOfVehicleRecording(recording: number): number;

/**
 * ```
 * Returns true when in a vehicle, false whilst entering/exiting.  
 * ```
 * @param vehicle - 
 * @returns number
 * @remarks Hash: 0xAE31E7DF9B5B132E
 */
export function getIsVehicleEngineRunning(vehicle: number): number;

/**
 * Sets the specified door index open on the passed vehicle. See [`IS_VEHICLE_DOOR_FULLY_OPEN`](#_0x3E933CFF7B111C22).
 * @param vehicle - 
 * @param doorIndex - 
 * @param loose - 
 * @param openInstantly - 
 * @returns void
 * @remarks Hash: 0x7C65DAC73C35C862
 */
export function setVehicleDoorOpen(vehicle: number, doorIndex: number, loose: number, openInstantly: number): void;

/**
 * ```
 * Gets the trailer of a vehicle and puts it into the trailer parameter.  
 * ```
 * @param vehicle - 
 * @param trailer - 
 * @returns number
 * @remarks Hash: 0x1CDD6BADC297830D
 */
export function getVehicleTrailerVehicle(vehicle: number, trailer: number): number;

/**
 * ## Parameters
 * *
 * @param vehicle - 
 * @param p1 - 
 * @returns void
 * @remarks Hash: 0xE851E480B814D4BA
 */
export function 0xe851e480b814d4ba(vehicle: number, p1: number): void;

/**
 * ## Parameters
 * *
 * @param vehicle - 
 * @param color - 
 * @returns void
 * @remarks Hash: 0x6089CDF6A57F326C
 */
export function SetVehicleDashboardColor(vehicle: number, color: number): void;

/**
 * ## Parameters
 * *
 * @param vehicle - 
 * @param p1 - 
 * @returns void
 * @remarks Hash: 0xAB04325045427AAE
 */
export function 0xab04325045427aae(vehicle: number, p1: number): void;

/**
 * ## Parameters
 * *
 * @param vehicle - 
 * @param speed - 
 * @returns void
 * @remarks Hash: 0x6683AB880E427778
 */
export function setPlaybackSpeed(vehicle: number, speed: number): void;

/**
 * ## Parameters
 * *
 * @param cargobob - 
 * @param vehicle - 
 * @param vehicleBoneIndex - 
 * @param x - 
 * @param y - 
 * @param z - 
 * @returns void
 * @remarks Hash: 0x4127F1D84E347769
 */
export function attachVehicleToCargobob(cargobob: number, vehicle: number, vehicleBoneIndex: number, x: number, y: number, z: number): void;

/**
 * ## Parameters
 * *
 * @param vehicle - 
 * @returns number
 * @remarks Hash: 0x28D37D4F71AC5C58
 */
export function getVehicleLayoutHash(vehicle: number): number;

/**
 * ## Parameters
 * *
 * @param vehicle - 
 * @param player - 
 * @returns number
 * @remarks Hash: 0xF6AF6CB341349015
 */
export function getVehicleDoorsLockedForPlayer(vehicle: number, player: number): number;

/**
 * ```
 * Can be used with GET_TOTAL_DURATION_OF_VEHICLE_RECORDING{_ID} to compute a percentage.
 * ```
 * @param vehicle - 
 * @returns number
 * @remarks Hash: 0x5746F3A7AB7FE544
 */
export function getTimePositionInRecording(vehicle: number): number;

/**
 * ```
 * Sets the wanted state of this vehicle.  
 * ```
 * @param vehicle - 
 * @param state - 
 * @returns void
 * @remarks Hash: 0xF7EC25A3EBEEC726
 */
export function setVehicleIsWanted(vehicle: number, state: number): void;

/**
 * ## Parameters
 * *
 * @param id - 
 * @param time - 
 * @returns { x: number, y: number, z: number }
 * @remarks Hash: 0xF0F2103EFAF8CBA7
 */
export function getRotationOfVehicleRecordingIdAtTime(id: number, time: number): { x: number, y: number, z: number };

/**
 * Does not work for vehicle of type: CBike, CBmx, CBoat, CTrain, CSubmarine.
 * @param vehicle - 
 * @returns number
 * @remarks Hash: 0x3DE51E9C80B116CF
 */
export function IsVehicleParachuteActive(vehicle: number): number;

/**
 * ```
 * Returns the text label of a mod type for a given vehicle  
 * Use _GET_LABEL_TEXT to get the part name in the game's language  
 * ```
 * @param vehicle - 
 * @param modType - 
 * @param modValue - 
 * @returns string
 * @remarks Hash: 0x8935624F8C5592CC
 */
export function getModTextLabel(vehicle: number, modType: number, modValue: number): string;

/**
 * See eDoorId declared in [`SET_VEHICLE_DOOR_SHUT`](#_0x93D9BD300D7789E5)
 * @param veh - 
 * @param doorID - 
 * @returns number
 * @remarks Hash: 0xB8E181E559464527
 */
export function isVehicleDoorDamaged(veh: number, doorID: number): number;

/**
 * ```
 * Retracts the hook on the cargobob.  
 * Note: after you retract it the natives for dropping the hook no longer work  
 * ```
 * @param cargobob - 
 * @returns void
 * @remarks Hash: 0x9768CF648F54C804
 */
export function removePickUpRopeForCargobob(cargobob: number): void;

/**
 * ```
 * SET_VEHICLE_D*
 * ```
 * @param vehicle - 
 * @param p1 - 
 * @returns any
 * @remarks Hash: 0x4E20D2A627011E8E
 */
export function SetVehicleDamageModifier(vehicle: number, p1: number): any;

/**
 * ## Parameters
 * *
 * @param vehicle - 
 * @returns number
 * @remarks Hash: 0x6325D1A044AE510D
 */
export function getVehicleModKit(vehicle: number): number;

/**
 * ```
 * Sets the turn signal enabled for a vehicle.  
 * Set turnSignal to 1 for left light, 0 for right light.  
 * ```
 * @param vehicle - 
 * @param turnSignal - 
 * @param toggle - 
 * @returns void
 * @remarks Hash: 0xB5D45264751B7DF0
 */
export function setVehicleIndicatorLights(vehicle: number, turnSignal: number, toggle: number): void;

/**
 * ```
 * From the driver's perspective, is the left headlight broken.  
 * ```
 * @param vehicle - 
 * @returns number
 * @remarks Hash: 0x5EF77C9ADD3B11A3
 */
export function getIsLeftVehicleHeadlightDamaged(vehicle: number): number;

/**
 * ## Parameters
 * *
 * @param x - 
 * @param y - 
 * @param z - 
 * @param radius - 
 * @returns number
 * @remarks Hash: 0x61E1DD6125A3EEE6
 */
export function isAnyVehicleNearPoint(x: number, y: number, z: number, radius: number): number;

/**
 * ## Parameters
 * *
 * @param vehicle - 
 * @param health - 
 * @returns void
 * @remarks Hash: 0xFE205F38AAA58E5B
 */
export function SetHeliTailRotorHealth(vehicle: number, health: number): void;

/**
 * ## Parameters
 * *
 * @param vehicle - 
 * @returns number
 * @remarks Hash: 0xF7F203E31F96F6A1
 */
export function isSeatWarpOnly(vehicle: number): number;

/**
 * ```
 * Only called once inside main_persitant with the parameters, 0  
 * ```
 * @param trackIndex - 
 * @param frequency - 
 * @returns void
 * @remarks Hash: 0x21973BBF8D17EDFA
 */
export function setTrainTrackSpawnFrequency(trackIndex: number, frequency: number): void;

/**
 * ```
 * Finds the vehicle that is carrying this entity with a handler frame.
 * The model of the entity must be prop_contr_03b_ld or the function will return 0.
 * ```
 * @param entity - 
 * @returns number
 * @remarks Hash: 0x375E7FC44F21C8AB
 */
export function FindVehicleCarryingThisEntity(entity: number): number;

/**
 * Queries whether the control panels of a plane are intact. This native is used to determine the operational status of a plane's cockpit controls, which can affect the plane's flyability.
 * @param vehicle - 
 * @param checkForZeroHealth - 
 * @returns number
 * @remarks Hash: 0xF78F94D60248C737
 */
export function arePlaneControlPanelsIntact(vehicle: number, checkForZeroHealth: number): number;

/**
 * ## Parameters
 * *
 * @param vehicle - 
 * @returns void
 * @remarks Hash: 0xFC40CBF7B90CA77C
 */
export function setCarBootOpen(vehicle: number): void;

/**
 * ```
 * It switch to highbeam when p1 is set to true.  
 * ```
 * @param vehicle - 
 * @param toggle - 
 * @returns void
 * @remarks Hash: 0x8B7FD87F0DDB421E
 */
export function setVehicleFullbeam(vehicle: number, toggle: number): void;

/**
 * ## Parameters
 * *
 * @param vehicle - 
 * @param toggle - 
 * @returns void
 * @remarks Hash: 0x465BF26AB9684352
 */
export function setDisableVehiclePetrolTankFires(vehicle: number, toggle: number): void;

/**
 * ```
 * Returns an int  
 * Vehicle Classes:  
 * 0: Compacts  
 * 1: Sedans  
 * 2: SUVs  
 * 3: Coupes  
 * 4: Muscle  
 * 5: Sports Classics  
 * 6: Sports  
 * 7: Super  
 * 8: Motorcycles  
 * 9: Off-road  
 * 10: Industrial  
 * 11: Utility  
 * 12: Vans  
 * 13: Cycles  
 * 14: Boats  
 * 15: Helicopters  
 * 16: Planes  
 * 17: Service  
 * 18: Emergency  
 * 19: Military  
 * 20: Commercial  
 * 21: Trains  
 * 22: Open Wheel
 * char buffer[128];  
 * std::sprintf(buffer, "VEH_CLASS_%i", VEHICLE::GET_VEHICLE_CLASS(vehicle));  
 * char* className = UI::_GET_LABEL_TEXT(buffer);  
 * ```
 * @param vehicle - 
 * @returns number
 * @remarks Hash: 0x29439776AAA00A62
 */
export function getVehicleClass(vehicle: number): number;

/**
 * Usex in decompiled scripts in combination with [`_GET_VEHICLE_SUSPENSION_BOUNDS`](#_0xDF7E3EEB29642C38).
 * 
 * NativeDB Introduced: v1180
 * 
 * ```
 * // Example from fm_bj_race_controller.c
 * if (!VEHICLE::_0x51F30DB60626A20E(uParam0->f_26, uParam0->f_12.f_3, uParam0->f_12, 2, 1) && !func_282(uParam0->f_6))
 * {
 *     VEHICLE::_GET_VEHICLE_SUSPENSION_BOUNDS(*uParam0, &vVar15, &uVar16);
 *     VEHICLE::_GET_VEHICLE_SUSPENSION_BOUNDS(uParam0->f_26, &vVar17, &uVar18);
 *     fVar19 = SYSTEM::VDIST2(0f, 0f, vVar15.z, 0f, 0f, vVar17.z);
 *     uParam0->f_12.f_3.f_2 = (uParam0->f_12.f_3.f_2 + fVar19);
 *     if (!VEHICLE::_0x51F30DB60626A20E(uParam0->f_26, uParam0->f_12.f_3, uParam0->f_12, 2, 1))
 *     {
 *         uParam0->f_12.f_3 = { uParam0->f_6 };
 *         uParam0->f_12 = { uParam0->f_9 };
 *     }
 * }
 * ```
 * @param vehicle - 
 * @param x - 
 * @param y - 
 * @param z - 
 * @param rotX - 
 * @param rotY - 
 * @param rotZ - 
 * @param p7 - 
 * @param p8 - 
 * @returns number
 * @remarks Hash: 0x51F30DB60626A20E
 */
export function 0x51f30db60626a20e(vehicle: number, x: number, y: number, z: number, rotX: number, rotY: number, rotZ: number, p7: number, p8: any): number;

/**
 * ## Parameters
 * *
 * @param vehicle - 
 * @param p1 - 
 * @param p2 - 
 * @returns void
 * @remarks Hash: 0x9BDDC73CC6A115D4
 */
export function 0x9bddc73cc6a115d4(vehicle: number, p1: number, p2: number): void;

/**
 * ## Parameters
 * *
 * @param vehicle - 
 * @returns number
 * @remarks Hash: 0x62CA17B74C435651
 */
export function isAnyEntityAttachedToHandlerFrame(vehicle: number): number;

/**
 * ```
 * returns a string which is the codename of the vehicle's currently selected primary color  
 * p1 is always 0  
 * ```
 * @param vehicle - 
 * @param p1 - 
 * @returns string
 * @remarks Hash: 0xB45085B721EFD38C
 */
export function getVehicleModColor1Name(vehicle: number, p1: number): string;

/**
 * Last named native above this one is `TRACK_VEHICLE_VISIBILITY` and first named native below is `UNCUFF_PED`. 
 * Unknown what it does, couldn't find good examples in the decompiled scripts.
 * @param p0 - 
 * @param p1 - 
 * @param p2 - 
 * @returns any
 * @remarks Hash: 0x725012A415DBA050
 */
export function 0x725012a415dba050(p0: any, p1: any, p2: any): any;

/**
 * See eDoorId declared in [`SET_VEHICLE_DOOR_SHUT`](#_0x93D9BD300D7789E5)
 * @param vehicle - 
 * @param doorIndex - 
 * @param forceClose - 
 * @param lock - 
 * @param p4 - 
 * @returns void
 * @remarks Hash: 0xA5A9653A8D2CAF48
 */
export function setVehicleDoorLatched(vehicle: number, doorIndex: number, forceClose: number, lock: number, p4: number): void;

/**
 * ## Parameters
 * *
 * @param vehicle - 
 * @param plateText - 
 * @returns void
 * @remarks Hash: 0x95A88F0B409CDA47
 */
export function setVehicleNumberPlateText(vehicle: number, plateText: string): void;

/**
 * ## Parameters
 * *
 * @param vehicle - 
 * @param front - 
 * @returns number
 * @remarks Hash: 0x468056A6BB6F3846
 */
export function isVehicleBumperBrokenOff(vehicle: number, front: number): number;

/**
 * ## Parameters
 * *
 * @param vehicle - 
 * @param angle - 
 * @returns void
 * @remarks Hash: 0x9AA47FFF660CB932
 */
export function setVehicleFlightNozzlePositionImmediate(vehicle: number, angle: number): void;

/**
 * See eWindowId declared in [`IS_VEHICLE_WINDOW_INTACT`](#_0x46E571A0E20D01F1).
 * @param vehicle - 
 * @param windowIndex - 
 * @returns void
 * @remarks Hash: 0x602E548F46E24D59
 */
export function rollUpWindow(vehicle: number, windowIndex: number): void;

/**
 * @returns void
 * @remarks Hash: 0xAA3F739ABDDCF21F
 */
export function ClearVehiclePhoneExplosiveDevice(): void;

/**
 * ## Parameters
 * *
 * @param toggle - 
 * @returns void
 * @remarks Hash: 0x84436EC293B1415F
 */
export function setRandomBoats(toggle: number): void;

/**
 * ## Parameters
 * *
 * @param vehicle - 
 * @param active - 
 * @returns void
 * @remarks Hash: 0x81E1552E35DC3839
 */
export function SetVehicleRocketBoostActive(vehicle: number, active: number): void;

/**
 * ```
 * NativeDB Introduced: v1290
 * ```
 * @param vehicle - 
 * @param toggle - 
 * @returns void
 * @remarks Hash: 0xB68CFAF83A02768D
 */
export function 0xb68cfaf83a02768d(vehicle: number, toggle: number): void;

/**
 * ```
 * Deletes a vehicle.  
 * The vehicle must be a mission entity to delete, so call this before deleting: SET_ENTITY_AS_MISSION_ENTITY(vehicle, true, true);  
 * eg how to use:  
 * SET_ENTITY_AS_MISSION_ENTITY(vehicle, true, true);  
 * DELETE_VEHICLE(&vehicle);  
 * Deletes the specified vehicle, then sets the handle pointed to by the pointer to NULL.  
 * ```
 * @param vehicle - 
 * @returns void
 * @remarks Hash: 0xEA386986E786A54F
 */
export function deleteVehicle(vehicle: number): void;

/**
 * ```
 * GET_VEHICLE_MODEL_*
 * 9.8 * thrust if air vehicle, else 0.38 + drive force?
 * ```
 * @param modelHash - 
 * @returns number
 * @remarks Hash: 0x53409B5163D5B846
 */
export function GetVehicleModelEstimatedAgility(modelHash: number): number;

/**
 * Sets whether the vehicle's lights can be broken.
 * 
 * ```
 * NativeDB Introduced: v323
 * ```
 * @param vehicle - 
 * @param toggle - 
 * @returns void
 * @remarks Hash: 0x1AA8A837D2169D94
 */
export function setVehicleHasUnbreakableLights(vehicle: number, toggle: number): void;

/**
 * See eDoorId declared in [`SET_VEHICLE_DOOR_SHUT`](#_0x93D9BD300D7789E5)
 * @param vehicle - 
 * @param doorIndex - 
 * @param deleteDoor - 
 * @returns void
 * @remarks Hash: 0xD4D4F6A4AB575A33
 */
export function setVehicleDoorBroken(vehicle: number, doorIndex: number, deleteDoor: number): void;

/**
 * ## Parameters
 * *
 * @param vehicle - 
 * @param toggle - 
 * @returns void
 * @remarks Hash: 0x28B18377EB6E25F6
 */
export function SetHydraulicRaised(vehicle: number, toggle: number): void;

/**
 * ## Parameters
 * *
 * @param p0 - 
 * @param p1 - 
 * @returns void
 * @remarks Hash: 0x1312DDD8385AEE4E
 */
export function 0x1312ddd8385aee4e(p0: any, p1: any): void;

/**
 * ## Parameters
 * *
 * @param vehicle - 
 * @returns void
 * @remarks Hash: 0x6D6AF961B72728AE
 */
export function clearVehicleRouteHistory(vehicle: number): void;

/**
 * ## Parameters
 * *
 * @param cargobob - 
 * @param vehicle - 
 * @returns void
 * @remarks Hash: 0x0E21D3DF1051399D
 */
export function detachVehicleFromCargobob(cargobob: number, vehicle: number): void;

/**
 * ```
 * p1 is always 0  
 * ```
 * @param train - 
 * @param p1 - 
 * @returns void
 * @remarks Hash: 0xBBE7648349B49BE8
 */
export function setMissionTrainAsNoLongerNeeded(train: number, p1: number): void;

/**
 * ```
 * Max value is 32767
 * ```
 * @param vehicle - 
 * @param range - 
 * @returns void
 * @remarks Hash: 0x79DF7E806202CE01
 */
export function setVehicleExtendedRemovalRange(vehicle: number, range: number): void;

/**
 * ```
 * Setting this to false, makes the specified vehicle to where if you press Y your character doesn't even attempt the animation to enter the vehicle. Hence it's not considered aka ignored.  
 * ```
 * @param vehicle - 
 * @param toggle - 
 * @returns void
 * @remarks Hash: 0x31B927BBC44156CD
 */
export function setVehicleIsConsideredByPlayer(vehicle: number, toggle: number): void;

/**
 * Enables spawning random trains on the preset tracks. 
 * 
 * Requires [`SWITCH_TRAIN_TRACK`](#_0xFD813BB7DB977F20) and [`SET_TRAIN_TRACK_SPAWN_FREQUENCY`](#_0x21973BBF8D17EDFA) to be set.
 * @param toggle - 
 * @returns void
 * @remarks Hash: 0x80D9F74197EA47D9
 */
export function setRandomTrains(toggle: number): void;

/**
 * ## Parameters
 * *
 * @param vehicle - 
 * @returns number
 * @remarks Hash: 0xA7C4F2C6E744A550
 */
export function getVehicleMaxNumberOfPassengers(vehicle: number): number;

/**
 * ```
 * For a full enum, see here : pastebin.com/i2GGAjY0
 * char buffer[128];
 * std::sprintf(buffer, "VEH_CLASS_%i", VEHICLE::GET_VEHICLE_CLASS_FROM_NAME (hash));
 * const char* className = HUD::_GET_LABEL_TEXT(buffer);
 * ```
 * @param modelHash - 
 * @returns number
 * @remarks Hash: 0xDEDF1C8BD47C2200
 */
export function getVehicleClassFromName(modelHash: number): number;

/**
 * ## Parameters
 * *
 * @param vehicle - 
 * @returns number
 * @remarks Hash: 0x1DA0DA9CB3F0C8BF
 */
export function GetIsWheelsLoweredStateActive(vehicle: number): number;

/**
 * ## Parameters
 * *
 * @param vehicle - 
 * @returns void
 * @remarks Hash: 0xC53EB42A499A7E90
 */
export function removeVehicleUpsidedownCheck(vehicle: number): void;

/**
 * ## Parameters
 * *
 * @param vehicle - 
 * @returns number
 * @remarks Hash: 0x0EE21293DAD47C95
 */
export function getVehicleWindowTint(vehicle: number): number;

/**
 * Enables or disables the ability for a helicopter's tail boom to break off.
 * 
 * ```
 * NativeDB Introduced: v323
 * ```
 * @param heli - 
 * @param toggle - 
 * @returns void
 * @remarks Hash: 0x3EC8BF18AA453FE9
 */
export function setHeliTailBoomCanBreakOff(heli: number, toggle: number): void;

/**
 * ## Parameters
 * *
 * @param vehicle - 
 * @param toggle - 
 * @returns void
 * @remarks Hash: 0x8ABA6AF54B942B95
 */
export function setVehicleUndriveable(vehicle: number, toggle: number): void;

/**
 * Despite its name, this works on Helicopters and Planes.
 * 
 * Sets the speed of the helicopter blades to full speed.
 * 
 * This is equivalent to calling `SetHeliBladesSpeed(vehicleHandle, 1.0);`
 * @param vehicle - 
 * @returns void
 * @remarks Hash: 0xA178472EBB8AE60D
 */
export function setHeliBladesFullSpeed(vehicle: number): void;

/**
 * Transforms the `stormberg` to its "water vehicle" variant. If the vehicle is already in that state then the vehicle transformation audio will still play, but the vehicle won't change at all.
 * @param vehicle - 
 * @param instantly - 
 * @returns void
 * @remarks Hash: 0xBE4C854FFDB6EEBE
 */
export function transformToSubmarine(vehicle: number, instantly: number): void;

/**
 * ## Parameters
 * *
 * @param vehicle - 
 * @param state - 
 * @returns void
 * @remarks Hash: 0x0D5F65A8F4EBDAB5
 */
export function setCargobobPickupRopeType(vehicle: number, state: number): void;

/**
 * ```
 * Adds some kind of shadow to the vehicle.
 * -1 disables the effect.
 * DISABLE_*
 * ```
 * @param vehicle - 
 * @param p1 - 
 * @param p2 - 
 * @returns void
 * @remarks Hash: 0xF0E4BA16D1DB546C
 */
export function SetVehicleShadowEffect(vehicle: number, p1: number, p2: number): void;

/**
 * Set a specific offset for helis chasing target in combat
 * 
 * ```
 * NativeDB Introduced: v1180
 * ```
 * @param vehicle - 
 * @param x - 
 * @param y - 
 * @param z - 
 * @returns void
 * @remarks Hash: 0x0A3F820A9A9A9AC5
 */
export function setHeliCombatOffset(vehicle: number, x: number, y: number, z: number): void;

/**
 * ## Parameters
 * *
 * @param vehicle - 
 * @param r - 
 * @param g - 
 * @param b - 
 * @returns void
 * @remarks Hash: 0xB64CF2CCA9D95F52
 */
export function getVehicleCustomPrimaryColour(vehicle: number, r: number, g: number, b: number): void;

/**
 * Use [_SET_VEHICLE_HEADLIGHTS_COLOUR](#_0xE41033B25D003A07) to set the headlights color for the vehicle.
 * 
 * You must enable xenon headlights for this native to work properly.
 * 
 * ```c
 * enum eHeadlightColors {
 *     Default = 255,
 *     White = 0,
 *     Blue = 1,
 *     ElectricBlue = 2,
 *     MintGreen = 3,
 *     LimeGreen = 4,
 *     Yellow = 5,
 *     GoldenShower = 6,
 *     Orange = 7,
 *     Red = 8,
 *     PonyPink = 9,
 *     HotPink = 10,
 *     Purple = 11,
 *     Blacklight = 12
 * }
 * ```
 * @param vehicle - 
 * @returns number
 * @remarks Hash: 0x3DFF319A831E0CDB
 */
export function GetVehicleXenonLightsColor(vehicle: number): number;

/**
 * ```
 * If set to true, vehicle will not take crash damage, but is still susceptible to damage from bullets and explosives  
 * ```
 * @param vehicle - 
 * @param toggle - 
 * @returns void
 * @remarks Hash: 0x3E8C8727991A8A0B
 */
export function setVehicleStrong(vehicle: number, toggle: number): void;

/**
 * Enables individual propeller on a propeller plane. This native is the inverse of [`DISABLE_INDIVIDUAL_PLANE_PROPELLER`](#_0x500873A45724C863).
 * 
 * ```
 * NativeDB Introduced: v3407
 * ```
 * @param plane - 
 * @param propeller - 
 * @returns void
 * @remarks Hash: 0xDC05D2777F855F44
 */
export function EnableIndividualPlanePropeller(plane: number, propeller: number): void;

/**
 * ```
 * in the decompiled scripts, seems to be always called on the vehicle right after being attached to a trailer.
 * ```
 * @param vehicle - 
 * @returns void
 * @remarks Hash: 0x95CF53B3D687F9FA
 */
export function setTrailerLegsRaised(vehicle: number): void;

/**
 * ## Parameters
 * *
 * @param vehicle - 
 * @param toggle - 
 * @returns void
 * @remarks Hash: 0x02398B627547189C
 */
export function setVehicleHasBeenDrivenFlag(vehicle: number, toggle: number): void;

/**
 * ## Parameters
 * *
 * @param vehicle - 
 * @param toggle - 
 * @returns void
 * @remarks Hash: 0x89F149B6131E57DA
 */
export function setVehicleGravity(vehicle: number, toggle: number): void;

/**
 * ```c
 * // eVehicleModType values modified to conform to script native reorganization (see 0x140D25327 in 1604).
 * enum eVehicleModType
 * {
 * 	VMT_SPOILER = 0,
 * 	VMT_BUMPER_F = 1,
 * 	VMT_BUMPER_R = 2,
 * 	VMT_SKIRT = 3,
 * 	VMT_EXHAUST = 4,
 * 	VMT_CHASSIS = 5,
 * 	VMT_GRILL = 6,
 * 	VMT_BONNET = 7,
 * 	VMT_WING_L = 8,
 * 	VMT_WING_R = 9,
 * 	VMT_ROOF = 10,
 * 	VMT_ENGINE = 11,
 * 	VMT_BRAKES = 12,
 * 	VMT_GEARBOX = 13,
 * 	VMT_HORN = 14,
 * 	VMT_SUSPENSION = 15,
 * 	VMT_ARMOUR = 16,
 * 	VMT_NITROUS = 17,
 * 	VMT_TURBO = 18,
 * 	VMT_SUBWOOFER = 19,
 * 	VMT_TYRE_SMOKE = 20,
 * 	VMT_HYDRAULICS = 21,
 * 	VMT_XENON_LIGHTS = 22,
 * 	VMT_WHEELS = 23,
 * 	VMT_WHEELS_REAR_OR_HYDRAULICS = 24,
 * 	VMT_PLTHOLDER = 25,
 * 	VMT_PLTVANITY = 26,
 * 	VMT_INTERIOR1 = 27,
 * 	VMT_INTERIOR2 = 28,
 * 	VMT_INTERIOR3 = 29,
 * 	VMT_INTERIOR4 = 30,
 * 	VMT_INTERIOR5 = 31,
 * 	VMT_SEATS = 32,
 * 	VMT_STEERING = 33,
 * 	VMT_KNOB = 34,
 * 	VMT_PLAQUE = 35,
 * 	VMT_ICE = 36,
 * 	VMT_TRUNK = 37,
 * 	VMT_HYDRO = 38,
 * 	VMT_ENGINEBAY1 = 39,
 * 	VMT_ENGINEBAY2 = 40,
 * 	VMT_ENGINEBAY3 = 41,
 * 	VMT_CHASSIS2 = 42,
 * 	VMT_CHASSIS3 = 43,
 * 	VMT_CHASSIS4 = 44,
 * 	VMT_CHASSIS5 = 45,
 * 	VMT_DOOR_L = 46,
 * 	VMT_DOOR_R = 47,
 * 	VMT_LIVERY_MOD = 48,
 * 	VMT_LIGHTBAR = 49,
 * };
 * ```
 * @param vehicle - 
 * @param modType - 
 * @param modIndex - 
 * @param customTires - 
 * @returns void
 * @remarks Hash: 0x6AF0636DDEDCB6DD
 */
export function setVehicleMod(vehicle: number, modType: number, modIndex: number, customTires: number): void;

/**
 * @returns void
 * @remarks Hash: 0x0F87E938BDF29D66
 */
export function stopAllGarageActivity(): void;

/**
 * ## Parameters
 * *
 * @param vehicle - 
 * @param p1 - 
 * @returns void
 * @remarks Hash: 0x786A4EB67B01BF0B
 */
export function explodeVehicleInCutscene(vehicle: number, p1: number): void;

/**
 * Apply damage to vehicle at a location. Location is relative to vehicle model (not world).
 * Radius of effect damage applied in a sphere at impact location
 * When `focusOnModel` set to `true`, the damage sphere will travel towards the vehicle from the given point, thus guaranteeing an impact
 * @param vehicle - 
 * @param xOffset - 
 * @param yOffset - 
 * @param zOffset - 
 * @param damage - 
 * @param radius - 
 * @param focusOnModel - 
 * @returns void
 * @remarks Hash: 0xA1DD317EA8FD4F29
 */
export function setVehicleDamage(vehicle: number, xOffset: number, yOffset: number, zOffset: number, damage: number, radius: number, focusOnModel: number): void;

/**
 * ```
 * To check if the model is an amphibious car, see gtaforums.com/topic/717612-v-scriptnative-documentation-and-research/page-33#entry1069317363 (for build 944 and above only!)  
 * ```
 * @param model - 
 * @returns number
 * @remarks Hash: 0x7F6DB52EEFC96DF8
 */
export function isThisModelACar(model: number): number;

/**
 * Sets the anchor state for a boat.
 * 
 * ```
 * NativeDB Introduced: v323
 * ```
 * @param boat - 
 * @param toggle - 
 * @returns void
 * @remarks Hash: 0x75DBEC174AEEAD10
 */
export function setBoatAnchor(boat: number, toggle: number): void;

/**
 * ```
 * Also includes some "turnOffBones" when vehicle mods are installed.
 * ```
 * @param vehicle - 
 * @returns number
 * @remarks Hash: 0x42A4BEB35D372407
 */
export function GetVehicleNumberOfBrokenOffBones(vehicle: number): number;

/**
 * ```
 * NativeDB Introduced: v1604
 * ```
 * @param toggle - 
 * @returns void
 * @remarks Hash: 0x211E95CE9903940C
 */
export function SetDisableVehicleUnk2(toggle: number): void;

/**
 * ## Parameters
 * *
 * @param vehicle - 
 * @param ped - 
 * @param toggle - 
 * @returns void
 * @remarks Hash: 0x2E0A74E1002380B1
 */
export function setVehicleTimedExplosion(vehicle: number, ped: number, toggle: number): void;

/**
 * Gets hash related to task happening with seat index
 * Native name: GET_I*
 * @param vehicle - 
 * @param seatIndex - 
 * @returns number
 * @remarks Hash: 0xA01BC64DD4BFBBAC
 */
export function 0xa01bc64dd4bfbbac(vehicle: number, seatIndex: number): number;

/**
 * ## Parameters
 * *
 * @param p0 - 
 * @param modType - 
 * @param p2 - 
 * @returns void
 * @remarks Hash: 0x758F49C24925568A
 */
export function preloadVehicleMod(p0: any, modType: number, p2: any): void;

/**
 * ```
 * NativeDB Introduced: v323
 * ```
 * @param vehicle - 
 * @param propeller - 
 * @returns void
 * @remarks Hash: 0x500873A45724C863
 */
export function disableIndividualPlanePropeller(vehicle: number, propeller: number): void;

/**
 * ## Parameters
 * *
 * @param value - 
 * @returns void
 * @remarks Hash: 0xCAA15F13EBD417FF
 */
export function setNumberOfParkedVehicles(value: number): void;

/**
 * ## Parameters
 * *
 * @param p0 - 
 * @param p1 - 
 * @param p2 - 
 * @returns void
 * @remarks Hash: 0xC0ED6438E6D39BA8
 */
export function 0xc0ed6438e6d39ba8(p0: any, p1: any, p2: any): void;

/**
 * Checks whether the specified boat vehicle is capsized, meaning it has overturned or is upside down in the water.
 * @param vehicle - 
 * @returns number
 * @remarks Hash: 0xBA91D045575699AD
 */
export function getIsBoatCapsized(vehicle: number): number;

/**
 * ## Parameters
 * *
 * @param vehicle - 
 * @param color - 
 * @returns void
 * @remarks Hash: 0xB7635E80A5C31BFF
 */
export function GetVehicleDashboardColor(vehicle: number, color: number): void;

/**
 * ## Parameters
 * *
 * @param id - 
 * @param time - 
 * @returns { x: number, y: number, z: number }
 * @remarks Hash: 0x92523B76657A517D
 */
export function getPositionOfVehicleRecordingIdAtTime(id: number, time: number): { x: number, y: number, z: number };

/**
 * ## Parameters
 * *
 * @param vehicle - 
 * @param toggle - 
 * @returns void
 * @remarks Hash: 0x428BACCDF5E26EAD
 */
export function setVehicleCanSaveInGarage(vehicle: number, toggle: number): void;

/**
 * ## Parameters
 * *
 * @param vehicleClass - 
 * @returns number
 * @remarks Hash: 0x2F83E7E45D9EA7AE
 */
export function getVehicleClassMaxAcceleration(vehicleClass: number): number;

/**
 * Returns the display name/text label (`gameName` in `vehicles.meta`) for the specified vehicle model.
 * @param modelHash - 
 * @returns string
 * @remarks Hash: 0xB215AAC32D25D019
 */
export function getDisplayNameFromVehicleModel(modelHash: number): string;

/**
 * ```
 * makes the train all jumbled up and derailed as it moves on the tracks (though that wont stop it from its normal operations)  
 * ```
 * @param train - 
 * @param toggle - 
 * @returns void
 * @remarks Hash: 0x317B11A312DF5534
 */
export function setRenderTrainAsDerailed(train: number, toggle: number): void;

/**
 * Refer to [GET_VEHICLE_WHEEL_TYPE](#_0xB3ED1BFB4BE636DC) for wheel types.
 * @param vehicle - 
 * @param wheelType - 
 * @returns void
 * @remarks Hash: 0x487EB21CC7295BA1
 */
export function setVehicleWheelType(vehicle: number, wheelType: number): void;

/**
 * ```
 * NativeDB Introduced: v1365
 * ```
 * @param vehicle - 
 * @param toggle - 
 * @returns void
 * @remarks Hash: 0x203B527D1B77904C
 */
export function SetVehicleDoorsLockedForUnk(vehicle: number, toggle: number): void;

/**
 * ## Parameters
 * *
 * @param vehicle - 
 * @returns number
 * @remarks Hash: 0xE7CF3C4F9F489F0C
 */
export function isVehicleAttachedToTrailer(vehicle: number): number;

/**
 * ## Parameters
 * *
 * @param vehicle - 
 * @param toggle - 
 * @returns void
 * @remarks Hash: 0x97CE68CB032583F0
 */
export function setForceHdVehicle(vehicle: number, toggle: number): void;

/**
 * ```
 * Locks the vehicle's steering to the desired angle, explained below.  
 * Requires to be called onTick. Steering is unlocked the moment the function stops being called on the vehicle.  
 * Steer bias:  
 * -1.0 = full right  
 * 0.0 = centered steering  
 * 1.0 = full left  
 * ```
 * @param vehicle - 
 * @param value - 
 * @returns void
 * @remarks Hash: 0x42A8EC77D5150CBE
 */
export function setVehicleSteerBias(vehicle: number, value: number): void;

/**
 * ## Parameters
 * *
 * @param vehicle - 
 * @param percentage - 
 * @returns void
 * @remarks Hash: 0xFEB2DDED3509562E
 */
export function SetVehicleRocketBoostPercentage(vehicle: number, percentage: number): void;

/**
 * ```
 * SET_VEHICLE_AL*
 * ```
 * @param vehicle - 
 * @param canBeLockedOn - 
 * @param unk - 
 * @returns void
 * @remarks Hash: 0x1DDA078D12879EEE
 */
export function SetVehicleCanBeLockedOn(vehicle: number, canBeLockedOn: number, unk: number): void;

/**
 * ```
 * Inverts vehicle's controls. So INPUT_VEH_ACCELERATE will be INPUT_VEH_BRAKE and vise versa (same for A/D controls)
 * Doesn't work for planes/helis.
 * ```
 * @param vehicle - 
 * @param state - 
 * @returns void
 * @remarks Hash: 0x5B91B229243351A8
 */
export function SetVehicleControlsInverted(vehicle: number, state: number): void;

/**
 * ```
 * Only used for wheels(ModType = 23/24) Returns true if the wheels are custom wheels
 * ```
 * @param vehicle - 
 * @param modType - 
 * @returns number
 * @remarks Hash: 0xB3924ECD70E095DC
 */
export function getVehicleModVariation(vehicle: number, modType: number): number;

/**
 * ## Parameters
 * *
 * @param vehicle - 
 * @returns void
 * @remarks Hash: 0x99AD4CCCB128CBC9
 */
export function addVehiclePhoneExplosiveDevice(vehicle: number): void;

/**
 * ```
 * 0.0 = Lowest 1.0 = Highest. This is best to be used if you wanna pick-up a car since un-realistically on GTA V forklifts can't pick up much of anything due to vehicle mass. If you put this under a car then set it above 0.0 to a 'lifted-value' it will raise the car with no issue lol
 * ```
 * @param vehicle - 
 * @param height - 
 * @returns void
 * @remarks Hash: 0x37EBBF3117BD6A25
 */
export function setForkliftForkHeight(vehicle: number, height: number): void;

/**
 * ## Parameters
 * *
 * @param model - 
 * @returns number
 * @remarks Hash: 0x633F6F44A537EBB6
 */
export function IsThisModelAnAmphibiousCar(model: number): number;

/**
 * ## Parameters
 * *
 * @param vehicle - 
 * @returns number
 * @remarks Hash: 0xAE3FEE8709B39DCB
 */
export function 0xae3fee8709b39dcb(vehicle: number): number;

/**
 * ```
 * <1.0 - Decreased torque
 * =1.0 - Default torque
 * >1.0 - Increased torque
 * Negative values will cause the vehicle to go backwards instead of forwards while accelerating.
 * value - is between 0.2 and 1.8 in the decompiled scripts.
 * This needs to be called every frame to take effect.
 * ```
 * @param vehicle - 
 * @param value - 
 * @returns void
 * @remarks Hash: 0xB59E4BD37AE292DB
 */
export function setVehicleCheatPowerIncrease(vehicle: number, value: number): void;

/**
 * ```
 * Returns max speed (without mods) of the specified vehicle model in m/s.
 * ```
 * @param modelHash - 
 * @returns number
 * @remarks Hash: 0xF417C2502FFFED43
 */
export function getVehicleModelEstimatedMaxSpeed(modelHash: number): number;

/**
 * ## Parameters
 * *
 * @param plane - 
 * @returns number
 * @remarks Hash: 0x5991A01434CE9677
 */
export function ArePlaneWingsIntact(plane: number): number;

/**
 * ```
 * SET_TIME_POSITION_IN_RECORDING can be emulated by: desired_time - GET_TIME_POSITION_IN_RECORDING(vehicle)
 * ```
 * @param vehicle - 
 * @param time - 
 * @returns void
 * @remarks Hash: 0x9438F7AD68771A20
 */
export function skipTimeInPlaybackRecordedVehicle(vehicle: number, time: number): void;

/**
 * ```
 * what does this do?  
 * ```
 * @param vehicle - 
 * @returns void
 * @remarks Hash: 0xCFD778E7904C255E
 */
export function 0xcfd778e7904c255e(vehicle: number): void;

/**
 * ## Parameters
 * *
 * @param vehicle - 
 * @param p1 - 
 * @returns void
 * @remarks Hash: 0x9BECD4B9FEF3F8A6
 */
export function 0x9becd4b9fef3f8a6(vehicle: number, p1: number): void;

/**
 * ```
 * Tested on the player's current vehicle. Unless you kill the driver, the vehicle doesn't loose control, however, if enabled, explodeOnImpact is still active. The moment you crash, boom.  
 * ```
 * @param vehicle - 
 * @param killDriver - 
 * @param explodeOnImpact - 
 * @returns void
 * @remarks Hash: 0xF19D095E42D430CC
 */
export function setVehicleOutOfControl(vehicle: number, killDriver: number, explodeOnImpact: number): void;

/**
 * Returns the plates a vehicle has.
 * 
 * ```c
 * enum eVehiclePlateType
 * {
 * 	VPT_FRONT_AND_BACK_PLATES = 0,
 * 	VPT_FRONT_PLATES = 1,
 * 	VPT_BACK_PLATES = 2,
 * 	VPT_NONE = 3,
 * };
 * ```
 * 
 * Motorcycles with no visible plates will sometimes return a 2 for unknown reasons.
 * @param vehicle - 
 * @returns number
 * @remarks Hash: 0x9CCC9525BF2408E0
 */
export function getVehiclePlateType(vehicle: number): number;

/**
 * ## Parameters
 * *
 * @param vehicle - 
 * @returns number
 * @remarks Hash: 0x1C8A4C2C19E68EEC
 */
export function isPlaybackGoingOnForVehicle(vehicle: number): number;

/**
 * Calling this native will keep a vehicle's engine running after exiting.
 * @param vehicle - 
 * @param toggle - 
 * @returns void
 * @remarks Hash: 0xB8FBC8B1330CA9B4
 */
export function setVehicleKeepEngineOnWhenAbandoned(vehicle: number, toggle: number): void;

/**
 * ## Parameters
 * *
 * @param vehicle - 
 * @returns void
 * @remarks Hash: 0xACFB2463CC22BED2
 */
export function setLastDrivenVehicle(vehicle: number): void;

/**
 * ## Parameters
 * *
 * @param vehicle - 
 * @returns number
 * @remarks Hash: 0x92922A607497B14D
 */
export function GetNumberOfVehicleDoors(vehicle: number): number;

/**
 * ## Parameters
 * *
 * @param modelHash - 
 * @returns number
 * @remarks Hash: 0xBFBA3BA79CFF7EBF
 */
export function getVehicleModelMaxBrakingMaxMods(modelHash: number): number;

/**
 * This native checks if the given vehicle is stopped at a red or amber traffic light junction, provided the driver's personality is set to not run amber lights.
 * @param vehicle - 
 * @returns number
 * @remarks Hash: 0x2959F696AE390A99
 */
export function isVehicleStoppedAtTrafficLights(vehicle: number): number;

/**
 * ## Parameters
 * *
 * @param vehicle - 
 * @param p1 - 
 * @returns void
 * @remarks Hash: 0xE6F13851780394DA
 */
export function setHeliTurbulenceScalar(vehicle: number, p1: number): void;

/**
 * Used to delete mission trains created with [`CREATE_MISSION_TRAIN`](#_0x63C6CCA8E68AE8C8).
 * @param train - 
 * @returns void
 * @remarks Hash: 0x5B76B14AE875C795
 */
export function deleteMissionTrain(train: number): void;

/**
 * ## Parameters
 * *
 * @param vehicle - 
 * @returns number
 * @remarks Hash: 0xE495D1EF4C91FD20
 */
export function getVehicleCauseOfDestruction(vehicle: number): number;

/**
 * ## Parameters
 * *
 * @param vehicle - 
 * @returns void
 * @remarks Hash: 0x55E1D2758F34E437
 */
export function clearVehicleCustomPrimaryColour(vehicle: number): void;

/**
 * ```
 * Stops CTaskBringVehicleToHalt
 * ```
 * @param vehicle - 
 * @returns void
 * @remarks Hash: 0x7C06330BFDDA182E
 */
export function StopBringVehicleToHalt(vehicle: number): void;

/**
 * ```
 * NativeDB Introduced: v2372
 * ```
 * @param vehicle - 
 * @param toggle - 
 * @returns void
 * @remarks Hash: 0x5AC79C98C5C17F05
 */
export function SetDriftTyresEnabled(vehicle: number, toggle: number): void;

/**
 * ```
 * NativeDB Introduced: v2372
 * ```
 * @param vehicle - 
 * @returns number
 * @remarks Hash: 0xAD464F2E18836BFC
 */
export function IsMissionTrain(vehicle: number): number;

/**
 * See eDoorId declared in [`SET_VEHICLE_DOOR_SHUT`](#_0x93D9BD300D7789E5)
 * @param vehicle - 
 * @param doorIndex - 
 * @param speed - 
 * @param angle - 
 * @returns void
 * @remarks Hash: 0xF2BFA0430F0A0FCB
 */
export function setVehicleDoorControl(vehicle: number, doorIndex: number, speed: number, angle: number): void;

/**
 * ```
 * NativeDB Introduced: v1290
 * ```
 * @param vehicle - 
 * @returns void
 * @remarks Hash: 0x107A473D7A6647A9
 */
export function 0x107a473d7a6647a9(vehicle: number): void;

/**
 * ## Parameters
 * *
 * @param vehicle - 
 * @returns number
 * @remarks Hash: 0x634148744F385576
 */
export function isHeliLandingAreaBlocked(vehicle: number): number;

/**
 * ## Parameters
 * *
 * @param p0 - 
 * @param p1 - 
 * @param p2 - 
 * @param p3 - 
 * @returns void
 * @remarks Hash: 0x796A877E459B99EA
 */
export function 0x796a877e459b99ea(p0: any, p1: number, p2: number, p3: number): void;

/**
 * ```
 * Allows creation of CEventShockingPlaneFlyby, CEventShockingHelicopterOverhead, and other(?) Shocking events
 * ```
 * @param vehicle - 
 * @param toggle - 
 * @returns void
 * @remarks Hash: 0x279D50DE5652D935
 */
export function setVehicleGeneratesEngineShockingEvents(vehicle: number, toggle: number): void;

/**
 * ```
 * NativeDB Introduced: v1604
 * ```
 * @param vehicle - 
 * @param toggle - 
 * @returns void
 * @remarks Hash: 0xAE71FB656C600587
 */
export function HideVehicleTombstone(vehicle: number, toggle: number): void;

/**
 * Returns true when the bomb bay doors of this plane are open. False if they're closed.
 * @param aircraft - 
 * @returns number
 * @remarks Hash: 0xD0917A423314BBA8
 */
export function AreBombBayDoorsOpen(aircraft: number): number;

/**
 * ## Parameters
 * *
 * @param p0 - 
 * @param p1 - 
 * @returns void
 * @remarks Hash: 0xBB2333BB87DDD87F
 */
export function 0xbb2333bb87ddd87f(p0: any, p1: any): void;

/**
 * ## Parameters
 * *
 * @param ped - 
 * @param vehicle - 
 * @param outIndex - 
 * @returns number
 * @remarks Hash: 0xB09D25E77C33EB3F
 */
export function IsPedExclusiveDriverOfVehicle(ped: number, vehicle: number, outIndex: number): number;

/**
 * Enables or disables the opening of a vehicle's rear doors in the event of a sticky bomb explosion. This native is effective for armored vehicles, such as the Stockade (Brinks vehicle), allowing the rear doors to be opened through controlled explosions, which might otherwise remain locked due to the vehicle nature.
 * @param vehicle - 
 * @param toggle - 
 * @returns void
 * @remarks Hash: 0x1B212B26DD3C04DF
 */
export function setOpenRearDoorsOnExplosion(vehicle: number, toggle: number): void;

/**
 * ## Parameters
 * *
 * @param vehicle - 
 * @param toggle - 
 * @returns void
 * @remarks Hash: 0xE16142B94664DEFD
 */
export function setPlaneResistToExplosion(vehicle: number, toggle: number): void;

/**
 * Retrieves a static value representing the maximum drive force of specific a vehicle, including any vehicle mods. This value does not change dynamically during gameplay. This value provides an approximation and should be considered alongside other performance metrics like top speed for a more comprehensive understanding of the vehicle's capabilities.
 * 
 * ```
 * NativeDB Introduced: v323
 * ```
 * @param vehicle - 
 * @returns number
 * @remarks Hash: 0x5DD35C8D074E57AE
 */
export function getVehicleAcceleration(vehicle: number): number;

/**
 * Returns a number of available rooftop liveries, or -1 if vehicle has no rooftop liveries available.
 * @param vehicle - 
 * @returns number
 * @remarks Hash: 0x5ECB40269053C0D4
 */
export function GetVehicleRoofLiveryCount(vehicle: number): number;

/**
 * ```
 * Seems related to vehicle health, like the one in IV.  
 * Max 1000, min 0.  
 * Vehicle does not necessarily explode or become undrivable at 0.  
 * ```
 * @param vehicle - 
 * @returns number
 * @remarks Hash: 0xF271147EB7B40F12
 */
export function getVehicleBodyHealth(vehicle: number): number;

/**
 * ```
 * Related to monster trucks in native scripts.
 * ```
 * 
 * ```
 * NativeDB Introduced: v1604
 * ```
 * @param vehicle - 
 * @param toggle - 
 * @returns void
 * @remarks Hash: 0x2970EAA18FD5E42F
 */
export function SetVehicleWheelsDealDamage(vehicle: number, toggle: number): void;

/**
 * ## Parameters
 * *
 * @param vehicle - 
 * @param p1 - 
 * @returns void
 * @remarks Hash: 0xEFC13B1CE30D755D
 */
export function SetVehicleRampLaunchModifier(vehicle: number, p1: number): void;

/**
 * ## Parameters
 * *
 * @param vehicle - 
 * @param weaponSlot - 
 * @returns void
 * @remarks Hash: 0x86B4B6212CB8B627
 */
export function SetVehicleWeaponsDisabled(vehicle: number, weaponSlot: number): void;

/**
 * ## Parameters
 * *
 * @param vehicleClass - 
 * @returns number
 * @remarks Hash: 0xDBC86D85C5059461
 */
export function getVehicleClassMaxTraction(vehicleClass: number): number;

/**
 * ## Parameters
 * *
 * @param model - 
 * @returns number
 * @remarks Hash: 0xDCE4334788AF94EA
 */
export function isThisModelAHeli(model: number): number;

/**
 * Similar to [`_SET_AIRCRAFT_BOMB_COUNT`](#_0xF4B2ED59DEB5D774), this sets the amount of countermeasures that are present on this vehicle.
 * 
 * Use [`_GET_AIRCRAFT_COUNTERMEASURE_COUNT`](#_0xF846AA63DF56B804) to get the current amount.
 * @param aircraft - 
 * @param count - 
 * @returns void
 * @remarks Hash: 0x9BDA23BF666F0855
 */
export function SetVehicleCountermeasureCount(aircraft: number, count: number): void;

/**
 * ## Parameters
 * *
 * @param p0 - 
 * @param p1 - 
 * @returns void
 * @remarks Hash: 0x6501129C9E0FFA05
 */
export function 0x6501129c9e0ffa05(p0: any, p1: any): void;

/**
 * ## Parameters
 * *
 * @param vehicle - 
 * @returns void
 * @remarks Hash: 0x8386BFB614D06749
 */
export function removeVehicleStuckCheck(vehicle: number): void;

/**
 * ```
 * 1000 is max health  
 * Begins leaking gas at around 650 health  
 * ```
 * @param vehicle - 
 * @param health - 
 * @returns void
 * @remarks Hash: 0x70DB57649FA8D0D8
 */
export function setVehiclePetrolTankHealth(vehicle: number, health: number): void;

/**
 * ## Parameters
 * *
 * @param vehicle - 
 * @param scalar - 
 * @returns void
 * @remarks Hash: 0x9007A2F21DC108D4
 */
export function setVehicleSteeringBiasScalar(vehicle: number, scalar: number): void;

/**
 * Only used with the "akula" in the decompiled native scripts.
 * 
 * ```
 * NativeDB Introduced: v1290
 * ```
 * @param vehicle - 
 * @returns number
 * @remarks Hash: 0xAEF12960FA943792
 */
export function AreHeliStubWingsDeployed(vehicle: number): number;

/**
 * ## Parameters
 * *
 * @param vehicle - 
 * @returns number
 * @remarks Hash: 0x1F25887F3C104278
 */
export function isVehicleHighDetail(vehicle: number): number;

/**
 * ```
 * Appears to return true if the vehicle has any damage, including cosmetically.
 * GET_*
 * ```
 * @param vehicle - 
 * @returns number
 * @remarks Hash: 0xBCDC5017D3CE1E9E
 */
export function IsVehicleDamaged(vehicle: number): number;

/**
 * ## Parameters
 * *
 * @param vehicle - 
 * @returns number
 * @remarks Hash: 0x9F243D3919F442FE
 */
export function isBigVehicle(vehicle: number): number;

/**
 * Seat indices range from -1 to [`GET_VEHICLE_MAX_NUMBER_OF_PASSENGERS`](#_0xA7C4F2C6E744A550) minus one.
 * 
 * ```c
 * // CTaskExitVehicleSeat::eSeatPosition - 1
 * enum eSeatPosition
 * {
 *     SF_FrontDriverSide = -1,
 *     SF_FrontPassengerSide = 0,
 *     SF_BackDriverSide = 1,
 *     SF_BackPassengerSide = 2,
 *     SF_AltFrontDriverSide = 3,
 *     SF_AltFrontPassengerSide = 4,
 *     SF_AltBackDriverSide = 5,
 *     SF_AltBackPassengerSide = 6,
 * };
 * ```
 * 
 * ```
 * NativeDB Added Parameter 3: BOOL isTaskRunning
 * 
 * isTaskRunning = on true the function returns already false while a task on the target seat is running (TASK_ENTER_VEHICLE/TASK_SHUFFLE_TO_NEXT_VEHICLE_SEAT) - on false only when a ped is finally sitting in the seat.
 * ```
 * @param vehicle - 
 * @param seatIndex - 
 * @returns number
 * @remarks Hash: 0x22AC59A870E6A669
 */
export function isVehicleSeatFree(vehicle: number, seatIndex: number): number;

/**
 * ```
 * NativeDB Introduced: v3258
 * ```
 * @param vehicleModel - 
 * @returns number
 * @remarks Hash: 0x1423725069EE1D14
 */
export function GetVehicleDrivetrainType(vehicleModel: number): number;

/**
 * Starts or stops the engine on the specified vehicle.
 * From what I've tested when I do this to a helicopter the propellers turn off after the engine has started.
 * @param vehicle - 
 * @param value - 
 * @param instantly - 
 * @param disableAutoStart - 
 * @returns void
 * @remarks Hash: 0x2497C4717C8B881E
 */
export function setVehicleEngineOn(vehicle: number, value: number, instantly: number, disableAutoStart: number): void;

/**
 * ```
 * NativeDB Introduced: v1604
 * ```
 * @param vehicle - 
 * @param p1 - 
 * @returns void
 * @remarks Hash: 0x6A973569BA094650
 */
export function 0x6a973569ba094650(vehicle: number, p1: any): void;

/**
 * ## Parameters
 * *
 * @param vehicle - 
 * @param active - 
 * @returns void
 * @remarks Hash: 0x0BFFB028B3DD0A97
 */
export function SetVehicleParachuteActive(vehicle: number, active: number): void;

/**
 * ```
 * Usually used alongside other vehicle door natives.
 * ```
 * @param vehicle - 
 * @param doorIndex - 
 * @param toggle - 
 * @returns void
 * @remarks Hash: 0x3B458DDB57038F08
 */
export function 0x3b458ddb57038f08(vehicle: number, doorIndex: number, toggle: number): void;

/**
 * ```
 * Reduces grip significantly so it's hard to go anywhere.  
 * ```
 * @param vehicle - 
 * @param toggle - 
 * @returns void
 * @remarks Hash: 0x222FF6A823D122E2
 */
export function setVehicleReduceGrip(vehicle: number, toggle: number): void;

/**
 * ## Parameters
 * *
 * @param vehicle - 
 * @param p1 - 
 * @returns void
 * @remarks Hash: 0xE5810AC70602F2F5
 */
export function 0xe5810ac70602f2f5(vehicle: number, p1: number): void;

/**
 * ```
 * parachuteModel = 230075693  
 * ```
 * @param vehicle - 
 * @param modelHash - 
 * @returns void
 * @remarks Hash: 0x4D610C6B56031351
 */
export function SetVehicleParachuteModel(vehicle: number, modelHash: number): void;

/**
 * ## Parameters
 * *
 * @param p0 - 
 * @param p1 - 
 * @returns void
 * @remarks Hash: 0xAB31EF4DE6800CE9
 */
export function 0xab31ef4de6800ce9(p0: any, p1: any): void;

/**
 * ```
 * NativeDB Introduced: v1868
 * ```
 * @param vehicle - 
 * @param wheelIndex - 
 * @returns number
 * @remarks Hash: 0x55EAB010FAEE9380
 */
export function GetTyreHealth(vehicle: number, wheelIndex: number): number;

/**
 * ```
 * Hardcoded to not work in multiplayer.  
 * ```
 * @param vehicle - 
 * @param state - 
 * @returns void
 * @remarks Hash: 0x52BBA29D5EC69356
 */
export function setCanResprayVehicle(vehicle: number, state: number): void;

/**
 * ```
 * Returns max number of passengers (including the driver) for the specified vehicle model.
 * ```
 * @param modelHash - 
 * @returns number
 * @remarks Hash: 0x2AD93716F184EDA4
 */
export function getVehicleModelNumberOfSeats(modelHash: number): number;

/**
 * ## Parameters
 * *
 * @param vehicle - 
 * @param toggle - 
 * @returns void
 * @remarks Hash: 0x5AFEEDD9BB2899D7
 */
export function setVehicleProvidesCover(vehicle: number, toggle: number): void;

/**
 * This native sets whether a specific vehicle influences the player's wanted level when it is involved in an incident that typically triggers a wanted response, such as being marked as a "victim" vehicle.
 * 
 * This is particularly useful when utilizing the wanted system from GTA, and you want to prevent a vehicle from affecting the wanted level when it is stolen. In the decompiled scripts this native is only used to disable the influence of the vehicle on the wanted level.
 * @param vehicle - 
 * @param influenceWantedLevel - 
 * @returns void
 * @remarks Hash: 0x0AD9E8F87FF7C16F
 */
export function setVehicleInfluencesWantedLevel(vehicle: number, influenceWantedLevel: number): void;

/**
 * Gets the amount of bombs that this vehicle has. As far as I know, this does _not_ impact vehicle weapons or the ammo of those weapons in any way, it is just a way to keep track of the amount of bombs in a specific plane. 
 * 
 * In decompiled scripts this is used to check if the vehicle has enough bombs before a bomb can be dropped (bombs are dropped by using [`_SHOOT_SINGLE_BULLET_BETWEEN_COORDS_WITH_EXTRA_PARAMS`](#_0xBFE5756E7407064A)). 
 * 
 * Use [`_SET_AIRCRAFT_BOMB_COUNT`](#_0xF4B2ED59DEB5D774) to set the amount of bombs on that vehicle.
 * @param aircraft - 
 * @returns number
 * @remarks Hash: 0xEA12BD130D7569A1
 */
export function GetVehicleBombCount(aircraft: number): number;

/**
 * ```
 * NativeDB Introduced: v1493
 * ```
 * @param vehicle - 
 * @param toggle - 
 * @param p2 - 
 * @returns void
 * @remarks Hash: 0x59C3757B3B7408E8
 */
export function 0x59c3757b3b7408e8(vehicle: number, toggle: number, p2: number): void;

/**
 * ## Parameters
 * *
 * @param vehicle - 
 * @param frontBumper - 
 * @returns number
 * @remarks Hash: 0x27B926779DEB502D
 */
export function isVehicleBumperBouncing(vehicle: number, frontBumper: number): number;

/**
 * Gets the color of the neon lights of the specified vehicle.  
 * 
 * See [`_SET_VEHICLE_NEON_LIGHTS_COLOUR`](#_0x8E0A582209A62695) for more information
 * @param vehicle - 
 * @param r - 
 * @param g - 
 * @param b - 
 * @returns void
 * @remarks Hash: 0x7619EEE8C886757F
 */
export function GetVehicleNeonLightsColour(vehicle: number, r: number, g: number, b: number): void;

/**
 * ## Parameters
 * *
 * @param vehicleGenerator - 
 * @returns number
 * @remarks Hash: 0xF6086BC836400876
 */
export function doesScriptVehicleGeneratorExist(vehicleGenerator: number): number;

/**
 * ## Parameters
 * *
 * @param vehicle - 
 * @returns void
 * @remarks Hash: 0x54833611C17ABDEA
 */
export function stopPlaybackRecordedVehicle(vehicle: number): void;

/**
 * ```
 * SET_VEHICLE_AL*
 * ```
 * @param vehicle - 
 * @param toggle - 
 * @param p2 - 
 * @returns void
 * @remarks Hash: 0x7D6F9A3EF26136A0
 */
export function 0x7d6f9a3ef26136a0(vehicle: number, toggle: number, p2: number): void;

/**
 * ```
 * Corrected p1. it's basically the 'carriage/trailer number'. So if the train has 3 trailers you'd call the native once with a var or 3 times with 1, 2, 3.  
 * ```
 * @param train - 
 * @param trailerNumber - 
 * @returns number
 * @remarks Hash: 0x08AAFD0814722BC3
 */
export function getTrainCarriage(train: number, trailerNumber: number): number;

/**
 * ## Parameters
 * *
 * @param vehicle - 
 * @param toggle - 
 * @returns void
 * @remarks Hash: 0x83F813570FF519DE
 */
export function DisableVehicleNeonLights(vehicle: number, toggle: number): void;

/**
 * ```
 * Returns true if the vehicle has the FLAG_JUMPING_CAR flag set.
 * ```
 * @param vehicle - 
 * @returns number
 * @remarks Hash: 0x9078C0C5EF8C19E9
 */
export function GetCanVehicleJump(vehicle: number): number;

/**
 * Checks if a boat can be anchored at its present position without possibly intersecting collision later.
 * 
 * ```
 * NativeDB Introduced: v323
 * ```
 * @param boat - 
 * @returns number
 * @remarks Hash: 0x26C10ECBDA5D043B
 */
export function canAnchorBoatHere(boat: number): number;

/**
 * ```
 * Example usage  
 * VEHICLE::GET_CLOSEST_VEHICLE(x, y, z, radius, hash, unknown leave at 70)   
 * x, y, z: Position to get closest vehicle to.  
 * radius: Max radius to get a vehicle.  
 * modelHash: Limit to vehicles with this model. 0 for any.  
 * flags: The bitwise flags altering the function's behaviour.  
 * Does not return police cars or helicopters.  
 * It seems to return police cars for me, does not seem to return helicopters, planes or boats for some reason  
 * Only returns non police cars and motorbikes with the flag set to 70 and modelHash to 0. ModelHash seems to always be 0 when not a modelHash in the scripts, as stated above.   
 * These flags were found in the b617d scripts: 0,2,4,6,7,23,127,260,2146,2175,12294,16384,16386,20503,32768,67590,67711,98309,100359.  
 * Converted to binary, each bit probably represents a flag as explained regarding another native here: gtaforums.com/topic/822314-guide-driving-styles  
 * Conversion of found flags to binary: pastebin.com/kghNFkRi  
 * At exactly 16384 which is 0100000000000000 in binary and 4000 in hexadecimal only planes are returned.   
 * It's probably more convenient to use worldGetAllVehicles(int *arr, int arrSize) and check the shortest distance yourself and sort if you want by checking the vehicle type with for example VEHICLE::IS_THIS_MODEL_A_BOAT
 * @param x - 
 * @param y - 
 * @param z - 
 * @param radius - 
 * @param modelHash - 
 * @param flags - 
 * @returns number
 * @remarks Hash: 0xF73EB622C4F1689B
 */
export function getClosestVehicle(x: number, y: number, z: number, radius: number, modelHash: number, flags: number): number;

/**
 * ```
 * NativeDB Introduced: v3407
 * ```
 * 
 * Prevents the plane from exploding when taking body damage if the inflictor is an AI-controlled vehicle. Only works for planes.
 * @param plane - 
 * @param disable - 
 * @returns void
 * @remarks Hash: 0xB0B7DF5CB876FF5E
 */
export function SetDisableExplodeFromBodyDamageReceivedByAiVehicle(plane: number, disable: number): void;

/**
 * ## Parameters
 * *
 * @param vehicle - 
 * @returns number
 * @remarks Hash: 0x2C8CBFE1EA5FC631
 */
export function GetVehicleNumberOfBrokenBones(vehicle: number): number;

/**
 * ## Parameters
 * *
 * @param vehicle - 
 * @param paintType - 
 * @param color - 
 * @returns void
 * @remarks Hash: 0x81592BE4E3878728
 */
export function getVehicleModColor2(vehicle: number, paintType: number, color: number): void;

/**
 * This only works for planes.
 * 
 * Prevents a vehicle from exploding upon sustaining body damage from physical collisions. 
 * 
 * For helicopters, you might want to check [`SET_DISABLE_HELI_EXPLODE_FROM_BODY_DAMAGE`](#_0xEDBC8405B3895CC9) instead.
 * 
 * ```
 * NativeDB Introduced: v1290
 * ```
 * @param vehicle - 
 * @returns void
 * @remarks Hash: 0x26E13D440E7F6064
 */
export function setDisableExplodeFromBodyDamageOnCollision(vehicle: number): void;

/**
 * Only ever used once in decompiled scripts:
 * @param vehicle - 
 * @returns number
 * @remarks Hash: 0xEC69ADF931AAE0C3
 */
export function IsVehicleEngineOnFire(vehicle: number): number;

/**
 * ## Parameters
 * *
 * @param vehicle - 
 * @param color - 
 * @returns void
 * @remarks Hash: 0xF40DD601A65F7F19
 */
export function SetVehicleInteriorColor(vehicle: number, color: number): void;

/**
 * ## Parameters
 * *
 * @param distance - 
 * @returns void
 * @remarks Hash: 0xBC3CCA5844452B06
 */
export function setLightsCutoffDistanceTweak(distance: number): void;

/**
 * Identical to SET_PLAYBACK_TO_USE_AI_TRY_TO_REVERT_BACK_LATER with 0 as arguments for p1 and p3.
 * @param vehicle - 
 * @param drivingStyle - 
 * @returns void
 * @remarks Hash: 0xA549C3B37EA28131
 */
export function setPlaybackToUseAi(vehicle: number, drivingStyle: number): void;

/**
 * ## Parameters
 * *
 * @param vehicle - 
 * @param position - 
 * @param p2 - 
 * @returns void
 * @remarks Hash: 0x56B94C6D7127DFBA
 */
export function setVehicleTankTurretPosition(vehicle: number, position: number, p2: number): void;

/**
 * ## Parameters
 * *
 * @param vehicle - 
 * @param extraId - 
 * @returns number
 * @remarks Hash: 0xD2E6822DBFD6C8BD
 */
export function isVehicleExtraTurnedOn(vehicle: number, extraId: number): number;

/**
 * ## Parameters
 * *
 * @param vehicle - 
 * @param pearlescentColor - 
 * @param wheelColor - 
 * @returns void
 * @remarks Hash: 0x3BC4245933A166F7
 */
export function getVehicleExtraColours(vehicle: number, pearlescentColor: number, wheelColor: number): void;

/**
 * See [REQUEST_VEHICLE_RECORDING](#_0xAF514CABE74CBF15)
 * @param recording - 
 * @param script - 
 * @returns void
 * @remarks Hash: 0xF1160ACCF98A3FC8
 */
export function removeVehicleRecording(recording: number, script: string): void;

/**
 * ## Parameters
 * *
 * @param vehicle - 
 * @returns number
 * @remarks Hash: 0x4AF9BD80EEBEB453
 */
export function isVehicleStolen(vehicle: number): number;

/**
 * ## Parameters
 * *
 * @param p0 - 
 * @returns void
 * @remarks Hash: 0x65B080555EA48149
 */
export function 0x65b080555ea48149(p0: any): void;

/**
 * Detaches the specified entity currently being carried by a Cargobob.
 * @param vehicle - 
 * @param entity - 
 * @returns any
 * @remarks Hash: 0xAF03011701811146
 */
export function detachEntityFromCargobob(vehicle: number, entity: number): any;

/**
 * ## Parameters
 * *
 * @param p0 - 
 * @param p1 - 
 * @returns void
 * @remarks Hash: 0xED5EDE9E676643C9
 */
export function 0xed5ede9e676643c9(p0: any, p1: any): void;

/**
 * Sets the arm position of a bulldozer. Position must be a value between 0.0 and 1.0. Ignored when `p2` is set to false, instead incrementing arm position by 0.1 (or 10%).
 * @param vehicle - 
 * @param position - 
 * @param p2 - 
 * @returns void
 * @remarks Hash: 0xF8EBCCC96ADB9FB7
 */
export function setVehicleBulldozerArmPosition(vehicle: number, position: number, p2: number): void;

/**
 * ## Parameters
 * *
 * @param vehicle - 
 * @returns void
 * @remarks Hash: 0x87E7F24270732CB1
 */
export function openBombBayDoors(vehicle: number): void;

/**
 * ## Parameters
 * *
 * @param cargobob - 
 * @param p1 - 
 * @returns void
 * @remarks Hash: 0xED8286F71A819BAA
 */
export function setCargobobPickupMagnetPullStrength(cargobob: number, p1: number): void;

/**
 * ## Parameters
 * *
 * @param p0 - 
 * @returns void
 * @remarks Hash: 0x41290B40FA63E6DA
 */
export function 0x41290b40fa63e6da(p0: any): void;

/**
 * ```
 * Returns `nMonetaryValue` from handling.meta for specific model, which is the vehicle's monetary value.
 * ```
 * @returns number
 * @remarks Hash: 0x5873C14A52D74236
 */
export function getVehicleModelValue(): number;

/**
 * ```
 * NativeDB Introduced: v2372
 * ```
 * @param vehicle - 
 * @param wheelId - 
 * @returns number
 * @remarks Hash: 0x0BB5CBDDD0F25AE3
 */
export function GetHydraulicWheelValue(vehicle: number, wheelId: number): number;

/**
 * See eWindowId declared in [`IS_VEHICLE_WINDOW_INTACT`](#_0x46E571A0E20D01F1).
 * @param vehicle - 
 * @param windowIndex - 
 * @returns void
 * @remarks Hash: 0xA711568EEDB43069
 */
export function removeVehicleWindow(vehicle: number, windowIndex: number): void;

/**
 * ```
 * NativeDB Introduced: v2060
 * ```
 * @param vehicle - 
 * @param wheelIndex - 
 * @returns number
 * @remarks Hash: 0x6E387895952F4F71
 */
export function GetTyreWearMultiplier(vehicle: number, wheelIndex: number): number;

/**
 * ## Parameters
 * *
 * @param vehicle - 
 * @returns number
 * @remarks Hash: 0x25ECB9F8017D98E0
 */
export function doesVehicleHaveWeapons(vehicle: number): number;

/**
 * ## Parameters
 * *
 * @param vehicle - 
 * @param entity - 
 * @returns number
 * @remarks Hash: 0x8F5EBAB1F260CFCE
 */
export function getVehicleLockOnTarget(vehicle: number, entity: number): number;

/**
 * ## Parameters
 * *
 * @param p0 - 
 * @returns void
 * @remarks Hash: 0xA247F9EF01D8082E
 */
export function 0xa247f9ef01d8082e(p0: any): void;

/**
 * ## Parameters
 * *
 * @param vehicle - 
 * @returns number
 * @remarks Hash: 0x6636C535F6CC2725
 */
export function getBoatBoomPositionRatio(vehicle: number): number;

/**
 * This native it's a debug native. Won't do anything.
 * @param vehicle - 
 * @returns void
 * @remarks Hash: 0xB264C4D2F2B0A78B
 */
export function allowAmbientVehiclesToAvoidAdverseConditions(vehicle: number): void;

/**
 * ## Parameters
 * *
 * @param p0 - 
 * @returns void
 * @remarks Hash: 0xD3301660A57C9272
 */
export function 0xd3301660a57c9272(p0: any): void;

/**
 * ```
 * Sets some health value. Looks like it's used for helis.
 * ```
 * @param vehicle - 
 * @param health - 
 * @returns void
 * @remarks Hash: 0x5EE5632F47AE9695
 */
export function 0x5ee5632f47ae9695(vehicle: number, health: number): void;

/**
 * Returns whether the vehicle's lights and sirens are on.
 * @param vehicle - 
 * @returns number
 * @remarks Hash: 0x4C9BF537BE2634B2
 */
export function isVehicleSirenOn(vehicle: number): number;

/**
 * ```c
 * enum eVehicleWheels
 * {
 * 	WHEEL_LF = 0, // Vehicle Left front
 * 	WHEEL_RF = 1, // Vehicle Right front
 * 	WHEEL_LM = 2, // Vehicle Left middle
 * 	WHEEL_RM = 3, // Vehicle Right middle
 * 	WHEEL_LR = 4, // Vehicle Left rear
 * 	WHEEL_RR = 5, // Vehicle Right rear
 * 	WHEEL_BF = 6, // Bike front
 * 	WHEEL_BR = 7, // Bike rear
 * 	MAX_WHEELS = 8
 * };
 * ```
 * @param vehicle - 
 * @param wheelID - 
 * @param isBurstToRim - 
 * @returns number
 * @remarks Hash: 0xBA291848A0815CA9
 */
export function isVehicleTyreBurst(vehicle: number, wheelID: number, isBurstToRim: number): number;

/**
 * ## Parameters
 * *
 * @param x1 - 
 * @param y1 - 
 * @param z1 - 
 * @param x2 - 
 * @param y2 - 
 * @param z2 - 
 * @param unk - 
 * @returns void
 * @remarks Hash: 0x46A1E1A299EC4BBA
 */
export function removeVehiclesFromGeneratorsInArea(x1: number, y1: number, z1: number, x2: number, y2: number, z2: number, unk: any): void;

/**
 * ## Parameters
 * *
 * @param vehicle - 
 * @param toggle - 
 * @returns void
 * @remarks Hash: 0xBC2042F090AF6AD3
 */
export function setVehicleInteriorlight(vehicle: number, toggle: number): void;

/**
 * Specifies an area of interest where cargens will focus on spawning vehicles
 * 
 * You can clear the area of interest with [`CLEAR_VEHICLE_GENERATOR_AREA_OF_INTEREST`](#_0x0A436B8643716D14)
 * @param x - 
 * @param y - 
 * @param z - 
 * @param radius - 
 * @returns void
 * @remarks Hash: 0x9A75585FB2E54FAD
 */
export function setVehicleGeneratorAreaOfInterest(x: number, y: number, z: number, radius: number): void;

/**
 * ## Parameters
 * *
 * @param vehicle - 
 * @returns number
 * @remarks Hash: 0x25BC98A59C2EA962
 */
export function getVehicleDoorLockStatus(vehicle: number): number;

/**
 * ```
 * "To burst tyres VEHICLE::SET_VEHICLE_TYRE_BURST(vehicle, 0, true, 1000.0)  
 * to burst all tyres type it 8 times where p1 = 0 to 7.  
 * p3 seems to be how much damage it has taken. 0 doesn't deflate them, 1000 completely deflates them.  
 * '0 = wheel_lf / bike, plane or jet front  
 * '1 = wheel_rf  
 * '2 = wheel_lm / in 6 wheels trailer, plane or jet is first one on left  
 * '3 = wheel_rm / in 6 wheels trailer, plane or jet is first one on right  
 * '4 = wheel_lr / bike rear / in 6 wheels trailer, plane or jet is last one on left  
 * '5 = wheel_rr / in 6 wheels trailer, plane or jet is last one on right  
 * '45 = 6 wheels trailer mid wheel left  
 * '47 = 6 wheels trailer mid wheel right  
 * ```
 * @param vehicle - 
 * @param index - 
 * @param onRim - 
 * @param p3 - 
 * @returns void
 * @remarks Hash: 0xEC6A202EE4960385
 */
export function setVehicleTyreBurst(vehicle: number, index: number, onRim: number, p3: number): void;

/**
 * Detaches the vehicle's windscreen.
 * @param vehicle - 
 * @returns void
 * @remarks Hash: 0x6D645D59FB5F5AD3
 */
export function popOutVehicleWindscreen(vehicle: number): void;

/**
 * ```
 * NativeDB Introduced: 3095
 * ```
 * 
 * Activates or deactivates the nitrous system in the specified vehicle, based on the boolean value provided.
 * You can clear the nitrous with [`CLEAR_NITROUS`](#_0xC889AE921400E1ED), if you want to have more control on the nitrous and those settings, use [`SET_OVERRIDE_NITROUS_LEVEL`](#_0xC8E9B6B71B8E660D)
 * @param vehicle - 
 * @param isActive - 
 * @returns void
 * @remarks Hash: 0x465EEA70AF251045
 */
export function setNitrousIsActive(vehicle: number, isActive: number): void;

/**
 * ```
 * Gets a random vehicle in a sphere at the specified position, of the specified radius.  
 * x: The X-component of the position of the sphere.  
 * y: The Y-component of the position of the sphere.  
 * z: The Z-component of the position of the sphere.  
 * radius: The radius of the sphere. Max is 9999.9004.  
 * modelHash: The vehicle model to limit the selection to. Pass 0 for any model.  
 * flags: The bitwise flags that modifies the behaviour of this function.  
 * ```
 * @param x - 
 * @param y - 
 * @param z - 
 * @param radius - 
 * @param modelHash - 
 * @param flags - 
 * @returns number
 * @remarks Hash: 0x386F6CE5BAF6091C
 */
export function getRandomVehicleInSphere(x: number, y: number, z: number, radius: number, modelHash: number, flags: number): number;

/**
 * ## Parameters
 * *
 * @param vehicle - 
 * @param toggle - 
 * @returns void
 * @remarks Hash: 0xA2F80B8D040727CC
 */
export function setVehicleDoorsLockedForAllPlayers(vehicle: number, toggle: number): void;

/**
 * ## Parameters
 * *
 * @param vehicle - 
 * @param toggle - 
 * @returns void
 * @remarks Hash: 0x99C82F8A139F3E4E
 */
export function setVehicleKersAllowed(vehicle: number, toggle: number): void;

/**
 * ```
 * NativeDB Introduced: v1290
 * ```
 * @param vehicle - 
 * @param toggle - 
 * @returns void
 * @remarks Hash: 0xAA653AE61924B0A0
 */
export function 0xaa653ae61924b0a0(vehicle: number, toggle: number): void;

/**
 * Allows locking the hover/non-hover mode of a vehicle, such as the flying mode of the `Deluxo`. In the decompiled scripts, this native is used on `oppressor2` but couldn't get it to work on it.
 * @param vehicle - 
 * @param toggle - 
 * @returns void
 * @remarks Hash: 0xF1211889DF15A763
 */
export function setSpecialFlightModeAllowed(vehicle: number, toggle: number): void;

/**
 * See eDoorId declared in [`SET_VEHICLE_DOOR_SHUT`](#_0x93D9BD300D7789E5)
 * @param vehicle - 
 * @param doorIndex - 
 * @returns number
 * @remarks Hash: 0x645F4B6E8499F632
 */
export function GetIsDoorValid(vehicle: number, doorIndex: number): number;

/**
 * ## Parameters
 * *
 * @param vehicle - 
 * @param toggle - 
 * @returns void
 * @remarks Hash: 0xE4E2FD323574965C
 */
export function setVehicleBrake(vehicle: number, toggle: number): void;

/**
 * ## Parameters
 * *
 * @param vehicle - 
 * @param toggle - 
 * @returns void
 * @remarks Hash: 0x300504B23BD3B711
 */
export function setVehicleCanBeUsedByFleeingPeds(vehicle: number, toggle: number): void;

/**
 * ## Parameters
 * *
 * @param plane - 
 * @returns number
 * @remarks Hash: 0x4198AB0022B15F87
 */
export function isPlaneLandingGearIntact(plane: number): number;

/**
 * Enable/Disables global slipstream physics
 * @param toggle - 
 * @returns void
 * @remarks Hash: 0xE6C0C80B8C867537
 */
export function setEnableVehicleSlipstreaming(toggle: number): void;

/**
 * Retrieves the manufacturer's name for a specified vehicle.
 * 
 * ```
 * NativeDB Introduced: v1868
 * ```
 * @param modelHash - 
 * @returns string
 * @remarks Hash: 0xF7AF4F159FF99F97
 */
export function getMakeNameFromVehicleModel(modelHash: number): string;

/**
 * ## Parameters
 * *
 * @param model - 
 * @returns number
 * @remarks Hash: 0xB50C0B0CEDC6CE84
 */
export function isThisModelABike(model: number): number;

/**
 * Gets the ped in the specified seat of the passed vehicle.
 * 
 * If there is no ped in the seat, and the game considers the vehicle as ambient population, this will create a random occupant ped in the seat, which may be cleaned up by the game fairly soon if not marked as script-owned mission entity.
 * @param vehicle - 
 * @param seatIndex - 
 * @returns number
 * @remarks Hash: 0xBB40DD2270B65366
 */
export function getPedInVehicleSeat(vehicle: number, seatIndex: number): number;

/**
 * ## Parameters
 * *
 * @param x1 - 
 * @param y1 - 
 * @param z1 - 
 * @param x2 - 
 * @param y2 - 
 * @param z2 - 
 * @param p6 - 
 * @param p7 - 
 * @returns void
 * @remarks Hash: 0xC12321827687FE4D
 */
export function setAllVehicleGeneratorsActiveInArea(x1: number, y1: number, z1: number, x2: number, y2: number, z2: number, p6: number, p7: number): void;

/**
 * ## Parameters
 * *
 * @param p0 - 
 * @param p1 - 
 * @param p2 - 
 * @param p3 - 
 * @param p4 - 
 * @param p5 - 
 * @param p6 - 
 * @returns number
 * @remarks Hash: 0xB50807EABE20A8DC
 */
export function getRandomVehicleBackBumperInSphere(p0: number, p1: number, p2: number, p3: number, p4: number, p5: number, p6: number): number;

/**
 * Checks if a Submarine has any air leaks, when there is more than 4 the player will drown.
 * 
 * ```
 * NativeDB Introduced: v2189
 * ```
 * @param submarine - 
 * @returns number
 * @remarks Hash: 0x093D6DDCA5B8FBAE
 */
export function getSubmarineNumberOfAirLeaks(submarine: number): number;

/**
 * ```
 * in script hook .net   
 * Vehicle v = ...;  
 * Function.Call(Hash.TRACK_VEHICLE_VISIBILITY, v.Handle);  
 * ```
 * @param vehicle - 
 * @returns void
 * @remarks Hash: 0x64473AEFDCF47DCA
 */
export function trackVehicleVisibility(vehicle: number): void;

/**
 * ```
 * REQUEST_VEHICLE_ASSET(GET_HASH_KEY(cargobob3), 3);  
 * vehicle found that have asset's:  
 * cargobob3  
 * submersible  
 * blazer  
 * ```
 * @param vehicleHash - 
 * @param vehicleAsset - 
 * @returns void
 * @remarks Hash: 0x81A15811460FAB3A
 */
export function requestVehicleAsset(vehicleHash: number, vehicleAsset: number): void;

/**
 * ```c
 * enum eVehiclePlateIndicies {
 * 	SanAndreasCursive = 0,
 * 	SanAndreasBlack = 1,
 * 	SanAndreasBlue = 2,
 * 	SanAndreasPlain = 3,
 * 	SRExcept = 4,
 * 	NorthYankton = 5,
 * 	// All indicies below this require b3095
 * 	ECola = 6,
 * 	LasVenturas = 7,
 * 	LiberyCity = 8,
 * 	LSCarMeet = 9,
 * 	LSPanic = 10,
 * 	LSPounders = 11,
 * 	Sprunk = 12,
 * }
 * ```
 * @param vehicle - 
 * @returns number
 * @remarks Hash: 0xF11BC2DD9A3E7195
 */
export function getVehicleNumberPlateTextIndex(vehicle: number): number;

/**
 * ```
 * Actually number of color combinations  
 * ```
 * @param vehicle - 
 * @returns number
 * @remarks Hash: 0x3B963160CD65D41E
 */
export function getNumberOfVehicleColours(vehicle: number): number;

/**
 * ## Parameters
 * *
 * @param vehicle - 
 * @returns number
 * @remarks Hash: 0xADF7BE450512C12F
 */
export function detachVehicleFromAnyCargobob(vehicle: number): number;

/**
 * ```
 * Makes the vehicle accept no passengers.  
 * ```
 * @param veh - 
 * @param toggle - 
 * @returns void
 * @remarks Hash: 0x5D14D4154BFE7B2C
 */
export function setVehicleAllowNoPassengersLockon(veh: number, toggle: number): void;

/**
 * ## Parameters
 * *
 * @param vehicle - 
 * @returns number
 * @remarks Hash: 0x99093F60746708CA
 */
export function GetEntityAttachedToCargobob(vehicle: number): number;

/**
 * ```
 * Returns max traction of the specified vehicle model.
 * ```
 * @param modelHash - 
 * @returns number
 * @remarks Hash: 0x539DE94D44FDFD0D
 */
export function getVehicleModelMaxTraction(modelHash: number): number;

/**
 * Disables wings for `Deluxo` and `Oppressor MK II`. For the Deluxo, it retracts the wings immediately, preventing flight. For the Oppressor Mk II, the wings retract after landing and take-off is not possible, though it can still glide if launched into the air.
 * @param vehicle - 
 * @param toggle - 
 * @returns void
 * @remarks Hash: 0x2D55FE374D5FDB91
 */
export function setDisableHoverModeFlight(vehicle: number, toggle: number): void;

/**
 * R* used it to "remove" vehicle windows when "nightshark" had some mod, which adding some kind of armored windows. When enabled, you can't break vehicles glass. All your bullets wiil shoot through glass. You also will not able to break the glass with any other way (hitting and etc)
 * @param vehicle - 
 * @param toggle - 
 * @returns void
 * @remarks Hash: 0x1087BC8EC540DAEB
 */
export function SetDisableVehicleWindowCollisions(vehicle: number, toggle: number): void;

/**
 * See eDoorId declared in [`SET_VEHICLE_DOOR_SHUT`](#_0x93D9BD300D7789E5)
 * @param vehicle - 
 * @param doorIndex - 
 * @returns number
 * @remarks Hash: 0x218297BF0CFD853B
 */
export function getPedUsingVehicleDoor(vehicle: number, doorIndex: number): number;

/**
 * ## Parameters
 * *
 * @param vehicle - 
 * @returns number
 * @remarks Hash: 0xBC9CFF381338CB4F
 */
export function GetVehicleHasParachute(vehicle: number): number;

/**
 * For FiveM, use [`GET_GAME_POOL`](#_0x2B9D4F50).
 * @returns number
 * @remarks Hash: 0x9B8E1BF04B51F2E8
 */
export function getAllVehicles(): number;

/**
 * ```c
 * enum eWindowId {
 * 	VEH_EXT_WINDOW_LF = 0,
 * 	VEH_EXT_WINDOW_RF = 1,
 * 	VEH_EXT_WINDOW_LR = 2,
 * 	VEH_EXT_WINDOW_RR = 3,
 * 	VEH_EXT_WINDOW_LM = 4,
 * 	VEH_EXT_WINDOW_RM = 5,
 * 	VEH_EXT_WINDSCREEN = 6,
 * 	VEH_EXT_WINDSCREEN_R = 7,
 * }
 * ```
 * @param vehicle - 
 * @param windowIndex - 
 * @returns number
 * @remarks Hash: 0x46E571A0E20D01F1
 */
export function isVehicleWindowIntact(vehicle: number, windowIndex: number): number;

/**
 * Despite its name, this works on Helicopters and Planes.
 * 
 * Sets the speed of the helicopter blades in percentage of the full speed.
 * @param vehicle - 
 * @param speed - 
 * @returns void
 * @remarks Hash: 0xFD280B4D7F3ABC4D
 */
export function setHeliBladesSpeed(vehicle: number, speed: number): void;

/**
 * Often used in conjunction with: [SET_VEHICLE_REDUCE_GRIP](#_0x222FF6A823D122E2).
 * 
 * ```
 * NativeDB Introduced: v1604
 * ```
 * @param vehicle - 
 * @returns void
 * @remarks Hash: 0x6DEE944E1EE90CFB
 */
export function SetVehicleReduceTraction(vehicle: number): void;

/**
 * ```
 * Returns 1000.0 if the function is unable to get the address of the specified vehicle or if it's not a vehicle.  
 * Minimum: -4000  
 * Maximum: 1000  
 * -4000: Engine is destroyed  
 * 0 and below: Engine catches fire and health rapidly declines  
 * 300: Engine is smoking and losing functionality  
 * 1000: Engine is perfect  
 * ```
 * @param vehicle - 
 * @returns number
 * @remarks Hash: 0xC45D23BAF168AAB8
 */
export function getVehicleEngineHealth(vehicle: number): number;

/**
 * Returns index of the current vehicle's rooftop livery.
 * A getter for [_SET_VEHICLE_ROOF_LIVERY](#_0xA6D3A8750DC73270).
 * @param vehicle - 
 * @returns number
 * @remarks Hash: 0x60190048C0764A26
 */
export function GetVehicleRoofLivery(vehicle: number): number;

/**
 * ## Parameters
 * *
 * @param vehicle - 
 * @param state - 
 * @returns void
 * @remarks Hash: 0x4C7028F78FFD3681
 */
export function setVehicleCanBeVisiblyDamaged(vehicle: number, state: number): void;

/**
 * ## Parameters
 * *
 * @param vehicle - 
 * @param toggle - 
 * @returns void
 * @remarks Hash: 0x756AE6E962168A04
 */
export function SetVehicleRampUpwardsLaunchMotion(vehicle: number, toggle: number): void;

/**
 * ```
 * Gets the height of the vehicle's suspension.  
 * The higher the value the lower the suspension. Each 0.002 corresponds with one more level lowered.  
 * 0.000 is the stock suspension.  
 * 0.008 is Ultra Suspension.  
 * ```
 * @param vehicle - 
 * @returns number
 * @remarks Hash: 0x53952FD2BAA19F17
 */
export function GetVehicleSuspensionHeight(vehicle: number): number;

/**
 * ```
 * Activate siren on vehicle (Only works if the vehicle has a siren).  
 * ```
 * @param vehicle - 
 * @param toggle - 
 * @returns void
 * @remarks Hash: 0xF4924635A19EB37D
 */
export function setVehicleSiren(vehicle: number, toggle: number): void;

/**
 * Raises the roof on a convertible vehicle, utilizing any available animations for the action. This native is particularly useful for creating a realistic interaction with convertible vehicles by animating the process of raising the roof.
 * 
 * You can check if the vehicle has an convertible roof using [`IS_VEHICLE_A_CONVERTIBLE`](#_0x52F357A30698BCCE).
 * 
 * To lower the convertible roof, you can use [`LOWER_CONVERTIBLE_ROOF`](#_0xDED51F703D0FA83D).
 * 
 * ```
 * NativeDB Introduced: v323
 * ```
 * @param vehicle - 
 * @param instantlyRaise - 
 * @returns void
 * @remarks Hash: 0x8F5FB35D7E88FC70
 */
export function raiseConvertibleRoof(vehicle: number, instantlyRaise: number): void;

/**
 * ```
 * Inverse of 0x95CF53B3D687F9FA
 * ```
 * 
 * ```
 * NativeDB Added Parameter 1: Vehicle vehicle
 * ```
 * @returns void
 * @remarks Hash: 0x878C75C09FBDB942
 */
export function SetTrailerLegsLowered(): void;

/**
 * ## Parameters
 * *
 * @param vehicle - 
 * @param toggle - 
 * @returns void
 * @remarks Hash: 0x192547247864DFDD
 */
export function setVehicleCanLeakPetrol(vehicle: number, toggle: number): void;

/**
 * ## Parameters
 * *
 * @param vehicle - 
 * @param p1 - 
 * @returns void
 * @remarks Hash: 0x88BC673CA9E0AE99
 */
export function 0x88bc673ca9e0ae99(vehicle: number, p1: number): void;

/**
 * ## Parameters
 * *
 * @param p0 - 
 * @returns any
 * @remarks Hash: 0x6EAAEFC76ACC311F
 */
export function 0x6eaaefc76acc311f(p0: any): any;

/**
 * ```
 * Returns attached vehicle (Vehicle in parameter must be cargobob)  
 * ```
 * @param cargobob - 
 * @returns number
 * @remarks Hash: 0x873B82D42AC2B9E5
 */
export function getVehicleAttachedToCargobob(cargobob: number): number;

/**
 * ## Parameters
 * *
 * @param vehicle - 
 * @param color - 
 * @returns void
 * @remarks Hash: 0x7D1464D472D32136
 */
export function GetVehicleInteriorColor(vehicle: number, color: number): void;

/**
 * ## Parameters
 * *
 * @param vehicle - 
 * @param toggle - 
 * @returns void
 * @remarks Hash: 0x2B6747FAA9DB9D6B
 */
export function setVehicleDisableTowing(vehicle: number, toggle: number): void;

/**
 * ```
 * NativeDB Introduced: v1604
 * ```
 * @param toggle - 
 * @returns void
 * @remarks Hash: 0x8F0D5BA1C2CC91D7
 */
export function 0x8f0d5ba1c2cc91d7(toggle: number): void;

/**
 * Only used with the "akula" and "annihilator2" in the decompiled native scripts.
 * 
 * ```
 * NativeDB Introduced: v1290
 * ```
 * @param vehicle - 
 * @param deploy - 
 * @param p2 - 
 * @returns void
 * @remarks Hash: 0xB251E0B33E58B424
 */
export function SetDeployHeliStubWings(vehicle: number, deploy: number, p2: number): void;

/**
 * ## Parameters
 * *
 * @param vehicle - 
 * @param isStolen - 
 * @returns void
 * @remarks Hash: 0x67B2C79AA7FF5738
 */
export function setVehicleIsStolen(vehicle: number, isStolen: number): void;

/**
 * ```
 * REQUEST_VEHICLE_*  
 * ```
 * @param vehicle - 
 * @returns void
 * @remarks Hash: 0xDBA3C090E3D74690
 */
export function RequestVehicleDashboardScaleformMovie(vehicle: number): void;

/**
 * Checks the angle of the door mapped from 0.0 - 1.0 where 0.0 is fully closed and 1.0 is fully open.
 * 
 * See eDoorId declared in [`SET_VEHICLE_DOOR_SHUT`](#_0x93D9BD300D7789E5)
 * @param vehicle - 
 * @param doorIndex - 
 * @returns number
 * @remarks Hash: 0xFE3F9C29F7B32BD5
 */
export function getVehicleDoorAngleRatio(vehicle: number, doorIndex: number): number;

/**
 * Sets the vehicle lights state. Allowing for different lighting modes.
 * 
 * ```
 * NativeDB Introduced: v323
 * ```
 * 
 * ```c
 * enum eVehicleLightSetting {
 *     // Normal light behavior. Lights cycle through off, then low beams, then high beams.
 *     // Note: It's affected by day or night; high beams don't exist in daytime.
 *     NO_VEHICLE_LIGHT_OVERRIDE = 0,
 *     // Vehicle doesn't have lights, always off.
 *     FORCE_VEHICLE_LIGHTS_OFF  = 1, 
 *     // Vehicle has always-on lights.
 *     // During day: Cycles between low beams and high beams. 
 *     // At night: Cycles between low beams, low beams, and high beams.
 *     FORCE_VEHICLE_LIGHTS_ON   = 2,
 *     // Sets vehicle lights on. Behaves like normal lights (same as 0).
 *     SET_VEHICLE_LIGHTS_ON     = 3,
 *     // Sets vehicle lights off. Behaves like normal lights (same as 0).
 *     SET_VEHICLE_LIGHTS_OFF    = 4 
 * };
 * ```
 * @param vehicle - 
 * @param state - 
 * @returns void
 * @remarks Hash: 0x34E710FF01247C5A
 */
export function setVehicleLights(vehicle: number, state: number): void;

/**
 * ## Parameters
 * *
 * @param vehicle - 
 * @param modType - 
 * @returns string
 * @remarks Hash: 0x51F0FEB9F6AE98C0
 */
export function getModSlotName(vehicle: number, modType: number): string;

/**
 * ```
 * paintType:  
 * 0: Normal  
 * 1: Metallic  
 * 2: Pearl  
 * 3: Matte  
 * 4: Metal  
 * 5: Chrome  
 * color: number of the color.  
 * p3 seems to always be 0.  
 * ```
 * @param vehicle - 
 * @param paintType - 
 * @param color - 
 * @param pearlescentColor - 
 * @returns void
 * @remarks Hash: 0x43FEB945EE7F85B8
 */
export function setVehicleModColor1(vehicle: number, paintType: number, color: number, pearlescentColor: number): void;

/**
 * ```
 * p1 is always 0 in the scripts.  
 * p1 = check if vehicle is on fire  
 * ```
 * @param vehicle - 
 * @param isOnFireCheck - 
 * @returns number
 * @remarks Hash: 0x4C241E39B23DF959
 */
export function isVehicleDriveable(vehicle: number, isOnFireCheck: number): number;

/**
 * ## Parameters
 * *
 * @param vehicle - 
 * @param toggle - 
 * @returns void
 * @remarks Hash: 0x80E3357FDEF45C21
 */
export function 0x80e3357fdef45c21(vehicle: number, toggle: number): void;

/**
 * ## Parameters
 * *
 * @param decorator - 
 * @returns number
 * @remarks Hash: 0x956B409B984D9BF7
 */
export function doesVehicleExistWithDecorator(decorator: string): number;

/**
 * ## Parameters
 * *
 * @param vehicle - 
 * @param toggle - 
 * @returns void
 * @remarks Hash: 0x25367DE49D64CF16
 */
export function setDisablePretendOccupants(vehicle: number, toggle: number): void;

/**
 * ```
 * HAS_*
 * ```
 * @returns number
 * @remarks Hash: 0x91D6DD290888CBAB
 */
export function HasFilledVehiclePopulation(): number;

/**
 * ## Parameters
 * *
 * @param vehicle - 
 * @param toggle - 
 * @returns void
 * @remarks Hash: 0xD8050E0EB60CF274
 */
export function setVehicleHasMutedSirens(vehicle: number, toggle: number): void;

/**
 * ## Parameters
 * *
 * @param vehicle - 
 * @param p1 - 
 * @returns void
 * @remarks Hash: 0x3441CAD2F2231923
 */
export function 0x3441cad2f2231923(vehicle: number, p1: number): void;

/**
 * ## Parameters
 * *
 * @param vehicle - 
 * @param p1 - 
 * @param p2 - 
 * @returns any
 * @remarks Hash: 0x8BA6F76BC53A1493
 */
export function setVehicleAutomaticallyAttaches(vehicle: number, p1: number, p2: any): any;

/**
 * ## Parameters
 * *
 * @param vehicle - 
 * @returns void
 * @remarks Hash: 0x8F719973E1445BA2
 */
export function setBoatSinksWhenWrecked(vehicle: number): void;

/**
 * ```
 * Has something to do with trains. Always precedes SET_MISSION_TRAIN_AS_NO_LONGER_NEEDED.  
 * ============================================  
 * May be true that it can be used with trains not sure, but not specifically for trains. Go find Xbox360 decompiled scripts and search for 'func_1333' in freemode.c it isn't used just for trains. Thanks for the info tho.  
 * Btw, func_1333 ends up calling this func which uses this native,  
 * void func_1338(int iParam0)//Position   
 * {  
 * 	ENTITY::FREEZE_ENTITY_POSITION(iParam0, true);  
 * 	ENTITY::SET_ENTITY_COLLISION(iParam0, false, 0);  
 * 	ENTITY::SET_ENTITY_INVINCIBLE(iParam0, true);  
 * 	VEHICLE::_0xDF594D8D(iParam0, true);  
 * }  
 * ```
 * @param vehicle - 
 * @param toggle - 
 * @returns void
 * @remarks Hash: 0x1CF38D529D7441D9
 */
export function SetVehicleSt(vehicle: number, toggle: number): void;

/**
 * Sets the amount of bombs that this vehicle has. As far as I know, this does _not_ impact vehicle weapons or the ammo of those weapons in any way, it is just a way to keep track of the amount of bombs in a specific plane. 
 * 
 * In decompiled scripts this is used to deduct from or add to the count whenever bombs are dropped or purchased/restocked. 
 * 
 * Use [`_GET_AIRCRAFT_BOMB_COUNT`](#_0xEA12BD130D7569A1) to get the amount of bombs on that vehicle.
 * @param aircraft - 
 * @param bombCount - 
 * @returns void
 * @remarks Hash: 0xF4B2ED59DEB5D774
 */
export function SetVehicleBombCount(aircraft: number, bombCount: number): void;

/**
 * ## Parameters
 * *
 * @param cargobob - 
 * @param strength - 
 * @returns void
 * @remarks Hash: 0xBCBFCD9D1DAC19E2
 */
export function setCargobobPickupMagnetStrength(cargobob: number, strength: number): void;

/**
 * ## Parameters
 * *
 * @param recording - 
 * @param time - 
 * @returns { x: number, y: number, z: number }
 * @remarks Hash: 0x2058206FBE79A8AD
 */
export function getRotationOfVehicleRecordingAtTime(recording: number, time: number): { x: number, y: number, z: number };

/**
 * ```
 * -1 = no livery  
 * ```
 * @param vehicle - 
 * @returns number
 * @remarks Hash: 0x2BB9230590DA5E8A
 */
export function getVehicleLivery(vehicle: number): number;

/**
 * ## Parameters
 * *
 * @param vehicle - 
 * @returns void
 * @remarks Hash: 0x632A689BF42301B1
 */
export function pausePlaybackRecordedVehicle(vehicle: number): void;

/**
 * ## Parameters
 * *
 * @param cargobob - 
 * @returns void
 * @remarks Hash: 0xCF1182F682F65307
 */
export function setCargobobPickupRopeDampingMultiplier(cargobob: number): void;

/**
 * ## Parameters
 * *
 * @param vehicle - 
 * @param weaponIndex - 
 * @returns number
 * @remarks Hash: 0x8181CE2F25CB9BB7
 */
export function getVehicleWeaponRestrictedAmmo(vehicle: number, weaponIndex: number): number;

/**
 * Returns the acceleration of the specified model.
 * @param modelHash - 
 * @returns number
 * @remarks Hash: 0x8C044C5C84505B6A
 */
export function getVehicleModelAcceleration(modelHash: number): number;

/**
 * ```
 * NativeDB Introduced: v3095
 * ```
 * 
 * Enables or disables the use of the vehicle's horn button for activating the nitrous system.
 * @param vehicle - 
 * @param bToggle - 
 * @returns void
 * @remarks Hash: 0x1980F68872CC2C3D
 */
export function SetVehicleUseHornButtonForNitrous(vehicle: number, bToggle: number): void;

/**
 * ```
 * Controls how fast the tires wear out.
 * Default values from Rockstar's Open Wheel Race JSON's:
 * "owrtss" (Soft): 2.2
 * "owrtsm" (Medium): 1.7
 * "owrtsh" (Hard): 1.2
 * Usable wheels:
 * 0: wheel_lf
 * 1: wheel_rf
 * 2: wheel_lm1
 * 3: wheel_rm1
 * 4: wheel_lr
 * 5: wheel_rr
 * ```
 * 
 * ```
 * NativeDB Introduced: v2060
 * ```
 * @param vehicle - 
 * @param wheelIndex - 
 * @param multiplier - 
 * @returns void
 * @remarks Hash: 0x392183BB9EA57697
 */
export function SetTyreSoftnessMultiplier(vehicle: number, wheelIndex: number, multiplier: number): void;

/**
 * ```
 * vehicle must be a plane
 * ```
 * @param vehicle - 
 * @param toggle - 
 * @returns void
 * @remarks Hash: 0xCAC66558B944DA67
 */
export function setVehicleUsesLargeRearRamp(vehicle: number, toggle: number): void;

/**
 * Used to control train speed, can be used to start and stop its movement as well.
 * @param train - 
 * @param speed - 
 * @returns void
 * @remarks Hash: 0x16469284DB8C62B5
 */
export function setTrainCruiseSpeed(train: number, speed: number): void;

/**
 * Remove the weird shadow applied by [_SET_VEHICLE_SHADOW_EFFECT](#_0x2A70BAE8883E4C81)
 * @param vehicle - 
 * @returns void
 * @remarks Hash: 0xF87D9F2301F7D206
 */
export function RemoveVehicleShadowEffect(vehicle: number): void;

/**
 * ```
 * Returns whether this vehicle is currently disabled by an EMP mine.
 * 
 * NativeDB Introduced: v1604
 * ```
 * @param vehicle - 
 * @returns number
 * @remarks Hash: 0x0506ED94363AD905
 */
export function GetIsVehicleEmpDisabled(vehicle: number): number;

/**
 * ## Parameters
 * *
 * @param model - 
 * @returns number
 * @remarks Hash: 0x45A9187928F4B9E3
 */
export function isThisModelABoat(model: number): number;

/**
 * Sets the boat boom position for the `TR3` trailer.
 * 
 * Ratio value is between `0.0` and `1.0`, where `0.0` is 90 degrees to the left of the boat, and `1.0` is just slightly to the right/back of the boat.
 * 
 * To get the current boom position ratio, use [GET_BOAT_BOOM_POSITION_RATIO](#_0x6636C535F6CC2725).
 * @param vehicle - 
 * @param ratio - 
 * @returns void
 * @remarks Hash: 0xF488C566413B4232
 */
export function SetBoatBoomPositionRatio(vehicle: number, ratio: number): void;

/**
 * ```
 * indices:  
 * 0 = Left  
 * 1 = Right  
 * 2 = Front  
 * 3 = Back  
 * ```
 * @param vehicle - 
 * @param index - 
 * @returns number
 * @remarks Hash: 0x8C4B92553E4766A5
 */
export function IsVehicleNeonLightEnabled(vehicle: number, index: number): number;

/**
 * ## Parameters
 * *
 * @param recording - 
 * @returns number
 * @remarks Hash: 0x300D614A4C785FC4
 */
export function hasVehicleRecordingBeenLoaded(recording: number): number;

/**
 * Sets the vehicle headlight shadow flags.
 * 
 * ```
 * NativeDB Introduced: v323
 * ```
 * 
 * ```c
 * enum eVehicleHeadlightShadowFlags {
 *     // Default (Lights can be toggled between off, normal and high beams)
 *     NO_HEADLIGHT_SHADOWS = 0,
 *     // Lights Disabled (Lights are fully disabled, cannot be toggled)
 *     HEADLIGHTS_CAST_DYNAMIC_SHADOWS = 1,
 *     // Always On (Lights can be toggled between normal and high beams)
 *     HEADLIGHTS_CAST_STATIC_SHADOWS = 2,
 *     HEADLIGHTS_CAST_FULL_SHADOWS = 3 
 * };
 * ```
 * @param vehicle - 
 * @param flag - 
 * @returns void
 * @remarks Hash: 0x1FD09E7390A74D54
 */
export function setVehicleHeadlightShadows(vehicle: number, flag: number): void;

/**
 * ## Parameters
 * *
 * @param vehicle - 
 * @param toggle - 
 * @returns void
 * @remarks Hash: 0x21115BCD6E44656A
 */
export function setVehicleActiveForPedNavigation(vehicle: number, toggle: number): void;

/**
 * ## Parameters
 * *
 * @param vehicleClass - 
 * @returns number
 * @remarks Hash: 0x4BF54C16EC8FEC03
 */
export function getVehicleClassMaxBraking(vehicleClass: number): number;

/**
 * ```
 * Public Function isVehicleOnAllWheels(vh As Vehicle) As Boolean  
 * Return Native.Function.Call(Of Boolean)(Hash.IS_VEHICLE_ON_ALL_WHEELS, vh)  
 * 		    End Function  
 * ```
 * @param vehicle - 
 * @returns number
 * @remarks Hash: 0xB104CD1BABF302E2
 */
export function isVehicleOnAllWheels(vehicle: number): number;

/**
 * This native does no interpolation between pathpoints. The same position will be returned for all times up to the next pathpoint in the recording.
 * 
 * See [`REQUEST_VEHICLE_RECORDING`](#_0xAF514CABE74CBF15).
 * @param recording - 
 * @param time - 
 * @param script - 
 * @returns { x: number, y: number, z: number }
 * @remarks Hash: 0xD242728AA6F0FBA2
 */
export function getPositionOfVehicleRecordingAtTime(recording: number, time: number, script: string): { x: number, y: number, z: number };

/**
 * ```
 * Usage:  
 * public bool isCopInRange(Vector3 Location, float Range)  
 *         {  
 *             return Function.Call<bool>(Hash.IS_COP_PED_IN_AREA_3D, Location.X - Range, Location.Y - Range, Location.Z - Range, Location.X + Range, Location.Y + Range, Location.Z + Range);  
 *         }  
 * ```
 * @param x1 - 
 * @param x2 - 
 * @param y1 - 
 * @param y2 - 
 * @param z1 - 
 * @param z2 - 
 * @returns number
 * @remarks Hash: 0x7EEF65D5F153E26A
 */
export function isCopVehicleInArea3d(x1: number, x2: number, y1: number, y2: number, z1: number, z2: number): number;

/**
 * ## Parameters
 * *
 * @param plane - 
 * @param height - 
 * @returns void
 * @remarks Hash: 0xB893215D8D4C015B
 */
export function setTaskVehicleGotoPlaneMinHeightAboveTerrain(plane: number, height: number): void;

/**
 * ## Parameters
 * *
 * @param train - 
 * @param speed - 
 * @returns void
 * @remarks Hash: 0xAA0BC91BE0B796E3
 */
export function setTrainSpeed(train: number, speed: number): void;

/**
 * ## Parameters
 * *
 * @param vehicle - 
 * @param toggle - 
 * @returns void
 * @remarks Hash: 0xC45C27EF50F36ADC
 */
export function setVehicleUsePlayerLightSettings(vehicle: number, toggle: number): void;

/**
 * ## Parameters
 * *
 * @param vehicle - 
 * @param toggle - 
 * @returns void
 * @remarks Hash: 0x1201E8A3290A3B98
 */
export function SetCamberedWheelsDisabled(vehicle: number, toggle: number): void;

/**
 * ```
 * Appears to return false if any window is broken.  
 * ```
 * @param vehicle - 
 * @returns number
 * @remarks Hash: 0x11D862A3E977A9EF
 */
export function areAllVehicleWindowsIntact(vehicle: number): number;

/**
 * ## Parameters
 * *
 * @param p0 - 
 * @param p1 - 
 * @returns any
 * @remarks Hash: 0xD3E51C0AB8C26EEE
 */
export function 0xd3e51c0ab8c26eee(p0: any, p1: any): any;

/**
 * ## Parameters
 * *
 * @param p0 - 
 * @param p1 - 
 * @returns void
 * @remarks Hash: 0x35BB21DE06784373
 */
export function 0x35bb21de06784373(p0: any, p1: any): void;

/**
 * ```
 * Only used in R* Script fm_content_cargo
 * ```
 * 
 * ```
 * NativeDB Introduced: v2699
 * ```
 * @param vehicle - 
 * @param p1 - 
 * @returns void
 * @remarks Hash: 0xEF9D388F8D377F44
 */
export function 0xef9d388f8d377f44(vehicle: number, p1: number): void;

/**
 * ```
 * Only works on bikes, both X and Y work in the -1 - 1 range.
 * X forces the bike to turn left or right (-1, 1)
 * Y forces the bike to lean to the left or to the right (-1, 1)
 * Example with X -1/Y 1
 * http://i.imgur.com/TgIuAPJ.jpg
 * ```
 * @param vehicle - 
 * @param x - 
 * @param y - 
 * @returns void
 * @remarks Hash: 0x9CFA4896C3A53CBB
 */
export function setBikeOnStand(vehicle: number, x: number, y: number): void;

/**
 * ## Parameters
 * *
 * @param vehicle - 
 * @returns number
 * @remarks Hash: 0x50634E348C8D44EF
 */
export function getVehicleHasKers(vehicle: number): number;

/**
 * ```
 * Money pickups are created around cars when they explode. Only works when the vehicle model is a car. A single pickup is between 1 and 18 dollars in size. All car models seem to give the same amount of money.
 * youtu.be/3arlUxzHl5Y
 * i.imgur.com/WrNpYFs.jpg
 * ```
 * @param vehicle - 
 * @param toggle - 
 * @returns void
 * @remarks Hash: 0x068F64F2470F9656
 */
export function setVehicleDropsMoneyWhenBlownUp(vehicle: number, toggle: number): void;

/**
 * Toggles whether ambient trains can spawn on the specified track or not.
 * 
 * | trackId | File | Description |
 * |
 * @param trackId - 
 * @param state - 
 * @returns void
 * @remarks Hash: 0xFD813BB7DB977F20
 */
export function switchTrainTrack(trackId: number, state: number): void;

/**
 * ## Parameters
 * *
 * @param p0 - 
 * @param p1 - 
 * @param p2 - 
 * @param p3 - 
 * @returns any
 * @remarks Hash: 0xE023E8AC4EF7C117
 */
export function setVehicleUseCutsceneWheelCompression(p0: number, p1: number, p2: number, p3: number): any;

/**
 * ```
 * GET_VEHICLE_MODEL_*
 * Function pertains only to aviation vehicles.
 * ```
 * @param modelHash - 
 * @returns number
 * @remarks Hash: 0xC6AD107DDC9054CC
 */
export function GetVehicleModelMaxKnots(modelHash: number): number;

/**
 * ## Parameters
 * *
 * @param vehicle - 
 * @returns number
 * @remarks Hash: 0x7504C0F113AB50FC
 */
export function isTaxiLightOn(vehicle: number): number;

/**
 * See eDoorId declared in [`SET_VEHICLE_DOOR_SHUT`](#_0x93D9BD300D7789E5)
 * @param vehicle - 
 * @param doorIndex - 
 * @returns { x: number, y: number, z: number }
 * @remarks Hash: 0xC0572928C0ABFDA3
 */
export function GetEntryPositionOfDoor(vehicle: number, doorIndex: number): { x: number, y: number, z: number };

/**
 * ## Parameters
 * *
 * @param vehicle - 
 * @returns void
 * @remarks Hash: 0x32CAEDF24A583345
 */
export function DisableVehicleTurretMovementThisFrame(vehicle: number): void;

/**
 * ```
 * seems to make the vehicle stop spawning naturally in traffic. Here's an essential example:  
 * VEHICLE::SET_VEHICLE_MODEL_IS_SUPPRESSED(GAMEPLAY::GET_HASH_KEY("taco"), true);  
 * ```
 * @param model - 
 * @param suppressed - 
 * @returns void
 * @remarks Hash: 0x0FC2D89AC25A5814
 */
export function setVehicleModelIsSuppressed(model: number, suppressed: number): void;

/**
 * ## Parameters
 * *
 * @param cargobob - 
 * @param vehicle - 
 * @returns void
 * @remarks Hash: 0xE301BD63E9E13CF0
 */
export function setCargobobPickupMagnetReducedStrength(cargobob: number, vehicle: number): void;

/**
 * ```
 * Returns the number of *types* of licence plates, enumerated below in SET_VEHICLE_NUMBER_PLATE_TEXT_INDEX.  
 * ```
 * @returns number
 * @remarks Hash: 0x4C4D6B2644F458CB
 */
export function getNumberOfVehicleNumberPlates(): number;

/**
 * ## Parameters
 * *
 * @param vehicle - 
 * @param health - 
 * @returns void
 * @remarks Hash: 0x4056EA1105F5ABD7
 */
export function SetHeliMainRotorHealth(vehicle: number, health: number): void;

/**
 * ```
 * p1 can be anywhere from 0 to 3 in the scripts. p2 is generally somewhere in the 1000 to 10000 range.  
 * ```
 * @param vehicle - 
 * @param p1 - 
 * @param p2 - 
 * @returns number
 * @remarks Hash: 0x679BE1DAF71DA874
 */
export function isVehicleStuckTimerUp(vehicle: number, p1: number, p2: number): number;

/**
 * See eWindowId declared in [`IS_VEHICLE_WINDOW_INTACT`](#_0x46E571A0E20D01F1).
 * 
 * This function is coded to not work on vehicles of type: `CBike`, `Bmx`, `CBoat`, `CTrain`, and `CSubmarine`.
 * @param vehicle - 
 * @param windowIndex - 
 * @returns void
 * @remarks Hash: 0x772282EBEB95E682
 */
export function fixVehicleWindow(vehicle: number, windowIndex: number): void;

/**
 * ```c
 * enum eColourBitField {
 *     HAS_BODY_COLOUR1 = 1,
 *     HAS_BODY_COLOUR2 = 2,
 *     HAS_BODY_COLOUR3 = 4,
 *     HAS_BODY_COLOUR4 = 8,
 *     HAS_BODY_COLOUR5 = 16
 * }
 * ```
 * @param vehicle - 
 * @returns number
 * @remarks Hash: 0xEEBFC7A7EFDC35B4
 */
export function getVehicleColoursWhichCanBeSet(vehicle: number): number;

/**
 * ## Parameters
 * *
 * @param vehicle - 
 * @param multiplier - 
 * @returns void
 * @remarks Hash: 0x45A561A9421AB6AD
 */
export function SetVehicleUnkDamageMultiplier(vehicle: number, multiplier: number): void;

/**
 * ## Parameters
 * *
 * @param vehicle - 
 * @returns void
 * @remarks Hash: 0x3556041742A0DC74
 */
export function closeBombBayDoors(vehicle: number): void;

/**
 * ## Parameters
 * *
 * @param vehicleAsset - 
 * @returns number
 * @remarks Hash: 0x1BBE0523B8DB9A21
 */
export function hasVehicleAssetLoaded(vehicleAsset: number): number;

/**
 * ## Parameters
 * *
 * @param cargobob - 
 * @param vehicleAttached - 
 * @returns number
 * @remarks Hash: 0xD40148F22E81A1D9
 */
export function isVehicleAttachedToCargobob(cargobob: number, vehicleAttached: number): number;

/**
 * ## Parameters
 * *
 * @param vehicle - 
 * @param toggle - 
 * @returns void
 * @remarks Hash: 0x06582AFF74894C75
 */
export function setVehicleInactiveDuringPlayback(vehicle: number, toggle: number): void;

/**
 * ```
 * Returns true if vehicle is halted by BRING_VEHICLE_TO_HALT
 * _IS_VEHICLE_*
 * ```
 * 
 * ```
 * NativeDB Introduced: v1493
 * ```
 * @param vehicle - 
 * @returns number
 * @remarks Hash: 0xC69BB1D832A710EF
 */
export function IsVehicleBeingHalted(vehicle: number): number;

/**
 * Examples with a besra:
 * 
 * - [fade value `0.0`](https://i.imgur.com/DXNk63e.jpg)
 * - [fade value `0.5`](https://i.imgur.com/2Vb35fq.jpg)
 * - [fade value `1.0`](https://i.imgur.com/aa8cxaD.jpg)
 * 
 * The parameter fade is a value from 0-1, where 0 is fresh paint.
 * @param vehicle - 
 * @param fade - 
 * @returns void
 * @remarks Hash: 0x3AFDC536C3D01674
 */
export function setVehicleEnveffScale(vehicle: number, fade: number): void;

/**
 * ## Parameters
 * *
 * @param aircraft - 
 * @returns number
 * @remarks Hash: 0xDA62027C8BDB326E
 */
export function getVehicleFlightNozzlePosition(aircraft: number): number;

/**
 * ## Parameters
 * *
 * @param vehicle - 
 * @param toggle - 
 * @returns void
 * @remarks Hash: 0x9737A37136F07E75
 */
export function setVehicleDoorsLockedForNonScriptPlayers(vehicle: number, toggle: number): void;

/**
 * ## Parameters
 * *
 * @returns number
 * @remarks Hash: 0xA1A9FC1C76A6730D
 */
export function IsThisModelAnAmphibiousQuadbike(): number;

/**
 * Disables collision for this vehicle (maybe it also supports other entities, not sure).
 * Only world/building/fixed world objects will have their collisions disabled, props, peds, or any other entity still collides with the vehicle.
 * 
 * [Example video](https://streamable.com/6n45d5)
 * 
 * Not sure if there is a native (and if so, which one) that resets the collisions.
 * @param vehicle - 
 * @returns void
 * @remarks Hash: 0x75627043C6AA90AD
 */
export function DisableVehicleWorldCollision(vehicle: number): void;

/**
 * ## Parameters
 * *
 * @param range - 
 * @returns void
 * @remarks Hash: 0x90B6DA738A9A25DA
 */
export function setAmbientVehicleRangeMultiplierThisFrame(range: number): void;

/**
 * ## Parameters
 * *
 * @param vehicle - 
 * @param toggle - 
 * @returns void
 * @remarks Hash: 0x28D034A93FE31BF5
 */
export function SetVehicleReceivesRampDamage(vehicle: number, toggle: number): void;

/**
 * ## Parameters
 * *
 * @param vehicle - 
 * @returns void
 * @remarks Hash: 0x5FFBDEEC3E8E2009
 */
export function clearVehicleCustomSecondaryColour(vehicle: number): void;

/**
 * Transforms the `stormberg` to its "road vehicle" variant. If the vehicle is already in that state then the vehicle transformation audio will still play, but the vehicle won't change at all.
 * @param vehicle - 
 * @param instantly - 
 * @returns void
 * @remarks Hash: 0x2A69FFD1B42BFF9E
 */
export function transformToCar(vehicle: number, instantly: number): void;

/**
 * ## Parameters
 * *
 * @param vehicle - 
 * @param toggle - 
 * @returns void
 * @remarks Hash: 0x09606148B6C71DEF
 */
export function setVehicleRudderBroken(vehicle: number, toggle: number): void;

/**
 * ```
 * Implemented only for Trains.
 * ```
 * 
 * ```
 * NativeDB Introduced: v2372
 * ```
 * @param vehicle - 
 * @param toggle - 
 * @returns void
 * @remarks Hash: 0xEC0C1D4922AF9754
 */
export function NetworkUseHighPrecisionVehicleBlending(vehicle: number, toggle: number): void;

/**
 * ## Parameters
 * *
 * @param vehicle - 
 * @param speed - 
 * @returns void
 * @remarks Hash: 0x1093408B4B9D1146
 */
export function setVehicleTurretSpeedThisFrame(vehicle: number, speed: number): void;

/**
 * ```
 * Returns last vehicle that was rammed by the given vehicle using the shunt boost.
 * 
 * NativeDB Introduced: v1604
 * ```
 * @param vehicle - 
 * @returns number
 * @remarks Hash: 0x04F2FA6E234162F7
 */
export function GetLastRammedVehicle(vehicle: number): number;

/**
 * ## Parameters
 * *
 * @param p0 - 
 * @param p1 - 
 * @param p2 - 
 * @param p3 - 
 * @param p4 - 
 * @param p5 - 
 * @param p6 - 
 * @returns number
 * @remarks Hash: 0xC5574E0AEB86BA68
 */
export function getRandomVehicleFrontBumperInSphere(p0: number, p1: number, p2: number, p3: number, p4: number, p5: number, p6: number): number;

/**
 * ## Parameters
 * *
 * @param vehicle - 
 * @param toggle - 
 * @returns void
 * @remarks Hash: 0x206BC5DC9D1AC70A
 */
export function SetVehicleCanEngineOperateOnFire(vehicle: number, toggle: number): void;

/**
 * Retrieves a static value representing the estimated max speed of a specific vehicle, including any vehicle mods. This value does not change dynamically during gameplay.
 * 
 * ```
 * NativeDB Introduced: v323
 * ```
 * @param vehicle - 
 * @returns number
 * @remarks Hash: 0x53AF99BAA671CA47
 */
export function getVehicleEstimatedMaxSpeed(vehicle: number): number;

/**
 * ## Parameters
 * *
 * @param vehicle - 
 * @param toggle - 
 * @returns void
 * @remarks Hash: 0xFBA550EA44404EE6
 */
export function setVehicleNeedsToBeHotwired(vehicle: number, toggle: number): void;

/**
 * ## Parameters
 * *
 * @param vehicleClass - 
 * @returns number
 * @remarks Hash: 0x00C09F246ABEDD82
 */
export function getVehicleClassEstimatedMaxSpeed(vehicleClass: number): number;

/**
 * ## Parameters
 * *
 * @param vehicle - 
 * @param p1 - 
 * @returns void
 * @remarks Hash: 0xA17BAD153B51547E
 */
export function setCargobobPickupMagnetEffectRadius(vehicle: number, p1: number): void;

/**
 * ## Parameters
 * *
 * @param vehicle - 
 * @param modType - 
 * @returns number
 * @remarks Hash: 0x84B233A8C8FC8AE7
 */
export function isToggleModOn(vehicle: number, modType: number): number;

/**
 * ## Parameters
 * *
 * @param vehicle - 
 * @param toggle - 
 * @returns void
 * @remarks Hash: 0x21D2E5662C1F6FED
 */
export function resetVehicleWheels(vehicle: number, toggle: number): void;

/**
 * ```
 * Controls how much traction the wheel loses.
 * Default values from Rockstar's Open Wheel Race JSON's:
 * "owrtds" (Soft): 0.05
 * "owrtdm" (Medium): 0.45
 * "owrtdh" (Hard): 0.8
 * Usable wheels:
 * 0: wheel_lf
 * 1: wheel_rf
 * 2: wheel_lm1
 * 3: wheel_rm1
 * 4: wheel_lr
 * 5: wheel_rr
 * ```
 * 
 * ```
 * NativeDB Introduced: v2060
 * ```
 * @param vehicle - 
 * @param wheelIndex - 
 * @param multiplier - 
 * @returns void
 * @remarks Hash: 0xC970D0E0FC31D768
 */
export function SetTyreTractionLossMultiplier(vehicle: number, wheelIndex: number, multiplier: number): void;

/**
 * ## Parameters
 * *
 * @param p0 - 
 * @returns any
 * @remarks Hash: 0x8533CAFDE1F0F336
 */
export function 0x8533cafde1f0f336(p0: any): any;

/**
 * Stops cargobob from being able to detach the attached vehicle.
 * @param cargobob - 
 * @param toggle - 
 * @returns void
 * @remarks Hash: 0x571FEB383F629926
 */
export function setCargobobForceDontDetachVehicle(cargobob: number, toggle: number): void;

/**
 * ## Parameters
 * *
 * @param x - 
 * @param y - 
 * @param z - 
 * @param radius - 
 * @param speed - 
 * @param p5 - 
 * @returns number
 * @remarks Hash: 0x2CE544C68FB812A0
 */
export function addRoadNodeSpeedZone(x: number, y: number, z: number, radius: number, speed: number, p5: number): number;

/**
 * ## Parameters
 * *
 * @param vehicle - 
 * @returns number
 * @remarks Hash: 0xFC058F5121E54C32
 */
export function getVehicleModKitType(vehicle: number): number;

/**
 * ```
 * p1, p2, p3 are RGB values for color (255,0,0 for Red, ect)  
 * ```
 * @param vehicle - 
 * @param r - 
 * @param g - 
 * @param b - 
 * @returns void
 * @remarks Hash: 0x36CED73BFED89754
 */
export function setVehicleCustomSecondaryColour(vehicle: number, r: number, g: number, b: number): void;

/**
 * ```
 * NativeDB Introduced: v1604
 * ```
 * @param p0 - 
 * @returns void
 * @remarks Hash: 0xA4A9A4C40E615885
 */
export function 0xa4a9a4c40e615885(p0: any): void;

/**
 * ## Parameters
 * *
 * @param p0 - 
 * @param p1 - 
 * @returns any
 * @remarks Hash: 0x5BA68A0840D546AC
 */
export function 0x5ba68a0840d546ac(p0: any, p1: any): any;

/**
 * ```
 * Changes the secondary paint type and color  
 * paintType:  
 * 0: Normal  
 * 1: Metallic  
 * 2: Pearl  
 * 3: Matte  
 * 4: Metal  
 * 5: Chrome  
 * color: number of the color  
 * ```
 * @param vehicle - 
 * @param paintType - 
 * @param color - 
 * @returns void
 * @remarks Hash: 0x816562BADFDEC83E
 */
export function setVehicleModColor2(vehicle: number, paintType: number, color: number): void;

/**
 * ```
 * min: 1.9f, max: 100.0f
 * ```
 * @param cargobob - 
 * @param length1 - 
 * @param length2 - 
 * @returns void
 * @remarks Hash: 0x877C1EAEAC531023
 */
export function setPickupRopeLengthForCargobob(cargobob: number, length1: number, length2: number): void;

/**
 * ## Parameters
 * *
 * @param vehicle - 
 * @returns void
 * @remarks Hash: 0x445D79F995508307
 */
export function releasePreloadMods(vehicle: number): void;

/**
 * ## Parameters
 * *
 * @param p0 - 
 * @param p1 - 
 * @returns any
 * @remarks Hash: 0xD4196117AF7BB974
 */
export function 0xd4196117af7bb974(p0: any, p1: any): any;

/**
 * ## Parameters
 * *
 * @param vehicle - 
 * @returns void
 * @remarks Hash: 0x90532EDF0D2BDD86
 */
export function detachVehicleFromTrailer(vehicle: number): void;

/**
 * ## Parameters
 * *
 * @param toggle - 
 * @returns void
 * @remarks Hash: 0x51DB102F4A3BA5E0
 */
export function 0x51db102f4a3ba5e0(toggle: number): void;

/**
 * Will disable a plane or a helicopter's need to swerve around object in its heightmap when using TASK_PLANE_MISSION or other AI / Pilot behavior.  Will ensure plane flys directly to it's destination or die trying! This native does NOT need to be called every frame, but instead, just called once on the vehicle (NOT THE PED) you're trying to disable avoidance for!
 * @param vehicle - 
 * @param avoidObstacles - 
 * @returns void
 * @remarks Hash: 0x8AA9180DE2FEDD45
 */
export function EnableAircraftObstacleAvoidance(vehicle: number, avoidObstacles: number): void;

/**
 * ```
 * Returns true if the vehicle's current speed is less than, or equal to 0.0025f.
 * For some vehicles it returns true if the current speed is <= 0.00039999999.
 * ```
 * @param vehicle - 
 * @returns number
 * @remarks Hash: 0x5721B434AD84D57A
 */
export function isVehicleStopped(vehicle: number): number;

/**
 * ```
 * Only works during nighttime.
 * ```
 * @param heli - 
 * @param toggle - 
 * @param canBeUsedByAI - 
 * @returns void
 * @remarks Hash: 0x14E85C5EE7A4D542
 */
export function setVehicleSearchlight(heli: number, toggle: number, canBeUsedByAI: number): void;

/**
 * ```
 * True stops vtols from switching modes. Doesn't stop the sound though.
 * ```
 * 
 * ```
 * NativeDB Introduced: v1290
 * ```
 * @param vehicle - 
 * @returns void
 * @remarks Hash: 0xCE2B43770B655F8F
 */
export function SetDisableVehicleFlightNozzlePosition(vehicle: number): void;

/**
 * ## Parameters
 * *
 * @param vehicle - 
 * @param seatIndex - 
 * @returns number
 * @remarks Hash: 0xE33FFA906CE74880
 */
export function isTurretSeat(vehicle: number, seatIndex: number): number;

/**
 * Checks if a boat can be anchored at its present position, ignoring any players standing on the boat.
 * 
 * ```
 * NativeDB Introduced: v678
 * ```
 * @param boat - 
 * @returns number
 * @remarks Hash: 0x24F4121D07579880
 */
export function canAnchorBoatHereIgnorePlayers(boat: number): number;

/**
 * Determines whether the specified vehicle is equipped with a searchlight.
 * 
 * ```
 * NativeDB Introduced: v2189
 * ```
 * @param vehicle - 
 * @returns number
 * @remarks Hash: 0x99015ED7DBEA5113
 */
export function doesVehicleHaveSearchlight(vehicle: number): number;

/**
 * Allows vehicles with the FLAG_JUMPING_CAR flag to jump higher (i.e. Ruiner 2000).
 * @param vehicle - 
 * @param toggle - 
 * @returns void
 * @remarks Hash: 0xF06A16CA55D138D8
 */
export function SetUseHigherVehicleJumpForce(vehicle: number, toggle: number): void;

/**
 * ## Parameters
 * *
 * @param vehicle - 
 * @returns number
 * @remarks Hash: 0x33F2E3FE70EAAE1D
 */
export function getNumModKits(vehicle: number): number;

/**
 * ```
 * This fixes the deformation of a vehicle but the vehicle health doesn't improve  
 * ```
 * @param vehicle - 
 * @returns void
 * @remarks Hash: 0x953DA1E1B12C0491
 */
export function setVehicleDeformationFixed(vehicle: number): void;

/**
 * This multiplier has no limit, by default the game has this set to `1.0`.
 * @param vehicle - 
 * @param multiplier - 
 * @returns void
 * @remarks Hash: 0xB385454F8791F57C
 */
export function setVehicleLightMultiplier(vehicle: number, multiplier: number): void;

/**
 * ```
 * NativeDB Introduced: v3407
 * ```
 * @param plane - 
 * @param toggle - 
 * @returns void
 * @remarks Hash: 0xFC40CBF7B90CA77C
 */
export function SetPlaneAvoidsOthers(plane: number, toggle: number): void;

/**
 * ```
 * NativeDB Introduced: v3095
 * ```
 * 
 * Recharges the nitrous system of the specified vehicle to its maximum capacity. This action sets the nitrous charge duration to the maximum limit defined by previous settings applied through [`SET_OVERRIDE_NITROUS_LEVEL`](#_0xC8E9B6B71B8E660D).
 * @param vehicle - 
 * @returns void
 * @remarks Hash: 0x1A2BCC8C636F9226
 */
export function fullyChargeNitrous(vehicle: number): void;

/**
 * ## Parameters
 * *
 * @param vehicle - 
 * @param player - 
 * @param toggle - 
 * @returns void
 * @remarks Hash: 0x517AAF684BB50CD1
 */
export function setVehicleDoorsLockedForPlayer(vehicle: number, player: number, toggle: number): void;

/**
 * ## Parameters
 * *
 * @param p0 - 
 * @param p1 - 
 * @returns any
 * @remarks Hash: 0x0419B167EE128F33
 */
export function 0x0419b167ee128f33(p0: any, p1: any): any;

/**
 * ```
 * Returns a float value between 0.0 and 3.0 related to its slipstream draft (boost/speedup).
 * GET_VEHICLE_*
 * ```
 * @param vehicle - 
 * @returns number
 * @remarks Hash: 0x36492C2F0D134C56
 */
export function GetVehicleCurrentSlipstreamDraft(vehicle: number): number;

/**
 * ## Parameters
 * *
 * @param vehicle - 
 * @param toggle - 
 * @returns void
 * @remarks Hash: 0x983765856F2564F9
 */
export function setVehicleEngineCanDegrade(vehicle: number, toggle: number): void;

/**
 * colorPrimary & colorSecondary are the paint indexes for the vehicle.  
 * 
 * For a list of valid paint indexes, view: pastebin.com/pwHci0xK
 * @param vehicle - 
 * @param colorPrimary - 
 * @param colorSecondary - 
 * @returns void
 * @remarks Hash: 0x4F1D4BE3A7F24601
 */
export function setVehicleColours(vehicle: number, colorPrimary: number, colorSecondary: number): void;

/**
 * Set state to true to extend the wings, false to retract them.
 * @param vehicle - 
 * @param extend - 
 * @returns void
 * @remarks Hash: 0x544996C0081ABDEB
 */
export function SetOppressorTransformState(vehicle: number, extend: number): void;

/**
 * ```
 * Returns false if the vehicle has the FLAG_NO_RESPRAY flag set.
 * ```
 * @param vehicle - 
 * @returns number
 * @remarks Hash: 0x8D474C8FAEFF6CDE
 */
export function isVehicleSprayable(vehicle: number): number;

/**
 * ```
 * If set to TRUE, it seems to suppress door noises and doesn't allow the horn to be continuous.  
 * ```
 * @param vehicle - 
 * @param toggle - 
 * @returns void
 * @remarks Hash: 0x9D44FCCE98450843
 */
export function SetVehicleSilent(vehicle: number, toggle: number): void;

/**
 * A getter for [`SET_VEHICLE_DIRT_LEVEL`](#_0x79D3B596FE44EE8B).
 * @param vehicle - 
 * @returns number
 * @remarks Hash: 0x8F17BC8BA08DA62B
 */
export function getVehicleDirtLevel(vehicle: number): number;

/**
 * ## Parameters
 * *
 * @param p0 - 
 * @returns void
 * @remarks Hash: 0xC4B3347BD68BD609
 */
export function 0xc4b3347bd68bd609(p0: any): void;

/**
 * ## Parameters
 * *
 * @param vehicle - 
 * @param p1 - 
 * @returns void
 * @remarks Hash: 0x6EBFB22D646FFC18
 */
export function 0x6ebfb22d646ffc18(vehicle: number, p1: number): void;

/**
 * ```
 * NativeDB Introduced: v1604
 * ```
 * @param toggle - 
 * @returns void
 * @remarks Hash: 0x5BBCF35BF6E456F7
 */
export function 0x5bbcf35bf6e456f7(toggle: number): void;

/**
 * ## Parameters
 * *
 * @param p0 - 
 * @param p1 - 
 * @returns void
 * @remarks Hash: 0xE05DD0E9707003A3
 */
export function 0xe05dd0e9707003a3(p0: any, p1: number): void;

/**
 * ```
 * NativeDB Introduced: v1604
 * ```
 * @param vehicle - 
 * @returns number
 * @remarks Hash: 0x71AFB258CCED3A27
 */
export function GetDoesVehicleHaveTombstone(vehicle: number): number;

/**
 * ## Parameters
 * *
 * @param vehicle - 
 * @param toggle - 
 * @param depth1 - 
 * @param depth2 - 
 * @param depth3 - 
 * @returns void
 * @remarks Hash: 0xC59872A5134879C7
 */
export function setSubmarineCrushDepths(vehicle: number, toggle: number, depth1: number, depth2: number, depth3: number): void;

/**
 * ## Parameters
 * *
 * @param p0 - 
 * @param p1 - 
 * @returns void
 * @remarks Hash: 0x73561D4425A021A2
 */
export function 0x73561d4425a021a2(p0: any, p1: any): void;

/**
 * ```
 * AI abides by the provided driving style (e.g., stopping at red lights or waiting behind traffic) while executing the specificed vehicle recording.
 * 
 * 0x1F2E4E06DEA8992B is a related native that deals with the AI physics for such recordings.
 * ```
 * @param vehicle - 
 * @param recording - 
 * @returns void
 * @remarks Hash: 0x29DE5FA52D00428C
 */
export function startPlaybackRecordedVehicleUsingAi(vehicle: number, recording: number): void;

/**
 * ```
 * Seems to be related to the metal parts, not tyres (like i was expecting lol)  
 * Must be called every tick.  
 * ```
 * @param vehicle - 
 * @param friction - 
 * @returns void
 * @remarks Hash: 0x1837AF7C627009BA
 */
export function setVehicleFrictionOverride(vehicle: number, friction: number): void;

/**
 * ## Parameters
 * *
 * @param vehicle - 
 * @param lightsOn - 
 * @param highbeamsOn - 
 * @returns number
 * @remarks Hash: 0xB91B4C20085BD12F
 */
export function getVehicleLightsState(vehicle: number, lightsOn: number, highbeamsOn: number): number;

/**
 * ## Parameters
 * *
 * @param p0 - 
 * @returns void
 * @remarks Hash: 0xE30524E1871F481D
 */
export function removeVehicleCombatAvoidanceArea(p0: any): void;

/**
 * ## Parameters
 * *
 * @param toggle - 
 * @returns void
 * @remarks Hash: 0x26324F33423F3CC3
 */
export function setFarDrawVehicles(toggle: number): void;

/**
 * ## Parameters
 * *
 * @param vehicle - 
 * @returns number
 * @remarks Hash: 0xAA0A52D24FB98293
 */
export function isVehicleVisible(vehicle: number): number;

/**
 * ## Parameters
 * *
 * @param vehicle - 
 * @param toggle - 
 * @returns void
 * @remarks Hash: 0xB055A34527CB8FD7
 */
export function setVehicleForceAfterburner(vehicle: number, toggle: number): void;

/**
 * ## Parameters
 * *
 * @param vehicle - 
 * @param entity - 
 * @returns number
 * @remarks Hash: 0x57715966069157AD
 */
export function isEntityAttachedToHandlerFrame(vehicle: number, entity: number): number;

/**
 * ```
 * NativeDB Introduced: v323
 * ```
 * @param heli - 
 * @returns number
 * @remarks Hash: 0xAE8CE82A4219AC8C
 */
export function getHeliTailRotorHealth(heli: number): number;

/**
 * ## Parameters
 * *
 * @param vehicle - 
 * @param toggle - 
 * @returns void
 * @remarks Hash: 0x0CDDA42F9E360CA6
 */
export function setVehicleCanDeformWheels(vehicle: number, toggle: number): void;

/**
 * Enables or disables the convertible roof on vehicles that support old-style GTA IV roofs, which are not animated. Setting `toggle` to true will apply the roof to the vehicle, and setting it to false will remove the roof, assuming the vehicle has versions with and without a roof.
 * 
 * If you want to lock or unlock the roof mechanism, use [`SET_CONVERTIBLE_ROOF_LATCH_STATE`](#_0x1A78AD3D8240536F).
 * 
 * You can check if the vehicle has a roof with [`DOES_VEHICLE_HAVE_ROOF`](#_0x8AC862B0B32C5B80).
 * 
 * ```
 * NativeDB Introduced: v323
 * ```
 * @param vehicle - 
 * @param toggle - 
 * @returns void
 * @remarks Hash: 0xF39C4F538B5124C2
 */
export function setConvertibleRoof(vehicle: number, toggle: number): void;

/**
 * ```
 * Related to locking the vehicle or something similar.  
 * In the decompiled scripts, its always called after  
 * VEHICLE::_SET_EXCLUSIVE_DRIVER(a_0, 0, 0);  
 * VEHICLE::SET_VEHICLE_DOORS_LOCKED_FOR_ALL_PLAYERS(a_0, 1);  
 * VEHICLE::SET_VEHICLE_DOORS_LOCKED_FOR_PLAYER(a_0, PLAYER::PLAYER_ID(), 0);  
 * ```
 * @param vehicle - 
 * @param p1 - 
 * @returns void
 * @remarks Hash: 0xDBC631F109350B8C
 */
export function 0xdbc631f109350b8c(vehicle: number, p1: number): void;

/**
 * ```
 * Returns true if the vehicle has the FLAG_ALLOWS_RAPPEL flag set.
 * ```
 * @param vehicle - 
 * @returns number
 * @remarks Hash: 0x4E417C547182C84D
 */
export function DoesVehicleAllowRappel(vehicle: number): number;

/**
 * ```
 * Set modKit to 0 if you plan to call SET_VEHICLE_MOD. That's what the game does. Most body modifications through SET_VEHICLE_MOD will not take effect until this is set to 0.
 * ```
 * @param vehicle - 
 * @param modKit - 
 * @returns void
 * @remarks Hash: 0x1F2AA07F00B3217A
 */
export function setVehicleModKit(vehicle: number, modKit: number): void;

/**
 * ## Parameters
 * *
 * @param p0 - 
 * @returns void
 * @remarks Hash: 0x7BBE7FF626A591FE
 */
export function 0x7bbe7ff626a591fe(p0: any): void;

/**
 * ## Parameters
 * *
 * @param vehicle - 
 * @returns number
 * @remarks Hash: 0x6A842D197F845D56
 */
export function getVehicleColourCombination(vehicle: number): number;

/**
 * ## Parameters
 * *
 * @param p0 - 
 * @param p1 - 
 * @returns void
 * @remarks Hash: 0xB9562064627FF9DB
 */
export function 0xb9562064627ff9db(p0: any, p1: any): void;

/**
 * ## Parameters
 * *
 * @param vehicle - 
 * @returns number
 * @remarks Hash: 0xA132FB5370554DB0
 */
export function getVehicleMaxTraction(vehicle: number): number;

/**
 * ## Parameters
 * *
 * @param vehicle - 
 * @returns number
 * @remarks Hash: 0xDCA174A42133F08C
 */
export function GetHasRetractableWheels(vehicle: number): number;

/**
 * Determines whether the specified Cargobob can pick up a given entity.
 * @param cargobob - 
 * @param entity - 
 * @returns number
 * @remarks Hash: 0x2C1D8B3B19E517CC
 */
export function canCargobobPickUpEntity(cargobob: number, entity: number): number;

/**
 * ```
 * Won't attract or magnetize to any helicopters or planes of course, but that's common sense.  
 * ```
 * @param cargobob - 
 * @param isActive - 
 * @returns void
 * @remarks Hash: 0x9A665550F8DA349B
 */
export function setCargobobPickupMagnetActive(cargobob: number, isActive: number): void;

/**
 * ```
 * Closes all doors of a vehicle:  
 * ```
 * @param vehicle - 
 * @param closeInstantly - 
 * @returns void
 * @remarks Hash: 0x781B3D62BB013EF5
 */
export function setVehicleDoorsShut(vehicle: number, closeInstantly: number): void;

/**
 * See [`SET_VEHICLE_CUSTOM_PRIMARY_COLOUR`](#_0x7141766F91D15BEA) and [`SET_VEHICLE_CUSTOM_SECONDARY_COLOUR`](#_0x36CED73BFED89754).
 * @param vehicle - 
 * @param r - 
 * @param g - 
 * @param b - 
 * @returns void
 * @remarks Hash: 0xF3CC740D36221548
 */
export function getVehicleColor(vehicle: number, r: number, g: number, b: number): void;

/**
 * ## Parameters
 * *
 * @param vehicle - 
 * @param p1 - 
 * @returns void
 * @remarks Hash: 0x9F3F689B814F2599
 */
export function 0x9f3f689b814f2599(vehicle: number, p1: number): void;

/**
 * ## Parameters
 * *
 * @param vehicle - 
 * @returns number
 * @remarks Hash: 0x4319E335B71FFF34
 */
export function isVehicleAlarmActivated(vehicle: number): number;

/**
 * Disables the additional physics forces applied to BMX bikes that enable them to perform tricks.
 * 
 * ```
 * NativeDB Introduced: v463
 * ```
 * @returns void
 * @remarks Hash: 0x26D99D5A82FD18E8
 */
export function setDisableBmxExtraTrickForces(): void;

/**
 * ## Parameters
 * *
 * @param train - 
 * @param x - 
 * @param y - 
 * @param z - 
 * @returns void
 * @remarks Hash: 0x591CA673AA6AB736
 */
export function setMissionTrainCoords(train: number, x: number, y: number, z: number): void;

/**
 * ## Parameters
 * *
 * @param vehicle - 
 * @param toggle - 
 * @returns void
 * @remarks Hash: 0x1D97D1E3A70A649F
 */
export function setVehicleUseAlternateHandling(vehicle: number, toggle: number): void;

/**
 * ## Parameters
 * *
 * @param towTruck - 
 * @returns number
 * @remarks Hash: 0xEFEA18DCF10F8F75
 */
export function getEntityAttachedToTowTruck(towTruck: number): number;

/**
 * ## Parameters
 * *
 * @param vehicle - 
 * @returns void
 * @remarks Hash: 0xBD32E46AA95C1DD2
 */
export function SetBoatIsSinking(vehicle: number): void;

/**
 * ## Parameters
 * *
 * @param vehicle - 
 * @returns number
 * @remarks Hash: 0xD0E9CE05A1E68CD8
 */
export function detachVehicleFromAnyTowTruck(vehicle: number): number;

/**
 * ```
 * This has not yet been tested - it's just an assumption of what the types could be.  
 * ```
 * @param vehicle - 
 * @param state - 
 * @returns void
 * @remarks Hash: 0x3750146A28097A82
 */
export function setVehicleCanBeTargetted(vehicle: number, state: number): void;

/**
 * ## Parameters
 * *
 * @param vehicle - 
 * @param model - 
 * @returns number
 * @remarks Hash: 0x423E8DE37D934D89
 */
export function isVehicleModel(vehicle: number, model: number): number;

/**
 * ## Parameters
 * *
 * @param vehicle - 
 * @param p1 - 
 * @returns void
 * @remarks Hash: 0xA7DCDF4DED40A8F4
 */
export function 0xa7dcdf4ded40a8f4(vehicle: number, p1: number): void;

/**
 * ```
 * Does nothing. It's a nullsub.
 * 
 * NativeDB Introduced: v1604
 * ```
 * @param toggle - 
 * @returns void
 * @remarks Hash: 0x36DE109527A2C0C4
 */
export function 0x36de109527a2c0c4(toggle: number): void;

/**
 * ## Parameters
 * *
 * @param vehicle - 
 * @returns number
 * @remarks Hash: 0x48C633E94A8142A7
 */
export function IsVehicleSlipstreamLeader(vehicle: number): number;

/**
 * ```
 * tyreIndex = 0 to 4 on normal vehicles  
 * '0 = wheel_lf / bike, plane or jet front  
 * '1 = wheel_rf  
 * '2 = wheel_lm / in 6 wheels trailer, plane or jet is first one on left  
 * '3 = wheel_rm / in 6 wheels trailer, plane or jet is first one on right  
 * '4 = wheel_lr / bike rear / in 6 wheels trailer, plane or jet is last one on left  
 * '5 = wheel_rr / in 6 wheels trailer, plane or jet is last one on right  
 * '45 = 6 wheels trailer mid wheel left  
 * '47 = 6 wheels trailer mid wheel right  
 * ```
 * @param vehicle - 
 * @param tyreIndex - 
 * @returns void
 * @remarks Hash: 0x6E13FC662B882D1D
 */
export function setVehicleTyreFixed(vehicle: number, tyreIndex: number): void;

/**
 * ## Parameters
 * *
 * @param vehicle - 
 * @returns void
 * @remarks Hash: 0xF660602546D27BA8
 */
export function RaiseRetractableWheels(vehicle: number): void;

/**
 * Returns the convertible state of the specified vehicle.
 * 
 * 
 * 
 * ```c
 * enum eRoofState {
 *     RAISED = 0,
 *     LOWERING = 1,
 *     LOWERED = 2,
 *     RAISING = 3,
 *     CLOSING_BOOT = 4,
 *     ROOF_STUCK_RAISED = 5,
 *     ROOF_STUCK_LOWERED = 6
 * }
 * ```
 * @param vehicle - 
 * @returns number
 * @remarks Hash: 0xF8C397922FC03F41
 */
export function getConvertibleRoofState(vehicle: number): number;

/**
 * Checks if the vehicle is electric.
 * 
 * ```
 * NativeDB Introduced: v3258
 * ```
 * @param vehicleModel - 
 * @returns number
 * @remarks Hash: 0x1FCB07FE230B6639
 */
export function GetIsVehicleElectric(vehicleModel: number): number;

/**
 * ## Parameters
 * *
 * @param vehicle - 
 * @returns number
 * @remarks Hash: 0xB497F06B288DCFDF
 */
export function isVehicleStuckOnRoof(vehicle: number): number;

/**
 * Sets the distance from the player at which anchored boats switch between high and low LOD (Level of Detail) buoyancy mode.
 * 
 * ```
 * NativeDB Introduced: v323
 * ```
 * @param boat - 
 * @param value - 
 * @returns void
 * @remarks Hash: 0xE842A9398079BD82
 */
export function setBoatLowLodAnchorDistance(boat: number, value: number): void;

/**
 * ## Parameters
 * *
 * @param vehicle - 
 * @param x - 
 * @param y - 
 * @param z - 
 * @param p4 - 
 * @returns void
 * @remarks Hash: 0x5845066D8A1EA7F7
 */
export function 0x5845066d8a1ea7f7(vehicle: number, x: number, y: number, z: number, p4: any): void;

/**
 * Lowers the vehicle's stance. Only works for vehicles that support this feature.
 * 
 * ```
 * NativeDB Introduced: v2372
 * ```
 * @param vehicle - 
 * @param enable - 
 * @returns void
 * @remarks Hash: 0x3A375167F5782A65
 */
export function SetReduceDriftVehicleSuspension(vehicle: number, enable: number): void;

/**
 * Sets the dirt level of the passed vehicle.
 * @param vehicle - 
 * @param dirtLevel - 
 * @returns void
 * @remarks Hash: 0x79D3B596FE44EE8B
 */
export function setVehicleDirtLevel(vehicle: number, dirtLevel: number): void;

/**
 * Drops the Hook/Magnet on a cargobob  
 * 
 * ```c
 * enum eCargobobHook  
 * {  
 * 	CARGOBOB_HOOK = 0,  
 * 	CARGOBOB_MAGNET = 1,  
 * };  
 * ```
 * @param cargobob - 
 * @param state - 
 * @returns void
 * @remarks Hash: 0x7BEB0C7A235F6F3B
 */
export function createPickUpRopeForCargobob(cargobob: number, state: number): void;

/**
 * ```
 * how does this work?  
 * ```
 * @param disabled - 
 * @param weaponHash - 
 * @param vehicle - 
 * @param owner - 
 * @returns void
 * @remarks Hash: 0xF4FC6A6F67D8D856
 */
export function disableVehicleWeapon(disabled: number, weaponHash: number, vehicle: number, owner: number): void;

/**
 * ## Parameters
 * *
 * @param vehicle - 
 * @param p1 - 
 * @returns void
 * @remarks Hash: 0x56EB5E94318D3FB6
 */
export function 0x56eb5e94318d3fb6(vehicle: number, p1: number): void;

/**
 * Forces a submarine to maintain neutral buoyancy for a specified duration, preventing it from rising when unoccupied or without a driver.
 * 
 * ```
 * NativeDB Introduced: v2189
 * ```
 * @param submarine - 
 * @param time - 
 * @returns void
 * @remarks Hash: 0xC67DB108A9ADE3BE
 */
export function forceSubmarineNeurtalBuoyancy(submarine: number, time: number): void;

/**
 * ```
 * Returns the license plate text from a vehicle.  8 chars maximum.  
 * ```
 * @param vehicle - 
 * @returns string
 * @remarks Hash: 0x7CE1CCB9B293020E
 */
export function getVehicleNumberPlateText(vehicle: number): string;

/**
 * ```
 * Returns true only when the hook is active, will return false if the magnet is active  
 * ```
 * @param cargobob - 
 * @returns number
 * @remarks Hash: 0x1821D91AD4B56108
 */
export function doesCargobobHavePickUpRope(cargobob: number): number;

/**
 * ```
 * SET_VEHICLE_LI*
 * ```
 * @param vehicle - 
 * @param p1 - 
 * @returns void
 * @remarks Hash: 0xC50CE861B55EAB8B
 */
export function 0xc50ce861b55eab8b(vehicle: number, p1: number): void;

/**
 * Locks the doors of a specified vehicle to a defined lock state, affecting how players and NPCs can interact with the vehicle.
 * 
 * ```
 * NativeDB Introduced: v323
 * ```
 * 
 * ```c
 * enum eVehicleLockState {
 *     // No specific lock state, vehicle behaves according to the game's default settings.
 *     VEHICLELOCK_NONE = 0,
 *     // Vehicle is fully unlocked, allowing free entry by players and NPCs.
 *     VEHICLELOCK_UNLOCKED = 1,
 *     // Vehicle is locked, preventing entry by players and NPCs.
 *     VEHICLELOCK_LOCKED = 2,
 *     // Vehicle locks out only players, allowing NPCs to enter.
 *     VEHICLELOCK_LOCKOUT_PLAYER_ONLY = 3,
 *     // Vehicle is locked once a player enters, preventing others from entering.
 *     VEHICLELOCK_LOCKED_PLAYER_INSIDE = 4,
 *     // Vehicle starts in a locked state, but may be unlocked through game events.
 *     VEHICLELOCK_LOCKED_INITIALLY = 5,
 *     // Forces the vehicle's doors to shut and lock.
 *     VEHICLELOCK_FORCE_SHUT_DOORS = 6,
 *     // Vehicle is locked but can still be damaged.
 *     VEHICLELOCK_LOCKED_BUT_CAN_BE_DAMAGED = 7,
 *     // Vehicle is locked, but its trunk/boot remains unlocked.
 *     VEHICLELOCK_LOCKED_BUT_BOOT_UNLOCKED = 8,
 *     // Vehicle is locked and does not allow passengers, except for the driver.
 *     VEHICLELOCK_LOCKED_NO_PASSENGERS = 9,
 *     // Vehicle is completely locked, preventing entry entirely, even if previously inside.
 *     VEHICLELOCK_CANNOT_ENTER = 10 
 * };
 * 
 * ```
 * @param vehicle - 
 * @param doorLockStatus - 
 * @returns void
 * @remarks Hash: 0xB664292EAECF7FA6
 */
export function setVehicleDoorsLocked(vehicle: number, doorLockStatus: number): void;

/**
 * Disables turret movement when called in a loop. You can still fire and aim. You cannot shoot backwards though.
 * 
 * ```
 * NativeDB Introduced: v1365
 * ```
 * @param vehicle - 
 * @param turretIdx - 
 * @returns void
 * @remarks Hash: 0xE615BB7A7752C76A
 */
export function SetDisableTurretMovementThisFrame(vehicle: number, turretIdx: number): void;

/**
 * ```
 * colorIndex = 0 - 7
 * ```
 * @param vehicle - 
 * @param textureVariation - 
 * @returns void
 * @remarks Hash: 0xA74AD2439468C883
 */
export function SetVehicleParachuteTextureVariation(vehicle: number, textureVariation: number): void;

/**
 * ## Parameters
 * *
 * @param vehicle - 
 * @param toggle - 
 * @returns void
 * @remarks Hash: 0x92B35082E0B42F66
 */
export function setVehicleBrakeLights(vehicle: number, toggle: number): void;

/**
 * ## Parameters
 * *
 * @param model - 
 * @returns number
 * @remarks Hash: 0x39DAC362EE65FA28
 */
export function isThisModelAQuadbike(model: number): number;

/**
 * Gets the position of the cargobob hook, in world coords.
 * @param cargobob - 
 * @returns { x: number, y: number, z: number }
 * @remarks Hash: 0xCBDB9B923CACC92D
 */
export function GetCargobobHookPosition(cargobob: number): { x: number, y: number, z: number };

/**
 * ## Parameters
 * *
 * @param vehicle - 
 * @param entity - 
 * @param p2 - 
 * @returns void
 * @remarks Hash: 0x374706271354CB18
 */
export function stabiliseEntityAttachedToHeli(vehicle: number, entity: number, p2: number): void;

/**
 * ```
 * Check if Vehicle Secondary is avaliable for customize  
 * ```
 * @param vehicle - 
 * @returns number
 * @remarks Hash: 0x910A32E7AAD2656C
 */
export function getIsVehicleSecondaryColourCustom(vehicle: number): number;

/**
 * Check if a entry point for a certain seat is clear, useful for checking if a vehicle seat is accesible.
 * If you park your vehicle near a wall and the ped cannot enter/exit this side, the return value toggles from true (not blocked) to false (blocked).
 * 
 * Keep in mind, with checkSide set to true, that only certain vehicles have entry points on both sides for the same seat, like motorcycles, most normal vehicles don't have this and if the native doesn't find a entry point with the given parameters it will always return false. So for most normal usecases leaving checkSide set to false would result in the expected behavior.
 * @param ped - 
 * @param vehicle - 
 * @returns number
 * @remarks Hash: 0x639431E895B9AA57
 */
export function isEntryPointForSeatClear(ped: number, vehicle: number): number;

/**
 * ## Parameters
 * *
 * @param vehicle - 
 * @param seatIndex - 
 * @returns number
 * @remarks Hash: 0x83F969AA1EE2A664
 */
export function getLastPedInVehicleSeat(vehicle: number, seatIndex: number): number;

/**
 * ## Parameters
 * *
 * @param p0 - 
 * @param p1 - 
 * @returns void
 * @remarks Hash: 0xD565F438137F0E10
 */
export function 0xd565f438137f0e10(p0: any, p1: any): void;

/**
 * ## Parameters
 * *
 * @param vehicleAsset - 
 * @returns void
 * @remarks Hash: 0xACE699C71AB9DEB5
 */
export function removeVehicleAsset(vehicleAsset: number): void;

/**
 * ## Parameters
 * *
 * @param vehicle - 
 * @param toggle - 
 * @returns void
 * @remarks Hash: 0x91A0BD635321F145
 */
export function setDisableVehicleEngineFires(vehicle: number, toggle: number): void;

/**
 * See [`REQUEST_VEHICLE_RECORDING`](#_0xAF514CABE74CBF15).
 * @param recording - 
 * @param script - 
 * @returns number
 * @remarks Hash: 0x21543C612379DB3C
 */
export function getVehicleRecordingId(recording: number, script: string): number;

/**
 * See eDoorId declared in [`SET_VEHICLE_DOOR_SHUT`](#_0x93D9BD300D7789E5)
 * @param vehicle - 
 * @param doorIndex - 
 * @param isBreakable - 
 * @returns void
 * @remarks Hash: 0x2FA133A4A9D37ED8
 */
export function SetVehicleDoorCanBreak(vehicle: number, doorIndex: number, isBreakable: number): void;

/**
 * ```
 * NativeDB Introduced: v1493
 * ```
 * @param vehicle - 
 * @param color - 
 * @returns void
 * @remarks Hash: 0xB93B2867F7B479D1
 */
export function SetVehicleNeonLightsColor2(vehicle: number, color: number): void;

/**
 * ```
 * Returns false if every seat is occupied.  
 * ```
 * @param vehicle - 
 * @returns number
 * @remarks Hash: 0x2D34FC3BC4ADB780
 */
export function areAnyVehicleSeatsFree(vehicle: number): number;

/**
 * Lowers the roof on a convertible vehicle, utilizing any available animations for the action. This native is particularly useful for creating a realistic interaction with convertible vehicles by animating the process of lowering the roof.
 * 
 * You can check if the vehicle has an convertible roof using [`IS_VEHICLE_A_CONVERTIBLE`](#_0x52F357A30698BCCE).
 * 
 * ```
 * NativeDB Introduced: v323
 * ```
 * @param vehicle - 
 * @param instantlyLower - 
 * @returns void
 * @remarks Hash: 0xDED51F703D0FA83D
 */
export function lowerConvertibleRoof(vehicle: number, instantlyLower: number): void;

/**
 * This native is used to simulate a high-speed impact for a vehicle when it collides with a breakable object (frag). It's particularly useful in scripted sequences where a vehicle is required to break through a barrier but might not actually be moving at a sufficient speed to do so realistically. Note that this setting is temporary and will reset after one frame, so it needs to be called every frame for a lasting effect.
 * @param vehicle - 
 * @param actHighSpeed - 
 * @returns void
 * @remarks Hash: 0x1F9FB66F3A3842D2
 */
export function setVehicleActAsIfHighSpeedForFragSmashing(vehicle: number, actHighSpeed: number): void;

/**
 * ## Parameters
 * *
 * @param vehicle - 
 * @param paintType - 
 * @param color - 
 * @param pearlescentColor - 
 * @returns void
 * @remarks Hash: 0xE8D65CA700C9A693
 */
export function getVehicleModColor1(vehicle: number, paintType: number, color: number, pearlescentColor: number): void;

/**
 * ```
 * p1, p2, p3 are RGB values for color (255,0,0 for Red, ect)  
 * ```
 * @param vehicle - 
 * @param r - 
 * @param g - 
 * @param b - 
 * @returns void
 * @remarks Hash: 0x7141766F91D15BEA
 */
export function setVehicleCustomPrimaryColour(vehicle: number, r: number, g: number, b: number): void;

/**
 * Sets the specified door index shut on the passed vehicle.
 * 
 * ```c
 * enum eDoorId
 * {
 * 	VEH_EXT_DOOR_DSIDE_F = 0,
 * 	VEH_EXT_DOOR_DSIDE_R = 1,
 * 	VEH_EXT_DOOR_PSIDE_F = 2,
 * 	VEH_EXT_DOOR_PSIDE_R = 3,
 * 	VEH_EXT_BONNET = 4,
 * 	VEH_EXT_BOOT = 5,
 * 	// 0x872E72B8 = 0xFFFFFFFF,
 * }
 * ```
 * @param vehicle - 
 * @param doorIndex - 
 * @param closeInstantly - 
 * @returns void
 * @remarks Hash: 0x93D9BD300D7789E5
 */
export function setVehicleDoorShut(vehicle: number, doorIndex: number, closeInstantly: number): void;

/**
 * ## Parameters
 * *
 * @param vehicle - 
 * @param team - 
 * @param toggle - 
 * @returns void
 * @remarks Hash: 0xB81F6D4A8F5EEBA8
 */
export function setVehicleDoorsLockedForTeam(vehicle: number, team: number, toggle: number): void;

/**
 * ## Parameters
 * *
 * @param cargobob - 
 * @param p1 - 
 * @returns void
 * @remarks Hash: 0x6D8EAC07506291FB
 */
export function setCargobobPickupMagnetPullRopeLength(cargobob: number, p1: number): void;

/**
 * ## Parameters
 * *
 * @param p0 - 
 * @param p1 - 
 * @returns void
 * @remarks Hash: 0xAF60E6A2936F982A
 */
export function 0xaf60e6a2936f982a(p0: any, p1: any): void;

/**
 * ## Parameters
 * *
 * @param model - 
 * @returns number
 * @remarks Hash: 0xAB935175B22E822B
 */
export function isThisModelATrain(model: number): number;

/**
 * Used to set the tornado custom (convertible) rooftop livery.
 * 
 * Livery value that works for tornado custom is between 0 and 9 from what i can tell. Maybe 0-8 even.
 * 
 * Might work on other custom vehicles but im not sure what those might be, only confirmed it working with the tornado custom.
 * @param vehicle - 
 * @param livery - 
 * @returns void
 * @remarks Hash: 0xA6D3A8750DC73270
 */
export function SetVehicleRoofLivery(vehicle: number, livery: number): void;

/**
 * This native makes the vehicle stop immediately, as it happens when we enter a multiplayer garage.
 * @param vehicle - 
 * @param distance - 
 * @param duration - 
 * @param bControlVerticalVelocity - 
 * @returns void
 * @remarks Hash: 0x260BE8F09E326A20
 */
export function bringVehicleToHalt(vehicle: number, distance: number, duration: number, bControlVerticalVelocity: number): void;

/**
 * ```
 * NativeDB Introduced: v1493
 * ```
 * @param p0 - 
 * @param p1 - 
 * @returns void
 * @remarks Hash: 0x407DC5E97DB1A4D3
 */
export function 0x407dc5e97db1a4d3(p0: any, p1: any): void;

/**
 * ## Parameters
 * *
 * @param model - 
 * @returns number
 * @remarks Hash: 0xBF94DD42F63BDED2
 */
export function isThisModelABicycle(model: number): number;

/**
 * ```
 * NativeDB Introduced: v2060
 * ```
 * @param vehicle - 
 * @param p1 - 
 * @returns void
 * @remarks Hash: 0xF8B49F5BA7F850E7
 */
export function 0xf8b49f5ba7f850e7(vehicle: number, p1: number): void;

/**
 * ```
 * flags requires further research, e.g., 0x4/0x8 are related to the AI driving task and 0x20 is internally set and interacts with dynamic entity components.
 * time, often zero and capped at 500, is related to SET_PLAYBACK_TO_USE_AI_TRY_TO_REVERT_BACK_LATER
 * ```
 * @param vehicle - 
 * @param recording - 
 * @param script - 
 * @param flags - 
 * @param time - 
 * @param drivingStyle - 
 * @returns void
 * @remarks Hash: 0x7D80FD645D4DA346
 */
export function startPlaybackRecordedVehicleWithFlags(vehicle: number, recording: number, script: string, flags: number, time: number, drivingStyle: number): void;

/**
 * See eDoorId declared in [`SET_VEHICLE_DOOR_SHUT`](#_0x93D9BD300D7789E5)
 * @param vehicle - 
 * @param doorIndex - 
 * @returns number
 * @remarks Hash: 0xCA4AC3EAAE46EC7B
 */
export function getVehicleIndividualDoorLockStatus(vehicle: number, doorIndex: number): number;

/**
 * ## Parameters
 * *
 * @param vehicle - 
 * @returns void
 * @remarks Hash: 0x10655FAB9915623D
 */
export function SetVehicleHandlingHashForAi(vehicle: number): void;

/**
 * Sets the color of the neon lights on the specified vehicle.
 * 
 * RGB values and colour names taken from the decompiled scripts:
 * 
 * | Colour         |  R  |  G  |  B  |
 * |---------------|:---:|:---:|:---:|
 * | White         | 222 | 222 | 255 |
 * | Blue          | 2   | 21  | 255 |
 * | Electric Blue | 3   | 83  | 255 |
 * | Mint Green    | 0   | 255 | 140 |
 * | Lime Green    | 94  | 255 | 1   |
 * | Yellow        | 255 | 255 | 0   |
 * | Golden Shower | 255 | 150 | 0   |
 * | Orange        | 255 | 62  | 0   |
 * | Red           | 255 | 1   | 1   |
 * | Pony Pink     | 255 | 50  | 100 |
 * | Hot Pink      | 255 | 5   | 190 |
 * | Purple        | 35  | 1   | 255 |
 * | Blacklight    | 15  | 3   | 255 |
 * @param vehicle - 
 * @param r - 
 * @param g - 
 * @param b - 
 * @returns void
 * @remarks Hash: 0x8E0A582209A62695
 */
export function SetVehicleNeonLightsColour(vehicle: number, r: number, g: number, b: number): void;

/**
 * ```
 * NativeDB Introduced: v3407
 * ```
 * @param plane - 
 * @param toggle - 
 * @returns void
 * @remarks Hash: 0xDD8A2D3337F04196
 */
export function setPlaneControlSectionsShouldBreakOffFromExplosions(plane: number, toggle: number): void;

/**
 * Sets whether a boat should remain anchored even when a player is driving it.
 * @param boat - 
 * @param toggle - 
 * @returns void
 * @remarks Hash: 0xE3EBAAE484798530
 */
export function setBoatRemainsAnchoredWhilePlayerIsDriver(boat: number, toggle: number): void;

/**
 * ```
 * GET_H*
 * 
 * NativeDB Introduced: v1604
 * ```
 * @param vehicle - 
 * @returns number
 * @remarks Hash: 0xE8718FAF591FD224
 */
export function 0xe8718faf591fd224(vehicle: number): number;

/**
 * ## Parameters
 * *
 * @param handler - 
 * @param container - 
 * @returns void
 * @remarks Hash: 0x6A98C2ECF57FA5D4
 */
export function AttachContainerToHandlerFrame(handler: number, container: number): void;

/**
 * ```
 * SCALE: Setting the speed to 30 would result in a speed of roughly 60mph, according to speedometer.  
 * Speed is in meters per second  
 * You can convert meters/s to mph here:  
 * http://www.calculateme.com/Speed/MetersperSecond/ToMilesperHour.htm  
 * ```
 * @param vehicle - 
 * @param speed - 
 * @returns void
 * @remarks Hash: 0xAB54A438726D25D5
 */
export function setVehicleForwardSpeed(vehicle: number, speed: number): void;

/**
 * ```
 * Sets vehicle wheel hydraulic states transition. Known states:
 * 0 - reset
 * 1 - raise wheel (uses value arg, works just like _SET_VEHICLE_HYDRAULIC_WHEEL_VALUE)
 * 2 - jump using wheel
 * ```
 * @param vehicle - 
 * @param wheelId - 
 * @param state - 
 * @param value - 
 * @param p4 - 
 * @returns void
 * @remarks Hash: 0xC24075310A8B9CD1
 */
export function SetHydraulicWheelStateTransition(vehicle: number, wheelId: number, state: number, value: number, p4: number): void;

/**
 * ## Parameters
 * *
 * @param plane - 
 * @returns number
 * @remarks Hash: 0x755D6D5267CBBD7E
 */
export function arePlanePropellersIntact(plane: number): number;

/**
 * This native is used to latch or unlatch the convertible roof of a vehicle. It allows for direct control over the roof's mechanism without actually opening or closing the roof. This can be useful for scenarios where you need to prepare a vehicle's roof to be opened or closed by another action or to ensure it remains fixed in its current state regardless of other interactions.
 * 
 * ```
 * NativeDB Introduced: v323
 * ```
 * @param vehicle - 
 * @param bLatched - 
 * @returns void
 * @remarks Hash: 0x1A78AD3D8240536F
 */
export function setConvertibleRoofLatchState(vehicle: number, bLatched: number): void;

/**
 * ```
 * NativeDB Introduced: v1868
 * ```
 * @param vehicle - 
 * @param wheelIndex - 
 * @param health - 
 * @returns void
 * @remarks Hash: 0x74C68EF97645E79D
 */
export function SetTyreHealth(vehicle: number, wheelIndex: number, health: number): void;

/**
 * ## Parameters
 * *
 * @param vehicle - 
 * @param colorPrimary - 
 * @param colorSecondary - 
 * @returns void
 * @remarks Hash: 0xA19435F193E081AC
 */
export function getVehicleColours(vehicle: number, colorPrimary: number, colorSecondary: number): void;

/**
 * ```
 * NativeDB Introduced: v3407
 * ```
 * @param vehicle - 
 * @returns number
 * @remarks Hash: 0xFC40CBF7B90CA77C
 */
export function IsVehicleOnBoostPad(vehicle: number): number;

/**
 * ## Parameters
 * *
 * @param vehicle - 
 * @returns number
 * @remarks Hash: 0x36D782F68B309BDA
 */
export function GetHasRocketBoost(vehicle: number): number;

/**
 * ```
 * Time is number of milliseconds before reverting, zero for indefinitely.
 * ```
 * @param vehicle - 
 * @param time - 
 * @param drivingStyle - 
 * @param p3 - 
 * @returns void
 * @remarks Hash: 0x6E63860BBB190730
 */
export function setPlaybackToUseAiTryToRevertBackLater(vehicle: number, time: number, drivingStyle: number, p3: number): void;

/**
 * Determines if the submarine is operating below its designated crush depth.
 * 
 * ```
 * NativeDB Introduced: v2189
 * ```
 * @param submarine - 
 * @returns number
 * @remarks Hash: 0x3E71D0B300B7AA79
 */
export function getSubmarineIsUnderDesignDepth(submarine: number): number;

/**
 * ```
 * Note: only some vehicle have extras  
 * extra ids are from 1 - 9 depending on the vehicle
 * @param vehicle - 
 * @param extraId - 
 * @param disable - 
 * @returns void
 * @remarks Hash: 0x7EE3A3C5E4A40CC9
 */
export function setVehicleExtra(vehicle: number, extraId: number, disable: number): void;

/**
 * ## Parameters
 * *
 * @param vehicle - 
 * @param p1 - 
 * @returns void
 * @remarks Hash: 0x2A8F319B392E7B3F
 */
export function setTrailerInverseMassScale(vehicle: number, p1: number): void;

/**
 * Incorrectly named `SET_VEHICLE_EXCLUSIVE_DRIVER`; likely `SET_VEHICLE_ALLOW_*`.
 * 
 * Toggles a flag related to `SET_VEHICLE_EXCLUSIVE_DRIVER`, however, doesn't enable that feature (or trigger script events related to it).
 * 
 * See [`_SET_VEHICLE_EXCLUSIVE_DRIVER_2`](#_0xB5C51B5502E85E83).
 * 
 * ```
 * NativeDB Removed Parameter 2: int index
 * ```
 * @param vehicle - 
 * @returns void
 * @remarks Hash: 0x41062318F23ED854
 */
export function setVehicleExclusiveDriver(vehicle: number): void;

/**
 * ```
 * Toggles specific flag on turret
 * ```
 * 
 * ```
 * NativeDB Introduced: v1290
 * ```
 * @param vehicle - 
 * @param index - 
 * @param toggle - 
 * @returns void
 * @remarks Hash: 0xC60060EB0D8AC7B1
 */
export function SetVehicleTurretUnk(vehicle: number, index: number, toggle: number): void;

/**
 * ## Parameters
 * *
 * @param p0 - 
 * @returns number
 * @remarks Hash: 0x06F43E5175EB6D96
 */
export function hasPreloadModsFinished(p0: any): number;

/**
 * ## Parameters
 * *
 * @param vehicle - 
 * @param toggle - 
 * @returns void
 * @remarks Hash: 0x737E398138550FFF
 */
export function 0x737e398138550fff(vehicle: number, toggle: number): void;

/**
 * Similar to [`_GET_AIRCRAFT_BOMB_COUNT`](#_0xEA12BD130D7569A1), this gets the amount of countermeasures that are present on this vehicle.
 * 
 * Use [`_SET_AIRCRAFT_COUNTERMEASURE_COUNT`](#_0x9BDA23BF666F0855) to set the current amount.
 * @param aircraft - 
 * @returns number
 * @remarks Hash: 0xF846AA63DF56B804
 */
export function GetVehicleCountermeasureCount(aircraft: number): number;

/**
 * @returns void
 * @remarks Hash: 0xF25E02CB9C5818F8
 */
export function 0xf25e02cb9c5818f8(): void;

/**
 * ```
 * if true, axles won't bend.  
 * ```
 * @param vehicle - 
 * @param toggle - 
 * @returns void
 * @remarks Hash: 0x92F0CF722BC4202F
 */
export function setVehicleHasStrongAxles(vehicle: number, toggle: number): void;

/**
 * ```
 * NativeDB Introduced: v1180
 * ```
 * @param p0 - 
 * @returns void
 * @remarks Hash: 0x430A7631A84C9BE7
 */
export function 0x430a7631a84c9be7(p0: any): void;

/**
 * @returns void
 * @remarks Hash: 0x34AD89078831A4BC
 */
export function setAllVehicleGeneratorsActive(): void;

/**
 * ```
 * The only example I can find of this function in the scripts, is this:  
 * struct _s = VEHICLE::GET_VEHICLE_DEFORMATION_AT_POS(rPtr((A_0) + 4), 1.21f, 6.15f, 0.3f);
 * @param vehicle - 
 * @param offsetX - 
 * @param offsetY - 
 * @param offsetZ - 
 * @returns { x: number, y: number, z: number }
 * @remarks Hash: 0x4EC6CFBC7B2E9536
 */
export function getVehicleDeformationAtPos(vehicle: number, offsetX: number, offsetY: number, offsetZ: number): { x: number, y: number, z: number };

/**
 * @returns void
 * @remarks Hash: 0xEF49CF0270307CBE
 */
export function detonateVehiclePhoneExplosiveDevice(): void;

/**
 * ```
 * NativeDB Introduced: v1604
 * ```
 * @param vehicle - 
 * @param toggle - 
 * @returns void
 * @remarks Hash: 0x9849DE24FCF23CCC
 */
export function 0x9849de24fcf23ccc(vehicle: number, toggle: number): void;

/**
 * ```
 * returns a string which is the codename of the vehicle's currently selected secondary color  
 * ```
 * @param vehicle - 
 * @returns string
 * @remarks Hash: 0x4967A516ED23A5A1
 */
export function getVehicleModColor2Name(vehicle: number): string;

/**
 * Returns whether the outrigger legs are deployed for the vehicle.
 * The Chernobog is one of the few vehicles with outrigger legs.
 * 
 * ```
 * NativeDB Introduced: v1290
 * ```
 * @param vehicle - 
 * @returns number
 * @remarks Hash: 0x3A9128352EAC9E85
 */
export function AreOutriggerLegsDeployed(vehicle: number): number;

/**
 * ```
 * NativeDB Introduced: 3095
 * ```
 * 
 * Determines if the nitrous is currently activated in the specified vehicle.
 * @param vehicle - 
 * @returns number
 * @remarks Hash: 0x491E822B2C464FE4
 */
export function isNitrousActive(vehicle: number): number;

/**
 * ```
 * Sets the neon lights of the specified vehicle on/off.  
 * Indices:  
 * 0 = Left  
 * 1 = Right  
 * 2 = Front  
 * 3 = Back  
 * ```
 * @param vehicle - 
 * @param index - 
 * @param toggle - 
 * @returns void
 * @remarks Hash: 0x2AA720E4287BF269
 */
export function SetVehicleNeonLightEnabled(vehicle: number, index: number, toggle: number): void;

/**
 * ## Parameters
 * *
 * @param p0 - 
 * @param p1 - 
 * @returns void
 * @remarks Hash: 0x065D03A9D6B2C6B5
 */
export function 0x065d03a9d6b2c6b5(p0: any, p1: any): void;

/**
 * ```
 * Returns true only when the magnet is active, will return false if the hook is active  
 * ```
 * @param cargobob - 
 * @returns number
 * @remarks Hash: 0x6E08BF5B3722BAC9
 */
export function doesCargobobHavePickupMagnet(cargobob: number): number;

/**
 * ```
 * Scripts verify that towTruck is the first parameter, not the second.  
 * ```
 * @param towTruck - 
 * @param vehicle - 
 * @returns number
 * @remarks Hash: 0x146DF9EC4C4B9FD4
 */
export function isVehicleAttachedToTowTruck(towTruck: number, vehicle: number): number;

/**
 * ## Parameters
 * *
 * @param vehicle - 
 * @returns void
 * @remarks Hash: 0xBCDF8BAF56C87B6A
 */
export function setPlayersLastVehicle(vehicle: number): void;

/**
 * ```
 * The inner function has a switch on the second parameter. It's the stuck timer index.  
 * Here's some pseudo code I wrote for the inner function:  
 * void __fastcall NATIVE_RESET_VEHICLE_STUCK_TIMER_INNER(CUnknown* unknownClassInVehicle, int timerIndex)  
 * {  
 * 	switch (timerIndex)  
 * 	{  
 * 	case 0:  
 * unknownClassInVehicle->FirstStuckTimer = (WORD)0u;  
 * 	case 1:  
 * unknownClassInVehicle->SecondStuckTimer = (WORD)0u;  
 * 	case 2:  
 * unknownClassInVehicle->ThirdStuckTimer = (WORD)0u;  
 * 	case 3:  
 * unknownClassInVehicle->FourthStuckTimer = (WORD)0u;  
 * 	case 4:  
 * unknownClassInVehicle->FirstStuckTimer = (WORD)0u;  
 * unknownClassInVehicle->SecondStuckTimer = (WORD)0u;  
 * unknownClassInVehicle->ThirdStuckTimer = (WORD)0u;  
 * unknownClassInVehicle->FourthStuckTimer = (WORD)0u;  
 * break;  
 * 	};  
 * }  
 * ```
 * @param vehicle - 
 * @param nullAttributes - 
 * @returns void
 * @remarks Hash: 0xD7591B0065AFAA7A
 */
export function resetVehicleStuckTimer(vehicle: number, nullAttributes: number): void;

/**
 * ## Parameters
 * *
 * @param p0 - 
 * @returns void
 * @remarks Hash: 0xF051D9BFB6BA39C0
 */
export function 0xf051d9bfb6ba39c0(p0: any): void;

/**
 * ## Parameters
 * *
 * @param vehicle - 
 * @param trailer - 
 * @param radius - 
 * @returns void
 * @remarks Hash: 0x3C7D42D58F770B54
 */
export function attachVehicleToTrailer(vehicle: number, trailer: number, radius: number): void;

/**
 * Sets whether the trailer can attach to vehicles
 * @param vehicle - 
 * @returns void
 * @remarks Hash: 0x2FA2494B47FDD009
 */
export function setTrailerAttachmentEnabled(vehicle: number): void;

/**
 * ## Parameters
 * *
 * @param p0 - 
 * @param p1 - 
 * @param p2 - 
 * @returns void
 * @remarks Hash: 0x66E3AAFACE2D1EB8
 */
export function 0x66e3aaface2d1eb8(p0: any, p1: any, p2: any): void;

/**
 * This native sets the turbulence multiplier. It only works for planes.
 * 0.0 = no turbulence at all.
 * 1.0 = heavy turbulence.
 * 
 * Works by just calling it once, does not need to be called every tick.
 * @param vehicle - 
 * @param multiplier - 
 * @returns void
 * @remarks Hash: 0xAD2D28A1AFDFF131
 */
export function setPlaneTurbulenceMultiplier(vehicle: number, multiplier: number): void;

/**
 * Changes the key used to transform a vehicle into submarine mode. When set to true, the transformation key switches from the default raise/lower convertible roof key (usually 'H') to the special vehicle transformation key (usually 'X').
 * 
 * ```
 * NativeDB Introduced: v1365
 * ```
 * @param vehicle - 
 * @param useAlternateInput - 
 * @returns void
 * @remarks Hash: 0x41B9FB92EDED32A6
 */
export function setTransformToSubmarineUsesAlternateInput(vehicle: number, useAlternateInput: boolean): void;

/**
 * ## Parameters
 * *
 * @param vehicle - 
 * @returns number
 * @remarks Hash: 0x9A83F5F9963775EF
 */
export function haveVehicleModsStreamedIn(vehicle: number): number;

/**
 * ## Parameters
 * *
 * @param vehicle - 
 * @returns void
 * @remarks Hash: 0x7C0043FDFF6436BC
 */
export function detachContainerFromHandlerFrame(vehicle: number): void;

/**
 * ## Parameters
 * *
 * @param vehicle - 
 * @param modType - 
 * @returns void
 * @remarks Hash: 0x92D619E420858204
 */
export function removeVehicleMod(vehicle: number, modType: number): void;

/**
 * ```
 * Outputs 2 Vector3's.
 * Scripts check if out2.x - out1.x > something.x
 * Could be suspension related, as in max suspension height and min suspension height, considering the natives location.
 * ```
 * @param vehicle - 
 * @param out1 - 
 * @param out2 - 
 * @returns void
 * @remarks Hash: 0xDF7E3EEB29642C38
 */
export function GetVehicleSuspensionBounds(vehicle: number, out1: { x: number, y: number, z: number }, out2: { x: number, y: number, z: number }): void;

/**
 * See eWindowId declared in [`IS_VEHICLE_WINDOW_INTACT`](#_0x46E571A0E20D01F1).
 * @param vehicle - 
 * @param windowIndex - 
 * @returns void
 * @remarks Hash: 0x7AD9E6CE657D69E3
 */
export function rollDownWindow(vehicle: number, windowIndex: number): void;

/**
 * ## Parameters
 * *
 * @param vehicle - 
 * @param toggle - 
 * @returns void
 * @remarks Hash: 0x1BBAC99C0BC53656
 */
export function SetVehicleRampSidewaysLaunchMotion(vehicle: number, toggle: number): void;

/**
 * ## Parameters
 * *
 * @param vehicle - 
 * @param toggle - 
 * @returns void
 * @remarks Hash: 0x51BB2D88D31A914B
 */
export function setVehicleCanLeakOil(vehicle: number, toggle: number): void;

/**
 * ```
 * formerly known as _GET_VEHICLE_PAINT_FADE
 * The result is a value from 0-1, where 0 is fresh paint.
 * ```
 * @param vehicle - 
 * @returns number
 * @remarks Hash: 0xA82819CAC9C4C403
 */
export function getVehicleEnveffScale(vehicle: number): number;

/**
 * ```
 * NativeDB Introduced: v2189
 * ```
 * @param p0 - 
 * @param p1 - 
 * @returns void
 * @remarks Hash: 0x8664170EF165C4A6
 */
export function 0x8664170ef165c4a6(p0: any, p1: any): void;

/**
 * ## Parameters
 * *
 * @param vehicle - 
 * @param p1 - 
 * @param p2 - 
 * @returns void
 * @remarks Hash: 0x23428FC53C60919C
 */
export function disablePlaneAileron(vehicle: number, p1: number, p2: number): void;

/**
 * ```
 * NativeDB Introduced: v2372
 * ```
 * @param p0 - 
 * @returns void
 * @remarks Hash: 0x2310A8F9421EBF43
 */
export function 0x2310a8f9421ebf43(p0: any): void;

/**
 * ```
 * NativeDB Introduced: v3407
 * ```
 * @param vehicle - 
 * @param scale - 
 * @returns any
 * @remarks Hash: 0x84D7FFD223CAAFFD
 */
export function SetVehicleExplosiveDamageScale(vehicle: number, scale: number): any;

/**
 * ## Parameters
 * *
 * @param vehicle - 
 * @param toggle - 
 * @returns void
 * @remarks Hash: 0x59BF8C3D52C92F66
 */
export function setVehicleCanBreak(vehicle: number, toggle: number): void;

/**
 * ```
 * Stops the cargobob from being able to attach any vehicle
 * ```
 * 
 * ```
 * NativeDB Introduced: v1180
 * ```
 * @param vehicle - 
 * @param toggle - 
 * @returns void
 * @remarks Hash: 0x94A68DA412C4007D
 */
export function SetCargobobHookCanAttach(vehicle: number, toggle: number): void;

/**
 * ## Parameters
 * *
 * @param vehicle - 
 * @returns void
 * @remarks Hash: 0xB72E26D81006005B
 */
export function addVehicleUpsidedownCheck(vehicle: number): void;

/**
 * ## Parameters
 * *
 * @param vehicle - 
 * @returns number
 * @remarks Hash: 0xB5CC40FBCB586380
 */
export function isVehicleSirenAudioOn(vehicle: number): number;

/**
 * ## Parameters
 * *
 * @param vehicle - 
 * @param toggle - 
 * @returns void
 * @remarks Hash: 0x33506883545AC0DF
 */
export function forceSubmarineSurfaceMode(vehicle: number, toggle: number): void;

/**
 * ## Parameters
 * *
 * @param multiplier - 
 * @returns void
 * @remarks Hash: 0xB3B3359379FE77D3
 */
export function setRandomVehicleDensityMultiplierThisFrame(multiplier: number): void;

/**
 * Vehicle must be a plane.
 * Native name is between SET_VEHICLE_BRAKE_LIGHTS and SET_VEHICLE_BULLDOZER_ARM_POSITION alphabetically.
 * @param vehicle - 
 * @param p1 - 
 * @returns void
 * @remarks Hash: 0xC361AA040D6637A8
 */
export function 0xc361aa040d6637a8(vehicle: number, p1: number): void;

/**
 * ## Parameters
 * *
 * @param vehicle - 
 * @param enabled - 
 * @returns void
 * @remarks Hash: 0x29B18B4FD460CA8F
 */
export function setVehicleWheelsCanBreak(vehicle: number, enabled: number): void;

/**
 * ## Parameters
 * *
 * @param vehicle - 
 * @param modType - 
 * @param modIndex - 
 * @returns number
 * @remarks Hash: 0x00834EAC4A96E010
 */
export function isVehicleModGen9Exclusive(vehicle: number, modType: number, modIndex: number): number;

/**
 * ```
 * NativeDB Introduced: v1290
 * ```
 * @param vehicle - 
 * @param toggle - 
 * @returns void
 * @remarks Hash: 0x8821196D91FA2DE5
 */
export function 0x8821196d91fa2de5(vehicle: number, toggle: number): void;

/**
 * ```
 * SET_VEHICLE_*
 * ```
 * @param vehicle - 
 * @returns void
 * @remarks Hash: 0x76D26A22750E849E
 */
export function 0x76d26a22750e849e(vehicle: number): void;

/**
 * ## Parameters
 * *
 * @param vehicle - 
 * @param entity - 
 * @param p2 - 
 * @param x - 
 * @param y - 
 * @param z - 
 * @returns void
 * @remarks Hash: 0xA1DD82F3CCF9A01E
 */
export function attachEntityToCargobob(vehicle: number, entity: number, p2: number, x: number, y: number, z: number): void;

/**
 * ```
 * Not present in the retail version! It's just a nullsub.  
 * p0 always true (except in one case)  
 * p1 a random vehicle hash loaded in memory  
 * successIndicator: 0 if success, -1 if failed
 * ```
 * @param p0 - 
 * @param modelHash - 
 * @param successIndicator - 
 * @returns void
 * @remarks Hash: 0x055BF0AC0C34F4FD
 */
export function getRandomVehicleModelInMemory(p0: number, modelHash: number, successIndicator: number): void;

/**
 * Paint index goes from 0 to 12.
 * 
 * You can find the list of colors and ids here: [_GET_VEHICLE_HEADLIGHTS_COLOUR](#_0x3DFF319A831E0CDB)
 * @param vehicle - 
 * @param color - 
 * @returns void
 * @remarks Hash: 0xE41033B25D003A07
 */
export function SetVehicleXenonLightsColor(vehicle: number, color: number): void;

/**
 * ## Parameters
 * *
 * @param active - 
 * @returns void
 * @remarks Hash: 0x608207E7A8FB787C
 */
export function setAllLowPriorityVehicleGeneratorsActive(active: number): void;

/**
 * ```
 * Request the vehicle recording defined by the lowercase format string "%s%03d.yvr". For example, REQUEST_VEHICLE_RECORDING(1, "FBIs1UBER") corresponds to fbis1uber001.yvr.
 * For all vehicle recording/playback natives, "script" is a common prefix that usually corresponds to the script/mission the recording is used in, "recording" is its int suffix, and "id" (e.g., in native GET_TOTAL_DURATION_OF_VEHICLE_RECORDING_ID) corresponds to a unique identifier within the recording streaming module.
 * Note that only 24 recordings (hardcoded in multiple places) can ever active at a given time before clobbering begins.
 * ```
 * @param recording - 
 * @param script - 
 * @returns void
 * @remarks Hash: 0xAF514CABE74CBF15
 */
export function requestVehicleRecording(recording: number, script: string): void;

/**
 * This method is utilized solely for debugging purposes and is functional only in debug builds of the game. Please note that its functionality may not be available in the retail version.
 * @param vehicle - 
 * @param name - 
 * @returns void
 * @remarks Hash: 0xBFDF984E2C22B94F
 */
export function setVehicleNameDebug(vehicle: number, name: string): void;

/**
 * ```
 * NativeDB Introduced: v1604
 * ```
 * @param toggle - 
 * @returns void
 * @remarks Hash: 0x143921E45EC44D62
 */
export function SetDisableVehicleUnk(toggle: number): void;

/**
 * ```
 * Possibly: Returns whether the searchlight (found on police vehicles) is toggled on.  
 * ```
 * @param vehicle - 
 * @returns number
 * @remarks Hash: 0xC0F97FCE55094987
 */
export function isVehicleSearchlightOn(vehicle: number): number;

/**
 * ```
 * They use the same color indexs as SET_VEHICLE_COLOURS.  
 * ```
 * @param vehicle - 
 * @param pearlescentColor - 
 * @param wheelColor - 
 * @returns void
 * @remarks Hash: 0x2036F561ADD12E33
 */
export function setVehicleExtraColours(vehicle: number, pearlescentColor: number, wheelColor: number): void;

/**
 * ## Parameters
 * *
 * @param p0 - 
 * @param p1 - 
 * @param p2 - 
 * @param p3 - 
 * @param p4 - 
 * @param p5 - 
 * @param p6 - 
 * @returns any
 * @remarks Hash: 0x54B0F614960F4A5F
 */
export function addVehicleCombatAngledAvoidanceArea(p0: number, p1: number, p2: number, p3: number, p4: number, p5: number, p6: number): any;

/**
 * Train models must be [requested](#_0x963D27A58DF860AC) before use. See trains.xml (located in `Grand Theft Auto V\update\update.rpf\common\data\levels\gta5\trains.xml`) for freight and metro variations.
 * 
 * Model names to request can be found by searching `model_name` in the file.
 * 
 * The `Lua` usage example provided down below has been provided in such way so users can test each and every train variation.
 * @param variation - 
 * @param x - 
 * @param y - 
 * @param z - 
 * @param direction - 
 * @returns number
 * @remarks Hash: 0x63C6CCA8E68AE8C8
 */
export function createMissionTrain(variation: number, x: number, y: number, z: number, direction: number): number;

/**
 * ```
 * Creates a script vehicle generator at the given coordinates. Most parameters after the model hash are unknown.  
 * Parameters:  
 * a/w/s - Generator position  
 * heading - Generator heading  
 * p4 - Unknown (always 5.0)  
 * p5 - Unknown (always 3.0)  
 * modelHash - Vehicle model hash  
 * p7/8/9/10 - Unknown (always -1)  
 * p11 - Unknown (usually TRUE, only one instance of FALSE)  
 * p12/13 - Unknown (always FALSE)  
 * p14 - Unknown (usally FALSE, only two instances of TRUE)  
 * p15 - Unknown (always TRUE)  
 * p16 - Unknown (always -1)  
 * Vector3 coords = GET_ENTITY_COORDS(PLAYER_PED_ID(), 0);	CREATE_SCRIPT_VEHICLE_GENERATOR(coords.x, coords.y, coords.z, 1.0f, 5.0f, 3.0f, GET_HASH_KEY("adder"), -1. -1, -1, -1, -1, true, false, false, false, true, -1);  
 * ```
 * @param x - 
 * @param y - 
 * @param z - 
 * @param heading - 
 * @param p4 - 
 * @param p5 - 
 * @param modelHash - 
 * @param p7 - 
 * @param p8 - 
 * @param p9 - 
 * @param p10 - 
 * @param p11 - 
 * @param p12 - 
 * @param p13 - 
 * @param p14 - 
 * @param p15 - 
 * @param p16 - 
 * @returns number
 * @remarks Hash: 0x9DEF883114668116
 */
export function createScriptVehicleGenerator(x: number, y: number, z: number, heading: number, p4: number, p5: number, modelHash: number, p7: number, p8: number, p9: number, p10: number, p11: number, p12: number, p13: number, p14: number, p15: number, p16: number): number;

/**
 * ## Parameters
 * *
 * @param vehicle - 
 * @returns number
 * @remarks Hash: 0x3D34E80EED4AE3BE
 */
export function IsVehicleRocketBoostActive(vehicle: number): number;

/**
 * Affects the playback speed of the submarine car conversion animations. Does not affect hardcoded animations such as the wheels being retracted. In decompiled scripts the only value used for transformRate is 2.5.
 * @param vehicle - 
 * @param transformRate - 
 * @returns void
 * @remarks Hash: 0x498218259FB7C72D
 */
export function setTransformRateForAnimation(vehicle: number, transformRate: number): void;

/**
 * ```
 * Allows you to toggle bulletproof tires.  
 * ```
 * @param vehicle - 
 * @param toggle - 
 * @returns void
 * @remarks Hash: 0xEB9DC3C7D8596C46
 */
export function setVehicleTyresCanBurst(vehicle: number, toggle: number): void;

/**
 * This native allows opening or closing the wings of the Deluxo/Oppressor. For the Deluxo, wing deployment depends on sufficient altitude.
 * @param vehicle - 
 * @param ratio - 
 * @returns void
 * @remarks Hash: 0x70A252F60A3E036B
 */
export function setHoverModeWingRatio(vehicle: number, ratio: number): void;

/**
 * ```
 * HookOffset defines where the hook is attached. leave at 0 for default attachment.
 * ```
 * @param towTruck - 
 * @param vehicle - 
 * @param rear - 
 * @param hookOffsetX - 
 * @param hookOffsetY - 
 * @param hookOffsetZ - 
 * @returns void
 * @remarks Hash: 0x29A16F8D621C4508
 */
export function attachVehicleToTowTruck(towTruck: number, vehicle: number, rear: number, hookOffsetX: number, hookOffsetY: number, hookOffsetZ: number): void;

/**
 * ```
 * Often called after START_PLAYBACK_RECORDED_VEHICLE and SKIP_TIME_IN_PLAYBACK_RECORDED_VEHICLE; similar in use to FORCE_ENTITY_AI_AND_ANIMATION_UPDATE.
 * ```
 * @param vehicle - 
 * @param p1 - 
 * @returns void
 * @remarks Hash: 0x1F2E4E06DEA8992B
 */
export function forcePlaybackRecordedVehicleUpdate(vehicle: number, p1: number): void;

/**
 * ```
 * NativeDB Introduced: v1868
 * ```
 * @param vehicle - 
 * @param togle - 
 * @returns void
 * @remarks Hash: 0x4AD280EB48B2D8E6
 */
export function 0x4ad280eb48b2d8e6(vehicle: number, togle: number): void;

/**
 * ## Parameters
 * *
 * @param vehicle - 
 * @param trailer - 
 * @param offsetX - 
 * @param offsetY - 
 * @param offsetZ - 
 * @param coordsX - 
 * @param coordsY - 
 * @param coordsZ - 
 * @param rotationX - 
 * @param rotationY - 
 * @param rotationZ - 
 * @param disableColls - 
 * @returns void
 * @remarks Hash: 0x16B5E274BDE402F8
 */
export function attachVehicleOnToTrailer(vehicle: number, trailer: number, offsetX: number, offsetY: number, offsetZ: number, coordsX: number, coordsY: number, coordsZ: number, rotationX: number, rotationY: number, rotationZ: number, disableColls: number): void;

/**
 * @returns void
 * @remarks Hash: 0x48ADC8A773564670
 */
export function instantlyFillVehiclePopulation(): void;

/**
 * ## Parameters
 * *
 * @param vehicle - 
 * @param x - 
 * @param y - 
 * @param z - 
 * @returns void
 * @remarks Hash: 0xE38CB9D7D39FDBCC
 */
export function EjectJb700Roof(vehicle: number, x: number, y: number, z: number): void;

/**
 * ## Parameters
 * *
 * @param vehicle - 
 * @param time - 
 * @returns void
 * @remarks Hash: 0xE00F2AB100B76E89
 */
export function SetVehicleRocketBoostRefillTime(vehicle: number, time: number): void;

/**
 * ```
 * Max 1000.
 * At -100 both helicopter rotors will stall.
 * ```
 * @param vehicle - 
 * @returns number
 * @remarks Hash: 0xAC51915D27E4A5F7
 */
export function getHeliTailBoomHealth(vehicle: number): number;

/**
 * ## Parameters
 * *
 * @param vehicle - 
 * @returns number
 * @remarks Hash: 0x291E373D483E7EE7
 */
export function isAnyPedRappellingFromHeli(vehicle: number): number;

/**
 * Removes the cargen area of interest and resumes normal cargen spawning.
 * 
 * You can set the area of interest with [`SET_VEHICLE_GENERATOR_AREA_OF_INTEREST`](#_0x9A75585FB2E54FAD)
 * @returns void
 * @remarks Hash: 0x0A436B8643716D14
 */
export function clearVehicleGeneratorAreaOfInterest(): void;

/**
 * ## Parameters
 * *
 * @param vehicle - 
 * @returns number
 * @remarks Hash: 0xF095C0405307B21B
 */
export function getIsVehiclePrimaryColourCustom(vehicle: number): number;

/**
 * ```
 * Sets the tire smoke's color of this vehicle.  
 * vehicle: The vehicle that is the target of this method.  
 * r: The red level in the RGB color code.  
 * g: The green level in the RGB color code.  
 * b: The blue level in the RGB color code.  
 * Note:  
 * setting r,g,b to 0 will give the car independance day tyre smoke  
 * ```
 * @param vehicle - 
 * @param r - 
 * @param g - 
 * @param b - 
 * @returns void
 * @remarks Hash: 0xB5BA80F839791C0F
 */
export function setVehicleTyreSmokeColor(vehicle: number, r: number, g: number, b: number): void;

/**
 * ## Parameters
 * *
 * @param outVec - 
 * @param p1 - 
 * @param outVec1 - 
 * @param p3 - 
 * @param p4 - 
 * @param p5 - 
 * @param p6 - 
 * @param p7 - 
 * @param p8 - 
 * @returns number
 * @remarks Hash: 0xA4822F1CF23F4810
 */
export function 0xa4822f1cf23f4810(outVec: { x: number, y: number, z: number }, p1: { x: number, y: number, z: number }, outVec1: { x: number, y: number, z: number }, p3: any, p4: any, p5: any, p6: any, p7: any, p8: any): number;

/**
 * ```
 * NativeDB Introduced: v1180
 * ```
 * @param vehicle - 
 * @param toggle - 
 * @returns void
 * @remarks Hash: 0x97841634EF7DF1D6
 */
export function 0x97841634ef7df1d6(vehicle: number, toggle: number): void;

/**
 * ```
 * Toggles to render distant vehicles. They may not be vehicles but images to look like vehicles.  
 * ```
 * @param toggle - 
 * @returns void
 * @remarks Hash: 0xF796359A959DF65D
 */
export function setDistantCarsEnabled(toggle: number): void;

/**
 * ```c
 * enum WindowTints  
 * {  
 * 	WINDOWTINT_NONE = 0,
 * 	WINDOWTINT_PURE_BLACK = 1,
 * 	WINDOWTINT_DARKSMOKE = 2,
 * 	WINDOWTINT_LIGHTSMOKE = 3,
 * 	WINDOWTINT_STOCK = 4,
 * 	WINDOWTINT_LIMO = 5,
 * 	WINDOWTINT_GREEN = 6
 * };  
 * ```
 * @param vehicle - 
 * @param tint - 
 * @returns void
 * @remarks Hash: 0x57C51E6BAD752696
 */
export function setVehicleWindowTint(vehicle: number, tint: number): void;

/**
 * Smashes a vehicles window. See eWindowId declared in [`IS_VEHICLE_WINDOW_INTACT`](#_0x46E571A0E20D01F1).
 * @param vehicle - 
 * @param windowIndex - 
 * @returns void
 * @remarks Hash: 0x9E5B5E4D2CCD2259
 */
export function smashVehicleWindow(vehicle: number, windowIndex: number): void;

/**
 * ## Parameters
 * *
 * @param p0 - 
 * @param p1 - 
 * @returns any
 * @remarks Hash: 0xF3B0E0AED097A3F5
 */
export function 0xf3b0e0aed097a3f5(p0: any, p1: any): any;

/**
 * ## Parameters
 * *
 * @param vehicle - 
 * @param modType - 
 * @returns number
 * @remarks Hash: 0x772960298DA26FDB
 */
export function getVehicleMod(vehicle: number, modType: number): number;

/**
 * ```
 * Works only on vehicles that support hydraulic.
 * ```
 * @param vehicle - 
 * @param wheelId - 
 * @param value - 
 * @returns void
 * @remarks Hash: 0x84EA99C62CB3EF0C
 */
export function SetHydraulicWheelValue(vehicle: number, wheelId: number, value: number): void;

/**
 * ## Parameters
 * *
 * @param vehicle - 
 * @returns void
 * @remarks Hash: 0xAB8E2EDA0C0A5883
 */
export function skipToEndAndStopPlaybackRecordedVehicle(vehicle: number): void;

/**
 * ## Parameters
 * *
 * @param toggle - 
 * @returns void
 * @remarks Hash: 0xD4B8E3D1917BC86B
 */
export function setDisableRandomTrainsThisFrame(toggle: number): void;

/**
 * ```
 * NativeDB Introduced: 3095
 * ```
 * 
 * Retrieves the remaining duration of nitrous boost available for the specified vehicle.
 * @param vehicle - 
 * @returns number
 * @remarks Hash: 0xBEC4B8653462450E
 */
export function GetRemainingNitrousDuration(vehicle: number): number;

/**
 * ```
 * SET_*
 * ```
 * @param vehicle - 
 * @param x - 
 * @param y - 
 * @param z - 
 * @param p4 - 
 * @returns void
 * @remarks Hash: 0x428AD3E26C8D9EB0
 */
export function 0x428ad3e26c8d9eb0(vehicle: number, x: number, y: number, z: number, p4: number): void;

/**
 * ```
 * NativeDB Introduced: v1290
 * ```
 * @param vehicle - 
 * @returns number
 * @remarks Hash: 0xA77DC70BD689A1E5
 */
export function isVehicleInSubmarineMode(vehicle: number): number;

/**
 * ```
 * 1000 is max health  
 * Begins leaking gas at around 650 health  
 * ```
 * @param vehicle - 
 * @returns number
 * @remarks Hash: 0x7D5DABE888D2D074
 */
export function getVehiclePetrolTankHealth(vehicle: number): number;

/**
 * ## Parameters
 * *
 * @param vehicle - 
 * @param toggle - 
 * @returns void
 * @remarks Hash: 0x5E569EC46EC21CAE
 */
export function 0x5e569ec46ec21cae(vehicle: number, toggle: number): void;

/**
 * ```
 * NativeDB Added Parameter 2: float maxEngineHealth
 * NativeDB Added Parameter 3: float maxPetrolTankHealth
 * NativeDB Added Parameter 4: float maxBodyHealth
 * NativeDB Added Parameter 5: float maxMainRotorHealth
 * NativeDB Added Parameter 6: float maxTailRotorHealth
 * NativeDB Added Parameter 7: float maxUnkHealth
 * ```
 * @param vehicle - 
 * @returns number
 * @remarks Hash: 0xB8EF61207C2393A9
 */
export function getVehicleHealthPercentage(vehicle: number): number;

/**
 * Sounds the horn for the specified vehicle. Note that if a player is in the vehicle, it will only sound briefly.
 * @param vehicle - 
 * @param duration - 
 * @param mode - 
 * @param forever - 
 * @returns void
 * @remarks Hash: 0x9C8C6504B5B63D2C
 */
export function startVehicleHorn(vehicle: number, duration: number, mode: number, forever: number): void;

/**
 * ## Parameters
 * *
 * @param vehicle - 
 * @param toggle - 
 * @returns void
 * @remarks Hash: 0x684785568EF26A22
 */
export function setVehicleHandbrake(vehicle: number, toggle: number): void;

/**
 * ```
 * SET_VEHICLE_W* (next character is either H or I)
 * ```
 * @param vehicle - 
 * @param p1 - 
 * @returns void
 * @remarks Hash: 0x2C4A1590ABF43E8B
 */
export function 0x2c4a1590abf43e8b(vehicle: number, p1: number): void;

/**
 * ```
 * Maximum amount of vehicles with vehicle stuck check appears to be 16.  
 * ```
 * @param vehicle - 
 * @returns number
 * @remarks Hash: 0x57E4C39DE5EE8470
 */
export function doesVehicleHaveStuckVehicleCheck(vehicle: number): number;

/**
 * Adjusts the scale of damage applied to a specified section of a plane.
 * In the decompiled scripts the `damageScale` is always set to `0f` (maybe to disable damages on the specified section)
 * 
 * ```c
 * enum ePlaneDamageSection {
 *     WING_L = 0,
 *     WING_R = 1,
 *     TAIL = 2,
 *     ENGINE_L = 3,
 *     ENGINE_R = 4,
 *     ELEVATOR_L = 5,
 *     ELEVATOR_R = 6,
 *     AILERON_L = 7,
 *     AILERON_R = 8,
 *     RUDDER = 9,
 *     RUDDER_2 = 10,
 *     AIRBRAKE_L = 11,
 *     AIRBRAKE_R = 12
 * }
 * ```
 * 
 * ```
 * NativeDB Introduced: v1290
 * ```
 * @param vehicle - 
 * @param damageSection - 
 * @returns void
 * @remarks Hash: 0x0BBB9A7A8FFE931B
 */
export function setPlaneSectionDamageScale(vehicle: number, damageSection: number): void;

/**
 * ## Parameters
 * *
 * @param vehicle - 
 * @param doorIndex - 
 * @param doorLockStatus - 
 * @returns void
 * @remarks Hash: 0xBE70724027F85BCD
 */
export function setVehicleIndividualDoorsLocked(vehicle: number, doorIndex: number, doorLockStatus: number): void;

/**
 * ## Parameters
 * *
 * @param vehicle - 
 * @param p1 - 
 * @returns void
 * @remarks Hash: 0x0F3B4D4E43177236
 */
export function GetBoatBoomPositionRatio3(vehicle: number, p1: number): void;

/**
 * Please refer to [`GET_VEHICLE_NUMBER_PLATE_TEXT_INDEX`](#_0xF11BC2DD9A3E7195) for plate indicies.
 * @param vehicle - 
 * @param plateIndex - 
 * @returns void
 * @remarks Hash: 0x9088EB5A43FFB0A1
 */
export function setVehicleNumberPlateTextIndex(vehicle: number, plateIndex: number): void;

/**
 * ## Parameters
 * *
 * @param vehicle - 
 * @param p1 - 
 * @param p2 - 
 * @param p3 - 
 * @returns number
 * @remarks Hash: 0xBC74B4BE25EB6C8A
 */
export function isHeliPartBroken(vehicle: number, p1: number, p2: number, p3: number): number;

/**
 * Prevents a specified entity from being detached from a Cargobob, even in the event of collisions.
 * @param cargobob - 
 * @param entity - 
 * @returns void
 * @remarks Hash: 0x1F34B0626C594380
 */
export function setCargobobExcludeFromPickupEntity(cargobob: number, entity: number): void;

/**
 * ```
 * NativeDB Introduced: v1180
 * ```
 * @param vehicle - 
 * @returns number
 * @remarks Hash: 0xE43701C36CAFF1A4
 */
export function DoesVehicleHaveLandingGear(vehicle: number): number;

/**
 * ```
 * Checks via CVehicleModelInfo  
 * ```
 * @param vehicle - 
 * @param extraId - 
 * @returns number
 * @remarks Hash: 0x1262D55792428154
 */
export function doesExtraExist(vehicle: number, extraId: number): number;

/**
 * ```
 * Something to do with "high speed bump severity"?  
 * if (!sub_87a46("SET_CAR_HIGH_SPEED_BUMP_SEVERITY_MULTIPLIER")) {  
 *     VEHICLE::_84FD40F56075E816(0.0);  
 *     sub_8795b("SET_CAR_HIGH_SPEED_BUMP_SEVERITY_MULTIPLIER", 1);  
 * }  
 * ```
 * @param multiplier - 
 * @returns void
 * @remarks Hash: 0x84FD40F56075E816
 */
export function SetCarHighSpeedBumpSeverityMultiplier(multiplier: number): void;

/**
 * ```
 * First two parameters swapped. Scripts verify that towTruck is the first parameter, not the second.  
 * ```
 * @param towTruck - 
 * @param vehicle - 
 * @returns void
 * @remarks Hash: 0xC2DB6B6708350ED8
 */
export function detachVehicleFromTowTruck(towTruck: number, vehicle: number): void;

/**
 * p3 is some flag related to 'trailers' (invokes CVehicle::GetTrailer).
 * 
 * See [`REQUEST_VEHICLE_RECORDING`](#_0xAF514CABE74CBF15).
 * @param vehicle - 
 * @param recording - 
 * @param script - 
 * @param p3 - 
 * @returns void
 * @remarks Hash: 0x3F878F92B3A7A071
 */
export function startPlaybackRecordedVehicle(vehicle: number, recording: number, script: string, p3: number): void;

/**
 * ```
 * 1000 is max health
 * Begins leaking gas at around 650 health
 * -999.90002441406 appears to be minimum health, although nothing special occurs <- false statement
 * @param vehicle - 
 * @param health - 
 * @returns void
 * @remarks Hash: 0x45F6D8EEF34ABEF1
 */
export function setVehicleEngineHealth(vehicle: number, health: number): void;

/**
 * ## Parameters
 * *
 * @param id - 
 * @returns number
 * @remarks Hash: 0x102D125411A7B6E6
 */
export function getTotalDurationOfVehicleRecordingId(id: number): number;

/**
 * ```
 * Checks if model is a boat, then checks for FLAG_IS_JETSKI.
 * ```
 * @param model - 
 * @returns number
 * @remarks Hash: 0x9537097412CF75FE
 */
export function IsThisModelAJetski(model: number): number;

/**
 * ```
 * A vehicle recording playback flag only used in jewelry_heist
 * ```
 * @param vehicle - 
 * @param p1 - 
 * @returns void
 * @remarks Hash: 0x063AE2B2CC273588
 */
export function 0x063ae2b2cc273588(vehicle: number, p1: number): void;

/**
 * ## Parameters
 * *
 * @param vehicle - 
 * @param toggle - 
 * @returns void
 * @remarks Hash: 0xDFFCEF48E511DB48
 */
export function setVehicleActiveDuringPlayback(vehicle: number, toggle: number): void;

/**
 * ```
 * NativeDB Introduced: v1180
 * ```
 * @param vehicle - 
 * @param toggle - 
 * @returns void
 * @remarks Hash: 0x8235F1BEAD557629
 */
export function 0x8235f1bead557629(vehicle: number, toggle: number): void;

/**
 * ```
 * NativeDB Introduced: v1604
 * ```
 * @param vehicle - 
 * @returns number
 * @remarks Hash: 0xA2459F72C14E2E8D
 */
export function GetIsVehicleShuntBoostActive(vehicle: number): number;

/**
 * ```
 * Distance traveled in the vehicles current recording.
 * ```
 * @param vehicle - 
 * @returns number
 * @remarks Hash: 0x2DACD605FC681475
 */
export function getPositionInRecording(vehicle: number): number;

/**
 * ## Parameters
 * *
 * @param vehicle - 
 * @param state - 
 * @returns void
 * @remarks Hash: 0x8EA86DF356801C7D
 */
export function SetHydraulicWheelState(vehicle: number, state: number): void;

/**
 * ## Parameters
 * *
 * @param vehicle - 
 * @param toggle - 
 * @returns void
 * @remarks Hash: 0xA37B9A517B133349
 */
export function setVehicleWheelsCanBreakOffWhenBlowUp(vehicle: number, toggle: number): void;

/**
 * ## Parameters
 * *
 * @param vehicle - 
 * @param p1 - 
 * @returns void
 * @remarks Hash: 0x685D5561680D088B
 */
export function setCargobobPickupMagnetFalloff(vehicle: number, p1: number): void;

/**
 * ## Parameters
 * *
 * @param vehicle - 
 * @param state - 
 * @returns void
 * @remarks Hash: 0xCDE5E70C1DDB954C
 */
export function setVehicleAlarm(vehicle: number, state: number): void;

/**
 * This works on helicopters and planes.
 * 
 * Prevents a helicopter from exploding due to relatively minor body damage. 
 * 
 * ```
 * NativeDB Introduced: v1103
 * ```
 * @param helicopter - 
 * @returns void
 * @remarks Hash: 0xEDBC8405B3895CC9
 */
export function setDisableHeliExplodeFromBodyDamage(helicopter: number): void;

/**
 * ```
 * CLEAR_VEHICLE_*
 * ```
 * @param vehicle - 
 * @returns void
 * @remarks Hash: 0x4419966C9936071A
 */
export function 0x4419966c9936071a(vehicle: number): void;

/**
 * ## Parameters
 * *
 * @param vehicle - 
 * @returns number
 * @remarks Hash: 0x678B9BB8C3F58FEB
 */
export function getVehicleTyresCanBurst(vehicle: number): number;

/**
 * ```
 * Sets a vehicle to be strongly resistant to explosions. p0 is the vehicle; set p1 to false to toggle the effect on/off.  
 * ```
 * @param vehicle - 
 * @param toggle - 
 * @returns void
 * @remarks Hash: 0x71B0892EC081D60A
 */
export function setVehicleExplodesOnHighExplosionDamage(vehicle: number, toggle: number): void;

/**
 * According to decompiled scripts this should work with the `deluxo` and `oppressor2` vehicles.
 * Does nothing when used on `oppressor2`.
 * 
 * For the deluxo:
 * - Set `state` to `0.0`: Fully transform to a 'road' vehicle (non-hover mode).
 * - Set `state` to `1.0`: Fully transform to a 'flying' vehicle (hover mode).
 * 
 * If you set it to something like 0.5, then something [weird happens](https://streamable.com/p6wmr), you end up in some 50% hover mode, 50% not hover mode.
 * 
 * This doesn't need to be called every tick, just once and the vehicle will transform to that state at the usual transform speed. It'll just stop transforming when it reaches the state you provided.
 * 
 * Once this native is used then players will just be able to hit the vehicle transform key to toggle the transformation cycle; it won't block users from using the key.
 * @param vehicle - 
 * @param state - 
 * @returns void
 * @remarks Hash: 0x438B3D7CA026FE91
 */
export function setSpecialFlightModeTargetRatio(vehicle: number, state: number): void;

/**
 * ## Parameters
 * *
 * @param vehicle - 
 * @param p1 - 
 * @param p2 - 
 * @returns void
 * @remarks Hash: 0x99CAD8E7AFDB60FA
 */
export function 0x99cad8e7afdb60fa(vehicle: number, p1: number, p2: number): void;

/**
 * Copies sourceVehicle's damage (broken bumpers, broken lights, etc.) to targetVehicle.
 * @param sourceVehicle - 
 * @param targetVehicle - 
 * @returns void
 * @remarks Hash: 0xE44A982368A4AF23
 */
export function copyVehicleDamages(sourceVehicle: number, targetVehicle: number): void;

/**
 * See eDoorId declared in [`SET_VEHICLE_DOOR_SHUT`](#_0x93D9BD300D7789E5)
 * @param vehicle - 
 * @param doorIndex - 
 * @returns number
 * @remarks Hash: 0x3E933CFF7B111C22
 */
export function isVehicleDoorFullyOpen(vehicle: number, doorIndex: number): number;

/**
 * ```
 * NativeDB Introduced: v2372
 * ```
 * @param toggle - 
 * @returns void
 * @remarks Hash: 0xDA5E12F728DB30CA
 */
export function SetRandomBoatsInMp(toggle: number): void;

/**
 * ## Parameters
 * *
 * @param vehicle - 
 * @param modType - 
 * @param modIndex - 
 * @returns number
 * @remarks Hash: 0x90A38E9838E0A8C1
 */
export function getVehicleModModifierValue(vehicle: number, modType: number, modIndex: number): number;

/**
 * Used alongside [`SET_SPECIAL_FLIGHT_MODE_TARGET_RATIO`](#_0x438B3D7CA026FE91), this function initiates hover transformation for vehicles with a hover mode, like the `Deluxo`, based on a specified ratio (0.0 to 1.0). Incorrect values can glitch the vehicle. Without pairing, vehicles revert to car mode. Ineffective on the `oppressor2`
 * @param vehicle - 
 * @param ratio - 
 * @returns void
 * @remarks Hash: 0xD138FA15C9776837
 */
export function setSpecialFlightModeRatio(vehicle: number, ratio: number): void;

/**
 * ```
 * p1 (toggle) was always 1 (true) except in one case in the b678 scripts.  
 * ```
 * @param vehicle - 
 * @param toggle - 
 * @returns void
 * @remarks Hash: 0x07116E24E9D1929D
 */
export function setVehicleIsRacing(vehicle: number, toggle: number): void;

/**
 * ## Parameters
 * *
 * @param vehicle - 
 * @returns void
 * @remarks Hash: 0x5335BE58C083E74E
 */
export function LowerRetractableWheels(vehicle: number): void;

/**
 * Native is significantly more complicated than simply generating a random vector & length.
 * 
 * The 'point' is either 400.0 or 250.0 units away from the Ped's current coordinates; and paths into functions like rage::grcViewport___IsSphereVisible.
 * 
 * ```
 * NativeDB Introduced: v1290
 * ```
 * @param ped - 
 * @returns { x: number, y: number, z: number }
 * @remarks Hash: 0x8DC9675797123522
 */
export function FindRandomPointInSpace(ped: number): { x: number, y: number, z: number };

/**
 * ```
 * Previously named GET_VEHICLE_DEFORMATION_GET_TREE (hash collision)
 * from Decrypted Scripts I found
 * VEHICLE::SET_VEHICLE_CEILING_HEIGHT(l_BD9[2/*2*/], 420.0);
 * ```
 * @param vehicle - 
 * @param height - 
 * @returns void
 * @remarks Hash: 0xA46413066687A328
 */
export function setVehicleCeilingHeight(vehicle: number, height: number): void;

/**
 * ## Parameters
 * *
 * @param p0 - 
 * @param p1 - 
 * @param p2 - 
 * @returns void
 * @remarks Hash: 0x870B8B7A766615C8
 */
export function 0x870b8b7a766615c8(p0: any, p1: any, p2: any): void;

/**
 * ```
 * Returns how many possible mods a vehicle has for a given mod type  
 * ```
 * @param vehicle - 
 * @param modType - 
 * @returns number
 * @remarks Hash: 0xE38E9162A2500646
 */
export function getNumVehicleMods(vehicle: number, modType: number): number;

/**
 * @returns void
 * @remarks Hash: 0xE01903C47C7AC89E
 */
export function clearLastDrivenVehicle(): void;

/**
 * ```
 * Works for vehicles with a retractable landing gear  
 * landing gear states:  
 * 0: Deployed  
 * 1: Closing  
 * 2: Opening  
 * 3: Retracted  
 * ```
 * @param vehicle - 
 * @param state - 
 * @returns void
 * @remarks Hash: 0xCFC8BE9A5E1FE575
 */
export function controlLandingGear(vehicle: number, state: number): void;

/**
 * ## Parameters
 * *
 * @param vehicle - 
 * @param seatIndex - 
 * @returns number
 * @remarks Hash: 0x30785D90C956BF35
 */
export function canShuffleSeat(vehicle: number, seatIndex: number): number;

/**
 * ## Parameters
 * *
 * @param vehicle - 
 * @param p1 - 
 * @returns void
 * @remarks Hash: 0x182F266C2D9E2BEB
 */
export function 0x182f266c2d9e2beb(vehicle: number, p1: number): void;

/**
 * ```
 * Returns max braking of the specified vehicle model.
 * ```
 * @param modelHash - 
 * @returns number
 * @remarks Hash: 0xDC53FD41B4ED944C
 */
export function getVehicleModelMaxBraking(modelHash: number): number;

/**
 * ```
 * NativeDB Introduced: v1365
 * ```
 * @param vehicle - 
 * @returns void
 * @remarks Hash: 0x887FA38787DE8C72
 */
export function 0x887fa38787de8c72(vehicle: number): void;

/**
 * Determines if a vehicle is a convertible with an animatable roof. This native checks if the specified vehicle model features a convertible roof that can be lowered or raised through an animation.
 * 
 * ```
 * NativeDB Introduced: v323
 * ```
 * @param vehicle - 
 * @param checkRoofExtras - 
 * @returns number
 * @remarks Hash: 0x52F357A30698BCCE
 */
export function isVehicleAConvertible(vehicle: number, checkRoofExtras: number): number;

/**
 * Determines whether a specific vehicle is equipped with a roof.
 * 
 * ```
 * NativeDB Introduced: v323
 * ```
 * @param vehicle - 
 * @returns number
 * @remarks Hash: 0x8AC862B0B32C5B80
 */
export function doesVehicleHaveRoof(vehicle: number): number;

/**
 * ## Parameters
 * *
 * @returns number
 * @remarks Hash: 0x563B65A643ED072E
 */
export function IsVehicleWeaponDisabled(): number;

/**
 * ## Parameters
 * *
 * @param p0 - 
 * @returns void
 * @remarks Hash: 0x35E0654F4BAD7971
 */
export function 0x35e0654f4bad7971(p0: number): void;

/**
 * ```
 * Does nothing. It's a nullsub.
 * 
 * NativeDB Introduced: v1604
 * ```
 * @param toggle - 
 * @returns void
 * @remarks Hash: 0x99A05839C46CE316
 */
export function 0x99a05839c46ce316(toggle: number): void;

/**
 * ```
 * Roll down all the windows of the vehicle passed through the first parameter.  
 * ```
 * @param vehicle - 
 * @returns void
 * @remarks Hash: 0x85796B0549DDE156
 */
export function rollDownWindows(vehicle: number): void;

/**
 * ```
 * Same call as VEHICLE::_0x0F3B4D4E43177236
 * ```
 * @param vehicle - 
 * @param p1 - 
 * @returns void
 * @remarks Hash: 0xC1F981A6F74F0C23
 */
export function GetBoatBoomPositionRatio2(vehicle: number, p1: number): void;

/**
 * ```
 * NativeDB Introduced: v1290
 * ```
 * @param vehicle - 
 * @param p1 - 
 * @param p2 - 
 * @param p3 - 
 * @param p4 - 
 * @returns void
 * @remarks Hash: 0x9640E30A7F395E4B
 */
export function 0x9640e30a7f395e4b(vehicle: number, p1: any, p2: any, p3: any, p4: any): void;

/**
 * Sets flag on vehicle that changes behaviour in relation to when player gets wanted level
 * @param vehicle - 
 * @param p1 - 
 * @returns void
 * @remarks Hash: 0x4E74E62E0A97E901
 */
export function setPoliceFocusWillTrackVehicle(vehicle: number, p1: number): void;

/**
 * ```
 * Commands the driver of an armed vehicle (p0) to shoot its weapon at a target (p1). p3, p4 and p5 are the coordinates of the target. Example:  
 * WEAPON::SET_CURRENT_PED_VEHICLE_WEAPON(pilot,GAMEPLAY::GET_HASH_KEY("VEHICLE_WEAPON_PLANE_ROCKET"));VEHICLE::SET_VEHICLE_SHOOT_AT_TARGET(pilot, target, targPos.x, targPos.y, targPos.z);  
 * ```
 * @param driver - 
 * @param entity - 
 * @param xTarget - 
 * @param yTarget - 
 * @param zTarget - 
 * @returns void
 * @remarks Hash: 0x74CD9A9327A282EA
 */
export function setVehicleShootAtTarget(driver: number, entity: number, xTarget: number, yTarget: number, zTarget: number): void;

/**
 * ```
 * Landing gear states:  
 * 0: Deployed  
 * 1: Closing (Retracting)
 * 3: Opening (Deploying)
 * 4: Retracted  
 * 5: Broken
 * ```
 * 
 * Landing gear state 2 is never used.
 * @param vehicle - 
 * @returns number
 * @remarks Hash: 0x9B0F3DCA3DB0F4CD
 */
export function getLandingGearState(vehicle: number): number;

/**
 * ```
 * NativeDB Introduced: v1290
 * ```
 * @param vehicle - 
 * @param p1 - 
 * @returns void
 * @remarks Hash: 0x0205F5365292D2EB
 */
export function 0x0205f5365292d2eb(vehicle: number, p1: number): void;

/**
 * Checks if a boat is currently anchored.
 * 
 * This native is a getter for [SET_BOAT_ANCHOR](#_0x75DBEC174AEEAD10).
 * 
 * 
 * ```
 * NativeDB Introduced: v573
 * ```
 * @param boat - 
 * @returns number
 * @remarks Hash: 0xB0AD1238A709B1A2
 */
export function isBoatAnchored(boat: number): number;

/**
 * ```
 * Gets the number of passengers, NOT including the driver. Use IS_VEHICLE_SEAT_FREE(Vehicle, -1) to also check for the driver  
 * ```
 * @param vehicle - 
 * @returns number
 * @remarks Hash: 0x24CB2137731FFE89
 */
export function getVehicleNumberOfPassengers(vehicle: number): number;

/**
 * ```
 * Works just like SET_VEHICLE_ENGINE_HEALTH, didn't saw any difference. But this native works only for planes.
 * ```
 * @param vehicle - 
 * @param health - 
 * @returns void
 * @remarks Hash: 0x2A86A0475B6A1434
 */
export function SetPlaneEngineHealth(vehicle: number, health: number): void;

/**
 * ## Parameters
 * *
 * @param vehicle - 
 * @returns void
 * @remarks Hash: 0xA6E9FDCB2C76785E
 */
export function requestVehicleHighDetailModel(vehicle: number): void;

/**
 * ## Return value
 * @returns number
 * @remarks Hash: 0x9D1224004B3A6707
 */
export function getNumVehicleWindowTints(): number;

/**
 * ## Parameters
 * *
 * @param vehicle - 
 * @param value - 
 * @returns void
 * @remarks Hash: 0x93A3996368C94158
 */
export function modifyVehicleTopSpeed(vehicle: number, value: number): void;

/**
 * ```
 * NativeDB Introduced: v1604
 * NativeDB Added Parameter 2 (2060): float durationMod : A multiplier applied to the default nitrous duration (default is 3 seconds). 
 * NativeDB Added Parameter 3 (2060): float power : A multiplier applied to the default nitrous power multiplier (default is 3).
 * NativeDB Added Parameter 4 (2060): float rechargeTime : A multiplier applied to the default nitrous recharge rate.
 * NativeDB Added Parameter 5 (2060): BOOL disableSound : A boolean to disable the default nitrous sound when the nitrous is active.
 * ```
 * 
 * Overrides the default settings of a vehicle's nitrous system, allowing custom control over its performance characteristics.
 * @param vehicle - 
 * @param override - 
 * @returns void
 * @remarks Hash: 0xC8E9B6B71B8E660D
 */
export function setOverrideNitrousLevel(vehicle: number, override: number): void;

/**
 * ```
 * value between 0.0 and 1.0  
 * ```
 * @param helicopter - 
 * @param multiplier - 
 * @returns void
 * @remarks Hash: 0x6E0859B530A365CC
 */
export function SetHelicopterRollPitchYawMult(helicopter: number, multiplier: number): void;

/**
 * ```
 * NativeDB Introduced: v1868
 * ```
 * @param vehicle - 
 * @param wheelIndex - 
 * @param multiplier - 
 * @returns void
 * @remarks Hash: 0x01894E2EDE923CA2
 */
export function SetTyreWearMultiplier(vehicle: number, wheelIndex: number, multiplier: number): void;

/**
 * ## Parameters
 * *
 * @param vehicleClass - 
 * @returns number
 * @remarks Hash: 0x4F930AD022D6DE3B
 */
export function getVehicleClassMaxAgility(vehicleClass: number): number;

/**
 * ## Parameters
 * *
 * @param p0 - 
 * @param p1 - 
 * @returns void
 * @remarks Hash: 0x78CEEE41F49F421F
 */
export function 0x78ceee41f49f421f(p0: any, p1: any): void;

/**
 * ## Parameters
 * *
 * @param vehicle - 
 * @param multiplier - 
 * @returns void
 * @remarks Hash: 0x93AE6A61BE015BF1
 */
export function setVehicleLodMultiplier(vehicle: number, multiplier: number): void;

/**
 * Sets a limited number of ammo for a particular vehicle weapon index on a vehicle.  
 * 
 * Information about weapon indexes can be found in `handling.meta`. 
 * 
 * In the example given below, `uWeaponHash` defines weapon hashes for the vehicle. Each `<Item>` corresponds to an index starting from `0`.
 * 
 * ```
 * <uWeaponHash>
 *     <Item>VEHICLE_WEAPON_PLAYER_BUZZARD</Item>  <!-- Index: 0 -->
 *     <Item>VEHICLE_WEAPON_SPACE_ROCKET</Item>    <!-- Index: 1 -->
 *     <Item>VEHICLE_WEAPON_SEARCHLIGHT</Item>     <!-- Index: 2 -->
 * </uWeaponHash>
 * ```
 * @param vehicle - 
 * @param weaponIndex - 
 * @param ammoCount - 
 * @returns void
 * @remarks Hash: 0x44CD1F493DB2A0A6
 */
export function setVehicleWeaponRestrictedAmmo(vehicle: number, weaponIndex: number, ammoCount: number): void;

/**
 * ```
 * paintType:
 * 0: Normal
 * 1: Metallic
 * 2: Pearl
 * 3: Matte
 * 4: Metal
 * 5: Chrome
 * ```
 * @param paintType - 
 * @param p1 - 
 * @returns number
 * @remarks Hash: 0xA551BE18C11A476D
 */
export function getNumModColors(paintType: number, p1: number): number;

/**
 * ```
 * Checks if vehicle tyre at index exists. Also returns false if tyre was removed.
 * ```
 * 
 * ```
 * NativeDB Introduced: v1493
 * ```
 * @param vehicle - 
 * @param tyreIndex - 
 * @returns number
 * @remarks Hash: 0x534E36D4DB9ECC5D
 */
export function DoesVehicleTyreExist(vehicle: number, tyreIndex: number): number;

/**
 * ```
 * Returns a value depending on the lock-on state of vehicle weapons.
 * 0: not locked on
 * 1: locking on
 * 2: locked on
 * ```
 * @param vehicle - 
 * @returns number
 * @remarks Hash: 0xE6B0E8CFC3633BF0
 */
export function getVehicleHomingLockonState(vehicle: number): number;

/**
 * @returns void
 * @remarks Hash: 0x736A718577F39C7D
 */
export function deleteAllTrains(): void;

/**
 * ## Parameters
 * *
 * @param vehicle - 
 * @param r - 
 * @param g - 
 * @param b - 
 * @returns void
 * @remarks Hash: 0x8389CD56CA8072DC
 */
export function getVehicleCustomSecondaryColour(vehicle: number, r: number, g: number, b: number): void;

/**
 * ## Parameters
 * *
 * @param vehicle - 
 * @returns void
 * @remarks Hash: 0x00689CDE5F7C6787
 */
export function removeVehicleHighDetailModel(vehicle: number): void;

/**
 * Makes a helicopter resistant to multiple explosions. When enabled, helicopters can survive two or more explosions.
 * 
 * ```
 * NativeDB Introduced: 2545
 * ```
 * @param helicopter - 
 * @param bResistToExplosion - 
 * @returns void
 * @remarks Hash: 0x8074CC1886802912
 */
export function setHeliResistToExplosion(helicopter: number, bResistToExplosion: number): void;

/**
 * ## Parameters
 * *
 * @param p0 - 
 * @param p1 - 
 * @returns void
 * @remarks Hash: 0x4D9D109F63FEE1D4
 */
export function 0x4d9d109f63fee1d4(p0: any, p1: number): void;

/**
 * ```
 * Returns whether the specified vehicle is currently in a burnout.  
 * vb.net  
 * Public Function isVehicleInBurnout(vh As Vehicle) As Boolean  
 *         Return Native.Function.Call(Of Boolean)(Hash.IS_VEHICLE_IN_BURNOUT, vh)  
 *     End Function  
 * ```
 * @param vehicle - 
 * @returns number
 * @remarks Hash: 0x1297A88E081430EB
 */
export function isVehicleInBurnout(vehicle: number): number;

/**
 * ```
 * Returns -1 if the vehicle has no livery  
 * ```
 * @param vehicle - 
 * @returns number
 * @remarks Hash: 0x87B63E25A529D526
 */
export function getVehicleLiveryCount(vehicle: number): number;

/**
 * ```
 * Second Param = LiveryIndex  
 * example   
 * int count = VEHICLE::GET_VEHICLE_LIVERY_COUNT(veh);  
 * for (int i = 0; i < count; i++)    
 * 	{  
 * char* LiveryName = VEHICLE::GET_LIVERY_NAME(veh, i);  
 * 	}  
 * this example will work fine to fetch all names   
 * for example for Sanchez we get   
 * SANC_LV1  
 * SANC_LV2  
 * SANC_LV3  
 * SANC_LV4  
 * SANC_LV5  
 * Use _GET_LABEL_TEXT, to get the localized livery name.  
 * ```
 * 
 * NOTE: You may need to set the vehicle's modKit to 0 by using this function [SET_VEHICLE_MOD_KIT](#_0x1F2AA07F00B3217A) before getting the name, otherwise this native may return NULL.
 * @param vehicle - 
 * @param liveryIndex - 
 * @returns string
 * @remarks Hash: 0xB4C7A93837C91A1F
 */
export function getLiveryName(vehicle: number, liveryIndex: number): string;

/**
 * ## Parameters
 * *
 * @param vehicle - 
 * @returns void
 * @remarks Hash: 0xB8FF7AB45305C345
 */
export function startVehicleAlarm(vehicle: number): void;

/**
 * ## Parameters
 * *
 * @param vehicle - 
 * @param r - 
 * @param g - 
 * @param b - 
 * @returns void
 * @remarks Hash: 0xB635392A4938B3C3
 */
export function getVehicleTyreSmokeColor(vehicle: number, r: number, g: number, b: number): void;

/**
 * **Usage:**
 * 
 * - Use this native inside a looped function.
 * - Values:
 *   - `0.0` = no vehicles on streets
 *   - `1.0` = normal vehicles on streets
 * 
 * `1.0` Seems to be the maximum.
 * @param multiplier - 
 * @returns void
 * @remarks Hash: 0x245A6883D966D537
 */
export function setVehicleDensityMultiplierThisFrame(multiplier: number): void;

/**
 * ## Parameters
 * *
 * @param vehicle - 
 * @returns void
 * @remarks Hash: 0x8879EE09268305D5
 */
export function unpausePlaybackRecordedVehicle(vehicle: number): void;

/**
 * Enables or disables a vehicle mod by index (`modType`) for a given vehicle.  
 * 
 * `eVehicleModType` enum, used for `modType` index can be found under [`SET_VEHICLE_MOD`](#_0x6AF0636DDEDCB6DD).
 * @param vehicle - 
 * @param modType - 
 * @param toggle - 
 * @returns void
 * @remarks Hash: 0x2A1F4F37F95BAD08
 */
export function toggleVehicleMod(vehicle: number, modType: number, toggle: number): void;

/**
 * To reset the max speed, set the `speed` value to `0.0` or lower.
 * @param vehicle - 
 * @param speed - 
 * @returns void
 * @remarks Hash: 0xBAA045B4E42F3C06
 */
export function SetVehicleMaxSpeed(vehicle: number, speed: number): void;

/**
 * ## Parameters
 * *
 * @param vehicle - 
 * @param p1 - 
 * @returns void
 * @remarks Hash: 0x2311DD7159F00582
 */
export function 0x2311dd7159f00582(vehicle: number, p1: number): void;

/**
 * ```
 * This is not tested - it's just an assumption.  
 * Doesn't seem to work.  I'll try with an int instead. --JT  
 * Read the scripts, im dumpass.   
 * Doesn't work at all, wether with an bool neither an int  
 *                             if (!VEHICLE::IS_TAXI_LIGHT_ON(l_115)) {  
 *                                 VEHICLE::SET_TAXI_LIGHTS(l_115, 1);  
 *                             }  
 * ```
 * @param vehicle - 
 * @param state - 
 * @returns void
 * @remarks Hash: 0x598803E85E8448D9
 */
export function setTaxiLights(vehicle: number, state: number): void;

/**
 * Fix a given vehicle.
 * If the vehicle's engine's broken then you cannot fix it with this native.
 * @param vehicle - 
 * @returns void
 * @remarks Hash: 0x115722B1B9C14C1C
 */
export function setVehicleFixed(vehicle: number): void;

/**
 * ## Parameters
 * *
 * @param vehicle - 
 * @param livery - 
 * @returns void
 * @remarks Hash: 0x60BF608F1B8CD1B6
 */
export function setVehicleLivery(vehicle: number, livery: number): void;

/**
 * ```
 * Explodes a selected vehicle.  
 * Vehicle vehicle = Vehicle you want to explode.  
 * BOOL isAudible = If explosion makes a sound.  
 * BOOL isInvisible = If the explosion is invisible or not.  
 * First BOOL does not give any visual explosion, the vehicle just falls apart completely but slowly and starts to burn.  
 * ```
 * @param vehicle - 
 * @param isAudible - 
 * @param isInvisible - 
 * @returns void
 * @remarks Hash: 0xBA71116ADF5B514C
 */
export function explodeVehicle(vehicle: number, isAudible: number, isInvisible: number): void;

/**
 * ## Parameters
 * *
 * @param p0 - 
 * @param p1 - 
 * @param p2 - 
 * @param p3 - 
 * @returns void
 * @remarks Hash: 0xFAF2A78061FD9EF4
 */
export function 0xfaf2a78061fd9ef4(p0: any, p1: number, p2: number, p3: number): void;

/**
 * ## Parameters
 * *
 * @param model - 
 * @returns number
 * @remarks Hash: 0xA0948AB42D7BA0DE
 */
export function isThisModelAPlane(model: number): number;

/**
 * ## Parameters
 * *
 * @param toggle - 
 * @returns void
 * @remarks Hash: 0x2AFD795EEAC8D30D
 */
export function setGarbageTrucks(toggle: number): void;

/**
 * ```
 * p2 often set to 1000.0 in the decompiled scripts.  
 * ```
 * @param vehicle - 
 * @param value - 
 * @returns void
 * @remarks Hash: 0xB77D05AC8C78AADB
 */
export function setVehicleBodyHealth(vehicle: number, value: number): void;

/**
 * ## Return value
 * @returns number
 * @remarks Hash: 0x6ADAABD3068C5235
 */
export function hasVehiclePhoneExplosiveDevice(): number;

/**
 * ## Parameters
 * *
 * @param handler - 
 * @param container - 
 * @returns number
 * @remarks Hash: 0x89D630CF5EA96D23
 */
export function IsHandlerFrameAboveContainer(handler: number, container: number): number;

/**
 * ## Parameters
 * *
 * @param cargobob - 
 * @param p1 - 
 * @returns void
 * @remarks Hash: 0x66979ACF5102FD2F
 */
export function setCargobobPickupMagnetReducedFalloff(cargobob: number, p1: number): void;

/**
 * ## Parameters
 * *
 * @param speedzone - 
 * @returns number
 * @remarks Hash: 0x1033371FC8E842A7
 */
export function removeRoadNodeSpeedZone(speedzone: number): number;

/**
 * Creates a vehicle with the specified model at the specified position. This vehicle will initially be owned by the creating
 * script as a mission entity, and the model should be loaded already (e.g. using REQUEST_MODEL).
 * 
 * ```
 * NativeDB Added Parameter 8: BOOL p7
 * ```
 * @param modelHash - 
 * @param x - 
 * @param y - 
 * @param z - 
 * @param heading - 
 * @param isNetwork - 
 * @param netMissionEntity - 
 * @returns number
 * @remarks Hash: 0xAF35D0D2583051B0
 */
export function createVehicle(modelHash: number, x: number, y: number, z: number, heading: number, isNetwork: number, netMissionEntity: number): number;

/**
 * ```
 * Only called once in the decompiled scripts. Presumably activates the specified generator.  
 * ```
 * @param vehicleGenerator - 
 * @param enabled - 
 * @returns void
 * @remarks Hash: 0xD9D620E0AC6DC4B0
 */
export function setScriptVehicleGenerator(vehicleGenerator: number, enabled: number): void;

/**
 * ## Parameters
 * *
 * @param p0 - 
 * @returns void
 * @remarks Hash: 0x9D30687C57BAA0BB
 */
export function 0x9d30687c57baa0bb(p0: any): void;

/**
 * ```
 * Can be used for IS_DLC_VEHICLE_MOD and _0xC098810437312FFF
 * ```
 * @param vehicle - 
 * @param modType - 
 * @param modIndex - 
 * @returns number
 * @remarks Hash: 0x4593CF82AA179706
 */
export function getVehicleModIdentifierHash(vehicle: number, modType: number, modIndex: number): number;

/**
 * ```
 * Sets how much the crane on the tow truck is raised, where 0.0 is fully lowered and 1.0 is fully raised.  
 * ```
 * @param vehicle - 
 * @param position - 
 * @returns void
 * @remarks Hash: 0xFE54B92A344583CA
 */
export function setVehicleTowTruckArmPosition(vehicle: number, position: number): void;

/**
 * ```
 * RESET_*
 * 
 * Resets the effect of 0x428AD3E26C8D9EB0
 * ```
 * @returns void
 * @remarks Hash: 0xE2F53F172B45EDE1
 */
export function 0xe2f53f172b45ede1(): void;

/**
 * Sets whether a boat should remain in the non-physical, low LOD anchor mode even when a player is driving it.
 * @param boat - 
 * @param toggle - 
 * @returns void
 * @remarks Hash: 0xB28B1FE5BFADD7F5
 */
export function setForceLowLodAnchorMode(boat: number, toggle: number): void;

/**
 * ## Parameters
 * *
 * @param vehicle - 
 * @returns number
 * @remarks Hash: 0xAD7E85FC227197C4
 */
export function getVehicleMaxBraking(vehicle: number): number;

/**
 * ## Parameters
 * *
 * @param vehicle - 
 * @param toggle - 
 * @returns void
 * @remarks Hash: 0x37C8252A7C92D017
 */
export function setDisableVehiclePetrolTankDamage(vehicle: number, toggle: number): void;

/**
 * Sets the selected vehicle's colors to their default value (specific variant specified using the colorCombination parameter).
 * 
 * Range of possible values for colorCombination is currently unknown, I couldn't find where these values are stored either (Disquse's guess was vehicles.meta but I haven't seen it in there.)
 * @param vehicle - 
 * @param colorCombination - 
 * @returns void
 * @remarks Hash: 0x33E8CD3322E2FE31
 */
export function setVehicleColourCombination(vehicle: number, colorCombination: number): void;

/**
 * Removes a scripted vehicle generator.
 * @param vehicleGenerator - 
 * @returns void
 * @remarks Hash: 0x22102C9ABFCF125D
 */
export function deleteScriptVehicleGenerator(vehicleGenerator: number): void;

/**
 * ## Parameters
 * *
 * @param p0 - 
 * @param p1 - 
 * @returns void
 * @remarks Hash: 0x72BECCF4B829522E
 */
export function 0x72beccf4b829522e(p0: any, p1: any): void;

/**
 * ## Parameters
 * *
 * @param vehicle - 
 * @param owned - 
 * @returns void
 * @remarks Hash: 0x2B5F9D2AF1F1722D
 */
export function setVehicleHasBeenOwnedByPlayer(vehicle: number, owned: number): void;

/**
 * ## Parameters
 * *
 * @param p0 - 
 * @returns void
 * @remarks Hash: 0xCF9159024555488C
 */
export function 0xcf9159024555488c(p0: any): void;

/**
 * ```
 * NativeDB Introduced: 3095
 * ```
 * 
 * Resets or clears the nitrous system for a specified vehicle. You can check if a vehicle has nitrous with [`IS_NITROUS_ACTIVE`](#_0x491E822B2C464FE4)
 * @param vehicle - 
 * @returns void
 * @remarks Hash: 0xC889AE921400E1ED
 */
export function clearNitrous(vehicle: number): void;

/**
 * Returns whether the specified vehicle is designated as a mercenary vehicle
 * @param vehicle - 
 * @returns number
 * @remarks Hash: 0xD4C4642CB7F50B5D
 */
export function getVehicleIsMercenary(vehicle: number): number;

/**
 * ## Parameters
 * *
 * @param vehicle - 
 * @param toggle - 
 * @returns void
 * @remarks Hash: 0xBE5C1255A1830FF5
 */
export function 0xbe5c1255a1830ff5(vehicle: number, toggle: number): void;

/**
 * ## Parameters
 * *
 * @param vehicle - 
 * @param p1 - 
 * @returns void
 * @remarks Hash: 0x0A6A279F3AA4FD70
 */
export function setBoatDisableAvoidance(vehicle: number, p1: number): void;

/**
 * ## Parameters
 * *
 * @param p0 - 
 * @param p1 - 
 * @param p2 - 
 * @param p3 - 
 * @param p4 - 
 * @param p5 - 
 * @returns void
 * @remarks Hash: 0x0581730AB9380412
 */
export function 0x0581730ab9380412(p0: any, p1: any, p2: any, p3: any, p4: any, p5: any): void;

/**
 * ## Parameters
 * *
 * @param plane - 
 * @param health - 
 * @returns void
 * @remarks Hash: 0x4C815EB175086F84
 */
export function SetPlanePropellersHealth(plane: number, health: number): void;

/**
 * ```
 * Does nothing. It's a nullsub.
 * 
 * NativeDB Introduced: v1604
 * ```
 * @param toggle - 
 * @returns void
 * @remarks Hash: 0x82E0AC411E41A5B4
 */
export function 0x82e0ac411e41a5b4(toggle: number): void;

/**
 * ## Return value
 * @returns number
 * @remarks Hash: 0xB2D06FAEDE65B577
 */
export function getLastDrivenVehicle(): number;

/**
 * The
 * @param vehicle - 
 * @param ped - 
 * @param index - 
 * @returns void
 * @remarks Hash: 0xB5C51B5502E85E83
 */
export function SetVehicleExclusiveDriver2(vehicle: number, ped: number, index: number): void;

/**
 * Retrieves the agility for a specific boat model, including any vehicle mods. Unlike other vehicles where Rockstar Games typically assess performance based on traction, boats use agility as a measure. This static value is distinct from the traction metrics used for other vehicle types.
 * 
 * ```
 * NativeDB Introduced: v323
 * ```
 * @param modelHash - 
 * @returns number
 * @remarks Hash: 0x5AA3F878A178C4FC
 */
export function getBoatVehicleModelAgility(modelHash: number): number;

/**
 * ## Parameters
 * *
 * @param vehicle - 
 * @param p1 - 
 * @returns void
 * @remarks Hash: 0xB088E9A47AE6EDD5
 */
export function SetDisableSuperdummyMode(vehicle: number, p1: number): void;

/**
 * ## Parameters
 * *
 * @param vehicle - 
 * @returns number
 * @remarks Hash: 0xA916396DF4154EE3
 */
export function GetVehicleCanActivateParachute(vehicle: number): number;

/**
 * ```
 * Max 1000.  
 * At 0 the main rotor will stall.  
 * ```
 * @param vehicle - 
 * @returns number
 * @remarks Hash: 0xE4CB7541F413D2C5
 */
export function getHeliMainRotorHealth(vehicle: number): number;

/**
 * ```
 * SET_C*
 * ```
 * @param vehicle - 
 * @param toggle - 
 * @returns void
 * @remarks Hash: 0xB2E0C0D6922D31F2
 */
export function 0xb2e0c0d6922d31f2(vehicle: number, toggle: number): void;

/**
 * ```
 * garageName example "Michael - Beverly Hills"
 * ```
 * @param garageName - 
 * @param vehicle - 
 * @returns number
 * @remarks Hash: 0xCEE4490CD57BB3C2
 */
export function isVehicleInGarageArea(garageName: string, vehicle: number): number;

/**
 * ## Parameters
 * *
 * @param vehicle - 
 * @param angleRatio - 
 * @returns void
 * @remarks Hash: 0x30D779DE7C4F6DD3
 */
export function setVehicleFlightNozzlePosition(vehicle: number, angleRatio: number): void;

/**
 * ```
 * NativeDB Introduced: v2372
 * ```
 * @param vehicle - 
 * @returns number
 * @remarks Hash: 0x2F5A72430E78C8D3
 */
export function GetDriftTyresEnabled(vehicle: number): number;

/**
 * ```
 * NativeDB Added Parameter 2: Any p1
 * ```
 * @param player - 
 * @returns void
 * @remarks Hash: 0x821FDC827D6F4090
 */
export function SpecialAbilityActivate(player: any): void;

/**
 * ### Warning
 * This native will return `0` if the last vehicle the player was in was destroyed.
 * @returns number
 * @remarks Hash: 0xB6997A7EB3F5C8C0
 */
export function getPlayersLastVehicle(): number;

/**
 * Gets the ped for a specified player index.
 * @param playerId - 
 * @returns number
 * @remarks Hash: 0x43A66C31C68491C0
 */
export function getPlayerPed(playerId: number): number;

/**
 * ## Parameters
 * *
 * @param p0 - 
 * @returns any
 * @remarks Hash: 0x6E4361FF3E8CD7CA
 */
export function 0x6e4361ff3e8cd7ca(p0: any): any;

/**
 * ## Parameters
 * *
 * @param player - 
 * @param multiplier - 
 * @returns void
 * @remarks Hash: 0xDB89EF50FF25FCE9
 */
export function setPlayerNoiseMultiplier(player: number, multiplier: number): void;

/**
 * ## Parameters
 * *
 * @param player - 
 * @param distance - 
 * @returns void
 * @remarks Hash: 0xEFD79FA81DFBA9CB
 */
export function SetPlayerFallDistance(player: number, distance: number): void;

/**
 * ```
 * Achievements from 0-57
 * more achievements came with update 1.29 (freemode events update), I'd say that they now go to 60, but I'll need to check.
 * ```
 * @param achievement - 
 * @returns number
 * @remarks Hash: 0xBEC7076D64130195
 */
export function giveAchievementToPlayer(achievement: number): number;

/**
 * ```
 * This can be between 1.0f - 14.9f   
 * You can change the max in IDA from 15.0. I say 15.0 as the function blrs if what you input is greater than or equal to 15.0 hence why it's 14.9 max default.  
 * On PC the multiplier can be between 0.0f and 50.0f (inclusive).  
 * ```
 * @param player - 
 * @param multiplier - 
 * @returns void
 * @remarks Hash: 0xCA7DC8329F0A1E9E
 */
export function setAirDragMultiplierForPlayersVehicle(player: number, multiplier: number): void;

/**
 * ## Parameters
 * *
 * @returns void
 * @remarks Hash: 0x5DC40A8869C22141
 */
export function setPlayerBluetoothState(): void;

/**
 * ## Parameters
 * *
 * @param player - 
 * @param modifier - 
 * @returns void
 * @remarks Hash: 0x2D83BC011CA14A3C
 */
export function setPlayerWeaponDefenseModifier(player: number, modifier: number): void;

/**
 * ```
 * Default is 100. Use player id and not ped id. For instance: PLAYER::SET_PLAYER_MAX_ARMOUR(PLAYER::PLAYER_ID(), 100); // main_persistent.ct4  
 * ```
 * @param player - 
 * @param value - 
 * @returns void
 * @remarks Hash: 0x77DFCCF5948B8C71
 */
export function setPlayerMaxArmour(player: number, value: number): void;

/**
 * ```
 * NativeDB Added Parameter 2: Any p1
 * ```
 * @param playerModel - 
 * @returns void
 * @remarks Hash: 0xF145F3BE2EFA9A3B
 */
export function specialAbilityUnlock(playerModel: number): void;

/**
 * ## Parameters
 * *
 * @param player - 
 * @param entity - 
 * @returns number
 * @remarks Hash: 0x7912F7FC4F6264B6
 */
export function isPlayerTargettingEntity(player: number, entity: number): number;

/**
 * Sets whether the player is able to do drive-bys in vehicle (shooting & aiming in vehicles), this also includes middle finger taunts.
 * 
 * This is a toggle, it does not have to be ran every frame.
 * @param player - 
 * @param toggle - 
 * @returns void
 * @remarks Hash: 0x6E8834B52EC20C77
 */
export function setPlayerCanDoDriveBy(player: number, toggle: number): void;

/**
 * ## Parameters
 * *
 * @param player - 
 * @param toggle - 
 * @returns void
 * @remarks Hash: 0x2E8AABFA40A84F8C
 */
export function setDisableAmbientMeleeMove(player: number, toggle: number): void;

/**
 * ```
 * Gets a value indicating whether the specified player is currently aiming freely.  
 * ```
 * @param player - 
 * @returns number
 * @remarks Hash: 0x2E397FD2ECD37C87
 */
export function isPlayerFreeAiming(player: number): number;

/**
 * ```
 * Tints:  
 * None = -1,  
 * Rainbow = 0,  
 * Red = 1,  
 * SeasideStripes = 2,  
 * WidowMaker = 3,  
 * Patriot = 4,  
 * Blue = 5,  
 * Black = 6,  
 * Hornet = 7,  
 * AirFocce = 8,  
 * Desert = 9,  
 * Shadow = 10,  
 * HighAltitude = 11,  
 * Airbone = 12,  
 * Sunrise = 13,  
 * ```
 * @param player - 
 * @param index - 
 * @returns void
 * @remarks Hash: 0xD5A016BC3C09CF40
 */
export function getPlayerReserveParachuteTintIndex(player: number, index: number): void;

/**
 * ```
 * NativeDB Added Parameter 2: Any p1
 * ```
 * @param playerModel - 
 * @returns void
 * @remarks Hash: 0x6A09D0D590A47D13
 */
export function specialAbilityLock(playerModel: number): void;

/**
 * ```
 * Returns the group ID the player is member of.  
 * ```
 * @param player - 
 * @returns number
 * @remarks Hash: 0x0D127585F77030AF
 */
export function getPlayerGroup(player: number): number;

/**
 * ## Parameters
 * *
 * @param player - 
 * @returns number
 * @remarks Hash: 0xD559D2BE9E37853B
 */
export function getTimeSincePlayerDroveOnPavement(player: number): number;

/**
 * Returns the players name from a specified player index
 * @param player - 
 * @returns string
 * @remarks Hash: 0x6D0DE6A7B5DA71F8
 */
export function getPlayerName(player: number): string;

/**
 * ## Parameters
 * *
 * @param p0 - 
 * @returns any
 * @remarks Hash: 0x7E07C78925D5FD96
 */
export function 0x7e07c78925d5fd96(p0: any): any;

/**
 * ```c
 * enum eViolationType {
 *   // Checks if the player is driving on pedestrians walk ways
 *   VT_PAVED_PEDESTRIAN_AREAS = 0,
 *   // Checks if the player is running through red lights
 *   // This takes some time to return true.
 *   VT_RUNNING_REDS = 1,
 *   // checks if the player is driving on the wrong side of the road
 *   VT_AGAINST_TRAFFIC = 2
 * };
 * ```
 * 
 * Used solely in "Al Di Napoli" with type 2 for a voiceline.
 * @param player - 
 * @param type - 
 * @returns number
 * @remarks Hash: 0xF10B44FD479D69F3
 */
export function IsPlayerDrivingDangerously(player: number, type: number): number;

/**
 * ## Parameters
 * *
 * @param player - 
 * @param toggle - 
 * @returns void
 * @remarks Hash: 0x6BC97F4F4BB3C04B
 */
export function SetPlayerInvincibleKeepRagdollEnabled(player: number, toggle: number): void;

/**
 * ```
 * Returns the Player's Invincible status.  
 * This function will always return false if 0x733A643B5B0C53C1 is used to set the invincibility status. To always get the correct result, use this:  
 * 	bool IsPlayerInvincible(Player player)  
 * 	{  
 * auto addr = getScriptHandleBaseAddress(GET_PLAYER_PED(player));	  
 * if (addr)  
 * {  
 * 	DWORD flag = *(DWORD *)(addr + 0x188);  
 * 	return ((flag & (1 << 8)) != 0) || ((flag & (1 << 9)) != 0);  
 * }  
 * return false;  
 * 	}  
 * ============================================================  
 * This has bothered me for too long, whoever may come across this, where did anyone ever come up with this made up hash? 0x733A643B5B0C53C1 I've looked all over old hash list, and this nativedb I can not find that PC hash anywhere. What native name is it now or was it?  
 * ```
 * @param player - 
 * @returns number
 * @remarks Hash: 0xB721981B2B939E07
 */
export function getPlayerInvincible(player: number): number;

/**
 * ```
 * NativeDB Added Parameter 3: BOOL p2
 * ```
 * @param player - 
 * @param modifier - 
 * @returns void
 * @remarks Hash: 0x4A3DC7ECCC321032
 */
export function setPlayerMeleeWeaponDamageModifier(player: number, modifier: number): void;

/**
 * ## Parameters
 * *
 * @param multiplier - 
 * @returns void
 * @remarks Hash: 0x020E5F00CDA207BA
 */
export function setWantedLevelMultiplier(multiplier: number): void;

/**
 * ```
 * Does exactly the same thing as PLAYER_ID()  
 * ```
 * @returns number
 * @remarks Hash: 0xEE68096F9F37341E
 */
export function networkPlayerIdToInt(): number;

/**
 * ```
 * Returns the time since the character was arrested in (ms) milliseconds.  
 * example  
 * var time = Function.call<int>(Hash.GET_TIME_SINCE_LAST_ARREST();  
 * UI.DrawSubtitle(time.ToString());  
 * if player has not been arrested, the int returned will be -1.  
 * ```
 * @returns number
 * @remarks Hash: 0x5063F92F07C2A316
 */
export function getTimeSinceLastArrest(): number;

/**
 * @returns void
 * @remarks Hash: 0xAEBF081FFC0A0E5E
 */
export function assistedMovementCloseRoute(): void;

/**
 * ## Parameters
 * *
 * @param player - 
 * @returns number
 * @remarks Hash: 0x92659B4CE1863CB3
 */
export function getPlayerMaxArmour(player: number): number;

/**
 * ## Parameters
 * *
 * @param player - 
 * @returns { x: number, y: number, z: number }
 * @remarks Hash: 0x0C92BA89F1AF26F8
 */
export function getPlayerWantedCentrePosition(player: number): { x: number, y: number, z: number };

/**
 * ADD_*
 * 
 * ```
 * NativeDB Introduced: v1868
 * ```
 * @param player - 
 * @param entity - 
 * @returns void
 * @remarks Hash: 0x9097EB6D4BB9A12A
 */
export function 0x9097eb6d4bb9a12a(player: number, entity: number): void;

/**
 * ```
 * Returns true if an unk value is greater than 0.0f  
 * ```
 * @param player - 
 * @returns number
 * @remarks Hash: 0x38D28DA81E4E9BF9
 */
export function isPlayerBattleAware(player: number): number;

/**
 * ## Parameters
 * *
 * @param player - 
 * @returns number
 * @remarks Hash: 0x78CFE51896B6B8A4
 */
export function isPlayerTargettingAnything(player: number): number;

/**
 * ## Parameters
 * *
 * @param player - 
 * @returns void
 * @remarks Hash: 0x10C54E4389C12B42
 */
export function clearPlayerParachutePackModelOverride(player: number): void;

/**
 * ## Parameters
 * *
 * @param player - 
 * @param r - 
 * @param g - 
 * @param b - 
 * @returns void
 * @remarks Hash: 0xEF56DBABD3CD4887
 */
export function getPlayerParachuteSmokeTrailColor(player: number, r: number, g: number, b: number): void;

/**
 * ## Parameters
 * *
 * @param player - 
 * @returns number
 * @remarks Hash: 0x5DDFE2FF727F3CA3
 */
export function getPlayerHasReserveParachute(player: number): number;

/**
 * ## Parameters
 * *
 * @param player - 
 * @returns number
 * @remarks Hash: 0xE36A25322DC35F42
 */
export function getTimeSincePlayerHitPed(player: number): number;

/**
 * Returns the player index for the local player.
 * @returns number
 * @remarks Hash: 0x4F8644AF03D0E0D6
 */
export function playerId(): number;

/**
 * ```
 * Returns TRUE if it found an entity in your crosshair within range of your weapon. Assigns the handle of the target to the *entity that you pass it.  
 * Returns false if no entity found.  
 * ```
 * @param player - 
 * @param entity - 
 * @returns number
 * @remarks Hash: 0x2975C866E6713290
 */
export function getEntityPlayerIsFreeAimingAt(player: number, entity: number): number;

/**
 * ## Parameters
 * *
 * @param player - 
 * @returns number
 * @remarks Hash: 0x20CE80B0C2BF4ACC
 */
export function hasPlayerDamagedAtLeastOnePed(player: number): number;

/**
 * ## Return value
 * @returns number
 * @remarks Hash: 0x9A41CF4674A12272
 */
export function getCauseOfMostRecentForceCleanup(): number;

/**
 * Establishes a reset flag to prevent the player from entering any vehicle. Not that this native must be called every frame.
 * @param player - 
 * @returns void
 * @remarks Hash: 0x1DE37BBF9E9CC14A
 */
export function setPlayerMayNotEnterAnyVehicle(player: number): void;

/**
 * ```
 * Affects the range of auto aim target.  
 * ```
 * @param player - 
 * @param range - 
 * @returns void
 * @remarks Hash: 0x29961D490E5814FD
 */
export function setPlayerLockonRangeOverride(player: number, range: number): void;

/**
 * ```
 * NativeDB Introduced: v2372
 * ```
 * @param player - 
 * @returns void
 * @remarks Hash: 0x290D248E25815AE8
 */
export function ClearPlayerReserveParachuteModelOverride(player: number): void;

/**
 * Make the player impervious to all forms of damage.
 * @param player - 
 * @param bInvincible - 
 * @returns void
 * @remarks Hash: 0x239528EACDC3E7DE
 */
export function setPlayerInvincible(player: number, bInvincible: number): void;

/**
 * ```
 * 6 matches across 4 scripts. 5 occurrences were 240. The other was 255.  
 * ```
 * @param value - 
 * @returns void
 * @remarks Hash: 0x14D913B777DFF5DA
 */
export function setPlayerClothLockCounter(value: number): void;

/**
 * ```
 * Also known as _RECHARGE_SPECIAL_ABILITY
 * ```
 * 
 * ```
 * NativeDB Added Parameter 3: Any p2
 * ```
 * @param player - 
 * @param p1 - 
 * @returns void
 * @remarks Hash: 0x3DACA8DDC6FD4980
 */
export function specialAbilityFillMeter(player: number, p1: number): void;

/**
 * ```
 * Seems to only appear in scripts used in Singleplayer.  
 * Always used like this in scripts  
 * PLAYER::_BC9490CA15AEA8FB(PLAYER::PLAYER_ID());  
 * ```
 * @param player - 
 * @returns void
 * @remarks Hash: 0xBC9490CA15AEA8FB
 */
export function 0xbc9490ca15aea8fb(player: number): void;

/**
 * ## Parameters
 * *
 * @param player - 
 * @returns void
 * @remarks Hash: 0x8753997EB5F6EE3F
 */
export function clearPlayerParachuteModelOverride(player: number): void;

/**
 * For Steam.
 * Does nothing and always returns false in the retail version of the game.
 * @param achievement - 
 * @param progress - 
 * @returns number
 * @remarks Hash: 0xC2AFFFDABBDC2C5C
 */
export function SetAchievementProgress(achievement: number, progress: number): number;

/**
 * ## Parameters
 * *
 * @param player - 
 * @returns void
 * @remarks Hash: 0xDC64D2C53493ED12
 */
export function reportPoliceSpottedPlayer(player: number): void;

/**
 * REMOVE_*
 * 
 * ```
 * NativeDB Introduced: v1868
 * ```
 * @param player - 
 * @param entity - 
 * @returns void
 * @remarks Hash: 0x9F260BFB59ADBCA3
 */
export function 0x9f260bfb59adbca3(player: number, entity: number): void;

/**
 * ```
 * NativeDB Introduced: v323
 * ```
 * @returns void
 * @remarks Hash: 0xDA1DF03D5A315F4E
 */
export function resetWorldBoundaryForPlayer(): void;

/**
 * ```
 * Sets whether this player can be hassled by gangs.  
 * ```
 * @param player - 
 * @param toggle - 
 * @returns void
 * @remarks Hash: 0xD5E460AD7020A246
 */
export function setPlayerCanBeHassledByGangs(player: number, toggle: number): void;

/**
 * ```
 * PLAYER::REPORT_CRIME(PLAYER::PLAYER_ID(), 37, PLAYER::GET_WANTED_LEVEL_THRESHOLD(1));  
 * From am_armybase.ysc.c4:  
 * PLAYER::REPORT_CRIME(PLAYER::PLAYER_ID(4), 36, PLAYER::GET_WANTED_LEVEL_THRESHOLD(4));
 * @param player - 
 * @param crimeType - 
 * @param wantedLvlThresh - 
 * @returns void
 * @remarks Hash: 0xE9B09589827545E7
 */
export function reportCrime(player: number, crimeType: number, wantedLvlThresh: number): void;

/**
 * ```
 * example:  
 * PLAYER::SET_PLAYER_PARACHUTE_MODEL_OVERRIDE(PLAYER::PLAYER_ID(), 0x73268708);  
 * ```
 * @param player - 
 * @param model - 
 * @returns void
 * @remarks Hash: 0x977DB4641F6FC3DB
 */
export function setPlayerParachuteModelOverride(player: number, model: number): void;

/**
 * ## Parameters
 * *
 * @param player - 
 * @param ped - 
 * @returns number
 * @remarks Hash: 0xF297383AA91DCA29
 */
export function canPedHearPlayer(player: number, ped: number): number;

/**
 * ## Parameters
 * *
 * @param player - 
 * @returns void
 * @remarks Hash: 0x7DDAB28D31FAC363
 */
export function setPlayerHasReserveParachute(player: number): void;

/**
 * ## Parameters
 * *
 * @param player - 
 * @returns number
 * @remarks Hash: 0x3F9F16F8E65A7ED7
 */
export function getPlayerSprintStaminaRemaining(player: number): number;

/**
 * ```
 * NativeDB Added Parameter 2: Any p1
 * ```
 * @param p0 - 
 * @returns void
 * @remarks Hash: 0x17F7471EACA78290
 */
export function SpecialAbilityDeplete(p0: any): void;

/**
 * ## Parameters
 * *
 * @param player - 
 * @param toggle - 
 * @returns void
 * @remarks Hash: 0x8EEDA153AD141BA4
 */
export function setEveryoneIgnorePlayer(player: number, toggle: number): void;

/**
 * ```
 * NativeDB Introduced: v2372
 * ```
 * @param player - 
 * @param model - 
 * @returns void
 * @remarks Hash: 0x0764486AEDE748DB
 */
export function SetPlayerReserveParachuteModelOverride(player: number, model: number): void;

/**
 * ```
 * Remnant from GTA IV. Does nothing in GTA V.
 * ```
 * @param player - 
 * @returns number
 * @remarks Hash: 0x085DEB493BE80812
 */
export function getWantedLevelRadius(player: number): number;

/**
 * ## Parameters
 * *
 * @param player - 
 * @param value - 
 * @returns void
 * @remarks Hash: 0x4E9021C1FCDD507A
 */
export function setPlayerStealthPerceptionModifier(player: number, value: number): void;

/**
 * Set the player's current team.
 * @param player - 
 * @param team - 
 * @returns void
 * @remarks Hash: 0x0299FA38396A4940
 */
export function setPlayerTeam(player: number, team: number): void;

/**
 * ```
 * p1 was always true.
 * ```
 * 
 * ```
 * NativeDB Added Parameter 3: Any p2
 * ```
 * @param player - 
 * @param p1 - 
 * @returns void
 * @remarks Hash: 0x1D506DBBBC51E64B
 */
export function specialAbilityDepleteMeter(player: number, p1: number): void;

/**
 * Set the model for a specific Player. Note that this will destroy the current Ped for the Player and create a new one, any reference to the old ped will be invalid after calling this.
 * 
 * As per usual, make sure to request the model first and wait until it has loaded.
 * @param player - 
 * @param model - 
 * @returns void
 * @remarks Hash: 0x00A1CADD00108836
 */
export function setPlayerModel(player: number, model: number): void;

/**
 * ## Parameters
 * *
 * @param player - 
 * @returns number
 * @remarks Hash: 0xE28E54788CE8F12D
 */
export function getPlayerWantedLevel(player: number): number;

/**
 * ```
 * This has been found in use in the decompiled files.  
 * ```
 * @param player - 
 * @returns void
 * @remarks Hash: 0xAD73CE5A09E42D12
 */
export function 0xad73ce5a09e42d12(player: number): void;

/**
 * ```
 * NativeDB Added Parameter 3: Any p2
 * ```
 * @param player - 
 * @param p1 - 
 * @returns void
 * @remarks Hash: 0xB214D570EAD7F81A
 */
export function SetSpecialAbility(player: number, p1: number): void;

/**
 * ```
 * Sets your targeting mode.
 * 0 = Assisted Aim - Full
 * 1 = Assisted Aim - Partial
 * 2 = Free Aim - Assisted
 * 3 = Free Aim
 * ```
 * @param targetMode - 
 * @returns void
 * @remarks Hash: 0xB1906895227793F3
 */
export function setPlayerTargetingMode(targetMode: number): void;

/**
 * ```
 * Used with radios:
 * void sub_cf383(auto _a0) {
 *     if ((a_0)==1) {
 *         if (MISC::IS_BIT_SET((g_240005._f1), 3)) {
 *             PLAYER::_2F7CEB6520288061(0);
 *             AUDIO::SET_AUDIO_FLAG("AllowRadioDuringSwitch", 0);
 *             AUDIO::SET_MOBILE_PHONE_RADIO_STATE(0);
 *             AUDIO::SET_AUDIO_FLAG("MobileRadioInGame", 0);
 *         }
 *         sub_cf3f6(1);
 *     } else {
 *         if (MISC::IS_BIT_SET((g_240005._f1), 3)) {
 *             PLAYER::_2F7CEB6520288061(1);
 *             AUDIO::SET_AUDIO_FLAG("AllowRadioDuringSwitch", 1);
 *             AUDIO::SET_MOBILE_PHONE_RADIO_STATE(1);
 *             AUDIO::SET_AUDIO_FLAG("MobileRadioInGame", 1);
 *         }
 *         sub_cf3f6(0);
 *     }
 * }
 * SET_PLAYER_S*
 * ```
 * @param p0 - 
 * @returns void
 * @remarks Hash: 0x2F7CEB6520288061
 */
export function 0x2f7ceb6520288061(p0: number): void;

/**
 * ```
 * Multiplier goes up to 1.49 any value above will be completely overruled by the game and the multiplier will not take effect, this can be edited in memory however.  
 * Just call it one time, it is not required to be called once every tick.  
 * Note: At least the IDA method if you change the max float multiplier from 1.5 it will change it for both this and SWIM above. I say 1.5 as the function blrs if what you input is greater than or equal to 1.5 hence why it's 1.49 max default.  
 * It is not possible to "decrease" speed. Anything below 1 will be ignored.  
 * ```
 * @param player - 
 * @param multiplier - 
 * @returns void
 * @remarks Hash: 0x6DB47AA77FD94E09
 */
export function setRunSprintMultiplierForPlayer(player: number, multiplier: number): void;

/**
 * ## Parameters
 * *
 * @param player - 
 * @returns number
 * @remarks Hash: 0xAFAF86043E5874E9
 */
export function arePlayerFlashingStarsAboutToDrop(player: number): number;

/**
 * ## Parameters
 * *
 * @param player - 
 * @returns void
 * @remarks Hash: 0x19531C47A2ABD691
 */
export function resetPlayerInputGait(player: number): void;

/**
 * Seems to lock the underwater timer of the specified player. Set `percentage` to `50.0` will reduce the value of [GET_PLAYER_UNDERWATER_TIME_REMAINING](#_0xA1FCF8E6AF40B731) to 5.0.
 * 
 * If you want to increase the underwater time for ped, use [SET_PED_MAX_TIME_UNDERWATER](#_0x6BA428C528D9E522) instead.
 * 
 * Using this native after [SET_PED_MAX_TIME_UNDERWATER](#_0x6BA428C528D9E522)
 * @param player - 
 * @param percentage - 
 * @returns any
 * @remarks Hash: 0xA0D3E4F7AAFB7E78
 */
export function SetPlayerUnderwaterTimeRemaining(player: number, percentage: number): any;

/**
 * ## Parameters
 * *
 * @param player - 
 * @returns number
 * @remarks Hash: 0x2F395D61F3A1F877
 */
export function getPlayerCurrentStealthNoise(player: number): number;

/**
 * ## Parameters
 * *
 * @param player - 
 * @returns void
 * @remarks Hash: 0xA6F312FCCE9C1DFE
 */
export function resetPlayerStamina(player: number): void;

/**
 * ## Parameters
 * *
 * @param player1 - 
 * @param player2 - 
 * @param toggle - 
 * @returns void
 * @remarks Hash: 0x55FCC0C390620314
 */
export function 0x55fcc0c390620314(player1: number, player2: number, toggle: number): void;

/**
 * ## Parameters
 * *
 * @param player - 
 * @param toggle - 
 * @returns void
 * @remarks Hash: 0x596976B02B6B5700
 */
export function setIgnoreLowPriorityShockingEvents(player: number, toggle: number): void;

/**
 * ```
 * this function is hard-coded to always return 0.  
 * ```
 * @returns number
 * @remarks Hash: 0x74556E1420867ECA
 */
export function isPlayerLoggingInNp(): number;

/**
 * ```
 * Values around 1.0f to 2.0f used in game scripts.  
 * ```
 * @param player - 
 * @param multiplier - 
 * @returns void
 * @remarks Hash: 0xB2C1A29588A9F47C
 */
export function setPlayerSneakingNoiseMultiplier(player: number, multiplier: number): void;

/**
 * ## Parameters
 * *
 * @param player - 
 * @param model - 
 * @returns void
 * @remarks Hash: 0xDC80A4C2F18A2B64
 */
export function setPlayerParachutePackModelOverride(player: number, model: number): void;

/**
 * ## Parameters
 * *
 * @param player - 
 * @returns number
 * @remarks Hash: 0xE23D5873C2394C61
 */
export function UpdatePlayerTeleport(player: number): number;

/**
 * ```
 * Appears only 3 times in the scripts, more specifically in michael1.ysc
 * -
 * This can be used to prevent dying if you are "out of the world"
 * ```
 * @param x - 
 * @param y - 
 * @param z - 
 * @returns void
 * @remarks Hash: 0x5006D96C995A5827
 */
export function extendWorldBoundaryForPlayer(x: number, y: number, z: number): void;

/**
 * ## Parameters
 * *
 * @param player - 
 * @param ped - 
 * @param b2 - 
 * @param resetDamage - 
 * @returns void
 * @remarks Hash: 0x048189FAC643DEEE
 */
export function changePlayerPed(player: number, ped: number, b2: number, resetDamage: number): void;

/**
 * ```
 * Tints:  
 * None = -1,  
 * Rainbow = 0,  
 * Red = 1,  
 * SeasideStripes = 2,  
 * WidowMaker = 3,  
 * Patriot = 4,  
 * Blue = 5,  
 * Black = 6,  
 * Hornet = 7,  
 * AirFocce = 8,  
 * Desert = 9,  
 * Shadow = 10,  
 * HighAltitude = 11,  
 * Airbone = 12,  
 * Sunrise = 13,  
 * ```
 * @param player - 
 * @param index - 
 * @returns void
 * @remarks Hash: 0xAF04C87F5DC1DF38
 */
export function setPlayerReserveParachuteTintIndex(player: number, index: number): void;

/**
 * ```
 * NativeDB Introduced: v2372
 * ```
 * @param player - 
 * @returns number
 * @remarks Hash: 0xC219887CA3E65C41
 */
export function GetPlayerParachuteModelOverride(player: number): number;

/**
 * ```
 * This executes at the same as speed as PLAYER::SET_PLAYER_WANTED_LEVEL(player, 0, false);  
 * PLAYER::GET_PLAYER_WANTED_LEVEL(player); executes in less than half the time. Which means that it's worth first checking if the wanted level needs to be cleared before clearing. However, this is mostly about good code practice and can important in other situations. The difference in time in this example is negligible.  
 * ```
 * @param player - 
 * @returns void
 * @remarks Hash: 0xB302540597885499
 */
export function clearPlayerWantedLevel(player: number): void;

/**
 * ## Parameters
 * *
 * @param playerModel - 
 * @returns number
 * @remarks Hash: 0xC6017F6A6CDFA694
 */
export function isSpecialAbilityUnlocked(playerModel: number): number;

/**
 * @returns void
 * @remarks Hash: 0x8621390F0CDCFE1F
 */
export function assistedMovementFlushRoute(): void;

/**
 * This is to make the player walk without accepting input.
 * 
 * Call this native every frame so you can control the direction of your ped.
 * @param player - 
 * @param amount - 
 * @param gaitType - 
 * @param rotationSpeed - 
 * @param p4 - 
 * @param p5 - 
 * @returns void
 * @remarks Hash: 0x477D5D63E63ECA5D
 */
export function simulatePlayerInputGait(player: number, amount: number, gaitType: number, rotationSpeed: number, p4: number, p5: number): void;

/**
 * ## Parameters
 * *
 * @param player - 
 * @param ms - 
 * @param p2 - 
 * @returns number
 * @remarks Hash: 0xBC0753C9CA14B506
 */
export function HasPlayerBeenShotByCop(player: number, ms: number, p2: number): number;

/**
 * ```
 * p1 appears as 5, 10, 15, 25, or 30. p2 is always true.
 * ```
 * 
 * ```
 * NativeDB Added Parameter 4: Any p3
 * ```
 * @param player - 
 * @param p1 - 
 * @param p2 - 
 * @returns void
 * @remarks Hash: 0xB7B0870EB531D08D
 */
export function specialAbilityChargeAbsolute(player: number, p1: number, p2: number): void;

/**
 * ```
 * Max value is 1.0  
 * ```
 * @param player - 
 * @param difficulty - 
 * @returns void
 * @remarks Hash: 0x9B0BB33B04405E7A
 */
export function setWantedLevelDifficulty(player: number, difficulty: number): void;

/**
 * ## Parameters
 * *
 * @param player - 
 * @param toggle - 
 * @returns void
 * @remarks Hash: 0xDB172424876553F4
 */
export function setDispatchCopsForPlayer(player: number, toggle: number): void;

/**
 * ## Parameters
 * *
 * @param player - 
 * @returns void
 * @remarks Hash: 0xFAC75988A7D078D3
 */
export function 0xfac75988a7d078d3(player: number): void;

/**
 * ## Parameters
 * *
 * @param player - 
 * @returns number
 * @remarks Hash: 0xDB89591E290D9182
 */
export function getTimeSincePlayerDroveAgainstTraffic(player: number): number;

/**
 * The native ensures the 'modifier' parameter is 0.1 or greater.
 * @param player - 
 * @param modifier - 
 * @returns void
 * @remarks Hash: 0xCE07B9F7817AADA3
 */
export function setPlayerWeaponDamageModifier(player: number, modifier: number): void;

/**
 * ## Parameters
 * *
 * @param multiplier - 
 * @returns void
 * @remarks Hash: 0xA49C426ED0CA4AB7
 */
export function setSpecialAbilityMultiplier(multiplier: number): void;

/**
 * ## Parameters
 * *
 * @param player - 
 * @returns void
 * @remarks Hash: 0x9EDD76E87D5D51BA
 */
export function 0x9edd76e87d5d51ba(player: number): void;

/**
 * ## Parameters
 * *
 * @param playerId - 
 * @returns number
 * @remarks Hash: 0x5FC472C501CCADB3
 */
export function getIsPlayerDrivingOnHighway(playerId: number): number;

/**
 * ```
 * NativeDB Introduced: v1290
 * ```
 * @param p0 - 
 * @returns void
 * @remarks Hash: 0x237440E46D918649
 */
export function 0x237440e46d918649(p0: any): void;

/**
 * ```
 * modifier's min value is 0.1
 * ```
 * @param player - 
 * @param modifier - 
 * @returns void
 * @remarks Hash: 0xAE540335B4ABC4E2
 */
export function setPlayerMeleeWeaponDefenseModifier(player: number, modifier: number): void;

/**
 * ## Parameters
 * *
 * @param player - 
 * @returns number
 * @remarks Hash: 0x0A6EB355EE14A2DB
 */
export function arePlayerStarsGreyedOut(player: number): number;

/**
 * Returns the entity handle for the local player ped. Note that this entity handle will change after using commands such as SET\_PLAYER\_MODEL.
 * @returns number
 * @remarks Hash: 0xD80958FC74E988A6
 */
export function playerPedId(): number;

/**
 * ```
 * PLAYER::FORCE_CLEANUP_FOR_ALL_THREADS_WITH_THIS_NAME("pb_prostitute", 1); // Found in decompilation  
 * ```
 * @param name - 
 * @param cleanupFlags - 
 * @returns void
 * @remarks Hash: 0x4C68DDDDF0097317
 */
export function forceCleanupForAllThreadsWithThisName(name: string, cleanupFlags: number): void;

/**
 * ## Parameters
 * *
 * @param duration - 
 * @returns void
 * @remarks Hash: 0xBF9BD71691857E48
 */
export function startFiringAmnesty(duration: number): void;

/**
 * ## Parameters
 * *
 * @param player - 
 * @returns number
 * @remarks Hash: 0xDE7465A27D403C06
 */
export function canPlayerStartMission(player: number): number;

/**
 * ```
 * Returns the time since the character died in (ms) milliseconds.  
 * example  
 * var time = Function.call<int>(Hash.GET_TIME_SINCE_LAST_DEATH();  
 * UI.DrawSubtitle(time.ToString());  
 * if player has not died, the int returned will be -1.  
 * ```
 * @returns number
 * @remarks Hash: 0xC7034807558DDFCA
 */
export function getTimeSinceLastDeath(): number;

/**
 * ## Parameters
 * *
 * @param player - 
 * @param toggle - 
 * @returns void
 * @remarks Hash: 0xFF300C7649724A0B
 */
export function setPlayerLeavePedBehind(player: number, toggle: number): void;

/**
 * ## Parameters
 * *
 * @param achievement - 
 * @returns number
 * @remarks Hash: 0x867365E111A3B6EB
 */
export function hasAchievementBeenPassed(achievement: number): number;

/**
 * ```
 * NativeDB Added Parameter 2: Any p1
 * ```
 * @param player - 
 * @returns number
 * @remarks Hash: 0xB1D200FE26AEF3CB
 */
export function isSpecialAbilityEnabled(player: number): number;

/**
 * ## Parameters
 * *
 * @param player - 
 * @returns number
 * @remarks Hash: 0xD705740BB0A1CF4C
 */
export function hasPlayerBeenSpottedInStolenVehicle(player: number): number;

/**
 * ```
 * NativeDB Introduced: v2372
 * ```
 * @param player - 
 * @returns number
 * @remarks Hash: 0x37FAAA68DCA9D08D
 */
export function GetPlayerReserveParachuteModelOverride(player: number): number;

/**
 * ## Parameters
 * *
 * @param player - 
 * @returns number
 * @remarks Hash: 0x424D4687FA1E5652
 */
export function isPlayerDead(player: number): number;

/**
 * ```
 * 2 occurrences in agency_heist3a. p1 was 0.7f then 0.4f.  
 * ```
 * @param player - 
 * @param p1 - 
 * @returns number
 * @remarks Hash: 0xDD2620B7B9D16FF1
 */
export function 0xdd2620b7b9d16ff1(player: number, p1: number): number;

/**
 * ```
 * Simply returns whatever is passed to it (Regardless of whether the handle is valid or not).  
 * ```
 * @param value - 
 * @returns number
 * @remarks Hash: 0x41BD2A6B006AF756
 */
export function intToPlayerindex(value: number): number;

/**
 * ## Parameters
 * *
 * @param maxWantedLevel - 
 * @returns void
 * @remarks Hash: 0xAA5F02DB48D704B9
 */
export function setMaxWantedLevel(maxWantedLevel: number): void;

/**
 * ## Parameters
 * *
 * @param player - 
 * @returns void
 * @remarks Hash: 0x0F4CC924CF8C7B21
 */
export function clearPlayerParachuteVariationOverride(player: number): void;

/**
 * ## Parameters
 * *
 * @param player - 
 * @returns number
 * @remarks Hash: 0xD55DDFB47991A294
 */
export function hasPlayerLeftTheWorld(player: number): number;

/**
 * ## Parameters
 * *
 * @param player - 
 * @param r - 
 * @param g - 
 * @param b - 
 * @returns void
 * @remarks Hash: 0xE902EF951DCE178F
 */
export function getPlayerRgbColour(player: number, r: number, g: number, b: number): void;

/**
 * ```
 * Gets the number of players in the current session.
 * If not multiplayer, always returns 1.
 * ```
 * @returns number
 * @remarks Hash: 0x407C7F91DDB46C16
 */
export function getNumberOfPlayers(): number;

/**
 * ## Parameters
 * *
 * @param p0 - 
 * @param p1 - 
 * @returns void
 * @remarks Hash: 0x2F41A3BAE005E5FA
 */
export function 0x2f41a3bae005e5fa(p0: any, p1: any): void;

/**
 * ```
 * Name between DISABLE_ALL_CONTROL_ACTIONS and DISABLE_CONTROL_ACTION
 * ```
 * @param player - 
 * @returns void
 * @remarks Hash: 0x5501B7A5CDB79D37
 */
export function 0x5501b7a5cdb79d37(player: number): void;

/**
 * ```
 * NativeDB Added Parameter 2: Any p1
 * ```
 * @param player - 
 * @returns number
 * @remarks Hash: 0x05A1FE504B7F2587
 */
export function isSpecialAbilityMeterFull(player: number): number;

/**
 * Always returns false.
 * 
 * ```
 * NativeDB Introduced: v1868
 * ```
 * @param player - 
 * @returns number
 * @remarks Hash: 0xDCC07526B8EC45AF
 */
export function 0xdcc07526b8ec45af(player: number): number;

/**
 * ## Parameters
 * *
 * @param player - 
 * @param wantedLevel - 
 * @param delayedResponse - 
 * @returns void
 * @remarks Hash: 0x39FF19C64EF7DA5B
 */
export function setPlayerWantedLevel(player: number, wantedLevel: number, delayedResponse: number): void;

/**
 * ```
 * Return true while player is being arrested / busted.  
 * If atArresting is set to 1, this function will return 1 when player is being arrested (while player is putting his hand up, but still have control)  
 * If atArresting is set to 0, this function will return 1 only when the busted screen is shown.  
 * ```
 * @param player - 
 * @param atArresting - 
 * @returns number
 * @remarks Hash: 0x388A47C51ABDAC8E
 */
export function isPlayerBeingArrested(player: number, atArresting: number): number;

/**
 * ## Parameters
 * *
 * @param player - 
 * @param toggle - 
 * @returns void
 * @remarks Hash: 0x9F343285A00B4BB6
 */
export function setAutoGiveParachuteWhenEnterPlane(player: number, toggle: number): void;

/**
 * ```
 * Returns true when the player is not able to control the cam i.e. when running a benchmark test, switching the player or viewing a cutscene.  
 * Note: I am not 100% sure if the native actually checks if the cam control is disabled but it seems promising.  
 * ```
 * @returns number
 * @remarks Hash: 0x7C814D2FB49F40C0
 */
export function IsPlayerCamControlDisabled(): number;

/**
 * ```
 * Purpose of the BOOL currently unknown.  
 * Both, true and false, work  
 * ```
 * @param unk - 
 * @returns void
 * @remarks Hash: 0x94DD7888C10A979E
 */
export function displaySystemSigninUi(unk: number): void;

/**
 * ```
 * Only 1 match. ob_sofa_michael.  
 * PLAYER::PLAYER_ATTACH_VIRTUAL_BOUND(-804.5928f, 173.1801f, 71.68436f, 0f, 0f, 0.590625f, 1f, 0.7f);1.0.335.2, 1.0.350.1/2, 1.0.372.2, 1.0.393.2, 1.0.393.4, 1.0.463.1;  
 * ```
 * @param p0 - 
 * @param p1 - 
 * @param p2 - 
 * @param p3 - 
 * @param p4 - 
 * @param p5 - 
 * @param p6 - 
 * @param p7 - 
 * @returns void
 * @remarks Hash: 0xED51733DC73AED51
 */
export function playerAttachVirtualBound(p0: number, p1: number, p2: number, p3: number, p4: number, p5: number, p6: number, p7: number): void;

/**
 * ```
 * normalizedValue is from 0.0 - 1.0
 * p2 is always 1
 * ```
 * 
 * ```
 * NativeDB Added Parameter 4: Any p3
 * ```
 * @param player - 
 * @param normalizedValue - 
 * @param p2 - 
 * @returns void
 * @remarks Hash: 0xA0696A65F009EE18
 */
export function specialAbilityChargeNormalized(player: number, normalizedValue: number, p2: number): void;

/**
 * ```
 * Disables something. Used only once in R* scripts (freemode.ysc).
 * DISABLE_PLAYER_*
 * ```
 * @returns void
 * @remarks Hash: 0xB885852C39CC265D
 */
export function 0xb885852c39cc265d(): void;

/**
 * ```
 * Tints:  
 * None = -1,  
 * Rainbow = 0,  
 * Red = 1,  
 * SeasideStripes = 2,  
 * WidowMaker = 3,  
 * Patriot = 4,  
 * Blue = 5,  
 * Black = 6,  
 * Hornet = 7,  
 * AirFocce = 8,  
 * Desert = 9,  
 * Shadow = 10,  
 * HighAltitude = 11,  
 * Airbone = 12,  
 * Sunrise = 13,  
 * ```
 * @param player - 
 * @param tintIndex - 
 * @returns void
 * @remarks Hash: 0x75D3F7A1B0D9B145
 */
export function getPlayerParachuteTintIndex(player: number, tintIndex: number): void;

/**
 * ```
 * This has been found in use in the decompiled files.  
 * ```
 * @param player - 
 * @returns void
 * @remarks Hash: 0x4669B3ED80F24B4E
 */
export function 0x4669b3ed80f24b4e(player: number): void;

/**
 * ```
 * modifier's min value is 0.1
 * ```
 * @param player - 
 * @param modifier - 
 * @returns void
 * @remarks Hash: 0x4C60E6EFDAFF2462
 */
export function setPlayerVehicleDefenseModifier(player: number, modifier: number): void;

/**
 * ## Parameters
 * *
 * @param player - 
 * @returns void
 * @remarks Hash: 0x36F1B38855F2A8DF
 */
export function 0x36f1b38855f2a8df(player: number): void;

/**
 * ```
 * Used to toggle the square up aim.
 * ```
 * @param player - 
 * @param toggle - 
 * @returns void
 * @remarks Hash: 0x5C8B2F450EE4328E
 */
export function setPlayerLockon(player: number, toggle: number): void;

/**
 * Disables vehicle rewards for the current frame.
 * @param player - 
 * @returns void
 * @remarks Hash: 0xC142BE3BB9CE125F
 */
export function disablePlayerVehicleRewards(player: number): void;

/**
 * ## Parameters
 * *
 * @param player - 
 * @returns number
 * @remarks Hash: 0xDCCFD3F106C36AB4
 */
export function isPlayerFreeForAmbientTask(player: number): number;

/**
 * ## Parameters
 * *
 * @param player - 
 * @returns number
 * @remarks Hash: 0xFA1E2BF8B10598F9
 */
export function isPlayerPressingHorn(player: number): number;

/**
 * ```
 * NativeDB Added Parameter 2: Any p1
 * ```
 * @param player - 
 * @returns void
 * @remarks Hash: 0x9CB5CE07A3968D5A
 */
export function specialAbilityDeactivateFast(player: number): void;

/**
 * ```
 * Drft  
 * ```
 * @param wantedLevel - 
 * @returns number
 * @remarks Hash: 0xFDD179EAF45B556C
 */
export function getWantedLevelThreshold(wantedLevel: number): number;

/**
 * ## Parameters
 * *
 * @param player - 
 * @param p1 - 
 * @returns void
 * @remarks Hash: 0xD821056B9ACF8052
 */
export function 0xd821056b9acf8052(player: number, p1: any): void;

/**
 * ```
 * Sets whether this player can take cover.
 * ```
 * @param player - 
 * @param toggle - 
 * @returns void
 * @remarks Hash: 0xD465A8599DFF6814
 */
export function setPlayerCanUseCover(player: number, toggle: number): void;

/**
 * ```
 * Returns profile setting 243.
 * GET_*
 * ```
 * @returns number
 * @remarks Hash: 0xCB645E85E97EA48B
 */
export function 0xcb645e85e97ea48b(): number;

/**
 * ```
 * Gets a value indicating whether the specified player is currently aiming freely at the specified entity.  
 * ```
 * @param player - 
 * @param entity - 
 * @returns number
 * @remarks Hash: 0x3C06B5C839B38F7B
 */
export function isPlayerFreeAimingAtEntity(player: number, entity: number): number;

/**
 * ```
 * 2 matches in 1 script - am_hold_up
 * Used in multiplayer scripts?
 * ```
 * @returns void
 * @remarks Hash: 0x0032A6DBA562C518
 */
export function 0x0032a6dba562c518(): void;

/**
 * ## Parameters
 * *
 * @param player - 
 * @param toggle - 
 * @returns void
 * @remarks Hash: 0xC54C95DA968EC5B5
 */
export function setPlayerSimulateAiming(player: number, toggle: number): void;

/**
 * ## Parameters
 * *
 * @param player - 
 * @param modifier - 
 * @returns void
 * @remarks Hash: 0xBCFDE9EDE4CF27DC
 */
export function SetPlayerWeaponDefenseModifier2(player: number, modifier: number): void;

/**
 * ```
 * Returns the same as PLAYER_ID and NETWORK_PLAYER_ID_TO_INT  
 * ```
 * @returns number
 * @remarks Hash: 0xA5EDC40EF369B48D
 */
export function getPlayerIndex(): number;

/**
 * ## Parameters
 * *
 * @param player - 
 * @param enabled - 
 * @returns void
 * @remarks Hash: 0xF401B182DBA8AF53
 */
export function setPlayerCanLeaveParachuteSmokeTrail(player: number, enabled: number): void;

/**
 * ## Parameters
 * *
 * @param p0 - 
 * @param p1 - 
 * @returns void
 * @remarks Hash: 0x2382AB11450AE7BA
 */
export function 0x2382ab11450ae7ba(p0: any, p1: any): void;

/**
 * ```
 * Found in "director_mode", "fm_bj_race_controler", "fm_deathmatch_controler", "fm_impromptu_dm_controler", "fm_race_controler", "gb_deathmatch".  
 * ```
 * @param player - 
 * @param p1 - 
 * @returns void
 * @remarks Hash: 0xCAC57395B151135F
 */
export function 0xcac57395b151135f(player: number, p1: number): void;

/**
 * ## Parameters
 * *
 * @param player - 
 * @returns void
 * @remarks Hash: 0x471D2FF42A94B4F2
 */
export function setAllRandomPedsFleeThisFrame(player: number): void;

/**
 * ## Parameters
 * *
 * @param player - 
 * @param r - 
 * @param g - 
 * @param b - 
 * @returns void
 * @remarks Hash: 0x8217FD371A4625CF
 */
export function setPlayerParachuteSmokeTrailColor(player: number, r: number, g: number, b: number): void;

/**
 * ## Parameters
 * *
 * @param player - 
 * @returns number
 * @remarks Hash: 0x908CBECC2CAA3690
 */
export function isPlayerReadyForCutscene(player: number): number;

/**
 * ## Parameters
 * *
 * @param player - 
 * @param toggle - 
 * @returns void
 * @remarks Hash: 0x75E7D505F2B15902
 */
export function setPlayerForcedZoom(player: number, toggle: number): void;

/**
 * Teleports the player to the given coordinates.
 * 
 * If findCollisionLand is true it will try to find the Z value for you, this however has a timeout of 100 frames.
 * 
 * When trying to find the Z value the native will take longer the higher the difference from the given Z to the ground, this combined with the timeout can cause the teleport to just teleport to the given Z value, so try to estimate the z value, so don't just pass in 1000.0.
 * 
 * Also if you're in a vehicle and teleportWithVehicle is true it will not find the Z value for you.
 * @param player - 
 * @param x - 
 * @param y - 
 * @param z - 
 * @param heading - 
 * @param teleportWithVehicle - 
 * @param findCollisionLand - 
 * @param p7 - 
 * @returns void
 * @remarks Hash: 0xAD15F075A4DA0FDE
 */
export function startPlayerTeleport(player: number, x: number, y: number, z: number, heading: number, teleportWithVehicle: number, findCollisionLand: number, p7: number): void;

/**
 * ## Parameters
 * *
 * @param player - 
 * @param toggle - 
 * @returns void
 * @remarks Hash: 0x0FEE4F80AC44A726
 */
export function setPlayerForcedAim(player: number, toggle: number): void;

/**
 * ```
 * Simply returns whatever is passed to it (Regardless of whether the handle is valid or not).
 * @param value - 
 * @returns number
 * @remarks Hash: 0x9EC6603812C24710
 */
export function intToParticipantindex(value: number): number;

/**
 * ```
 * NativeDB Introduced: v1180
 * ```
 * @param team - 
 * @returns number
 * @remarks Hash: 0x1FC200409F10E6F1
 */
export function GetNumberOfPlayersInTeam(team: number): number;

/**
 * ```
 * Checks whether the specified player has a Ped, the Ped is not dead, is not injured and is not arrested.  
 * ```
 * @param player - 
 * @returns number
 * @remarks Hash: 0x5E9564D8246B909A
 */
export function isPlayerPlaying(player: number): number;

/**
 * ```
 * PLAYER::0xBF6993C7(rPtr((&l_122) + 71)); // Found in decompilation
 * @param p0 - 
 * @returns void
 * @remarks Hash: 0xB45EFF719D8427A6
 */
export function 0xb45eff719d8427a6(p0: number): void;

/**
 * ```
 * Returns profile setting 237.
 * GET_*
 * ```
 * @returns number
 * @remarks Hash: 0xB9CF1F793A9F1BF1
 */
export function 0xb9cf1f793a9f1bf1(): number;

/**
 * ## Parameters
 * *
 * @param player - 
 * @param p1 - 
 * @returns void
 * @remarks Hash: 0x31E90B8873A4CD3B
 */
export function 0x31e90b8873a4cd3b(player: number, p1: number): void;

/**
 * ## Parameters
 * *
 * @param player - 
 * @returns void
 * @remarks Hash: 0x2D03E13C460760D6
 */
export function resetPlayerArrestState(player: number): void;

/**
 * ## Parameters
 * *
 * @param player - 
 * @param toggle - 
 * @returns void
 * @remarks Hash: 0xD2B315B6689D537D
 */
export function setAutoGiveScubaGearWhenExitVehicle(player: number, toggle: number): void;

/**
 * Adds a percentage to a players stamina
 * @param player - 
 * @param percentage - 
 * @returns void
 * @remarks Hash: 0xA352C1B864CAFD33
 */
export function restorePlayerStamina(player: number, percentage: number): void;

/**
 * ```
 * Returns true if the player is riding a train.  
 * ```
 * @param player - 
 * @returns number
 * @remarks Hash: 0x4EC12697209F2196
 */
export function isPlayerRidingTrain(player: number): number;

/**
 * ```
 * # Predominant call signatures  
 * PLAYER::SET_PLAYER_WANTED_CENTRE_POSITION(PLAYER::PLAYER_ID(), ENTITY::GET_ENTITY_COORDS(PLAYER::PLAYER_PED_ID(), 1));  
 * # Parameter value ranges  
 * P0: PLAYER::PLAYER_ID()  
 * P1: ENTITY::GET_ENTITY_COORDS(PLAYER::PLAYER_PED_ID(), 1)  
 * P2: Not set by any call  
 * ```
 * @param player - 
 * @returns void
 * @remarks Hash: 0x520E541A97A13354
 */
export function setPlayerWantedCentrePosition(player: number): void;

/**
 * ## Parameters
 * *
 * @param id - 
 * @param cleanupFlags - 
 * @returns void
 * @remarks Hash: 0xF745B37630DF176B
 */
export function forceCleanupForThreadWithThisId(id: number, cleanupFlags: number): void;

/**
 * ```
 * Gets the player's team.  
 * Does nothing in singleplayer.  
 * ```
 * @param player - 
 * @returns number
 * @remarks Hash: 0x37039302F4E0A008
 */
export function getPlayerTeam(player: number): number;

/**
 * ## Parameters
 * *
 * @param player - 
 * @returns number
 * @remarks Hash: 0xA1FCF8E6AF40B731
 */
export function getPlayerUnderwaterTimeRemaining(player: number): number;

/**
 * ```
 * NativeDB Introduced: v1604
 * ```
 * @param coordX - 
 * @param coordY - 
 * @param coordZ - 
 * @returns void
 * @remarks Hash: 0x70A382ADEC069DD3
 */
export function 0x70a382adec069dd3(coordX: number, coordY: number, coordZ: number): void;

/**
 * ```
 * For Steam.
 * Always returns 0 in retail version of the game.
 * ```
 * @param achievement - 
 * @returns number
 * @remarks Hash: 0x1C186837D0619335
 */
export function GetAchievementProgress(achievement: number): number;

/**
 * ```
 * NativeDB Introduced: v2060
 * ```
 * @param player - 
 * @param wantedLevel - 
 * @param lossTime - 
 * @returns void
 * @remarks Hash: 0x49B856B1360C47C7
 */
export function SetWantedLevelHiddenEvasionTime(player: number, wantedLevel: number, lossTime: number): void;

/**
 * ```
 * NativeDB Added Parameter 2: Any p1
 * ```
 * @param player - 
 * @returns void
 * @remarks Hash: 0x375F0E738F861A94
 */
export function specialAbilityReset(player: number): void;

/**
 * ## Parameters
 * *
 * @param player - 
 * @returns number
 * @remarks Hash: 0x5D35ECF3A81A0EE0
 */
export function getTimeSincePlayerHitVehicle(player: number): number;

/**
 * ## Return value
 * @returns number
 * @remarks Hash: 0x02B15662D7F8886F
 */
export function isPlayerTeleportActive(): number;

/**
 * ```
 * Gets the maximum wanted level the player can get.  
 * Ranges from 0 to 5.  
 * ```
 * @returns number
 * @remarks Hash: 0x462E0DB9B137DC5F
 */
export function getMaxWantedLevel(): number;

/**
 * ```
 * modifier's min value is 0.1
 * ```
 * @param player - 
 * @param modifier - 
 * @returns void
 * @remarks Hash: 0xA50E117CDDF82F0C
 */
export function setPlayerVehicleDamageModifier(player: number, modifier: number): void;

/**
 * ## Parameters
 * *
 * @param player - 
 * @param tintIndex - 
 * @returns void
 * @remarks Hash: 0x6E9C742F340CE5A2
 */
export function getPlayerParachutePackTintIndex(player: number, tintIndex: number): void;

/**
 * ## Parameters
 * *
 * @param player - 
 * @returns number
 * @remarks Hash: 0x65FAEE425DE637B0
 */
export function isPlayerBluetoothEnable(player: number): number;

/**
 * ## Parameters
 * *
 * @param player - 
 * @returns void
 * @remarks Hash: 0xB9D0DD990DC141DD
 */
export function resetWantedLevelDifficulty(player: number): void;

/**
 * ```
 * IS_*
 * ```
 * @param player - 
 * @returns number
 * @remarks Hash: 0x690A61A6D13583F6
 */
export function 0x690a61a6d13583f6(player: number): number;

/**
 * ## Parameters
 * *
 * @param player - 
 * @returns void
 * @remarks Hash: 0x4AACB96203D11A31
 */
export function clearPlayerHasDamagedAtLeastOneNonAnimalPed(player: number): void;

/**
 * ## Parameters
 * *
 * @param player - 
 * @param p2 - 
 * @returns void
 * @remarks Hash: 0xF3AC26D3CC576528
 */
export function removePlayerHelmet(player: number, p2: number): void;

/**
 * ```
 * If toggle is set to false:
 *  The police won't be shown on the (mini)map
 * If toggle is set to true:
 *  The police will be shown on the (mini)map
 * ```
 * @param toggle - 
 * @returns void
 * @remarks Hash: 0x43286D561B72B8BF
 */
export function setPoliceRadarBlips(toggle: number): void;

/**
 * ## Parameters
 * *
 * @param player - 
 * @returns void
 * @remarks Hash: 0x749FADDF97DFE930
 */
export function setPlayerClothPinFrames(player: number): void;

/**
 * ```
 * Every occurrence of p1 & p2 were both true.
 * ```
 * 
 * ```
 * NativeDB Added Parameter 4: Any p3
 * ```
 * @param player - 
 * @param p1 - 
 * @param p2 - 
 * @returns void
 * @remarks Hash: 0x2E7B9B683481687D
 */
export function specialAbilityChargeSmall(player: number, p1: number, p2: number): void;

/**
 * ## Parameters
 * *
 * @param player - 
 * @param wantedLevel - 
 * @returns number
 * @remarks Hash: 0x238DB2A2C23EE9EF
 */
export function isPlayerWantedLevelGreater(player: number, wantedLevel: number): number;

/**
 * Inhibits the player from using any method of combat including melee and firearms.  
 * NOTE: Only disables the firing for one frame
 * @param player - 
 * @param toggle - 
 * @returns void
 * @remarks Hash: 0x5E6CC07646BBEAB8
 */
export function disablePlayerFiring(player: number, toggle: number): void;

/**
 * ## Return value
 * @returns number
 * @remarks Hash: 0x5D511E3867C87139
 */
export function isSystemUiBeingDisplayed(): number;

/**
 * This multiplier is reset to `1.0` every time the player ped is changed, often times via [`SET_PLAYER_MODEL`](#_0x00A1CADD00108836) or [`CHANGE_PLAYER_PED`](#_0x048189FAC643DEEE).
 * @param player - 
 * @param regenRate - 
 * @returns void
 * @remarks Hash: 0x5DB660B38DD98A31
 */
export function setPlayerHealthRechargeMultiplier(player: number, regenRate: number): void;

/**
 * ```
 * NativeDB Introduced: v2060
 * ```
 * @param p0 - 
 * @returns void
 * @remarks Hash: 0x823EC8E82BA45986
 */
export function 0x823ec8e82ba45986(p0: any): void;

/**
 * ```
 * used with 1,2,8,64,128 in the scripts  
 * ```
 * @param cleanupFlags - 
 * @returns void
 * @remarks Hash: 0xBC8983F38F78ED51
 */
export function forceCleanup(cleanupFlags: number): void;

/**
 * ```
 * NativeDB Added Parameter 2: Any p1
 * ```
 * @param player - 
 * @returns void
 * @remarks Hash: 0xC9A763D8FE87436A
 */
export function specialAbilityChargeOnMissionFailed(player: number): void;

/**
 * ```
 * Assigns the handle of locked-on melee target to *entity that you pass it.  
 * Returns false if no entity found.  
 * ```
 * @param player - 
 * @param entity - 
 * @returns number
 * @remarks Hash: 0x13EDE1A5DBF797C9
 */
export function getPlayerTargetEntity(player: number, entity: number): number;

/**
 * ```
 * NativeDB Introduced: v1604
 * ```
 * @returns void
 * @remarks Hash: 0x7148E0F43D11F0D9
 */
export function 0x7148e0f43d11f0d9(): void;

/**
 * ## Parameters
 * *
 * @param player - 
 * @returns number
 * @remarks Hash: 0x8A876A65283DD7D7
 */
export function isPlayerScriptControlOn(player: number): number;

/**
 * ```
 * tints 0- 13
 * 0 - unkown
 * 1 - unkown
 * 2 - unkown
 * 3 - unkown
 * 4 - unkown
 * ```
 * @param player - 
 * @param tintIndex - 
 * @returns void
 * @remarks Hash: 0x93B0FB27C9A04060
 */
export function setPlayerParachutePackTintIndex(player: number, tintIndex: number): void;

/**
 * ```
 * Returns TRUE if the player ('s ped) is climbing at the moment.  
 * ```
 * @param player - 
 * @returns number
 * @remarks Hash: 0x95E8F73DC65EFB9C
 */
export function isPlayerClimbing(player: number): number;

/**
 * ## Parameters
 * *
 * @param player - 
 * @returns number
 * @remarks Hash: 0x1885BC9B108B4C99
 */
export function getPlayerSprintTimeRemaining(player: number): number;

/**
 * ```
 * p1 appears to always be 1 (only comes up twice)
 * ```
 * 
 * ```
 * NativeDB Added Parameter 3: Any p2
 * ```
 * @param player - 
 * @param p2 - 
 * @returns void
 * @remarks Hash: 0xED481732DFF7E997
 */
export function specialAbilityChargeContinuous(player: number, p2: number): void;

/**
 * ```
 * 2 matches. p1 was always true.
 * ```
 * 
 * ```
 * NativeDB Added Parameter 4: Any p3
 * ```
 * @param player - 
 * @param p1 - 
 * @param p2 - 
 * @returns void
 * @remarks Hash: 0xF733F45FA4497D93
 */
export function specialAbilityChargeLarge(player: number, p1: number, p2: number): void;

/**
 * ```
 * 1.0.335.2, 1.0.350.1/2, 1.0.372.2, 1.0.393.2, 1.0.393.4, 1.0.463.1;  
 * ```
 * @returns void
 * @remarks Hash: 0x1DD5897E2FA6E7C9
 */
export function playerDetachVirtualBound(): void;

/**
 * ## Parameters
 * *
 * @param player - 
 * @param toggle - 
 * @returns void
 * @remarks Hash: 0xA01B8075D8B92DF4
 */
export function setPlayerSprint(player: number, toggle: number): void;

/**
 * ```
 * Flags:
 * SPC_AMBIENT_SCRIPT = (1 << 1),
 * SPC_CLEAR_TASKS = (1 << 2),
 * SPC_REMOVE_FIRES = (1 << 3),
 * SPC_REMOVE_EXPLOSIONS = (1 << 4),
 * SPC_REMOVE_PROJECTILES = (1 << 5),
 * SPC_DEACTIVATE_GADGETS = (1 << 6),
 * SPC_REENABLE_CONTROL_ON_DEATH = (1 << 7),
 * SPC_LEAVE_CAMERA_CONTROL_ON = (1 << 8),
 * SPC_ALLOW_PLAYER_DAMAGE = (1 << 9),
 * SPC_DONT_STOP_OTHER_CARS_AROUND_PLAYER = (1 << 10),
 * SPC_PREVENT_EVERYBODY_BACKOFF = (1 << 11),
 * SPC_ALLOW_PAD_SHAKE = (1 << 12)
 * See: https://alloc8or.re/gta5/doc/enums/eSetPlayerControlFlag.txt
 * ```
 * @param player - 
 * @param bHasControl - 
 * @param flags - 
 * @returns void
 * @remarks Hash: 0x8D32347D6D4C40A2
 */
export function setPlayerControl(player: number, bHasControl: number, flags: number): void;

/**
 * ```
 * The player will be ignored by the police if toggle is set to true  
 * ```
 * @param player - 
 * @param toggle - 
 * @returns void
 * @remarks Hash: 0x32C62AA929C2DA6A
 */
export function setPoliceIgnorePlayer(player: number, toggle: number): void;

/**
 * ## Parameters
 * *
 * @param targetLevel - 
 * @returns void
 * @remarks Hash: 0x5702B917B99DB1CD
 */
export function setPlayerTargetLevel(targetLevel: number): void;

/**
 * ```
 * Only 1 match. Both p1 & p2 were true.
 * ```
 * 
 * ```
 * NativeDB Added Parameter 4: Any p3
 * ```
 * @param player - 
 * @param p1 - 
 * @param p2 - 
 * @returns void
 * @remarks Hash: 0xF113E3AA9BC54613
 */
export function specialAbilityChargeMedium(player: number, p1: number, p2: number): void;

/**
 * ## Parameters
 * *
 * @param player - 
 * @param toggle - 
 * @returns void
 * @remarks Hash: 0x7651BC64AE59E128
 */
export function setPlayerForceSkipAimIntro(player: number, toggle: number): void;

/**
 * ```
 * NativeDB Added Parameter 2: Any p1
 * ```
 * @param player - 
 * @returns number
 * @remarks Hash: 0x3E5F7FC85D854E15
 */
export function isSpecialAbilityActive(player: number): number;

/**
 * ## Parameters
 * *
 * @param cleanupFlags - 
 * @returns number
 * @remarks Hash: 0xC968670BFACE42D9
 */
export function hasForceCleanupOccurred(cleanupFlags: number): number;

/**
 * ## Parameters
 * *
 * @param player - 
 * @param toggle - 
 * @returns void
 * @remarks Hash: 0x3C49C870E66F0A28
 */
export function givePlayerRagdollControl(player: number, toggle: number): void;

/**
 * ```
 * Can the player control himself, used to disable controls for player for things like a cutscene.
 * @param player - 
 * @returns number
 * @remarks Hash: 0x49C32D60007AFA47
 */
export function isPlayerControlOn(player: number): number;

/**
 * ```
 * Swim speed multiplier.  
 * Multiplier goes up to 1.49  
 * Just call it one time, it is not required to be called once every tick. - Note copied from below native.  
 * Note: At least the IDA method if you change the max float multiplier from 1.5 it will change it for both this and RUN_SPRINT below. I say 1.5 as the function blrs if what you input is greater than or equal to 1.5 hence why it's 1.49 max default.  
 * ```
 * @param player - 
 * @param multiplier - 
 * @returns void
 * @remarks Hash: 0xA91C6F0FF7D16A13
 */
export function setSwimMultiplierForPlayer(player: number, multiplier: number): void;

/**
 * ## Parameters
 * *
 * @param player - 
 * @param limit - 
 * @returns void
 * @remarks Hash: 0xC388A0F065F5BC34
 */
export function SetPlayerHealthRechargeLimit(player: number, limit: number): void;

/**
 * Sets whether all random peds will run away from the player if they are agitated (threatened) (bool=true), or if they will stand their ground (bool=false).
 * @param player - 
 * @param toggle - 
 * @returns void
 * @remarks Hash: 0x056E0FE8534C2949
 */
export function setAllRandomPedsFlee(player: number, toggle: number): void;

/**
 * ```
 * - This is called after SET_ALL_RANDOM_PEDS_FLEE_THIS_FRAME
 * ```
 * @param player - 
 * @returns void
 * @remarks Hash: 0xC3376F42B1FACCC6
 */
export function 0xc3376f42b1faccc6(player: number): void;

/**
 * ```
 * Every occurrence was either 0 or 2.  
 * ```
 * @param index - 
 * @returns void
 * @remarks Hash: 0x9F7BBA2EA6372500
 */
export function setPlayerClothPackageIndex(index: number): void;

/**
 * ## Parameters
 * *
 * @param player - 
 * @returns number
 * @remarks Hash: 0xE4B90F367BD81752
 */
export function hasPlayerDamagedAtLeastOneNonAnimalPed(player: number): number;

/**
 * ## Parameters
 * *
 * @param player - 
 * @param wantedLevel - 
 * @param delayedResponse - 
 * @returns void
 * @remarks Hash: 0x340E61DE7F471565
 */
export function setPlayerWantedLevelNoDrop(player: number, wantedLevel: number, delayedResponse: number): void;

/**
 * ```
 * example:  
 * flags: 0-6  
 * PLAYER::SET_PLAYER_RESET_FLAG_PREFER_REAR_SEATS(PLAYER::PLAYER_ID(), 6);  
 * wouldnt the flag be the seatIndex?  
 * ```
 * @param player - 
 * @param flags - 
 * @returns void
 * @remarks Hash: 0x11D5F725F0E780E0
 */
export function setPlayerResetFlagPreferRearSeats(player: number, flags: number): void;

/**
 * ```
 * NativeDB Introduced: v1290
 * ```
 * @param p0 - 
 * @param p1 - 
 * @param p2 - 
 * @param p3 - 
 * @param p4 - 
 * @param p5 - 
 * @returns void
 * @remarks Hash: 0x7BAE68775557AE0B
 */
export function 0x7bae68775557ae0b(p0: any, p1: any, p2: any, p3: any, p4: any, p5: any): void;

/**
 * ```
 * Forces any pending wanted level to be applied to the specified player immediately.  
 * Call SET_PLAYER_WANTED_LEVEL with the desired wanted level, followed by SET_PLAYER_WANTED_LEVEL_NOW.  
 * Second parameter is unknown (always false).  
 * ```
 * @param player - 
 * @param p1 - 
 * @returns void
 * @remarks Hash: 0xE0A7D1E497FFCD6F
 */
export function setPlayerWantedLevelNow(player: number, p1: number): void;

/**
 * ```
 * NativeDB Introduced: v2372
 * ```
 * @returns number
 * @remarks Hash: 0xA72200F51875FEA4
 */
export function GetWantedLevelParoleDuration(): number;

/**
 * ```
 * SET_PLAYER_MAX_*
 * ```
 * @param player - 
 * @param p1 - 
 * @returns void
 * @remarks Hash: 0x8D768602ADEF2245
 */
export function 0x8d768602adef2245(player: number, p1: number): void;

/**
 * ```
 * p1 was always 5.  
 * p4 was always false.  
 * ```
 * @param player - 
 * @param p1 - 
 * @param p2 - 
 * @param p3 - 
 * @param p4 - 
 * @returns void
 * @remarks Hash: 0xD9284A8C0D48352C
 */
export function setPlayerParachuteVariationOverride(player: number, p1: number, p2: any, p3: any, p4: number): void;

/**
 * ```
 * NativeDB Added Parameter 2: Any p1
 * ```
 * @param player - 
 * @returns void
 * @remarks Hash: 0xFFEE8FA29AB9A18E
 */
export function 0xffee8fa29ab9a18e(player: number): void;

/**
 * ## Parameters
 * *
 * @param player - 
 * @returns void
 * @remarks Hash: 0xF0B67A4DE6AB5F98
 */
export function clearPlayerHasDamagedAtLeastOnePed(player: number): void;

/**
 * ```
 * NativeDB Introduced: v1180
 * ```
 * @param player - 
 * @param p1 - 
 * @returns void
 * @remarks Hash: 0xEE4EBDD2593BA844
 */
export function SetPlayerHomingRocketDisabled(player: number, p1: number): void;

/**
 * ```
 * NativeDB Added Parameter 3: Any p2
 * ```
 * @param player - 
 * @param toggle - 
 * @returns void
 * @remarks Hash: 0x181EC197DAEFE121
 */
export function enableSpecialAbility(player: number, toggle: number): void;

/**
 * Suppresses a crime for a given player for this frame only.
 * @param player - 
 * @param crimeType - 
 * @returns void
 * @remarks Hash: 0x9A987297ED8BD838
 */
export function suppressCrimeThisFrame(player: number, crimeType: number): void;

/**
 * ## Parameters
 * *
 * @param player - 
 * @returns number
 * @remarks Hash: 0x8BC515BAE4AAF8FF
 */
export function GetPlayerHealthRechargeLimit(player: number): number;

/**
 * It returns true if the player is online, suggesting they are also logged in locally. Note that this is an alias for `NETWORK_IS_SIGNED_ONLINE`.
 * @returns number
 * @remarks Hash: 0xF25D331DC2627BBC
 */
export function isPlayerOnline(): number;

/**
 * ```
 * NativeDB Added Parameter 2: Any p1
 * ```
 * @param player - 
 * @returns void
 * @remarks Hash: 0xD6A953C6D1492057
 */
export function specialAbilityDeactivate(player: number): void;

/**
 * ## Parameters
 * *
 * @param player - 
 * @param toggle - 
 * @returns void
 * @remarks Hash: 0xDE45D1A1EF45EE61
 */
export function 0xde45d1a1ef45ee61(player: number, toggle: number): void;

/**
 * ## Parameters
 * *
 * @param player - 
 * @returns number
 * @remarks Hash: 0x56105E599CAB0EFA
 */
export function getPlayerFakeWantedLevel(player: number): number;

/**
 * ```
 * Disables the player's teleportation  
 * ```
 * @returns void
 * @remarks Hash: 0xC449EDED9D73009C
 */
export function stopPlayerTeleport(): void;

/**
 * ```
 * Does the same like PLAYER::GET_PLAYER_PED
 * ```
 * @param player - 
 * @returns number
 * @remarks Hash: 0x50FAC3A3E030A6E1
 */
export function getPlayerPedScriptIndex(player: number): number;

/**
 * ```
 * Tints:  
 * None = -1,  
 * Rainbow = 0,  
 * Red = 1,  
 * SeasideStripes = 2,  
 * WidowMaker = 3,  
 * Patriot = 4,  
 * Blue = 5,  
 * Black = 6,  
 * Hornet = 7,  
 * AirFocce = 8,  
 * Desert = 9,  
 * Shadow = 10,  
 * HighAltitude = 11,  
 * Airbone = 12,  
 * Sunrise = 13,  
 * ```
 * @param player - 
 * @param tintIndex - 
 * @returns void
 * @remarks Hash: 0xA3D0E54541D9A5E5
 */
export function setPlayerParachuteTintIndex(player: number, tintIndex: number): void;

/**
 * Limit the player to only enter this vehicle. Note set vehicle to false if you want them to access any vehicle.
 * @param player - 
 * @param vehicle - 
 * @returns void
 * @remarks Hash: 0x8026FF78F208978A
 */
export function setPlayerMayOnlyEnterThisVehicle(player: number, vehicle: number): void;

