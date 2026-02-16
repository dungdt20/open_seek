use std::{fs::File, process::Command};
use std::io::{Read};

fn main() -> std::io::Result<()> {
    // let youtube_url = "https://www.youtube.com/watch?v=SEFIsuiDTm8&list=RDSEFIsuiDTm8";
    // println!("Downloading: {}", youtube_url);
    // let output = Command::new("yt-dlp")
    // .args([
    //     "--extractor-args", "youtube:player-client=ios,web",
    //     "-N", "16",
    //     "--quiet",
    //     "--restrict-filenames",
    //     "--extract-audio",
    //     "--audio-format", "wav",
    //     "--audio-quality", "0",   // Best quality
    //     "--no-playlist",
    //     "--print", "%(artist,uploader)s",
    //     "--print", "%(title)s",
    //     "--print", "after_move:filepath",
    //     "-o", "output.%(ext)s",
    //     youtube_url,
    // ])
    // .output()
    // .expect("Failed to execute yt-dlp");

    // if !output.status.success() {
    //     println!("Error: {}", String::from_utf8_lossy(&output.stderr));
    //     return Ok(());
    // }

    // let stdout = String::from_utf8_lossy(&output.stdout);
    // let lines: Vec<&str> = stdout.lines().collect();

    // let [artist_or_uploader, title] = [
    //     lines.get(0).unwrap_or(&"Unkown"), 
    //     lines.get(1).unwrap_or(&"Unknown"),
    // ];
    
    // println!("File info: {}, {}", artist_or_uploader, title);
    
    
    let mut file = File::open("output.wav")?;
    let mut buffer = Vec::new();
    file.read_to_end(&mut buffer)?; // Read whole file into memory for easier searching

    // 1. Find the "data" marker in the byte array
    // The actual samples start 4 bytes AFTER the "data" tag and its size field
    let data_tag = b"data";
    let data_start_index = buffer.windows(4)
        .position(|window| window == data_tag)
        .expect("Could not find 'data' chunk in WAV") + 8; // +4 for "data", +4 for size field

    // 2. Extract metadata safely from the buffer (offsets 24 and 34)
    let sample_rate = u32::from_le_bytes(buffer[24..28].try_into().unwrap());
    
    // 3. Convert ONLY the audio portion to f32
    let audio_bytes = &buffer[data_start_index..];
    let samples: Vec<f32> = audio_bytes
        .chunks_exact(4) // 2 bytes (Left) + 2 bytes (Right) = 4 bytes per "moment"
        .map(|frame| {
            // 1. Get Left Channel
            let left = i16::from_le_bytes([frame[0], frame[1]]) as f32;
            // 2. Get Right Channel
            let right = i16::from_le_bytes([frame[2], frame[3]]) as f32;
            
            // 3. Average them and normalize to -1.0 .. 1.0
            ((left + right) / 2.0) / 32768.0
        })
        .collect();

    println!("Success! Sample Rate: {}Hz, Samples: {}", sample_rate, samples.len());
    
    // Now pass to your filter
    let processed = process_audio(samples, sample_rate);
    
    println!("Success LPF! Samples: {}", processed.len());

    Ok(())
}

fn process_audio(samples: Vec<f32>, original_rate: u32) -> Vec<f32> {
    let target_rate = 11000.0;
    let cutoff = 5000.0;
    
    // 1. Simple Low Pass Filter (IIR) to avoid aliasing
    // RC filter math: alpha = dt / (RC + dt)
    let dt = 1.0 / original_rate as f32;
    let rc = 1.0 / (2.0 * std::f32::consts::PI * cutoff);
    let alpha = dt / (rc + dt);

    let mut filtered = Vec::with_capacity(samples.len());
    let mut prev_y = 0.0;
    for x in samples {
        let y = alpha * x + (1.0 - alpha) * prev_y;
        filtered.push(y);
        prev_y = y;
    }

    // 2. Naive Downsampling (Decimation)
    let skip_factor = original_rate as f32 / target_rate;

    let downsampled = filtered.into_iter().step_by(skip_factor as usize).collect();
    downsampled
}
