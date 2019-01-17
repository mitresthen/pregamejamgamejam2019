use std::sync::{Arc, Mutex};

use sdl2::AudioSubsystem;
use sdl2::audio::{AudioFormat, AudioDevice, AudioCallback, AudioSpecDesired,AudioSpecWAV,AudioCVT};

pub struct SoundInstance {
    data: Vec<f32>,
    position: usize,
}

impl SoundInstance {
    pub fn new(data: Vec<f32>) -> SoundInstance {
        SoundInstance { data, position: 0 }
    }

    pub fn next_sample(&mut self) -> Option<f32> {
        let maybe_sample : Option<f32> = self.data.iter().nth(self.position).map(|x| *x);
        self.position += 1;
        maybe_sample
    }

    pub fn is_done(&self) -> bool {
        self.position >= self.data.len()
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

        for dst in out {
            *dst = 0.0;
            for sound_instance in sounds.iter_mut() {
                *dst += sound_instance.next_sample().unwrap_or(0.0);
            }
        }

        let mut old : Vec<SoundInstance>  = sounds.drain(..).collect();
        *sounds = old.drain(..).filter(|s| !s.is_done()).collect();
    }
}


pub struct AudioEngine {
    audio_device: AudioDevice<AudioMixer>,
    mixer: AudioMixer,
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
            // initialize the audio callback
            mixer.clone()
        }).unwrap();

        device.resume();

        AudioEngine {
            audio_device: device,
            mixer: mixer
        }
    }

    pub fn play_sound_from_file(&mut self, filename: &str) {
        use std::slice;
        use std::mem;
        use std::i16;

        let wav = AudioSpecWAV::load_wav(filename)
            .expect("Could not load test WAV file");

        if wav.format != AudioFormat::S16LSB {
            panic!("Gamejam games only support signed 16-bit audio wav files");
        }

        if wav.channels != 2 {
            panic!("Gamejam games only support stereo audio files");
        }

        if wav.freq != 44100 {
            panic!("Gamejam games only support 44100hz audio");
        }

        let pcm_stereo_16 : &[i16]= unsafe {
            slice::from_raw_parts(
                mem::transmute(wav.buffer().as_ptr()),
                wav.buffer().len() / 2
            )
        };

        let pcm_stereo_float : Vec<f32> = pcm_stereo_16.iter().map(|x| (*x as f32) / (i16::MAX as f32)).collect();

        let pcm_mono_float = pcm_stereo_float.chunks(2).map(|lr| (lr[0] + lr[1]) / 2.0).collect();

        self.play_sound(pcm_mono_float);
    }

    pub fn play_sound(&mut self, data: Vec<f32>) {
        self.mixer.play_sound(SoundInstance::new(data));
    }
}
