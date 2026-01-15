# egui Wine Test

A minimal egui application designed for bisecting and testing OpenGL behavior under Wine.

## Purpose

This project is specifically designed to help bisect Wine behavior related to OpenGL rendering and sRGB framebuffer support. The application provides a simple egui interface with configurable OpenGL settings to isolate rendering issues.

## Features

- Minimal egui interface for testing rendering behavior
- Configurable sRGB framebuffer support via environment variable
- Cross-compileable to Windows for Wine testing
- Debug output showing active OpenGL configuration

## Building on Linux

### Prerequisites

```bash
cargo build
```

### Run on Linux

```bash
cargo run
```

## Testing Native Build

Before bisecting Wine issues, it's recommended to verify the application works correctly on native Linux:

```bash
# Build and run natively
cargo run

# Test with FORCE_SRGB enabled
FORCE_SRGB=1 RUST_BACKTRACE=full cargo run

# Test with FORCE_SRGB disabled
FORCE_SRGB=0 RUST_BACKTRACE=full cargo run
```

The native build should display a simple egui window with:
- A greeting heading
- A text input field (try typing your name)
- An age slider (0-120)
- A clickable button (prints to terminal when clicked)
- Dynamic text showing your inputs

If the native build works correctly, any rendering issues in Wine can be attributed to Wine-specific OpenGL behavior rather than application bugs.

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
cargo build --target x86_64-pc-windows-gnu
```

The Windows executable will be located at:
```
target/x86_64-pc-windows-gnu/debug/egui-wine-test.exe
```

### Testing with Wine

You can test the Windows build on Linux using Wine:

```bash
# Install Wine (on Ubuntu/Debian)
sudo apt-get install wine64

# Run the Windows executable
wine target/x86_64-pc-windows-gnu/debug/egui-wine-test.exe
```

## Wine Compatibility Notes

### FORCE_SRGB Environment Variable

The application supports the `FORCE_SRGB` environment variable to control sRGB framebuffer usage, which is useful for bisecting Wine rendering issues:

**Usage:**

```bash
# Enable sRGB framebuffer
FORCE_SRGB=1 RUST_BACKTRACE=full wine target/x86_64-pc-windows-gnu/debug/egui-wine-test.exe
# or
FORCE_SRGB=true RUST_BACKTRACE=full wine target/x86_64-pc-windows-gnu/debug/egui-wine-test.exe

# Disable sRGB framebuffer
FORCE_SRGB=0 RUST_BACKTRACE=full wine target/x86_64-pc-windows-gnu/debug/egui-wine-test.exe
# or
FORCE_SRGB=false RUST_BACKTRACE=full wine target/x86_64-pc-windows-gnu/debug/egui-wine-test.exe

# Use default OpenGL configuration (not set or empty)
wine target/x86_64-pc-windows-gnu/debug/egui-wine-test.exe
# or
FORCE_SRGB= RUST_BACKTRACE=full wine target/x86_64-pc-windows-gnu/debug/egui-wine-test.exe
```

**Behavior:**

- `FORCE_SRGB=1` or `FORCE_SRGB=true` (case insensitive) - Forces sRGB framebuffer ON
- `FORCE_SRGB=0` or `FORCE_SRGB=false` (case insensitive) - Forces sRGB framebuffer OFF
- Not set or empty string - Uses default baseview OpenGL configuration
- Invalid values - Falls back to default configuration with a warning

The application prints debug messages to stderr showing which configuration is active, making it easy to verify settings during bisecting.

## Project Structure

- `src/main.rs` - Main application code with egui implementation
- `Cargo.toml` - Project dependencies and metadata
- `.cargo/config.toml` - Cross-compilation configuration

## Dependencies

- `baseview` - Low-level windowing library
- `egui-baseview` - egui integration for baseview
- `egui` - Immediate mode GUI library

This project uses baseview instead of eframe for more direct control over OpenGL configuration, which is essential for Wine compatibility testing.
