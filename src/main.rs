use dioxus::prelude::*;
use dioxus::desktop::tao::window::WindowBuilder;
use rusqlite::Connection;
use std::path::{PathBuf};
use directories::ProjectDirs;
use std::sync::mpsc::channel;

// Android-specific imports (only compiled on Android targets)
#[cfg(target_os = "android")]
use dioxus::mobile::wry::prelude::dispatch;
#[cfg(target_os = "android")]
use jni::objects::{JObject, JString};
#[cfg(target_os = "android")]
use jni::JNIEnv;
#[cfg(target_os = "android")]
use anyhow::Result as AnyResult;

const MAIN_CSS: Asset = asset!("/assets/main.css");

fn main() {
    LaunchBuilder::new()
        .with_cfg(
            dioxus::desktop::Config::new().with_window(
                WindowBuilder::new()
                    .with_title("Todo")
                    .with_inner_size(dioxus::desktop::tao::dpi::LogicalSize::new(1200, 800))
                    .with_resizable(true),
            ),
        )
        .launch(App);
}

#[component]
fn App() -> Element {
    let db_path = get_db_path();
    let db_path_str = db_path.to_string_lossy().to_string();

    let conn = Connection::open(&db_path_str).expect("Failed to open SQLite database");
    let con = use_signal(|| Connection::open(&db_path_str).unwrap());
    // Ensure the table exists
    conn.execute(
        "CREATE TABLE IF NOT EXISTS todo (
            id INTEGER PRIMARY KEY,
            name TEXT NOT NULL
        )",
        (),
    )
    .expect("Failed to create table");

    // A signal to store a vector of TodoItems
    let mut todos = use_signal(
        Vec::<TodoItem>::new
    );

    // Runs after the component has rendered.
    use_effect(move || {
        let writer = con.read();

        // Get the todo items from the database and add it to the todo signal

        let mut stm = writer.prepare("SELECT id, name FROM todo").unwrap();

        let rows = stm.query_map((), |row| {
            Ok(TodoItem {
                id : row.get(0).unwrap(),
                name    : row.get(1).unwrap(),
                is_checked : false
            })
        }).unwrap();

        // clear the todo vector when the use_effect runs again.
        // This is stop duplicated items from appearing.
        todos.write().clear();

        for row in rows {
            let item = row.unwrap();
            todos.write().push(item);
        }
    });
   
    rsx! {
        document::Link { rel: "stylesheet", href: MAIN_CSS }
        Main { con : con, todos }
    }
}

#[component]
pub fn Main(con : Signal<Connection>, todos : Signal<Vec<TodoItem>>) -> Element {

    // A signal to store the todo item being entred.
    let mut item = use_signal(|| {
        String::new()
    });

    // Add a todo item to the database
    let mut add_item = move |item : String| {
        con.write().execute(
            "INSERT INTO todo (name) VALUES(?1)", 
            [&item]).unwrap();
    };

    // Delete a todo item from the database
    // This closure is used as a callback function.
    let update = move |item: TodoItem| {
        con.write().execute(
            "DELETE FROM todo WHERE id = ?1", 
            [&item.id]).unwrap();

    };

    rsx! {
        div {
            div {
                class : "header",
                input {
                    id : "name",
                    type : "text",
                    class : "input",
                    value : item,
                    placeholder : "Enter a todo",
                    oninput : move |e| {
                        item.set(e.value());
                    },
                    onkeydown : move |event| {
                        if event.code().to_string() == "Enter".to_string()  {
                            add_item(item.to_string());
                            item.set("".to_string());
                        }
                    }
                }
            },
            div {
                div {
                    for item in todos.read().iter() {
                        TodoElement {item : item.clone(), callback : update }
                    }
                }
            }
        }
    }
}

#[component]
fn TodoElement(item : TodoItem, callback : Callback<TodoItem>) -> Element{
    rsx!(
        div {
            class : "todo-item",
            label { {item.name.to_string()} },
            button {
                class : "delete-button",
                onclick : move |_| {
                    callback(item.clone())
                },
                "X"
            }
        }
    )
}

pub fn get_db_path() -> PathBuf {
    // Desktop platforms: Linux, macOS, Windows
    #[cfg(not(any(target_os = "android", target_os = "ios", target_arch = "wasm32")))]
    {
        if let Some(proj_dirs) = ProjectDirs::from("com", "showen", "TodoApp") {
            let data_dir = proj_dirs.data_dir();
            let _ = std::fs::create_dir_all(data_dir); // Ensure it exists
            return data_dir.join("data.db3");
        }
        // Fallback if directories fails
        PathBuf::from("data.db3")
    }

    // Android: Private internal files dir via JNI
    #[cfg(target_os = "android")]
    {
        let (tx, rx) = channel::<anyhow::Result<PathBuf>>();

        dispatch(move |env: &mut JNIEnv, activity: &JObject, _webview| {
            let result = (|| -> anyhow::Result<PathBuf> {
                let files_dir = env
                    .call_method(activity, "getFilesDir", "()Ljava/io/File;", &[])?
                    .l()?;

                let path_jstring: JString = env
                    .call_method(files_dir, "getAbsolutePath", "()Ljava/lang/String;", &[])?
                    .l()?
                    .into();

                let path: String = env.get_string(&path_jstring)?.into();

                Ok(PathBuf::from(path))
            })();

            let _ = tx.send(result);
        });

        let base_dir = rx.recv().expect("Channel closed").expect("Failed to get Android files dir");
        base_dir.join("data.db3")
       
    }

    // iOS: App Documents directory (sandboxed, writable)
    #[cfg(target_os = "ios")]
    {
        let home = std::env::var("HOME").unwrap_or_else(|_| ".".to_string());
        let mut path = PathBuf::from(home);
        path.push("Documents");
        let _ = std::fs::create_dir_all(&path);
        path.push("data.db3");
        path
    }

    // Ultimate fallback (e.g., web/WASM – no persistent storage)
    #[cfg(not(any(
        not(any(target_os = "android", target_os = "ios", target_arch = "wasm32")),
        target_os = "android",
        target_os = "ios"
    )))]
    PathBuf::from("data.db3")
}

#[derive(PartialEq, Eq, Clone, Debug)]
pub struct TodoItem {
    id : u32,
    name : String,
    is_checked : bool
}