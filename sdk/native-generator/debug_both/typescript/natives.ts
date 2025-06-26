// Auto-generated TypeScript definitions

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

