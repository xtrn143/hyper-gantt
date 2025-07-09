//! 节假日和工作日历配置
//!
//! 提供工作日判断和特殊日期配置功能

use chrono::NaiveDate;
use std::collections::HashMap;

/// 工作日历配置
#[derive(Debug, Clone)]
pub struct HolidayCalendar {
    /// 常规工作周配置 [周一至周日]
    pub work_week: [bool; 7],
    /// 特殊日期覆盖 (日期 -> 是否为工作日)
    pub exceptions: HashMap<NaiveDate, bool>,
}

impl HolidayCalendar {
    /// 创建默认日历 (周一到周五工作)
    pub fn new() -> Self {
        Self {
            work_week: [true, true, true, true, true, false, false],
            exceptions: HashMap::new(),
        }
    }

    /// 判断指定日期是否为工作日
    pub fn is_workday(&self, date: NaiveDate) -> bool {
        self.exceptions
            .get(&date)
            .copied()
            .unwrap_or(self.work_week[date.weekday().num_days_from_monday() as usize])
    }

    /// 添加节假日 (设置为非工作日)
    pub fn add_holiday(&mut self, date: NaiveDate) {
        self.exceptions.insert(date, false);
    }

    /// 添加调休工作日 (设置为工作日)
    pub fn add_workday(&mut self, date: NaiveDate) {
        self.exceptions.insert(date, true);
    }

    /// 批量导入节假日
    pub fn import_holidays(&mut self, dates: impl IntoIterator<Item = NaiveDate>) {
        for date in dates {
            self.add_holiday(date);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use chrono::NaiveDate;

    #[test]
    fn test_workday_calculation() {
        let mut calendar = HolidayCalendar::new();
        let test_date = NaiveDate::from_ymd(2025, 10, 1); // 周三

        // 默认是工作日
        assert!(calendar.is_workday(test_date));

        // 添加为节假日
        calendar.add_holiday(test_date);
        assert!(!calendar.is_workday(test_date));
    }
}
