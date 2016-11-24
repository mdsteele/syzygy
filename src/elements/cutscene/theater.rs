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

use std::cmp;
use std::collections::BTreeMap;
use std::mem;
use std::rc::Rc;

use elements::Paragraph;
use gui::{Background, Canvas, Point, Rect, Sound, Sprite};
use save::Direction;

// ========================================================================= //

pub struct Theater {
    background: Option<Rc<Background>>,
    actors: BTreeMap<i32, Actor>,
    queue: Vec<(i32, i32)>,
    sounds: Vec<Sound>,
    dark: bool,
}

impl Theater {
    pub fn new() -> Theater {
        Theater {
            background: None,
            actors: BTreeMap::new(),
            queue: Vec::new(),
            sounds: Vec::new(),
            dark: false,
        }
    }

    pub fn reset(&mut self) {
        self.background = None;
        self.actors.clear();
        self.queue.clear();
        self.dark = false;
    }

    pub fn set_background(&mut self, background: Rc<Background>) {
        self.background = Some(background);
    }

    pub fn place_actor(&mut self, slot: i32, sprite: Sprite, position: Point) {
        self.actors.insert(slot, Actor::new(sprite, position));
    }

    pub fn remove_actor(&mut self, slot: i32) { self.actors.remove(&slot); }

    pub fn get_actor_position(&mut self, slot: i32) -> Option<Point> {
        self.actors.get(&slot).map(|actor| actor.position)
    }

    pub fn set_actor_position(&mut self, slot: i32, position: Point) {
        if let Some(actor) = self.actors.get_mut(&slot) {
            actor.position = position;
        }
    }

    pub fn set_actor_speech(&mut self, slot: i32, bubble_sprites: Vec<Sprite>,
                            bg_color: (u8, u8, u8), talk_pos: TalkPos,
                            paragraph: Rc<Paragraph>) {
        if let Some(actor) = self.actors.get_mut(&slot) {
            actor.set_speech(bubble_sprites, bg_color, talk_pos, paragraph);
        }
    }

    pub fn clear_actor_speech(&mut self, slot: i32) {
        if let Some(actor) = self.actors.get_mut(&slot) {
            actor.clear_speech();
        }
    }

    pub fn set_actor_light(&mut self, slot: i32, light: Option<Sprite>) {
        if let Some(actor) = self.actors.get_mut(&slot) {
            actor.light = light;
        }
    }

    pub fn enqueue(&mut self, entry: (i32, i32)) { self.queue.push(entry); }

    pub fn drain_queue(&mut self) -> Vec<(i32, i32)> {
        mem::replace(&mut self.queue, Vec::new())
    }

    pub fn add_sound(&mut self, sound: Sound) { self.sounds.push(sound); }

    pub fn drain_sounds(&mut self) -> Vec<Sound> {
        mem::replace(&mut self.sounds, Vec::new())
    }

    pub fn set_dark(&mut self, dark: bool) { self.dark = dark; }

    pub fn draw_background(&self, canvas: &mut Canvas) {
        let bg_color = if let Some(ref background) = self.background {
            background.color()
        } else {
            (255, 255, 255)
        };
        canvas.clear(bg_color);
        for (&index, actor) in self.actors.iter() {
            if index >= 0 {
                break;
            }
            actor.draw_actor(canvas);
        }
        if let Some(ref background) = self.background {
            canvas.draw_background(background);
        }
    }

    pub fn draw_foreground(&self, canvas: &mut Canvas) {
        for (&index, actor) in self.actors.iter() {
            if index >= 0 {
                actor.draw_actor(canvas);
            }
        }
        if self.dark {
            let mut rects = vec![canvas.rect()];
            for actor in self.actors.values() {
                if let Some(ref sprite) = actor.light {
                    let mut rect = sprite.rect();
                    rect.center_on(actor.rect().center());
                    canvas.draw_sprite(&sprite, rect.top_left());
                    remove_rect(&mut rects, rect);
                }
            }
            for rect in rects {
                canvas.fill_rect((0, 0, 0), rect);
            }
        }
    }

    pub fn draw_speech_bubbles(&self, canvas: &mut Canvas) {
        for actor in self.actors.values() {
            actor.draw_speech(canvas);
        }
    }
}

fn remove_rect(rects: &mut Vec<Rect>, remove: Rect) {
    let mut new_rects = Vec::new();
    for &rect in rects.iter() {
        if let Some(inter) = rect.intersection(remove) {
            if inter.top() > rect.top() {
                new_rects.push(Rect::new(rect.left(),
                                         rect.top(),
                                         rect.width(),
                                         (inter.top() - rect.top()) as u32));
            }
            if inter.bottom() < rect.bottom() {
                new_rects.push(Rect::new(rect.left(), inter.bottom(),
                                         rect.width(),
                                         (rect.bottom() -
                                          inter.bottom()) as u32));
            }
            if inter.left() > rect.left() {
                new_rects.push(Rect::new(rect.left(),
                                         inter.top(),
                                         (inter.left() - rect.left()) as u32,
                                         inter.height()));
            }
            if inter.right() < rect.right() {
                new_rects.push(Rect::new(inter.right(), inter.top(),
                                         (rect.right() - inter.right()) as u32,
                                         inter.height()));
            }
        } else {
            new_rects.push(rect);
        }
    }
    *rects = new_rects;
}

// ========================================================================= //

struct Actor {
    sprite: Sprite,
    position: Point,
    light: Option<Sprite>,
    speech: Option<SpeechBubble>,
}

impl Actor {
    fn new(sprite: Sprite, position: Point) -> Actor {
        Actor {
            sprite: sprite,
            position: position,
            light: None,
            speech: None,
        }
    }

    fn rect(&self) -> Rect {
        Rect::new(self.position.x() - self.sprite.width() as i32 / 2,
                  self.position.y() - self.sprite.height() as i32,
                  self.sprite.width(),
                  self.sprite.height())
    }

    fn set_speech(&mut self, bubble_sprites: Vec<Sprite>,
                  bg_color: (u8, u8, u8), talk_pos: TalkPos,
                  paragraph: Rc<Paragraph>) {
        self.speech = Some(SpeechBubble::new(bubble_sprites,
                                             bg_color,
                                             talk_pos,
                                             paragraph,
                                             self.rect()));
    }

    fn clear_speech(&mut self) { self.speech = None; }

    fn draw_actor(&self, canvas: &mut Canvas) {
        canvas.draw_sprite(&self.sprite, self.rect().top_left());
    }

    fn draw_speech(&self, canvas: &mut Canvas) {
        if let Some(ref speech) = self.speech {
            speech.draw(canvas);
        }
    }
}

// ========================================================================= //

const SPEECH_MARGIN: i32 = 8;

struct SpeechBubble {
    sprites: Vec<Sprite>,
    bg_color: (u8, u8, u8),
    paragraph: Rc<Paragraph>,
    rect: Rect,
    tail_pos: Point,
    tail_dir: Direction,
    tail_flip_vert: bool,
}

impl SpeechBubble {
    fn new(bubble_sprites: Vec<Sprite>, bg_color: (u8, u8, u8),
           positioning: TalkPos, paragraph: Rc<Paragraph>, actor_rect: Rect)
           -> SpeechBubble {
        debug_assert_eq!(bubble_sprites.len(), 5);
        let width = round_up_to_16(cmp::max(48,
                                            paragraph.min_width() +
                                            2 * SPEECH_MARGIN));
        let height = round_up_to_16(cmp::max(32,
                                             paragraph.height() as i32 +
                                             2 * SPEECH_MARGIN));
        let tail_x = match positioning {
            TalkPos::NW | TalkPos::NE | TalkPos::SW | TalkPos::SE => {
                actor_rect.left() + actor_rect.width() as i32 / 2
            }
            TalkPos::W => actor_rect.left() - 10,
            TalkPos::E => actor_rect.right() + 10,
        };
        let tail_y = match positioning {
            TalkPos::W | TalkPos::E => {
                actor_rect.top() + actor_rect.height() as i32 / 2
            }
            TalkPos::NW | TalkPos::NE => actor_rect.top() - 10,
            TalkPos::SW | TalkPos::SE => actor_rect.bottom() + 10,
        };
        let left = match positioning {
            TalkPos::NW | TalkPos::SW => tail_x + 16 - width,
            TalkPos::NE | TalkPos::SE => tail_x - 16,
            TalkPos::W => tail_x - 8 - width,
            TalkPos::E => tail_x + 8,
        };
        let top = match positioning {
            TalkPos::NW | TalkPos::NE => tail_y - 8 - height,
            TalkPos::SW | TalkPos::SE => tail_y + 8,
            TalkPos::W | TalkPos::E => tail_y + 8 - height,
        };
        let tail_dir = match positioning {
            TalkPos::E => Direction::East,
            TalkPos::SW | TalkPos::SE => Direction::South,
            TalkPos::W => Direction::West,
            TalkPos::NW | TalkPos::NE => Direction::North,
        };
        let tail_flip_vert = match positioning {
            TalkPos::NE | TalkPos::SW | TalkPos::W => true,
            TalkPos::NW | TalkPos::SE | TalkPos::E => false,
        };
        SpeechBubble {
            sprites: bubble_sprites,
            bg_color: bg_color,
            paragraph: paragraph,
            rect: Rect::new(left, top, width as u32, height as u32),
            tail_pos: Point::new(tail_x, tail_y),
            tail_dir: tail_dir,
            tail_flip_vert: tail_flip_vert,
        }
    }

    fn draw(&self, canvas: &mut Canvas) {
        // Draw bubble:
        {
            let mut canvas = canvas.subcanvas(self.rect);
            let right = self.rect.width() as i32 - 16;
            let bottom = self.rect.height() as i32 - 16;
            canvas.draw_sprite(&self.sprites[0], Point::new(0, 0));
            canvas.draw_sprite(&self.sprites[1], Point::new(right, 0));
            canvas.draw_sprite_flipped(&self.sprites[1],
                                       Point::new(0, bottom),
                                       true,
                                       true);
            canvas.draw_sprite_flipped(&self.sprites[0],
                                       Point::new(right, bottom),
                                       true,
                                       true);
            for col in 1..(right / 16) {
                let x = 16 * col + 8;
                canvas.draw_sprite_transformed(&self.sprites[2],
                                               Point::new(x, 8),
                                               90,
                                               false,
                                               true);
                canvas.draw_sprite_transformed(&self.sprites[2],
                                               Point::new(x, bottom + 8),
                                               -90,
                                               false,
                                               true);
            }
            for row in 1..(bottom / 16) {
                let y = 16 * row + 8;
                canvas.draw_sprite_centered(&self.sprites[2],
                                            Point::new(8, y));
                canvas.draw_sprite_rotated(&self.sprites[2],
                                           Point::new(right + 8, y),
                                           180);
            }
            canvas.fill_rect(self.bg_color,
                             Rect::new(16,
                                       16,
                                       (right - 16) as u32,
                                       (bottom - 16) as u32));
        }
        // Draw tail:
        canvas.draw_sprite_transformed(&self.sprites[4],
                                       self.tail_pos,
                                       self.tail_dir.degrees(),
                                       false,
                                       self.tail_flip_vert);
        canvas.draw_sprite_transformed(&self.sprites[3],
                                       self.tail_pos +
                                       self.tail_dir.delta() * 16,
                                       self.tail_dir.degrees(),
                                       false,
                                       self.tail_flip_vert);
        // Draw text:
        {
            let width = self.paragraph.min_width();
            let height = self.paragraph.height();
            let subrect = Rect::new(self.rect.x() +
                                    (self.rect.width() as i32 - width) / 2,
                                    self.rect.y() +
                                    (self.rect.height() - height) as i32 / 2,
                                    width as u32,
                                    height);
            self.paragraph.draw(&mut canvas.subcanvas(subrect));
        }
    }
}

fn round_up_to_16(mut size: i32) -> i32 {
    let remainder = size % 16;
    if remainder != 0 {
        size += 16 - remainder;
    }
    size
}

// ========================================================================= //

#[derive(Clone, Copy)]
pub enum TalkPos {
    NE,
    NW,
    E,
    W,
    SE,
    SW,
}

// ========================================================================= //
