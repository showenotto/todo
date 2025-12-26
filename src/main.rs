mod app;
mod platform;
use dioxus::prelude::*;
use dioxus::desktop::tao::window::WindowBuilder;
use crate::platform::get_db_path;
use app::App;

fn main() {
    let mut builder = LaunchBuilder::new();

    #[cfg(feature = "desktop")]
    {
        use dioxus::desktop::tao::dpi::LogicalSize;
        use dioxus::desktop::tao::window::WindowBuilder;

        let mut desktop_cfg = dioxus::desktop::Config::new()
            .with_window(
                WindowBuilder::new()
                    .with_title("Todo")
                    .with_inner_size(LogicalSize::new(1200.0, 800.0))
                    .with_resizable(true),
            );
        #[cfg(target_os = "windows")]
        {
            let data_dir = get_db_path().parent().unwrap().to_path_buf();  // Reuse your logic: ~\AppData\Local\com.showen.TodoApp
            desktop_cfg = desktop_cfg.with_data_directory(data_dir);
        }

        builder = builder.with_cfg(desktop_cfg);
        builder.launch(App);
    }

    #[cfg(feature = "web")]
    dioxus::web::launch(App);

    #[cfg(feature = "mobile")]
    dioxus::launch(App);
}