//! 测试工具和数据集

use crate::{
    ecs::components::timeline::TimelineMarker,
    task::{Task, TaskId},
};
use bevy::prelude::*;
use chrono::{DateTime, TimeZone, Utc};
use std::collections::HashSet;

/// 极端测试数据集
#[derive(Resource)]
pub struct ExtremeTestData {
    pub tasks: Vec<Task>,
    pub holidays: HashSet<DateTime<Utc>>,
    pub expected_critical_path: Vec<TaskId>,
}

impl ExtremeTestData {
    pub fn new() -> Self {
        // 定义节假日 (2025年国庆节)
        let mut holidays = HashSet::new();
        holidays.insert(Utc.with_ymd_and_hms(2025, 10, 1, 0, 0, 0).unwrap());

        // 创建测试任务
        let mut tasks = Vec::new();

        // 1. 跨年任务 (2025-12-20 至 2026-01-10)
        tasks.push(Task {
            id: Entity::from_raw(0),
            name: "跨年项目".to_string(),
            start: Utc.with_ymd_and_hms(2025, 12, 20, 0, 0, 0).unwrap(),
            end: Utc.with_ymd_and_hms(2026, 1, 10, 0, 0, 0).unwrap(),
            progress: 0.3,
            dependencies: HashSet::new(),
            parent: None,
        });

        // 2. 零时长任务 (开始=结束)
        tasks.push(Task {
            id: Entity::from_raw(1),
            name: "里程碑".to_string(),
            start: Utc.with_ymd_and_hms(2025, 7, 15, 0, 0, 0).unwrap(),
            end: Utc.with_ymd_and_hms(2025, 7, 15, 0, 0, 0).unwrap(),
            progress: 1.0,
            dependencies: HashSet::new(),
            parent: None,
        });

        // 3. 超长任务 (6个月)
        tasks.push(Task {
            id: Entity::from_raw(2),
            name: "长期项目".to_string(),
            start: Utc.with_ymd_and_hms(2025, 7, 1, 0, 0, 0).unwrap(),
            end: Utc.with_ymd_and_hms(2025, 12, 31, 0, 0, 0).unwrap(),
            progress: 0.1,
            dependencies: HashSet::new(),
            parent: None,
        });

        // 4. 重叠任务集 (3个任务)
        let mut overlapping_tasks = vec![
            Task {
                id: Entity::from_raw(3),
                name: "重叠任务A".to_string(),
                start: Utc.with_ymd_and_hms(2025, 8, 1, 0, 0, 0).unwrap(),
                end: Utc.with_ymd_and_hms(2025, 8, 15, 0, 0, 0).unwrap(),
                progress: 0.5,
                dependencies: HashSet::new(),
                parent: None,
            },
            Task {
                id: Entity::from_raw(4),
                name: "重叠任务B".to_string(),
                start: Utc.with_ymd_and_hms(2025, 8, 1, 0, 0, 0).unwrap(),
                end: Utc.with_ymd_and_hms(2025, 8, 20, 0, 0, 0).unwrap(),
                progress: 0.3,
                dependencies: HashSet::new(),
                parent: None,
            },
            Task {
                id: Entity::from_raw(5),
                name: "重叠任务C".to_string(),
                start: Utc.with_ymd_and_hms(2025, 8, 10, 0, 0, 0).unwrap(),
                end: Utc.with_ymd_and_hms(2025, 8, 25, 0, 0, 0).unwrap(),
                progress: 0.0,
                dependencies: HashSet::new(),
                parent: None,
            },
        ];
        tasks.extend(overlapping_tasks);

        // 5. 复杂依赖链 (环形依赖)
        let mut dependent_tasks = vec![
            Task {
                id: Entity::from_raw(6),
                name: "依赖任务A".to_string(),
                start: Utc.with_ymd_and_hms(2025, 9, 1, 0, 0, 0).unwrap(),
                end: Utc.with_ymd_and_hms(2025, 9, 10, 0, 0, 0).unwrap(),
                progress: 0.0,
                dependencies: HashSet::new(),
                parent: None,
            },
            Task {
                id: Entity::from_raw(7),
                name: "依赖任务B".to_string(),
                start: Utc.with_ymd_and_hms(2025, 9, 5, 0, 0, 0).unwrap(),
                end: Utc.with_ymd_and_hms(2025, 9, 15, 0, 0, 0).unwrap(),
                progress: 0.0,
                dependencies: HashSet::new(),
                parent: None,
            },
            Task {
                id: Entity::from_raw(8),
                name: "依赖任务C".to_string(),
                start: Utc.with_ymd_and_hms(2025, 9, 10, 0, 0, 0).unwrap(),
                end: Utc.with_ymd_and_hms(2025, 9, 20, 0, 0, 0).unwrap(),
                progress: 0.0,
                dependencies: HashSet::new(),
                parent: None,
            },
        ];

        // 设置环形依赖
        dependent_tasks[0].dependencies.insert(Entity::from_raw(7)); // A依赖B
        dependent_tasks[1].dependencies.insert(Entity::from_raw(8)); // B依赖C
        dependent_tasks[2].dependencies.insert(Entity::from_raw(6)); // C依赖A
        tasks.extend(dependent_tasks);

        Self {
            tasks,
            holidays,
            expected_critical_path: vec![], // 环形依赖无关键路径
        }
    }
}

/// 创建测试用的时间轴配置
pub fn test_timeline() -> TimelineMarker {
    TimelineMarker {
        pixels_per_day: 10.0,
        current_scale: 1.0,
        target_scale: None,
    }
}
