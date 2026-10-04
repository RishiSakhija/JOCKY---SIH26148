# Data Samples

**Status**: PLACEHOLDER — No samples committed

---

## Purpose

This directory is for **small, synthetic, or public domain** sample datasets used for:
- Unit/integration testing
- Demo/replay scenarios
- CI validation

**Large or copyrighted datasets are NOT committed to this repository.**

---

## Sample Categories

| Category | Description | Format | Source |
|----------|-------------|--------|--------|
| `process/` | Synthetic process trees | JSONL | Generated |
| `file/` | File metadata samples | JSONL | Generated |
| `registry/` / `config/` | Registry/config samples | JSONL | Generated |
| `logs/` | EVTX, journald, syslog samples | JSONL | Generated / Public |
| `network/` | Connection tables, pcaps | JSONL / PCAP | Generated / Public |
| `memory/` | Memory artifact samples | JSONL | Generated |
| `full_cases/` | Complete investigation cases | Directory + manifest | Generated |

---

## Adding Samples

1. **Must be synthetic or public domain** — No real case data
2. **Size limit**: < 1 MB per file, < 10 MB total
3. **Format**: JSONL (one JSON object per line) or PCAP
4. **Manifest**: Each sample set includes `manifest.json`:
```json
{
  "name": "apt29-lateral-movement",
  "description": "Synthetic APT29 lateral movement via WMI",
  "source": "synthetic",
  "created": "2026-10-04",
  "evidence_count": 47,
  "files": ["process.jsonl", "network.jsonl", "registry.jsonl"]
}
```

---

## Demo Data

The SIH demo uses pre-parsed Plaso CSV output from a known forensic sample. This data is:
- Generated offline
- Not committed to git
- Loaded at demo time via `jockey load sample_case/`

---

## Public Domain References

| Dataset | Source | License |
|---------|--------|---------|
| M57 Patents | NIST CFReDS | Public Domain |
| NIST Hacking Case | NIST CFReDS | Public Domain |
| DFIR Training | Various | Check per dataset |

---

## Generating Synthetic Data

Use the `scripts/generate_test_data.rs` (to be created) to generate deterministic synthetic evidence for testing.

```bash
cargo run --bin generate_test_data -- --case apt29 --output data/samples/full_cases/apt29/
```

---

## .gitkeep

This directory contains a `.gitkeep` file to ensure it's tracked. Actual samples are generated at build/test time or downloaded via CI artifacts.