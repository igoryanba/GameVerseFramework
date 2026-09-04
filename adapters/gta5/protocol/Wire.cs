using System;
using System.IO;
using System.Text;
using System.Threading;
using System.Threading.Tasks;
using Newtonsoft.Json;
using Newtonsoft.Json.Linq;

namespace GameVerse.AdapterProtocol
{
    public sealed class EntityId : IEquatable<EntityId>
    {
        public uint slot;
        public ulong generation;
        public bool Equals(EntityId other) => other != null && slot == other.slot && generation == other.generation;
        public override bool Equals(object other) => Equals(other as EntityId);
        public override int GetHashCode() => unchecked((int)slot * 397 ^ generation.GetHashCode());
        public override string ToString() => slot + ":" + generation;
    }
    public sealed class PlayerState
    {
        public ulong timestamp_ms;
        public float[] position;
        public float[] rotation;
        public float[] velocity;
        public uint model_hash;
        public ushort health;
        public ushort armor;
        public ushort movement;
        public uint weapon_hash;
        public bool IsValid()
        {
            if (!VectorIsValid(position, 3, 20000) || !VectorIsValid(velocity, 3, 500) || !VectorIsValid(rotation, 4, 1.01f)) return false;
            double norm = 0; foreach (float v in rotation) norm += v * v;
            return Math.Abs(norm - 1) <= 0.02 && model_hash != 0 && health <= 1000 && armor <= 1000 && (movement & ~511) == 0;
        }
        internal static bool VectorIsValid(float[] a, int n, float bound)
        {
            if (a == null || a.Length != n) return false;
            foreach (float v in a) if (float.IsNaN(v) || float.IsInfinity(v) || Math.Abs(v) > bound) return false;
            return true;
        }
    }
    public sealed class SessionConfig
    {
        public float[] spawn;
        public float heading;
        public uint model_hash;
        public uint instance_id;
        public bool IsValid()
        {
            if (spawn == null || spawn.Length != 3 || float.IsNaN(heading) || float.IsInfinity(heading) || model_hash == 0) return false;
            foreach (float value in spawn)
                if (float.IsNaN(value) || float.IsInfinity(value) || Math.Abs(value) > 20000) return false;
            return true;
        }
    }
    public enum LocomotionState { Idle, Walk, Run, Sprint, Jump, Fall, Ragdoll, Aim, Dead }
    public sealed class TransformV2
    {
        public float[] position;
        public float[] rotation;
        public float[] velocity;
        public bool IsValid() => PlayerState.VectorIsValid(position, 3, 20000)
            && PlayerState.VectorIsValid(velocity, 3, 500)
            && PlayerState.VectorIsValid(rotation, 4, 1.01f)
            && Math.Abs(rotation[0] * rotation[0] + rotation[1] * rotation[1]
                + rotation[2] * rotation[2] + rotation[3] * rotation[3] - 1) <= 0.02;
    }
    public sealed class AppearanceV2 { public uint model_hash; }
    public sealed class CombatPresentationV2
    {
        public bool aiming;
        public bool shooting;
        public bool reloading;
        public bool dead;
        public uint weapon_hash;
        public float[] aim_target;
        public bool IsValid() => aim_target == null || PlayerState.VectorIsValid(aim_target, 3, 20000);
    }
    public sealed class VehicleOccupancyV2 { public EntityId vehicle; public sbyte seat; }
    public sealed class PlayerFrameV2
    {
        public ulong sequence;
        public ulong client_tick;
        public TransformV2 transform;
        public AppearanceV2 appearance;
        public string locomotion;
        public CombatPresentationV2 combat;
        public VehicleOccupancyV2 vehicle;
        public bool IsValid() => sequence > 0 && transform != null && transform.IsValid()
            && (appearance == null || appearance.model_hash != 0)
            && combat != null && combat.IsValid();
    }
    public sealed class LocomotionController
    {
        private LocomotionState current = LocomotionState.Idle;
        private int stableFrames;
        public LocomotionState Current => current;
        public bool Update(PlayerState state)
        {
            var speed = Math.Sqrt(state.velocity[0] * state.velocity[0] + state.velocity[1] * state.velocity[1]);
            LocomotionState candidate;
            if (state.health == 0) candidate = LocomotionState.Dead;
            else if ((state.movement & 16) != 0) candidate = LocomotionState.Ragdoll;
            else if ((state.movement & 64) != 0) candidate = LocomotionState.Fall;
            else if ((state.movement & 8) != 0) candidate = LocomotionState.Jump;
            else if ((state.movement & 32) != 0) candidate = LocomotionState.Aim;
            else if ((state.movement & 4) != 0 || speed >= 5.5) candidate = LocomotionState.Sprint;
            else if ((state.movement & 2) != 0 || speed >= 2.0) candidate = LocomotionState.Run;
            else if (speed >= 0.15) candidate = LocomotionState.Walk;
            else candidate = LocomotionState.Idle;

            if (candidate == current) { stableFrames = 0; return false; }
            if (candidate == LocomotionState.Jump || candidate == LocomotionState.Fall || candidate == LocomotionState.Ragdoll || candidate == LocomotionState.Dead)
            { current = candidate; stableFrames = 0; return true; }
            if (++stableFrames < 2) return false;
            current = candidate; stableFrames = 0; return true;
        }
    }
    public sealed class RemoteEntity { public EntityId id; public PlayerState state; }
    public static class Wire
    {
        public const int Version = 1;
        public const int MaxFrame = 65536;
        public const string PipeName = "gameverse-gta-v1";
        public const string GameBuild = "1.0.1158.13";
        private static readonly UTF8Encoding Utf8 = new UTF8Encoding(false, true);
        public static JObject Message(string type) => new JObject { ["type"] = type };
        public static byte[] Encode(JObject message)
        {
            byte[] body = Utf8.GetBytes(message.ToString(Formatting.None));
            if (body.Length == 0 || body.Length > MaxFrame) throw new InvalidDataException("Frame length");
            byte[] frame = new byte[body.Length + 4];
            frame[0] = (byte)(body.Length >> 24); frame[1] = (byte)(body.Length >> 16); frame[2] = (byte)(body.Length >> 8); frame[3] = (byte)body.Length;
            Buffer.BlockCopy(body, 0, frame, 4, body.Length); return frame;
        }
        public static async Task<JObject> Read(Stream stream, CancellationToken cancel)
        {
            byte[] prefix = new byte[4]; await Exact(stream, prefix, cancel).ConfigureAwait(false);
            uint length = ((uint)prefix[0] << 24) | ((uint)prefix[1] << 16) | ((uint)prefix[2] << 8) | prefix[3];
            if (length == 0 || length > MaxFrame) throw new InvalidDataException("Frame length");
            byte[] body = new byte[length]; await Exact(stream, body, cancel).ConfigureAwait(false);
            using (var reader = new JsonTextReader(new StringReader(Utf8.GetString(body))) { MaxDepth = 16, DateParseHandling = DateParseHandling.None })
            {
                var value = JObject.Load(reader, new JsonLoadSettings { DuplicatePropertyNameHandling = DuplicatePropertyNameHandling.Error });
                if (reader.Read()) throw new InvalidDataException("Trailing JSON");
                if (value["type"]?.Type != JTokenType.String) throw new InvalidDataException("Missing message type");
                return value;
            }
        }
        private static async Task Exact(Stream stream, byte[] buffer, CancellationToken cancel)
        {
            int offset = 0;
            while (offset < buffer.Length)
            {
                int n = await stream.ReadAsync(buffer, offset, buffer.Length - offset, cancel).ConfigureAwait(false);
                if (n == 0) throw new EndOfStreamException(); offset += n;
            }
        }
        public static async Task Write(Stream stream, JObject message, CancellationToken cancel)
        {
            byte[] bytes = Encode(message);
            Task write = stream.WriteAsync(bytes, 0, bytes.Length, cancel);
            if (await Task.WhenAny(write, Task.Delay(5000, cancel)).ConfigureAwait(false) != write) throw new IOException("Pipe write timeout");
            await write.ConfigureAwait(false);
            await stream.FlushAsync(cancel).ConfigureAwait(false);
        }
        public static RemoteEntity Entity(JObject message)
        {
            var entity = message["entity"]?.ToObject<RemoteEntity>();
            if (entity?.id == null || entity.id.generation == 0 || entity.id.slot >= 32 || entity.state == null || !entity.state.IsValid()) throw new InvalidDataException("Invalid remote entity");
            return entity;
        }
    }
}
