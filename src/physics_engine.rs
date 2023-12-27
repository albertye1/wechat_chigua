use bevy::ui::BackgroundColor;

use crate::vector2d::Vector2D;
pub struct PhysicsEngine {}
impl PhysicsEngine {
    pub fn new() -> PhysicsEngine {
        PhysicsEngine {}
    }
    pub fn test(&self) {
        let a = Vector2D::new(2., 3.);
        let b = Vector2D::new(1., 1.);
        let c = a + b;
        println!("A: {}, B:{}, A+B: {}", a, b, c);
    }
    // one tick of falling
    pub fn fall(pos: &mut Vector2D, vel: &mut Vector2D) {
        // println!("before; {} {}", pos.y(), vel.y());
        *pos = *pos + *vel;
        vel.set_y(vel.y() - 0.1);
        // println!("after; {} {}", pos.y(), vel.y());
    }

    // given two circles, test for collision,
    // and update positions and velocities of both if they collide
    pub fn collide(
        pos1: &mut Vector2D,
        vel1: &mut Vector2D,
        radius1: f32,
        pos2: &mut Vector2D,
        vel2: &mut Vector2D,
        radius2: f32,
    ) {
        let bounciness = 0.0;
        let vector = *pos2 - *pos1;
        let normal = vector.normalized();
        let vel_vector = *vel2 - *vel1;

        let depth = radius1 + radius2 - vector.magnitude();

        let mut diff = normal.x() * vel_vector.x() + normal.y() * vel_vector.y();
        diff = (-1.0 - bounciness) * (diff / 2.0);

        if depth > 0.0 {
            *pos1 = *pos1 + (normal * -depth / 2.0);
            *pos2 = *pos2 + (normal * depth / 2.0);

            *vel1 = *vel1 + normal * -diff;
            *vel2 = *vel2 + normal * diff;
        }
    }
}
