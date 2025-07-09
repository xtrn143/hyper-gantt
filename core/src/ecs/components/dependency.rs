//! 任务依赖关系组件

use crate::task::TaskId;
use bevy::prelude::Component;

/// 任务依赖关系组件
#[derive(Debug, Component)]
pub struct Dependency {
    pub source: TaskId,
    pub target: TaskId,
    pub path: Option<Vec<(f32, f32)>>,
}

impl Dependency {
    pub fn new(source: TaskId, target: TaskId) -> Self {
        Self {
            source,
            target,
            path: None,
        }
    }
}
