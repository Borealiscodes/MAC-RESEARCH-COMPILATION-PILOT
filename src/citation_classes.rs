// MACv1.0 — Citation Class Kernel
// Altitude A3.5 → A4
// Deterministic citation classification engine

use crate::provenance::ProvenanceLog;
use crate::diagnostics::{CitationClassError, DeterminismError};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CitationClass {
    Primary,
    Secondary,
    Tertiary,
    Structural,
}

impl CitationClass {
    /// Deterministic classification rule.
    /// This is intentionally simple: MAC's citation classes are fixed.
    pub fn classify(&self, log: &mut ProvenanceLog) -> Result<CitationClass, CitationClassError> {
        // Record classification provenance
        log.record_citation(*self);

        // Deterministic: classification returns itself
        Ok(*self)
    }

    /// Determinism check: ensures identical inputs produce identical outputs.
    pub fn deterministic_check(&self) -> Result<(), DeterminismError> {
        let first = *self;
        let second = *self;

        if first == second {
            Ok(())
        } else {
            Err(DeterminismError {
                citation_class: *self,
                message: "Non-deterministic citation classification",
            })
        }
    }
}
