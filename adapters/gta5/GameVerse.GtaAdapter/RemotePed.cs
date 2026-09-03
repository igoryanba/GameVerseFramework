using System;
using System.Diagnostics;
using GTA;
using GTA.Math;
using GameVerse.AdapterProtocol;

namespace GameVerse.GtaAdapter
{
    // All methods run on the SHVDNE Script.Tick execution context.
    internal sealed class RemotePed : IDisposable
    {
        private Ped ped;
        private RemoteEntity desired;
        private uint modelHash;
        private long requestedAt;
        private bool failed;
        private readonly Action<string, EntityId> report;
        public RemotePed(RemoteEntity entity, Action<string, EntityId> report) { desired = entity; this.report = report; }
        public void Update(RemoteEntity entity) { desired = entity; }
        public void Tick()
        {
            PlayerState state = desired.state;
            if (modelHash != state.model_hash) { Dispose(); modelHash = state.model_hash; failed = false; requestedAt = 0; }
            if (failed) return;
            if (ped == null || !ped.Exists())
            {
                var model = new Model(unchecked((int)state.model_hash));
                if (!model.IsInCdImage || !model.IsPed) { failed = true; report("remote_model_invalid", desired.id); return; }
                if (!model.IsLoaded)
                {
                    if (requestedAt == 0) requestedAt = Stopwatch.GetTimestamp();
                    if ((Stopwatch.GetTimestamp() - requestedAt) / (double)Stopwatch.Frequency > 10) { model.MarkAsNoLongerNeeded(); failed = true; report("remote_model_timeout", desired.id); return; }
                    model.Request(); return;
                }
                ped = World.CreatePed(model, Vector(state.position), 0);
                model.MarkAsNoLongerNeeded();
                if (ped == null || !ped.Exists()) return;
                ped.IsPersistent = true; ped.BlockPermanentEvents = true;
                ped.IsInvincible = true; ped.CanRagdoll = false;
                // M1 presentation is kinematic. GTA physics cannot fight interpolated poses.
                ped.IsPositionFrozen = true; ped.IsCollisionEnabled = false;
                report("remote_ped_created", desired.id);
            }
            ped.PositionNoOffset = Vector(state.position);
            ped.Quaternion = new Quaternion(state.rotation[0], state.rotation[1], state.rotation[2], state.rotation[3]);
        }
        private static Vector3 Vector(float[] a) => new Vector3(a[0], a[1], a[2]);
        public void Dispose()
        {
            if (ped != null && ped.Exists()) { ped.Delete(); report("remote_ped_destroyed", desired.id); }
            ped = null;
            if (modelHash != 0) new Model(unchecked((int)modelHash)).MarkAsNoLongerNeeded();
        }
    }
}
