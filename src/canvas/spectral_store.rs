//! Thread-local store for lazily-computed STFT columns.
//!
//! During file loading, spectrogram columns are inserted incrementally as
//! they are computed.  The tile cache reads completed tile-ranges from here
//! to render RGBA tiles progressively, instead of waiting for the entire
//! spectrogram to finish.

use std::cell::RefCell;
use crate::types::SpectrogramColumn;

struct SpectralColumnStore {
    /// Which file this store is for (invalidated when file changes).
    file_idx: usize,
    /// Columns indexed by spectrogram column number.  `None` = not yet computed.
    columns: Vec<Option<SpectrogramColumn>>,
    /// Running maximum magnitude across all inserted columns.
    max_magnitude: f32,
}

thread_local! {
    static STORE: RefCell<Option<SpectralColumnStore>> = const { RefCell::new(None) };
}

/// Initialise (or reset) the store for a new file.
pub fn init(file_idx: usize, total_cols: usize) {
    STORE.with(|s| {
        *s.borrow_mut() = Some(SpectralColumnStore {
            file_idx,
            columns: (0..total_cols).map(|_| None).collect(),
            max_magnitude: 0.0,
        });
    });
}

/// Insert computed columns into the store, starting at `start_col`.
/// Updates the running max magnitude.
pub fn insert_columns(file_idx: usize, start_col: usize, cols: &[SpectrogramColumn]) {
    STORE.with(|s| {
        let mut store = s.borrow_mut();
        let store = match store.as_mut() {
            Some(st) if st.file_idx == file_idx => st,
            _ => return,
        };
        for (i, col) in cols.iter().enumerate() {
            let idx = start_col + i;
            if idx < store.columns.len() {
                // Update running max
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
    STORE.with(|s| {
        let store = s.borrow();
        let store = match store.as_ref() {
            Some(st) if st.file_idx == file_idx => st,
            _ => return false,
        };
        let end = tile_end.min(store.columns.len());
        (tile_start..end).all(|i| store.columns[i].is_some())
    })
}

/// Borrow a completed slice of columns from the store and call `f` with them.
/// Returns `None` if any column in the range is missing or the store doesn't
/// match `file_idx`.
pub fn with_columns<R>(
    file_idx: usize,
    start: usize,
    end: usize,
    f: impl FnOnce(&[SpectrogramColumn], f32) -> R,
) -> Option<R> {
    STORE.with(|s| {
        let store = s.borrow();
        let store = store.as_ref()?;
        if store.file_idx != file_idx { return None; }
        let end = end.min(store.columns.len());
        // Collect the contiguous slice (all must be Some)
        let cols: Vec<&SpectrogramColumn> = (start..end)
            .map(|i| store.columns.get(i)?.as_ref())
            .collect::<Option<Vec<_>>>()?;
        // Build a temporary owned vec for the callback
        let owned: Vec<SpectrogramColumn> = cols.into_iter().cloned().collect();
        Some(f(&owned, store.max_magnitude))
    })
}

/// Get the current running max magnitude.
pub fn get_max_magnitude(file_idx: usize) -> f32 {
    STORE.with(|s| {
        let store = s.borrow();
        store.as_ref()
            .filter(|st| st.file_idx == file_idx)
            .map(|st| st.max_magnitude)
            .unwrap_or(0.0)
    })
}

/// Drain all columns from the store and return them as a contiguous Vec.
/// Missing columns are replaced with empty `SpectrogramColumn` structs.
/// Clears the store afterwards.
pub fn drain_columns(file_idx: usize) -> Option<Vec<SpectrogramColumn>> {
    STORE.with(|s| {
        let mut store = s.borrow_mut();
        let st = store.as_mut()?;
        if st.file_idx != file_idx { return None; }
        let cols = std::mem::take(&mut st.columns);
        let result: Vec<SpectrogramColumn> = cols.into_iter()
            .map(|opt| opt.unwrap_or_else(|| SpectrogramColumn {
                magnitudes: Vec::new(),
                time_offset: 0.0,
            }))
            .collect();
        *store = None;
        Some(result)
    })
}

/// Clear the store (e.g. when a file is unloaded).
pub fn clear() {
    STORE.with(|s| *s.borrow_mut() = None);
}
