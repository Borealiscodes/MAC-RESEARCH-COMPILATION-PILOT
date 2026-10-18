// MACv1.0 — Diagnostics Kernel
// Altitude A3.5 → A4
// Executable diagnostics engine enforcing correctness across MAC runtime

use crate::altitudes::Altitude;
use crate::membranes::Membrane;
use crate::citation_classes::CitationClass;
use crate::provenance::{ProvenanceLog, ProvenanceEvent};

#[derive(Debug)]
pub struct AltitudeDrift {
    pub from: Altitude,
    pub to: Altitude,
}

#[derive(Debug)]
pub struct MembraneViolation {
    pub from: Membrane,
    pub to: Membrane,
}

#[derive(Debug)]
pub struct DeterminismError {
    pub message: &'static str,
    pub altitude: Option<Altitude>,
    pub membrane: Option<Membrane>,
    pub citation_class: Option<CitationClass>,
    pub provenance: bool,
}

#[derive(Debug)]
pub struct ProvenanceBreak {
    pub message: &'static str,
}

#[derive(Debug)]
pub enum DiagnosticEvent {
    AltitudeDrift(AltitudeDrift),
    MembraneViolation(MembraneViolation),
    Determinism(DeterminismError),
    ProvenanceBreak(ProvenanceBreak),
}

#[derive(Debug, Default)]
pub struct Diagnostics {
    events: Vec<DiagnosticEvent>,
}

impl Diagnostics {
    pub fn new() -> Self {
        Diagnostics { events: Vec::new() }
    }

    /// Log a diagnostic event.
    pub fn record(&mut self, event: DiagnosticEvent) {
        self.events.push(event);
    }

    /// Check for altitude drift.
    pub fn check_altitude(
        &mut self,
        from: Altitude,
        to: Altitude,
    ) -> Result<(), AltitudeDrift> {
        if (from.adjacent()).contains(&to) {
            Ok(())
        } else {
            let drift = AltitudeDrift { from, to };
            self.record(DiagnosticEvent::AltitudeDrift(drift.clone()));
            Err(drift)
        }
    }

    /// Check for membrane violation.
    pub fn check_membrane(
        &mut self,
        from: Membrane,
        to: Membrane,
    ) -> Result<(), MembraneViolation> {
        if (from.adjacent()).contains(&to) {
            Ok(())
        } else {
            let violation = MembraneViolation { from, to };
            self.record(DiagnosticEvent::MembraneViolation(violation.clone()));
            Err(violation)
        }
    }

    /// Check determinism for altitude, membrane, and citation classes.
    pub fn check_determinism(
        &mut self,
        altitude: Altitude,
        membrane: Membrane,
        citation: CitationClass,
    ) -> Result<(), DeterminismError> {
        // Altitude determinism
        altitude.deterministic_check().map_err(|e| {
            let err = DeterminismError {
                message: e.message,
                altitude: Some(altitude),
                membrane: None,
                citation_class: None,
                provenance: false,
            };
            self.record(DiagnosticEvent::Determinism(err.clone()));
            err
        })?;

        // Membrane determinism
        membrane.deterministic_check().map_err(|e| {
            let err = DeterminismError {
                message: e.message,
                altitude: None,
                membrane: Some(membrane),
                citation_class: None,
                provenance: false,
            };
            self.record(DiagnosticEvent::Determinism(err.clone()));
            err
        })?;

        // Citation determinism
        citation.deterministic_check().map_err(|e| {
            let err = DeterminismError {
                message: e.message,
                altitude: None,
                membrane: None,
                citation_class: Some(citation),
                provenance: false,
            };
            self.record(DiagnosticEvent::Determinism(err.clone()));
            err
        })?;

        Ok(())
    }

    /// Check provenance integrity.
    pub fn check_provenance(
        &mut self,
        log: &ProvenanceLog,
    ) -> Result<(), ProvenanceBreak> {
        log.validate_nonempty().map_err(|e| {
            let break_event = ProvenanceBreak { message: e.message };
            self.record(DiagnosticEvent::ProvenanceBreak(break_event.clone()));
            break_event
        })
    }

    /// Retrieve all diagnostic events.
    pub fn events(&self) -> &Vec<DiagnosticEvent> {
        &self.events
    }
}
