#![no_std]
#![no_main]

extern crate alloc;

use agb::display::tiled::{RegularMap, TiledMap, VRamManager};
use agb::input::{Button, Tri};

use alloc::vec::Vec;
use backgrounds::game_ui;
use entities::block::BlockType;
use entities::block::{random_block_from_seed, Block};

mod constraints;
mod entities;

agb::include_background_gfx!(backgrounds,
    game_ui => deduplicate "gfx/game_background_ui.png",
);

#[agb::entry]
fn main(mut gba: agb::Gba) -> ! {
    use agb::display::{
        tiled::{RegularBackgroundSize, TileFormat},
        Priority,
    };

    agb::println!("Hello world");
    let gfx = gba.display.object.get_managed();
    let (tiled, mut vram) = gba.display.video.tiled0();

    let mut ui_bg = tiled.background(
        Priority::P0,
        RegularBackgroundSize::Background32x32,
        TileFormat::FourBpp,
    );

    show_game_ui(&mut ui_bg, &mut vram);

    let mut fallen_blocks: Vec<Block> = Vec::new();

    let mut falling_block = Block::new(&gfx, BlockType::BlueZ);

    let vblank = agb::interrupt::VBlank::get();
    let mut input = agb::input::ButtonController::new();

    let mut game_tick = 0;
    let game_speed = 10;
    let mut current_game_speed = 10;
    let mut falling_pieced_moved = false;

    loop {
        vblank.wait_for_vblank();
        input.update();

        if input.is_just_pressed(Button::A) {
            falling_block.rotate(&gfx);
        }

        if input.is_pressed(Button::RIGHT) || input.is_pressed(Button::LEFT) {
            let x_tri = input.x_tri();
            // for block in falling_block.iter_mut() {
            falling_pieced_moved = true;
            let previous_position = falling_block.entity.position;
            if current_game_speed & game_tick == 2 {
                match x_tri {
                    Tri::Positive => falling_block.move_block_x(1),
                    Tri::Negative => falling_block.move_block_x(-1),
                    Tri::Zero => falling_block.entity.position = previous_position,
                };
            }
            // }
        }

        if input.is_pressed(Button::DOWN) {
            current_game_speed = 1;
        }

        if game_tick % current_game_speed == 0 && !falling_pieced_moved {
            falling_block.drop(1);
        }

        //Updates the position of the sprite

        falling_block.entity.update_sprite_position();

        if !falling_block.moving {
            fallen_blocks.push(falling_block);
            let new_block_type = random_block_from_seed(game_tick);
            falling_block = Block::new(&gfx, new_block_type);
        }

        //Reset loop state and increment game tick
        game_tick += 1;
        if game_speed != current_game_speed {
            current_game_speed = game_speed;
        }
        falling_pieced_moved = false;
        gfx.commit();
    }
}

pub fn show_game_ui(map: &mut RegularMap, vram: &mut VRamManager) {
    map.set_scroll_pos((0i16, 0i16));

    let vblank = agb::interrupt::VBlank::get();

    vblank.wait_for_vblank();

    map.fill_with(vram, &game_ui);

    map.commit(vram);
    vram.set_background_palettes(backgrounds::PALETTES);
    map.set_visible(true);
}
