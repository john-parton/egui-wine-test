use baseview::{Size, WindowOpenOptions, WindowScalePolicy};
use egui_baseview::{egui, EguiWindow, GraphicsConfig};

fn main() {
    let window_settings = WindowOpenOptions {
        title: String::from("Hello egui"),
        size: Size::new(320.0, 240.0),
        scale: WindowScalePolicy::SystemScaleFactor,

        #[cfg(feature = "opengl")]
        gl_config: Some(get_gl_config()),
    };

    let graphics_config = GraphicsConfig::default();

    EguiWindow::open_blocking(
        window_settings,
        graphics_config,
        MyApp::default(),
        |_ctx, _queue, _state| {
            // Build function - called once at startup
        },
        |ctx, _queue, state| {
            // Update function - called every frame
            state.ui(ctx);
        },
    );
}

/// Create an OpenGL configuration suitable for Wine compatibility.
/// This can be enabled by setting the environment variable WINE_COMPAT=1
#[cfg(feature = "opengl")]
fn get_gl_config() -> baseview::gl::GlConfig {
    // Check for Wine compatibility mode
    let wine_compat = std::env::var("WINE_COMPAT")
        .map(|v| v == "1" || v.eq_ignore_ascii_case("true"))
        .unwrap_or(false);

    if wine_compat {
        eprintln!(
            "[egui-wine-test] Wine compatibility mode enabled - using conservative OpenGL settings"
        );
        eprintln!("[egui-wine-test] GL version: 3.0, sRGB: disabled, vsync: disabled");
        // Conservative settings for Wine compatibility
        baseview::gl::GlConfig {
            version: (3, 0), // Lower OpenGL version requirement
            profile: baseview::gl::Profile::Core,
            red_bits: 8,
            blue_bits: 8,
            green_bits: 8,
            alpha_bits: 8,
            depth_bits: 16,  // Reduced depth buffer
            stencil_bits: 0, // No stencil buffer
            samples: None,   // No MSAA
            srgb: false,     // sRGB can be problematic in Wine
            double_buffer: true,
            vsync: false, // vsync can cause issues under Wine
            ..Default::default()
        }
    } else {
        eprintln!("[egui-wine-test] Using default OpenGL settings");
        eprintln!("[egui-wine-test] GL version: 3.2, sRGB: enabled, vsync: enabled");
        // Default configuration
        baseview::gl::GlConfig {
            version: (3, 2),
            profile: baseview::gl::Profile::Core,
            red_bits: 8,
            blue_bits: 8,
            green_bits: 8,
            alpha_bits: 8,
            depth_bits: 24,
            stencil_bits: 8,
            samples: None,
            srgb: true, // Enable sRGB by default
            double_buffer: true,
            vsync: true,
            ..Default::default()
        }
    }
}

struct MyApp {
    name: String,
    age: u32,
}

impl Default for MyApp {
    fn default() -> Self {
        Self {
            name: "World".to_owned(),
            age: 0,
        }
    }
}

impl MyApp {
    fn ui(&mut self, ctx: &egui::Context) {
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.heading("Hello egui!");

            ui.horizontal(|ui| {
                ui.label("Your name: ");
                ui.text_edit_singleline(&mut self.name);
            });

            ui.add(egui::Slider::new(&mut self.age, 0..=120).text("age"));

            if ui.button("Click me").clicked() {
                println!("Button clicked!");
            }

            ui.label(format!("Hello '{}', age {}", self.name, self.age));
        });
    }
}
