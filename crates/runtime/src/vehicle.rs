//! Server-owned vehicle identity, occupancy and simulation-owner migration.
use gameverse_protocol::{
    presence_v2::{VehicleFrame, VehicleId, MAX_VEHICLES},
    SessionId,
};
use std::collections::{BTreeMap, BTreeSet};

#[derive(Debug, thiserror::Error, PartialEq, Eq)]
pub enum Error {
    #[error("vehicle capacity reached")]
    Full,
    #[error("unknown vehicle")]
    UnknownVehicle,
    #[error("seat already occupied")]
    SeatOccupied,
    #[error("session is not the simulation owner")]
    NotOwner,
    #[error("invalid vehicle frame")]
    InvalidFrame,
}

struct Vehicle {
    owner: Option<SessionId>,
    occupants: BTreeMap<i8, SessionId>,
    interested: BTreeSet<SessionId>,
    frame: Option<VehicleFrame>,
}

pub struct VehicleWorld {
    vehicles: BTreeMap<VehicleId, Vehicle>,
    generations: [u64; MAX_VEHICLES],
}

impl Default for VehicleWorld {
    fn default() -> Self {
        Self {
            vehicles: BTreeMap::new(),
            generations: [0; MAX_VEHICLES],
        }
    }
}

impl VehicleWorld {
    pub fn spawn(&mut self) -> Result<VehicleId, Error> {
        let slot = (0..MAX_VEHICLES)
            .find(|slot| !self.vehicles.keys().any(|id| id.slot == *slot as u32))
            .ok_or(Error::Full)?;
        self.generations[slot] += 1;
        let id = VehicleId {
            slot: slot as u32,
            generation: self.generations[slot],
        };
        self.vehicles.insert(
            id,
            Vehicle {
                owner: None,
                occupants: BTreeMap::new(),
                interested: BTreeSet::new(),
                frame: None,
            },
        );
        Ok(id)
    }

    pub fn set_interested(
        &mut self,
        id: VehicleId,
        sessions: impl IntoIterator<Item = SessionId>,
    ) -> Result<Option<SessionId>, Error> {
        let vehicle = self.vehicles.get_mut(&id).ok_or(Error::UnknownVehicle)?;
        vehicle.interested = sessions.into_iter().collect();
        migrate_owner(vehicle);
        Ok(vehicle.owner)
    }

    pub fn enter(
        &mut self,
        id: VehicleId,
        session: SessionId,
        seat: i8,
    ) -> Result<Option<SessionId>, Error> {
        let vehicle = self.vehicles.get_mut(&id).ok_or(Error::UnknownVehicle)?;
        if vehicle
            .occupants
            .get(&seat)
            .is_some_and(|occupant| *occupant != session)
        {
            return Err(Error::SeatOccupied);
        }
        vehicle.occupants.retain(|_, occupant| *occupant != session);
        vehicle.occupants.insert(seat, session);
        vehicle.interested.insert(session);
        migrate_owner(vehicle);
        Ok(vehicle.owner)
    }

    pub fn leave(&mut self, id: VehicleId, session: SessionId) -> Result<Option<SessionId>, Error> {
        let vehicle = self.vehicles.get_mut(&id).ok_or(Error::UnknownVehicle)?;
        vehicle.occupants.retain(|_, occupant| *occupant != session);
        migrate_owner(vehicle);
        Ok(vehicle.owner)
    }

    pub fn disconnect(&mut self, session: SessionId) {
        for vehicle in self.vehicles.values_mut() {
            vehicle.occupants.retain(|_, occupant| *occupant != session);
            vehicle.interested.remove(&session);
            migrate_owner(vehicle);
        }
    }

    pub fn publish(
        &mut self,
        id: VehicleId,
        session: SessionId,
        frame: VehicleFrame,
    ) -> Result<bool, Error> {
        if !frame.valid() {
            return Err(Error::InvalidFrame);
        }
        let vehicle = self.vehicles.get_mut(&id).ok_or(Error::UnknownVehicle)?;
        if vehicle.owner != Some(session) {
            return Err(Error::NotOwner);
        }
        if vehicle
            .frame
            .as_ref()
            .is_some_and(|old| old.sequence >= frame.sequence)
        {
            return Ok(false);
        }
        vehicle.frame = Some(frame);
        Ok(true)
    }

    pub fn owner(&self, id: VehicleId) -> Result<Option<SessionId>, Error> {
        Ok(self.vehicles.get(&id).ok_or(Error::UnknownVehicle)?.owner)
    }
}

fn migrate_owner(vehicle: &mut Vehicle) {
    let driver = vehicle
        .occupants
        .get(&-1)
        .copied()
        .filter(|session| vehicle.interested.contains(session));
    vehicle.owner = driver
        .or_else(|| {
            vehicle
                .occupants
                .values()
                .copied()
                .find(|session| vehicle.interested.contains(session))
        })
        .or_else(|| vehicle.interested.iter().next().copied());
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn driver_owns_then_migrates_to_passenger_on_disconnect() {
        let mut world = VehicleWorld::default();
        let id = world.spawn().unwrap();
        world.set_interested(id, [10, 20]).unwrap();
        world.enter(id, 20, 0).unwrap();
        assert_eq!(world.enter(id, 10, -1).unwrap(), Some(10));
        assert_eq!(world.enter(id, 30, -1), Err(Error::SeatOccupied));
        world.disconnect(10);
        assert_eq!(world.owner(id).unwrap(), Some(20));
        world.set_interested(id, []).unwrap();
        assert_eq!(world.owner(id).unwrap(), None);
    }
}
