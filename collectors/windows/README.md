# Windows Collectors Specification

**Status**: SPECIFIED — Not Implemented

---

## Overview

Windows collectors are native Rust implementations using `windows-sys` crate for safe Windows API access. Each collector implements the `Collector` trait from `jockey-collectors`.

**Design Principles:**
- Userspace only (no kernel drivers for MVP)
- Capability-scoped execution
- Evidence written to isolated evidence root
- Immediate hashing and receipt generation

---

## Collector Specifications

### 1. proc_windows — Process Enumeration

**Purpose**: Enumerate running processes with full metadata

**Actions**:
| Action | Description | Parameters |
|--------|-------------|------------|
| `list` | List all processes | `filter?: ProcessFilter` |
| `get` | Get single process by PID | `pid: u32` |
| `tree` | Build process tree | `root_pid?: u32` |

**Output Evidence Type**: `artifact/process`

**Windows APIs**:
- `CreateToolhelp32Snapshot` + `Process32First/Next`
- `OpenProcess` + `GetModuleFileNameEx` + `GetProcessImageFileName`
- WMI `Win32_Process` (fallback for command line)

**ProcessFilter**:
```rust
struct ProcessFilter {
    name_contains: Option<String>,
    pid: Option<u32>,
    ppid: Option<u32>,
    username_contains: Option<String>,
    cmdline_contains: Option<String>,
}
```

**Evidence Metadata**:
```json
{
  "pid": 1234,
  "ppid": 567,
  "name": "svchost.exe",
  "path": "C:\\Windows\\System32\\svchost.exe",
  "cmdline": "C:\\Windows\\System32\\svchost.exe -k netsvcs",
  "username": "SYSTEM",
  "integrity_level": "System",
  "start_time": "2026-10-04T10:00:00Z",
  "cpu_time_ms": 1250,
  "memory_bytes": 25600000,
  "threads": 12,
  "handles": 450,
  "modules": ["ntdll.dll", "kernel32.dll", "..."]
}
```

---

### 2. file_windows — File System Collection

**Purpose**: File enumeration, hashing, and collection

**Actions**:
| Action | Description | Parameters |
|--------|-------------|------------|
| `enumerate` | List files matching pattern | `path: String`, `recursive?: bool`, `pattern?: String` |
| `hash` | Compute hash of file(s) | `paths: Vec<String>`, `algorithm?: "blake3"\|"sha256"` |
| `collect` | Copy file to evidence root | `source: String`, `preserve_timestamps?: bool` |
| `mft` | Parse MFT records | `volume: String`, `filter?: MftFilter` |
| `usn` | Read USN journal | `volume: String`, `since_usn?: u64` |
| `vss` | List/access shadow copies | `volume: String` |

**Output Evidence Types**: `artifact/file`, `artifact/ntfs`, `artifact/mft`

**Windows APIs**:
- `FindFirstFile/FindNextFile` (enumeration)
- `CreateFile` + `ReadFile` (hashing/collection)
- `FSCTL_QUERY_USN_JOURNAL` (USN)
- `IVssBackupComponents` (VSS - optional)

**Evidence Metadata (file)**:
```json
{
  "path": "C:\\Users\\user\\Documents\\malware.exe",
  "size": 102400,
  "created": "2026-10-04T09:00:00Z",
  "modified": "2026-10-04T10:30:00Z",
  "accessed": "2026-10-04T11:00:00Z",
  "attributes": ["Archive", "Hidden"],
  "owner": "DOMAIN\\user",
  "hash_blake3": "a1b2c3d4...",
  "hash_sha256": "e5f6g7h8...",
  "mft_record_num": 12345,
  "mft_seq_num": 1,
  "is_ads": false,
  "ads_names": []
}
```

---

### 3. registry_windows — Registry Collection

**Purpose**: Registry hive enumeration and value collection

**Actions**:
| Action | Description | Parameters |
|--------|-------------|------------|
| `enumerate` | Enumerate keys/values | `hive: "HKLM"\|"HKCU"\|...`, `path: String`, `recursive?: bool` |
| `get` | Get specific value | `hive: String`, `path: String`, `name: String` |
| `monitor` | Monitor for changes (snapshot diff) | `hive: String`, `path: String`, `baseline_run_id: String` |

**Output Evidence Type**: `artifact/reg`

**Windows APIs**:
- `RegOpenKeyEx` + `RegEnumKeyEx` + `RegEnumValue`
- `RegLoadKey` / `RegUnLoadKey` (offline hives)
- `RegNotifyChangeKeyValue` (monitoring)

**Evidence Metadata**:
```json
{
  "hive": "HKLM",
  "path": "Software\\Microsoft\\Windows\\CurrentVersion\\Run",
  "key_name": "Malware",
  "value_type": "REG_SZ",
  "value_data": "C:\\Temp\\malware.exe",
  "last_write_time": "2026-10-04T10:15:00Z",
  "permissions": "KEY_ALL_ACCESS"
}
```

---

### 4. logs_windows — Event Log Collection

**Purpose**: Windows Event Log (EVTX) query and export

**Actions**:
| Action | Description | Parameters |
|--------|-------------|------------|
| `query` | XPath query events | `channel: String`, `xpath: String`, `limit?: u32` |
| `export` | Export full channel | `channel: String`, `format: "jsonl"\|"xml"` |
| `tail` | Follow new events | `channel: String`, `duration_ms: u32` |

**Output Evidence Type**: `artifact/evt`

**Channels**: `Security`, `System`, `Application`, `Microsoft-Windows-Sysmon/Operational`, `Windows PowerShell`, `Microsoft-Windows-TerminalServices-LocalSessionManager/Operational`, custom channels

**Windows APIs**:
- `EvtQuery` + `EvtNext` (XPath)
- `EvtExportLog` (full export)
- `EvtSubscribe` (real-time)

**Evidence Metadata**:
```json
{
  "channel": "Security",
  "event_id": 4625,
  "event_record_id": 1234567,
  "timestamp": "2026-10-04T10:00:00.123Z",
  "provider": "Microsoft-Windows-Security-Auditing",
  "level": "Information",
  "task": "Logon",
  "keywords": "Audit Failure",
  "computer": "WORKSTATION-01",
  "xml": "<Event>...</Event>",
  "parsed": {
    "SubjectUserSid": "S-1-0-0",
    "TargetUserName": "admin",
    "TargetDomainName": "WORKSTATION-01",
    "LogonType": 3,
    "Status": "0xC000006D",
    "IpAddress": "192.168.1.100",
    "IpPort": 49234
  }
}
```

---

### 5. net_windows — Network Connection Collection

**Purpose**: Network connection enumeration and capture

**Actions**:
| Action | Description | Parameters |
|--------|-------------|------------|
| `connections` | List active connections | `protocol?: "tcp"\|"udp"`, `state?: "established"\|"listen"\|...` |
| `listen` | List listening ports | `protocol?: "tcp"\|"udp"` |
| `capture` | Capture packets (requires admin) | `interface: String`, `filter?: String`, `duration_ms: u32`, `max_packets?: u32` |
| `resolve` | Resolve IPs to hostnames | `ips: Vec<String>` |

**Output Evidence Type**: `artifact/connection`

**Windows APIs**:
- `GetExtendedTcpTable` / `GetExtendedUdpTable` (connections)
- `GetTcpTable2` / `GetUdpTable2` (legacy)
- NDIS / WFP / ETW (packet capture - optional)

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
  "process_name": "chrome.exe",
  "offload_state": "TCP_OFFLOAD_STATE_NONE",
  "timestamp": "2026-10-04T12:00:00Z"
}
```

---

## Cross-Platform Evidence Mapping

| Windows Collector | Linux Counterpart | Common Evidence Type |
|-------------------|-------------------|---------------------|
| proc_windows | proc_linux | artifact/process |
| file_windows | file_linux | artifact/file |
| registry_windows | config_linux | artifact/reg / artifact/config |
| logs_windows | logs_linux | artifact/evt / artifact/journald |
| net_windows | net_linux | artifact/connection |

---

## Implementation Notes

### Safety
- All `windows-sys` calls wrapped in `unsafe` blocks with documented invariants
- Handle cleanup via `Drop` implementations
- No raw pointer arithmetic in safe code

### Error Handling
- Map Windows error codes to `CollectorError` variants
- Distinguish: `AccessDenied`, `NotFound`, `InvalidParameter`, `Timeout`, `Internal`

### Performance
- Batch enumeration where possible
- Stream large results (don't load all into memory)
- Configurable page sizes for paginated APIs

### Testing
- Unit tests with mocked Windows APIs (using `mockall` or similar)
- Integration tests on Windows CI runner
- Golden file tests for evidence JSON structure

---

## Future Extensions (Post-MVP)

| Collector | Description |
|-----------|-------------|
| `memory_windows` | Live memory acquisition (WinPMEM style) |
| `etw_windows` | Real-time ETW session collection |
| `wmi_windows` | WMI query executor |
| `srum_windows` | System Resource Usage Monitor (SRUM) database |
| `amcache_windows` | Amcache.hve parser |
| `prefetch_windows` | Prefetch file parser |

---

## References

- Windows Sysinternals: https://learn.microsoft.com/en-us/sysinternals/
- Windows API documentation: https://learn.microsoft.com/en-us/windows/win32/apiindex/
- EVTX format: https://github.com/libyal/libevtx/blob/main/documentation/Windows%20XML%20Event%20Log%20(EVTX).asciidoc
- USN Journal: https://learn.microsoft.com/en-us/windows/win32/fileio/change-journals
- VSS API: https://learn.microsoft.com/en-us/windows/win32/vss/volume-shadow-copy-service-portal