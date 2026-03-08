use wasm_bindgen::JsCast;
use wasm_bindgen_futures::JsFuture;
use web_sys::{FileSystemDirectoryHandle, FileSystemFileHandle, FileSystemWritableFileStream, WritableStream};

const OPFS_DIR: &str = "batmonic-annotations";

/// Get the OPFS batmonic-annotations directory, creating it if needed.
async fn get_opfs_dir() -> Result<FileSystemDirectoryHandle, String> {
    let window = web_sys::window().ok_or("no window")?;
    let navigator = window.navigator();
    let storage = navigator.storage();
    let root: FileSystemDirectoryHandle = JsFuture::from(storage.get_directory())
        .await
        .map_err(|e| format!("OPFS root: {e:?}"))?
        .unchecked_into();

    let opts = web_sys::FileSystemGetDirectoryOptions::new();
    opts.set_create(true);
    let dir: FileSystemDirectoryHandle =
        JsFuture::from(root.get_directory_handle_with_options(OPFS_DIR, &opts))
            .await
            .map_err(|e| format!("OPFS dir: {e:?}"))?
            .unchecked_into();
    Ok(dir)
}

/// Build a storage key for a file. Uses spot_hash if available, else filename+size.
pub fn opfs_key(identity: &crate::annotations::FileIdentity) -> String {
    if let Some(ref hash) = identity.spot_hash {
        format!("{}.batm", hash)
    } else {
        // Sanitize filename for filesystem use
        let safe_name: String = identity.filename.chars()
            .map(|c| if c.is_alphanumeric() || c == '.' || c == '-' || c == '_' { c } else { '_' })
            .collect();
        format!("{}_{}.batm", safe_name, identity.file_size)
    }
}

/// Save annotation YAML to OPFS.
pub async fn opfs_save(key: &str, yaml: &str) -> Result<(), String> {
    let dir = get_opfs_dir().await?;

    let opts = web_sys::FileSystemGetFileOptions::new();
    opts.set_create(true);
    let file_handle: FileSystemFileHandle =
        JsFuture::from(dir.get_file_handle_with_options(key, &opts))
            .await
            .map_err(|e| format!("OPFS get file: {e:?}"))?
            .unchecked_into();

    let writable: FileSystemWritableFileStream =
        JsFuture::from(file_handle.create_writable())
            .await
            .map_err(|e| format!("OPFS create writable: {e:?}"))?
            .unchecked_into();

    JsFuture::from(
        writable.write_with_str(yaml).map_err(|e| format!("OPFS write: {e:?}"))?,
    )
    .await
    .map_err(|e| format!("OPFS write await: {e:?}"))?;

    let ws: &WritableStream = writable.unchecked_ref();
    JsFuture::from(ws.close())
        .await
        .map_err(|e| format!("OPFS close: {e:?}"))?;

    Ok(())
}

/// Load annotation YAML from OPFS. Returns None if file doesn't exist.
pub async fn opfs_load(key: &str) -> Result<Option<String>, String> {
    let dir = get_opfs_dir().await?;

    // Try to get file handle without create — returns error if not found
    let file_handle_result = JsFuture::from(dir.get_file_handle(key)).await;
    let file_handle: FileSystemFileHandle = match file_handle_result {
        Ok(h) => h.unchecked_into(),
        Err(_) => return Ok(None), // file doesn't exist
    };

    let file: web_sys::File = JsFuture::from(file_handle.get_file())
        .await
        .map_err(|e| format!("OPFS get file: {e:?}"))?
        .unchecked_into();

    let text = JsFuture::from(file.text())
        .await
        .map_err(|e| format!("OPFS read text: {e:?}"))?;

    Ok(text.as_string())
}

/// Save annotations for a specific file index to OPFS.
pub fn save_annotations_to_opfs(state: crate::state::AppState, file_idx: usize) {
    use leptos::prelude::GetUntracked;

    let store = state.annotation_store.get_untracked();
    let set = match store.sets.get(file_idx).and_then(|s| s.as_ref()) {
        Some(s) => s.clone(),
        None => return,
    };

    let key = opfs_key(&set.file_identity);
    let yaml = match yaml_serde::to_string(&set) {
        Ok(y) => y,
        Err(e) => {
            log::warn!("OPFS serialize error: {e}");
            return;
        }
    };

    wasm_bindgen_futures::spawn_local(async move {
        if let Err(e) = opfs_save(&key, &yaml).await {
            log::warn!("OPFS save error: {e}");
        } else {
            log::debug!("OPFS saved annotations: {key}");
        }
    });
}

/// Try to load annotations from OPFS for a file with the given identity.
/// If found, merges into the annotation store at the given index.
pub fn load_annotations_from_opfs(state: crate::state::AppState, file_idx: usize, identity: crate::annotations::FileIdentity) {
    use leptos::prelude::{Update, GetUntracked};

    let key = opfs_key(&identity);

    wasm_bindgen_futures::spawn_local(async move {
        match opfs_load(&key).await {
            Ok(Some(yaml)) => {
                match yaml_serde::from_str::<crate::annotations::AnnotationSet>(&yaml) {
                    Ok(loaded) => {
                        // Only apply if this file index still exists and has no annotations yet
                        let already_has = state.annotation_store.get_untracked()
                            .sets.get(file_idx)
                            .and_then(|s| s.as_ref())
                            .map(|s| !s.annotations.is_empty())
                            .unwrap_or(false);
                        if !already_has {
                            state.annotation_store.update(|store| {
                                store.ensure_len(file_idx + 1);
                                store.sets[file_idx] = Some(loaded);
                            });
                            log::debug!("OPFS loaded annotations for file {file_idx}: {key}");
                        }
                    }
                    Err(e) => log::warn!("OPFS deserialize error for {key}: {e}"),
                }
            }
            Ok(None) => {
                // Also try the Layer 1 key (filename+size) as fallback
                // if we searched by spot_hash and found nothing
                if identity.spot_hash.is_some() {
                    let fallback_key = {
                        let safe_name: String = identity.filename.chars()
                            .map(|c| if c.is_alphanumeric() || c == '.' || c == '-' || c == '_' { c } else { '_' })
                            .collect();
                        format!("{}_{}.batm", safe_name, identity.file_size)
                    };
                    if fallback_key != key {
                        match opfs_load(&fallback_key).await {
                            Ok(Some(yaml)) => {
                                if let Ok(loaded) = yaml_serde::from_str::<crate::annotations::AnnotationSet>(&yaml) {
                                    let already_has = state.annotation_store.get_untracked()
                                        .sets.get(file_idx)
                                        .and_then(|s| s.as_ref())
                                        .map(|s| !s.annotations.is_empty())
                                        .unwrap_or(false);
                                    if !already_has {
                                        state.annotation_store.update(|store| {
                                            store.ensure_len(file_idx + 1);
                                            store.sets[file_idx] = Some(loaded);
                                        });
                                        log::debug!("OPFS loaded annotations (fallback key) for file {file_idx}: {fallback_key}");
                                        // Re-save under the spot_hash key for faster future lookups
                                        save_annotations_to_opfs(state, file_idx);
                                    }
                                }
                            }
                            _ => {}
                        }
                    }
                }
            }
            Err(e) => log::warn!("OPFS load error for {key}: {e}"),
        }
    });
}
