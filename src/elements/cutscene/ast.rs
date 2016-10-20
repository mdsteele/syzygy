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
use gui::{Point, Resources};
use super::scene::{DarkNode, JumpNode, LightNode, LoopNode, ParallelNode,
                   PlaceNode, Scene, SceneNode, SequenceNode, SlideNode,
                   TalkNode, WaitNode};

// ========================================================================= //

pub enum Ast {
    Seq(Vec<Ast>),
    Par(Vec<Ast>),
    Loop(i32, i32, Box<Ast>),
    Dark(bool),
    Jump(i32, (i32, i32), f64),
    Light(i32, bool),
    Place(i32, &'static str, (i32, i32)),
    Slide(i32, (i32, i32), bool, bool, f64),
    Talk(i32, &'static str),
    Wait(f64),
}

impl Ast {
    pub fn compile_scene(resources: &mut Resources, nodes: Vec<Ast>) -> Scene {
        Scene::new(nodes.iter()
                        .map(|ast| ast.to_scene_node(resources))
                        .collect())
    }

    pub fn to_scene_node(&self, resources: &mut Resources) -> Box<SceneNode> {
        match self {
            &Ast::Seq(ref asts) => {
                let nodes = asts.iter()
                                .map(|ast| ast.to_scene_node(resources))
                                .collect();
                Box::new(SequenceNode::new(nodes))
            }
            &Ast::Par(ref asts) => {
                let nodes = asts.iter()
                                .map(|ast| ast.to_scene_node(resources))
                                .collect();
                Box::new(ParallelNode::new(nodes))
            }
            &Ast::Loop(min, max, ref ast) => {
                let max = if max < 0 {
                    None
                } else {
                    Some(max)
                };
                Box::new(LoopNode::new(ast.to_scene_node(resources), min, max))
            }
            &Ast::Dark(dark) => Box::new(DarkNode::new(dark)),
            &Ast::Jump(slot, (x, y), duration) => {
                Box::new(JumpNode::new(slot, Point::new(x, y), duration))
            }
            &Ast::Light(slot, light) => {
                let sprite = if light {
                    Some(resources.get_sprites("halo")[0].clone())
                } else {
                    None
                };
                Box::new(LightNode::new(slot, sprite))
            }
            &Ast::Place(slot, name, (x, y)) => {
                let sprite = match name {
                    "Argony" => resources.get_sprites("chars")[1].clone(),
                    "Elinsa" => resources.get_sprites("chars")[0].clone(),
                    "Relyng" => resources.get_sprites("chars")[5].clone(),
                    "Tezure" => resources.get_sprites("chars")[2].clone(),
                    "Ugrent" => resources.get_sprites("chars")[4].clone(),
                    "Yttris" => resources.get_sprites("chars")[3].clone(),
                    _ => {
                        resources.get_font("block").glyph('?').sprite().clone()
                    }
                };
                Box::new(PlaceNode::new(slot,
                                        sprite.clone(),
                                        Point::new(x, y)))
            }
            &Ast::Slide(slot, (x, y), accel, decel, duration) => {
                Box::new(SlideNode::new(slot,
                                        Point::new(x, y),
                                        accel,
                                        decel,
                                        duration))
            }
            &Ast::Talk(slot, text) => {
                Box::new(TalkNode::new(slot, Paragraph::new(resources, text)))
            }
            &Ast::Wait(duration) => Box::new(WaitNode::new(duration)),
        }
    }
}

// ========================================================================= //
