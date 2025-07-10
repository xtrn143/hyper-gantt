//! 时间轴标记组件

use crate::timeline::Timeline;
use bevy::prelude::*;

/// 时间轴标记组件
#[derive(Debug, Component, Resource)]
pub struct TimelineMarker {
    pub pixels_per_day: f32,
    pub current_scale: f32,
    pub target_scale: Option<f32>,
}

impl Timeline for TimelineMarker {
    fn calculate_ticks(
        &self,
        range: std::ops::Range<chrono::DateTime<chrono::Utc>>,
    ) -> Vec<crate::timeline::Tick> {
        // 简单实现：每天一个刻度
        let mut ticks = Vec::new();
        let mut current = range.start;
        while current <= range.end {
            let days = (current - range.start).num_days() as f32;
            ticks.push(crate::timeline::Tick {
                position: days * self.pixels_per_day * self.current_scale,
                label: current.format("%m-%d").to_string(),
                level: crate::timeline::TickLevel::Major,
            });
            current = current + chrono::Duration::days(1);
        }
        ticks
    }

    fn time_to_position(&self, time: chrono::DateTime<chrono::Utc>) -> f32 {
        let days = (time - chrono::Utc::now()).num_days() as f32;
        days * self.pixels_per_day * self.current_scale
    }

    fn time_scale(&self) -> crate::timeline::TimeScale {
        crate::timeline::TimeScale::Days
    }
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
