# Frontend Dashboard

**Status**: SPECIFIED — Not Implemented

---

## Overview

Static HTML/CSS/JS dashboard (no build step) served by Axum. Uses Chart.js for timelines and Cytoscape.js for graph visualization.

**Design Principles:**
- Zero build dependencies (no npm, webpack, vite)
- Works offline (CDN fallback for libraries)
- Responsive, accessible
- Evidence-first: every UI element ties to evidence ID

---

## Technology Stack

| Layer | Technology | Version | Source |
|-------|------------|---------|--------|
| HTML/CSS/JS | Vanilla | ES2022 | Inline / local files |
| Charts | Chart.js | 4.4.x | CDN + local fallback |
| Graph | Cytoscape.js | 3.29.x | CDN + local fallback |
| Icons | Lucide | 0.400+ | CDN + local fallback |
| Markdown | marked.js | 14.x | CDN + local fallback |

---

## Page Specifications

### 1. index.html — Dashboard Overview
- Recent investigations (table + status badges)
- Quick stats: total runs, evidence collected, findings open
- New investigation button → navigate to `investigation.html?new=1`

### 2. investigation.html — Investigation Detail
**Tabs:**
- **Graph** → `graph.html` embedded via iframe or dynamic load
- **Timeline** → `timeline.html` embedded
- **Evidence** → `evidence.html` embedded
- **Findings** → `findings.html` embedded
- **Export** → Export buttons (CASE/UCO, Cytoscape, Markdown, Timesketch)
- **Verify** → `verify.html` embedded

**URL Params**: `?id=<run_id>` or `?new=1`

### 3. graph.html — Cytoscape Graph View
**Features:**
- Directed graph layout (cose-bilkent or dagre)
- Node styling by evidence type (color/shape)
- Edge styling by relation type (color/width = confidence)
- Filters: evidence type, time range, hypothesis overlay
- Click node → side panel with evidence + receipt + provenance
- Click edge → correlation details
- Export: Cytoscape JSON, PNG, SVG

**Cytoscape Stylesheet**:
```javascript
const style = [
  { selector: 'node', style: { 'label': 'data(summary)', 'width': 40, 'height': 40 } },
  { selector: 'node[type="artifact/process"]', style: { 'background-color': '#3b82f6', 'shape': 'ellipse' } },
  { selector: 'node[type="artifact/file"]', style: { 'background-color': '#10b981', 'shape': 'rectangle' } },
  { selector: 'node[type="artifact/reg"]', style: { 'background-color': '#f59e0b', 'shape': 'hexagon' } },
  { selector: 'node[type="artifact/evt"]', style: { 'background-color': '#8b5cf6', 'shape': 'diamond' } },
  { selector: 'node[type="artifact/connection"]', style: { 'background-color': '#ef4444', 'shape': 'triangle' } },
  { selector: 'edge', style: { 'width': 'mapData(confidence, 0, 1, 1, 5)', 'line-color': '#64748b', 'target-arrow-shape': 'triangle', 'target-arrow-color': '#64748b', 'label': 'data(relation)', 'font-size': 8 } },
  { selector: 'edge[relation="spawned"]', style: { 'line-color': '#3b82f6', 'target-arrow-color': '#3b82f6' } },
  { selector: 'edge[relation="wrote"]', style: { 'line-color': '#10b981', 'target-arrow-color': '#10b981' } },
  { selector: 'edge[relation="connected"]', style: { 'line-color': '#ef4444', 'target-arrow-color': '#ef4444' } },
  { selector: '.hypothesis-supported', style: { 'border-width': 3, 'border-color': '#f59e0b' } },
];
```

### 4. timeline.html — Chart.js Timeline
**Features:**
- Horizontal timeline (time on X, evidence types on Y lanes)
- Zoom/pan (mouse wheel + drag)
- Filter by evidence type, time range, hypothesis
- Click event → side panel with evidence details
- Annotation layer (hypothesis markers, finding flags)
- Export: Timesketch CSV, JSON, PNG

**Chart.js Config**:
```javascript
const config = {
  type: 'scatter',
  data: { datasets: [] },  // One dataset per evidence type
  options: {
    scales: {
      x: { type: 'time', time: { unit: 'minute' }, title: { display: true, text: 'Time (UTC)' } },
      y: { type: 'category', labels: ['Process', 'File', 'Registry', 'Log', 'Network', 'Finding'], reverse: true }
    },
    plugins: { zoom: { pan: { enabled: true, mode: 'xy' }, zoom: { wheel: { enabled: true }, pinch: { enabled: true }, mode: 'xy' } } }
  }
};
```

### 5. evidence.html — Evidence Browser
**Features:**
- Searchable/sortable table (evidence ID, type, collector, size, hash, time)
- Row click → detail modal with:
  - Evidence metadata (JSON viewer)
  - Receipt (signature, public key, payload hash, host info)
  - Provenance chain (expandable list with hash links)
  - Raw evidence download link
- Bulk actions: verify selected, export selected

### 6. findings.html — Findings Board
**Features:**
- Kanban columns: Open, Triaged, False Positive, Resolved
- Cards show: severity badge, title, evidence count, MITRE tags
- Drag-drop between columns (updates status via API)
- Click card → detail with evidence citations (clickable → evidence.html)
- Add finding manually (for analyst annotations)

### 7. verify.html — Verification Report
**Features:**
- Overall status: PASS / FAIL
- Checks:
  - Receipt signatures (all valid?)
  - Provenance chain integrity (hash links intact?)
  - Evidence file hashes match receipts?
  - Collector versions match registry?
- Per-evidence detail: expandable rows with check results
- Tamper demo: button to "simulate tamper" (modifies file) → re-verify → FAIL

### 8. report.html — Generated Report View
**Features:**
- Rendered Markdown (from `export report` output)
- Evidence citations as clickable links → `evidence.html?eid=...`
- Graph embed (static PNG or interactive Cytoscape)
- Print to PDF button
- Download Markdown source

---

## API Integration

**Base URL**: `/api/v1`

| Page | Endpoints Used |
|------|----------------|
| index.html | `GET /investigations` |
| investigation.html | `GET /investigations/{id}`, `GET /investigations/{id}/graph`, `GET /investigations/{id}/timeline`, `GET /investigations/{id}/evidence`, `GET /investigations/{id}/findings`, `POST /investigations/{id}/verify`, `GET /investigations/{id}/export/{format}` |
| graph.html | `GET /investigations/{id}/graph` |
| timeline.html | `GET /investigations/{id}/timeline` |
| evidence.html | `GET /investigations/{id}/evidence`, `GET /investigations/{id}/evidence/{eid}` |
| findings.html | `GET /investigations/{id}/findings`, `PATCH /findings/{id}` |
| verify.html | `POST /investigations/{id}/verify`, `GET /investigations/{id}/verify` |
| report.html | `GET /investigations/{id}/export/markdown` |

**WebSocket**: `/api/v1/investigations/{id}/stream` for live updates during execution.

---

## Static Asset Structure

```
frontend/
├── index.html
├── investigation.html
├── graph.html
├── timeline.html
├── evidence.html
├── findings.html
├── verify.html
├── report.html
├── css/
│   ├── main.css
│   ├── graph.css
│   ├── timeline.css
│   └── evidence.css
├── js/
│   ├── main.js           # Shared utilities, API client
│   ├── graph.js          # Cytoscape initialization
│   ├── timeline.js       # Chart.js initialization
│   ├── evidence.js       # Evidence table + detail modal
│   ├── findings.js       # Kanban board
│   ├── verify.js         # Verification report
│   └── report.js         # Markdown rendering
├── lib/                  # Local copies of CDN libraries (for offline)
│   ├── chart.min.js
│   ├── cytoscape.min.js
│   ├── marked.min.js
│   └── lucide.min.js
└── assets/
    └── favicon.ico
```

---

## Offline Support

- All CDN libraries have local fallbacks in `frontend/lib/`
- Service Worker (optional) for full offline capability
- Evidence files served from local evidence root

---

## Accessibility

- Semantic HTML5
- ARIA labels on interactive elements
- Keyboard navigation for all controls
- Color contrast ratios (WCAG AA)
- Focus indicators
- Screen reader compatible

---

## Responsive Breakpoints

| Breakpoint | Layout |
|------------|--------|
| < 640px | Stacked tabs, single-column tables |
| 640-1024px | Side-by-side graph + panel |
| > 1024px | Full multi-panel layout |

---

## Implementation Order

1. `main.css` + `main.js` (shared)
2. `index.html` + `investigation.html` (shell)
3. `graph.html` + `graph.js` (Cytoscape)
4. `timeline.html` + `timeline.js` (Chart.js)
5. `evidence.html` + `evidence.js`
6. `findings.html` + `findings.js`
7. `verify.html` + `verify.js`
8. `report.html` + `report.js`
9. Offline libs + Service Worker
10. Accessibility audit

---

## References

- Chart.js: https://www.chartjs.org/docs/latest/
- Cytoscape.js: https://js.cytoscape.org/
- marked.js: https://marked.js.org/
- Lucide Icons: https://lucide.dev/
- WCAG: https://www.w3.org/WAI/WCAG21/quickref/