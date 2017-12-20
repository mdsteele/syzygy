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

use ahi;
use sdl2::render::Renderer;
use std::collections::HashMap;
use std::fs::File;
use std::io;
use std::path::{Path, PathBuf};
use std::rc::Rc;

use super::background::Background;
use super::font::Font;
use super::sprite::Sprite;

// ========================================================================= //

pub struct Resources<'a> {
    renderer: &'a Renderer<'static>,
    cache: &'a mut ResourceCache,
}

impl<'a> Resources<'a> {
    pub fn new(renderer: &'a Renderer<'static>, cache: &'a mut ResourceCache)
               -> Resources<'a> {
        Resources {
            renderer: renderer,
            cache: cache,
        }
    }

    pub fn get_background(&mut self, name: &str) -> Rc<Background> {
        self.cache.get_background(self.renderer, name)
    }

    pub fn get_font(&mut self, name: &str) -> Rc<Font> {
        self.cache.get_font(self.renderer, name)
    }

    pub fn get_sprites(&mut self, name: &str) -> Vec<Sprite> {
        self.cache.get_sprites(self.renderer, name)
    }
}

// ========================================================================= //

pub struct ResourceCache {
    backgrounds: HashMap<String, Rc<Background>>,
    fonts: HashMap<String, Rc<Font>>,
    sprites: HashMap<String, Vec<Sprite>>,
}

impl ResourceCache {
    pub fn new() -> ResourceCache {
        ResourceCache {
            backgrounds: HashMap::new(),
            fonts: HashMap::new(),
            sprites: HashMap::new(),
        }
    }

    fn get_background(&mut self, renderer: &Renderer, name: &str)
                      -> Rc<Background> {
        if let Some(background) = self.backgrounds.get(name) {
            return background.clone();
        }
        if cfg!(debug_assertions) {
            println!("Loading background: {}", name);
        }
        let path =
            PathBuf::from("data/backgrounds").join(name).with_extension("bg");
        let background = Rc::new(
            Background::load(&path, |name| {
                self.get_sprites(renderer, &format!("tiles/{}", name))
            }).expect(name),
        );
        self.backgrounds.insert(name.to_string(), background.clone());
        background
    }

    fn get_font(&mut self, renderer: &Renderer, name: &str) -> Rc<Font> {
        if let Some(font) = self.fonts.get(name) {
            return font.clone();
        }
        if cfg!(debug_assertions) {
            println!("Loading font: {}", name);
        }
        let path =
            PathBuf::from("data/fonts").join(name).with_extension("ahf");
        let ahf = load_ahf_from_file(&path).expect(name);
        let font = Rc::new(Font::new(renderer, &ahf));
        self.fonts.insert(name.to_string(), font.clone());
        font
    }

    fn get_sprites(&mut self, renderer: &Renderer, name: &str) -> Vec<Sprite> {
        if let Some(vec) = self.sprites.get(name) {
            return vec.clone();
        }
        if cfg!(debug_assertions) {
            println!("Loading sprites: {}", name);
        }
        let path =
            PathBuf::from("data/sprites").join(name).with_extension("ahi");
        let ahi = load_ahi_from_file(&path).expect(name);
        let vec: Vec<Sprite> =
            ahi.iter().map(|image| Sprite::new(renderer, image)).collect();
        self.sprites.insert(name.to_string(), vec.clone());
        vec
    }
}

// ========================================================================= //

fn load_ahf_from_file(path: &Path) -> io::Result<ahi::Font> {
    let mut file = File::open(path)?;
    ahi::Font::read(&mut file)
}

fn load_ahi_from_file(path: &Path) -> io::Result<Vec<ahi::Image>> {
    let mut file = File::open(path)?;
    ahi::Image::read_all(&mut file)
}

// ========================================================================= //
