#![cfg_attr(feature = "axstd", no_std)]
#![cfg_attr(feature = "axstd", no_main)]

#[cfg(feature = "axstd")]
use axstd::println;
use axstd::string::String;
#[cfg_attr(feature = "axstd", no_mangle)]
fn main() {
    let prompt = String::from("Greet from ArceOS");
    println!("Hello, world: {}", prompt);
}
