use crate::audio::guano::GuanoMetadata;
use std::sync::Arc;

#[derive(Clone, Debug)]
pub struct FileMetadata {
    pub file_size: usize,
    pub format: &'static str,
    pub bits_per_sample: u16,
    pub is_float: bool,
    pub guano: Option<GuanoMetadata>,
}

#[derive(Clone, Debug)]
pub struct AudioData {
    pub samples: Arc<Vec<f32>>,
    pub sample_rate: u32,
    pub channels: u32,
    pub duration_secs: f64,
    pub metadata: FileMetadata,
}

#[derive(Clone, Debug)]
pub struct SpectrogramColumn {
    pub magnitudes: Vec<f32>,
    pub time_offset: f64,
}

#[derive(Clone, Debug)]
pub struct SpectrogramData {
    pub columns: Arc<Vec<SpectrogramColumn>>,
    pub freq_resolution: f64,
    pub time_resolution: f64,
    pub max_freq: f64,
    pub sample_rate: u32,
}

#[derive(Clone, Debug)]
pub struct PreviewImage {
    pub width: u32,
    pub height: u32,
    pub pixels: Arc<Vec<u8>>, // RGBA, row-major, row 0 = highest freq
}

#[derive(Clone, Debug)]
pub struct ZeroCrossingResult {
    pub estimated_frequency_hz: f64,
    pub crossing_count: usize,
    pub duration_secs: f64,
}
