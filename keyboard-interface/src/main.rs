#![no_std]
#![no_main]

use gba::prelude::*;

static KEYBOARD_PROMPT_BUFFER: [GbaCell<u8>; 11] = [
    GbaCell::new(0),
    GbaCell::new(0),
    GbaCell::new(0),
    GbaCell::new(0),
    GbaCell::new(0),
    GbaCell::new(0),
    GbaCell::new(0),
    GbaCell::new(0),
    GbaCell::new(0),
    GbaCell::new(0),
    GbaCell::new(0),
];

static KEYBOARD_INPUT_LENGTH: GbaCell<u8> = GbaCell::new(0);
static KEYBOARD_INPUT_BUFFER: [GbaCell<u8>; 8] = [
    GbaCell::new(0),
    GbaCell::new(0),
    GbaCell::new(0),
    GbaCell::new(0),
    GbaCell::new(0),
    GbaCell::new(0),
    GbaCell::new(0),
    GbaCell::new(0),
];

static PLAYER_NAME_LENGTH: GbaCell<u8> = GbaCell::new(0);
static PLAYER_NAME_BUFFER: [GbaCell<u8>; 8] = [
    GbaCell::new(0),
    GbaCell::new(0),
    GbaCell::new(0),
    GbaCell::new(0),
    GbaCell::new(0),
    GbaCell::new(0),
    GbaCell::new(0),
    GbaCell::new(0),
];

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
struct Position {
  x: u16,
  y: u16,
}

fn index_to_tile_id(index: u16) -> u16{
    match index {
      0 => 65,
      1 => 66,
      2 => 67,
      3 => 68,
      4 => 69,
      5 => 70,
      6 => 71,
      7 => 72,
      8 => 73,
      9 => 74,
      10 => 75,
      11 => 76,
      12 => 77,
      13 => 78,
      14 => 79,
      15 => 80,
      16 => 81,
      17 => 82,
      18 => 83,
      19 => 84,
      20 => 85,
      21 => 86,
      22 => 87,
      23 => 88,
      24 => 89,
      25 => 90,
      26 => 48,
      27 => 49,
      28 => 50,
      29 => 51,
      30 => 52,
      31 => 53,
      32 => 54,
      33 => 55,
      34 => 56,
      35 => 57,
      36 => 27,
      37 => 26,
      _ => 0,
    }
}

fn keyboard_ix_task(){
  let world = [[0_u8; 32]; 32];
  let mut key_object_index = [[0_u8; 32]; 32];
  DISPSTAT.write(DisplayStatus::new().with_irq_vblank(true));
  IE.write(IrqBits::VBLANK);
  IME.write(true);

  BG_PALETTE.index(1).write(Color::WHITE);
  let colors =
    [Color::WHITE, Color(0b0_11111_11111_111111), Color::RED, Color::BLUE, Color::YELLOW];
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

 
  let prompt_positions: [Position; 11] = [
    Position { x: 10, y: 8 },
    Position { x: 11, y: 8 },
    Position { x: 12, y: 8 },
    Position { x: 13, y: 8 },
    Position { x: 14, y: 8 },
    Position { x: 15, y: 8 },
    Position { x: 16, y: 8 },
    Position { x: 17, y: 8 },
    Position { x: 18, y: 8 },
    Position { x: 19, y: 8 },
    Position { x: 20, y: 8 },
  ];

  let input_positions: [Position; 8] = [
    Position { x: 10, y: 10 },
    Position { x: 11, y: 10 },
    Position { x: 12, y: 10 },
    Position { x: 13, y: 10 },
    Position { x: 14, y: 10 },
    Position { x: 15, y: 10 },
    Position { x: 16, y: 10 },
    Position { x: 17, y: 10 },
  ];

  let keyboard_positions: [Position; 38] = [
        Position { x: 10, y: 15 },
        Position { x: 11, y: 15 },
        Position { x: 12, y: 15 },
        Position { x: 13, y: 15 },
        Position { x: 14, y: 15 },
        Position { x: 15, y: 15 },
        Position { x: 16, y: 15 },
        Position { x: 10, y: 16 },
        Position { x: 11, y: 16 },
        Position { x: 12, y: 16 },
        Position { x: 13, y: 16 },
        Position { x: 14, y: 16 },
        Position { x: 15, y: 16 },
        Position { x: 16, y: 16 },
        Position { x: 10, y: 17 },
        Position { x: 11, y: 17 },
        Position { x: 12, y: 17 },
        Position { x: 13, y: 17 },
        Position { x: 14, y: 17 },
        Position { x: 15, y: 17 },
        Position { x: 16, y: 17 },
        Position { x: 11, y: 18 },
        Position { x: 12, y: 18 },
        Position { x: 13, y: 18 },
        Position { x: 14, y: 18 },
        Position { x: 15, y: 18 },
        Position { x: 18, y: 15 },
        Position { x: 19, y: 15 },
        Position { x: 20, y: 15 },
        Position { x: 18, y: 16 },
        Position { x: 19, y: 16 },
        Position { x: 20, y: 16 },
        Position { x: 18, y: 17 },
        Position { x: 19, y: 17 },
        Position { x: 20, y: 17 },
        Position { x: 19, y: 18 },
        Position { x: 10, y: 18 },
        Position { x: 16, y: 18 },
    ];

  for (i, pos) in prompt_positions.iter().enumerate() {
    let mut obj = ObjAttr::new();
    obj.set_x(pos.x*8);
    obj.set_y(pos.y*8);
    obj.set_tile_id(KEYBOARD_PROMPT_BUFFER[i].read() as u16);
    obj.set_palbank(1);
    OBJ_ATTR_ALL.index(i).write(obj);
  }

  let mut cursor_position = Position { x: 10, y: 15 };
  let mut cursor_object = ObjAttr::new();
  cursor_object.set_x(cursor_position.x*8);
  cursor_object.set_y((cursor_position.y*8)+1);
  cursor_object.set_tile_id(64);
  cursor_object.set_palbank(1);

  let mut input_index = 0;
  let mut input_buffer = [0u8; 11];
  let mut input_objects = [ObjAttr::new(); 11];
  input_positions.iter().enumerate().for_each(|(i, pos)| {
    let tile_id = 255;
    input_objects[i].set_x(pos.x*8);
    input_objects[i].set_y(pos.y*8);
    input_objects[i].set_tile_id(tile_id as u16);
    input_objects[i].set_palbank(0);
  });

  let mut keyboard_objects = [ObjAttr::new(); 38];
  keyboard_positions.iter().enumerate().for_each(|(i, pos)| {
    let tile_id = index_to_tile_id(i as u16);
    keyboard_objects[i].set_x(pos.x*8);
    keyboard_objects[i].set_y(pos.y*8);
    keyboard_objects[i].set_tile_id(tile_id as u16);
    keyboard_objects[i].set_palbank(0);
  });


  let mut prompt_objects = [ObjAttr::new(); 11];
  prompt_positions.iter().enumerate().for_each(|(i, pos)| {
    let tile_id = KEYBOARD_PROMPT_BUFFER[i].read();
    prompt_objects[i].set_x(pos.x*8);
    prompt_objects[i].set_y(pos.y*8);
    prompt_objects[i].set_tile_id(tile_id as u16);
    prompt_objects[i].set_palbank(1);
  });

  keyboard_positions.iter().enumerate().for_each(|(i, pos)| {
    let tile_id = index_to_tile_id(i as u16);
    key_object_index[pos.x as usize][pos.y as usize] = tile_id as u8;
  });

  let mut tick = 0;
  let mut pressed = false;
  let mut done = false;
      loop {
    VBlankIntrWait();
    if tick == 15 {
        tick = 0;
        pressed = false;
    }

    let keys = KEYINPUT.read();

    for (i, obj) in keyboard_objects.iter_mut().enumerate() {
      OBJ_ATTR_ALL.index(1+11+i).write(obj.clone());
    }
    
    for (i, obj) in input_objects.iter_mut().enumerate() {
      OBJ_ATTR_ALL.index(1+i).write(obj.clone());
    }

    for (i, obj) in prompt_objects.iter().enumerate() {
      OBJ_ATTR_ALL.index(1+11+38+i).write(obj.clone());
    }

    if tick < 7 {
      cursor_object.set_tile_id(95);
    } else {
      cursor_object.set_tile_id(255);
    }
    OBJ_ATTR_ALL.index(0).write(cursor_object.clone());

    if keys.up() && !pressed {
        if cursor_position.y > 15 {
            cursor_position.y -= 1;
        }
        pressed = true;
        tick = 0;
    }
    if keys.down() && !pressed {
        if cursor_position.y < 18 {
            cursor_position.y += 1;
        }
        pressed = true;
        tick = 0;
    }
    if keys.left() && !pressed {
        if cursor_position.x > 10 {
            cursor_position.x -= 1;
        }
        pressed = true;
        tick = 0;
    }
    if keys.right() && !pressed {
        if cursor_position.x < 20 {
            cursor_position.x += 1;
        }
        pressed = true;
        tick = 0;
    }
    if keys.a() && !pressed {
        if input_index < 11 {
            let x = cursor_position.x as usize;
            let y = cursor_position.y as usize;
            if x == 10 && y == 18 {
                if input_index > 0 {
                    input_objects[input_index].set_tile_id(0);
                    OBJ_ATTR_ALL.index(input_index).write(input_objects[input_index].clone());
                    input_index -= 1;
                    pressed = true;
                    tick = 0;
                }
                continue;
            }
            if x == 16 && y == 18 {
                done = true;
                KEYBOARD_INPUT_LENGTH.write(input_index as u8);
                for i in 0..input_index {
                    KEYBOARD_INPUT_BUFFER[i].write(input_buffer[i]);
                }
                continue;
            }
            input_objects[input_index].set_tile_id(key_object_index[x][y] as u16);
            input_buffer[input_index] = key_object_index[x][y];
            input_index += 1;
        } 
        pressed = true;
        tick = 0;
    }
    cursor_object.set_x(cursor_position.x*8);
    cursor_object.set_y(cursor_position.y*8);
    tick += 1;
    if done {
        break;
    }
  }
}

static PENDING_WRITES: [GbaCell::<u8>; 64] = [
    GbaCell::new(0),
    GbaCell::new(0),
    GbaCell::new(0),
    GbaCell::new(0),
    GbaCell::new(0),
    GbaCell::new(0),
    GbaCell::new(0),
    GbaCell::new(0),
    GbaCell::new(0),
    GbaCell::new(0),
    GbaCell::new(0),
    GbaCell::new(0),
    GbaCell::new(0),
    GbaCell::new(0),
    GbaCell::new(0),
    GbaCell::new(0),
    GbaCell::new(0),
    GbaCell::new(0),
    GbaCell::new(0),
    GbaCell::new(0),
    GbaCell::new(0),
    GbaCell::new(0),
    GbaCell::new(0),
    GbaCell::new(0),
    GbaCell::new(0),
    GbaCell::new(0),
    GbaCell::new(0),
    GbaCell::new(0),
    GbaCell::new(0),
    GbaCell::new(0),
    GbaCell::new(0),
    GbaCell::new(0),
    GbaCell::new(0),
    GbaCell::new(0),
    GbaCell::new(0),
    GbaCell::new(0),
    GbaCell::new(0),
    GbaCell::new(0),
    GbaCell::new(0),
    GbaCell::new(0),
    GbaCell::new(0),    
    GbaCell::new(0),
    GbaCell::new(0),
    GbaCell::new(0),
    GbaCell::new(0),
    GbaCell::new(0),
    GbaCell::new(0),
    GbaCell::new(0),
    GbaCell::new(0),
    GbaCell::new(0),
    GbaCell::new(0),
    GbaCell::new(0),
    GbaCell::new(0),
    GbaCell::new(0),
    GbaCell::new(0),
    GbaCell::new(0),
    GbaCell::new(0),
    GbaCell::new(0),
    GbaCell::new(0),
    GbaCell::new(0),
    GbaCell::new(0),
    GbaCell::new(0),
    GbaCell::new(0),
    GbaCell::new(0),
];

static WRITE_INDEX: GbaCell<u8> = GbaCell::new(0);
static OUTPUT_CHANGED: GbaCell<bool> = GbaCell::new(false);

#[no_mangle]
extern "C" fn main() -> ! {
    let prompt = [b'N',b'A', b'M', b'E'];
    for (i, c) in prompt.iter().enumerate() {
        KEYBOARD_PROMPT_BUFFER[i].write(*c); 
    }
    KEYBOARD_PROMPT_BUFFER[prompt.len()].write(b':');
    keyboard_ix_task();
    DISPSTAT.write(DisplayStatus::new().with_irq_vblank(true));
    IE.write(IrqBits::VBLANK);
    IME.write(true);
    let mut world = [[0_u8; 32]; 32];
    let message_suffix = [b',', b' ', b'W', b'E', b'L', b'C', b'O', b'M', b'E', b'!', b' ', b'H', b'O', b'W', b' ', b'M', b'A', b'N', b'Y', b' ', b'T', b'H', b'I', b'N', b'G',b'S', b' ', b'W', b'I', b'L', b'L', b' ', b'Y', b'O', b'U', b' ', b'B', b'U', b'Y', b'?', b'?'];
    for i in 0..KEYBOARD_INPUT_LENGTH.read() {
        PENDING_WRITES[i as usize].write(KEYBOARD_INPUT_BUFFER[i as usize].read());
    }
    for i in 0..message_suffix.len() {
        PENDING_WRITES[KEYBOARD_INPUT_LENGTH.read() as usize + i as usize].write(message_suffix[i]);
    }
    BG0CNT.write(BackgroundControl::new().with_screenblock(8));
    let screenblock = TEXT_SCREENBLOCKS.get_frame(8).unwrap();
    for y in 0..32 {
        let row = screenblock.get_row(y).unwrap();
        for (x, addr) in row.iter().enumerate() {
            let te = TextEntry::new().with_tile(world[y][x] as u16);
            addr.write(te);
        }
    }
    DISPCNT.write(DisplayControl::new().with_show_bg0(true));
    let mut tick = 0;
    let mut done = false;
    loop {
        VBlankIntrWait();
        if tick == 5 {
            if OUTPUT_CHANGED.read() {
                OUTPUT_CHANGED.write(false);
                for y in 0..32 {
                    let row = screenblock.get_row(y).unwrap();
                    for (x, addr) in row.iter().enumerate() {
                        let te = TextEntry::new().with_tile(world[y][x] as u16);
                        addr.write(te);
                    }
                }
            }
            if WRITE_INDEX.read() > 64 {
                WRITE_INDEX.write(0);
                break;
            }
            if PENDING_WRITES[WRITE_INDEX.read() as usize+1].read() != 0 {
                world[((WRITE_INDEX.read() as usize) / 30)+15][(WRITE_INDEX.read() as usize % 30)] = PENDING_WRITES[WRITE_INDEX.read() as usize].read();
                PENDING_WRITES[WRITE_INDEX.read() as usize].write(0);
                WRITE_INDEX.write(WRITE_INDEX.read()+1);
                OUTPUT_CHANGED.write(true);
            } else {
                if WRITE_INDEX.read() > 0 {
                    done = true;
                }
                WRITE_INDEX.write(0);
                OUTPUT_CHANGED.write(false);
            }
            tick = 0;
        }
        tick += 1;
        if done {
            break;
        }
    }
    loop {}
}
