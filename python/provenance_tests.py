# MACv1.0 — Provenance Test Suite (Python)
# Altitude A4.5
# Tests for provenance integrity, event correctness, and diagnostic behavior

from diagnostic_hooks import DiagnosticHooks


def test_empty_provenance_break():
    hooks = DiagnosticHooks()

    # Empty provenance log → break
    assert not hooks.check_provenance([])

    events = hooks.get_events()
    assert len(events) == 1
    assert events[0].kind == "ProvenanceBreak"
    assert events[0].detail["message"] == "Empty provenance log"


def test_nonempty_provenance_ok():
    hooks = DiagnosticHooks()

    # Non-empty provenance log → OK
    assert hooks.check_provenance(["altitude_change"])


def test_multiple_events():
    hooks = DiagnosticHooks()

    # Trigger multiple breaks
    hooks.check_provenance([])
    hooks.check_provenance([])

    events = hooks.get_events()
    assert len(events) == 2
    assert all(evt.kind == "ProvenanceBreak" for evt in events)
