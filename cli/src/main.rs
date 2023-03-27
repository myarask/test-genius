use std::io::Write;
use std::env;


fn main() {
    let args: Vec<String> = env::args().collect();
    let app_code_path = &args[1];
    // TODO: more reliable extention modification
    let test_code_path = &app_code_path.replace(".tsx", ".test.tsx").to_string();    

    let import_statement_regex = regex::Regex::new(r#"import .* from .*"#).unwrap();

    println!("{}", app_code);

    // Create a new test file
    let mut file = std::fs::File::create(test_code_path).expect("create failed");
    file.write_all("Hello World!!!".as_bytes()).expect("write failed");
    // file.write_all("\nTutorialsPoint".as_bytes()).expect("write failed");
    // file.write_all(("\n".to_owned() + test_code_path).as_bytes()).expect("write failed");
}
    