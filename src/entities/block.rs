use super::*;

static BLUE_Z: &Graphics = agb::include_aseprite!("gfx/cyan.aseprite");
static BLUE_Z_SPRITES: &[Sprite] = BLUE_Z.sprites();

#[derive(Clone)]
pub enum BlockRotation {
    Zero = 0,
    Ninety = 1,
    OneEighty = 2,
    TwoSeventy = 3,
}

pub struct Block<'a> {
    pub entity: Entity<'a>,
    block_type: BlockType,
    rotation: BlockRotation,
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
        }
    }

    /// Moves the block right(positive) or left(negative)
    pub fn move_block_x(&mut self, x: i32) {
        let previous_position = self.entity.position;
        self.entity.position = (previous_position.x + x, previous_position.y).into();
    }

    pub fn drop(&mut self, drop_amount: i32) {
        let previous_position = self.entity.position;
        self.entity.position = (previous_position.x, previous_position.y + drop_amount).into();
    }

    pub fn rotate(&mut self, object: &'a OamManaged) {
        self.rotation = match self.rotation.clone() {
            BlockRotation::Zero => BlockRotation::Ninety,
            BlockRotation::Ninety => BlockRotation::OneEighty,
            BlockRotation::OneEighty => BlockRotation::TwoSeventy,
            BlockRotation::TwoSeventy => BlockRotation::Zero,
        };

        let raw_sprite = BLUE_Z_SPRITES.get(self.rotation.clone() as usize).unwrap();
        let sprite = object.sprite(&raw_sprite);

        self.entity.sprite.set_sprite(sprite);
    }
}
