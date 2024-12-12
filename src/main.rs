#![no_std]
#![no_main]

extern crate alloc;

use agb::fixnum::num;
use agb::{
    display::{
        affine::AffineMatrix,
        object::{self, Graphics, Sprite},
    },
    fixnum::Num,
};
use alloc::vec::Vec;
use entities::block::Block;

mod entities;

static BLUE_Z: &Graphics = agb::include_aseprite!("gfx/cyan.aseprite");
static BLUE_Z_SPRITES: &[Sprite] = BLUE_Z.sprites();
// static TAG_MAP: &TagMap = BLUE_Z.tags();

#[agb::entry]
fn main(mut gba: agb::Gba) -> ! {
    use agb::input::{Button, Tri};

    let gfx = gba.display.object.get_managed();

    let mut rotation: Num<i32, 16> = num!(0.);
    let rotation_matrix = AffineMatrix::from_rotation(rotation);
    let matrix = object::AffineMatrixInstance::new(rotation_matrix.to_object_wrapping());

    let mut falling_block: Vec<Block> = Vec::new();

    let mut block = Block::new(&gfx, entities::BlockType::BlueZ);

    falling_block.push(block);

    let vblank = agb::interrupt::VBlank::get();
    let mut input = agb::input::ButtonController::new();

    let mut game_tick = 0;
    let game_speed = 10;
    let mut current_game_speed = 10;
    let mut falling_pieced_moved = false;
    let mut rotation: Num<i32, 16> = num!(0.);
    let rotation_speed = num!(0.1);

    //TODO need to just have one falling block
    //Then when it lands stick it in a vec, Then a new falling block

    loop {
        vblank.wait_for_vblank();
        input.update();

        if input.is_just_pressed(Button::A) {
            for obj in falling_block.iter_mut() {
                obj.rotate(&gfx);
            }
            rotation += rotation_speed;
        }

        if input.is_pressed(Button::RIGHT) || input.is_pressed(Button::LEFT) {
            let x_tri = input.x_tri();
            for block in falling_block.iter_mut() {
                falling_pieced_moved = true;
                let previous_position = block.entity.position;
                if current_game_speed & game_tick == 2 {
                    match x_tri {
                        Tri::Positive => block.move_block_x(1),
                        Tri::Negative => block.move_block_x(-1),
                        Tri::Zero => block.entity.position = previous_position,
                    };
                }
            }
        }

        if input.is_pressed(Button::DOWN) {
            current_game_speed = 1;
        }

        if game_tick % current_game_speed == 0 && !falling_pieced_moved {
            for obj in falling_block.iter_mut() {
                obj.drop(1);
            }
        }

        //Updates the position of the sprite
        for obj in falling_block.iter_mut() {
            obj.entity.update_sprite_position();
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
