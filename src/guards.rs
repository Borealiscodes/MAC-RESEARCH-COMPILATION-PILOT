// MACv1.0 — Guard Kernel
// Altitude A3.5 → A4
// Structural guards enforcing correctness across runtime physics

use crate::altitudes::Altitude;
use crate::membranes::Membrane;
use crate::provenance::ProvenanceLog;
use crate::diagnostics::{
    AltitudeDrift,
    MembraneViolation,
    DeterminismError,
    ProvenanceBreak,
};

#[derive(Debug)]
pub enum GuardError {
    Altitude(AltitudeDrift),
    Membrane(MembraneViolation),
    Determinism(DeterminismError),
    Provenance(ProvenanceBreak),
}

pub struct Guards;

impl Guards {
    /// Guard altitude transitions.
    pub fn guard_altitude(
        current: Altitude,
        next: Altitude,
        log: &mut ProvenanceLog,
    ) -> Result<Altitude, GuardError> {
        current
            .transition(next, log)
            .map_err(GuardError::Altitude)
    }

    /// Guard membrane transitions.
    pub fn guard_membrane(
        current: Membrane,
        next: Membrane,
        log: &mut ProvenanceLog,
    ) -> Result<Membrane, GuardError> {
        current
            .transition(next, log)
            .map_err(GuardError::Membrane)
    }

    /// Guard determinism for altitude and membrane machines.
    pub fn guard_determinism(
        altitude: Altitude,
        membrane: Membrane,
    ) -> Result<(), GuardError> {
        altitude
            .deterministic_check()
            .map_err(GuardError::Determinism)?;

        membrane
            .deterministic_check()
            .map_err(GuardError::Determinism)?;

        Ok(())
    }

    /// Guard provenance integrity.
    pub fn guard_provenance(log: &ProvenanceLog) -> Result<(), GuardError> {
        log.validate_nonempty().map_err(GuardError::Provenance)
    }
}
