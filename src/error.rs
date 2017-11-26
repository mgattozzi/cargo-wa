//! Dealing with errors in the binary

#[derive(Debug, Fail)]
pub enum CargoWasmError {
    #[fail(display = "Failed to run cargo. Exit code was: {}", exit)]
    CargoFail {
        exit: String,
    },
    #[fail(display = "Failed to run rustup. Exit code was: {}", exit)]
    RustupFail {
        exit: String,
    },
    #[fail(display = "Failed to run wasm-gc. Exit code was: {}", exit)]
    WasmGcFail {
        exit: String,
    },
    #[fail(display = "Unable to find a compiled wasm file to run")]
    NoWasmCompiled,
    #[fail(display = "Failed to open browser to run project. Methods used were: {}", methods)]
    BrowserOpenFail {
        methods: String,
    }
}
