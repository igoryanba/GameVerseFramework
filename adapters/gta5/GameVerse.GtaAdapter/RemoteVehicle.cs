using System;
using System.Diagnostics;
using GTA;
using GTA.Math;
using GTA.Native;
using GameVerse.AdapterProtocol;

namespace GameVerse.GtaAdapter
{
    // Presentation-only vehicle. The server selects the simulation owner; this instance never publishes authority.
    internal sealed class RemoteVehicle : IDisposable
    {
        private readonly EntityId id;
        private readonly Action<string, EntityId> report;
        private Vehicle vehicle;
        private uint modelHash;
        private VehicleFrameV2 desired;
        private long requestedAt;
        private bool failed;

        public RemoteVehicle(RemoteVehicleEntity entity, Action<string, EntityId> report)
        {
            id = entity.id;
            modelHash = entity.model_hash;
            desired = entity.frame;
            this.report = report;
        }

        public void Update(VehicleFrameV2 frame) { if (frame.sequence > desired.sequence) desired = frame; }

        public void Tick()
        {
            if (failed) return;
            if (vehicle == null || !vehicle.Exists())
            {
                var model = new Model(unchecked((int)modelHash));
                if (!model.IsInCdImage || !model.IsVehicle) { failed = true; report("remote_vehicle_model_invalid", id); return; }
                if (!model.IsLoaded)
                {
                    if (requestedAt == 0) requestedAt = Stopwatch.GetTimestamp();
                    if ((Stopwatch.GetTimestamp() - requestedAt) / (double)Stopwatch.Frequency > 10) { model.MarkAsNoLongerNeeded(); failed = true; report("remote_vehicle_model_timeout", id); return; }
                    model.Request(); return;
                }
                vehicle = World.CreateVehicle(model, Vector(desired.transform.position), 0);
                model.MarkAsNoLongerNeeded();
                if (vehicle == null || !vehicle.Exists()) return;
                vehicle.IsPersistent = true;
                vehicle.IsCollisionEnabled = false;
                vehicle.IsEngineRunning = true;
                report("remote_vehicle_created", id);
            }
            Vector3 target = Vector(desired.transform.position);
            Vector3 delta = target - vehicle.Position;
            float distanceSquared = delta.LengthSquared();
            if (distanceSquared > 100f) vehicle.PositionNoOffset = target;
            else vehicle.PositionNoOffset = Vector3.Lerp(vehicle.Position, target, distanceSquared > 2.25f ? 0.35f : 0.12f);
            var rotation = desired.transform.rotation;
            vehicle.Quaternion = Quaternion.Slerp(vehicle.Quaternion, new Quaternion(rotation[0], rotation[1], rotation[2], rotation[3]), 0.2f);
            vehicle.Velocity = Vector(desired.transform.velocity);
            vehicle.EngineHealth = desired.engine_health;
            vehicle.BodyHealth = desired.body_health;
            Function.Call(Hash.SET_VEHICLE_STEER_BIAS, vehicle.Handle, desired.steering);
        }

        private static Vector3 Vector(float[] values) => new Vector3(values[0], values[1], values[2]);

        public void Dispose()
        {
            if (vehicle != null && vehicle.Exists()) { vehicle.Delete(); report("remote_vehicle_destroyed", id); }
            vehicle = null;
            if (modelHash != 0) new Model(unchecked((int)modelHash)).MarkAsNoLongerNeeded();
        }
    }
}
