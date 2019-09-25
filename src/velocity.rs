use amethyst::{
    core::{math::Vector3, transform::Transform},
    ecs::prelude::{Component, DenseVecStorage, Join, ReadStorage, System, WriteStorage},
};

#[derive(Component)]
pub struct Velocity {
    pub vector: Vector3<f32>,
}

impl Velocity {
    pub fn new(vector: Vector3<f32>) -> Self {
        Velocity { vector: vector }
    }
}

pub struct VelocitySystem;

impl<'a> System<'a> for VelocitySystem {
    type SystemData = (WriteStorage<'a, Transform>, ReadStorage<'a, Velocity>);

    fn run(&mut self, data: Self::SystemData) {
        let (mut transforms, velocities) = data;
        for (transform, velocity) in (&mut transforms, &velocities).join() {
            transform.append_translation(velocity.vector);
        }
    }
}
