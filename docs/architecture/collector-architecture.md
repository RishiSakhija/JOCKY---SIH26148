# Collector Architecture

**Status**: SPECIFIED

---

## Collector Trait

```rust
#[async_trait]
pub trait Collector: Send + Sync {
    fn name(&self) -> &'static str;
    fn version(&self) -> &'static str;
    fn supported_actions(&self) -> Vec<&'static str>;
    fn supported_os(&self) -> Vec<&'static str>;
    async fn execute(
        &self,
        action: &str,
        params: Value,
        context: &ExecutionContext,
    ) -> Result<CollectorOutput, CollectorError>;
}
```

### ExecutionContext

```rust
pub struct ExecutionContext {
    pub run_id: String,
    pub evidence_root: PathBuf,
    pub target: TargetSpec,
    pub temp_dir: PathBuf,
    pub credentials: Option<Value>,
}
```

### CollectorOutput

```rust
pub struct CollectorOutput {
    pub evidence: Vec<Evidence>,
    pub artifacts: Vec<ArtifactRef>,
    pub metrics: CollectorMetrics,
}

pub struct ArtifactRef {
    pub path: String,
    pub type: String,
    pub description: String,
}

pub struct CollectorMetrics {
    pub duration_ms: u64,
    pub bytes_read: u64,
    pub bytes_written: u64,
    pub errors: Vec<String>,
}
```

### CollectorError

```rust
enum CollectorError {
    NotFound(String),
    PermissionDenied(String),
    InvalidParameter(String),
    Timeout,
    Internal(String),
    UnsupportedAction(String),
    UnsupportedOS(String),
}
```

---

## Collector Registry

```rust
pub struct CollectorRegistry {
    collectors: HashMap<String, Box<dyn Collector>>,
    manifests: HashMap<String, CollectorManifest>,
}

impl CollectorRegistry {
    pub fn register(&mut self, collector: Box<dyn Collector>) -> Result<(), RegistryError>;
    pub fn get(&self, name: &str) -> Option<&dyn Collector>;
    pub fn get_for_os(&self, os: &str) -> Vec<&dyn Collector>;
    pub fn get_manifest(&self, name: &str) -> Option<&CollectorManifest>;
    pub fn validate_contract(&self, contract: &ExecutionContract) -> Result<(), PolicyError>;
}
```

### CollectorManifest

```rust
pub struct CollectorManifest {
    pub name: String,
    pub version: String,
    pub supported_os: Vec<String>,
    pub capabilities: Vec<Capability>,
    pub actions: HashMap<String, ActionManifest>,
    pub public_key: String,  // Ed25519 public key for receipts
}

pub struct ActionManifest {
    pub capabilities: Vec<Capability>,
    pub params_schema: Value,  // JSON Schema
    pub description: String,
}
```

---

## Windows Collectors (MVP)

### 1. proc_windows — Process Enumeration

**Actions**: `list`, `get`, `tree`

**Evidence Type**: `artifact/process`

**Windows APIs**:
- `CreateToolhelp32Snapshot` + `Process32First/Next`
- `OpenProcess` + `GetModuleFileNameEx` + `GetProcessImageFileName`
- WMI `Win32_Process` (fallback for command line)

### 2. file_windows — File System Collection

**Actions**: `enumerate`, `hash`, `collect`, `mft`, `usn`, `vss`

**Evidence Types**: `artifact/file`, `artifact/ntfs`, `artifact/mft`

**Windows APIs**:
- `FindFirstFile/FindNextFile`
- `CreateFile` + `ReadFile`
- `FSCTL_QUERY_USN_JOURNAL`
- `IVssBackupComponents` (optional)

### 3. registry_windows — Registry Collection

**Actions**: `enumerate`, `get`, `monitor`

**Evidence Type**: `artifact/reg`

**Windows APIs**:
- `RegOpenKeyEx` + `RegEnumKeyEx` + `RegEnumValue`
- `RegLoadKey` / `RegUnLoadKey` (offline hives)
- `RegNotifyChangeKeyValue` (monitoring)

### 4. logs_windows — Event Log Collection

**Actions**: `query`, `export`, `tail`

**Evidence Type**: `artifact/evt`

**Windows APIs**:
- `EvtQuery` + `EvtNext` (XPath)
- `EvtExportLog` (full export)
- `EvtSubscribe` (real-time)

### 5. net_windows — Network Connection Collection

**Actions**: `connections`, `listen`, `capture`, `resolve`

**Evidence Type**: `artifact/connection`

**Windows APIs**:
- `GetExtendedTcpTable` / `GetExtendedUdpTable`
- NDIS / WFP / ETW (packet capture - optional)

---

## Linux Collectors (MVP)

### 1. proc_linux — Process Enumeration

**Actions**: `list`, `get`, `tree`

**Evidence Type**: `artifact/process`

**Linux Interfaces**:
- `/proc/[pid]/stat`, `/proc/[pid]/status`, `/proc/[pid]/cmdline`
- `/proc/[pid]/exe`, `/proc/[pid]/cwd`, `/proc/[pid]/fd/`
- `/proc/[pid]/maps`, `/proc/[pid]/smaps`
- `/proc/[pid]/loginuid`, `/proc/[pid]/sessionid`

### 2. file_linux — File System Collection

**Actions**: `enumerate`, `hash`, `collect`, `fanotify`

**Evidence Type**: `artifact/file`

**Linux Interfaces**:
- `libc::opendir` / `readdir` / `closedir`
- `openat` + `read`
- `fanotify_init` + `fanotify_mark` (optional)
- `statx`, `getxattr` / `listxattr`

### 3. config_linux — Configuration Collection

**Actions**: `enumerate`, `get`, `systemd`, `journald`

**Evidence Types**: `artifact/config`, `artifact/journald`

**Linux Interfaces**:
- Standard file I/O for `/etc`, `/usr/lib/systemd`, `~/.config`
- `sd_journal` API or `journalctl --output=json`
- `systemctl list-units --output=json`

### 4. logs_linux — Log Collection

**Actions**: `query`, `export`, `audit`

**Evidence Types**: `artifact/syslog`, `artifact/audit`

**Linux Interfaces**:
- Standard file I/O for `/var/log/*`
- `auditd` logs: `/var/log/audit/audit.log`
- `rsyslog`/`syslog-ng` structured logs

### 5. net_linux — Network Connection Collection

**Actions**: `connections`, `listen`, `capture`, `conntrack`, `resolve`

**Evidence Type**: `artifact/connection`

**Linux Interfaces**:
- `/proc/net/tcp`, `/proc/net/tcp6`, `/proc/net/udp`, `/proc/net/udp6`
- `/proc/net/netlink` (conntrack via `libnetfilter_conntrack`)
- `AF_PACKET` sockets / `libpcap` (packet capture)
- `getaddrinfo` / `getnameinfo` (resolution)

---

## Cross-Platform Evidence Mapping

| Semantic Type | Windows Collector | Linux Collector | Common Evidence Type |
|---------------|-------------------|-----------------|---------------------|
| Process | `proc_windows` | `proc_linux` | `artifact/process` |
| File | `file_windows` | `file_linux` | `artifact/file` |
| Registry/Config | `registry_windows` | `config_linux` | `artifact/reg` / `artifact/config` |
| Logs | `logs_windows` | `logs_linux` | `artifact/evt` / `artifact/journald` / `artifact/syslog` / `artifact/audit` |
| Network | `net_windows` | `net_linux` | `artifact/connection` |

---

## eBPF Extension Architecture (Post-MVP)

### eBPF Collectors

| Collector | Purpose | Kernel Features |
|-----------|---------|-----------------|
| `proc_ebpf` | Process exec/fork/exit tracing | `tracepoint`, `kprobe` (sched_process_exec, sched_process_fork, sched_process_exit) |
| `file_ebpf` | File open/read/write/unlink tracing | `tracepoint` (sys_enter_openat, sys_exit_openat, vfs_write, vfs_unlink) |
| `net_ebpf` | Socket connect/accept/send/recv | `kprobe`/`tracepoint` (inet_csk_accept, tcp_connect, tcp_sendmsg) |
| `cred_ebpf` | Credential access (ptrace, process_vm_readv) | `kprobe` (ptrace_access_vm, __vma_link_file) |

### Architecture

```
┌─────────────────────────────────────────────────────────────┐
│                      JOCKY RUNTIME                           │
├─────────────────────────────────────────────────────────────┤
│  Collector Registry                                          │
│  ├── proc_linux (userspace)                                 │
│  ├── proc_ebpf (eBPF) ◄── eBPF Manager                      │
│  ├── file_linux (userspace)                                 │
│  ├── file_ebpf (eBPF) ◄── eBPF Manager                      │
│  └── ...                                                     │
├─────────────────────────────────────────────────────────────┤
│  eBPF Manager (Aya)                                         │
│  ├── Program loading (BTF, CO-RE)                           │
│  ├── Map management (perf ring buffer, hash maps)           │
│  │── Kernel version compatibility matrix                    │
│  │── Verifier log capture                                   │
│  │── Graceful fallback to userspace collectors              │
│  └── Safety: no persistent kernel state, unload on exit     │
└─────────────────────────────────────────────────────────────┘
```

### Compatibility Matrix (Documented)

| Kernel Version | BTF | CO-RE | Ring Buffer | Notes |
|----------------|-----|-------|-------------|-------|
| 5.10+ | ✅ | ✅ | ✅ | Full support |
| 5.4-5.9 | ⚠️ | ⚠️ | ✅ | Partial BTF |
| 4.19-5.3 | ❌ | ❌ | ⚠️ | Legacy, limited |
| <4.19 | ❌ | ❌ | ❌ | Not supported |

### Safety Guarantees

- Programs verified by kernel verifier
- No persistent kernel state
- Automatic unload on collector drop
- Resource limits (CPU, memory) via cgroups
- Capability bounding: `CAP_BPF`, `CAP_PERFMON`, `CAP_SYS_ADMIN` required

---

## Implementation Guidelines

### Safety

- All `windows-sys` calls wrapped in `unsafe` blocks with documented invariants
- Handle cleanup via `Drop` implementations
- No raw pointer arithmetic in safe code
- Linux syscalls via `nix` crate or safe `libc` wrappers

### Error Handling

- Map Windows error codes / `errno` to `CollectorError` variants
- Distinguish: `AccessDenied`/`PermissionDenied`, `NotFound`, `InvalidParameter`, `Timeout`, `Internal`

### Performance

- Batch enumeration where possible
- Stream large results (don't load all into memory)
- Configurable page sizes for paginated APIs
- Use `uring` for async I/O (optional, future)

### Testing

- Unit tests with mocked OS interfaces
- Integration tests on Windows/Linux CI runners
- Golden file tests for evidence JSON structure

---

## Related Documents

- `overview.md` — Architecture overview
- `system-architecture.md` — Component architecture
- `data-flow.md` — Evidence pipeline and data flow
- `execution-contract.md` — Execution Contract specification
- `correlation-architecture.md` — Correlation rules and graph
- `deployment-model.md` — Deployment topology and models
- `evidence-pipeline.md` — Evidence pipeline details
- `collectors/windows/README.md` — Windows collector specs
- `collectors/linux/README.md` — Linux collector specs