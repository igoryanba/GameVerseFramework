using System;
using System.Diagnostics;
using GTA;
using GTA.Math;
using GTA.Native;
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
        private readonly LocomotionController locomotion = new LocomotionController();
        private LocomotionState appliedState = LocomotionState.Idle;
        private long lastTaskAt;
        private uint weaponHash;
        private bool wasDead;
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
                // The server owns identity and the remote client owns the sampled pose.
                // Local physics and ambient AI must not take ownership of this presentation ped.
                ped.IsPositionFrozen = false; ped.IsCollisionEnabled = false;
                report("remote_ped_created", desired.id);
            }
            var target = Vector(state.position);
            var delta = target - ped.Position;
            float distanceSquared = delta.LengthSquared();
            if (distanceSquared > 100f) ped.PositionNoOffset = target;
            else if (distanceSquared > 2.25f) ped.PositionNoOffset = Vector3.Lerp(ped.Position, target, 0.35f);

            var targetRotation = new Quaternion(state.rotation[0], state.rotation[1], state.rotation[2], state.rotation[3]);
            ped.Quaternion = Quaternion.Slerp(ped.Quaternion, targetRotation, 0.2f);
            bool changed = locomotion.Update(state);
            long now = Stopwatch.GetTimestamp();
            bool refresh = (now - lastTaskAt) / (double)Stopwatch.Frequency >= 0.5;
            if (changed || refresh) ApplyLocomotion(state, target, now);
            ApplyWeapon(state.weapon_hash);
        }
        private void ApplyLocomotion(PlayerState state, Vector3 target, long now)
        {
            appliedState = locomotion.Current;
            lastTaskAt = now;
            if (wasDead && appliedState != LocomotionState.Dead)
            {
                Dispose();
                failed = false;
                requestedAt = 0;
                wasDead = false;
                return;
            }
            ped.CanRagdoll = appliedState == LocomotionState.Ragdoll;
            var predicted = target + Vector(state.velocity) * 0.15f;
            switch (appliedState)
            {
                case LocomotionState.Dead:
                    Function.Call(Hash.SET_ENTITY_HEALTH, ped.Handle, 0);
                    wasDead = true;
                    return;
                case LocomotionState.Ragdoll:
                    ped.CanRagdoll = true;
                    Function.Call(Hash.SET_PED_TO_RAGDOLL, ped.Handle, 750, 1000, 0, false, false, false);
                    return;
                case LocomotionState.Jump:
                    Function.Call(Hash.TASK_JUMP, ped.Handle, true);
                    return;
                case LocomotionState.Fall:
                    Function.Call(Hash.TASK_SKY_DIVE, ped.Handle, true);
                    return;
                case LocomotionState.Aim:
                    float aimX = predicted.X + state.velocity[0] * 5f;
                    float aimY = predicted.Y + state.velocity[1] * 5f;
                    float aimZ = predicted.Z + 1f;
                    if ((state.movement & 128) != 0)
                        Function.Call(Hash.TASK_SHOOT_AT_COORD, ped.Handle, aimX, aimY, aimZ, 750, 0);
                    else
                        Function.Call(Hash.TASK_AIM_GUN_AT_COORD, ped.Handle, aimX, aimY, aimZ, 750, false, false);
                    if ((state.movement & 256) != 0) Function.Call(Hash.MAKE_PED_RELOAD, ped.Handle);
                    return;
                case LocomotionState.Walk: Move(predicted, 1.0f); return;
                case LocomotionState.Run: Move(predicted, 2.0f); return;
                case LocomotionState.Sprint: Move(predicted, 3.0f); return;
                default:
                    Function.Call(Hash.TASK_STAND_STILL, ped.Handle, 750);
                    return;
            }
        }
        private void Move(Vector3 target, float speed)
        {
            Function.Call(Hash.TASK_GO_STRAIGHT_TO_COORD, ped.Handle,
                target.X, target.Y, target.Z, speed, 750, 0f, 0.5f);
        }
        private void ApplyWeapon(uint next)
        {
            if (next == weaponHash) return;
            weaponHash = next;
            Function.Call(Hash.REMOVE_ALL_PED_WEAPONS, ped.Handle, true);
            if (next != 0 && next != 0xa2719263)
                Function.Call(Hash.GIVE_WEAPON_TO_PED, ped.Handle, next, 999, false, true);
        }
        private static Vector3 Vector(float[] a) => new Vector3(a[0], a[1], a[2]);
        public void Dispose()
        {
            if (ped != null && ped.Exists()) { ped.Delete(); report("remote_ped_destroyed", desired.id); }
            ped = null;
            weaponHash = 0;
            wasDead = false;
            if (modelHash != 0) new Model(unchecked((int)modelHash)).MarkAsNoLongerNeeded();
        }
    }
}
