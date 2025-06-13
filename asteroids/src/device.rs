use gba::prelude::*;

pub fn init() {
  DISPSTAT.write(DisplayStatus::new().with_irq_vblank(true));
  IE.write(IrqBits::VBLANK);
  IME.write(true);

  BG_PALETTE.index(1).write(Color::WHITE);
  let colors =
    [Color::BLACK, Color::WHITE, Color::RED, Color::BLUE, Color::YELLOW];
  for (pal, color) in colors.iter().enumerate() {
    obj_palbank(pal).index(1).write(*color);
  }

  Cga8x8Thick.bitunpack_4bpp(CHARBLOCK0_4BPP.as_region(), 0);
  Cga8x8Thick.bitunpack_4bpp(OBJ_TILES.as_region(), 0);

  BG0CNT.write(BackgroundControl::new().with_screenblock(8));

  let no_display = ObjAttr0::new().with_style(ObjDisplayStyle::NotDisplayed);
  OBJ_ATTR0.iter().skip(512).for_each(|va| va.write(no_display));

  DISPCNT.write(DisplayControl::new().with_show_obj(true).with_show_bg0(true));
}
