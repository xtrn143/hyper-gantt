//! 时间轴标记组件

use bevy::prelude::Component;

/// 时间轴标记组件
#[derive(Debug, Component)]
pub struct TimelineMarker {
    pub pixels_per_day: f32,
    pub current_scale: f32,
    pub target_scale: Option<f32>,
}

impl TimelineMarker {
    pub fn new(pixels_per_day: f32) -> Self {
        Self {
            pixels_per_day,
            current_scale: 1.0,
            target_scale: None,
        }
    }
}
