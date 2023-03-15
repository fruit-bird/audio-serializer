use hound::{SampleFormat, WavSpec, WavWriter};
use rodio::Decoder;
use serde::{Deserialize, Serialize};
use std::{
    error::Error,
    fs::{self, File},
    io::BufReader,
    path::Path,
};

const CHANNELS: u16 = 2;
const SAMPLE_RATE: u32 = 44100;

#[derive(Debug, Serialize, Deserialize)]
pub struct AudioBuffer {
    buffer: Vec<i16>,
}

impl AudioBuffer {
    pub fn from_audio_file(path: impl AsRef<Path>) -> Result<AudioBuffer, Box<dyn Error>> {
        let audio_file = File::open(path)?;
        let buffer = Decoder::new(BufReader::new(audio_file))?.collect();

        Ok(AudioBuffer { buffer })
    }

    /// Creating a wav file from the buffer
    pub fn write_to_wav(&self, output_path: impl AsRef<Path>) -> Result<(), Box<dyn Error>> {
        let spec = WavSpec {
            channels: CHANNELS,
            sample_rate: SAMPLE_RATE,
            bits_per_sample: 16,
            sample_format: SampleFormat::Int,
        };
        let mut writer = WavWriter::create(output_path, spec)?;

        for sample in &self.buffer {
            writer.write_sample(*sample)?;
        }
        writer.finalize()?;

        Ok(())
    }

    pub fn to_json(&self, path: impl AsRef<Path>) -> Result<(), Box<dyn Error>> {
        let mut json_file = File::create(path)?;
        let json_string = serde_json::to_string(self)?;

        serde_json::to_writer(&mut json_file, &json_string)?;
        Ok(())
    }

    pub fn from_json(path: impl AsRef<Path>) -> Result<AudioBuffer, Box<dyn Error>> {
        let json = fs::read_to_string(path)?;
        let audio_buffer = serde_json::from_str(&json)?;

        Ok(audio_buffer)
    }
}
