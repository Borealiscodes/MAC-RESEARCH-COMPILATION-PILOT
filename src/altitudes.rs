// MACv1.0 — Altitude Kernel
// Altitude A3.5 → A4
// Deterministic runtime altitude state machine

use crate::provenance::ProvenanceLog;
use crate::diagnostics::{AltitudeDrift, DeterminismError};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Altitude {
    A0,
    A1,
    A2,
    A3,
    A4,
    A5,
}

impl Altitude {
    /// Returns the allowed adjacent altitudes.
    pub fn adjacent(&self) -> Vec<Altitude> {
        match self {
            Altitude::A0 => vec![Altitude::A1],
            Altitude::A1 => vec![Altitude::A0, Altitude::A2],
            Altitude::A2 => vec![Altitude::A1, Altitude::A3],
            Altitude::A3 => vec![Altitude::A2, Altitude::A4],
            Altitude::A4 => vec![Altitude::A3, Altitude::A5],
            Altitude::A5 => vec![Altitude::A4],
        }
    }

    /// Deterministic altitude transition.
    /// Enforces adjacency, provenance logging, and diagnostic invariants.
    pub fn transition(
        &self,
        next: Altitude,
        log: &mut ProvenanceLog,
    ) -> Result<Altitude, AltitudeDrift> {
        if self.adjacent().contains(&next) {
            log.record_altitude(*self, next);
            Ok(next)
        } else {
            Err(AltitudeDrift {
                from: *self,
                to: next,
            })
        }
    }

    /// Determinism check: ensures identical inputs produce identical outputs.
    pub fn deterministic_check(&self) -> Result<(), DeterminismError> {
        let first = self.adjacent();
        let second = self.adjacent();

        if first == second {
            Ok(())
        } else {
            Err(DeterminismError {
                altitude: *self,
                message: "Non-deterministic adjacency computation",
            })
        }
    }
}
