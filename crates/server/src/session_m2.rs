//! Strict M2 control-plane state machine. It does not allocate a world entity
//! until a character is selected and it rejects realtime input before spawn.
use std::time::Duration;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Phase {
    Connected,
    Negotiated,
    Authenticated,
    CharacterSelected,
    SpawnPending,
    Active,
    Closing,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TransitionError {
    InvalidState,
    InvalidIdentity,
    TimedOut,
}

impl std::fmt::Display for TransitionError {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        formatter.write_str(match self {
            Self::InvalidState => "command is not valid in the current session state",
            Self::InvalidIdentity => "account or character identity is invalid",
            Self::TimedOut => "session state timed out",
        })
    }
}
impl std::error::Error for TransitionError {}

#[derive(Debug)]
pub struct SessionMachine {
    phase: Phase,
    entered_at_ms: u64,
    account_id: Option<u64>,
    character_id: Option<u64>,
}

impl SessionMachine {
    pub fn new(now_ms: u64) -> Self {
        Self {
            phase: Phase::Connected,
            entered_at_ms: now_ms,
            account_id: None,
            character_id: None,
        }
    }

    pub fn phase(&self) -> Phase {
        self.phase
    }
    pub fn account_id(&self) -> Option<u64> {
        self.account_id
    }
    pub fn character_id(&self) -> Option<u64> {
        self.character_id
    }

    pub fn negotiated(&mut self, now_ms: u64) -> Result<(), TransitionError> {
        self.transition(Phase::Connected, Phase::Negotiated, now_ms)
    }

    pub fn authenticated(&mut self, account_id: u64, now_ms: u64) -> Result<(), TransitionError> {
        if account_id == 0 {
            return Err(TransitionError::InvalidIdentity);
        }
        self.transition(Phase::Negotiated, Phase::Authenticated, now_ms)?;
        self.account_id = Some(account_id);
        Ok(())
    }

    pub fn select_character(
        &mut self,
        character_id: u64,
        owner_account_id: u64,
        now_ms: u64,
    ) -> Result<(), TransitionError> {
        if character_id == 0 || Some(owner_account_id) != self.account_id {
            return Err(TransitionError::InvalidIdentity);
        }
        self.transition(Phase::Authenticated, Phase::CharacterSelected, now_ms)?;
        self.character_id = Some(character_id);
        Ok(())
    }

    pub fn begin_spawn(&mut self, now_ms: u64) -> Result<(), TransitionError> {
        self.transition(Phase::CharacterSelected, Phase::SpawnPending, now_ms)
    }

    pub fn spawn_ready(&mut self, now_ms: u64) -> Result<(), TransitionError> {
        self.transition(Phase::SpawnPending, Phase::Active, now_ms)
    }

    pub fn ensure_realtime_allowed(&self) -> Result<(), TransitionError> {
        if self.phase == Phase::Active {
            Ok(())
        } else {
            Err(TransitionError::InvalidState)
        }
    }

    pub fn ensure_not_timed_out(&self, now_ms: u64) -> Result<(), TransitionError> {
        if now_ms.saturating_sub(self.entered_at_ms) <= self.timeout().as_millis() as u64 {
            Ok(())
        } else {
            Err(TransitionError::TimedOut)
        }
    }

    pub fn close(&mut self, now_ms: u64) {
        self.phase = Phase::Closing;
        self.entered_at_ms = now_ms;
    }

    pub fn timeout(&self) -> Duration {
        match self.phase {
            Phase::Connected => Duration::from_secs(15),
            Phase::Negotiated | Phase::Authenticated => Duration::from_secs(10 * 60),
            Phase::CharacterSelected => Duration::from_secs(2 * 60),
            Phase::SpawnPending => Duration::from_secs(30),
            Phase::Active => Duration::from_secs(120),
            Phase::Closing => Duration::from_secs(5),
        }
    }

    fn transition(
        &mut self,
        expected: Phase,
        next: Phase,
        now_ms: u64,
    ) -> Result<(), TransitionError> {
        self.ensure_not_timed_out(now_ms)?;
        if self.phase != expected {
            return Err(TransitionError::InvalidState);
        }
        self.phase = next;
        self.entered_at_ms = now_ms;
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn enforces_full_path_and_blocks_realtime_until_active() {
        let mut session = SessionMachine::new(0);
        assert_eq!(
            session.ensure_realtime_allowed(),
            Err(TransitionError::InvalidState)
        );
        session.negotiated(1).unwrap();
        session.authenticated(7, 2).unwrap();
        assert_eq!(
            session.select_character(9, 8, 3),
            Err(TransitionError::InvalidIdentity)
        );
        session.select_character(9, 7, 3).unwrap();
        session.begin_spawn(4).unwrap();
        session.spawn_ready(5).unwrap();
        assert_eq!(session.phase(), Phase::Active);
        assert_eq!(session.account_id(), Some(7));
        assert_eq!(session.character_id(), Some(9));
        session.ensure_realtime_allowed().unwrap();
    }

    #[test]
    fn rejects_skipped_states_and_expired_state() {
        let mut session = SessionMachine::new(0);
        assert_eq!(
            session.authenticated(1, 1),
            Err(TransitionError::InvalidState)
        );
        assert_eq!(session.negotiated(15_001), Err(TransitionError::TimedOut));
    }

    #[test]
    fn allows_interactive_authentication_and_character_selection() {
        let mut session = SessionMachine::new(0);
        session.negotiated(1).unwrap();
        session.authenticated(7, 599_999).unwrap();
        session.select_character(9, 7, 1_199_998).unwrap();
        assert_eq!(session.phase(), Phase::CharacterSelected);
        assert_eq!(session.timeout(), Duration::from_secs(2 * 60));
    }
}
