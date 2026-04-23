// src/traits.rs
use macroquad::prelude::Rect;

pub trait Colisionable {
    fn get_rect(&self) -> Rect;
}

pub trait Entity {
    fn update(&mut self, dt: f32, nivel: &crate::level::Level);
    fn draw(&self);
}

pub fn detect_collision<A: Colisionable, B: Colisionable>(obj_a: &A, obj_b: &B) -> bool {
    obj_a.get_rect().overlaps(&obj_b.get_rect())
}
