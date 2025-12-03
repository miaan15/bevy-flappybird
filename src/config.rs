use crate::define::Vec2f;

pub struct GameConfig {
    gravity: f32,
    speed: f32,
    jump_force: f32,
    gap_size: f32,
    pipe_width: f32,
    bird_size: Vec2f,
}
