//! M1 validates and relays client-owned poses; it does not simulate GTA physics.
use crate::Error;
use gameverse_protocol::{
    presence::{Entity, PlayerState, Snapshot},
    EntityId, SessionId, MAX_PLAYERS,
};
use std::collections::BTreeMap;
struct Player {
    id: EntityId,
    state: Option<PlayerState>,
    sequence: u64,
}
#[derive(Default)]
pub struct World {
    players: BTreeMap<SessionId, Player>,
    generations: [u64; MAX_PLAYERS],
    next_session: u64,
    tick: u64,
}
impl World {
    pub fn connect(&mut self) -> Result<(SessionId, EntityId), Error> {
        let slot = (0..MAX_PLAYERS)
            .find(|s| !self.players.values().any(|p| p.id.slot == *s as u32))
            .ok_or(Error::Full)?;
        self.generations[slot] += 1;
        self.next_session += 1;
        let id = EntityId {
            slot: slot as u32,
            generation: self.generations[slot],
        };
        self.players.insert(
            self.next_session,
            Player {
                id,
                state: None,
                sequence: 0,
            },
        );
        Ok((self.next_session, id))
    }
    pub fn input(
        &mut self,
        session: SessionId,
        sequence: u64,
        state: PlayerState,
    ) -> Result<bool, Error> {
        if sequence == 0 || !state.valid() {
            return Err(Error::InvalidInput);
        }
        let player = self
            .players
            .get_mut(&session)
            .ok_or(Error::UnknownSession)?;
        if sequence <= player.sequence {
            return Ok(false);
        }
        if player
            .state
            .as_ref()
            .is_some_and(|old| state.timestamp_ms < old.timestamp_ms)
        {
            return Err(Error::InvalidInput);
        }
        player.sequence = sequence;
        player.state = Some(state);
        Ok(true)
    }
    pub fn disconnect(&mut self, session: SessionId) {
        self.players.remove(&session);
    }
    pub fn step(&mut self) {
        self.tick += 1;
    }
    pub fn tick(&self) -> u64 {
        self.tick
    }
    pub fn players(&self) -> usize {
        self.players.len()
    }
    pub fn snapshot(&self, session: SessionId) -> Result<Snapshot, Error> {
        let p = self.players.get(&session).ok_or(Error::UnknownSession)?;
        Ok(Snapshot {
            tick: self.tick,
            ack: p.sequence,
            entities: self
                .players
                .values()
                .filter_map(|p| p.state.clone().map(|state| Entity { id: p.id, state }))
                .collect(),
        })
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn owner_state_stale_sequence_and_generation() {
        let s = PlayerState {
            timestamp_ms: 10,
            position: [1.0, 2.0, 3.0],
            rotation: [0.0, 0.0, 0.0, 1.0],
            velocity: [0.0; 3],
            model_hash: 1,
            health: 200,
            armor: 0,
            movement: 1,
            weapon_hash: 0,
        };
        let mut world = World::default();
        let (a, id) = world.connect().unwrap();
        let (b, _) = world.connect().unwrap();
        assert!(world.snapshot(b).unwrap().entities.is_empty());
        assert!(world.input(a, 2, s.clone()).unwrap());
        assert!(!world.input(a, 1, s.clone()).unwrap());
        assert_eq!(world.input(999, 1, s.clone()), Err(Error::UnknownSession));
        let mut old = s;
        old.timestamp_ms = 9;
        assert_eq!(world.input(a, 3, old), Err(Error::InvalidInput));
        assert_eq!(world.snapshot(b).unwrap().entities[0].id, id);
        world.disconnect(a);
        assert!(world.snapshot(b).unwrap().entities.is_empty());
        let (_, new) = world.connect().unwrap();
        assert_eq!(id.slot, new.slot);
        assert_ne!(id.generation, new.generation);
    }
}
