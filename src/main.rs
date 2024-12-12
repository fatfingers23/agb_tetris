#![no_std]
#![no_main]

extern crate alloc;

use agb::fixnum::num;
use agb::{
    display::{
        affine::AffineMatrix,
        object::{self, Graphics, OamManaged, Sprite, TagMap},
    },
    fixnum::Num,
};
// use agb_fixnum::Num;
use alloc::vec::Vec;

static GRAPHICS: &Graphics = agb::include_aseprite!("gfx/cyan.aseprite");
static SPRITES: &[Sprite] = GRAPHICS.sprites();
static TAG_MAP: &TagMap = GRAPHICS.tags();

#[agb::entry]
fn main(mut gba: agb::Gba) -> ! {
    use agb::input::Tri;

    let gfx = gba.display.object.get_managed();

    let mut rotation: Num<i32, 16> = num!(0.);
    let rotation_matrix = AffineMatrix::from_rotation(rotation);
    let matrix = object::AffineMatrixInstance::new(rotation_matrix.to_object_wrapping());

    let mut falling_block = Vec::new();

    let mut block = gfx.object_sprite(&SPRITES[0]);
    block.set_affine_matrix(matrix.clone());
    block.show_affine(object::AffineMode::Affine);
    block.set_position((50, 50));
    falling_block.push(block);

    let vblank = agb::interrupt::VBlank::get();
    let mut input = agb::input::ButtonController::new();

    let mut game_tick = 0;
    let game_speed = 10;
    let mut current_game_speed = 10;
    let mut falling_pieced_moved = false;
    loop {
        vblank.wait_for_vblank();
        input.update();

        if input.is_pressed(agb::input::Button::RIGHT) || input.is_pressed(agb::input::Button::LEFT)
        {
            let x_tri = input.x_tri();
            for obj in falling_block.iter_mut() {
                falling_pieced_moved = true;

                if current_game_speed & game_tick == 2 {
                    match x_tri {
                        Tri::Positive => obj.set_position((obj.x() + 1, obj.y())),
                        Tri::Negative => obj.set_position((obj.x() - 1, obj.y())),
                        Tri::Zero => obj,
                    };
                }
            }
        }

        // if input.is_pressed(agb::input::Button::LEFT) {
        //     for obj in falling_block.iter_mut() {
        //         obj.set_position((obj.x() - 1, obj.y()));
        //     }
        // }

        if input.is_pressed(agb::input::Button::DOWN) {
            current_game_speed = 1;
        }

        if game_tick % current_game_speed == 0 && !falling_pieced_moved {
            for obj in falling_block.iter_mut() {
                obj.set_position((obj.x(), obj.y() + 1));
            }
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
