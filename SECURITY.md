# Security Policy

**Status**: ACTIVE

---

## Supported Versions

| Version | Supported |
|---------|-----------|
| 0.x (pre-1.0) | ✅ Active development |

JOCKY is currently in pre-1.0 development. Security fixes will be applied to the latest `main` branch.

---

## Reporting a Vulnerability

**DO NOT** open a public issue for security vulnerabilities.

Instead, report security vulnerabilities privately via:

**Email:** security@jockey-forensics.example.com (placeholder — update with actual contact)

Or use GitHub's private vulnerability reporting:
1. Go to the repository's **Security** tab
2. Click **Report a vulnerability**
3. Fill in the details privately

### What to Include
- Description of the vulnerability
- Steps to reproduce (if applicable)
- Potential impact
- Suggested fix (if known)
- Your contact information

### Response Timeline
- **Acknowledgment**: Within 48 hours
- **Initial Assessment**: Within 1 week
- **Fix Timeline**: Depends on severity (critical: ASAP, high: 2 weeks, medium: 1 month)

---

## Security Model

### Threat Model
JOCKY is designed for **defensive forensic analysis** in trusted environments. The security model assumes:

| Assumption | Implication |
|------------|-------------|
| Analyst workstation is trusted | No need for analyst-to-server encryption in MVP |
| Evidence root is isolated | Path traversal prevented by runtime |
| Collector binaries are authentic | Keys embedded at build time |
| Network is controlled (MVP) | No mTLS in MVP; future agents use gRPC/mTLS |

### Out of Scope (MVP)
- Multi-tenancy / RBAC
- Remote agent compromise resistance
- Supply chain attacks on dependencies
- Side-channel attacks
- Physical access to evidence root

---

## Implemented Security Controls

### 1. Execution Contracts (Capability-Based)
- Script declares required capabilities via `collect` statements
- Type checker infers capabilities
- Policy engine validates against collector manifests
- **Enforcement**: Collector allowlist, parameter schemas, timeout bounds, evidence root isolation

### 2. Evidence Integrity
- **blake3** hash of all evidence bytes at collection
- **Ed25519** signature by collector over `hash || evidence_id || timestamp`
- Collector public key in manifest (pinned at build time)
- **Verification**: Re-compute hash, verify signature, check timestamp skew

### 3. Provenance Chains
- Hash-linked append-only log per evidence object
- Each transformation (collect, transform, correlate, export) creates signed record
- **Verification**: Chain continuity, actor signatures, hash continuity

### 4. Tamper Detection
- Any evidence modification → hash mismatch at receipt verification
- Any provenance modification → chain break at signature verification
- **Result**: Cryptographic tamper evidence

### 5. Collector Security
- Userspace only (no kernel drivers in MVP)
- Capability manifests declare allowed actions
- Parameter validation via JSON Schema
- Timeout enforcement (1s-300s per step)
- Evidence root chroot-style isolation

### 6. Evidence Root Isolation
- Per-run directory: `evidence_root/run_id/`
- All paths relative to evidence root
- Runtime validates no path traversal (`..`, absolute paths)
- Collector receives `evidence_root` in `ExecutionContext`

### 7. Crypto Primitives
| Primitive | Algorithm | Rationale |
|-----------|-----------|-----------|
| Hashing | blake3 | Fast, parallel, 256-bit |
| Hashing (fallback) | SHA-256 | FIPS compatibility |
| Signatures | Ed25519 | Small keys, fast verify, constant-time |
| IDs | ULID | Monotonic, sortable, 128-bit |
| Time | UTC (chrono) | Timezone-safe, RFC3339 |

---

## Secure Development Practices

### Dependency Management
- `cargo audit` in CI pipeline
- `cargo deny check licenses` in CI
- Minimal dependencies (prefer stdlib)
- Pinned versions in `Cargo.toml`
- `cargo deny check advisories` before release

### Code Safety
- **No `unsafe` without documented invariants**
- **No `unwrap()`/`expect()` in production paths**
- **No `panic!` in library code**
- `cargo clippy -- -D warnings` enforced

### Input Validation
- All API endpoints validate input (serde + custom validators)
- Collector parameters validated against JSON Schema
- Script parsing with error recovery (pest)
- Path traversal prevention in evidence root

### Secrets Management
- **No secrets in code or config files**
- Collector Ed25519 keys embedded at build time
- No runtime key generation
- No external secret stores in MVP

---

## Vulnerability Disclosure Process

1. **Report** → Private email/GitHub Security tab
2. **Acknowledge** → Within 48 hours
3. **Assess** → Severity, impact, exploitability
4. **Fix** → Develop patch, test thoroughly
5. **Coordinate** → CVE request if applicable
6. **Disclose** → Public advisory after fix released
7. **Credit** → Acknowledge reporter (if desired)

---

## Security Boundaries (What We Don't Implement)

### Offensive Capabilities (EXPLICITLY EXCLUDED)
- EDR/AV disabling or bypass
- BYOVD (Bring Your Own Vulnerable Driver)
- Process injection (CreateRemoteThread, etc.)
- Covert C2 channels (DNS, HTTP, etc.)
- Malware persistence (registry, scheduled tasks, etc.)
- Credential theft (LSASS dumping, DCSync, etc.)
- Security control bypass (AMSI, ETW, WDAC, etc.)
- Anti-forensics (timestomping, log clearing, etc.)
- Lateral movement execution (psexec, WMI, SMB, etc.)
- Privilege escalation exploits

### Dual-Use Features (Monitored)
| Feature | Legitimate Use | Risk | Mitigation |
|---------|----------------|------|------------|
| Memory acquisition | Forensic analysis | Could dump LSASS | Capability: `MemoryAcquire` (audited) |
| Registry modification | Evidence collection | Could create persistence | Capability: `RegistryWrite` (logged) |
| Network capture | Traffic analysis | Could sniff credentials | Capability: `NetworkCapture` (time-limited) |
| Process enumeration | Investigation | Could recon targets | Capability: `ProcessEnumerate` (read-only) |

---

## Compliance Considerations

| Standard | Alignment |
|----------|-----------|
| NIST SP 800-86 | Evidence integrity, chain of custody |
| NIST SP 800-53 | AU-2, AU-3, AU-6, AU-10, SC-8, SI-7 |
| ISO 27001 | A.8.2, A.12.4, A.16.1 |
| GDPR | Data minimization (no PII collected by default) |

---

## Security Contacts

| Role | Contact |
|------|---------|
| Security Lead | security@jockey-forensics.example.com |
| Lead Architect | architect@jockey-forensics.example.com |
| Maintainer | maintainers@jockey-forensics.example.com |

---

## Updates

This policy will be updated as the project evolves. Significant changes will be announced via GitHub releases and the project's communication channels.

---

**Last Updated:** October 2024  
**Version:** 0.1.0-draft