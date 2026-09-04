//! Server-owned identity, interest and latest-baseline delta generation for Presence v2.
use gameverse_protocol::{presence_v2 as p, EntityId, SessionId};
use std::collections::{BTreeMap, BTreeSet};

const GRID_CELL_SIZE: f32 = 250.0;

#[derive(Debug, thiserror::Error, PartialEq)]
pub enum Error {
    #[error("server full")]
    Full,
    #[error("unknown session")]
    UnknownSession,
    #[error("invalid frame")]
    InvalidFrame,
}

#[derive(Clone)]
struct EntityState {
    frame: p::PlayerFrame,
}
struct Peer {
    entity: EntityId,
    sequence: u64,
    visible: BTreeSet<EntityId>,
    baseline_state: BTreeMap<EntityId, p::PlayerFrame>,
    pending_destroy: BTreeSet<EntityId>,
    baseline: u64,
}

#[derive(Default)]
pub struct World {
    peers: BTreeMap<SessionId, Peer>,
    entities: BTreeMap<EntityId, EntityState>,
    generations: [u64; p::MAX_PLAYERS],
    next_session: SessionId,
    server_tick: u64,
}
impl World {
    pub fn connect(&mut self) -> Result<(SessionId, EntityId), Error> {
        let slot = (0..p::MAX_PLAYERS)
            .find(|slot| {
                !self
                    .peers
                    .values()
                    .any(|peer| peer.entity.slot == *slot as u32)
            })
            .ok_or(Error::Full)?;
        self.generations[slot] += 1;
        self.next_session += 1;
        let entity = EntityId {
            slot: slot as u32,
            generation: self.generations[slot],
        };
        self.peers.insert(
            self.next_session,
            Peer {
                entity,
                sequence: 0,
                visible: BTreeSet::new(),
                baseline_state: BTreeMap::new(),
                pending_destroy: BTreeSet::new(),
                baseline: 0,
            },
        );
        Ok((self.next_session, entity))
    }
    pub fn publish(&mut self, session: SessionId, frame: p::PlayerFrame) -> Result<bool, Error> {
        if !frame.valid() {
            return Err(Error::InvalidFrame);
        }
        let peer = self.peers.get_mut(&session).ok_or(Error::UnknownSession)?;
        if frame.sequence <= peer.sequence {
            return Ok(false);
        }
        peer.sequence = frame.sequence;
        self.entities.insert(peer.entity, EntityState { frame });
        Ok(true)
    }
    pub fn disconnect(&mut self, session: SessionId) -> Result<EntityId, Error> {
        let peer = self.peers.remove(&session).ok_or(Error::UnknownSession)?;
        self.entities.remove(&peer.entity);
        for observer in self.peers.values_mut() {
            if observer.visible.remove(&peer.entity)
                || observer.baseline_state.remove(&peer.entity).is_some()
            {
                observer.pending_destroy.insert(peer.entity);
            }
        }
        Ok(peer.entity)
    }
    pub fn step(&mut self) {
        self.server_tick += 1;
    }
    pub fn delta(&mut self, session: SessionId) -> Result<p::ServerFrame, Error> {
        let own_id = self
            .peers
            .get(&session)
            .ok_or(Error::UnknownSession)?
            .entity;
        let own = self
            .entities
            .get(&own_id)
            .ok_or(Error::InvalidFrame)?
            .frame
            .transform
            .clone();
        let grid = self.spatial_grid();
        let next_visible = self.visible_from(&grid, own_id, &own);
        let (previous_visible, previous_state, pending_destroy) = {
            let peer = self.peers.get(&session).expect("session checked above");
            (
                peer.visible.clone(),
                peer.baseline_state.clone(),
                peer.pending_destroy.clone(),
            )
        };
        let mut deltas = Vec::new();
        for id in &pending_destroy {
            deltas.push(empty(*id, p::DeltaKind::Destroy));
        }
        for id in previous_visible.difference(&next_visible) {
            if !pending_destroy.contains(id) {
                deltas.push(empty(*id, p::DeltaKind::StreamOut));
            }
        }
        for id in &next_visible {
            let frame = &self.entities[id].frame;
            if let Some(delta) = component_delta(*id, previous_state.get(id), frame) {
                deltas.push(delta);
            }
        }
        let peer = self.peers.get_mut(&session).expect("session checked above");
        peer.visible = next_visible;
        peer.baseline_state = peer
            .visible
            .iter()
            .filter_map(|id| self.entities.get(id).map(|s| (*id, s.frame.clone())))
            .collect();
        peer.pending_destroy.clear();
        peer.baseline += 1;
        Ok(p::ServerFrame {
            server_tick: self.server_tick,
            baseline: peer.baseline,
            deltas,
        })
    }

    fn spatial_grid(&self) -> BTreeMap<(i32, i32), Vec<EntityId>> {
        let mut grid = BTreeMap::<_, Vec<_>>::new();
        for (id, state) in &self.entities {
            grid.entry(cell(&state.frame.transform))
                .or_default()
                .push(*id);
        }
        grid
    }

    fn visible_from(
        &self,
        grid: &BTreeMap<(i32, i32), Vec<EntityId>>,
        own_id: EntityId,
        own: &p::Transform,
    ) -> BTreeSet<EntityId> {
        let center = cell(own);
        let reach = (p::INTEREST_RADIUS / GRID_CELL_SIZE).ceil() as i32;
        let mut visible = BTreeSet::new();
        for x in center.0 - reach..=center.0 + reach {
            for y in center.1 - reach..=center.1 + reach {
                if let Some(ids) = grid.get(&(x, y)) {
                    for id in ids {
                        if *id != own_id
                            && own.distance_squared(&self.entities[id].frame.transform)
                                <= p::INTEREST_RADIUS.powi(2)
                        {
                            visible.insert(*id);
                        }
                    }
                }
            }
        }
        visible
    }
}
fn cell(transform: &p::Transform) -> (i32, i32) {
    (
        (transform.position[0] / GRID_CELL_SIZE).floor() as i32,
        (transform.position[1] / GRID_CELL_SIZE).floor() as i32,
    )
}
fn empty(id: EntityId, kind: p::DeltaKind) -> p::EntityDelta {
    p::EntityDelta {
        id,
        kind,
        transform: None,
        appearance: None,
        locomotion: None,
        combat: None,
        vehicle: None,
        cleared: vec![],
    }
}
fn component_delta(
    id: EntityId,
    old: Option<&p::PlayerFrame>,
    new: &p::PlayerFrame,
) -> Option<p::EntityDelta> {
    let mut delta = p::EntityDelta {
        id,
        kind: p::DeltaKind::Upsert,
        transform: old
            .is_none_or(|old| old.transform != new.transform)
            .then(|| new.transform.clone()),
        appearance: old
            .is_none_or(|old| old.appearance != new.appearance)
            .then(|| new.appearance.clone())
            .flatten(),
        locomotion: old
            .is_none_or(|old| old.locomotion != new.locomotion)
            .then_some(new.locomotion),
        combat: old
            .is_none_or(|old| old.combat != new.combat)
            .then(|| new.combat.clone()),
        vehicle: old
            .is_none_or(|old| old.vehicle != new.vehicle)
            .then(|| new.vehicle.clone())
            .flatten(),
        cleared: Vec::new(),
    };
    if let Some(old) = old {
        if old.appearance.is_some() && new.appearance.is_none() {
            delta.cleared.push(p::ComponentKind::Appearance);
        }
        if old.vehicle.is_some() && new.vehicle.is_none() {
            delta.cleared.push(p::ComponentKind::Vehicle);
        }
    }
    delta.valid().then_some(delta)
}

#[cfg(test)]
mod tests {
    use super::*;
    fn frame(sequence: u64, x: f32) -> p::PlayerFrame {
        p::PlayerFrame {
            sequence,
            client_tick: sequence,
            transform: p::Transform {
                position: [x, 0.0, 0.0],
                rotation: [0.0, 0.0, 0.0, 1.0],
                velocity: [0.0; 3],
            },
            appearance: Some(p::Appearance { model_hash: 1 }),
            locomotion: p::Locomotion::Idle,
            combat: p::CombatPresentation {
                aiming: false,
                shooting: false,
                reloading: false,
                dead: false,
                weapon_hash: 0,
                aim_target: None,
            },
            vehicle: None,
        }
    }
    #[test]
    fn capacity_interest_stream_out_and_generation() {
        let mut world = World::default();
        let (a, _) = world.connect().unwrap();
        let (b, bid) = world.connect().unwrap();
        world.publish(a, frame(1, 0.0)).unwrap();
        world.publish(b, frame(1, 10.0)).unwrap();
        assert_eq!(world.delta(a).unwrap().deltas[0].kind, p::DeltaKind::Upsert);
        assert!(world.delta(a).unwrap().deltas.is_empty());
        world.publish(b, frame(2, 500.0)).unwrap();
        assert_eq!(
            world.delta(a).unwrap().deltas[0].kind,
            p::DeltaKind::StreamOut
        );
        world.publish(b, frame(3, 10.0)).unwrap();
        assert_eq!(world.delta(a).unwrap().deltas[0].kind, p::DeltaKind::Upsert);
        world.disconnect(b).unwrap();
        assert_eq!(
            world.delta(a).unwrap().deltas[0].kind,
            p::DeltaKind::Destroy
        );
        let (_, next) = world.connect().unwrap();
        assert_eq!(bid.slot, next.slot);
        assert!(next.generation > bid.generation);
        for i in 2..p::MAX_PLAYERS {
            let _ = i;
            world.connect().unwrap();
        }
        assert_eq!(world.connect(), Err(Error::Full));
    }
    #[test]
    fn thirty_two_clients_remain_bounded_and_emit_only_changes() {
        let mut world = World::default();
        let mut sessions = Vec::new();
        for slot in 0..p::MAX_PLAYERS {
            let (session, _) = world.connect().unwrap();
            world.publish(session, frame(1, slot as f32 * 2.0)).unwrap();
            sessions.push(session);
        }
        let initial = world.delta(sessions[0]).unwrap();
        assert_eq!(initial.deltas.len(), p::MAX_PLAYERS - 1);
        assert!(
            gameverse_protocol::presence_v2::encode_frame(&initial)
                .unwrap()
                .len()
                <= gameverse_protocol::MAX_FRAME
        );
        assert!(world.delta(sessions[0]).unwrap().deltas.is_empty());

        for tick in 2..=100 {
            for (slot, session) in sessions.iter().enumerate() {
                world
                    .publish(
                        *session,
                        frame(tick, slot as f32 * 2.0 + tick as f32 * 0.01),
                    )
                    .unwrap();
            }
            world.step();
            let delta = world.delta(sessions[0]).unwrap();
            assert!(delta.valid());
            assert!(delta.deltas.len() < p::MAX_PLAYERS);
        }
    }
}
