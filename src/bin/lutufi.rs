use std::path::PathBuf;
use clap::{Parser, Subcommand};
use lutufi_core::core::{
    error::{LutufiError, LutufiResult},
    io::{lmf::LmfDocument, convert::ConversionService},
};

#[derive(Parser)]
#[command(name = "lutufi", version, about = "Probabilistic inference over social and economic networks")]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Verify that a model file reproduces its stored results
    Verify {
        /// Path to the LMF model file
        file: PathBuf,
    },
    /// Convert between model file formats
    Convert {
        /// Input file path
        input: PathBuf,
        /// Output file path (format determined by extension)
        output: PathBuf,
    },
    /// Show information about a model file
    Info {
        /// Path to the model file
        file: PathBuf,
    },
    /// Query a model (requires LMF file with model data)
    Query {
        /// Path to the LMF model file
        file: PathBuf,
        /// Variables to query (comma-separated)
        #[arg(long, short)]
        variable: String,
    },
}

fn main() -> LutufiResult<()> {
    let cli = Cli::parse();

    match cli.command {
        Commands::Verify { file } => {
            let doc = LmfDocument::load(&file)?;
            let network = doc.to_bayesian_network()?;
            let report = doc.verify(&network)?;

            if report.passed {
                println!("✓ Verification passed for: {}", file.display());
            } else {
                println!("✗ Verification FAILED for: {}", file.display());
            }

            for check in &report.checks {
                let icon = if check.passed { "✓" } else { "✗" };
                println!("  {} {}: {}", icon, check.name, check.detail);
            }

            if !report.passed {
                std::process::exit(1);
            }
            Ok(())
        }
        Commands::Convert { input, output } => {
            println!("Converting {} -> {}", input.display(), output.display());
            ConversionService::convert(&input, &output)?;
            println!("✓ Conversion complete");
            Ok(())
        }
        Commands::Info { file } => {
            let fmt = lutufi_core::core::io::FileFormat::from_extension(&file)
                .ok_or_else(|| LutufiError::DeserializationError {
                    reason: format!("Unknown format for: {}", file.display()),
                })?;

            let network = match fmt {
                lutufi_core::core::io::FileFormat::LMF => {
                    let doc = LmfDocument::load(&file)?;
                    println!("Format version: {}", doc.format_version);
                    println!(
                        "Lutufi version: {}",
                        doc.metadata.lutufi_version
                    );
                    println!("Created: {}", doc.metadata.created_at);
                    println!("Model type: {:?}", doc.model_type);
                    if let Some(ref author) = doc.metadata.author {
                        println!("Author: {}", author);
                    }
                    if let Some(ref desc) = doc.metadata.description {
                        println!("Description: {}", desc);
                    }
                    if doc.evidence.is_some() {
                        println!("Evidence: present");
                    }
                    if doc.inference_settings.is_some() {
                        println!("Inference settings: present");
                    }
                    if doc.results.is_some() {
                        println!("Results: present");
                    }
                    doc.to_bayesian_network()?
                }
                lutufi_core::core::io::FileFormat::BIF => {
                    lutufi_core::core::io::BifFormat::import_from_file(&file)?
                }
                lutufi_core::core::io::FileFormat::XMLBIF => {
                    lutufi_core::core::io::XmlBifFormat::import_from_file(&file)?
                }
                lutufi_core::core::io::FileFormat::UAI => {
                    lutufi_core::core::io::UaiFormat::import_from_file(&file)?
                }
                _ => {
                    return Err(LutufiError::DeserializationError {
                        reason: format!("Unsupported format for info: {:?}", fmt),
                    });
                }
            };

            println!("\nNetwork: {} variables, {} edges",
                network.nodes().len(),
                network.edges().len()
            );

            let valid = network.is_valid();
            println!("Valid: {}", valid);

            if !valid {
                for err in network.validate() {
                    println!("  Warning: {}", err);
                }
            }

            Ok(())
        }
        Commands::Query { file, variable } => {
            let doc = LmfDocument::load(&file)?;
            let network = doc.to_bayesian_network()?;

            let vars: Vec<&str> = variable.split(',').map(|s| s.trim()).collect();

            let evidence = match doc.evidence.as_ref() {
                Some(ev) => {
                    let mut assign = lutufi_core::core::assignment::Assignment::new();
                    for (var, val) in &ev.assignments {
                        let vid = network.id_of(var)?;
                        assign.set(vid, val.as_str());
                    }
                    assign
                }
                None => lutufi_core::core::assignment::Assignment::new(),
            };

            let algorithm = doc
                .inference_settings
                .as_ref()
                .map(|s| match s.algorithm.to_lowercase().as_str() {
                    "variableelimination" | "variable_elimination" => {
                        lutufi_core::core::inference::Algorithm::VariableElimination
                    }
                    "exact" | "junctiontree" | "junction_tree" => {
                        lutufi_core::core::inference::Algorithm::Exact
                    }
                    "lbp" | "loopybeliefpropagation" | "loopy_belief_propagation" => {
                        lutufi_core::core::inference::Algorithm::LBP
                    }
                    "mcmc" | "gibbs" | "gibbsampling" | "gibbs_sampling" => {
                        lutufi_core::core::inference::Algorithm::MCMC
                    }
                    "variational" | "meanfield" | "mean_field" => {
                        lutufi_core::core::inference::Algorithm::Variational
                    }
                    _ => lutufi_core::core::inference::Algorithm::Auto,
                })
                .unwrap_or(lutufi_core::core::inference::Algorithm::Auto);

            let result = lutufi_core::core::inference::InferenceEngine::query(
                &network,
                &vars,
                &evidence,
                algorithm,
            )?;

            println!("Query results (using {}):", result.algorithm_used.name());
            for var in &vars {
                if let Some(factor) = result.distributions.get(*var) {
                    let n = factor.scope().num_entries();
                    for i in 0..n {
                        let prob = factor.value_at(i);
                        println!("  {} = {}: {:.6}", var, i, prob);
                    }
                }
            }
            println!("Log-likelihood: {:.6}", result.log_z);
            println!(
                "Computation time: {:?}",
                result.computation_time
            );

            Ok(())
        }
    }
}
