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
use super::super::gui::{Action, Align, Canvas, Element, Event, Font,
                        GroupElement, Point, Rect, Resources, Sprite,
                        SubrectElement};
use super::super::elements::DialogBox;
use super::super::save::SaveData;

// ========================================================================= //

const START_BUTTON_WIDTH: u32 = 64;
const START_BUTTON_HEIGHT: u32 = 32;

const BOTTOM_BUTTONS_MARGIN: i32 = 20;
const BOTTOM_BUTTONS_HEIGHT: u32 = 32;
const FULLSCREEN_BUTTON_WIDTH: u32 = 32;
const ABOUT_BUTTON_WIDTH: u32 = 64;
const ERASE_BUTTON_WIDTH: u32 = 96;
const QUIT_BUTTON_WIDTH: u32 = 64;

// ========================================================================= //

pub enum TitleAction {
    SetFullscreen(bool),
    StartGame,
    EraseGame,
    ShowAboutBox,
    Quit,
}

// ========================================================================= //

pub struct TitleView {
    elements: GroupElement<SaveData, TitleAction>,
}

impl TitleView {
    pub fn new(resources: &mut Resources, visible: Rect) -> TitleView {
        let spacing = (visible.width() as i32 - 2 * BOTTOM_BUTTONS_MARGIN -
                       FULLSCREEN_BUTTON_WIDTH as i32 -
                       ABOUT_BUTTON_WIDTH as i32 -
                       ERASE_BUTTON_WIDTH as i32 -
                       QUIT_BUTTON_WIDTH as i32) / 3;
        let mut elements: Vec<Box<Element<SaveData, TitleAction>>> = vec![];
        elements.push(Box::new({
            let mut rect = Rect::new(0,
                                     0,
                                     START_BUTTON_WIDTH,
                                     START_BUTTON_HEIGHT);
            rect.center_on(visible.center());
            SubrectElement::new(StartGameButton::new(resources), rect)
        }));
        if !cfg!(any(target_os = "android", target_os = "ios")) {
            elements.push(Box::new({
                let rect = Rect::new(visible.left() + BOTTOM_BUTTONS_MARGIN,
                                     visible.bottom() -
                                     BOTTOM_BUTTONS_HEIGHT as i32 -
                                     BOTTOM_BUTTONS_MARGIN,
                                     FULLSCREEN_BUTTON_WIDTH,
                                     BOTTOM_BUTTONS_HEIGHT);
                SubrectElement::new(FullscreenButton::new(resources), rect)
            }));
            elements.push(Box::new({
                let rect = Rect::new(visible.right() - BOTTOM_BUTTONS_MARGIN -
                                     QUIT_BUTTON_WIDTH as i32,
                                     visible.bottom() -
                                     BOTTOM_BUTTONS_HEIGHT as i32 -
                                     BOTTOM_BUTTONS_MARGIN,
                                     QUIT_BUTTON_WIDTH,
                                     BOTTOM_BUTTONS_HEIGHT);
                SubrectElement::new(QuitButton::new(resources), rect)
            }));
        }
        elements.push(Box::new({
            let rect = Rect::new(visible.left() + BOTTOM_BUTTONS_MARGIN +
                                 FULLSCREEN_BUTTON_WIDTH as i32 +
                                 spacing,
                                 visible.bottom() -
                                 BOTTOM_BUTTONS_HEIGHT as i32 -
                                 BOTTOM_BUTTONS_MARGIN,
                                 ABOUT_BUTTON_WIDTH,
                                 BOTTOM_BUTTONS_HEIGHT);
            SubrectElement::new(AboutButton::new(resources), rect)
        }));
        elements.push(Box::new({
            let rect = Rect::new(visible.left() + BOTTOM_BUTTONS_MARGIN +
                                 FULLSCREEN_BUTTON_WIDTH as i32 +
                                 ABOUT_BUTTON_WIDTH as i32 +
                                 2 * spacing,
                                 visible.bottom() -
                                 BOTTOM_BUTTONS_HEIGHT as i32 -
                                 BOTTOM_BUTTONS_MARGIN,
                                 ERASE_BUTTON_WIDTH,
                                 BOTTOM_BUTTONS_HEIGHT);
            SubrectElement::new(EraseGameButton::new(resources), rect)
        }));
        TitleView { elements: GroupElement::new(elements) }
    }
}

impl Element<SaveData, TitleAction> for TitleView {
    fn draw(&self, data: &SaveData, canvas: &mut Canvas) {
        canvas.clear((64, 64, 128));
        self.elements.draw(data, canvas);
    }

    fn handle_event(&mut self, event: &Event, data: &mut SaveData)
                    -> Action<TitleAction> {
        self.elements.handle_event(event, data)
    }
}

// ========================================================================= //

struct FullscreenButton {
    to_fullscreen_icon: Sprite,
    to_windowed_icon: Sprite,
}

impl FullscreenButton {
    fn new(resources: &mut Resources) -> FullscreenButton {
        let sprites = resources.get_sprites("fullscreen");
        FullscreenButton {
            to_fullscreen_icon: sprites[0].clone(),
            to_windowed_icon: sprites[1].clone(),
        }
    }
}

impl Element<SaveData, TitleAction> for FullscreenButton {
    fn draw(&self, data: &SaveData, canvas: &mut Canvas) {
        let icon = if data.prefs().fullscreen() {
            &self.to_windowed_icon
        } else {
            &self.to_fullscreen_icon
        };
        canvas.draw_sprite(icon, Point::new(0, 0));
    }

    fn handle_event(&mut self, event: &Event, data: &mut SaveData)
                    -> Action<TitleAction> {
        match event {
            &Event::MouseDown(_) => {
                let full = !data.prefs().fullscreen();
                Action::redraw().and_return(TitleAction::SetFullscreen(full))
            }
            _ => Action::ignore(),
        }
    }
}

// ========================================================================= //

struct StartGameButton {
    font: Rc<Font>,
}

impl StartGameButton {
    fn new(resources: &mut Resources) -> StartGameButton {
        StartGameButton { font: resources.get_font("roman") }
    }
}

impl Element<SaveData, TitleAction> for StartGameButton {
    fn draw(&self, data: &SaveData, canvas: &mut Canvas) {
        let rect = canvas.rect();
        let label = if data.game().is_some() {
            "Continue"
        } else {
            "New Game"
        };
        canvas.draw_text(&self.font, Align::Center, rect.center(), label);
    }

    fn handle_event(&mut self, event: &Event, _: &mut SaveData)
                    -> Action<TitleAction> {
        match event {
            &Event::MouseDown(_) => {
                Action::redraw().and_return(TitleAction::StartGame)
            }
            _ => Action::ignore(),
        }
    }
}

// ========================================================================= //

struct EraseGameButton {
    font: Rc<Font>,
}

impl EraseGameButton {
    fn new(resources: &mut Resources) -> EraseGameButton {
        EraseGameButton { font: resources.get_font("roman") }
    }
}

impl Element<SaveData, TitleAction> for EraseGameButton {
    fn draw(&self, data: &SaveData, canvas: &mut Canvas) {
        if data.game().is_some() {
            let rect = canvas.rect();
            canvas.draw_text(&self.font,
                             Align::Center,
                             rect.center(),
                             "Erase Game");
        }
    }

    fn handle_event(&mut self, event: &Event, data: &mut SaveData)
                    -> Action<TitleAction> {
        match event {
            &Event::MouseDown(_) if data.game().is_some() => {
                Action::redraw().and_return(TitleAction::EraseGame)
            }
            _ => Action::ignore(),
        }
    }
}

// ========================================================================= //

struct AboutButton {
    font: Rc<Font>,
}

impl AboutButton {
    fn new(resources: &mut Resources) -> AboutButton {
        AboutButton { font: resources.get_font("roman") }
    }
}

impl Element<SaveData, TitleAction> for AboutButton {
    fn draw(&self, _: &SaveData, canvas: &mut Canvas) {
        let rect = canvas.rect();
        canvas.draw_text(&self.font, Align::Center, rect.center(), "About");
    }

    fn handle_event(&mut self, event: &Event, _: &mut SaveData)
                    -> Action<TitleAction> {
        match event {
            &Event::MouseDown(_) => {
                Action::redraw().and_return(TitleAction::ShowAboutBox)
            }
            _ => Action::ignore(),
        }
    }
}

// ========================================================================= //

struct QuitButton {
    font: Rc<Font>,
}

impl QuitButton {
    fn new(resources: &mut Resources) -> QuitButton {
        QuitButton { font: resources.get_font("roman") }
    }
}

impl Element<SaveData, TitleAction> for QuitButton {
    fn draw(&self, _: &SaveData, canvas: &mut Canvas) {
        let rect = canvas.rect();
        canvas.draw_text(&self.font, Align::Center, rect.center(), "Quit");
    }

    fn handle_event(&mut self, event: &Event, _: &mut SaveData)
                    -> Action<TitleAction> {
        match event {
            &Event::MouseDown(_) => {
                Action::redraw().and_return(TitleAction::Quit)
            }
            _ => Action::ignore(),
        }
    }
}

// ========================================================================= //

pub struct AboutBoxView<'a> {
    title_view: &'a TitleView,
    dialog: DialogBox<()>,
}

impl<'a> AboutBoxView<'a> {
    pub fn new(resources: &mut Resources, visible: Rect,
               title_view: &'a TitleView)
               -> AboutBoxView<'a> {
        let text = ABOUT_BOX_TEXT;
        let buttons = vec![("OK".to_string(), ())];
        let dialog = DialogBox::new(resources, visible, text, buttons);
        AboutBoxView {
            title_view: title_view,
            dialog: dialog,
        }
    }
}

impl<'a> Element<SaveData, ()> for AboutBoxView<'a> {
    fn draw(&self, data: &SaveData, canvas: &mut Canvas) {
        self.title_view.draw(data, canvas);
        self.dialog.draw(&(), canvas);
    }

    fn handle_event(&mut self, event: &Event, _data: &mut SaveData)
                    -> Action<()> {
        self.dialog.handle_event(event, &mut ())
    }
}

const ABOUT_BOX_TEXT: &'static str = "\
System Syzygy\n\
\n\
Copyright 2012 Matthew D. Steele <mdsteele@alum.mit.edu>\n\
\n\
Source code: https://github.com/mdsteele/syzygy/\n\
\n\
System Syzygy comes with ABSOLUTELY NO WARRANTY.\n\
System Syzygy is free software: you can redistribute it and/or\n\
modify it under the terms of the GNU General Public License as\n\
published by the Free Software Foundation, either version 3 of\n\
the License, or (at your option) any later version.\n\
\n\
Thanks for playing!";

// ========================================================================= //

pub struct ConfirmEraseView<'a> {
    title_view: &'a TitleView,
    dialog: DialogBox<bool>,
}

impl<'a> ConfirmEraseView<'a> {
    pub fn new(resources: &mut Resources, visible: Rect,
               title_view: &'a TitleView)
               -> ConfirmEraseView<'a> {
        let text = "Really erase game data?\nAll progress will be lost!";
        let buttons = vec![("Cancel".to_string(), false),
                           ("Erase".to_string(), true)];
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
