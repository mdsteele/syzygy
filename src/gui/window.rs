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
use sdl2::rect::Rect;
use sdl2::render::Renderer;
use super::canvas::Canvas;
use super::element::Element;
use super::event::Event;
use super::resources::{Resources, ResourceCache};

// ========================================================================= //

pub struct Window {
    _video_subsystem: VideoSubsystem,
    renderer: Renderer<'static>,
    full_rect: Rect,
    event_pump: EventPump,
    resource_cache: ResourceCache,
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
        Window {
            _video_subsystem: video_subsystem,
            renderer: renderer,
            full_rect: Rect::new(offset_x, offset_y, full_width, full_height),
            event_pump: sdl_context.event_pump().unwrap(),
            resource_cache: ResourceCache::new(),
        }
    }

    pub fn render<S, E: Element<S>>(&mut self, state: &S, view: &E) {
        {
            let mut canvas = Canvas::new(&mut self.renderer, self.full_rect);
            view.draw(state, &mut canvas);
        }
        self.renderer.present();
    }

    /// Blocks until the next event is available.
    pub fn next_event(&mut self) -> Event {
        loop {
            match Event::from_sdl2(&self.event_pump.wait_event()) {
                Some(event) => return event,
                None => {}
            }
        }
    }

    pub fn resources(&mut self) -> Resources {
        Resources::new(&self.renderer, &mut self.resource_cache)
    }
}

// ========================================================================= //
