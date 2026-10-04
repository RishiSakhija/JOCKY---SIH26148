# JOCKY Script Examples

**Status**: SPECIFIED

---

## Example 1: Failed Logon + Persistence Detection

```jocky
investigation "failed-logon-persistence" {
    hypothesis "Brute force followed by Run key persistence" {
        mitre: ["T1110", "T1547.001"]
    }

    collect process as logon_events where command_line contains "logon" and EventID == 4625
    collect registry as run_keys where path matches "HKLM\\Software\\Microsoft\\Windows\\CurrentVersion\\Run*"
    collect registry as run_keys_user where path matches "HKCU\\Software\\Microsoft\\Windows\\CurrentVersion\\Run*"

    correlate logon_events -> run_keys by "timestamp_join" window 300s as hk_anomaly
    correlate logon_events -> run_keys_user by "timestamp_join" window 300s as hkcu_anomaly

    timeline "logon_persistence" from hk_anomaly, hkcu_anomaly
    bind hypothesis "Brute force followed by Run key persistence" to hk_anomaly, hkcu_anomaly

    export case "logon-persist-case" format case_uco
    export graph "logon-persist-graph" format cytoscape
    export report "logon-persist-report" format markdown
}
```

---

## Example 2: PowerShell Command Line Analysis

```jocky
investigation "powershell-reconnaissance" {
    hypothesis "PowerShell used for reconnaissance and credential access" {
        mitre: ["T1059.001", "T1003", "T1082"]
    }

    collect process as ps_procs where name == "powershell.exe"
    collect process as ps_children where parent_name == "powershell.exe"
    collect logs as ps_logs where source == "Microsoft-Windows-PowerShell/Operational"
    collect network as ps_net where process_name == "powershell.exe"

    filter ps_procs where command_line matches "(?i)(invoke-expression|iex|downloadstring|webclient)"
    match ps_children where name in ["whoami.exe", "net.exe", "systeminfo.exe", "ipconfig.exe"]

    correlate ps_procs -> ps_children by "pid_link" as ps_tree
    correlate ps_tree -> ps_net by "process_netlink" as ps_network
    correlate ps_logs -> ps_procs by "timestamp_join" window 60s as ps_logged

    timeline "powershell_activity" from ps_tree, ps_network, ps_logged
    bind hypothesis "PowerShell used for reconnaissance and credential access" to ps_tree, ps_network

    export case "ps-recon-case" format case_uco
    export timeline "ps-recon-timeline" format timesketch
    export report "ps-recon-report" format markdown
}
```

---

## Example 3: Lateral Movement via SMB

```jocky
investigation "smb-lateral-movement" {
    hypothesis "Lateral movement via SMB admin shares and service creation" {
        mitre: ["T1021.002", "T1543.003", "T1569.002"]
    }

    collect logs as smb_logs where source == "Security" and EventID in [4624, 4625, 5145]
    collect logs as svc_logs where source == "System" and EventID == 7045
    collect network as smb_connections where destination_port == 445
    collect process as svc_procs where name == "services.exe"

    correlate smb_logs -> svc_logs by "timestamp_join" window 60s as smb_svc
    correlate smb_svc -> smb_connections by "process_netlink" as smb_chain
    correlate svc_procs -> smb_chain by "pid_link" as svc_smb

    timeline "smb_lateral" from smb_svc, smb_chain, svc_smb
    bind hypothesis "Lateral movement via SMB admin shares and service creation" to smb_svc, smb_chain

    export case "smb-lateral-case" format case_uco
    export graph "smb-lateral-graph" format cytoscape
    export report "smb-lateral-report" format markdown
}
```

---

## Example 4: Linux SSH Brute Force + Persistence

```jocky
investigation "ssh-brute-force-linux" {
    hypothesis "SSH brute force followed by authorized_keys persistence" {
        mitre: ["T1110", "T1548.002", "T1098.004"]
    }

    collect logs as ssh_logs where source == "auth" and message contains "Failed password"
    collect logs as ssh_accept where source == "auth" and message contains "Accepted password"
    collect file as auth_keys where path matches "/home/*/.ssh/authorized_keys"
    collect process as ssh_procs where name == "sshd"

    correlate ssh_logs -> ssh_accept by "timestamp_join" window 300s as brute_success
    correlate brute_success -> auth_keys by "timestamp_join" window 600s as key_persist
    correlate ssh_procs -> brute_success by "pid_link" as ssh_chain

    timeline "ssh_attack" from brute_success, key_persist, ssh_chain
    bind hypothesis "SSH brute force followed by authorized_keys persistence" to brute_success, key_persist

    export case "ssh-brute-case" format case_uco
    export graph "ssh-brute-graph" format cytoscape
    export report "ssh-brute-report" format markdown
}
```

---

## Example 5: Web Shell Detection

```jocky
investigation "web-shell-detection" {
    hypothesis "Web shell deployed via file upload, used for command execution" {
        mitre: ["T1190", "T1059", "T1505.003"]
    }

    collect logs as web_logs where source in ["IIS", "Apache", "Nginx"] and status == 200 and method == "POST"
    collect file as uploads where path matches "/var/www/*/uploads/*" or path matches "C:\\inetpub\\*\\uploads\\*"
    collect process as web_procs where parent_name in ["w3wp.exe", "httpd", "nginx", "php-fpm"]
    collect network as web_net where process_name in ["w3wp.exe", "httpd", "nginx", "php-fpm"]

    filter uploads where extension in [".php", ".asp", ".aspx", ".jsp", ".jspx"]
    match web_procs where command_line contains "cmd.exe" or command_line contains "/bin/bash"

    correlate web_logs -> uploads by "timestamp_join" window 120s as upload_exec
    correlate upload_exec -> web_procs by "process_netlink" as shell_chain
    correlate shell_chain -> web_net by "process_netlink" as shell_c2

    timeline "web_shell" from upload_exec, shell_chain, shell_c2
    bind hypothesis "Web shell deployed via file upload, used for command execution" to upload_exec, shell_chain

    export case "web-shell-case" format case_uco
    export graph "web-shell-graph" format cytoscape
    export report "web-shell-report" format markdown
}
```

---

## Example 6: Data Exfiltration via DNS

```jocky
investigation "dns-exfiltration" {
    hypothesis "Data exfiltration via DNS tunneling" {
        mitre: ["T1048.003", "T1572"]
    }

    collect network as dns_traffic where destination_port == 53 and protocol == "UDP"
    collect process as dns_procs where name in ["dns.exe", "named", "dnsmasq", "systemd-resolved"]
    collect logs as dns_logs where source == "DNS" and query_type == "TXT"

    filter dns_traffic where query_length > 100 or subdomain_count > 10
    match dns_logs where response_size > 512

    correlate dns_traffic -> dns_logs by "timestamp_join" window 10s as dns_anomaly
    correlate dns_anomaly -> dns_procs by "pid_link" as dns_chain

    timeline "dns_exfil" from dns_anomaly, dns_chain
    bind hypothesis "Data exfiltration via DNS tunneling" to dns_anomaly, dns_chain

    export case "dns-exfil-case" format case_uco
    export graph "dns-exfil-graph" format cytoscape
    export report "dns-exfil-report" format markdown
}
```

---

## Example 7: Scheduled Task Persistence

```jocky
investigation "scheduled-task-persistence" {
    hypothesis "Persistence via scheduled task creation" {
        mitre: ["T1053.005"]
    }

    collect logs as task_logs where source == "Microsoft-Windows-TaskScheduler/Operational" and EventID in [106, 140, 200, 201]
    collect registry as task_reg where path matches "HKLM\\Software\\Microsoft\\Windows NT\\CurrentVersion\\Schedule\\TaskCache\\Tasks\\*"
    collect process as task_procs where parent_name == "svchost.exe" and command_line contains "taskeng.exe"

    correlate task_logs -> task_reg by "timestamp_join" window 60s as task_created
    correlate task_created -> task_procs by "process_netlink" as task_exec

    timeline "task_persistence" from task_created, task_exec
    bind hypothesis "Persistence via scheduled task creation" to task_created, task_exec

    export case "task-persist-case" format case_uco
    export graph "task-persist-graph" format cytoscape
    export report "task-persist-report" format markdown
}
```

---

## Example 8: WMI Event Subscription Persistence

```jocky
investigation "wmi-persistence" {
    hypothesis "Persistence via WMI event subscription" {
        mitre: ["T1546.003"]
    }

    collect logs as wmi_logs where source == "Microsoft-Windows-WMI-Activity/Operational" and EventID in [5860, 5861]
    collect registry as wmi_reg where path matches "HKLM\\Software\\Microsoft\\WBEM\\Scripting\\*"
    collect process as wmi_procs where name == "wmiprvse.exe"

    correlate wmi_logs -> wmi_reg by "timestamp_join" window 60s as wmi_sub
    correlate wmi_sub -> wmi_procs by "process_netlink" as wmi_chain

    timeline "wmi_persistence" from wmi_sub, wmi_chain
    bind hypothesis "Persistence via WMI event subscription" to wmi_sub, wmi_chain

    export case "wmi-persist-case" format case_uco
    export graph "wmi-persist-graph" format cytoscape
    export report "wmi-persist-report" format markdown
}
```

---

## Example 9: Cross-Platform Credential Dumping

```jocky
investigation "credential-dumping" {
    hypothesis "Credential dumping via LSASS memory access" {
        mitre: ["T1003.001"]
    }

    // Windows
    collect process as lsass_access where target_process == "lsass.exe" and access_mask contains "PROCESS_VM_READ"
    collect file as dump_files where path matches "*.dmp" and size > 10000000
    
    // Linux
    collect process as gcore_access where command_line contains "gcore" and target contains "sshd"
    collect file as core_dumps where path matches "/var/crash/*" or path matches "/tmp/core.*"

    correlate lsass_access -> dump_files by "timestamp_join" window 30s as win_dump
    correlate gcore_access -> core_dumps by "timestamp_join" window 30s as lin_dump

    timeline "cred_dump" from win_dump, lin_dump
    bind hypothesis "Credential dumping via LSASS memory access" to win_dump, lin_dump

    export case "cred-dump-case" format case_uco
    export graph "cred-dump-graph" format cytoscape
    export report "cred-dump-report" format markdown
}
```

---

## Example 10: Multi-Stage Attack Reconstruction

```jocky
investigation "multi-stage-apt" {
    hypothesis "Multi-stage APT attack: initial access → lateral movement → data staging → exfil" {
        mitre: ["T1190", "T1021.004", "T1005", "T1041"]
    }

    // Stage 1: Initial Access (Web exploit)
    collect logs as web_exploit where source == "IIS" and status == 500 and path contains "exploit"
    collect process as web_shell where parent_name == "w3wp.exe" and command_line contains "cmd"

    // Stage 2: Lateral Movement (Pass-the-Hash)
    collect logs as pth_logs where source == "Security" and EventID == 4624 and LogonType == 9
    collect network as smb_admin where destination_port == 445 and share_name in ["ADMIN$", "C$", "IPC$"]

    // Stage 3: Data Staging
    collect file as staged_data where path matches "*\\staging\\*" and size > 100000000
    collect process as archive_procs where command_line contains "7z" or command_line contains "rar"

    // Stage 4: Exfiltration
    collect network as exfil_traffic where destination_port in [443, 80, 21, 22] and bytes_out > 10000000
    collect logs as proxy_logs where source == "Proxy" and bytes_out > 10000000

    // Correlations
    correlate web_exploit -> web_shell by "pid_link" as stage1
    correlate stage1 -> pth_logs by "timestamp_join" window 3600s as stage2
    correlate pth_logs -> smb_admin by "process_netlink" as smb_movement
    correlate smb_admin -> staged_data by "timestamp_join" window 7200s as stage3
    correlate staged_data -> archive_procs by "pid_link" as archived
    correlate archived -> exfil_traffic by "timestamp_join" window 1800s as stage4
    correlate exfil_traffic -> proxy_logs by "timestamp_join" window 60s as exfil_confirmed

    timeline "full_attack" from stage1, stage2, smb_movement, stage3, archived, stage4, exfil_confirmed
    bind hypothesis "Multi-stage APT attack" to stage1, stage2, smb_movement, stage3, archived, stage4, exfil_confirmed

    export case "apt-full-case" format case_uco
    export graph "apt-full-graph" format cytoscape
    export timeline "apt-full-timeline" format timesketch
    export report "apt-full-report" format markdown
}
```

---

## Cross-Platform Notes

All examples use **platform-agnostic evidence types**:
- `collect process` → `proc_windows` on Windows, `proc_linux` on Linux
- `collect file` → `file_windows` on Windows, `file_linux` on Linux
- `collect registry` → `registry_windows` on Windows, `config_linux` on Linux
- `collect logs` → `logs_windows` on Windows, `logs_linux` on Linux
- `collect network` → `net_windows` on Windows, `net_linux` on Linux

The compiler resolves the correct collector based on the Execution Contract's target OS.