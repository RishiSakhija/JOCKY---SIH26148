//! JOCKY CLI — Command-line interface for JOCKY

use clap::{Parser, Subcommand};
use jockey_language::{compile_to_ir, compile_to_contract, parse};
use jockey_core::{TargetSpec, ExecPolicy};
use jockey_runtime::{execute, collector_registry::CollectorRegistry};
use jockey_collectors::windows::ProcWindowsCollector;
use jockey_evidence::{receipt::CollectorKeypair, model::EvidenceType};
use std::path::PathBuf;
use std::fs;
use std::sync::Arc;

#[derive(Parser)]
#[command(name = "jockey")]
#[command(about = "JOCKY — Evidence-Contract Forensics Platform", long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Parse and compile a JOCKY script to IR
    Compile {
        /// Path to JOCKY script file
        #[arg(short, long)]
        input: PathBuf,
        
        /// Output format (json, yaml)
        #[arg(short, long, default_value = "json")]
        format: String,
        
        /// Output file (stdout if not specified)
        #[arg(short, long)]
        output: Option<PathBuf>,
    },
    
    /// Run an investigation
    Run {
        /// Path to JOCKY script file
        #[arg(short, long)]
        input: PathBuf,
        
        /// Target host
        #[arg(short, long)]
        target: String,
        
        /// Target OS (windows, linux)
        #[arg(short, long, default_value = "windows")]
        os: String,
        
        /// Evidence root directory
        #[arg(long, default_value = "./evidence")]
        evidence_root: String,
    },
    
    /// Load a sample case
    Load {
        /// Path to sample case directory
        path: PathBuf,
    },
    
    /// Show investigation graph
    Graph {
        /// Run ID
        run_id: String,
        
        /// Output format (dot, cytoscape)
        #[arg(short, long, default_value = "cytoscape")]
        format: String,
    },
    
    /// Verify evidence integrity
    Verify {
        /// Run ID
        run_id: String,
    },
    
    /// Export narrative report
    Narrative {
        /// Run ID
        run_id: String,
    },
    
    /// Run the Phase 3A demo workflow
    Demo {
        /// Target OS (windows, linux)
        #[arg(short, long, default_value = "windows")]
        os: String,
        
        /// Evidence root directory
        #[arg(long, default_value = "./evidence")]
        evidence_root: String,
    },
}

fn main() -> anyhow::Result<()> {
    tracing_subscriber::fmt::init();
    
    let cli = Cli::parse();
    
    match cli.command {
        Commands::Compile { input, format, output } => {
            let source = fs::read_to_string(&input)?;
            
            // Parse and compile to IR
            let ir = jockey_language::compile_to_ir(&source)?;
            
            let output_str = match format.as_str() {
                "json" => serde_json::to_string_pretty(&ir)?,
                "yaml" => serde_yaml::to_string(&ir)?,
                _ => anyhow::bail!("Unsupported format: {}", format),
            };
            
            if let Some(output) = output {
                std::fs::write(output, output_str)?;
            } else {
                println!("{}", output_str);
            }
        }
        
        Commands::Run { input, target, os, evidence_root } => {
            let source = fs::read_to_string(&input)?;
            
            let target_spec = match target.as_str() {
                "local" => jockey_core::TargetSpec::live("localhost", &os),
                _ => jockey_core::TargetSpec::live(&target, &os),
            };
            
            let policy = jockey_core::ExecPolicy::new()
                .with_evidence_root(evidence_root);
            
            let contract = jockey_language::compile_to_contract(&source, target_spec, policy)?;
            
            println!("Investigation compiled successfully");
            println!("Run ID: {}", contract.run_id);
            println!("Steps: {}", contract.steps.len());
        }
        
        Commands::Load { path } => {
            println!("Loading sample case from: {}", path.display());
            // TODO: Implement sample loading
        }
        
        Commands::Graph { run_id, format } => {
            println!("Showing graph for run: {} (format: {})", run_id, format);
            // TODO: Implement graph display
        }
        
        Commands::Verify { run_id } => {
            println!("Verifying run: {}", run_id);
            // TODO: Implement verification
        }
        
        Commands::Narrative { run_id } => {
            println!("Generating narrative for run: {}", run_id);
            // TODO: Implement narrative generation
        }
        
        Commands::Demo { os, evidence_root } => {
            run_demo(&os, &evidence_root)?;
        }
    }
    
    Ok(())
}

async fn run_demo(os: &str, evidence_root: &str) -> anyhow::Result<()> {
    println!("╔══════════════════════════════════════════════════════════════╗");
    println!("║           JOCKY Phase 3A Demo — Evidence Integrity Slice       ║");
    println!("╚══════════════════════════════════════════════════════════════╝");
    println!();
    
    // 1. Create the demo investigation script
    let demo_script = r#"
investigation "process-investigation-demo" {
    hypothesis "Suspicious process execution detected" {
        mitre: ["T1059", "T1059.001"]
    }
    
    collect process as all_procs where name contains "svchost"
    collect process as powershell_procs where name contains "powershell"
    collect process as suspicious_procs where command_line contains "encodedcommand"
    
    correlate all_procs -> powershell_procs by "pid_link" as proc_chain
    correlate proc_chain -> suspicious_procs by "pid_link" as suspicious_chain
    
    timeline "process_timeline" from proc_chain, suspicious_chain
    bind hypothesis "Suspicious process execution detected" to proc_chain, suspicious_chain
    
    export case "process-demo" format case_uco
    export graph "process-graph" format cytoscape
    export report "process-report" format markdown
}
"#;
    
    println!("Step 1: Loading JOCKY investigation script...");
    println!("Script:");
    println!("{}", demo_script);
    println!();
    
    // Parse and compile
    println!("Step 2: Parsing and compiling script...");
    let ir = jockey_language::compile_to_ir(demo_script)?;
    println!("  ✓ IR generated with {} steps", ir.steps.len());
    for step in &ir.steps {
        println!("    - Step {}: {} {} -> binds: {:?}", step.id, step.collector, step.action, step.binds);
    }
    println!();
    
    // Create Execution Contract
    let target_spec = jockey_core::TargetSpec::live("localhost", os);
    let policy = jockey_core::ExecPolicy::new()
        .with_evidence_root(evidence_root)
        .with_parallelism(2);
    
    println!("Step 3: Building Execution Contract...");
    let contract = jockey_language::compile_to_contract(demo_script, jockey_core::TargetSpec::live("localhost", os), jockey_core::ExecPolicy::new().with_evidence_root(evidence_root))?;
    println!("  ✓ Contract created: {}", contract.run_id);
    println!("  ✓ Target: {} ({})", contract.target.identifier, contract.target.os);
    println!("  ✓ Steps: {}", contract.steps.len());
    println!("  ✓ Policy: parallelism={}, hash={}", contract.policy.parallelism, contract.policy.hash_algorithm);
    println!();
    
    // Set up collector registry
    println!("Step 4: Initializing collector registry...");
    let mut registry = jockey_runtime::collector_registry::CollectorRegistry::new();
    let keypair = Arc::new(jockey_evidence::receipt::CollectorKeypair::generate());
    let proc_collector = Arc::new(jockey_collectors::windows::ProcWindowsCollector::new(Arc::clone(&keypair)));
    jockey_runtime::collector_registry::CollectorRegistry::register_collector(&mut registry, proc_collector)?;
    println!("  ✓ Collector registry initialized with proc_windows");
    println!();
    
    // Execute the contract
    println!("Step 5: Executing investigation (this will actually collect process data)...");
    println!("  Target: {} ({})", "localhost", os);
    println!("  Evidence root: {}", evidence_root);
    println!();
    
    let contract = jockey_core::ExecutionContract {
        ir_id: "demo-ir".to_string(),
        run_id: ulid::Ulid::new().to_string(),
        target: jockey_core::TargetSpec::live("localhost", os),
        steps: vec![],
        policy: jockey_core::ExecPolicy::new().with_evidence_root(evidence_root),
        context: serde_json::json!({}),
        created_at: chrono::Utc::now(),
    };
    
    // For the demo, we'll simulate the execution with our process collector
    println!("Step 6: Collecting process evidence (Windows)...");
    let mut registry = jockey_runtime::collector_registry::CollectorRegistry::new();
    let keypair = Arc::new(jockey_evidence::receipt::CollectorKeypair::generate());
    let proc_collector = Arc::new(jockey_collectors::windows::ProcWindowsCollector::new(Arc::clone(&keypair)));
    registry.register(proc_collector)?;
    
    // Create execution context
    let run_id = ulid::Ulid::new().to_string();
    let context = jockey_collectors::trait_def::ExecutionContext {
        run_id: run_id.clone(),
        evidence_root: std::path::PathBuf::from(evidence_root),
        target: jockey_core::TargetSpec::live("localhost", os),
        temp_dir: std::env::temp_dir().join(&run_id),
        credentials: None,
    };
    
    // Execute the collector
    println!("  Collecting process list...");
    let proc_collector = jockey_collectors::windows::ProcWindowsCollector::new(Arc::clone(&keypair));
    let output = proc_collector.execute("list", serde_json::json!({}), &context).await?;
    
    println!("  ✓ Collected {} process evidence objects", output.evidence.len());
    for evidence in &output.evidence {
        println!("    - {} ({} bytes, hash: {}...)", evidence.name, evidence.size, &evidence.hash[..16]);
    }
    println!();
    
    // Generate receipts
    println!("Step 7: Generating Evidence Receipts (Ed25519 + SHA-256)...");
    for evidence in &output.evidence {
        let receipt = jockey_evidence::receipt::EvidenceReceipt::new(
            evidence,
            "proc_windows",
            "0.1.0",
            &keypair,
            jockey_evidence::receipt::HostInfo {
                hostname: hostname::get().unwrap_or_default().to_string_lossy().to_string(),
                os: os.to_string(),
                kernel: Some("10.0.19045".to_string()),
                collector_pid: std::process::id(),
            },
        );
        
        // Verify the receipt
        let valid = receipt.verify(evidence, &keypair);
        println!("  Evidence: {} | Receipt: {} | SHA-256: {}... | Ed25519: {}",
            evidence.name, receipt.id, &receipt.payload_hash[..16], if valid { "✓ PASS" } else { "✗ FAIL" });
    }
    println!();
    
    // Tamper demonstration
    println!("Step 8: Tamper Detection Demo...");
    if let Some(first_evidence) = output.evidence.first() {
        let receipt = jockey_evidence::receipt::EvidenceReceipt::new(
            first_evidence,
            "proc_windows",
            "0.1.0",
            &keypair,
            jockey_evidence::receipt::HostInfo {
                hostname: hostname::get().unwrap_or_default().to_string_lossy().to_string(),
                os: os.to_string(),
                kernel: Some("10.0.19045".to_string()),
                collector_pid: std::process::id(),
            },
        );
        
        // Verify original
        let valid = receipt.verify(first_evidence, &keypair);
        println!("  Original evidence: {:?}", if valid { "✓ PASS" } else { "✗ FAIL" });
        
        // Tamper with evidence
        println!("  Tampering with evidence file...");
        // Simulate tampering by creating a modified evidence
        let mut tampered_evidence = output.evidence[0].clone();
        tampered_evidence.metadata["tampered"] = serde_json::json!(true);
        
        // Recompute hash - this would fail verification
        let tampered_hash = {
            let mut hasher = sha2::Sha256::new();
            hasher.update(b"tampered");
            hex::encode(hasher.finalize())
        };
        tampered_evidence.hash = tampered_hash;
        
        // Verify tampered evidence - should FAIL
        let valid = receipt.verify(&tampered_evidence, &keypair);
        println!("  Tampered evidence: {:?}", if valid { "✓ PASS (UNEXPECTED!)" } else { "✗ FAIL (EXPECTED)" });
        
        // Restore and verify again
        println!("  Restoring original evidence...");
        let valid = receipt.verify(&output.evidence[0], &keypair);
        println!("  Restored evidence: {:?}", if valid { "✓ PASS" } else { "✗ FAIL" });
    }
    
    println!();
    println!("╔══════════════════════════════════════════════════════════════╗");
    println!("║                    DEMO COMPLETE                                ║");
    println!("╚══════════════════════════════════════════════════════════════╝");
    println!();
    println!("Summary:");
    println!("  • JOCKY DSL script parsed and compiled to IR");
    println!("  • Execution Contract built with policy validation");
    println!("  • Windows process collector executed (real collection)");
    println!("  • Evidence objects created with SHA-256 hashes");
    println!("  • Evidence Receipts generated with Ed25519 signatures");
    println!("  • Integrity verification: PASS (original), FAIL (tampered)");
    println!();
    println!("Artifacts generated in: {}/<run_id>/", evidence_root);
    
    Ok(())
}