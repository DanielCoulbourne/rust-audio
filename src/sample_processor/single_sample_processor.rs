pub fn amplify(sample: AudioSample, factor: f32) -> AudioSample {
    let amplified = sample as f32 * factor;
    return amplified as i16;
}

pub fn noise_gate(sample: AudioSample, threshold: f32) -> AudioSample {
    if sample as f32 > threshold {
        return sample;
    } else {
        return 0;
    }
}

pub fn normalize(sample: AudioSample, target: f32) -> AudioSample {
    let factor = target / sample as f32;

    return amplify(sample, factor);
}