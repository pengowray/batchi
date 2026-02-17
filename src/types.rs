#[derive(Clone, Debug)]
pub struct AudioData {
    pub samples: Vec<f32>,
    pub sample_rate: u32,
    pub channels: u32,
    pub duration_secs: f64,
}

#[derive(Clone, Debug)]
pub struct SpectrogramColumn {
    pub magnitudes: Vec<f32>,
    pub time_offset: f64,
}

#[derive(Clone, Debug)]
pub struct SpectrogramData {
    pub columns: Vec<SpectrogramColumn>,
    pub freq_resolution: f64,
    pub time_resolution: f64,
    pub max_freq: f64,
    pub sample_rate: u32,
}

#[derive(Clone, Debug)]
pub struct PreviewImage {
    pub width: u32,
    pub height: u32,
    pub pixels: Vec<u8>, // RGBA, row-major, row 0 = highest freq
}

#[derive(Clone, Debug)]
pub struct ZeroCrossingResult {
    pub estimated_frequency_hz: f64,
    pub crossing_count: usize,
    pub duration_secs: f64,
}
