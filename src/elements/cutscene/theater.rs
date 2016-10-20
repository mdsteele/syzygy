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

use std::collections::BTreeMap;
use std::rc::Rc;

use elements::Paragraph;
use gui::{Background, Canvas, Point, Rect, Sprite};

// ========================================================================= //

pub struct Theater {
    background: Rc<Background>,
    actors: BTreeMap<i32, Actor>,
    dark: bool,
}

impl Theater {
    pub fn new(background: Rc<Background>) -> Theater {
        Theater {
            background: background,
            actors: BTreeMap::new(),
            dark: false,
        }
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

    pub fn set_actor_speech(&mut self, slot: i32,
                            speech: Option<Rc<Paragraph>>) {
        if let Some(actor) = self.actors.get_mut(&slot) {
            actor.set_speech(speech);
        }
    }

    pub fn set_actor_light(&mut self, slot: i32, light: Option<Sprite>) {
        if let Some(actor) = self.actors.get_mut(&slot) {
            actor.light = light;
        }
    }

    pub fn set_dark(&mut self, dark: bool) { self.dark = dark; }

    pub fn draw_background(&self, canvas: &mut Canvas) {
        for (&index, actor) in self.actors.iter() {
            if index >= 0 {
                break;
            }
            actor.draw_actor(canvas);
        }
        canvas.draw_background(&self.background);
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

    fn set_speech(&mut self, paragraph: Option<Rc<Paragraph>>) {
        self.speech = paragraph.map(|p| SpeechBubble::new(p, self.rect()));
    }

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
    paragraph: Rc<Paragraph>,
    rect: Rect,
}

impl SpeechBubble {
    fn new(paragraph: Rc<Paragraph>, actor_rect: Rect) -> SpeechBubble {
        let width = paragraph.min_width() + 2 * SPEECH_MARGIN;
        let height = paragraph.height() as i32 + 2 * SPEECH_MARGIN;
        let left = actor_rect.left() - (width - actor_rect.width() as i32) / 2;
        let top = actor_rect.top() - height - 8;
        SpeechBubble {
            paragraph: paragraph,
            rect: Rect::new(left, top, width as u32, height as u32),
        }
    }

    fn draw(&self, canvas: &mut Canvas) {
        let mut canvas = canvas.subcanvas(self.rect);
        canvas.clear((255, 255, 255));
        let subrect = Rect::new(SPEECH_MARGIN,
                                SPEECH_MARGIN,
                                self.rect.width() - 2 * SPEECH_MARGIN as u32,
                                self.rect.height() - 2 * SPEECH_MARGIN as u32);
        self.paragraph.draw(&mut canvas.subcanvas(subrect));
    }
}

// ========================================================================= //
