//! Thread-local store for lazily-computed STFT columns.
//!
//! During file loading, spectrogram columns are inserted incrementally as
//! they are computed.  The tile cache reads completed tile-ranges from here
//! to render RGBA tiles progressively, instead of waiting for the entire
//! spectrogram to finish.
//!
//! Supports multiple concurrent file loads — each file_idx gets its own store.

use std::cell::RefCell;
use std::collections::HashMap;
use crate::types::SpectrogramColumn;

struct SpectralColumnStore {
    /// Columns indexed by spectrogram column number.  `None` = not yet computed.
    columns: Vec<Option<SpectrogramColumn>>,
    /// Running maximum magnitude across all inserted columns.
    max_magnitude: f32,
}

thread_local! {
    /// Keyed by file_idx so multiple files can load concurrently.
    static STORES: RefCell<HashMap<usize, SpectralColumnStore>> =
        RefCell::new(HashMap::new());
}

/// Initialise (or reset) the store for a file.
pub fn init(file_idx: usize, total_cols: usize) {
    STORES.with(|s| {
        s.borrow_mut().insert(file_idx, SpectralColumnStore {
            columns: (0..total_cols).map(|_| None).collect(),
            max_magnitude: 0.0,
        });
    });
}

/// Insert computed columns into the store, starting at `start_col`.
/// Updates the running max magnitude.
pub fn insert_columns(file_idx: usize, start_col: usize, cols: &[SpectrogramColumn]) {
    STORES.with(|s| {
        let mut stores = s.borrow_mut();
        let Some(store) = stores.get_mut(&file_idx) else { return };
        for (i, col) in cols.iter().enumerate() {
            let idx = start_col + i;
            if idx < store.columns.len() {
                for &mag in &col.magnitudes {
                    if mag > store.max_magnitude {
                        store.max_magnitude = mag;
                    }
                }
                store.columns[idx] = Some(col.clone());
            }
        }
    });
}

/// Check whether all columns in the range `tile_start..tile_end` are present.
pub fn tile_complete(file_idx: usize, tile_start: usize, tile_end: usize) -> bool {
    STORES.with(|s| {
        let stores = s.borrow();
        let Some(store) = stores.get(&file_idx) else { return false };
        let end = tile_end.min(store.columns.len());
        (tile_start..end).all(|i| store.columns[i].is_some())
    })
}

/// Borrow a completed slice of columns from the store and call `f` with them.
/// Returns `None` if any column in the range is missing or the store doesn't
/// exist for `file_idx`.
pub fn with_columns<R>(
    file_idx: usize,
    start: usize,
    end: usize,
    f: impl FnOnce(&[SpectrogramColumn], f32) -> R,
) -> Option<R> {
    STORES.with(|s| {
        let stores = s.borrow();
        let store = stores.get(&file_idx)?;
        let end = end.min(store.columns.len());
        let cols: Vec<&SpectrogramColumn> = (start..end)
            .map(|i| store.columns.get(i)?.as_ref())
            .collect::<Option<Vec<_>>>()?;
        let owned: Vec<SpectrogramColumn> = cols.into_iter().cloned().collect();
        Some(f(&owned, store.max_magnitude))
    })
}

/// Get the current running max magnitude.
pub fn get_max_magnitude(file_idx: usize) -> f32 {
    STORES.with(|s| {
        s.borrow().get(&file_idx)
            .map(|st| st.max_magnitude)
            .unwrap_or(0.0)
    })
}

/// Drain all columns from the store and return them as a contiguous Vec.
/// Missing columns are replaced with empty `SpectrogramColumn` structs.
/// Removes this file's store entry afterwards.
pub fn drain_columns(file_idx: usize) -> Option<Vec<SpectrogramColumn>> {
    STORES.with(|s| {
        let mut stores = s.borrow_mut();
        let store = stores.remove(&file_idx)?;
        let result: Vec<SpectrogramColumn> = store.columns.into_iter()
            .map(|opt| opt.unwrap_or_else(|| SpectrogramColumn {
                magnitudes: Vec::new(),
                time_offset: 0.0,
            }))
            .collect();
        Some(result)
    })
}

/// Remove a specific file's store (e.g. when a file is unloaded).
pub fn clear_file(file_idx: usize) {
    STORES.with(|s| { s.borrow_mut().remove(&file_idx); });
}

/// Clear all stores.
pub fn clear() {
    STORES.with(|s| s.borrow_mut().clear());
}
