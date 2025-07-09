//! MVP功能集成测试模块

use super::{
    ecs::{components::*, systems::*},
    task::{BasicTaskLayout, Task},
    test_utils::generate_test_data,
    timeline::TimeScale,
};
use bevy::prelude::*;
use std::collections::{HashMap, VecDeque};

/// Bevy测试插件
struct MvpTestPlugin;

impl Plugin for MvpTestPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, setup_test_data)
            .add_systems(Update, (task_layout_system, dependency_layout_system));
    }
}

fn setup_test_data(mut commands: Commands) {
    let (tasks, calendar) = generate_test_data();

    // 添加时间轴
    commands.spawn(TimelineMarker::new(10.0));

    // 添加任务实体
    for task in tasks {
        commands.spawn((task, Transform::default(), GlobalTransform::default()));
    }

    // 添加布局
    commands.insert_resource(BasicTaskLayout::new(30.0, 20.0, 5.0));
}

#[test]
fn run_mvp_test() {
    App::new()
        .add_plugins((MinimalPlugins, MvpTestPlugin))
        .run();
}

#[test]
fn test_critical_path() {
    let (tasks, _) = generate_test_data();
    let layout = BasicTaskLayout::new(30.0, 20.0, 5.0);
    let critical_path = layout.calculate_critical_path(&tasks);

    // 验证关键路径不为空
    assert!(!critical_path.is_empty(), "关键路径计算失败");

    // 验证关键路径上的任务顺序
    let mut prev_end = None;
    for &task_id in &critical_path {
        let task = tasks.iter().find(|t| t.id == task_id).unwrap();
        if let Some(end) = prev_end {
            assert!(task.start.timestamp() >= end, "关键路径任务顺序错误");
        }
        prev_end = Some(task.end.timestamp());
    }
}
