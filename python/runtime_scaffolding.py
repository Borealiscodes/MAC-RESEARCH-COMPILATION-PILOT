# MACv1.0 — Python Runtime Scaffolding
# Altitude A4.5
# Unified runtime harness for structural AI experimentation

from diagnostic_hooks import (
    DiagnosticHooks,
    Altitude,
    Membrane,
    CitationClass,
)


class RuntimeState:
    """
    Lightweight Python representation of MAC runtime state.
    Mirrors Rust runtime physics but remains purely Pythonic.
    """

    def __init__(self):
        self.altitude = Altitude.A0
        self.membrane = Membrane.STRUCTURAL
        self.citation_class = CitationClass.STRUCTURAL
        self.provenance = []
        self.diagnostics = DiagnosticHooks()

    def record(self, event):
        """Record a provenance event."""
        self.provenance.append(event)

    def step_altitude(self, next_altitude):
        """Attempt altitude transition with diagnostic checks."""
        ok = self.diagnostics.check_altitude(self.altitude, next_altitude)
        if ok:
            self.record({"altitude": (self.altitude, next_altitude)})
            self.altitude = next_altitude
        return ok

    def step_membrane(self, next_membrane):
        """Attempt membrane transition with diagnostic checks."""
        ok = self.diagnostics.check_membrane(self.membrane, next_membrane)
        if ok:
            self.record({"membrane": (self.membrane, next_membrane)})
            self.membrane = next_membrane
        return ok

    def classify(self, citation_class):
        """Record citation classification."""
        self.citation_class = citation_class
        self.record({"citation_class": citation_class})

    def validate(self):
        """Run full diagnostic validation."""
        # Determinism checks
        def altitude_adj(value):
            return {
                Altitude.A0: [Altitude.A1],
                Altitude.A1: [Altitude.A0, Altitude.A2],
                Altitude.A2: [Altitude.A1, Altitude.A3],
                Altitude.A3: [Altitude.A2, Altitude.A4],
                Altitude.A4: [Altitude.A3, Altitude.A5],
                Altitude.A5: [Altitude.A4],
            }[value]

        def membrane_adj(value):
            return {
                Membrane.STRUCTURAL: [Membrane.FORMAL],
                Membrane.FORMAL: [Membrane.STRUCTURAL, Membrane.DIAGNOSTIC],
                Membrane.DIAGNOSTIC: [Membrane.FORMAL, Membrane.PROVENANCE],
                Membrane.PROVENANCE: [Membrane.DIAGNOSTIC],
            }[value]

        self.diagnostics.check_determinism(self.altitude, altitude_adj)
        self.diagnostics.check_determinism(self.membrane, membrane_adj)

        # Provenance integrity
        self.diagnostics.check_provenance(self.provenance)

    def get_diagnostics(self):
        """Retrieve diagnostic events."""
        return self.diagnostics.get_events()

    def get_provenance(self):
        """Retrieve provenance log."""
        return list(self.provenance)
