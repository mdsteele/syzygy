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

use sdl2::{EventPump, Sdl, VideoSubsystem};
use sdl2::rect::{Point, Rect};
use sdl2::render::Renderer;
use sdl2::video::FullscreenType;
use std::rc::Rc;

use super::canvas::{Align, Canvas};
use super::element::Element;
use super::event::Event;
use super::font::Font;
use super::resources::{Resources, ResourceCache};

// ========================================================================= //

pub struct Window {
    _video_subsystem: VideoSubsystem,
    renderer: Renderer<'static>,
    full_rect: Rect,
    event_pump: EventPump,
    resource_cache: ResourceCache,
    debug_font: Option<Rc<Font>>,
    debug_counter: i32,
}

impl Window {
    pub fn new(sdl_context: &Sdl, title: &str, full_size: (u32, u32),
               ideal_size: (u32, u32), force_ideal: bool, fullscreen: bool)
               -> Window {
        let (full_width, full_height) = full_size;
        let (ideal_width, ideal_height) = ideal_size;
        let video_subsystem = sdl_context.video().unwrap();
        let sdl_window = if fullscreen {
            video_subsystem.window(title, ideal_width, ideal_height)
                           .position_centered()
                           .fullscreen_desktop()
                           .build()
                           .unwrap()
        } else {
            video_subsystem.window(title, ideal_width, ideal_height)
                           .position_centered()
                           .build()
                           .unwrap()
        };
        let (actual_width, actual_height) = if force_ideal {
            ideal_size
        } else {
            let (native_width, native_height) = sdl_window.size();
            let aspect_ratio = (native_width as f64) / (native_height as f64);
            let ideal_ratio = (ideal_width as f64) / (ideal_height as f64);
            if aspect_ratio > ideal_ratio {
                let actual_width =
                    (aspect_ratio * (ideal_height as f64)).round() as u32;
                (actual_width, ideal_height)
            } else {
                let actual_height =
                    ((ideal_width as f64) / aspect_ratio).round() as u32;
                (ideal_width, actual_height)
            }
        };
        let mut renderer = sdl_window.renderer()
                                     .present_vsync()
                                     .build()
                                     .unwrap();
        renderer.set_logical_size(actual_width, actual_height).unwrap();
        let offset_x = (actual_width as i32 - full_width as i32) / 2;
        let offset_y = (actual_height as i32 - full_height as i32) / 2;
        let mut resource_cache = ResourceCache::new();
        let debug_font = if cfg!(debug_assertions) {
            let mut resources = Resources::new(&renderer, &mut resource_cache);
            Some(resources.get_font("debug"))
        } else {
            None
        };
        Window {
            _video_subsystem: video_subsystem,
            renderer: renderer,
            full_rect: Rect::new(offset_x, offset_y, full_width, full_height),
            event_pump: sdl_context.event_pump().unwrap(),
            resource_cache: resource_cache,
            debug_font: debug_font,
            debug_counter: 0,
        }
    }

    pub fn visible_rect(&self) -> Rect {
        let (width, height) = self.renderer.logical_size();
        Rect::new(-self.full_rect.x(), -self.full_rect.y(), width, height)
    }

    pub fn is_fullscreen(&self) -> bool {
        self.renderer.window().unwrap().fullscreen_state() !=
        FullscreenType::Off
    }

    pub fn set_fullscreen(&mut self, fullscreen: bool) {
        if fullscreen != self.is_fullscreen() {
            let state = if fullscreen {
                FullscreenType::Desktop
            } else {
                FullscreenType::Off
            };
            if cfg!(debug_assertions) {
                println!("Setting fullscreen to {:?}.", state);
            }
            self.renderer.window_mut().unwrap().set_fullscreen(state).unwrap();
        }
    }

    pub fn render<S, A, E: Element<S, A>>(&mut self, state: &S, view: &E) {
        {
            let mut canvas = Canvas::new(&mut self.renderer, self.full_rect);
            view.draw(state, &mut canvas);
        }
        if cfg!(debug_assertions) {
            let visible = self.visible_rect();
            let mut canvas = Canvas::new(&mut self.renderer, self.full_rect);
            if let Some(ref font) = self.debug_font {
                canvas.fill_rect((0, 0, 0),
                                 Rect::new(visible.right() - 24,
                                           visible.bottom() - 12,
                                           22,
                                           10));
                canvas.draw_text(&font,
                                 Align::Right,
                                 Point::new(visible.right() - 2,
                                            visible.bottom() - 3),
                                 &format!("{:03}", self.debug_counter));
                self.debug_counter = (self.debug_counter + 1) % 1000;
            }
        }
        self.renderer.present();
    }

    /// Blocks until the next event is available.
    pub fn next_event(&mut self) -> Event {
        loop {
            match Event::from_sdl2(&self.event_pump.wait_event()) {
                Some(event) => {
                    return event.translate(-self.full_rect.x(),
                                           -self.full_rect.y())
                }
                None => {}
            }
        }
    }

    pub fn resources(&mut self) -> Resources {
        Resources::new(&self.renderer, &mut self.resource_cache)
    }
}

// ========================================================================= //
