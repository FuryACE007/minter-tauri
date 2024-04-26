fn main() {
    println!("cargo:rustc-link-search=native=./");
    println!("cargo:rustc-link-lib=dylib=hello");
    tauri_build::build()
}
