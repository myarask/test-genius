use std::io::Write;
use std::env;


fn main() {
    let args: Vec<String> = env::args().collect();
    let target_path = &args[1];
    
    println!("{}", target_path);
    // Create a new test file
    let mut file = std::fs::File::create("samples/ProfileMenu.test.tsx").expect("create failed");
    file.write_all("Hello World!!!".as_bytes()).expect("write failed");
    file.write_all("\nTutorialsPoint".as_bytes()).expect("write failed");
    file.write_all(("\n".to_owned() + target_path).as_bytes()).expect("write failed");
}
    