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
  let heart_1 = Object::new(0, 0, Cga8x8Thick::HEART as u16, 2, 0);
  let heart_2 = Object::new(8, 0, Cga8x8Thick::HEART as u16, 2, 1);
  let heart_3 = Object::new(16, 0, Cga8x8Thick::HEART as u16, 2, 2);
  let player = Object::new(120, 140, Cga8x8Thick::FACE_INVERSE as u16, 1, 3);
  let projectile_0 = Object::new(40, 90, Cga8x8Thick::SOLAR as u16, 3, 4);
  let projectile_1 = Object::new(60, 45, Cga8x8Thick::SOLAR as u16, 3, 5);
  let projectile_2 = Object::new(200, 00, Cga8x8Thick::SOLAR as u16, 3, 6);
  let score_1s = Object::new(228, 0, 48 as u16, 1, 7);
  let score_10s = Object::new(220, 0, 48 as u16, 1, 8);
  let score_100s = Object::new(212, 0, 48 as u16, 1, 9);
  let score_1000s = Object::new(204, 0, 48 as u16, 1, 10);
  let score_10000s = Object::new(196, 0, 48 as u16, 1, 11);
  let char_g = Object::new(30, 60, 71 as u16, 0, 12);
  let char_a = Object::new(50, 60, 65 as u16, 0, 13);
  let char_m = Object::new(70, 60, 77 as u16, 0, 14);
  let char_e = Object::new(90, 60, 69 as u16, 0, 15);
  let char_o = Object::new(130, 60, 79 as u16, 0, 16);
  let char_v = Object::new(150, 60, 86 as u16, 0, 17);
  let char_e_2 = Object::new(170, 60, 69 as u16, 0, 18);
  let char_r = Object::new(190, 60, 82 as u16, 0, 119);

  let mut objects = [
    heart_1,
    heart_2,
    heart_3,
    player,
    projectile_0,
    projectile_1,
    projectile_2,
    score_1s,
    score_10s,
    score_100s,
    score_1000s,
    score_10000s,
    char_g,
    char_a,
    char_m,
    char_e,
    char_o,
    char_v,
    char_e_2,
    char_r,
  ];

  let mut tick = 0u16;
  let mut projectile_speed = 2u16;
  let mut player_x = 120u16;
  let mut player_y = 140u16;
  let mut hp = 3u16;
  let mut dmg_cooldown = 0u16;

  let mut num_score_1s = 0u16;
  let mut num_score_10s = 0u16;
  let mut num_score_100s = 0u16;
  let mut num_score_1000s = 0u16;
  let mut num_score_10000s = 0u16;

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

      if index > PLAYER_INDEX && index < 7 {
        if obj.y >= 139 {
          obj.y = 0;
          let seed = tick % player_x;
          obj.x = rng(seed) % 220;
          if num_score_1s < 9 {
            num_score_1s += 1;
          } else {
            num_score_1s = 0;
            if num_score_10s < 9 {
              num_score_10s += 1;
            } else {
              num_score_10s = 0;
              if num_score_100s < 9 {
                num_score_100s += 1;
              } else {
                num_score_100s = 0;
                if num_score_1000s < 9 {
                  num_score_1000s += 1;
                } else {
                  num_score_1000s = 0;
                  if num_score_10000s < 9 {
                    num_score_10000s += 1;
                  } else {
                    num_score_10000s = 0;
                  }
                }
              }
            }
          }
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

      if index == 7 {
        obj.set_tile_id(48 + num_score_1s);
      }

      if index == 8 {
        obj.set_tile_id(48 + num_score_10s);
      }

      if index == 9 {
        obj.set_tile_id(48 + num_score_100s);
      }

      if index == 10 {
        obj.set_tile_id(48 + num_score_1000s);
      }

      if index == 11 {
        obj.set_tile_id(48 + num_score_10000s);
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
      objects[12].set_palbank(1);
      objects[13].set_palbank(1);
      objects[14].set_palbank(1);
      objects[15].set_palbank(1);
      objects[16].set_palbank(1);
      objects[17].set_palbank(1);
      objects[18].set_palbank(1);
      objects[19].set_palbank(1);
      loop {
        let keys = KEYINPUT.read();
        if keys.start() {
          hp = 3;
          dmg_cooldown = 0;
          projectile_speed = 2;
          player_x = 120;
          player_y = 140;
          num_score_1s = 0;
          num_score_10s = 0;
          num_score_100s = 0;
          num_score_1000s = 0;
          num_score_10000s = 0;
          objects = [
            heart_1,
            heart_2,
            heart_3,
            player,
            projectile_0,
            projectile_1,
            projectile_2,
            score_1s,
            score_10s,
            score_100s,
            score_1000s,
            score_10000s,
            char_g,
            char_a,
            char_m,
            char_e,
            char_o,
            char_v,
            char_e_2,
            char_r,
          ];
          break;
        }
      }
    }
  }
}
