# MACv1.0 — Diagnostic Hooks (Python)
# Altitude A4.5
# Python scaffolding for Rust diagnostic kernels

class Altitude:
    """Lightweight mirror of Rust Altitude enum."""
    A0 = "A0"
    A1 = "A1"
    A2 = "A2"
    A3 = "A3"
    A4 = "A4"
    A5 = "A5"


class Membrane:
    """Lightweight mirror of Rust Membrane enum."""
    STRUCTURAL = "Structural"
    FORMAL = "Formal"
    DIAGNOSTIC = "Diagnostic"
    PROVENANCE = "Provenance"


class CitationClass:
    """Lightweight mirror of Rust CitationClass enum."""
    PRIMARY = "Primary"
    SECONDARY = "Secondary"
    TERTIARY = "Tertiary"
    STRUCTURAL = "Structural"


class DiagnosticEvent:
    """Python-side representation of diagnostic events."""
    def __init__(self, kind, detail):
        self.kind = kind
        self.detail = detail

    def __repr__(self):
        return f"<DiagnosticEvent kind={self.kind} detail={self.detail}>"


class DiagnosticHooks:
    """
    Python scaffolding for MAC diagnostics.
    This does NOT execute Rust code — it mirrors the logic for testing and
    scaffolding purposes.
    """

    def __init__(self):
        self.events = []

    def record(self, kind, detail):
        self.events.append(DiagnosticEvent(kind, detail))

    # --- Altitude checks -----------------------------------------------------

    def check_altitude(self, current, next_):
        adjacency = {
            Altitude.A0: [Altitude.A1],
            Altitude.A1: [Altitude.A0, Altitude.A2],
            Altitude.A2: [Altitude.A1, Altitude.A3],
            Altitude.A3: [Altitude.A2, Altitude.A4],
            Altitude.A4: [Altitude.A3, Altitude.A5],
            Altitude.A5: [Altitude.A4],
        }

        if next_ not in adjacency[current]:
            self.record("AltitudeDrift", {"from": current, "to": next_})
            return False

        return True

    # --- Membrane checks -----------------------------------------------------

    def check_membrane(self, current, next_):
        adjacency = {
            Membrane.STRUCTURAL: [Membrane.FORMAL],
            Membrane.FORMAL: [Membrane.STRUCTURAL, Membrane.DIAGNOSTIC],
            Membrane.DIAGNOSTIC: [Membrane.FORMAL, Membrane.PROVENANCE],
            Membrane.PROVENANCE: [Membrane.DIAGNOSTIC],
        }

        if next_ not in adjacency[current]:
            self.record("MembraneViolation", {"from": current, "to": next_})
            return False

        return True

    # --- Determinism checks --------------------------------------------------

    def check_determinism(self, value, adjacency_fn):
        first = adjacency_fn(value)
        second = adjacency_fn(value)

        if first != second:
            self.record("DeterminismError", {"value": value})
            return False

        return True

    # --- Provenance checks ---------------------------------------------------

    def check_provenance(self, log):
        if not log:
            self.record("ProvenanceBreak", {"message": "Empty provenance log"})
            return False

        return True

    # --- Retrieve events -----------------------------------------------------

    def get_events(self):
        return list(self.events)
