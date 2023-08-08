
fn main() {
    println!("hello world!");
    let timestamp = std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH).unwrap().as_secs();
    // std::env::set_var("TEST_FOO", timestamp.to_string())
    println!("cargo:rustc-env=TEST_FOO={}", timestamp.to_string())
}