use std::path::PathBuf;

#[cfg(target_os = "android")]
pub mod android;

#[cfg(target_os = "ios")]
pub mod ios;

#[cfg(not(any(target_os = "android", target_os = "ios", target_arch = "wasm32")))]
pub mod desktop;

pub fn get_db_path() -> PathBuf {
    #[cfg(target_os = "android")]
    return android::get_db_path();

    #[cfg(target_os = "ios")]
    return ios::get_db_path();

    #[cfg(not(any(target_os = "android", target_os = "ios", target_arch = "wasm32")))]
    return desktop::get_db_path();

    #[cfg(target_arch = "wasm32")]
    PathBuf::from("data.db3")
}