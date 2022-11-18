fn main() {
    println!("cargo:rustc-link-arg=-L/opt/homebrew/lib");
    println!("cargo:rustc-link-arg=-Wl,-framework,Cocoa");
    println!("cargo:rustc-link-arg=-lSDLmain");
}
