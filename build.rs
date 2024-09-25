use clap_complete::generate_to;
use clap_complete::shells::Bash;
use std::env;
use std::fs;
use std::path::Path;

include!("src/cli.rs");
fn main() {
    // Define your Clap app here or import it
    let mut app = app();

    // Get the output directory where Cargo stores its build artifacts
    let outdir = env::var_os("OUT_DIR").unwrap();

    // Create the completion directory
    let completion_dir = Path::new(&outdir).join("completions");
    fs::create_dir_all(&completion_dir).unwrap();

    // Generate the completion file
    generate_to(Bash, &mut app, "hushcrumbs", &completion_dir)
        .expect("Failed to generate shell completions");
}
