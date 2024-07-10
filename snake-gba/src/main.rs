#![no_std]
#![no_main]

use gba::{
  prelude::*,
  random::{Gen32, Lcg32},
};

const FOOD_OBJECT_OFFSET: usize = 0;
const HEAD_OBJECT_OFFSET: usize = 1;

const INIT_POS: Position = Position { x: 8, y: 8 };
const INIT_DIR: DIR = DIR::RIGHT;
const INIT_SIZE: u16 = 1;

enum DIR {
  UP,
  DOWN,
  LEFT,
  RIGHT,
}

#[panic_handler]
fn panic_handler(info: &core::panic::PanicInfo) -> ! {
  #[cfg(debug_assertions)]
  if let Ok(mut logger) = MgbaBufferedLogger::try_new(MgbaMessageLevel::Fatal) {
    use core::fmt::Write;
    writeln!(logger, "{info}").ok();
  }
  loop {}
}

#[derive(Debug, Clone, Copy, Default)]
struct SnakeNodeState {
  position: Position,
  allocated: bool,
}

impl SnakeNodeState {
  fn default() -> Self {
    Self { position: Position::default(), allocated: false }
  }
}

#[derive(Debug, Clone, Copy, Default)]
struct Position {
  x: u16,
  y: u16,
}

impl Position {
  fn default() -> Self {
    INIT_POS
  }
}

#[derive(Debug, Clone, Copy, Default)]
struct Rect {
  x: u16,
  y: u16,
  w: u16,
  h: u16,
}
impl Rect {
  fn intersect(self, other: Self) -> bool {
    self.x < other.x + other.w
      && self.x + self.w > other.x
      && self.y < other.y + other.h
      && self.h + self.y > other.y
  }

  fn iter_tiles(self) -> impl Iterator<Item = (u16, u16)> {
    let y_range_incl = (self.y / 8)..=((self.y + self.h - 1) / 8);
    let x_range_incl = (self.x / 8)..=((self.x + self.w - 1) / 8);
    y_range_incl
      .map(move |y_index| {
        x_range_incl.clone().map(move |x_index| (x_index, y_index))
      })
      .flatten()
  }
}

fn random_in_range(min: u16, max: u16, random_value: u16) -> u16 {
  let range = max - min + 1;
  min + (random_value % range)
}

#[no_mangle]
extern "C" fn main() -> ! {
  // game simulation setup
  let mut player = Position::default();
  let mut size = INIT_SIZE;
  let player_x = 8;
  let player_y = 8;
  player.x = player_x;
  player.y = player_y;

  let mut world = [[0_u8; 32]; 32];
  for i in 0..32 {
    world[0][i] = u8::MAX;
    world[21][i] = u8::MAX;
    world[i][0] = u8::MAX;
    world[i][31] = u8::MAX;
  }
  world[0][0] = u8::MAX;
  world[0][31] = u8::MAX;
  world[21][0] = u8::MAX;
  world[21][31] = u8::MAX;

  DISPSTAT.write(DisplayStatus::new().with_irq_vblank(true));
  IE.write(IrqBits::VBLANK);
  IME.write(true);

  BG_PALETTE.index(1).write(Color::WHITE);
  let colors =
    [Color::GREEN, Color::WHITE, Color::RED, Color::BLUE, Color::YELLOW];
  for (pal, color) in colors.iter().enumerate() {
    obj_palbank(pal).index(1).write(*color);
  }

  Cga8x8Thick.bitunpack_4bpp(CHARBLOCK0_4BPP.as_region(), 0);
  Cga8x8Thick.bitunpack_4bpp(OBJ_TILES.as_region(), 0);

  BG0CNT.write(BackgroundControl::new().with_screenblock(8));
  let screenblock = TEXT_SCREENBLOCKS.get_frame(8).unwrap();
  for y in 0..32 {
    let row = screenblock.get_row(y).unwrap();
    for (x, addr) in row.iter().enumerate() {
      let te = TextEntry::new().with_tile(world[y][x] as u16);
      addr.write(te);
    }
  }

  let no_display = ObjAttr0::new().with_style(ObjDisplayStyle::NotDisplayed);
  OBJ_ATTR0.iter().skip(512).for_each(|va| va.write(no_display));

  DISPCNT.write(DisplayControl::new().with_show_obj(true).with_show_bg0(true));

  let mut rng = Lcg32::new(0x152);
  let mut snake_objects: [SnakeNodeState; 512] =
    [SnakeNodeState::default(); 512];

  let mut food_position: Position = Position {
    x: random_in_range(16, 220, rng.next_u16()),
    y: random_in_range(16, 132, rng.next_u16()),
  };
  let mut direction: DIR = DIR::RIGHT;

  let mut tick = 0;
  loop {
    VBlankIntrWait();

    if tick < 8 {
      tick += 1;
      continue;
    }

    snake_objects[0].position = player;
    snake_objects[0].allocated = true;
    let keys = KEYINPUT.read();

    for (i, attr_addr) in snake_objects.iter_mut().enumerate() {
      if !attr_addr.allocated {
        continue;
      }
      let mut obj = ObjAttr::new();
      obj.set_x(attr_addr.position.x);
      obj.set_y(attr_addr.position.y);
      obj.set_tile_id(Cga8x8Thick::CIRCLE as u16);
      obj.set_palbank(0);

      OBJ_ATTR_ALL.index(i + HEAD_OBJECT_OFFSET).write(obj);
    }

    let mut obj = ObjAttr::new();
    obj.set_x(food_position.x);
    obj.set_y(food_position.y);
    obj.set_tile_id(Cga8x8Thick::DIAMOND as u16);
    obj.set_palbank(1);
    OBJ_ATTR_ALL.index(FOOD_OBJECT_OFFSET).write(obj);

    if keys.up() {
        match direction {
          DIR::DOWN => {}
          _ => direction = DIR::UP,
        }
    }
    if keys.down() {
        match direction {
            DIR::UP => {}
            _ => direction = DIR::DOWN,
        }
    }
    if keys.left() {
      match direction {
        DIR::RIGHT => {}
        _ => direction = DIR::LEFT,
      }
    }
    if keys.right(){
      match direction {
        DIR::LEFT => {}
        _ => direction = DIR::RIGHT,
      }
    }

    match direction {
      DIR::UP => player.y -= 8,
      DIR::DOWN => player.y += 8,
      DIR::LEFT => player.x -= 8,
      DIR::RIGHT => player.x += 8,
    }

    let new_r = Rect { x: player.x, y: player.y, w: 8, h: 8 };
    let is_inbounds = new_r
      .iter_tiles()
      .all(|(tx, ty)| allows_movement(world[ty as usize][tx as usize]));

    let self_collision_occured =
      snake_objects[1..size as usize].iter().any(|node| {
        let node_r =
          Rect { x: node.position.x, y: node.position.y, w: 8, h: 8 };
        new_r.intersect(node_r)
      });

    let food_bounds =
      Rect { x: food_position.x, y: food_position.y, w: 8, h: 8 };

    let ate_food = new_r.intersect(food_bounds);
    if ate_food {
      snake_objects[size as usize].allocated = true;
      snake_objects[size as usize].position =
        snake_objects[size as usize - 1].position;
      match direction {
        DIR::UP => snake_objects[size as usize].position.y += 8,
        DIR::DOWN => snake_objects[size as usize].position.y -= 8,
        DIR::LEFT => snake_objects[size as usize].position.x += 8,
        DIR::RIGHT => snake_objects[size as usize].position.x -= 8,
      }
      size += 1;
      food_position.x = random_in_range(16, 220, rng.next_u16());
      food_position.y = random_in_range(16, 132, rng.next_u16());
    }
    if !is_inbounds || self_collision_occured {
      BG0CNT.write(BackgroundControl::new().with_screenblock(8));
      direction = INIT_DIR;
      size = INIT_SIZE;
      OBJ_ATTR_ALL.iter().for_each(|va| va.write(ObjAttr::new()));
      snake_objects = [SnakeNodeState::default(); 512];
      player = INIT_POS;
      snake_objects[0].allocated = true;
      let mut head_object_attr = ObjAttr::new();
      head_object_attr.set_x(INIT_POS.x);
      head_object_attr.set_y(INIT_POS.y);
      head_object_attr.set_tile_id(Cga8x8Thick::CIRCLE as u16);
      head_object_attr.set_palbank(0);
      OBJ_ATTR_ALL.index(HEAD_OBJECT_OFFSET).write(head_object_attr);
      food_position = Position {
        x: random_in_range(8, 220, rng.next_u16()),
        y: random_in_range(8, 132, rng.next_u16()),
      };
    }
    if size > 1 {
      snake_objects[0..size as usize].rotate_right(1);
    }
    tick = 0;
  }
}

const fn allows_movement(u: u8) -> bool {
  u == 0
}
