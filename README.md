# egui Wine Test

A minimal egui "Hello World" application that is cross-compileable to Windows with Wine compatibility.

## Features

This application demonstrates:
- Basic egui window with a greeting
- Text input field for entering a name
- A slider for age selection
- A clickable button
- Dynamic text display
- **Wine compatibility via OpenGL environment variables to disable sRGB**

## Building on Linux

### Prerequisites

```bash
cargo build
```

### Run on Linux

```bash
cargo run --release
```

## Cross-Compiling to Windows

### Prerequisites

Install the MinGW-w64 toolchain and Rust Windows target:

```bash
# On Ubuntu/Debian
sudo apt-get install mingw-w64

# Add Rust Windows target
rustup target add x86_64-pc-windows-gnu
```

### Build for Windows

```bash
cargo build --target x86_64-pc-windows-gnu --release
```

The Windows executable will be located at:
```
target/x86_64-pc-windows-gnu/release/egui-wine-test.exe
```

### Testing with Wine

You can test the Windows build on Linux using Wine:

```bash
# Install Wine (on Ubuntu/Debian)
sudo apt-get install wine64

# Run the Windows executable
wine target/x86_64-pc-windows-gnu/release/egui-wine-test.exe
```

## Wine Compatibility Notes

The application sets OpenGL environment variables at runtime to improve Wine compatibility:
- `MESA_GL_VERSION_OVERRIDE=3.3` - Ensures consistent OpenGL version
- `__GL_ALLOW_UNOFFICIAL_PROTOCOL=1` - Allows unofficial protocol extensions

These settings help avoid sRGB framebuffer issues that are common when running OpenGL applications under Wine.

## Project Structure

- `src/main.rs` - Main application code with egui implementation
- `Cargo.toml` - Project dependencies and metadata
- `.cargo/config.toml` - Cross-compilation configuration

## Dependencies

- `eframe` - Framework for building egui applications (uses OpenGL/glow backend)
- `egui` - Immediate mode GUI library

## Notes

The application uses eframe 0.30 which handles:
- Window creation and management via winit
- Event loop
- OpenGL rendering backend (glow)
- Cross-platform compatibility
