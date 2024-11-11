use std::path::Path;
use eyre::{Context, ContextCompat, Result};
use dotenv;
use tracing_subscriber;

/// Loads environment variables from the root `.env` file.
///
/// This function determines the parent directory of the current crate's manifest directory,
/// constructs the path to the `.env` file located there, and loads the environment variables
/// using the `dotenv` crate.
///
/// # Errors
///
/// Returns an error if:
/// - The parent directory of `CARGO_MANIFEST_DIR` cannot be determined.
/// - Loading the `.env` file fails.
pub fn load_environment() -> Result<()> {
    // Determine the path to the root `.env` file
    let env_path = Path::new(env!("CARGO_MANIFEST_DIR"))
        .parent()
        .context("Failed to determine the parent directory of CARGO_MANIFEST_DIR")?
        .join(".env");

    // Load environment variables from the `.env` file
    dotenv::from_path(&env_path)
        .with_context(|| format!("Failed to load environment variables from {:?}", env_path))?;

    Ok(())
}

/// Sets up logging using the `tracing` crate.
///
/// This function initializes the global default subscriber with a formatting layer
/// provided by `tracing_subscriber`. It configures the logging format and level.
///
/// # Panics
///
/// This function does not return a `Result` because `tracing_subscriber::fmt::init()`
/// is expected to succeed. If initialization fails, it will panic.
pub fn setup_logging() {
    // Initialize the tracing subscriber for logging
    tracing_subscriber::fmt::init();
}
