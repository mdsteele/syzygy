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

use sdl2;
use sdl2::mouse::Mouse;
use sdl2::rect::Point;
use std::ops::{BitOr, BitOrAssign};

pub use sdl2::keyboard::Keycode;

// ========================================================================= //

struct ClockTick;

#[derive(Clone, Eq, PartialEq)]
pub enum Event {
    Quit,
    ClockTick,
    MouseDrag(Point),
    MouseDown(Point),
    MouseUp,
    KeyDown(Keycode, KeyMod),
    TextInput(String),
}

impl Event {
    pub fn register_clock_ticks(subsystem: &sdl2::EventSubsystem) {
        subsystem.register_custom_event::<ClockTick>().unwrap();
    }

    pub fn push_clock_tick(subsystem: &sdl2::EventSubsystem) {
        subsystem.push_custom_event(ClockTick).unwrap();
    }

    fn from_sdl2(event: &sdl2::event::Event) -> Option<Event> {
        match event {
            &sdl2::event::Event::Quit { .. } => Some(Event::Quit),
            &sdl2::event::Event::MouseMotion { x, y, mousestate, .. } => {
                if mousestate.left() {
                    Some(Event::MouseDrag(Point::new(x, y)))
                } else {
                    None
                }
            }
            &sdl2::event::Event::MouseButtonDown { mouse_btn: Mouse::Left,
                                                   x, y, .. } => {
              Some(Event::MouseDown(Point::new(x, y)))
            }
            &sdl2::event::Event::MouseButtonUp { mouse_btn: Mouse::Left,
                                                 .. } => {
              Some(Event::MouseUp)
            }
            &sdl2::event::Event::KeyDown { keycode: Some(keycode),
                                           keymod,
                                           .. } => {
                Some(Event::KeyDown(keycode, KeyMod::from_sdl2(keymod)))
            }
            &sdl2::event::Event::TextInput { ref text, .. } => {
                Some(Event::TextInput(text.clone()))
            }
            &sdl2::event::Event::User { .. }
                if event.as_user_event_type::<ClockTick>()
                        .is_some() => Some(Event::ClockTick),
            _ => None,
        }
    }

    pub fn translate(&self, dx: i32, dy: i32) -> Event {
        match self {
            &Event::MouseDrag(pt) => Event::MouseDrag(pt.offset(dx, dy)),
            &Event::MouseDown(pt) => Event::MouseDown(pt.offset(dx, dy)),
            _ => self.clone(),
        }
    }
}

// ========================================================================= //

pub struct EventQueue {
    pump: sdl2::EventPump,
}

impl EventQueue {
    pub fn new(sdl_context: &sdl2::Sdl) -> EventQueue {
        EventQueue { pump: sdl_context.event_pump().unwrap() }
    }

    /// Blocks until the next event is available.
    pub fn next(&mut self) -> Event {
        loop {
            match Event::from_sdl2(&self.pump.wait_event()) {
                Some(event) => return event,
                None => {}
            }
        }
    }
}

// ========================================================================= //

#[derive(Clone, Copy, Eq, PartialEq)]
pub struct KeyMod {
    bits: u8,
}

impl KeyMod {
    fn from_sdl2(kmod: sdl2::keyboard::Mod) -> KeyMod {
        let mut result = NONE;

        let sdl2_shift = sdl2::keyboard::LSHIFTMOD | sdl2::keyboard::RSHIFTMOD;
        if kmod.intersects(sdl2_shift) {
            result |= SHIFT;
        }

        let sdl2_alt = sdl2::keyboard::LALTMOD | sdl2::keyboard::RALTMOD;
        if kmod.intersects(sdl2_alt) {
            result |= ALT;
        }

        let sdl2_command = if cfg!(any(target_os = "ios",
                                       target_os = "macos")) {
            sdl2::keyboard::LGUIMOD | sdl2::keyboard::RGUIMOD
        } else {
            sdl2::keyboard::LCTRLMOD | sdl2::keyboard::RCTRLMOD
        };
        if kmod.intersects(sdl2_command) {
            result |= COMMAND;
        }

        result
    }
}

impl BitOr for KeyMod {
    type Output = KeyMod;
    fn bitor(self, rhs: KeyMod) -> KeyMod {
        KeyMod { bits: self.bits | rhs.bits }
    }
}

impl BitOrAssign for KeyMod {
    fn bitor_assign(&mut self, rhs: KeyMod) { self.bits |= rhs.bits; }
}

pub const NONE: KeyMod = KeyMod { bits: 0x0 };
pub const SHIFT: KeyMod = KeyMod { bits: 0x1 };
pub const ALT: KeyMod = KeyMod { bits: 0x2 };
pub const COMMAND: KeyMod = KeyMod { bits: 0x4 };

// ========================================================================= //
