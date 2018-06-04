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

use elements::Paragraph;
use gui::{Align, Point, Resources, Sound};
use super::scene::{AnimNode, DarkNode, JumpNode, LightNode, LoopNode,
                   ParallelNode, PlaceNode, QueueNode, RemoveNode, Scene,
                   SceneNode, SequenceNode, SetBgNode, SetPosNode,
                   SetSpriteNode, ShakeNode, SlideNode, SoundNode, SwapNode,
                   TalkNode, WaitNode};
use super::theater::TalkPos;

// ========================================================================= //

#[derive(Clone, Copy)]
pub enum TalkStyle {
    Comm,
    Evil,
    Good,
    Normal,
    System,
    Thought,
}

// ========================================================================= //

pub enum Ast {
    Seq(Vec<Ast>),
    Par(Vec<Ast>),
    Loop(i32, i32, Box<Ast>),
    Anim(i32, &'static str, &'static [usize], i32),
    Dark(bool),
    Jump(i32, (i32, i32), f64),
    Light(i32, bool),
    Place(i32, &'static str, usize, (i32, i32)),
    Queue(i32, i32),
    Remove(i32),
    SetBg(&'static str),
    SetPos(i32, (i32, i32)),
    SetSprite(i32, &'static str, usize),
    Shake(i32),
    Slide(i32, (i32, i32), bool, bool, f64),
    Sound(Sound),
    Swap(i32, i32),
    Talk(i32, TalkStyle, TalkPos, &'static str),
    Wait(f64),
}

impl Ast {
    pub fn compile_scene(resources: &mut Resources, nodes: Vec<Ast>) -> Scene {
        Scene::new(nodes
                       .into_iter()
                       .map(|ast| ast.to_scene_node(resources))
                       .collect())
    }

    fn to_scene_node(self, resources: &mut Resources) -> Box<SceneNode> {
        match self {
            Ast::Seq(asts) => {
                let nodes = asts.into_iter()
                    .map(|ast| ast.to_scene_node(resources))
                    .collect();
                Box::new(SequenceNode::new(nodes))
            }
            Ast::Par(asts) => {
                let nodes = asts.into_iter()
                    .map(|ast| ast.to_scene_node(resources))
                    .collect();
                Box::new(ParallelNode::new(nodes))
            }
            Ast::Loop(min, max, ast) => {
                let max = if max <= 0 { None } else { Some(max) };
                Box::new(LoopNode::new(ast.to_scene_node(resources), min, max))
            }
            Ast::Anim(slot, name, indices, slowdown) => {
                let all_sprites = resources.get_sprites(name);
                let mut anim_sprites = Vec::with_capacity(indices.len());
                for &index in indices {
                    anim_sprites.push(all_sprites[index].clone());
                }
                Box::new(AnimNode::new(slot, anim_sprites, slowdown))
            }
            Ast::Dark(dark) => Box::new(DarkNode::new(dark)),
            Ast::Jump(slot, (x, y), duration) => {
                Box::new(JumpNode::new(slot, Point::new(x, y), duration))
            }
            Ast::Light(slot, light) => {
                let sprite = if light {
                    Some(resources.get_sprites("halo")[0].clone())
                } else {
                    None
                };
                Box::new(LightNode::new(slot, sprite))
            }
            Ast::Place(slot, name, index, (x, y)) => {
                let sprite = resources.get_sprites(name)[index].clone();
                Box::new(PlaceNode::new(slot, sprite, Point::new(x, y)))
            }
            Ast::Queue(v1, v2) => Box::new(QueueNode::new((v1, v2))),
            Ast::Remove(slot) => Box::new(RemoveNode::new(slot)),
            Ast::SetBg(name) => {
                Box::new(SetBgNode::new(resources.get_background(name)))
            }
            Ast::SetPos(slot, (x, y)) => {
                Box::new(SetPosNode::new(slot, Point::new(x, y)))
            }
            Ast::SetSprite(slot, name, index) => {
                let sprite = resources.get_sprites(name)[index].clone();
                Box::new(SetSpriteNode::new(slot, sprite))
            }
            Ast::Shake(amount) => Box::new(ShakeNode::new(amount)),
            Ast::Slide(slot, (x, y), accel, decel, duration) => {
                Box::new(SlideNode::new(slot,
                                        Point::new(x, y),
                                        accel,
                                        decel,
                                        duration))
            }
            Ast::Sound(sound) => Box::new(SoundNode::new(sound)),
            Ast::Swap(slot1, slot2) => Box::new(SwapNode::new(slot1, slot2)),
            Ast::Talk(slot, style, pos, text) => {
                let (bubble_name, color, init_font, init_align) =
                    match style {
                        TalkStyle::Comm => {
                            ("speech/comm", WHITE, "italic", Align::Center)
                        }
                        TalkStyle::Good => {
                            ("speech/good", BLACK, "good", Align::Center)
                        }
                        TalkStyle::Evil => {
                            ("speech/evil", BLACK, "evil", Align::Center)
                        }
                        TalkStyle::Normal => {
                            ("speech/normal", WHITE, "roman", Align::Center)
                        }
                        TalkStyle::System => {
                            ("speech/system", BLACK, "system", Align::Left)
                        }
                        TalkStyle::Thought => {
                            ("speech/thought", WHITE, "roman", Align::Center)
                        }
                    };
                let sprites = resources.get_sprites(bubble_name);
                let paragraph =
                    Paragraph::new(resources, init_font, init_align, text);
                Box::new(TalkNode::new(slot, sprites, color, pos, paragraph))
            }
            Ast::Wait(duration) => Box::new(WaitNode::new(duration)),
        }
    }
}

// ========================================================================= //

const BLACK: (u8, u8, u8) = (0, 0, 0);
const WHITE: (u8, u8, u8) = (255, 255, 255);

// ========================================================================= //
