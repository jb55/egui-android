
# hacky egui android runner

until eframe upstream gets android support

## Usage

```rust
use egui_android::run_android;
use egui_android::SimpleApp;

impl SimpleApp for MyApp {
    fn update_simple(&mut self, ctx: &Context) {
        self.mobile_ui(ctx)
    }
}

// Make sure this is in your lib (not main) so that it gets compiled into the dynamic lib for android to pick it up
#[cfg(target_os = "android")]
#[no_mangle]
pub fn android_main(app: winit::platform::android::activity::AndroidApp) {
    std::env::set_var("RUST_BACKTRACE", "full");
    android_logger::init_once(android_logger::Config::default().with_min_level(log::Level::Trace));

    run_android(app, Box::new(MyApp::new()));
}
```

## Credits

Thanks to https://github.com/inferrna/hello_world_android_egui for most of the
code. I've modified it slightly to support running egui apps.
