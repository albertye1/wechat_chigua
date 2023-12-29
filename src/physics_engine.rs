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
        vel.set_y(vel.y() - 0.07);
        // println!("after; {} {}", pos.y(), vel.y());
    }

    fn mass(radius: f32) -> f32 {
        return radius * radius * radius;
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
        let bounciness = 0.2;
        let vector = *pos2 - *pos1;
        let normal = vector.normalized();
        let ortho = vector.normalized().ortho();
        let vel_vector = *vel2 - *vel1;

        let depth = radius1 + radius2 - vector.magnitude();

        let mut diff = normal.x() * vel_vector.x() + normal.y() * vel_vector.y();
        diff = (-1.0 - bounciness) * (diff / 2.0);

        let mass1 = Self::mass(radius1);
        let mass2 = Self::mass(radius2);
        let spd1 = (*vel1).dot(&normal);
        let spd2 = (*vel2).dot(&normal);
        let rej1 = (*vel1) - normal * spd1;
        let rej2 = (*vel2) - normal * spd2;

        let new_spd1 = (mass1 * spd1 + mass2 * spd2 + mass2 * bounciness * (spd2 - spd1)) / (mass1 + mass2);
        let new_spd2 = (mass1 * spd1 + mass2 * spd2 + mass1 * bounciness * (spd1 - spd2)) / (mass1 + mass2);

        if depth > 0.0 {
            /*
            println!(
                "pos1:{}, pos2:{}\n pos2-pos1:{} normalized:{}\n depth:{}\n total dist moved:{}",
                *pos1,
                *pos2,
                vector,
                normal,
                depth,
                (normal * (depth / 2.0))
            );
            */
            *pos1 = *pos1 + (normal * (-depth / 2.0));
            *pos2 = *pos2 + (normal * (depth / 2.0));

            *vel1 = rej1 + normal * new_spd1 * 0.5;
            *vel2 = rej2 + normal * new_spd2 * 0.5;
        }
    }
}
