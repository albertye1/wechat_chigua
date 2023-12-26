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
    pub fn fall(mut pos: Vector2D, mut vel: Vector2D) {
        pos = pos + vel;
        vel.set_y(vel.y() - 1.0);
    }
}
