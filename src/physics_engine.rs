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
}
