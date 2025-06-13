use gba::prelude::*;

#[derive(PartialEq)]
pub enum Direction {
  UP,
  DOWN,
  LEFT,
  RIGHT,
}

pub fn read_dir_key_input() -> Option<Direction> {
  let keys = KEYINPUT.read();
  if keys.up() {
    return Some(Direction::UP);
  }
  if keys.down() {
    return Some(Direction::DOWN);
  }
  if keys.left() {
    return Some(Direction::LEFT);
  }
  if keys.right() {
    return Some(Direction::RIGHT);
  }
  None
}
