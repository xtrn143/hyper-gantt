//! 任务布局系统

use crate::{
    ecs::components::{Dependency, TimelineMarker},
    task::{Path, Task, TaskLayout},
};
use bevy::prelude::*;

/// 任务布局系统
pub fn task_layout_system(
    mut query: Query<(&Task, &mut Transform), With<TimelineMarker>>,
    timeline: Res<TimelineMarker>,
    layout: Res<dyn TaskLayout>,
) {
    for (task, mut transform) in query.iter_mut() {
        let rect = layout.layout_task(task, &*timeline, 0); // TODO: 实现行索引计算
        transform.translation.x = rect.x;
        transform.translation.y = rect.y;
        transform.scale.x = rect.width;
        transform.scale.y = rect.height;
    }
}

/// 依赖关系布局系统
pub fn dependency_layout_system(
    mut query: Query<(&Dependency, &mut Path)>,
    task_query: Query<&Task>,
    timeline: Res<TimelineMarker>,
    layout: Res<dyn TaskLayout>,
) {
    for (dependency, mut path) in query.iter_mut() {
        if let (Ok(from_task), Ok(to_task)) = (
            task_query.get(dependency.source),
            task_query.get(dependency.target),
        ) {
            *path = layout.layout_dependency(from_task, to_task, &*timeline);
        }
    }
}
