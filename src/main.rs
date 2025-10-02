// src/main.rs

use engi_math::calculate_matrix_vector_product;
use clap::Parser;

// --- 1. Error Handling (Rust Core Logic) ---

#[derive(thiserror::Error, Debug)]
enum EngiMathError {
    #[error("Invalid input format: {0}")]
    InputError(String),
    #[error("Calculation error: {0}")]
    CalculationFailed(String),
}

type Result<T> = std::result::Result<T, EngiMathError>;

// --- 2. CLI Command Structs (Clap) ---

#[derive(Parser, Debug)]
#[command(author, version, about = "Engineering Mathematics Hybrid Tool (Rust/C++ Compute)", long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Debug, Clone, clap::Subcommand)]
enum Commands {
    /// Solves a matrix-vector product problem (M * V)
    MatrixProduct {
        /// The dimension of the square matrix (e.g., 2 for 2x2)
        #[arg(short, long)]
        dim: usize,

        /// Matrix elements, flattened and comma-separated (e.g., "2,0,0,3")
        matrix: String,

        /// Vector elements, comma-separated (e.g., "4,5")
        vector: String,
    },
    // Future commands (e.g., ODESolve, Integrate, etc.) go here
}

// --- 3. Parsing Logic ---

// Renamed 'dim' to 'expected_len' to explicitly check the size required for the input string.
fn parse_input_vector(s: &str, expected_len: usize) -> Result<Vec<f64>> {
    let result: std::result::Result<Vec<f64>, _> = s
        .split(',')
        .map(|s| s.trim().parse::<f64>())
        .collect();

    match result {
        Ok(vec) => {
            // Check if the actual number of parsed elements matches the expected length.
            if vec.len() != expected_len {
                return Err(EngiMathError::InputError(format!(
                    "Expected {} elements in the input string, but found {}.", 
                    expected_len, vec.len()
                )));
            }
            Ok(vec)
        }
        Err(_) => Err(EngiMathError::InputError(
            "All elements must be valid floating-point numbers and separated by commas.".into(),
        )),
    }
}

// --- 4. Main Application Logic ---

fn main() -> Result<()> {
    let cli = Cli::parse();
    
    match cli.command {
        Commands::MatrixProduct { dim, matrix, vector } => {
            if dim == 0 {
                return Err(EngiMathError::InputError("Dimension (dim) must be greater than zero.".into()));
            }

            // Calculate the expected lengths for validation
            let expected_matrix_len = dim * dim;
            let expected_vector_len = dim;

            // Parse inputs using the explicit expected lengths
            let matrix_flat = parse_input_vector(&matrix, expected_matrix_len)?; 
            let vector_in = parse_input_vector(&vector, expected_vector_len)?; 
            
            // Call the Safe Rust API, which calls the C++ Eigen kernel
            let output = calculate_matrix_vector_product(&matrix_flat, &vector_in, dim)
                .map_err(EngiMathError::CalculationFailed)?;

            // Display result
            println!("\n--- Matrix-Vector Product Result ---");
            println!("Dimension: {dim}x{dim}");
            println!("Input Vector: {:?}", vector_in);
            println!("Output Vector: [ {} ]", output.iter().map(|f| format!("{:.4}", f)).collect::<Vec<_>>().join(", "));
            println!("------------------------------------\n");
        }
    }
    
    Ok(())
}

