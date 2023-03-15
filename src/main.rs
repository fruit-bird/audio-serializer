use audio_serializer::AudioBuffer;
use std::error::Error;

fn main() -> Result<(), Box<dyn Error>> {
    let buffer = AudioBuffer::from_audio_file("Boku_no_Iikata.mp3")?;
    buffer.to_json("audio.json")?;

    let buffer_again = AudioBuffer::from_json("audio.json")?;
    buffer_again.write_to_wav("again.wav")?;

    Ok(())
}
