#![feature(const_fn_floating_point_arithmetic)]
//! Hyper Gantt 核心库

pub mod animation;
pub mod ecs;
pub mod holiday;
pub mod task;
pub mod test_utils;
pub mod timeline;

use bevy::prelude::*;
use ecs::{dependency_layout_system, task_layout_system};

/// 核心插件
pub struct HyperGanttPlugin;

impl Plugin for HyperGanttPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            Update,
            (ecs::task_layout_system, ecs::dependency_layout_system),
        );
    }
}
