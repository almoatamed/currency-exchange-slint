// In order to be compatible with both desktop, wasm, and android, the example is both a binary and a library.
// Just forward to the library in main
#[cfg(not(any(target_os = "android", target_os = "ios")))]
fn main() {
    saraf::main();
}

#[cfg(target_os = "android")]
fn main() {
}
