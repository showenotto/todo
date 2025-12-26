use std::path::PathBuf;
use directories::ProjectDirs;
use std::fs;
use std::env;

pub fn get_db_path() -> PathBuf {
    #[cfg(not(target_os = "windows"))]
    {
        if let Some(proj_dirs) = ProjectDirs::from("com", "showen", "TodoApp") {
            let data_dir = proj_dirs.data_dir();
            let _ = std::fs::create_dir_all(data_dir); // Ensure it exists
            return data_dir.join("data.db3");
        }
        // Fallback if directories fails
        PathBuf::from("data.db3")
    }
    
    #[cfg(target_os = "windows")]
    {
        // Use %LOCALAPPDATA% as the base (writable without admin)
        if let Ok(app_data) = env::var("LOCALAPPDATA") {
            // Optionally append your app's subfolder for organization
            let data_dir = PathBuf::from(app_data).join("com.showen.TodoApp");
            let _ = fs::create_dir_all(&data_dir);
            return data_dir.join("data.db3");
        }
        // Fallback if directories fails
        PathBuf::from("data.db3")
    }
}