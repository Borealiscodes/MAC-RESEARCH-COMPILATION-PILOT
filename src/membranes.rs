// MACv1.0 — Membrane Kernel
// Altitude A3.5 → A4
// Deterministic runtime membrane state machine

use crate::provenance::ProvenanceLog;
use crate::diagnostics::{MembraneViolation, DeterminismError};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Membrane {
    Structural,
    Formal,
    Diagnostic,
    Provenance,
}

impl Membrane {
    /// Returns the allowed adjacent membranes.
    pub fn adjacent(&self) -> Vec<Membrane> {
        match self {
            Membrane::Structural => vec![Membrane::Formal],
            Membrane::Formal => vec![Membrane::Structural, Membrane::Diagnostic],
            Membrane::Diagnostic => vec![Membrane::Formal, Membrane::Provenance],
            Membrane::Provenance => vec![Membrane::Diagnostic],
        }
    }

    /// Deterministic membrane transition.
    /// Enforces adjacency, provenance logging, and diagnostic invariants.
    pub fn transition(
        &self,
        next: Membrane,
        log: &mut ProvenanceLog,
    ) -> Result<Membrane, MembraneViolation> {
        if self.adjacent().contains(&next) {
            log.record_membrane(*self, next);
            Ok(next)
        } else {
            Err(MembraneViolation {
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
                membrane: *self,
                message: "Non-deterministic adjacency computation",
            })
        }
    }
}
