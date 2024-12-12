use agb::display::object::{Graphics, OamManaged, Object, Sprite, TagMap};
use agb::display::Priority;
use agb::fixnum::{num, Vector2D};
use alloc::string::{String, ToString};

pub mod block;

static BLUE_Z: &Graphics = agb::include_aseprite!("gfx/cyan.aseprite");
static BLUE_Z_SPRITES: &[Sprite] = BLUE_Z.sprites();
static TAG_MAP: &TagMap = BLUE_Z.tags();

#[derive(Clone)]
pub enum BlockType {
    BlueZ,
}

#[derive(Clone)]
pub enum EntityType {
    Block(BlockType), //    Fruit(FruitType),
                      //    Arrow(Arrows),
}

/// A simple entity struct that holds the sprite and position for any sprite
pub struct Entity<'a> {
    pub type_of: EntityType,
    pub sprite: Object<'a>,
    pub position: Vector2D<i32>,
    pub velocity: Vector2D<i32>,
    pub collision_mask: Vector2D<i32>,
}

#[derive(Clone)]
pub struct CollisionEvent {
    pub direction: CollisionDirection,
}

/// impl of entity to allow for methods to interact with the sprite and setup
impl<'a> Entity<'a> {
    pub fn new(object: &'a OamManaged, collision_mask: Vector2D<i32>, type_of: EntityType) -> Self {
        //HACK picking the sprite for the entity. Feels real hacky
        let sprite = match type_of.clone() {
            EntityType::Block(block) => match block {
                BlockType::BlueZ => &BLUE_Z_SPRITES[0],

                _ => {
                    panic!("Not a valid fruit type");
                }
            },
            //            EntityType::Arrow(direction) => match direction {
            //                Down => Down.sprite(),
            //                _ => {
            //                    panic!("Not a valid arrow type");
            //                }
            //            },
            _ => {
                panic!("Not a valid entity type");
            }
        };

        let mut sprite_object = object.object_sprite(sprite);
        sprite_object.set_priority(Priority::P0);
        Entity {
            type_of,
            sprite: sprite_object,
            collision_mask,
            position: (0, 0).into(),
            velocity: (0, 0).into(),
        }
    }

    /// Updates the position of the sprite based on what has been set in the position variable
    pub fn update_sprite_position(&mut self) {
        self.sprite
            .set_x(self.position.x as u16)
            .set_y(self.position.y as u16);
    }

    /// Set where the entity should spawn the sprite
    pub fn set_spawn(&mut self, spawn: Vector2D<i32>) {
        self.position = spawn;
        self.sprite
            .set_x(self.position.x as u16)
            .set_y(self.position.y as u16);
    }

    pub fn collision_check(
        &mut self,
        other_pos: Vector2D<i32>,
        other_collision_mask: Vector2D<i32>,
    ) -> Option<CollisionEvent> {
        let x = (self.position.x - other_pos.x) * (self.position.x - other_pos.x);
        let y = (self.position.y - other_pos.y) * (self.position.y - other_pos.y);

        let dx = self.position.x - other_pos.x;
        let dy = self.position.y - other_pos.y;

        let collision_happened = (x + y)
            < ((self.collision_mask.x + self.collision_mask.y)
                * (other_collision_mask.x + other_collision_mask.y))
                .into();

        if collision_happened {
            let mut direction = CollisionDirection::None;

            if dx.abs() < dy.abs() {
                if dy > 0 {
                    direction = CollisionDirection::Down
                } else {
                    direction = CollisionDirection::Up
                }
            } else {
                if dx > 0 {
                    if direction.to_string() == CollisionDirection::Up.to_string() {
                        direction = CollisionDirection::UpperRight
                    } else if direction.to_string() == CollisionDirection::Down.to_string() {
                        direction = CollisionDirection::BottomRight
                    } else {
                        direction = CollisionDirection::Right
                    }
                    direction = CollisionDirection::Right
                } else {
                    if direction.to_string() == CollisionDirection::Up.to_string() {
                        direction = CollisionDirection::UpperLeft
                    } else if direction.to_string() == CollisionDirection::Down.to_string() {
                        direction = CollisionDirection::BottomLeft
                    } else {
                        direction = CollisionDirection::Left
                    }
                    direction = CollisionDirection::Left
                }
            };

            agb::println!("Collision direction: {:?}", direction);

            return Some(CollisionEvent { direction });
        }
        None
    }
}

#[derive(Clone, Debug)]
pub enum CollisionDirection {
    Up,
    Down,
    Left,
    Right,
    UpperRight,
    UpperLeft,
    BottomLeft,
    BottomRight,
    None,
}

impl CollisionDirection {
    pub fn to_string(&self) -> String {
        match self {
            CollisionDirection::Up => "Up".to_string(),
            CollisionDirection::Down => "Down".to_string(),
            CollisionDirection::Left => "Left".to_string(),
            CollisionDirection::Right => "Right".to_string(),
            CollisionDirection::UpperRight => "UpperRight".to_string(),
            CollisionDirection::UpperLeft => "UpperLeft".to_string(),
            CollisionDirection::BottomLeft => "BottomLeft".to_string(),
            CollisionDirection::BottomRight => "BottomRight".to_string(),
            CollisionDirection::None => "None".to_string(),
        }
    }
}
