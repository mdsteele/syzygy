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

use elements;
use elements::column::ColumnsView;
use elements::lasers::LaserField;
use elements::plane::{PlaneCmd, PlaneGridView};
use gui::{Action, Canvas, Element, Event, Point, Resources};
use save::SyzygyState;
use save::ice::BlockSlide;

// ========================================================================= //

#[derive(Clone, Debug)]
pub enum MezureCmd {
    Pipes(Vec<(Point, Point)>),
    IceBlocks(BlockSlide),
    Columns(usize, i32),
}

// ========================================================================= //

pub struct MezureView {
    columns: ColumnsView,
    ice_grid: elements::ice::GridView,
    laser_grid: LaserField,
    pipe_grid: PlaneGridView,
    animating_slide: bool,
}

impl MezureView {
    pub fn new(resources: &mut Resources, state: &SyzygyState) -> MezureView {
        MezureView {
            columns: ColumnsView::new(resources, 330, 250, 0),
            ice_grid: elements::ice::GridView::new(resources,
                                                   192,
                                                   104,
                                                   state.mezure_ice_grid()),
            laser_grid: LaserField::new(resources,
                                        192,
                                        104,
                                        state.mezure_laser_grid()),
            pipe_grid: PlaneGridView::new(resources, 60, 104),
            animating_slide: false,
        }
    }

    pub fn refresh(&mut self, state: &SyzygyState) {
        self.laser_grid.recalculate_lasers(state.mezure_laser_grid());
        self.ice_grid.reset_animation();
    }
}

impl Element<SyzygyState, MezureCmd> for MezureView {
    fn draw(&self, state: &SyzygyState, canvas: &mut Canvas) {
        self.columns.draw(state.mezure_columns(), canvas);
        self.laser_grid.draw_immovables(state.mezure_laser_grid(), canvas);
        self.ice_grid.draw_objects(state.mezure_ice_grid(), canvas);
        self.laser_grid.draw_lasers(canvas);
        self.ice_grid.draw_ice_blocks(state.mezure_ice_grid(), canvas);
        self.laser_grid.draw_sparks(canvas);
        self.pipe_grid.draw(state.mezure_pipe_grid(), canvas);
    }

    fn handle_event(&mut self, event: &Event, state: &mut SyzygyState)
                    -> Action<MezureCmd> {
        let mut action = {
            let columns = state.mezure_columns_mut();
            let subaction = self.columns.handle_event(event, columns);
            subaction.map(|(col, by)| MezureCmd::Columns(col, by))
        };
        if !action.should_stop() {
            let subaction = {
                let grid = state.mezure_ice_grid_mut();
                self.ice_grid.handle_event(event, grid)
            };
            if let Some(&(coords, dir)) = subaction.value() {
                if let Some(slide) = state.mezure_ice_grid_mut()
                                          .slide_ice_block(coords, dir) {
                    state.mezure_regenerate_laser_grid();
                    self.ice_grid.animate_slide(&slide);
                    self.laser_grid.clear_lasers();
                    self.animating_slide = true;
                    action = action.and_return(MezureCmd::IceBlocks(slide));
                }
            }
            action.merge(subaction.but_no_value());
        }
        if self.animating_slide && !self.ice_grid.is_animating() {
            self.animating_slide = false;
            let grid = state.mezure_laser_grid();
            self.laser_grid.recalculate_lasers(grid);
            action.also_redraw();
        }
        if event == &Event::ClockTick {
            let grid = state.mezure_laser_grid_mut();
            let subaction = self.laser_grid.handle_event(event, grid);
            action.merge(subaction.but_no_value());
        }
        if !action.should_stop() {
            let mut subaction = {
                let grid = state.mezure_pipe_grid_mut();
                self.pipe_grid.handle_event(event, grid)
            };
            match subaction.take_value() {
                Some(PlaneCmd::Changed) => {
                    state.mezure_regenerate_laser_grid();
                    let grid = state.mezure_laser_grid();
                    self.laser_grid.recalculate_lasers(grid);
                    action.also_redraw();
                }
                Some(PlaneCmd::PushUndo(pieces)) => {
                    action = action.and_return(MezureCmd::Pipes(pieces));
                }
                None => {}
            }
            action.merge(subaction.but_no_value());
        }
        action
    }
}

// ========================================================================= //
