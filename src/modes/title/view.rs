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

use std::rc::Rc;

use crate::gui::{Action, Align, Background, Canvas, Element, Event, Font, Point,
          Rect, Resources, Sound, Sprite};
use crate::elements::{DialogBox, FadeStyle, ScreenFade};
use crate::save::SaveData;

// ========================================================================= //

#[derive(Clone, Copy, Eq, PartialEq)]
pub enum Cmd {
    StartGame,
    EraseGame,
    ShowAboutBox,
    Quit,
}

// ========================================================================= //

pub struct View {
    screen_fade: ScreenFade<Cmd>,
    background: Rc<Background>,
    sun_sprites: Vec<Sprite>,
    xanadu3_sprites: Vec<Sprite>,
    xanadu4_sprites: Vec<Sprite>,
    ship_sprites: Vec<Sprite>,
    buttons: Vec<Button>,
    title_font_1: Rc<Font>,
    title_font_2: Rc<Font>,
}

impl View {
    pub fn new(resources: &mut Resources) -> View {
        let center_x = 288;
        let upper_y = 226;
        let lower_y = 302;
        let spacing = 128;
        View {
            screen_fade: ScreenFade::new(resources,
                                         FadeStyle::Uniform,
                                         FadeStyle::Uniform),
            background: resources.get_background("space"),
            sun_sprites: resources.get_sprites("title/sun"),
            xanadu3_sprites: resources.get_sprites("title/xanadu3"),
            xanadu4_sprites: resources.get_sprites("title/xanadu4"),
            ship_sprites: resources.get_sprites("title/ship"),
            buttons: vec![
                Button::new(resources,
                            Point::new(center_x, upper_y),
                            Cmd::StartGame),
                Button::new(resources,
                            Point::new(center_x - spacing, lower_y),
                            Cmd::ShowAboutBox),
                Button::new(resources,
                            Point::new(center_x, lower_y),
                            Cmd::EraseGame),
                Button::new(resources,
                            Point::new(center_x + spacing, lower_y),
                            Cmd::Quit),
            ],
            title_font_1: resources.get_font("title1"),
            title_font_2: resources.get_font("title2"),
        }
    }

    pub fn reset_buttons(&mut self) {
        for button in self.buttons.iter_mut() {
            button.active = false;
        }
    }
}

impl Element<SaveData, Cmd> for View {
    fn draw(&self, data: &SaveData, canvas: &mut Canvas) {
        canvas.draw_background(&self.background);
        canvas.fill_rect((255, 255, 255), Rect::new(0, 0, 64, 64));
        canvas.draw_sprite(&self.sun_sprites[0], Point::new(64, 0));
        canvas.draw_sprite(&self.sun_sprites[1], Point::new(64, 64));
        canvas.draw_sprite(&self.sun_sprites[2], Point::new(0, 64));
        canvas.draw_sprite_centered(&self.xanadu3_sprites[0],
                                    Point::new(288, 225));
        canvas.draw_sprite_centered(&self.xanadu4_sprites[0],
                                    Point::new(421, 166));
        canvas.draw_sprite(&self.ship_sprites[0], Point::new(0, 256));
        canvas.draw_sprite(&self.ship_sprites[1], Point::new(53, 256));
        canvas.draw_sprite(&self.ship_sprites[2], Point::new(106, 256));
        canvas.draw_text(&self.title_font_1,
                         Align::Center,
                         Point::new(288, 90),
                         "SYSTEM");
        canvas.draw_text(&self.title_font_2,
                         Align::Center,
                         Point::new(288, 165),
                         "SYZYGY");
        self.buttons.draw(data, canvas);
        self.screen_fade.draw(&(), canvas);
    }

    fn handle_event(&mut self, event: &Event, data: &mut SaveData)
                    -> Action<Cmd> {
        let mut action = self.screen_fade.handle_event(event, &mut ());
        if !action.should_stop() {
            let mut subaction = self.buttons.handle_event(event, data);
            if let Some(&cmd) = subaction.value() {
                if cmd == Cmd::StartGame || cmd == Cmd::Quit {
                    self.screen_fade.fade_out_and_return(cmd);
                    subaction = subaction.but_no_value();
                }
            }
            action.merge(subaction);
        }
        action
    }
}

// ========================================================================= //

struct Button {
    sprites: Vec<Sprite>,
    font: Rc<Font>,
    center: Point,
    command: Cmd,
    active: bool,
}

impl Button {
    fn new(resources: &mut Resources, center: Point, command: Cmd) -> Button {
        Button {
            sprites: resources.get_sprites("title/buttons"),
            font: resources.get_font("roman"),
            center: center,
            command: command,
            active: false,
        }
    }

    fn rect(&self) -> Rect {
        Rect::new(self.center.x() - 48, self.center.y() - 12, 96, 24)
    }
}

impl Element<SaveData, Cmd> for Button {
    fn draw(&self, data: &SaveData, canvas: &mut Canvas) {
        let (mut sprite_index, label, x_offset) = match self.command {
            Cmd::StartGame => {
                let label = if data.game().is_none() {
                    "New Game"
                } else {
                    "Continue"
                };
                (0, label, 0)
            }
            Cmd::ShowAboutBox => (2, "About", 4),
            Cmd::EraseGame => {
                if data.game().is_none() {
                    return;
                }
                (4, "Erase Game", 0)
            }
            Cmd::Quit => (6, "Quit", -4),
        };
        if self.active {
            sprite_index += 1;
        }
        canvas.draw_sprite_centered(&self.sprites[sprite_index], self.center);
        canvas.draw_text(&self.font,
                         Align::Center,
                         self.center + Point::new(x_offset, 4),
                         label);
    }

    fn handle_event(&mut self, event: &Event, data: &mut SaveData)
                    -> Action<Cmd> {
        if self.command == Cmd::EraseGame && data.game().is_none() {
            return Action::ignore();
        }
        match event {
            &Event::MouseDown(pt) if self.rect().contains_point(pt) => {
                self.active = true;
                Action::redraw()
                    .and_play_sound(Sound::beep())
                    .and_return(self.command)
            }
            _ => Action::ignore(),
        }
    }
}

// ========================================================================= //

#[cfg_attr(rustfmt, rustfmt_skip)]
pub const ABOUT_BOX_TEXT: &str = concat!("\
$C$f{block}SYSTEM SYZYGY$r$Rv", env!("CARGO_PKG_VERSION"), " $L\n\
\n\
Copyright 2016 Matthew D. Steele <mdsteele@alum.mit.edu>\n\
\n\
Game website:$Rhttps://mdsteele.games/syzygy/ $L\n\
Source code:$Rhttps://github.com/mdsteele/syzygy/ $L\n\
\n\
$iSystem Syzygy$r  comes with ABSOLUTELY NO WARRANTY.\n\
$iSystem Syzygy$r  is free software: you can redistribute it and/or\n\
modify it under the terms of the GNU General Public License as\n\
published by the Free Software Foundation, either version 3 of\n\
the License, or (at your option) any later version.\n\
\n\
$CThanks for playing!");

// ========================================================================= //

pub struct ConfirmEraseView<'a> {
    title_view: &'a View,
    dialog: DialogBox<bool>,
}

impl<'a> ConfirmEraseView<'a> {
    pub fn new(resources: &mut Resources, visible: Rect,
               title_view: &'a View)
               -> ConfirmEraseView<'a> {
        let text = "Really erase game data?\nAll progress will be lost!";
        let buttons =
            vec![("Cancel".to_string(), false), ("Erase".to_string(), true)];
        let dialog = DialogBox::new(resources, visible, text, buttons);
        ConfirmEraseView {
            title_view: title_view,
            dialog: dialog,
        }
    }
}

impl<'a> Element<SaveData, bool> for ConfirmEraseView<'a> {
    fn draw(&self, data: &SaveData, canvas: &mut Canvas) {
        self.title_view.draw(data, canvas);
        self.dialog.draw(&(), canvas);
    }

    fn handle_event(&mut self, event: &Event, _data: &mut SaveData)
                    -> Action<bool> {
        self.dialog.handle_event(event, &mut ())
    }
}

// ========================================================================= //
