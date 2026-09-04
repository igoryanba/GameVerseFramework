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
        public bool Applied => applied;
        public string Error { get; private set; }

        public void Begin(SessionConfig config)
        {
            pending = config != null && config.IsValid() ? config : null;
            applied = false;
            Error = null;
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
            if (applied) return true;
            if (!model.IsInCdImage || !model.IsPed) { Fail("invalid_model"); return false; }
            if (!model.IsLoaded) { model.Request(); return false; }

            Function.Call(Hash.SET_PLAYER_MODEL, Game.Player.Handle, model.Hash);
            model.MarkAsNoLongerNeeded();
            Ped player = Game.Player.Character;
            if (player == null || !player.Exists()) return false;
            Function.Call(Hash.CLEAR_PED_TASKS_IMMEDIATELY, player.Handle);
            Function.Call(Hash.SET_ENTITY_COORDS_NO_OFFSET, player.Handle,
                pending.spawn[0], pending.spawn[1], pending.spawn[2], false, false, false);
            Function.Call(Hash.SET_ENTITY_HEADING, player.Handle, pending.heading);
            applied = true;
            return true;
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
            Error = null;
            Function.Call(Hash.SET_PLAYER_CONTROL, Game.Player.Handle, true, 0);
            Function.Call(Hash.DO_SCREEN_FADE_IN, 0);
        }
    }
}
