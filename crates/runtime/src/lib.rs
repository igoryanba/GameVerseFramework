//! Deterministic authoritative world, independent of transport and game adapters.
pub mod presence;
use gameverse_protocol::{
    valid_direction, Entity, EntityId, SessionId, Snapshot, Tick, MAX_PLAYERS,
};
use std::collections::BTreeMap;

pub const STEP_MS: u64 = 50;
pub const SPEED: f32 = 5.0;

#[derive(Debug, thiserror::Error, PartialEq)]
pub enum Error {
    #[error("server full")]
    Full,
    #[error("unknown session")]
    UnknownSession,
    #[error("invalid input")]
    InvalidInput,
}

struct Player {
    entity: Entity,
    direction: [f32; 2],
    sequence: u64,
}

#[derive(Default)]
pub struct World {
    players: BTreeMap<SessionId, Player>,
    generations: [u64; MAX_PLAYERS],
    next_session: SessionId,
    tick: Tick,
}

impl World {
    pub fn connect(&mut self) -> Result<(SessionId, EntityId), Error> {
        let slot = (0..MAX_PLAYERS)
            .find(|s| !self.players.values().any(|p| p.entity.id.slot == *s as u32))
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
                entity: Entity {
                    id,
                    position: [0.0; 2],
                },
                direction: [0.0; 2],
                sequence: 0,
            },
        );
        Ok((self.next_session, id))
    }
    pub fn disconnect(&mut self, session: SessionId) {
        self.players.remove(&session);
    }
    pub fn input(
        &mut self,
        session: SessionId,
        sequence: u64,
        mut direction: [f32; 2],
    ) -> Result<bool, Error> {
        if sequence == 0 || !valid_direction(direction) {
            return Err(Error::InvalidInput);
        }
        let player = self
            .players
            .get_mut(&session)
            .ok_or(Error::UnknownSession)?;
        if sequence <= player.sequence {
            return Ok(false);
        }
        let length = direction[0].hypot(direction[1]);
        if length > 1.0 {
            direction.iter_mut().for_each(|v| *v /= length);
        }
        player.direction = direction;
        player.sequence = sequence;
        Ok(true)
    }
    pub fn step(&mut self) {
        self.tick += 1;
        for player in self.players.values_mut() {
            for axis in 0..2 {
                player.entity.position[axis] +=
                    player.direction[axis] * SPEED * STEP_MS as f32 / 1000.0;
            }
        }
    }
    pub fn snapshot(&self, session: SessionId) -> Result<Snapshot, Error> {
        let player = self.players.get(&session).ok_or(Error::UnknownSession)?;
        Ok(Snapshot {
            tick: self.tick,
            ack: player.sequence,
            entities: self.players.values().map(|p| p.entity.clone()).collect(),
        })
    }
    pub fn tick(&self) -> Tick {
        self.tick
    }
    pub fn players(&self) -> usize {
        self.players.len()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn replay_speed_ownership_and_stale_input() {
        fn run() -> Snapshot {
            let mut w = World::default();
            let (a, _) = w.connect().unwrap();
            let (_b, _) = w.connect().unwrap();
            assert_eq!(w.connect(), Err(Error::Full));
            w.input(a, 2, [1.0, 1.0]).unwrap();
            assert!(!w.input(a, 1, [-1.0, 0.0]).unwrap());
            assert!(!w.input(a, 2, [-1.0, 0.0]).unwrap());
            assert_eq!(w.input(999, 1, [0.0, 0.0]), Err(Error::UnknownSession));
            for _ in 0..20 {
                w.step();
            }
            let s = w.snapshot(a).unwrap();
            assert!(
                (s.entities[0].position[0].hypot(s.entities[0].position[1]) - SPEED).abs() < 0.0001
            );
            assert_eq!(s.entities[1].position, [0.0; 2]);
            s
        }
        assert_eq!(run(), run());
    }
    #[test]
    fn reconnect_reuses_slot_with_new_generation() {
        let mut w = World::default();
        let (s, e) = w.connect().unwrap();
        w.disconnect(s);
        let (next, new) = w.connect().unwrap();
        assert_ne!(s, next);
        assert_eq!(e.slot, new.slot);
        assert!(new.generation > e.generation);
        assert_eq!(w.snapshot(s), Err(Error::UnknownSession));
    }
}
