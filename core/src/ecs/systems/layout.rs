//! 任务布局系统

use crate::ecs::components::path::Path;
use crate::task::TaskLayoutResource;
use crate::{
    ecs::{Dependency, TimelineMarker},
    task::{Task, TaskLayout},
};
use bevy::prelude::*;

/// 任务布局系统
pub fn task_layout_system(
    mut query: Query<(&Task, &mut Transform), With<TimelineMarker>>,
    timeline: Res<TimelineMarker>,
    layout: Res<TaskLayoutResource>,
) {
    for (task, mut transform) in query.iter_mut() {
        let rect = layout.0.layout_task(task, timeline.as_ref(), 0); // TODO: 实现行索引计算
        transform.translation.x = rect.x;
        transform.translation.y = rect.y;
        transform.scale.x = rect.width;
        transform.scale.y = rect.height;
    }
}

/// 依赖关系布局系统
pub fn dependency_layout_system(
    mut query: Query<(&Dependency, &mut Path)>,
    task_query: Query<(&Task, &Transform)>,
    timeline: Res<TimelineMarker>,
    layout: Res<TaskLayoutResource>,
) {
    for (dependency, mut path) in query.iter_mut() {
        if let (Ok((from_task, from_transform)), Ok((to_task, to_transform))) = (
            task_query.get(dependency.source),
            task_query.get(dependency.target),
        ) {
            let task_path = layout
                .0
                .layout_dependency(from_task, to_task, timeline.as_ref());
            let mut new_path: Path = task_path.into();

            // 添加箭头样式
            new_path.arrow_head = Some((
                to_transform.translation.x - 10.0, // 箭头X位置
                to_transform.translation.y,        // 箭头Y位置
            ));

            // 计算贝塞尔曲线控制点
            let control_x = (from_transform.translation.x + to_transform.translation.x) / 2.0;
            let control_y = from_transform.translation.y + 50.0; // 曲线高度

            new_path.points = vec![
                (from_transform.translation.x, from_transform.translation.y),
                (control_x, control_y),
                (to_transform.translation.x, to_transform.translation.y),
            ];

            *path = new_path;
        }
    }
}
