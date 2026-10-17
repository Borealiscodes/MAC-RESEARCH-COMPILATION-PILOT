import Lean

def validAltitudeMembrane (altitude membrane : String) : Bool :=
  match altitude, membrane with
  | "A1", "Neutral" => true
  | "A5", "Provenance" => true
  | _, _ => false

def validateCommit (altitude membrane : String) : IO Unit :=
  if validAltitudeMembrane altitude membrane then
    IO.println "✓ Altitude/Membrane valid"
  else
    IO.println "✗ Invalid altitude/membrane combination"
