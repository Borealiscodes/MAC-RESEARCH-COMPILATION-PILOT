# 🌟 MAC Provenance Commit System — Accessible Explainer (Visual‑Grammar‑Aligned)

Why the .txt commit exists, how it works, why it’s worth it, and how to automate it.

---

🧩 1. The Core Idea (Accessible Version)

A commit in MAC has two representations:

1. GitHub Commit (Operational Layer)
- The real commit  
- Lives in GitHub’s history  
- Can be squashed, rewritten, or lost  
- Not altitude‑tagged  
- Not membrane‑tagged  
- Not ontology‑aware  

2. Provenance .txt Commit Log (Lineage Layer)
- A file inside the repo  
- Immutable  
- Altitude‑tagged  
- Membrane‑tagged  
- Ontology‑aware  
- Uses visual-grammar.json as the canonical reference  
- Survives platform changes, history rewrites, forks, exports  

GitHub commit = the event.  
Provenance .txt = the scientific record of the event.

This dual system is the Structural AI equivalent of:

- “lab notebook + Git history”  
- “ledger + transaction”  
- “metadata + data”  

It’s not duplication — it’s lineage preservation.

---

🌲 2. Diagram — “How the Two Commit Layers Work Together”

markdown
<pre>
┌──────────────────────────────────────────────┐
│                GitHub Commit                 │
│   (Operational Layer — version control)       │
└───────────────────────────────┬──────────────┘
                                │
                                │ mirrors into
                                ▼
┌──────────────────────────────────────────────┐
│      provenance/commit_log/*.txt             │
│   (Lineage Layer — ontology + provenance)     │
└──────────────────────────────────────────────┘
</pre>


The .txt file is committed to GitHub,  
but it is not the commit itself.

It is the record of the commit.

---

🧬 3. Comparison Table — GitHub Commit vs Provenance .txt

| Feature | GitHub Commit | provenance/*.txt |
|--------|----------------|------------------|
| Purpose | Version control | Lineage preservation |
| Altitude‑tagged? | No | Yes |
| Membrane‑tagged? | No | Yes |
| Ontology‑aware? | No | Yes |
| Immutable? | No | Yes |
| Platform‑dependent? | Yes | No |
| Human‑readable? | Medium | High |
| Survives history rewrite? | No | Yes |
| Survives export/fork? | Yes | Yes |
| Is it the commit? | Yes | No |
| Records the commit? | No | Yes |

---

🧠 4. Why the Extra Effort Is Worth It

1. GitHub commits are not reliable lineage artifacts
They can be:

- squashed  
- rebased  
- rewritten  
- lost  
- corrupted  
- platform‑dependent  

A scientific ecosystem cannot rely on GitHub alone.

2. Structural AI requires altitude + membrane tagging
GitHub commits cannot store:

- altitude  
- membrane  
- contamination rules  
- provenance seals  
- ontology class  

The .txt file can.

3. visual-grammar.json makes the .txt commit canonical
The .txt commit:

- references the visual grammar  
- uses the correct gitmoji  
- declares altitude  
- declares membrane  
- declares ontology class  
- declares provenance seal  

This makes the commit machine‑interpretable.

4. It enables future automation
Because the .txt commit is structured, MAC can later support:

- commit validators  
- provenance checkers  
- contamination guards  
- ontology‑aware CI pipelines  
- lineage diff tools  

GitHub alone cannot support these.

5. It makes MAC a real research compiler
This is the difference between:

- a normal repo  
- a scientific ecosystem  

MAC is the latter.

---

🛠️ 5. How We Might Automate It (Accessible + Practical)

Option A — Pre‑commit Hook (Local Automation)
A script reads:

`
visual-grammar.json
`

and generates:

`
provenance/commit_log/<timestamp>.txt
`

containing:

- commit hash  
- gitmoji  
- altitude  
- membrane  
- ontology class  
- message  
- seal  

Option B — GitHub Action (CI Automation)
On every push:

1. Parse commit message  
2. Validate against visual-grammar.json  
3. Generate .txt provenance record  
4. Commit it back to the repo  

Option C — MAC CLI Tool (Future)
A small Rust binary:

`
mac-commit "🌲 Initialize MAC scaffolding"
`

It:

- validates the commit  
- generates the .txt  
- commits both  
- updates provenance seals  

Option D — Lean‑based validator (Advanced)
Lean checks:

- altitude correctness  
- membrane correctness  
- contamination rules  
- provenance invariants  

Then approves or rejects the commit.

---

📦 6. Accessible Instructions — “How to Do a MAC Commit”

Step 1 — Make the GitHub commit
`
git commit -m "🌲 Initialize MAC scaffolding"
`

Step 2 — Create the .txt record
Inside:

`
provenance/commit_log/
`

Create:

`
macinitialcommit.txt
`

Step 3 — Fill it using visual-grammar.json
Include:

- commit hash  
- gitmoji  
- altitude  
- membrane  
- ontology class  
- message  
- seal  

Step 4 — Commit the .txt file
`
git add provenance/commitlog/macinitial_commit.txt
git commit -m "📜 Add provenance record for MAC initial commit"
`

Step 5 — Push
`
git push
`

Done.

---

🌲 Provenance Footer

`
---
Provenance:

This document, accessibleprovenanceexplainerv10.md, provides a
public-facing explanation of the MAC dual-layer provenance commit
system. It describes how GitHub commits (operational layer) are
mirrored into provenance .txt records (lineage layer), how visual-
grammar.json governs altitude and membrane tagging, and why this
dual system preserves scientific lineage, prevents contamination,
and enables future automation.

The artifact is altitude-tagged (A1 Seam Layer), membrane-neutral,
and harmonized with the MAC visual grammar. It is part of the
documentation plane of MAC-RESEARCH-COMPILATION-PILOT.

Authored by Borealis S. Hedling, Dublin, Ireland — 17 October 2026.
Compiler: MAC-RESEARCH-COMPILATION-PILOT
Seal: [ MACv10 . STRUCTURAL-AI . DOCUMENTATION ]
---
`

---

