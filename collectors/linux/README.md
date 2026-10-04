# Linux Collectors Specification

**Status**: SPECIFIED — Not Implemented

---

## Overview

Linux collectors are Rust userspace implementations using standard Linux interfaces (`/proc`, `/sys`, `auditd`, `journald`, `netlink`, `fanotify`). Each collector implements the `Collector` trait from `jockey-collectors`.

**Design Principles:**
- Userspace only (no kernel modules for MVP)
- Capability-scoped execution
- Evidence written to isolated evidence root
- Immediate hashing and receipt generation
- eBPF documented as extensible architecture (not implemented in MVP)

---

## Collector Specifications

### 1. proc_linux — Process Enumeration

**Purpose**: Enumerate running processes with full metadata

**Actions**:
| Action | Description | Parameters |
|--------|-------------|------------|
| `list` | List all processes | `filter?: ProcessFilter` |
| `get` | Get single process by PID | `pid: u32` |
| `tree` | Build process tree | `root_pid?: u32` |

**Output Evidence Type**: `artifact/process`

**Linux Interfaces**:
- `/proc/[pid]/stat`, `/proc/[pid]/status`, `/proc/[pid]/cmdline`
- `/proc/[pid]/exe` (symlink to executable)
- `/proc/[pid]/cwd`, `/proc/[pid]/fd/` (file descriptors)
- `/proc/[pid]/maps`, `/proc/[pid]/smaps` (memory maps)
- `/proc/[pid]/environ` (environment variables)
- `/proc/[pid]/loginuid`, `/proc/[pid]/sessionid` (audit context)

**ProcessFilter**:
```rust
struct ProcessFilter {
    name_contains: Option<String>,
    pid: Option<u32>,
    ppid: Option<u32>,
    uid: Option<u32>,
    gid: Option<u32>,
    cmdline_contains: Option<String>,
    state: Option<String>,  // R, S, D, Z, T, X
}
```

**Evidence Metadata**:
```json
{
  "pid": 1234,
  "ppid": 567,
  "name": "systemd",
  "path": "/usr/lib/systemd/systemd",
  "cmdline": ["/usr/lib/systemd/systemd", "--system", "--deserialize=24"],
  "uid": 0,
  "gid": 0,
  "euid": 0,
  "egid": 0,
  "state": "S",
  "start_time": "2026-10-04T10:00:00Z",
  "cpu_time_ms": 12500,
  "memory_rss_bytes": 25600000,
  "memory_vms_bytes": 102400000,
  "threads": 4,
  "fds": 45,
  "cgroups": ["systemd:1:/init.scope", "..."],
  "capabilities": ["CAP_DAC_OVERRIDE", "CAP_SYS_RESOURCE", "..."],
  "seccomp_mode": 2,
  "exe_inode": 123456,
  "exe_dev": 259
}
```

---

### 2. file_linux — File System Collection

**Purpose**: File enumeration, hashing, and collection

**Actions**:
| Action | Description | Parameters |
|--------|-------------|------------|
| `enumerate` | List files matching pattern | `path: String`, `recursive?: bool`, `pattern?: String`, `follow_symlinks?: bool` |
| `hash` | Compute hash of file(s) | `paths: Vec<String>`, `algorithm?: "blake3"\|"sha256"` |
| `collect` | Copy file to evidence root | `source: String`, `preserve_timestamps?: bool`, `preserve_xattrs?: bool` |
| `fanotify` | Monitor filesystem events | `path: String`, `mask?: u32`, `duration_ms: u32` |

**Output Evidence Type**: `artifact/file`

**Linux Interfaces**:
- `libc::opendir` / `readdir` / `closedir` (enumeration)
- `openat` + `read` (hashing/collection)
- `fanotify_init` + `fanotify_mark` (monitoring - optional)
- `statx` (extended attributes, birth time)
- `getxattr` / `listxattr` (extended attributes)

**Evidence Metadata**:
```json
{
  "path": "/home/user/malware",
  "size": 102400,
  "blocks": 200,
  "block_size": 4096,
  "inode": 1234567,
  "device": 259,
  "mode": "0755",
  "uid": 1000,
  "gid": 1000,
  "nlink": 1,
  "created": "2026-10-04T09:00:00Z",
  "modified": "2026-10-04T10:30:00Z",
  "accessed": "2026-10-04T11:00:00Z",
  "changed": "2026-10-04T10:30:00Z",
  "hash_blake3": "a1b2c3d4...",
  "hash_sha256": "e5f6g7h8...",
  "xattrs": {
    "user.comment": "downloaded from...",
    "security.selinux": "unconfined_u:object_r:user_home_t:s0"
  },
  "capabilities": "cap_dac_override,cap_sys_resource+ep",
  "selinux_context": "unconfined_u:object_r:user_home_t:s0"
}
```

---

### 3. config_linux — Configuration Collection

**Purpose**: System and user configuration file collection (replaces registry)

**Actions**:
| Action | Description | Parameters |
|--------|-------------|------------|
| `enumerate` | List config files in directory | `path: String`, `pattern?: String`, `recursive?: bool` |
| `get` | Get single config file | `path: String` |
| `systemd` | Enumerate systemd units | `type?: "service"\|"timer"\|"socket"\|...", state?: "active"\|"inactive"\|"failed"` |
| `journald` | Query systemd journal | `filter?: JournalFilter`, `since?: String`, `until?: String`, `limit?: u32` |

**Output Evidence Types**: `artifact/config`, `artifact/journald`

**Linux Interfaces**:
- Standard file I/O for `/etc`, `/usr/lib/systemd`, `~/.config`
- `sd_journal` API (libsystemd) or `journalctl --output=json`
- `systemctl list-units --output=json`

**JournalFilter**:
```rust
struct JournalFilter {
    unit: Option<String>,
    priority: Option<u32>,  // 0=emerg ... 7=debug
    syslog_facility: Option<u32>,
    message_contains: Option<String>,
    _uid: Option<u32>,
    _gid: Option<u32>,
    _comm: Option<String>,
    _exe: Option<String>,
}
```

**Evidence Metadata (systemd unit)**:
```json
{
  "unit_name": "malware.service",
  "description": "Malware Service",
  "load_state": "loaded",
  "active_state": "active",
  "sub_state": "running",
  "fragment_path": "/etc/systemd/system/malware.service",
  "exec_start": "/usr/bin/malware --daemon",
  "pid": 1234,
  "uid": 0,
  "gid": 0,
  "timer": null,
  "dependencies": ["network.target", "..."]
}
```

**Evidence Metadata (journald entry)**:
```json
{
  "cursor": "s=...;i=...;b=...",
  "realtime_timestamp": "2026-10-04T12:00:00.123456Z",
  "monotonic_timestamp": 123456789,
  "priority": 3,
  "syslog_facility": 3,
  "message": "Failed password for admin from 192.168.1.100",
  "unit": "sshd.service",
  "uid": 0,
  "gid": 0,
  "comm": "sshd",
  "exe": "/usr/sbin/sshd",
  "pid": 567,
  "syslog_identifier": "sshd",
  "transport": "audit",
  "audit_session": 123,
  "audit_loginuid": 1000
}
```

---

### 4. logs_linux — Log Collection

**Purpose**: Traditional syslog and auditd log collection

**Actions**:
| Action | Description | Parameters |
|--------|-------------|------------|
| `query` | Query log files | `path: String`, `pattern?: String`, `since?: String`, `until?: String`, `limit?: u32` |
| `export` | Export log file | `path: String`, `format: "jsonl"\|"text"` |
| `audit` | Query auditd logs | `filter?: AuditFilter`, `since?: String`, `limit?: u32` |

**Output Evidence Types**: `artifact/syslog`, `artifact/audit`

**Linux Interfaces**:
- Standard file I/O for `/var/log/*`
- `auditd` logs: `/var/log/audit/audit.log` (parsed with `libaudit` or custom parser)
- `rsyslog`/`syslog-ng` structured logs

**AuditFilter**:
```rust
struct AuditFilter {
    type_: Option<u16>,  // e.g., 1300=SYSCALL, 1100=USER_LOGIN
    syscall: Option<u32>,
    success: Option<bool>,
    pid: Option<u32>,
    uid: Option<u32>,
    auid: Option<u32>,
    exe_contains: Option<String>,
    key_contains: Option<String>,
}
```

**Evidence Metadata (syslog)**:
```json
{
  "source_file": "/var/log/auth.log",
  "line_number": 12345,
  "timestamp": "2026-10-04T12:00:00Z",
  "hostname": "server-01",
  "facility": "auth",
  "severity": "info",
  "program": "sshd",
  "pid": 567,
  "message": "Failed password for admin from 192.168.1.100 port 49234 ssh2"
}
```

**Evidence Metadata (audit)**:
```json
{
  "type": "USER_LOGIN",
  "timestamp": "2026-10-04T12:00:00.123Z",
  "serial": 12345,
  "audit_uid": 1000,
  "uid": 0,
  "gid": 0,
  "ses": 123,
  "pid": 567,
  "comm": "sshd",
  "exe": "/usr/sbin/sshd",
  "success": "no",
  "msg": "login failed for admin from 192.168.1.100",
  "terminal": "ssh",
  "res": "failed",
  "acct": "admin"
}
```

---

### 5. net_linux — Network Connection Collection

**Purpose**: Network connection enumeration and capture

**Actions**:
| Action | Description | Parameters |
|--------|-------------|------------|
| `connections` | List active connections | `protocol?: "tcp"\|"udp"`, `state?: "established"\|"listen"\|...` |
| `listen` | List listening ports | `protocol?: "tcp"\|"udp"` |
| `capture` | Capture packets (requires CAP_NET_RAW) | `interface: String`, `filter?: String`, `duration_ms: u32`, `max_packets?: u32` |
| `conntrack` | Query conntrack table | `filter?: ConntrackFilter` |
| `resolve` | Resolve IPs to hostnames | `ips: Vec<String>` |

**Output Evidence Type**: `artifact/connection`

**Linux Interfaces**:
- `/proc/net/tcp`, `/proc/net/tcp6`, `/proc/net/udp`, `/proc/net/udp6`
- `/proc/net/netlink` (conntrack via `libnetfilter_conntrack`)
- `AF_PACKET` sockets / `libpcap` (packet capture)
- `getaddrinfo` / `getnameinfo` (resolution)

**Evidence Metadata**:
```json
{
  "protocol": "TCP",
  "local_ip": "192.168.1.50",
  "local_port": 49234,
  "remote_ip": "192.168.1.100",
  "remote_port": 443,
  "state": "ESTABLISHED",
  "pid": 1234,
  "process_name": "firefox",
  "uid": 1000,
  "inode": 123456,
  "rx_bytes": 102400,
  "tx_bytes": 51200,
  "timestamp": "2026-10-04T12:00:00Z"
}
```

---

## eBPF Extension Architecture (Post-MVP)

**Direction**: Aya-based eBPF collectors for enhanced visibility

| eBPF Collector | Purpose | Kernel Features Required |
|----------------|---------|-------------------------|
| `proc_ebpf` | Process exec/fork/exit tracing | `tracepoint`, `kprobe` (sched_process_exec, sched_process_fork, sched_process_exit) |
| `file_ebpf` | File open/read/write/unlink tracing | `tracepoint` (sys_enter_openat, sys_exit_openat, vfs_write, vfs_unlink) |
| `net_ebpf` | Socket connect/accept/send/recv | `kprobe`/`tracepoint` (inet_csk_accept, tcp_connect, tcp_sendmsg) |
| `cred_ebpf` | Credential access (ptrace, process_vm_readv) | `kprobe` (ptrace_access_vm, __vma_link_file) |
| `cap_ebpf` | Capability checks | `kprobe` (capable, ns_capable) |

**Architecture**:
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
│   │── Kernel version compatibility matrix                   │
│   │── Verifier log capture                                  │
│   │── Graceful fallback to userspace collectors             │
│  └── Safety: no persistent kernel state, unload on exit     │
└─────────────────────────────────────────────────────────────┘
```

**Compatibility Matrix** (Documented, Not Implemented):
| Kernel Version | BTF | CO-RE | Ring Buffer | Notes |
|----------------|-----|-------|-------------|-------|
| 5.10+ | ✅ | ✅ | ✅ | Full support |
| 5.4-5.9 | ⚠️ | ⚠️ | ✅ | Partial BTF |
| 4.19-5.3 | ❌ | ❌ | ⚠️ | Legacy, limited |
| <4.19 | ❌ | ❌ | ❌ | Not supported |

**Safety Guarantees**:
- Programs verified by kernel verifier
- No persistent kernel state
- Automatic unload on collector drop
- Resource limits (CPU, memory) via cgroups
- Capability bounding: `CAP_BPF`, `CAP_PERFMON`, `CAP_SYS_ADMIN` required

---

## Cross-Platform Evidence Mapping

| Linux Collector | Windows Counterpart | Common Evidence Type |
|-----------------|---------------------|---------------------|
| proc_linux | proc_windows | artifact/process |
| file_linux | file_windows | artifact/file |
| config_linux | registry_windows | artifact/config / artifact/reg |
| logs_linux | logs_windows | artifact/journald / artifact/syslog / artifact/audit / artifact/evt |
| net_linux | net_windows | artifact/connection |

---

## Implementation Notes

### Safety
- All syscalls wrapped with proper error handling
- File descriptor management via `OwnedFd` / `RawFd` with `Drop`
- No `unsafe` without documented invariants

### Error Handling
- Map `errno` to `CollectorError` variants
- Distinguish: `PermissionDenied`, `NotFound`, `InvalidInput`, `Timeout`, `Internal`

### Performance
- Stream `/proc` reads (don't load all PIDs at once)
- Batch `statx` calls where possible
- Use `uring` for async I/O (optional, future)

### Testing
- Unit tests with mocked `/proc` (using `tempfile` + synthetic data)
- Integration tests on Linux CI runner (Ubuntu 22.04, 24.04)
- Golden file tests for evidence JSON structure

---

## Future Extensions (Post-MVP)

| Collector | Description |
|-----------|-------------|
| `memory_linux` | Live memory acquisition (LiME/AVML style) |
| `audit_ebpf` | eBPF-based auditd replacement |
| `container_linux` | Docker/containerd/podman artifact collection |
| `k8s_linux` | Kubernetes API resource collection |
| `ebpf_linux` | Unified eBPF manager + collectors |

---

## References

- procfs: https://man7.org/linux/man-pages/man5/proc.5.html
- fanotify: https://man7.org/linux/man-pages/man7/fanotify.7.html
- auditd: https://linux.die.net/man/8/auditd
- systemd journal: https://www.freedesktop.org/software/systemd/man/systemd-journald.service.html
- conntrack: https://www.netfilter.org/projects/conntrack-tools/
- Aya (eBPF): https://aya-rs.dev/
- BTF/CO-RE: https://docs.kernel.org/bpf/btf.html