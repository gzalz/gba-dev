use gba::prelude::*;

use crate::{
  input::{read_dir_key_input, Direction},
  object::Object,
};

const HP_1: usize = 0usize;
const HP_2: usize = 1usize;
const HP_3: usize = 2usize;
const PLAYER_INDEX: usize = 3usize;

fn rng(tick: u16) -> u16 {
  let multiplier: u16 = 25173;
  let increment: u16 = 13849;

  tick.wrapping_mul(multiplier).wrapping_add(increment)
}

pub fn start() {
  let heart_1 = Object::new(8, 8, Cga8x8Thick::HEART as u16, 2, 0);
  let heart_2 = Object::new(16, 8, Cga8x8Thick::HEART as u16, 2, 1);
  let heart_3 = Object::new(24, 8, Cga8x8Thick::HEART as u16, 2, 2);
  let player = Object::new(120, 140, Cga8x8Thick::FACE_INVERSE as u16, 1, 3);
  let projectile_0 = Object::new(40, 90, Cga8x8Thick::SOLAR as u16, 3, 4);
  let projectile_1 = Object::new(60, 45, Cga8x8Thick::SOLAR as u16, 3, 5);
  let projectile_2 = Object::new(200, 00, Cga8x8Thick::SOLAR as u16, 3, 6);

  let mut objects = [
    heart_1,
    heart_2,
    heart_3,
    player,
    projectile_0,
    projectile_1,
    projectile_2,
  ];

  let mut tick = 0u16;
  let mut projectile_speed = 2u16;
  let mut player_x = 120u16;
  let mut player_y = 140u16;
  let mut hp = 3u16;
  let mut dmg_cooldown = 0u16;

  loop {
    VBlankIntrWait();
    for (index, obj) in objects.iter_mut().enumerate() {
      let try_dir_input = read_dir_key_input();
      obj.on_tick();

      if index == HP_3 {
        if hp < 3 {
          obj.set_palbank(1);
        }
      }

      if index == HP_2 {
        if hp < 2 {
          obj.set_palbank(1);
        }
      }

      if index == HP_1 {
        if hp < 1 {
          obj.set_palbank(1);
        }
      }

      if index == PLAYER_INDEX {
        if let Some(direction) = try_dir_input {
          if direction == Direction::LEFT || direction == Direction::RIGHT {
            obj.transform(direction, 2);
          }
          player_x = obj.x;
          player_y = obj.y;
        }
        if dmg_cooldown > 0 {
          dmg_cooldown -= 1;
          obj.set_palbank(2);
        } else {
          obj.set_palbank(1);
        }
      }

      if index > PLAYER_INDEX {
        if obj.y >= 139 {
          obj.y = 0;
          let seed = tick % player_x;
          obj.x = rng(seed) % 220;
        }
        obj.transform(Direction::DOWN, projectile_speed);
        // Check for collision with player
        if obj.x >= player_x - 8
          && obj.x <= player_x + 8
          && obj.y >= player_y - 8
          && obj.y <= player_y + 8
          && dmg_cooldown == 0
        {
          hp -= 1;
          obj.y = 0;
          let seed = tick % player_x;
          obj.x = rng(seed) % 220;
          dmg_cooldown = 50;
        }
      }

      if tick > 0 && tick % 20000 == 0 {
        projectile_speed += 1;
      }

      if tick == u16::MAX {
        tick = 0;
      } else {
        tick += 1;
      }
    }
    if hp == 0 {
        objects[HP_1].set_palbank(1);
        loop {
            let keys = KEYINPUT.read();
            if keys.start() {
              hp = 3;
              dmg_cooldown = 0;
              projectile_speed = 2;
              player_x = 120;
              player_y = 140;
              objects = [
                heart_1,
                heart_2,
                heart_3,
                player,
                projectile_0,
                projectile_1,
                projectile_2,
              ];
              break;
            }
    }
    }
  }
}
