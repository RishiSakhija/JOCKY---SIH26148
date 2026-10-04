# Demo Plan

**Status**: SPECIFIED

---

## Demo Concept

```
Investigator
    -> JOCKY Script
    -> AST/IR
    -> Execution Contract
    -> Policy Validation
    -> Collection
    -> Evidence Receipt
    -> Provenance
    -> Correlation
    -> Investigation Graph
    -> Verification
    -> Report
```

---

## Centerpiece Demonstration

### The Tamper Detection Sequence

1. **Generate Evidence** — Run investigation, collect 47 evidence objects
2. **Show Evidence Receipt** — Display Ed25519 signature, payload hash, host info
3. **Show Provenance** — Display hash-linked chain from collection to graph
4. **Show Correlated Graph** — Interactive Cytoscape graph with hypothesis overlay
5. **Tamper** — Modify a demonstration evidence artifact (append data)
6. **Verify** — Run verification -> **FAIL** (hash mismatch detected)
7. **Restore** — Restore original evidence (git checkout)
8. **Verify Again** — Run verification -> **PASS** (integrity restored)
9. **Generate Report** — Export verification-backed narrative with citations

---

## Detailed Demo Script (8 Steps, ~7 Minutes)

### Pre-Demo Setup (Before Judges Arrive)

```bash
# 1. Build release binary
cargo build --release --workspace

# 2. Generate synthetic sample case
cargo run --bin generate_test_data -- --case apt29 --output sample_case/

# 3. Start API server (for dashboard)
./target/release/jockey serve --addr 0.0.0.0:8080 --evidence-root ./evidence &

# 4. Verify everything works
./target/release/jockey load sample_case/
./target/release/jockey run investigate.jky
./target/release/jockey verify
```

---

### Step 1: Load Sample (30 seconds)

**Command:**
```bash
jockey load sample_case/
```

**Expected Output:**
```
Loaded sample case: apt29-lateral-movement
  Events: 47 (12 processes, 8 network, 15 registry, 12 file)
  Time range: 2026-10-04 10:00:00 - 11:55:00 UTC
  Evidence objects created: 47
```

**What to Say:** "We're loading a synthetic APT29 lateral movement case — 47 forensic events across process, network, registry, and file domains. This is pre-parsed Plaso CSV data, simulating a real investigation."

---

### Step 2: Show Graph (30 seconds)

**Command:**
```bash
jockey graph show
```

**Expected Output:** (DOT format or Cytoscape JSON)
```
digraph investigation {
  "proc:100" [label="svchost.exe (PID 100)", shape=ellipse, color=blue];
  "proc:200" [label="powershell.exe (PID 200)", shape=ellipse, color=blue];
  "file:C:\\Temp\\script.ps1" [label="script.ps1", shape=box, color=green];
  "net:200->1.1.1.1:443" [label="TCP:443", shape=triangle, color=red];
  "proc:100" -> "proc:200" [label="spawned (1.0)", color=blue];
  "proc:200" -> "file:C:\\Temp\\script.ps1" [label="wrote (0.95)", color=green];
  "proc:200" -> "net:200->1.1.1.1:443" [label="connected (0.95)", color=red];
}
```

**What to Say:** "Here's the causal graph — svchost spawned powershell, which wrote a script and connected to a C2 server. Edges are deterministic correlations with confidence scores."

**Visual:** Open `http://localhost:8080` -> Investigation -> Graph tab for interactive Cytoscape view.

---

### Step 3: Run DSL (60 seconds)

**Command:**
```bash
jockey run investigate.jky
```

**Expected Output:**
```
Investigation: apt29-lateral-movement-demo
  Target: live, WORKSTATION-01, windows
  Steps: 5 (2 parallel groups)
  Evidence collected: 47
  Receipts generated: 47
  Correlations found: 12
  Findings: 3 (1 critical, 2 high)
  Hypothesis confidence: 0.87
  Completed in 2.3s
```

**What to Say:** "Running our hypothesis-driven JOCKY script. It declares the APT29 hypothesis, collects process/registry/network evidence, correlates via PID links and timestamp joins, and produces 3 findings with 87% hypothesis confidence."

---

### Step 4: Compile (30 seconds)

**Command:**
```bash
jockey compile investigate.jky
```

**Expected Output:**
```
Compiled to 3 backends:
  hunt.vql                 (Velociraptor VQL - 47 lines)
  kape_targets.json        (KAPE targets - 12 targets)
  evidence.case.json       (CASE/UCO bundle - 2.3 KB)
```

**What to Say:** "One script compiles to three native backends simultaneously — Velociraptor for live hunting, KAPE for disk triage, and a complete CASE/UCO evidence bundle with provenance."

**Show Files:** `cat hunt.vql`, `cat kape_targets.json`, `head evidence.case.json`

---

### Step 5: Export Narrative (30 seconds)

**Command:**
```bash
jockey narrative
```

**Expected Output:** `evidence_report.md`
```markdown
# Investigation Report: apt29-lateral-movement-demo

## Hypothesis
**APT29 used WMI for lateral movement** (MITRE: T1047, T1547.001)
Confidence: 87% | Status: Supported

## Causal Chain
1. **svchost.exe (PID 100)** spawned **powershell.exe (PID 200)** [spawned, 1.0]
   - Evidence: `proc:100`, `proc:200` | Receipt: `rec:abc123`
2. **powershell.exe (PID 200)** wrote **C:\Temp\script.ps1** [wrote, 0.95]
   - Evidence: `proc:200`, `file:C:\Temp\script.ps1` | Receipt: `rec:def456`
3. **powershell.exe (PID 200)** connected to **1.1.1.1:443** [connected, 0.95]
   - Evidence: `proc:200`, `net:200->1.1.1.1:443` | Receipt: `rec:ghi789`

## Findings
- **F1: WMI Lateral Movement** (Critical) — Evidence: proc:100, proc:200
- **F2: Persistence via Run Key** (High) — Evidence: reg:HKLM\Run\Malware
- **F3: C2 Beaconing** (High) — Evidence: net:200->1.1.1.1:443

## Verification
All 47 receipts valid. Provenance chain intact. No tampering detected.
```

**What to Say:** "The narrative report with clickable evidence citations — every claim traces back to a signed receipt."

---

### Step 6: Verify Integrity (30 seconds)

**Command:**
```bash
jockey verify
```

**Expected Output:**
```
Verification Report: PASS
  Run ID: 01ARZ3NDEKTSV4RRFFQ69G5FAZ
  Evidence objects: 47/47 verified
  Receipt signatures: 47/47 valid
  Payload hashes: 47/47 match
  Provenance chains: 47/47 intact
  Correlation edges: 12/12 valid
  Timestamp skew: within tolerance
  Collector versions: all match registry
  Overall: INTEGRITY CONFIRMED
```

**What to Say:** "Full cryptographic verification — every receipt signature valid, every hash matches, every provenance link intact."

---

### Step 7: Tamper Demo (60 seconds)

**Action:**
```bash
# Tamper with evidence file
echo "MALICIOUS_INJECTION" >> evidence/01ARZ3NDEKTSV4RRFFQ69G5FB0.jsonl

# Re-verify
jockey verify
```

**Expected Output:**
```
Verification Report: FAIL
  Run ID: 01ARZ3NDEKTSV4RRFFQ69G5FAZ
  Evidence objects: 46/47 verified
  FAILED: 01ARZ3NDEKTSV4RRFFQ69G5FB0
    HASH_MISMATCH: computed blake3 differs from receipt.payload_hash
    CHAIN_BROKEN: provenance record 0 output_hash doesn't match evidence
  Overall: TAMPERING DETECTED
```

**What to Say:** "We appended one line to an evidence file. Verification immediately detects the tamper — hash mismatch at the receipt level, provenance chain broken at the root."

---

### Step 8: Restore & Verify (30 seconds)

**Action:**
```bash
# Restore original
git checkout evidence/01ARZ3NDEKTSV4RRFFQ69G5FB0.jsonl

# Re-verify
jockey verify
```

**Expected Output:**
```
Verification Report: PASS
  Run ID: 01ARZ3NDEKTSV4RRFFQ69G5FAZ
  Evidence objects: 47/47 verified
  Receipt signatures: 47/47 valid
  Payload hashes: 47/47 match
  Provenance chains: 47/47 intact
  Overall: INTEGRITY RESTORED
```

**What to Say:** "Restored the original file — integrity confirmed again. This demonstrates cryptographic tamper evidence."

---

### Closing Statement (30 seconds)

> **"One script. Three backends. Complete evidence chain. Zero post-processing."**

**Key Takeaways for Judges:**
1. **Unified Language** — One script → Velociraptor + KAPE + CASE/UCO
2. **Evidence Contracts** — Every artifact signed at collection
3. **Provenance** — Hash-linked chain from raw to report
4. **Deterministic Correlation** — Rules with evidence citations
5. **Verifiable** — Cryptographic tamper detection
6. **Court-Ready** — Narrative with clickable citations

---

## Dashboard Demo (Parallel)

While CLI runs, show browser at `http://localhost:8080`:

1. **Dashboard** — Investigation list, quick stats
2. **Graph Tab** — Interactive Cytoscape, filter by hypothesis
3. **Timeline Tab** — Chart.js zoomable timeline
4. **Evidence Tab** — Table with receipt/provenance drill-down
5. **Findings Tab** — Kanban board with MITRE tags
6. **Verify Tab** — Run verification, see PASS/FAIL
7. **Export Tab** — Download CASE/UCO, Cytoscape, Markdown

---

## Backup Plan

If live demo fails:
- Pre-recorded video of all 8 steps
- Screenshots of each step output
- Pre-generated output files in `demo_outputs/`

---

## Related Documents

- `evaluation-plan.md` — Evaluation plan
- `success-criteria.md` — Success criteria
- `judge-questions.md` — Judge questions
- `limitations.md` — Limitations
- `risk-register.md` — Risk register