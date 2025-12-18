# Building Zoom Auto-Restarter from Source

Follow these instructions if you want to compile the executable yourself instead of using the pre-built binaries.

## Prerequisites

You must have the Rust toolchain installed on your Windows machine:

1. Download `rustup-init.exe` from [rust-lang.org](https://rust-lang.org/tools/install/).
2. During installation, ensure you install the **C++ Build Tools** if prompted by the installer.
3. Restart your terminal and verify by running:
   ```bash
   cargo --version
   ```

## Build Steps

1. Clone the repository:

```bash
git clone [https://github.com/Binukape/zoom-auto-restarter.git](https://github.com/Binukape/zoom-auto-restarter.git)
cd zoom-auto-restarter
```

2. Compile the Release version:

```bash
cargo build --release
```

3. Locate your Executable: Once the build finishes, your standalone .exe will be waiting for you here: target/release/zoom-auto-restarter.exe
