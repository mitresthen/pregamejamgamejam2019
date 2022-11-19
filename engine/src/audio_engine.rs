use std::sync::{Arc, Mutex};
use std::collections::HashMap;
use std::hash::{Hash, Hasher};
use std::collections::hash_map::DefaultHasher;

use sdl2::AudioSubsystem;
use sdl2::audio::{AudioFormat, AudioDevice, AudioCallback, AudioSpecDesired,AudioSpecWAV};

use super::Error;

pub struct SoundInstance {
    data: Vec<f32>,
    position: usize,
    repeats: i32,
    is_done: bool,
    paused: bool,
    volume: f32,
}

#[derive(Debug, Copy, Clone)]
pub enum WavError {
    Not16Bit,
    NotStereo,
    Not44100Hz
}

impl SoundInstance {
    pub fn new(data: Vec<f32>, repeats: i32) -> SoundInstance {
        SoundInstance { data, position: 0, repeats, is_done: false, paused: false, volume: 1.0 }
    }

    pub fn clear(&mut self) {
        self.data.clear();
        self.position = 0;
        self.repeats = 0;
        self.is_done = true;
        self.paused = true;
        self.volume = 0.0;
    }

    pub fn set_volume(&mut self, volume: f32) {
        self.volume = volume;
    }

    pub fn pause(&mut self) {
        self.paused = true;
    }

    pub fn stop(&mut self) {
        self.paused = true;
        self.position = 0;
    }

    pub fn stop_repetition(&mut self) {
        self.repeats = 0;
    }

    pub fn play(&mut self) {
        self.paused = false;
    }

    pub fn toggle_pause(&mut self) {
        self.paused = !self.paused;
    }

    pub fn is_playing(&self) -> bool {
        !(self.is_done || self.paused)
    }

    pub fn request_samples(&mut self, amount : usize) -> Option<Vec<f32>> {
        if !self.is_playing() {
            return None;
        }
        let mut return_vec = Vec::new();
        let mut remaining = amount;
        while remaining > 0 {

            let mut end : usize = self.position + remaining;
            if self.data.len() - self.position < remaining || end >= self.data.len() {
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
        return Some(return_vec.iter().map(|x| x * self.volume).collect());
    }

}

#[derive(Clone)]
pub struct AudioMixer {
    playing: Arc<Mutex<Vec<SoundInstance>>>,
    master_volume: Arc<Mutex<f32>>,
    mute: Arc<Mutex<bool>>,
}

impl AudioMixer {
    pub fn new(master_volume: f32) -> AudioMixer {
        AudioMixer {
            playing: Arc::new(Mutex::new(Vec::new())),
            master_volume: Arc::new(Mutex::new(master_volume)),
            mute: Arc::new(Mutex::new(false)),
        }
    }

    pub fn play_sound(&mut self, sound: SoundInstance) -> usize {
        let mut vector = self.playing.lock().unwrap();
        vector.push(sound);
        vector.len() - 1
    }

    pub fn replace_sound(&mut self, sound: SoundInstance, id: usize) -> usize {
        let mut vector = self.playing.lock().unwrap();
        while vector.len() <= id {
            vector.push(SoundInstance::new(Vec::new(), 0))
        }
        vector[id] = sound;
        id
    }

    pub fn set_volume(&mut self, volume: f32, id: usize) {
        let mut vector = self.playing.lock().unwrap();
        vector[id].set_volume(volume);
    }

    pub fn pause(&mut self, id: usize) {
        let mut vector = self.playing.lock().unwrap();
        vector[id].pause();
    }

    pub fn stop(&mut self, id: usize) {
        let mut vector = self.playing.lock().unwrap();
        vector[id].stop();
    }

    pub fn stop_repetition(&mut self, id: usize) {
        let mut vector = self.playing.lock().unwrap();
        vector[id].stop_repetition();
    }

    pub fn play(&mut self, id: usize) {
        let mut vector = self.playing.lock().unwrap();
        vector[id].play();
    }

    pub fn toggle_pause(&mut self, id: usize) {
        let mut vector = self.playing.lock().unwrap();
        vector[id].toggle_pause();
    }

    pub fn is_done(&self, id: usize) -> bool {
        let vector = self.playing.lock().unwrap();
        if id >= vector.len() {
            return true;
        }
        vector[id].is_done
    }

    pub fn is_playing(&self, id: usize) -> bool {
        let vector = self.playing.lock().unwrap();
        if id >= vector.len() {
            return false;
        }
        vector[id].is_playing()
    }

    pub fn reset(&mut self) {
        self.playing.lock().unwrap().clear();
    }

    pub fn set_master_volume(&mut self, volume: f32) {
        *self.master_volume.lock().unwrap() = volume;
    }

    pub fn set_mute(&mut self, mute: bool) {
        let mut self_mute = self.mute.lock().unwrap();
        *self_mute = mute;
    }

    pub fn toggle_mute(&mut self) {
        let mut self_mute = self.mute.lock().unwrap();
        *self_mute = !*self_mute;
    }

    pub fn change_volume(&mut self, volume_diff: f32) {
        let mut master_volume = self.master_volume.lock().unwrap();
        *master_volume += volume_diff;

        if *master_volume < 0.0 {
            *master_volume = 0.0;
        }

        if *master_volume > 2.0 {
            *master_volume = 2.0;
        }
    }

    pub fn is_mute(&self) -> bool {
        return *self.mute.lock().unwrap();
    }

    pub fn get_master_volume(&self) -> f32 {
        return *self.master_volume.lock().unwrap();
    }
}

impl AudioCallback for AudioMixer {
    type Channel = f32;

    fn callback(&mut self, out: &mut [f32]) {
        let mut sounds = self.playing.lock().unwrap();


        let mut samples = Vec::new();
        for sound_instance in sounds.iter_mut() {
            match sound_instance.request_samples(out.len()) {
                None => {},
                Some(sound_samples) => samples.push(sound_samples),
            }

        }
        for (i, out_i) in out.iter_mut().enumerate() {
            *out_i = 0.0;
            if self.is_mute() {
                continue;
            }
            for s in samples.iter() {
                *out_i += self.get_master_volume() * (*s)[i];
            }
        }

       for sound in &mut *sounds {
           if sound.is_done {
               sound.clear();
           }
       }
    }
}


pub struct AudioEngine {
    _audio_device: AudioDevice<AudioMixer>,
    mixer: AudioMixer,
    _sound_map: HashMap<u64, Vec<f32>>,
}


impl AudioEngine {
    pub fn new(audio_subsystem: AudioSubsystem) -> AudioEngine {
        let desired_spec = AudioSpecDesired {
            freq: Some(44_100),
            channels: Some(1), // mono
            samples: None      // default
        };

        let mixer = AudioMixer::new(1.0);

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
            mixer,
            _sound_map: HashMap::new(),
        }
    }

    pub fn replace_sound<T: Hash>(&mut self, key: T, id: usize, repeats: i32) -> Result<usize, Error> {
        let pcm_mono_float = self._sound_map.get(&self.get_hash(key)).unwrap().to_vec();
        Ok(self.mixer.replace_sound(SoundInstance::new(pcm_mono_float, repeats), id))
    }

    pub fn play_sound<T: Hash>(&mut self, key: T) -> Result<usize, Error> {
        self.loop_sound(key, 0)
    }

    pub fn prepare_sound<T: Hash>(&mut self, key: T) -> Result<usize, Error> {
        let id = self.play_sound(key)?;
        self.pause(id);
        Ok(id)
    }

    pub fn loop_sound<T: Hash>(&mut self, key: T, repeats:i32) -> Result<usize, Error> {
        let pcm_mono_float = self._sound_map.get(&self.get_hash(key)).unwrap().to_vec();
        let id = self.loop_sound_data(pcm_mono_float, repeats);
        Ok(id)
    }

    pub fn pre_load_files<T: Hash + Eq>(&mut self, file_map: HashMap<T, &str>) -> Result<(), Error> {
        for (key, filename) in file_map {
            self.pre_load_file(key, filename).unwrap();
        }
        Ok(())

    }
    pub fn pre_load_file<T: Hash>(&mut self, key: T,  filename: &str) -> Result<(), Error> {
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
        let hash = self.get_hash(key);
        self._sound_map.insert(hash, pcm_mono_float);
        Ok(())
    }

    fn get_hash<T: Hash>(&self, t: T) -> u64 {
        let mut hasher = DefaultHasher::new();
        t.hash(&mut hasher);
        hasher.finish()
    }

    pub fn play_sound_data(&mut self, data: Vec<f32>) -> usize {
        self.mixer.play_sound(SoundInstance::new(data, 0))
    }

    pub fn loop_sound_data(&mut self, data: Vec<f32>, repeats: i32) -> usize {
        self.mixer.play_sound(SoundInstance::new(data, repeats))
    }

    pub fn set_volume(&mut self, volume: f32, id: usize) {
        self.mixer.set_volume(volume, id);
    }

    pub fn pause(&mut self, id: usize) {
        self.mixer.pause(id);
    }

    pub fn stop(&mut self, id: usize) {
        self.mixer.stop(id);
    }

    pub fn stop_repetition(&mut self, id: usize) {
        self.mixer.stop_repetition(id);
    }

    pub fn play(&mut self, id: usize) {
        self.mixer.play(id);
    }

    pub fn toggle_pause(&mut self, id: usize) {
        self.mixer.toggle_pause(id);
    }

    pub fn is_done(&self, id: usize) -> bool {
        self.mixer.is_done(id)
    }

    pub fn is_playing(&self, id: usize) -> bool {
        self.mixer.is_playing(id)
    }

    pub fn reset(&mut self) -> Result<(), Error> {
        self.mixer.reset();
        Ok(())
    }

    pub fn increase_volume(&mut self, diff: f32) {
        self.mixer.change_volume(diff);
    }

    pub fn decrease_volume(&mut self, diff: f32) {
        self.mixer.change_volume(-diff);
    }

    pub fn mute_volume(&mut self) {
        self.mixer.set_mute(true);
    }

    pub fn unmute_volume(&mut self) {
        self.mixer.set_mute(false);
    }

    pub fn toggle_mute(&mut self) {
        self.mixer.toggle_mute();
    }

}
