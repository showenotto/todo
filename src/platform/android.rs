use std::path::PathBuf;
use std::sync::mpsc::channel;
use dioxus::mobile::wry::prelude::dispatch;
use jni::objects::{JObject, JString};
use jni::JNIEnv;
use anyhow::Result as AnyResult;

pub fn get_db_path() -> PathBuf {
    let (tx, rx) = channel::<AnyResult<PathBuf>>();

    dispatch(move |env: &mut JNIEnv, activity: &JObject, _webview| {
        let result = (|| -> AnyResult<PathBuf> {
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