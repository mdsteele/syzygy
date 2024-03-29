// +--------------------------------------------------------------------------+
// | Copyright 2016 Matthew D. Steele <mdsteele@alum.mit.edu>                 |
// |                                                                          |
// | This file is part of System Syzygy.                                      |
// |                                                                          |
// | System Syzygy is free software: you can redistribute it and/or modify it |
// | under the terms of the GNU General Public License as published by the    |
// | Free Software Foundation, either version 3 of the License, or (at your   |
// | option) any later version.                                               |
// |                                                                          |
// | System Syzygy is distributed in the hope that it will be useful, but     |
// | WITHOUT ANY WARRANTY; without even the implied warranty of               |
// | MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the GNU        |
// | General Public License for details.                                      |
// |                                                                          |
// | You should have received a copy of the GNU General Public License along  |
// | with System Syzygy.  If not, see <http://www.gnu.org/licenses/>.         |
// +--------------------------------------------------------------------------+

use super::itersynth::{self, Wave, WaveGen};
use sdl2::audio::{AudioCallback, AudioDevice, AudioSpecDesired};
use sdl2::AudioSubsystem;
use std::mem;
use std::sync::{Arc, Mutex};

// ========================================================================= //

const DESIRED_AUDIO_RATE: i32 = 44100; // samples/second
const DESIRED_BUFFER_SIZE: u16 = 2048; // num samples
const DESIRED_NUM_CHANNELS: u8 = 1; // mono

// ========================================================================= //

#[derive(Clone)]
pub struct Sound {
    wave: Wave,
}

impl Sound {
    fn new(wave: Wave) -> Sound {
        Sound { wave }
    }

    pub fn atlatl_beam(stage: i32) -> Sound {
        Sound::new(
            Wave::pulse(
                Wave::from(420.0 - 35.0 * (stage as f32))
                    .adshr(0.0, 0.0, 1.0, 0.0, 1.0),
                0.4,
            ) * 0.25,
        )
    }

    pub fn atlatl_charge(duty: f32) -> Sound {
        Sound::new(
            Wave::triangle(
                Wave::slide(340.0, 100.0, 0.0) + Wave::sine(40.0) * 10.0,
                duty,
            )
            .adshr(0.125, 0.0, 1.0, 0.0, 1.5)
                * 0.5,
        )
    }

    pub fn beep() -> Sound {
        Sound::new(
            Wave::pulse(440.0, 0.5).adshr(0.0, 0.0, 0.25, 0.3, 0.05) * 0.4,
        )
    }

    pub fn bridge_break() -> Sound {
        Sound::new(
            Wave::noise(Wave::slide(400.0, -150.0, 0.0))
                .adshr(0.0, 0.0, 0.75, 0.0, 0.6)
                * Wave::sine(8.0)
                * 0.4,
        )
    }

    pub fn bridge_crack() -> Sound {
        Sound::new(Wave::noise(1000.0).adshr(0.0, 0.0, 0.5, 0.0, 0.1) * 0.4)
    }

    pub fn character_collision() -> Sound {
        Sound::new(Wave::noise(2000.0).adshr(0.0, 0.0, 0.5, 0.0, 0.25) * 0.4)
    }

    pub fn device_drop() -> Sound {
        Sound::new(
            Wave::noise(Wave::from(2000.0) + Wave::from(3000.0).delayed(0.05))
                .adshr(0.0, 0.0, 0.25, 0.0, 0.075)
                * 0.4,
        )
    }

    pub fn device_pickup() -> Sound {
        Sound::new(
            Wave::noise(
                Wave::from(5000.0) + Wave::from(-3000.0).delayed(0.05),
            )
            .adshr(0.0, 0.0, 0.25, 0.0, 0.075)
                * 0.4,
        )
    }

    pub fn device_rotate() -> Sound {
        Sound::new(Wave::noise(8000.0).adshr(0.0, 0.0, 0.25, 0.0, 0.05) * 0.4)
    }

    pub fn device_slide() -> Sound {
        Sound::new(
            Wave::noise(Wave::slide(8000.0, -8000.0, 0.0))
                .adshr(0.05, 0.0, 1.0, 0.0, 0.25)
                * Wave::slide(0.125, -0.5, 0.0),
        )
    }

    pub fn explosion_small() -> Sound {
        Sound::new(
            Wave::noise(Wave::from(1500.0) + Wave::sine(20.0) * 250.0)
                .adshr(0.0, 0.0, 0.5, 0.0, 0.5)
                * 0.5,
        )
    }

    pub fn platform_shift(num_times: i32) -> Sound {
        Sound::new(
            Wave::noise(800.0)
                .adshr(0.071, 0.0, 1.0, 0.0, 0.142)
                .repeated(num_times)
                * 0.1,
        )
    }

    pub fn platform_shift_double(num_times: i32) -> Sound {
        let shift = Wave::noise(800.0)
            .adshr(0.071, 0.0, 1.0, 0.0, 0.142)
            .repeated(num_times)
            * 0.1;
        Sound::new(shift.clone() + shift.delayed(0.1065))
    }

    pub fn small_jump() -> Sound {
        Sound::new(
            Wave::pulse(Wave::slide(200.0, 750.0, 1500.0), 0.25)
                .adshr(0.0, 0.0, 0.25, 0.1, 0.1)
                * 0.4,
        )
    }

    pub fn mid_puzzle_chime() -> Sound {
        let duty = Wave::sine(3.0) * 0.1 + 0.75;
        let c4 = Wave::triangle(261.62, duty.clone())
            .adshr(0.01, 0.1, 0.5, 0.0, 0.2);
        let e4 = Wave::triangle(329.62, duty.clone())
            .adshr(0.01, 0.1, 0.5, 0.0, 0.2);
        let g4 = Wave::triangle(391.99, duty.clone())
            .adshr(0.01, 0.1, 0.5, 0.0, 0.4);
        Sound::new(c4 + e4.delayed(0.05) + g4.delayed(0.1))
    }

    pub fn solve_puzzle_chime() -> Sound {
        let duty = Wave::sine(3.0) * 0.1 + 0.75;
        let c5 = Wave::triangle(523.25, duty.clone());
        let e5 = Wave::triangle(659.25, duty.clone());
        let g5 = Wave::triangle(783.99, duty.clone());
        let c6 = Wave::triangle(1046.50, duty);
        let chord = c5 + e5 + g5 + c6;
        Sound::new(chord.adshr(0.01, 0.1, 0.5, 0.0, 0.75) * 0.3)
    }

    pub fn spawn_zap() -> Sound {
        let freq = (Wave::sine(15.0) * 0.02 + 1.0) * 450.0;
        let tone = Wave::triangle(freq, 0.85);
        Sound::new(tone.adshr(0.15, 0.0, 1.0, 1.0, 0.75) * 0.2)
    }

    pub fn talk_annoyed_hi() -> Sound {
        Sound::new(
            Wave::pulse(Wave::slide(120.0, 200.0, -3000.0), 0.2)
                .adshr(0.0, 0.0, 0.25, 0.25, 0.2)
                * 0.5,
        )
    }

    pub fn talk_annoyed_lo() -> Sound {
        Sound::new(
            Wave::pulse(Wave::slide(100.0, 200.0, -3000.0), 0.2)
                .adshr(0.0, 0.0, 0.25, 0.25, 0.2)
                * 0.5,
        )
    }

    pub fn talk_hi() -> Sound {
        Sound::new(
            Wave::noise(Wave::slide(7000.0, 20000.0, 0.0))
                .adshr(0.05, 0.0, 1.0, 0.0, 0.15)
                * 0.1,
        )
    }

    pub fn talk_lo() -> Sound {
        Sound::new(
            Wave::noise(Wave::slide(7000.0, -20000.0, 0.0))
                .adshr(0.05, 0.0, 1.0, 0.0, 0.15)
                * 0.1,
        )
    }

    pub fn talk_thought() -> Sound {
        Sound::new(
            (Wave::noise(5000.0) * Wave::sine(6.0))
                .adshr(0.0, 0.0, 0.08, 0.1, 0.2),
        )
    }

    pub fn transform_final() -> Sound {
        let freq = Wave::sine(10.0) * 2.0 + 200.0;
        let duty = Wave::sine(0.2) * 0.45 + 0.5;
        Sound::new(Wave::triangle(freq, duty).adshr(0.1, 0.0, 1.0, 0.9, 0.4))
    }

    pub fn transform_step(step: usize) -> Sound {
        let pitch = 275.0 + 25.0 * step as f32;
        let freq = (Wave::sine(10.0) * 0.01 + 1.0) * pitch;
        Sound::new(
            Wave::triangle(freq, 0.85).adshr(0.05, 0.0, 1.0, 0.0, 0.4) * 0.2,
        )
    }

    pub fn undo() -> Sound {
        Sound::new(
            Wave::pulse(Wave::from(-30.0).delayed(0.06) + 150.0, 0.5)
                .adshr(0.0, 0.0, 0.1, 0.1, 0.25),
        )
    }

    pub fn redo() -> Sound {
        Sound::new(
            Wave::pulse(Wave::from(30.0).delayed(0.06) + 120.0, 0.5)
                .adshr(0.0, 0.0, 0.1, 0.1, 0.25),
        )
    }

    pub fn reset() -> Sound {
        Sound::new(
            Wave::pulse(
                Wave::slide(135.0, -30.0, 0.0) + Wave::pulse(8.0, 0.5) * 15.0,
                0.5,
            )
            .adshr(0.0, 0.0, 0.1, 0.1, 0.75),
        )
    }
}

// ========================================================================= //

pub struct SoundQueue {
    queue: Mutex<Vec<Sound>>,
}

impl SoundQueue {
    pub fn new() -> SoundQueue {
        SoundQueue { queue: Mutex::new(Vec::new()) }
    }

    pub fn enqueue(&self, mut sounds: Vec<Sound>) {
        if !sounds.is_empty() {
            let mut vec = self.queue.lock().unwrap();
            vec.append(&mut sounds);
        }
    }

    pub fn drain(&self) -> Vec<Sound> {
        let mut vec = self.queue.lock().unwrap();
        mem::replace(&mut vec as &mut Vec<Sound>, Vec::new())
    }
}

// ========================================================================= //

pub struct SoundMixer {
    time_step: f32,
    sound_queue: Arc<SoundQueue>,
    active_sounds: Vec<Sound>,
}

impl SoundMixer {
    fn new(audio_rate: i32, sound_queue: Arc<SoundQueue>) -> SoundMixer {
        SoundMixer {
            time_step: 1.0 / audio_rate as f32,
            sound_queue,
            active_sounds: Vec::new(),
        }
    }

    pub fn audio_device(
        audio_subsystem: &AudioSubsystem,
        sound_queue: Arc<SoundQueue>,
    ) -> AudioDevice<SoundMixer> {
        let desired_audio_spec = AudioSpecDesired {
            freq: Some(DESIRED_AUDIO_RATE),
            channels: Some(DESIRED_NUM_CHANNELS),
            samples: Some(DESIRED_BUFFER_SIZE),
        };
        audio_subsystem
            .open_playback(None, &desired_audio_spec, |spec| {
                SoundMixer::new(spec.freq, sound_queue)
            })
            .unwrap()
    }
}

impl AudioCallback for SoundMixer {
    type Channel = itersynth::Sample;

    fn callback(&mut self, out: &mut [itersynth::Sample]) {
        let mut new_sounds = self.sound_queue.drain();
        self.active_sounds.append(&mut new_sounds);
        for sample in out.iter_mut() {
            *sample = 0.0;
        }
        debug_assert!(new_sounds.is_empty());
        'sounds: for mut sound in self.active_sounds.drain(..) {
            for sample in out.iter_mut() {
                if let Some(value) = sound.wave.next(self.time_step) {
                    *sample += value;
                } else {
                    continue 'sounds;
                }
            }
            new_sounds.push(sound);
        }
        debug_assert!(self.active_sounds.is_empty());
        self.active_sounds = new_sounds;
    }
}

// ========================================================================= //
