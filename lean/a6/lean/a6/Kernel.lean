/-
A6 Unified Kernel — Lean Scaffolding (v1.0)
Author: Borealis S. Hedling
Altitude: A6 (Dual Algebra)
Purpose: Define geometric, spectral, and activation kernels as
         regular-value submanifolds and construct the unified A6 kernel.
-/

import A6.Geom
import A6.Spec
import A6.Act

namespace A6

/-- Kernel predicates for each operator algebra -/
def K_geom (p : GeomState) : Prop :=
  curvatureContinuous p p ∧ modalAdjacent p p ∧ holonomyBound p

def K_spec (p : SpecState) : Prop :=
  spectralBound p

def K_act (p : ActState) : Prop :=
  semanticLift p ∧ provenanceLift p

/-- Unified A6 kernel: intersection of geometric, spectral, activation kernels -/
structure KernelState :=
  (geom : GeomState)
  (spec : SpecState)
  (act : ActState)
  (geom_ok : K_geom geom)
  (spec_ok : K_spec spec)
  (act_ok : K_act act)

end A6
