use agb::display::object::{Graphics, OamManaged, Object, Sprite};
use agb::display::Priority;
use agb::fixnum::Vector2D;
use block::BlockType;

pub mod block;

#[derive(Clone)]
pub enum EntityType {
    Block(BlockType),
}

/// A simple entity struct that holds the sprite and position for any sprite
#[allow(dead_code)]
pub struct Entity<'a> {
    pub type_of: EntityType,
    pub sprite: Object<'a>,
    pub position: Vector2D<i32>,
    pub velocity: Vector2D<i32>,
    pub collision_mask: Vector2D<i32>,
}

#[allow(dead_code)]
#[derive(Clone)]
pub struct CollisionEvent {
    pub direction: CollisionDirection,
}

/// impl of entity to allow for methods to interact with the sprite and setup
impl<'a> Entity<'a> {
    pub fn new(object: &'a OamManaged, collision_mask: Vector2D<i32>, type_of: EntityType) -> Self {
        let sprite = match type_of.clone() {
            EntityType::Block(block) => &block.sprites()[0],
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

    pub fn _collision_check(
        &mut self,
        other_pos: Vector2D<i32>,
        other_collision_mask: Vector2D<i32>,
    ) -> Option<CollisionEvent> {
        //TODO Start from a old project I'm sure will be changed
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
                    if direction.as_str() == CollisionDirection::Up.as_str() {
                        direction = CollisionDirection::UpperRight
                    } else if direction.as_str() == CollisionDirection::Down.as_str() {
                        direction = CollisionDirection::BottomRight
                    } else {
                        direction = CollisionDirection::Right
                    }
                //                    direction = CollisionDirection::Right
                } else {
                    if direction.as_str() == CollisionDirection::Up.as_str() {
                        direction = CollisionDirection::UpperLeft
                    } else if direction.as_str() == CollisionDirection::Down.as_str() {
                        direction = CollisionDirection::BottomLeft
                    } else {
                        direction = CollisionDirection::Left
                    }
                    //                    direction = CollisionDirection::Left
                }
            };

            agb::println!("Collision direction: {:?}", direction);

            return Some(CollisionEvent { direction });
        }
        None
    }
}

#[allow(dead_code)]
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

#[allow(dead_code)]
impl CollisionDirection {
    pub fn as_str(&self) -> &str {
        match self {
            CollisionDirection::Up => "Up",
            CollisionDirection::Down => "Down",
            CollisionDirection::Left => "Left",
            CollisionDirection::Right => "Right",
            CollisionDirection::UpperRight => "UpperRight",
            CollisionDirection::UpperLeft => "UpperLeft",
            CollisionDirection::BottomLeft => "BottomLeft",
            CollisionDirection::BottomRight => "BottomRight",
            CollisionDirection::None => "None",
        }
    }
}
