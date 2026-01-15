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
/// This can be enabled by setting the environment variable FORCE_SRGB=1 or FORCE_SRGB=true
#[cfg(feature = "opengl")]
fn get_gl_config() -> baseview::gl::GlConfig {
    let srgb_string = match std::env::var("FORCE_SRGB") {
        Ok(val) => val,
        Err(_) => {
            eprintln!("[egui-wine-test] FORCE_SRGB not set, using default OpenGL settings");
            return baseview::gl::GlConfig::default();
        }
    };

    let srgb = match srgb_string.to_ascii_lowercase().as_str() {
        "" => {
            eprintln!("[egui-wine-test] FORCE_SRGB is empty, using default OpenGL settings");
            return baseview::gl::GlConfig::default();
        }
        "1" | "true" => {
            eprintln!("[egui-wine-test] FORCE_SRGB enabled - using sRGB framebuffer");
            true
        }
        "0" | "false" => {
            eprintln!("[egui-wine-test] FORCE_SRGB disabled - not using sRGB framebuffer");
            false
        }
        _ => {
            eprintln!(
                "[egui-wine-test] FORCE_SRGB has invalid value '{}', using default",
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
