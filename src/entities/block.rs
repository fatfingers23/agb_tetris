use crate::constraints::{CONTAINER_LEFT, CONTAINER_RIGHT};

use super::*;

pub static BLUE_Z: &Graphics = agb::include_aseprite!("gfx/cyan.aseprite");
pub static BLUE_Z_SPRITES: &[Sprite] = BLUE_Z.sprites();

pub static YELLOW: &Graphics = agb::include_aseprite!("gfx/yellow.aseprite");
pub static YELLOW_SPRITES: &[Sprite] = YELLOW.sprites();

#[derive(Clone)]
pub enum BlockRotation {
    Zero = 0,
    Ninety = 1,
    OneEighty = 2,
    TwoSeventy = 3,
}

#[derive(Clone)]
pub enum BlockType {
    BlueZ,
    Yellow,
}

pub fn random_block_from_seed(seed: u32) -> BlockType {
    match seed % 2 {
        0 => BlockType::BlueZ,
        1 => BlockType::Yellow,
        _ => BlockType::BlueZ,
    }
}

#[allow(dead_code)]
impl BlockType {
    pub fn sprites(&self) -> &'static [Sprite] {
        match self {
            BlockType::BlueZ => BLUE_Z_SPRITES,
            BlockType::Yellow => YELLOW_SPRITES,
        }
    }

    pub fn as_str(&self) -> &str {
        match self {
            BlockType::BlueZ => "BlueZ",
            BlockType::Yellow => "Yellow",
        }
    }

    pub fn get_rotation_offset(&self, rotation: BlockRotation) -> Vector2D<i32> {
        match self {
            BlockType::BlueZ => match rotation {
                BlockRotation::Zero => Vector2D::new(0, 0),
                BlockRotation::Ninety => Vector2D::new(0, 0),
                BlockRotation::OneEighty => Vector2D::new(0, 0),
                BlockRotation::TwoSeventy => Vector2D::new(0, 0),
            },
            BlockType::Yellow => match rotation {
                BlockRotation::Zero => Vector2D::new(0, 0),
                BlockRotation::Ninety => Vector2D::new(0, 0),
                BlockRotation::OneEighty => Vector2D::new(0, 0),
                BlockRotation::TwoSeventy => Vector2D::new(0, 0),
            },
        }
    }
}

pub struct Block<'a> {
    pub entity: Entity<'a>,
    block_type: BlockType,
    rotation: BlockRotation,
    pub moving: bool,
}

impl<'a> Block<'a> {
    pub fn new(object: &'a OamManaged, block_type: BlockType) -> Self {
        let mut block_entity = Entity::new(
            object,
            (16, 16).into(),
            EntityType::Block(block_type.clone()),
        );
        block_entity.velocity.x = 0;
        block_entity.velocity.y = 1;
        block_entity.set_spawn((50, 50).into());
        block_entity.sprite.show();
        Self {
            entity: block_entity,
            block_type,
            rotation: BlockRotation::Zero,
            moving: true,
        }
    }

    /// Moves the block right(positive) or left(negative)
    pub fn move_block_x(&mut self, x: i32) {
        let previous_position = self.entity.position;
        let new_position = Vector2D::new(
            (previous_position.x + x).clamp(CONTAINER_LEFT, CONTAINER_RIGHT),
            previous_position.y,
        );

        self.entity.position = new_position.into();
    }

    pub fn drop(&mut self, drop_amount: i32) {
        let previous_position = self.entity.position;
        let new = Vector2D::new(previous_position.x, previous_position.y + drop_amount);
        if new.y >= 140 {
            self.moving = false;
        }
        self.entity.position = (previous_position.x, previous_position.y + drop_amount).into();
    }

    pub fn rotate(&mut self, object: &'a OamManaged) {
        self.rotation = match self.rotation.clone() {
            BlockRotation::Zero => BlockRotation::Ninety,
            BlockRotation::Ninety => BlockRotation::OneEighty,
            BlockRotation::OneEighty => BlockRotation::TwoSeventy,
            BlockRotation::TwoSeventy => BlockRotation::Zero,
        };
        let new_rotation_index = self.rotation.clone() as usize;
        let raw_sprite = &self.block_type.sprites()[new_rotation_index];
        let sprite = object.sprite(&raw_sprite);

        self.entity.sprite.set_sprite(sprite);
    }
}
