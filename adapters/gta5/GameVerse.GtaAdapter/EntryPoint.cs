using System;
using System.Collections.Generic;
using System.Diagnostics;
using System.IO;
using GTA;
using GTA.Native;
using Newtonsoft.Json.Linq;
using GameVerse.AdapterProtocol;

namespace GameVerse.GtaAdapter
{
    public sealed class EntryPoint : Script
    {
        private readonly Dictionary<EntityId, RemotePed> remotes = new Dictionary<EntityId, RemotePed>();
        private readonly Dictionary<EntityId, RemoteVehicle> remoteVehicles = new Dictionary<EntityId, RemoteVehicle>();
        private readonly Stopwatch elapsed = Stopwatch.StartNew();
        private readonly PipeLink link;
        private readonly SessionBootstrap bootstrap = new SessionBootstrap();
        private readonly string logPath;
        private readonly object logGate = new object();
        private EntityId local;
        private long lastSample = -50;
        private bool validBuild;
        private bool bootstrapReported;
        private string reportedBootstrapError;
        public EntryPoint()
        {
            logPath = Path.Combine(AppDomain.CurrentDomain.BaseDirectory, "GameVerse.GtaAdapter.log");
            string build = FileVersionInfo.GetVersionInfo(Process.GetCurrentProcess().MainModule.FileName).FileVersion;
            string executable = Path.GetFileName(Process.GetCurrentProcess().MainModule.FileName);
            validBuild = executable.Equals("GTA5_Enhanced.exe", StringComparison.OrdinalIgnoreCase) && build == Wire.GameBuild;
            Log("GTA_ADAPTER_LOADED=true BUILD=" + build + " SUPPORTED=" + validBuild);
            link = new PipeLink(Log, build ?? "unknown");
            Tick += OnTick;
            Aborted += (sender, args) => { link.Dispose(); Cleanup(); bootstrap.Reset(); Log("ADAPTER_STOPPED=true"); };
            link.Start();
        }
        private void OnTick(object sender, EventArgs args)
        {
            try
            {
                // SHV starts scripts after the initial loading screen; verify the ped below.
                bool ready = validBuild && !Game.IsPaused && !Function.Call<bool>(Hash.NETWORK_IS_GAME_IN_PROGRESS);
                Ped player = ready ? Game.Player.Character : null;
                ready = ready && player != null && player.Exists();
                if (!ready) { link.Publish(null, false); Cleanup(); return; }
                bool bootstrapApplied = bootstrap.Tick();
                if (bootstrapApplied && !bootstrapReported)
                {
                    bootstrapReported = true;
                    link.Report("session_ready", local);
                }
                else if (bootstrap.Error != null && bootstrap.Error != reportedBootstrapError)
                {
                    bootstrapReported = true;
                    reportedBootstrapError = bootstrap.Error;
                    link.ReportBootstrapFailure(bootstrap.Error, "Не удалось применить конфигурацию персонажа");
                }
                if (elapsed.ElapsedMilliseconds - lastSample >= 50)
                {
                    lastSample = elapsed.ElapsedMilliseconds;
                    var p = player.Position; var q = player.Quaternion; var v = player.Velocity;
                    ushort movement = 0;
                    if (!player.IsInVehicle()) movement |= 1;
                    if (player.IsRunning) movement |= 2;
                    if (player.IsSprinting) movement |= 4;
                    if (player.IsJumping) movement |= 8;
                    if (player.IsRagdoll) movement |= 16;
                    if (Function.Call<bool>(Hash.IS_PLAYER_FREE_AIMING, Game.Player.Handle)) movement |= 32;
                    if (player.IsFalling) movement |= 64;
                    if (player.IsShooting) movement |= 128;
                    if (Function.Call<bool>(Hash.IS_PED_RELOADING, player.Handle)) movement |= 256;
                    var state = new PlayerState { timestamp_ms = (ulong)elapsed.ElapsedMilliseconds, position = new[] { p.X, p.Y, p.Z }, rotation = new[] { q.X, q.Y, q.Z, q.W }, velocity = new[] { v.X, v.Y, v.Z }, model_hash = unchecked((uint)player.Model.Hash), health = (ushort)Math.Max(0, Math.Min(1000, player.Health)), armor = (ushort)Math.Max(0, Math.Min(1000, player.Armor)), movement = movement, weapon_hash = unchecked((uint)player.Weapons.Current.Hash) };
                    link.Publish(state.IsValid() ? state : null, true);
                }
                for (int i = 0; i < 64 && link.Commands.TryTake(out JObject message); i++) Handle(message);
                foreach (var ped in remotes.Values) ped.Tick();
                foreach (var vehicle in remoteVehicles.Values) vehicle.Tick();
            }
            catch (Exception e) { Log("ADAPTER_ERROR " + e); link.Publish(null, false); Cleanup(); }
        }
        private void Handle(JObject message)
        {
            switch ((string)message["type"])
            {
                case "session_begin":
                    Cleanup();
                    local = message["entity"].ToObject<EntityId>();
                    var config = message["config"]?.ToObject<SessionConfig>();
                    if (config == null || !config.IsValid()) throw new InvalidDataException("Invalid session config");
                    bootstrap.Begin(config);
                    bootstrapReported = false;
                    reportedBootstrapError = null;
                    break;
                case "session_active":
                    bootstrap.Activate();
                    break;
                case "session_end":
                    Cleanup();
                    bootstrap.Reset();
                    bootstrapReported = false;
                    reportedBootstrapError = null;
                    local = null;
                    break;
                case "remote_entity_create":
                case "remote_entity_update":
                    var entity = Wire.Entity(message);
                    if (local == null || entity.id.Equals(local)) return;
                    if (remotes.TryGetValue(entity.id, out RemotePed ped)) ped.Update(entity);
                    else if (remotes.Count < 31) remotes.Add(entity.id, new RemotePed(entity, (name, id) => { Log(name + " " + id); link.Report(name, id); }));
                    break;
                case "remote_entity_destroy":
                    var id = message["id"].ToObject<EntityId>();
                    if (remotes.TryGetValue(id, out RemotePed removed)) { removed.Dispose(); remotes.Remove(id); } break;
                case "remote_vehicle_create":
                    var vehicleEntity = message.ToObject<RemoteVehicleEntity>();
                    if (vehicleEntity == null || !vehicleEntity.IsValid()) throw new InvalidDataException("Invalid remote vehicle");
                    if (!remoteVehicles.ContainsKey(vehicleEntity.id) && remoteVehicles.Count < 128)
                        remoteVehicles.Add(vehicleEntity.id, new RemoteVehicle(vehicleEntity, (name, vehicleId) => { Log(name + " " + vehicleId); link.Report(name, vehicleId); }));
                    break;
                case "remote_vehicle_update":
                    var vehicleId = message["id"].ToObject<EntityId>();
                    var vehicleFrame = message["frame"].ToObject<VehicleFrameV2>();
                    if (vehicleId == null || vehicleId.generation == 0 || vehicleId.slot >= 128 || vehicleFrame == null || !vehicleFrame.IsValid()) throw new InvalidDataException("Invalid remote vehicle update");
                    if (remoteVehicles.TryGetValue(vehicleId, out RemoteVehicle remoteVehicle)) remoteVehicle.Update(vehicleFrame);
                    break;
                case "remote_vehicle_destroy":
                    var destroyedVehicleId = message["id"].ToObject<EntityId>();
                    if (remoteVehicles.TryGetValue(destroyedVehicleId, out RemoteVehicle destroyedVehicle)) { destroyedVehicle.Dispose(); remoteVehicles.Remove(destroyedVehicleId); }
                    break;
                case "reset": Cleanup(); bootstrap.Reset(); bootstrapReported = false; reportedBootstrapError = null; local = null; break;
                default: throw new InvalidDataException("Unexpected bridge command");
            }
        }
        private void Cleanup() { foreach (var ped in remotes.Values) ped.Dispose(); remotes.Clear(); foreach (var vehicle in remoteVehicles.Values) vehicle.Dispose(); remoteVehicles.Clear(); }
        private void Log(string message)
        {
            lock (logGate) { try { File.AppendAllText(logPath, DateTime.UtcNow.ToString("O") + " " + message + Environment.NewLine); } catch (Exception e) when (e is IOException || e is UnauthorizedAccessException) { } }
        }
    }
}
