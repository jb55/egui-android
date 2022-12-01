
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

Add these things to your Cargo.toml:

```toml
[package.metadata.android]
package = "com.myapp"
apk_name = "myapp"

[[package.metadata.android.uses_feature]]
name = "android.hardware.vulkan.level"
required = true
version = 1

[[package.metadata.android.uses_permission]]
name = "android.permission.WRITE_EXTERNAL_STORAGE"
max_sdk_version = 18

[[package.metadata.android.uses_permission]]
name = "android.permission.INTERNET"

[package.metadata.android.application]
label = "MyApp"
```

Install cargo-apk, then build and install:

```bash
$ cargo apk build && adb install target/debug/apk/damus.apk
```

## Credits

Thanks to https://github.com/inferrna/hello_world_android_egui for most of the
code. I've modified it slightly to support running egui apps.
