use std::collections::hash_map::DefaultHasher;
use std::hash::{Hash, Hasher};

#[derive(Debug, Clone, Copy)]
pub struct Peak {
    pub frame_index: u32, // Time dimension
    pub bin_index: u32,   // Frequency dimension
}

#[derive(Debug, Clone)]
pub struct Fingerprint {
    pub hash: u64,
    pub absolute_time: u32,
}

/// 1. Peak Picker
/// Scans the FFT output for local maxima that exceed a noise threshold.
pub fn find_peaks(magnitudes: &[f32], frame_idx: u32, threshold: f32) -> Vec<Peak> {
    let mut peaks = Vec::new();
    // Simple 1D peak finding (can be expanded to 2D across time frames)
    for i in 1..magnitudes.len() - 1 {
        if magnitudes[i] > threshold && magnitudes[i] > magnitudes[i - 1] && magnitudes[i] > magnitudes[i + 1] {
            peaks.push(Peak {
                frame_index: frame_idx,
                bin_index: i as u32,
            });
        }
    }
    peaks
}

/// 2. Combinatorial Hashing
/// Pairs peaks together to create a robust fingerprint (Landmark).
/// Concept: [Freq1 | Freq2 | TimeDelta] -> Hash
pub fn generate_hashes(peaks: &[Peak], fan_out: usize) -> Vec<Fingerprint> {
    let mut fingerprints = Vec::new();

    for i in 0..peaks.len() {
        // For every peak, pair it with 'fan_out' subsequent peaks
        for j in 1..=fan_out {
            if i + j >= peaks.len() { break; }

            let anchor = &peaks[i];
            let target = &peaks[i + j];

            let delta_time = target.frame_index - anchor.frame_index;
            
            // Optimization: Only pair peaks within a reasonable time window (e.g., < 2 seconds)
            if delta_time < 200 { 
                let mut s = DefaultHasher::new();
                anchor.bin_index.hash(&mut s);
                target.bin_index.hash(&mut s);
                delta_time.hash(&mut s);

                fingerprints.push(Fingerprint {
                    hash: s.finish(),
                    absolute_time: anchor.frame_index,
                });
            }
        }
    }
    fingerprints
}

/// 3. The Full Conversion Function
pub fn samples_to_fingerprints(all_frames_magnitudes: Vec<Vec<f32>>) -> Vec<Fingerprint> {
    let mut all_peaks = Vec::new();
    let threshold = 0.5; // Adjust based on your signal's energy
    let fan_out = 3;     // Number of pairs per anchor peak

    // Step A: Collect peaks from all time frames
    for (idx, frame) in all_frames_magnitudes.into_iter().enumerate() {
        let peaks = find_peaks(&frame, idx as u32, threshold);
        all_peaks.extend(peaks);
    }

    // Step B: Hash the peak constellations
    generate_hashes(&all_peaks, fan_out)
}