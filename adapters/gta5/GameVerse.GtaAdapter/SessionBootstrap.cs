using System;
using GTA;
using GTA.Math;
using GTA.Native;
using GameVerse.AdapterProtocol;

namespace GameVerse.GtaAdapter
{
    internal sealed class SessionBootstrap
    {
        private SessionConfig pending;
        private Model model;
        private bool applied;
        private bool configurationApplied;
        public bool Applied => applied;
        public string Error { get; private set; }

        public void Begin(SessionConfig config)
        {
            pending = config != null && config.IsValid() && IsFreemode(config.model_hash) ? config : null;
            applied = false;
            configurationApplied = false;
            Error = pending == null ? "invalid_session_config" : null;
            model = pending == null ? null : new Model(unchecked((int)pending.model_hash));
            Function.Call(Hash.DO_SCREEN_FADE_OUT, 250);
            Function.Call(Hash.SET_PLAYER_CONTROL, Game.Player.Handle, false, 0);
        }

        public bool Tick()
        {
            if (pending == null) return applied;
            Function.Call(Hash.HIDE_HUD_AND_RADAR_THIS_FRAME);
            Function.Call(Hash.SET_MAX_WANTED_LEVEL, 0);
            Function.Call(Hash.SET_PLAYER_WANTED_LEVEL, Game.Player.Handle, 0, false);
            Function.Call(Hash.SET_PLAYER_WANTED_LEVEL_NOW, Game.Player.Handle, false);
            Function.Call(Hash.SET_MISSION_FLAG, false);
            if (Function.Call<bool>(Hash.IS_CUTSCENE_ACTIVE)) Function.Call(Hash.STOP_CUTSCENE_IMMEDIATELY);
            if (Function.Call<bool>(Hash.IS_PLAYER_SWITCH_IN_PROGRESS)) Function.Call(Hash.STOP_PLAYER_SWITCH);
            if (applied)
            {
                Ped activePlayer = Game.Player.Character;
                if (activePlayer == null || !activePlayer.Exists() ||
                    IsStoryProtagonist(unchecked((uint)activePlayer.Model.Hash)) ||
                    unchecked((uint)activePlayer.Model.Hash) != pending.model_hash ||
                    Function.Call<bool>(Hash.IS_CUTSCENE_ACTIVE) ||
                    Function.Call<bool>(Hash.GET_MISSION_FLAG) ||
                    Function.Call<bool>(Hash.IS_PLAYER_SWITCH_IN_PROGRESS))
                {
                    Fail("story_isolation_failed");
                    return false;
                }
                return true;
            }
            if (!model.IsInCdImage || !model.IsPed) { Fail("invalid_model"); return false; }
            if (!model.IsLoaded) { model.Request(); return false; }

            Function.Call(Hash.SET_PLAYER_MODEL, Game.Player.Handle, model.Hash);
            model.MarkAsNoLongerNeeded();
            Ped player = Game.Player.Character;
            if (player == null || !player.Exists()) return false;
            if (configurationApplied)
            {
                var position = player.Position;
                float dx = position.X - pending.spawn[0], dy = position.Y - pending.spawn[1], dz = position.Z - pending.spawn[2];
                float headingError = Math.Abs(player.Heading - pending.heading) % 360f;
                if (headingError > 180f) headingError = 360f - headingError;
                if (unchecked((uint)player.Model.Hash) != pending.model_hash) { Fail("model_verification_failed"); return false; }
                if (dx * dx + dy * dy + dz * dz > 4f || headingError > 5f) { Fail("spawn_verification_failed"); return false; }
                applied = true;
                return true;
            }
            Function.Call(Hash.CLEAR_PED_TASKS_IMMEDIATELY, player.Handle);
            Function.Call(Hash.REQUEST_COLLISION_AT_COORD, pending.spawn[0], pending.spawn[1], pending.spawn[2]);
            ApplyAppearance(player);
            Function.Call(Hash.SET_ENTITY_COORDS_NO_OFFSET, player.Handle,
                pending.spawn[0], pending.spawn[1], pending.spawn[2], false, false, false);
            Function.Call(Hash.SET_ENTITY_HEADING, player.Handle, pending.heading);
            Function.Call(Hash.CLEAR_PED_TASKS_IMMEDIATELY, player.Handle);
            configurationApplied = true;
            return false;
        }

        public void Activate()
        {
            if (!applied) return;
            Function.Call(Hash.SET_PLAYER_CONTROL, Game.Player.Handle, true, 0);
            Function.Call(Hash.DO_SCREEN_FADE_IN, 350);
        }

        private void Fail(string code)
        {
            Reset();
            Error = code;
        }

        public void Reset()
        {
            if (model != null) model.MarkAsNoLongerNeeded();
            pending = null;
            model = null;
            applied = false;
            configurationApplied = false;
            Error = null;
            Function.Call(Hash.SET_PLAYER_CONTROL, Game.Player.Handle, true, 0);
            Function.Call(Hash.DO_SCREEN_FADE_IN, 0);
        }

        private void ApplyAppearance(Ped player)
        {
            if (pending.appearance == null) return;
            for (int component = 0; component <= 11; component++)
            {
                short drawable;
                if (!pending.appearance.TryGetValue("component_" + component + "_drawable", out drawable)) continue;
                short texture;
                pending.appearance.TryGetValue("component_" + component + "_texture", out texture);
                if (drawable < 0 || texture < 0) continue;
                Function.Call(Hash.SET_PED_COMPONENT_VARIATION, player.Handle, component, (int)drawable, (int)texture, 0);
            }
        }

        private static bool IsFreemode(uint hash) => hash == 0x705e61f2u || hash == 0x9c9effd8u;
        private static bool IsStoryProtagonist(uint hash) =>
            hash == 0x0d7114c9u || hash == 0x9b22dbafu || hash == 0x9b810fa2u;
    }
}
