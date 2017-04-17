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

use std::mem;

use super::sound::Sound;

// ========================================================================= //

pub struct Action<A> {
    redraw: bool,
    sounds: Vec<Sound>,
    value: Value<A>,
}

impl<A> Action<A> {
    pub fn ignore() -> Action<A> {
        Action {
            redraw: false,
            sounds: Vec::new(),
            value: Value::Continue,
        }
    }

    pub fn redraw() -> Action<A> {
        Action {
            redraw: true,
            sounds: Vec::new(),
            value: Value::Continue,
        }
    }

    pub fn redraw_if(redraw: bool) -> Action<A> {
        Action {
            redraw: redraw,
            sounds: Vec::new(),
            value: Value::Continue,
        }
    }

    pub fn and_play_sound(mut self, sound: Sound) -> Action<A> {
        self.sounds.push(sound);
        self
    }

    pub fn and_play_sounds(mut self, mut sounds: Vec<Sound>) -> Action<A> {
        self.sounds.append(&mut sounds);
        self
    }

    pub fn also_play_sound(&mut self, sound: Sound) {
        self.sounds.push(sound);
    }

    pub fn and_stop(mut self) -> Action<A> {
        self.value = Value::Stop;
        self
    }

    pub fn and_return(mut self, value: A) -> Action<A> {
        self.value = Value::Return(value);
        self
    }

    pub fn but_return<B>(self, value: B) -> Action<B> {
        Action {
            redraw: self.redraw,
            sounds: self.sounds,
            value: Value::Return(value),
        }
    }

    pub fn but_no_value<B>(self) -> Action<B> {
        Action {
            redraw: self.redraw,
            sounds: self.sounds,
            value: match self.value {
                Value::Continue => Value::Continue,
                _ => Value::Stop,
            },
        }
    }

    pub fn should_redraw(&self) -> bool { self.redraw }

    pub fn should_stop(&self) -> bool {
        match self.value {
            Value::Continue => false,
            _ => true,
        }
    }

    pub fn value(&self) -> Option<&A> {
        match self.value {
            Value::Return(ref value) => Some(value),
            _ => None,
        }
    }

    pub fn take_value(&mut self) -> Option<A> {
        match self.value {
            Value::Continue | Value::Stop => return None,
            Value::Return(_) => {}
        }
        match mem::replace(&mut self.value, Value::Stop) {
            Value::Return(value) => Some(value),
            _ => unreachable!(),
        }
    }

    pub fn merge(&mut self, mut action: Action<A>) {
        self.redraw |= action.redraw;
        self.sounds.append(&mut action.sounds);
        self.value.merge(action.value);
    }

    #[allow(dead_code)]
    pub fn map<B, F: FnOnce(A) -> B>(self, f: F) -> Action<B> {
        Action {
            redraw: self.redraw,
            sounds: self.sounds,
            value: self.value.map(f),
        }
    }

    pub fn drain_sounds(&mut self) -> Vec<Sound> {
        mem::replace(&mut self.sounds, Vec::new())
    }
}

// ========================================================================= //

enum Value<A> {
    Continue,
    Stop,
    Return(A),
}

impl<A> Value<A> {
    fn merge(&mut self, other: Value<A>) {
        match other {
            Value::Continue => {}
            _ => {
                *self = other;
            }
        }
    }

    fn map<B, F: FnOnce(A) -> B>(self, f: F) -> Value<B> {
        match self {
            Value::Continue => Value::Continue,
            Value::Stop => Value::Stop,
            Value::Return(a) => Value::Return(f(a)),
        }
    }
}

// ========================================================================= //
