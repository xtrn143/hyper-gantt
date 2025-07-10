use bevy::prelude::*;

/// 路径组件
#[derive(Debug, Component)]
pub struct Path {
    pub points: Vec<(f32, f32)>,
    pub arrow_head: Option<(f32, f32)>,
}

impl Path {
    pub fn new() -> Self {
        Self {
            points: Vec::new(),
            arrow_head: None,
        }
    }
}
