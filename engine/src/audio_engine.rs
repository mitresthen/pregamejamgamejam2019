use std::sync::{Arc, Mutex};
use std::collections::HashMap;

use sdl2::AudioSubsystem;
use sdl2::audio::{AudioFormat, AudioDevice, AudioCallback, AudioSpecDesired,AudioSpecWAV};

use super::Error;

pub struct SoundInstance {
    data: Vec<f32>,
    position: usize,
    repeats: i32,
    is_done: bool,
}

#[derive(Debug, Copy, Clone)]
pub enum WavError {
    Not16Bit,
    NotStereo,
    Not44100Hz
}

impl SoundInstance {
    pub fn new(data: Vec<f32>, repeats: i32) -> SoundInstance {
        SoundInstance { data, position: 0, repeats, is_done: false }
    }

    pub fn request_samples(&mut self, amount : usize) -> Vec<f32>{
        let mut return_vec = Vec::new();
        let mut remaining = amount;
        while remaining > 0 {
            let mut end : usize = self.position + remaining;
            if self.data.len() - self.position < remaining {
                end = self.data.len() - 1;
            }
            return_vec.extend(self.data[self.position..end].to_vec());
            remaining -= end - self.position;
            self.position = end;
            if remaining > 0 {
                if self.repeats == 0 {
                    return_vec.extend(vec![0.0; remaining]);
                    self.is_done = true;
                    break;
                }
                else {
                    self.position = 0;
                }
                if self.repeats > 0 {
                    self.repeats -= 1;
                }
            }
        }
        return return_vec;
    }

}

#[derive(Clone)]
pub struct AudioMixer {
    playing: Arc<Mutex<Vec<SoundInstance>>>,
}

impl AudioMixer {
    pub fn new() -> AudioMixer {
        AudioMixer {
            playing: Arc::new(Mutex::new(Vec::new()))
        }
    }

    pub fn play_sound(&mut self, sound: SoundInstance) {
        self.playing.lock().unwrap().push(sound);
    }
}

impl AudioCallback for AudioMixer {
    type Channel = f32;

    fn callback(&mut self, out: &mut [f32]) {
        let mut sounds = self.playing.lock().unwrap();


        let mut samples = Vec::new();
        for sound_instance in sounds.iter_mut() {
            samples.push(sound_instance.request_samples(out.len()));
        }
        for i in 0..out.len() {
            out[i] = 0.0;
            for s in samples.iter() {
                out[i] += (*s)[i];
            }
        }

        let mut old : Vec<SoundInstance>  = sounds.drain(..).collect();
        *sounds = old.drain(..).filter(|s| !s.is_done).collect();
    }
}


pub struct AudioEngine {
    _audio_device: AudioDevice<AudioMixer>,
    mixer: AudioMixer,
    _sound_map: HashMap<String, Vec<f32>>,
}


impl AudioEngine {
    pub fn new(audio_subsystem: AudioSubsystem) -> AudioEngine {
        let desired_spec = AudioSpecDesired {
            freq: Some(44_100),
            channels: Some(1), // mono
            samples: None      // default
        };

        let mixer = AudioMixer::new();

        let device = audio_subsystem.open_playback(None, &desired_spec, |spec| {
            if spec.format != AudioFormat::F32LSB {
                panic!("Audio device opened with wrong AudioFormat. Expected Float 32. Got {:?}", spec.format);
            }

            // initialize the audio callback
            mixer.clone()
        }).unwrap();

        device.resume();

        AudioEngine {
            _audio_device: device,
            mixer: mixer,
            _sound_map: HashMap::new(),
        }
    }

    pub fn play_sound_from_file(&mut self, filename: &str) -> Result<(), Error> {
        return self.loop_sound_from_file(filename, 0);
    }

    pub fn loop_sound_from_file(&mut self, filename: &str, repeats:i32) -> Result<(), Error> {

        if !self._sound_map.contains_key(filename) {
            self.pre_load_file(filename);
        }
        let pcm_mono_float = self._sound_map.get(filename).unwrap().to_vec();
        self.loop_sound(pcm_mono_float, repeats);
        Ok(())
    }

    pub fn pre_load_file(&mut self, filename: &str) -> Result<(), Error> {
        use std::slice;
        use std::mem;
        use std::i16;

        let wav = AudioSpecWAV::load_wav(filename)?;

        if wav.format != AudioFormat::S16LSB {
            return Err(Error::WavError(WavError::Not16Bit));
        }

        if wav.channels != 2 {
            return Err(Error::WavError(WavError::NotStereo));
        }

        if wav.freq != 44100 {
            return Err(Error::WavError(WavError::Not44100Hz));
        }

        let pcm_stereo_16 : &[i16]= unsafe {
            slice::from_raw_parts(
                mem::transmute(wav.buffer().as_ptr()),
                wav.buffer().len() / 2
            )
        };

        let pcm_stereo_float : Vec<f32> = pcm_stereo_16.iter().map(|x| (*x as f32) / (i16::MAX as f32)).collect();

        let pcm_mono_float = pcm_stereo_float.chunks(2).map(|lr| (lr[0] + lr[1]) / 2.0).collect();
        self._sound_map.insert(filename.to_string(), pcm_mono_float);
        Ok(())
    }

    pub fn play_sound(&mut self, data: Vec<f32>) {
        self.mixer.play_sound(SoundInstance::new(data, 0));
    }

    pub fn loop_sound(&mut self, data: Vec<f32>, repeats: i32) {
        self.mixer.play_sound(SoundInstance::new(data, repeats));
    }
}
