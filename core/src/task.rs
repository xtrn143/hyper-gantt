//! 任务核心组件实现
//!
//! 提供任务数据结构和布局计算功能

use super::timeline::Timeline;
use std::collections::{HashMap, HashSet, VecDeque};

/// 任务ID类型
pub type TaskId = u64;

/// 任务数据结构
#[derive(Debug, Clone, bevy::prelude::Component)]
pub struct Task {
    pub id: TaskId,
    pub name: String,
    pub start: chrono::DateTime<chrono::Utc>,
    pub end: chrono::DateTime<chrono::Utc>,
    pub progress: f32,
    pub dependencies: HashSet<TaskId>,
    pub parent: Option<TaskId>,
}

/// 矩形区域定义
#[derive(Debug, Clone, Copy)]
pub struct Rect {
    pub x: f32,
    pub y: f32,
    pub width: f32,
    pub height: f32,
}

/// 路径点定义
#[derive(Debug, Clone)]
pub struct Path {
    pub points: Vec<(f32, f32)>,
    pub arrow_head: Option<(f32, f32)>,
}

/// 任务布局trait
pub trait TaskLayout {
    /// 计算任务条的布局矩形
    ///
    /// # 参数
    /// - `task`: 要布局的任务
    /// - `timeline`: 时间轴引用
    /// - `row_index`: 所在行索引
    ///
    /// # 返回
    /// 任务条的矩形区域
    fn layout_task(&self, task: &Task, timeline: &dyn Timeline, row_index: usize) -> Rect;

    /// 计算依赖关系的连线路径
    ///
    /// # 参数
    /// - `from`: 起始任务
    /// - `to`: 目标任务
    /// - `timeline`: 时间轴引用
    ///
    /// # 返回
    /// 连接路径定义
    fn layout_dependency(&self, from: &Task, to: &Task, timeline: &dyn Timeline) -> Path;

    /// 计算关键路径
    ///
    /// # 参数
    /// - `tasks`: 所有任务列表
    ///
    /// # 返回
    /// 关键路径上的任务ID列表
    fn calculate_critical_path(&self, tasks: &[Task]) -> Vec<TaskId>;
}

/// 基础任务布局实现
pub struct BasicTaskLayout {
    pub row_height: f32,
    pub bar_height: f32,
    pub h_padding: f32,
}

impl BasicTaskLayout {
    pub fn new(row_height: f32, bar_height: f32, h_padding: f32) -> Self {
        Self {
            row_height,
            bar_height,
            h_padding,
        }
    }
}

impl TaskLayout for BasicTaskLayout {
    fn layout_task(&self, task: &Task, timeline: &dyn Timeline, row_index: usize) -> Rect {
        let x = timeline.time_to_position(task.start);
        let width = timeline.time_to_position(task.end) - x;
        Rect {
            x,
            y: (row_index as f32) * self.row_height,
            width,
            height: self.bar_height,
        }
    }

    fn layout_dependency(&self, from: &Task, to: &Task, timeline: &dyn Timeline) -> Path {
        // 待实现具体算法
        Path {
            points: vec![],
            arrow_head: None,
        }
    }

    fn calculate_critical_path(&self, tasks: &[Task]) -> Vec<TaskId> {
        // 1. 构建任务图
        let mut graph = HashMap::new();
        let mut durations = HashMap::new();

        for task in tasks {
            durations.insert(task.id, (task.end - task.start).num_days() as i32);
            graph.insert(
                task.id,
                task.dependencies.iter().copied().collect::<Vec<_>>(),
            );
        }

        // 2. 拓扑排序
        let sorted = topological_sort(&graph).unwrap_or_else(|_| vec![]);

        // 3. 计算最早开始时间
        let mut earliest_start = HashMap::new();
        for &task_id in &sorted {
            let mut max_time = 0;
            for &dep_id in &graph[&task_id] {
                max_time = max_time.max(earliest_start[&dep_id] + durations[&dep_id]);
            }
            earliest_start.insert(task_id, max_time);
        }

        // 4. 计算最晚开始时间
        let total_duration = earliest_start.values().max().copied().unwrap_or(0);
        let mut latest_start = HashMap::new();
        for &task_id in sorted.iter().rev() {
            let mut min_time = total_duration;
            for (id, deps) in &graph {
                if deps.contains(&task_id) {
                    min_time = min_time.min(latest_start[id] - durations[&task_id]);
                }
            }
            latest_start.insert(task_id, min_time);
        }

        // 5. 识别关键路径
        let mut critical_path = Vec::new();
        for &task_id in &sorted {
            if earliest_start[&task_id] == latest_start[&task_id] {
                critical_path.push(task_id);
            }
        }

        critical_path
    }
}

/// 拓扑排序辅助函数
fn topological_sort(graph: &HashMap<TaskId, Vec<TaskId>>) -> Result<Vec<TaskId>, &'static str> {
    let mut in_degree = HashMap::new();
    let mut queue = VecDeque::new();
    let mut result = Vec::new();

    // 初始化入度
    for &node in graph.keys() {
        in_degree.insert(node, 0);
    }
    for deps in graph.values() {
        for &node in deps {
            *in_degree.entry(node).or_insert(0) += 1;
        }
    }

    // 入度为0的节点入队
    for (&node, &degree) in &in_degree {
        if degree == 0 {
            queue.push_back(node);
        }
    }

    // 拓扑排序
    while let Some(node) = queue.pop_front() {
        result.push(node);

        for &neighbor in &graph[&node] {
            let degree = in_degree.get_mut(&neighbor).unwrap();
            *degree -= 1;
            if *degree == 0 {
                queue.push_back(neighbor);
            }
        }
    }

    if result.len() == graph.len() {
        Ok(result)
    } else {
        Err("图中存在环")
    }
}
