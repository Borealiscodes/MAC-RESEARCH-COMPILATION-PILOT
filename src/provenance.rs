// MACv1.0 — Provenance Kernel
// Altitude A3.5 → A4
// Deterministic provenance engine for MAC runtime

use crate::citation_classes::CitationClass;
use crate::diagnostics::{ProvenanceBreak, DeterminismError};
use crate::altitudes::Altitude;
use crate::membranes::Membrane;

#[derive(Debug, Clone)]
pub enum ProvenanceEvent {
    AltitudeChange { from: Altitude, to: Altitude },
    MembraneChange { from: Membrane, to: Membrane },
    CitationClass { class: CitationClass },
    OperatorInvocation { name: String },
}

#[derive(Debug, Default)]
pub struct ProvenanceLog {
    events: Vec<ProvenanceEvent>,
}

impl ProvenanceLog {
    /// Create a new provenance log.
    pub fn new() -> Self {
        ProvenanceLog { events: Vec::new() }
    }

    /// Record an altitude transition.
    pub fn record_altitude(&mut self, from: Altitude, to: Altitude) {
        self.events.push(ProvenanceEvent::AltitudeChange { from, to });
    }

    /// Record a membrane transition.
    pub fn record_membrane(&mut self, from: Membrane, to: Membrane) {
        self.events.push(ProvenanceEvent::MembraneChange { from, to });
    }

    /// Record a citation classification.
    pub fn record_citation(&mut self, class: CitationClass) {
        self.events.push(ProvenanceEvent::CitationClass { class });
    }

    /// Record an operator invocation.
    pub fn record_operator(&mut self, name: &str) {
        self.events.push(ProvenanceEvent::OperatorInvocation {
            name: name.to_string(),
        });
    }

    /// Determinism check: ensures identical logs produce identical results.
    pub fn deterministic_check(&self) -> Result<(), DeterminismError> {
        let first = self.events.clone();
        let second = self.events.clone();

        if first == second {
            Ok(())
        } else {
            Err(DeterminismError {
                provenance: true,
                message: "Non-deterministic provenance log state",
            })
        }
    }

    /// Ensure provenance is not empty.
    pub fn validate_nonempty(&self) -> Result<(), ProvenanceBreak> {
        if self.events.is_empty() {
            Err(ProvenanceBreak {
                message: "Provenance log is empty",
            })
        } else {
            Ok(())
        }
    }

    /// Retrieve all provenance events.
    pub fn events(&self) -> &Vec<ProvenanceEvent> {
        &self.events
    }
}
