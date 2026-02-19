use open_seek::{fingerprint, process_audio};

fn main() {
    // 1. Process the audio to get a list of FFT frames (the Spectrogram)
    // This should return Vec<Vec<f32>>
    let spectrogram = process_audio::process().unwrap(); 

    // 2. Convert those frames into peaks and then hashes
    let hashes = fingerprint::samples_to_fingerprints(spectrogram);

    // 3. Save them to your storage (File, DB, etc.)
    // fingerprint::Fingerprint::save_all(&hashes)?;

    println!("Successfully saved {} fingerprints.", hashes.len());
}
