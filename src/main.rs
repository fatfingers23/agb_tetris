#![no_std]
#![no_main]

extern crate alloc;

use agb::display::object::Object;
use agb::display::Priority;
use agb::fixnum::{num, Vector2D};
use agb::{
    display::{
        affine::AffineMatrix,
        object::{self, Graphics, OamManaged, Sprite, TagMap},
    },
    fixnum::Num,
};
use alloc::string::{String, ToString};
use alloc::vec::Vec;
mod entities;

static BLUE_Z: &Graphics = agb::include_aseprite!("gfx/cyan.aseprite");
static BLUE_Z_SPRITES: &[Sprite] = BLUE_Z.sprites();
static TAG_MAP: &TagMap = BLUE_Z.tags();

#[agb::entry]
fn main(mut gba: agb::Gba) -> ! {
    use agb::input::{Button, Tri};

    let gfx = gba.display.object.get_managed();

    let mut rotation: Num<i32, 16> = num!(0.);
    let rotation_matrix = AffineMatrix::from_rotation(rotation);
    let matrix = object::AffineMatrixInstance::new(rotation_matrix.to_object_wrapping());

    let mut falling_block = Vec::new();

    let mut block = gfx.object_sprite(&BLUE_Z_SPRITES[0]);
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
    let mut rotation: Num<i32, 16> = num!(0.);
    let rotation_speed = num!(0.1);

    loop {
        vblank.wait_for_vblank();
        input.update();

        if input.is_just_pressed(Button::A) {
            let rotation_matrix = AffineMatrix::from_rotation(rotation);
            for obj in falling_block.iter_mut() {
                //Turns the sprite around
                let idk = BLUE_Z_SPRITES.get(1).unwrap();
                let idk_two = gfx.sprite(idk);
                obj.set_sprite(idk_two);
            }
            rotation += rotation_speed;
        }

        if input.is_pressed(Button::RIGHT) || input.is_pressed(Button::LEFT) {
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

        if input.is_pressed(Button::DOWN) {
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
