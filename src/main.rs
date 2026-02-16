use std::{path::Path, process::Command};


fn main() {
    let youtube_url = "https://www.youtube.com/watch?v=SEFIsuiDTm8&list=RDSEFIsuiDTm8";
    println!("Downloading: {}", youtube_url);
    let output = Command::new("yt-dlp")
    .args([
        "--extractor-args", "youtube:player-client=ios,web",
        "-N", "16",
        "--quiet",
        "--restrict-filenames",
        "--extract-audio",
        "--audio-format", "wav",
        "--audio-quality", "0",   // Best quality
        "--no-playlist",
        "--print", "%(artist,uploader)s",
        "--print", "%(title)s",
        "--print", "after_move:filepath",
        "-o", "%(artist,uploader)s-%(title)s.%(ext)s",
        youtube_url,
    ])
    .output()
    .expect("Failed to execute yt-dlp");

    if !output.status.success() {
        println!("Error: {}", String::from_utf8_lossy(&output.stderr));
        return;
    }

    let stdout = String::from_utf8_lossy(&output.stdout);
    let lines: Vec<&str> = stdout.lines().collect();

    let [artist_or_uploader, title] = [
        lines.get(0).unwrap_or(&"Unkown"), 
        lines.get(1).unwrap_or(&"Unknown"),
        // lines.get(2).unwrap_or(&"Unknown.wav"),
    ];
    
    println!("File info: {}, {}", artist_or_uploader, title);
    
    // if !Path::new(filename).exists() {
    //     eprintln!("❌ Error: yt-dlp reported success, but the file '{}' was not found on disk.", filename);
    //     return;
    // }
    
    // let mut reader = hound::WavReader::open(filename).expect("Failed to open file");
    // let spec = reader.spec();
    
    // println!("Audio Specs: {}Hz, {} bits, {} channels", spec.sample_rate, spec.bits_per_sample, spec.channels);

    // let samples: Vec<f32> = reader
    //     .samples::<i16>()
    //     .skip(44100 * 60)
    //     .map(|s| s.unwrap() as f32 / i16::MAX as f32)
    //     .take(1000)
    //     .collect();

    // println!("Visualizing the first 1000 samples:");
    // for (i, sample) in samples.iter().enumerate() {
    //     if i % 20 == 0 {
    //         let width = (sample * 40.0).abs() as usize;
    //         let bar = "#".repeat(width);
    //         println!("{:>4} | {}", i, bar);
    //     }
    // }
}
