# MACv1.0 — Altitude Test Suite (Python)
# Altitude A4.5
# Tests for altitude adjacency, drift detection, and determinism

from diagnostic_hooks import (
    DiagnosticHooks,
    Altitude,
)


def adjacency(value):
    """Pure adjacency function mirroring Rust logic."""
    return {
        Altitude.A0: [Altitude.A1],
        Altitude.A1: [Altitude.A0, Altitude.A2],
        Altitude.A2: [Altitude.A1, Altitude.A3],
        Altitude.A3: [Altitude.A2, Altitude.A4],
        Altitude.A4: [Altitude.A3, Altitude.A5],
        Altitude.A5: [Altitude.A4],
    }[value]


def test_valid_transitions():
    hooks = DiagnosticHooks()

    assert hooks.check_altitude(Altitude.A2, Altitude.A3)
    assert hooks.check_altitude(Altitude.A3, Altitude.A4)
    assert hooks.check_altitude(Altitude.A4, Altitude.A5)


def test_invalid_transitions():
    hooks = DiagnosticHooks()

    # Skip one altitude → drift
    assert not hooks.check_altitude(Altitude.A2, Altitude.A4)
    assert not hooks.check_altitude(Altitude.A3, Altitude.A5)

    # Reverse skip → drift
    assert not hooks.check_altitude(Altitude.A4, Altitude.A2)


def test_determinism():
    hooks = DiagnosticHooks()

    assert hooks.check_determinism(Altitude.A3, adjacency)
    assert hooks.check_determinism(Altitude.A5, adjacency)


def test_diagnostic_events():
    hooks = DiagnosticHooks()

    # Trigger drift
    hooks.check_altitude(Altitude.A3, Altitude.A5)

    events = hooks.get_events()
    assert len(events) == 1
    assert events[0].kind == "AltitudeDrift"
    assert events[0].detail["from"] == Altitude.A3
    assert events[0].detail["to"] == Altitude.A5
