# MACv1.0 — Operator Test Suite (Python)
# Altitude A4.5
# Tests for operator determinism, adjacency, and diagnostic correctness

from diagnostic_hooks import (
    DiagnosticHooks,
    Altitude,
    Membrane,
    CitationClass,
)


def test_altitude_determinism():
    hooks = DiagnosticHooks()

    # Deterministic adjacency function
    def adjacency_fn(value):
        adjacency = {
            Altitude.A0: [Altitude.A1],
            Altitude.A1: [Altitude.A0, Altitude.A2],
            Altitude.A2: [Altitude.A1, Altitude.A3],
            Altitude.A3: [Altitude.A2, Altitude.A4],
            Altitude.A4: [Altitude.A3, Altitude.A5],
            Altitude.A5: [Altitude.A4],
        }
        return adjacency[value]

    assert hooks.check_determinism(Altitude.A3, adjacency_fn)
    assert hooks.check_determinism(Altitude.A4, adjacency_fn)


def test_membrane_determinism():
    hooks = DiagnosticHooks()

    def adjacency_fn(value):
        adjacency = {
            Membrane.STRUCTURAL: [Membrane.FORMAL],
            Membrane.FORMAL: [Membrane.STRUCTURAL, Membrane.DIAGNOSTIC],
            Membrane.DIAGNOSTIC: [Membrane.FORMAL, Membrane.PROVENANCE],
            Membrane.PROVENANCE: [Membrane.DIAGNOSTIC],
        }
        return adjacency[value]

    assert hooks.check_determinism(Membrane.FORMAL, adjacency_fn)
    assert hooks.check_determinism(Membrane.DIAGNOSTIC, adjacency_fn)


def test_altitude_adjacency():
    hooks = DiagnosticHooks()

    assert hooks.check_altitude(Altitude.A3, Altitude.A4)
    assert hooks.check_altitude(Altitude.A4, Altitude.A5)

    # Drift
    assert not hooks.check_altitude(Altitude.A3, Altitude.A5)


def test_membrane_adjacency():
    hooks = DiagnosticHooks()

    assert hooks.check_membrane(Membrane.FORMAL, Membrane.DIAGNOSTIC)
    assert hooks.check_membrane(Membrane.DIAGNOSTIC, Membrane.PROVENANCE)

    # Violation
    assert not hooks.check_membrane(Membrane.STRUCTURAL, Membrane.PROVENANCE)


def test_provenance_integrity():
    hooks = DiagnosticHooks()

    # Empty provenance log → break
    assert not hooks.check_provenance([])

    # Non-empty provenance log → OK
    assert hooks.check_provenance(["event"])
