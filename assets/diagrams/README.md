# Architecture Diagrams

**Status**: COMPLETE — 14 Mermaid diagrams created

---

## Overview

All architecture diagrams are created as **Mermaid.js** diagrams embedded in Markdown files. This ensures:
- Version control friendly (text-based)
- Renderable in GitHub, GitLab, VS Code, Obsidian
- No binary assets to maintain
- Single source of truth

---

## Diagram Index

| # | File | Description | Type |
|---|------|-------------|------|
| 1 | `system-architecture.mmd` | Full system architecture | Flowchart |
| 2 | `language-pipeline.mmd` | JOCKY language pipeline (DSL → AST → IR) | Flowchart |
| 3 | `execution-contract.mmd` | Execution Contract lifecycle | Sequence |
| 4 | `evidence-receipt.mmd` | Evidence Receipt lifecycle | Sequence |
| 5 | `collector-architecture.mmd` | Windows/Linux collector architecture | Flowchart |
| 6 | `evidence-pipeline.mmd` | Evidence pipeline (collection → receipt → provenance) | Flowchart |
| 7 | `provenance-chain.mmd` | Provenance chain (hash-linked) | Flowchart |
| 8 | `correlation-flow.mmd` | Deterministic correlation flow | Flowchart |
| 9 | `investigation-graph.mmd` | Investigation graph structure | Class/ER |
| 10 | `e2e-workflow.mmd` | End-to-end investigation workflow | Sequence |
| 11 | `deployment-topology.mmd` | Deployment topology | Flowchart |
| 12 | `data-model.mmd` | Data model / entity relationships | ER |
| 13 | `mvp-vs-future.mmd` | MVP vs Future architecture | Flowchart |
| 14 | `demo-sequence.mmd` | SIH Demo sequence | Sequence |

---

## Rendering

### GitHub
Automatically rendered in `.md` files when viewed on GitHub.

### VS Code
Install "Markdown Preview Mermaid Support" extension.

### CLI
```bash
# Install mermaid-cli
npm install -g @mermaid-js/mermaid-cli

# Render to PNG
mmdc -i system-architecture.mmd -o system-architecture.png

# Render to SVG
mmdc -i system-architecture.mmd -o system-architecture.svg
```

### Online
https://mermaid.live/ — Paste Mermaid source for live preview.

---

## Diagram Conventions

| Element | Style |
|---------|-------|
| Components | `rect` with rounded corners |
| Data stores | `cylinder` |
| External systems | `hexagon` |
| Processes | `rect` |
| Decisions | `diamond` |
| Arrows | `-->` (solid), `-.->` (dashed) |
| Colors | Consistent palette per layer |

**Color Palette:**
- Language/Compiler: `#3b82f6` (blue)
- Runtime/Execution: `#8b5cf6` (purple)
- Collectors: `#10b981` (green)
- Evidence/Provenance: `#f59e0b` (amber)
- Correlation/Graph: `#ef4444` (red)
- API/Frontend: `#06b6d4` (cyan)
- External: `#64748b` (slate)

---

## Maintenance

- Update diagrams when architecture changes
- Keep in sync with `JOCKY_SPEC_SHEET.md` and `docs/architecture/`
- Each diagram has a corresponding explanation in `docs/architecture/`
- Run `mmdc` to verify syntax before committing

---

## Source Files

All `.mmd` files are in this directory. They are also embedded in the corresponding `.md` files in `docs/architecture/` for documentation context.