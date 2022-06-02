fn main() {
    if cfg!(dev) {
        // Tell cargo to tell rustc to link the lcms2 library from tests/include.
        println!("cargo:rustc-link-search=native=./tests/include");
        println!("cargo:rustc-link-lib=static=lcms2");
    }
}
