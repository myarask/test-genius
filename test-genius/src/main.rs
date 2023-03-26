use std::io::Write;


fn main() {
    // Create a new test file
    let mut file = std::fs::File::create("samples/ProfileMenu.test.tsx").expect("create failed");
    file.write_all("Hello World".as_bytes()).expect("write failed");
    file.write_all("\nTutorialsPoint".as_bytes()).expect("write failed");
}
    