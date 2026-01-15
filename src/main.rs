use baseview::{
    Event, EventStatus, Size, Window, WindowHandler, WindowOpenOptions, WindowScalePolicy,
};

fn main() {
    let window_settings = WindowOpenOptions {
        title: String::from("Baseview OpenGL Test"),
        size: Size::new(640.0, 480.0),
        scale: WindowScalePolicy::SystemScaleFactor,

        #[cfg(feature = "opengl")]
        gl_config: Some(get_gl_config()),
    };

    #[cfg(feature = "opengl")]
    {
        if let Some(ref config) = window_settings.gl_config {
            eprintln!("[baseview-wine-test] OpenGL Config:");
            eprintln!("[baseview-wine-test]   version: {:?}", config.version);
            eprintln!("[baseview-wine-test]   profile: {:?}", config.profile);
            eprintln!("[baseview-wine-test]   red_bits: {}", config.red_bits);
            eprintln!("[baseview-wine-test]   blue_bits: {}", config.blue_bits);
            eprintln!("[baseview-wine-test]   green_bits: {}", config.green_bits);
            eprintln!("[baseview-wine-test]   alpha_bits: {}", config.alpha_bits);
            eprintln!("[baseview-wine-test]   depth_bits: {}", config.depth_bits);
            eprintln!(
                "[baseview-wine-test]   stencil_bits: {}",
                config.stencil_bits
            );
            eprintln!("[baseview-wine-test]   samples: {:?}", config.samples);
            eprintln!("[baseview-wine-test]   srgb: {}", config.srgb);
            eprintln!(
                "[baseview-wine-test]   double_buffer: {}",
                config.double_buffer
            );
            eprintln!("[baseview-wine-test]   vsync: {}", config.vsync);
        }
    }

    Window::open_blocking(window_settings, |window| MyWindowHandler::new(window));
}

/// Create an OpenGL configuration suitable for Wine compatibility.
/// This can be enabled by setting the environment variable FORCE_SRGB=1 or FORCE_SRGB=true
#[cfg(feature = "opengl")]
fn get_gl_config() -> baseview::gl::GlConfig {
    let srgb_string = match std::env::var("FORCE_SRGB") {
        Ok(val) => val,
        Err(_) => {
            eprintln!("[baseview-wine-test] FORCE_SRGB not set, using default OpenGL settings");
            return baseview::gl::GlConfig::default();
        }
    };

    let srgb = match srgb_string.to_ascii_lowercase().as_str() {
        "" => {
            eprintln!("[baseview-wine-test] FORCE_SRGB is empty, using default OpenGL settings");
            return baseview::gl::GlConfig::default();
        }
        "1" | "true" => {
            eprintln!("[baseview-wine-test] FORCE_SRGB enabled - using sRGB framebuffer");
            true
        }
        "0" | "false" => {
            eprintln!("[baseview-wine-test] FORCE_SRGB disabled - not using sRGB framebuffer");
            false
        }
        _ => {
            eprintln!(
                "[baseview-wine-test] FORCE_SRGB has invalid value '{}', using default",
                srgb_string
            );
            return baseview::gl::GlConfig::default();
        }
    };

    baseview::gl::GlConfig {
        srgb,
        ..Default::default()
    }
}

struct MyWindowHandler {
    // Store OpenGL state if needed
}

impl MyWindowHandler {
    fn new(window: &mut Window) -> Self {
        // Load OpenGL function pointers
        #[cfg(feature = "opengl")]
        {
            eprintln!("[baseview-wine-test] Attempting to get GL context...");

            match window.gl_context() {
                Some(gl_context) => {
                    eprintln!("[baseview-wine-test] GL context obtained, making it current...");

                    // Make the context current before loading GL functions
                    unsafe {
                        gl_context.make_current();
                    }

                    eprintln!("[baseview-wine-test] Context made current, loading GL functions...");
                    gl::load_with(|symbol| gl_context.get_proc_address(symbol));

                    // Check for GL errors
                    let err = unsafe { gl::GetError() };
                    if err != gl::NO_ERROR {
                        eprintln!(
                            "[baseview-wine-test] WARNING: GL error after loading: 0x{:X}",
                            err
                        );
                    }

                    // Print OpenGL version for debugging
                    let version = unsafe { gl::GetString(gl::VERSION) };
                    if !version.is_null() {
                        let version_str = unsafe { std::ffi::CStr::from_ptr(version as *const i8) };
                        eprintln!("[baseview-wine-test] OpenGL Version: {:?}", version_str);
                    } else {
                        eprintln!("[baseview-wine-test] WARNING: OpenGL VERSION string is null");
                    }

                    // Print OpenGL renderer
                    let renderer = unsafe { gl::GetString(gl::RENDERER) };
                    if !renderer.is_null() {
                        let renderer_str =
                            unsafe { std::ffi::CStr::from_ptr(renderer as *const i8) };
                        eprintln!("[baseview-wine-test] OpenGL Renderer: {:?}", renderer_str);
                    } else {
                        eprintln!("[baseview-wine-test] WARNING: OpenGL RENDERER string is null");
                    }

                    // Print OpenGL vendor
                    let vendor = unsafe { gl::GetString(gl::VENDOR) };
                    if !vendor.is_null() {
                        let vendor_str = unsafe { std::ffi::CStr::from_ptr(vendor as *const i8) };
                        eprintln!("[baseview-wine-test] OpenGL Vendor: {:?}", vendor_str);
                    }
                }
                None => {
                    eprintln!("[baseview-wine-test] ERROR: Failed to get GL context!");
                }
            }
        }

        Self {}
    }
}

impl WindowHandler for MyWindowHandler {
    fn on_frame(&mut self, _window: &mut Window) {
        // No rendering - just keeping the window open
    }

    fn on_event(&mut self, _window: &mut Window, event: Event) -> EventStatus {
        match event {
            Event::Window(window_event) => {
                eprintln!("[baseview-wine-test] Window event: {:?}", window_event);
            }
            Event::Mouse(mouse_event) => {
                eprintln!("[baseview-wine-test] Mouse event: {:?}", mouse_event);
            }
            Event::Keyboard(keyboard_event) => {
                eprintln!("[baseview-wine-test] Keyboard event: {:?}", keyboard_event);
            }
        }
        EventStatus::Ignored
    }
}
