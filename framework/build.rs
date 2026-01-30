use std::env;
use std::path::Path;

fn main() {
    // Load .env and .env.local if they exist
    // We check both the current directory and the parent directory (workspace root)
    let possible_paths = [
        Path::new(".env"),
        Path::new(".env.local"),
        Path::new("../.env"),
        Path::new("../.env.local"),
        Path::new("../../.env"),
        Path::new("../../.env.local"),
    ];

    let mut found = false;
    for path in possible_paths {
        if path.exists() {
            found = true;
            println!("cargo:warning=Loading environment from: {}", path.display());
            if let Ok(iter) = dotenvy::from_path_iter(path) {
                for item in iter {
                    if let Ok((key, value)) = item {
                        // Inform cargo about the environment variable
                        // This makes option_env! work at compile time
                        println!("cargo:rustc-env={}={}", key, value);
                    }
                }
            }
            // Tell cargo to rerun if the env file changes
            println!("cargo:rerun-if-changed={}", path.display());
        }
    }

    if !found {
        println!("cargo:warning=No .env or .env.local files found in possible paths.");
    }

    // Also handle regular environment variables if they are already set in the shell
    if let Ok(val) = env::var("OPENROUTER_API_KEY") {
        println!("cargo:rustc-env=OPENROUTER_API_KEY={}", val);
    }
}
