use gba::prelude::*;

use crate::input::Direction;

#[allow(dead_code)]
#[derive(Debug, Clone, Copy)]
pub struct Object {
  pub x: u16,
  pub y: u16,
  pub tile_id: u16,
  pub palbank: u16,
  pub index: usize,
  pub attr: ObjAttr,
}

impl Object {
  pub fn new(x: u16, y: u16, tile_id: u16, palbank: u16, index: usize) -> Self {
    let mut attr = ObjAttr::new();
    attr.set_x(x);
    attr.set_y(y);
    attr.set_tile_id(tile_id);
    attr.set_palbank(palbank);
    let obj = Self { x, y, tile_id, palbank, index, attr };
    obj.write();
    obj
  }

  fn write(&self) {
    OBJ_ATTR_ALL.index(self.index).write(self.attr)
  }

  pub fn set_palbank(&mut self, palbank: u16) {
    self.palbank = palbank;
    self.attr.set_palbank(palbank);
    self.write();
  }

  pub fn on_tick(&self) {
    self.write()
  }

  pub fn transform(&mut self, direction: Direction, magnitude: u16) {
    match direction {
      Direction::UP => self.y = self.y.saturating_sub(magnitude),
      Direction::DOWN => {
        if self.y < 140 {
          self.y = self.y + magnitude
        }
      }
      Direction::LEFT => self.x = self.x.saturating_sub(magnitude),
      Direction::RIGHT => {
        if self.x < 220 {
          self.x = self.x + magnitude
        }
      }
    }
    self.attr.set_x(self.x);
    self.attr.set_y(self.y);
  }
}
