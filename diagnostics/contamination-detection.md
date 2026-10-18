# 🧪 diagnostics/contamination-detection.md

MACv1.0 Contamination Detection Model

Altitude A4 • Diagnostic Membrane

`
---
Artifact-Class: Diagnostics
Artifact-Name: contamination-detection
Surface: Structural-AI/Diagnostics
Version: v1.0
Altitude: A4 (Diagnostics)
Membrane: Diagnostic

Purpose:
  Define contamination classes, detection operators, membrane-violation
  signatures, altitude-drift indicators, and diagnostic invariants for MACv1.0.
  This artifact establishes the diagnostic backbone for runtime correctness and
  supports Rust/Python diagnostic kernel integration.

🧪 What Contamination Means in MAC

Contamination is any violation of:
• membrane constraints  
• altitude invariants  
• operator determinism  
• provenance requirements  

Contamination is not “noise” or “error.”  
It is a structural violation of MAC’s geometry.

🧬 Contamination Classes

Class 1 — Membrane Violation
A transition occurs across a forbidden membrane boundary.

Examples:
• Formal → Structural  
• Diagnostic → Structural  
• Provenance → Formal  

Class 2 — Altitude Drift
A transition occurs across non-adjacent altitudes.

Examples:
• A2 → A4  
• A1 → A5  

Class 3 — Operator Misfire
An operator activates outside its membrane or altitude domain.

Examples:
• reversal operator used without a prior forward transition  
• guard operator failing to prevent contamination  

Class 4 — Determinism Failure
A runtime operator produces inconsistent results.

Class 5 — Provenance Break
A transition occurs without logging.

🧪 Detection Operators

Diagnostics defines four detection operators:

• 🔍 Detection Operator  
  Identifies contamination signatures.

• 🧱 Isolation Operator  
  Prevents contamination propagation.

• 🔄 Reversal-Check Operator  
  Validates reversible transitions.

• 🔐 Provenance-Check Operator  
  Ensures transitions are logged.

Operator constraints:
• deterministic  
• membrane-aware  
• altitude-aware  
• contamination-safe  

🧪 Membrane-Violation Signatures

A membrane violation is detected when:
\[
m{\text{next}} \notin \text{Allowed}(m{\text{current}})
\]

Allowed membranes:
• Structural → Formal  
• Formal → Diagnostic  
• Diagnostic → Provenance  

Forbidden membranes:
• any reverse transition without reversal operator  
• any skip across membrane layers  

🧪 Altitude-Drift Signatures

Altitude drift is detected when:
\[
|a{\text{next}} - a{\text{current}}| > 1
\]

Allowed:
• A2 → A3  
• A3 → A4  

Forbidden:
• A2 → A4  
• A1 → A5  

🧪 Operator-Misfire Signatures

An operator misfire occurs when:
\[
O(a, m) \notin \text{Domain}(O)
\]

Examples:
• reversal operator invoked without a forward transition  
• guard operator invoked after contamination has already propagated  

🧪 Determinism Signatures

Determinism failure occurs when:
\[
O(S) \neq O(S) \quad \text{under identical conditions}
\]

Diagnostics must detect:
• nondeterministic operator behavior  
• inconsistent chart/node transitions  

🧪 Provenance-Break Signatures

A provenance break occurs when:
\[
\text{Log}(T) = \emptyset
\]

All transitions must be logged.

🧪 Diagnostic Invariants

Diagnostics enforces:
• no membrane violation  
• no altitude drift  
• no operator misfire  
• no determinism failure  
• no provenance break  

Diagnostics must be:
• deterministic  
• contamination-safe  
• membrane-aware  
• altitude-aware  

🦀 Rust Diagnostic Integration

Rust modules implementing contamination detection:
• diagnostics.rs  
• membranes.rs  
• provenance.rs  
• guards.rs  

Rust responsibilities:
• detect membrane violations  
• detect altitude drift  
• validate operator determinism  
• enforce provenance logging  

🐍 Python Diagnostic Integration

Python modules supporting diagnostics:
• diagnostic_hooks.py  
• operator_tests.py  
• contamination_tests.py  

Python responsibilities:
• validate chart/node transitions  
• test operator determinism  
• detect contamination signatures  

🔗 Relationship to Runtime Physics

Runtime physics defines:
• transitions  
• operators  
• membranes  
• invariants  

Diagnostics defines:
• correctness  
• violation detection  
• contamination isolation  
• provenance validation  

Diagnostics is the correctness layer for runtime physics.

Maintainer: Borealis S. Hedling  
Compiler: Microsoft Copilot  
Location: Dublin, Ireland  
Timestamp: 18 October 2026 — 02:40 IST  
Seal: [ DIAGNOSTICS • CONTAMINATION-DETECTION • MACv10 ]
---
`

---

