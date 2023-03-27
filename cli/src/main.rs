// use std::io::Write;
// use std::env;
use clap::Parser;
use anyhow::{Context, Result};

/// Search for a pattern in a file and display the lines that contain it.
#[derive(Parser)]
struct Cli {
    /// The path to the file to read
    // path: std::path::PathBuf,
    path: String,
}

fn main() -> Result<()> {
    // Parse arguments
    let args = Cli::parse();
    let path = &args.path;

    // Read the file contents
    let content = std::fs::read_to_string(path)
        .with_context(|| format!("could not read file `{}`", path))?;

    // Print the file contents
    println!("file content: {}", content); 
    Ok(())
    // let content = std::fs::read_to_string(&args.path).expect("could not read file");

    // for line in content.lines() {
    //     if line.contains(&args.pattern) {
    //         println!("{}", line);
    //     }
    // }

    // let app_code_path = &args[1];
    // TODO: more reliable extention modification
    // let test_code_path = &app_code_path.replace(".tsx", ".test.tsx").to_string();    

    // let import_statement_regex = regex::Regex::new(r#"import .* from .*"#).unwrap();

    // println!("{}", app_code);

    // Create a new test file
    // let mut file = std::fs::File::create(test_code_path).expect("create failed");
    // file.write_all("Hello World!!!".as_bytes()).expect("write failed");
    // file.write_all("\nTutorialsPoint".as_bytes()).expect("write failed");
    // file.write_all(("\n".to_owned() + test_code_path).as_bytes()).expect("write failed");
}
    