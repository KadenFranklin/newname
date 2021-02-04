use std::env;
use std::fs;


fn main() -> std::io::Result<()> {
    let args: Vec<String> = env::args().collect();
    let cookie = &args[1];
    let muffin = &args[2];
    fs::rename(cookie, muffin)?;
    println!("{}has been renamed to{}", cookie, muffin);
    Ok(())
}