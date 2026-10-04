//! JOCKY CLI — Command-line interface for JOCKY

use clap::{Parser, Subcommand};
use jockey_language::{compile_to_ir, compile_to_contract};
use jockey_core::{TargetSpec, ExecPolicy};
use std::path::PathBuf;
use std::fs;

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
    }
    
    Ok(())
}