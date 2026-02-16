use web_sys::{AudioContext, AudioContextOptions, AudioBufferSourceNode};
use crate::types::AudioData;
use crate::state::Selection;
use crate::dsp::heterodyne::heterodyne_mix;
use crate::dsp::filters::lowpass_filter;
use std::cell::RefCell;

thread_local! {
    static CURRENT_SOURCE: RefCell<Option<AudioBufferSourceNode>> = RefCell::new(None);
    static CURRENT_CTX: RefCell<Option<AudioContext>> = RefCell::new(None);
}

pub fn stop() {
    CURRENT_SOURCE.with(|s| {
        if let Some(source) = s.borrow_mut().take() {
            #[allow(deprecated)]
            let _ = source.stop();
        }
    });
    CURRENT_CTX.with(|c| {
        if let Some(ctx) = c.borrow_mut().take() {
            let _ = ctx.close();
        }
    });
}

pub fn play_normal(audio: &AudioData, selection: Option<Selection>) {
    stop();

    let (samples, sample_rate) = extract_selection(audio, selection);

    // Apply bandpass if selection has frequency bounds
    let samples = if let Some(sel) = selection {
        if sel.freq_low > 0.0 || sel.freq_high < (sample_rate as f64 / 2.0) {
            apply_bandpass(&samples, sample_rate, sel.freq_low, sel.freq_high)
        } else {
            samples
        }
    } else {
        samples
    };

    play_samples(&samples, sample_rate);
}

pub fn play_heterodyne(audio: &AudioData, lo_freq: f64, selection: Option<Selection>) {
    stop();

    let (samples, sample_rate) = extract_selection(audio, selection);

    // If selection has frequency bounds, use center as LO frequency
    let effective_lo = if let Some(sel) = selection {
        if sel.freq_low > 0.0 || sel.freq_high > 0.0 {
            (sel.freq_low + sel.freq_high) / 2.0
        } else {
            lo_freq
        }
    } else {
        lo_freq
    };

    let processed = heterodyne_mix(&samples, sample_rate, effective_lo);
    play_samples(&processed, sample_rate);
}

fn extract_selection(audio: &AudioData, selection: Option<Selection>) -> (Vec<f32>, u32) {
    let sr = audio.sample_rate;
    if let Some(sel) = selection {
        let start = (sel.time_start * sr as f64) as usize;
        let end = (sel.time_end * sr as f64) as usize;
        let start = start.min(audio.samples.len());
        let end = end.min(audio.samples.len());
        if end > start {
            return (audio.samples[start..end].to_vec(), sr);
        }
    }
    (audio.samples.clone(), sr)
}

fn apply_bandpass(samples: &[f32], sample_rate: u32, freq_low: f64, freq_high: f64) -> Vec<f32> {
    let mut result = samples.to_vec();
    // High-pass via subtraction: result = original - lowpass(original, freq_low)
    if freq_low > 0.0 {
        let lp = cascaded_lowpass(&result, freq_low, sample_rate, 4);
        for (r, l) in result.iter_mut().zip(lp.iter()) {
            *r -= l;
        }
    }
    // Low-pass
    if freq_high < (sample_rate as f64 / 2.0) {
        result = cascaded_lowpass(&result, freq_high, sample_rate, 4);
    }
    result
}

fn cascaded_lowpass(samples: &[f32], cutoff: f64, sample_rate: u32, passes: usize) -> Vec<f32> {
    let mut result = samples.to_vec();
    for _ in 0..passes {
        result = lowpass_filter(&result, cutoff, sample_rate);
    }
    result
}

fn play_samples(samples: &[f32], sample_rate: u32) {
    // Try to create AudioContext at the file's native sample rate
    let opts = AudioContextOptions::new();
    opts.set_sample_rate(sample_rate as f32);
    let ctx = AudioContext::new_with_context_options(&opts)
        .or_else(|_| AudioContext::new())
        .unwrap();

    let buffer = ctx
        .create_buffer(1, samples.len() as u32, sample_rate as f32)
        .unwrap();
    let _ = buffer.copy_to_channel(&samples, 0);

    let source = ctx.create_buffer_source().unwrap();
    source.set_buffer(Some(&buffer));
    let _ = source.connect_with_audio_node(&ctx.destination());
    let _ = source.start();

    CURRENT_SOURCE.with(|s| {
        *s.borrow_mut() = Some(source);
    });
    CURRENT_CTX.with(|c| {
        *c.borrow_mut() = Some(ctx);
    });
}
