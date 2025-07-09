//! 测试数据生成工具
//!
//! 提供大规模测试数据生成功能，用于性能测试和功能验证

use super::{holiday::HolidayCalendar, task::Task};
use chrono::{Datelike, Duration, NaiveDate, Utc};
use rand::rngs::StdRng;
use rand::{Rng, SeedableRng};
use std::collections::{HashMap, HashSet};

/// 生成测试数据集
pub fn generate_test_data() -> (Vec<Task>, HolidayCalendar) {
    let mut rng = StdRng::seed_from_u64(42); // 固定种子保证可重复性
    let task_count = 10_000;
    let max_depth = 5;
    let start_date = Utc::now().date_naive();
    let end_date = start_date + Duration::days(365);

    // 1. 生成中国节假日日历
    let mut calendar = generate_chinese_holiday_calendar(Utc::now().year());

    // 2. 生成任务数据
    let mut tasks = Vec::with_capacity(task_count);
    let mut id_counter = 0;
    let mut parent_stack = vec![(0, None)]; // (depth, parent_id)

    while !parent_stack.is_empty() && tasks.len() < task_count {
        let (depth, parent) = parent_stack.pop().unwrap();
        id_counter += 1;

        // 随机生成任务时间
        let duration_days = rng.gen_range(1..30);
        let start_offset = rng.gen_range(0..365 - duration_days);
        let start = start_date + Duration::days(start_offset);
        let end = start + Duration::days(duration_days);

        // 创建任务
        let task = Task {
            id: id_counter,
            name: format!("Task {}", id_counter),
            start: start.and_hms_opt(9, 0, 0).unwrap().and_utc(),
            end: end.and_hms_opt(18, 0, 0).unwrap().and_utc(),
            progress: rng.gen_range(0.0..1.0),
            dependencies: HashSet::new(),
            parent,
        };

        // 随机添加子任务
        if depth < max_depth && rng.gen_bool(0.3) {
            let child_count = rng.gen_range(1..4);
            for _ in 0..child_count {
                parent_stack.push((depth + 1, Some(task.id)));
            }
        }

        tasks.push(task);
    }

    // 3. 添加随机依赖关系（确保无循环）
    for i in 0..tasks.len() {
        if rng.gen_bool(0.2) {
            // 20%的任务有依赖
            let dep_count = rng.gen_range(1..3);
            for _ in 0..dep_count {
                let dep_index = rng.gen_range(0..i); // 只依赖前面的任务
                tasks[i].dependencies.insert(tasks[dep_index].id);
            }
        }
    }

    (tasks, calendar)
}

/// 生成中国节假日日历
fn generate_chinese_holiday_calendar(year: i32) -> HolidayCalendar {
    let mut calendar = HolidayCalendar::new();

    // 添加固定节假日（示例）
    let holidays = vec![
        NaiveDate::from_ymd_opt(year, 1, 1).unwrap(),  // 元旦
        NaiveDate::from_ymd_opt(year, 2, 10).unwrap(), // 春节
        NaiveDate::from_ymd_opt(year, 5, 1).unwrap(),  // 劳动节
        NaiveDate::from_ymd_opt(year, 10, 1).unwrap(), // 国庆节
    ];

    calendar.import_holidays(holidays);
    calendar
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_data_generation() {
        let (tasks, _) = generate_test_data();
        assert_eq!(tasks.len(), 10_000);

        // 检查依赖无循环
        for task in tasks {
            for dep in task.dependencies {
                assert!(dep < task.id, "依赖关系存在循环");
            }
        }
    }
}
