//! 时间轴核心组件实现
//!
//! 提供时间刻度计算、时间-位置转换等功能

use crate::{animation::ZoomAnimation, holiday::HolidayCalendar};
use chrono::{DateTime, Datelike, Duration, NaiveDate, TimeZone, Timelike, Utc};
use std::ops::Range;

/// 时间精度配置
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum TimePrecision {
    Seconds,     // 秒级精度
    Minutes,     // 分钟级精度
    Hours,       // 小时级精度
    Days,        // 天级精度(默认)
    Custom(u32), // 自定义秒数精度(最小60秒)
}

/// 时间单位枚举
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum TimeScale {
    Days,
    Weeks,
    Months,
    Quarters,
}

/// 刻度级别
#[derive(Debug)]
pub enum TickLevel {
    Major,
    Minor,
}

/// 时间刻度结构
#[derive(Debug)]
pub struct Tick {
    pub position: f32,
    pub label: String,
    pub level: TickLevel,
}

/// 时间轴核心trait
pub trait Timeline {
    /// 计算指定时间范围内的刻度
    ///
    /// # 参数
    /// - `range`: 时间范围
    ///
    /// # 返回
    /// 刻度列表，按位置排序
    fn calculate_ticks(&self, range: Range<DateTime<Utc>>) -> Vec<Tick>;

    /// 将时间转换为像素位置
    ///
    /// # 参数
    /// - `time`: 要转换的时间点
    ///
    /// # 返回
    /// 对应的像素位置
    fn time_to_position(&self, time: DateTime<Utc>) -> f32;

    /// 获取当前使用的时间单位
    fn time_scale(&self) -> TimeScale;
}

/// 基础时间轴实现
#[derive(Debug, Clone)]
pub struct TickStyle {
    pub major_color: String,
    pub minor_color: String,
    pub major_width: f32,
    pub minor_width: f32,
    pub label_font: String,
    pub label_size: f32,
    pub label_color: String,
}

impl Default for TickStyle {
    fn default() -> Self {
        Self {
            major_color: "#333".into(),
            minor_color: "#999".into(),
            major_width: 2.0,
            minor_width: 1.0,
            label_font: "Arial".into(),
            label_size: 12.0,
            label_color: "#666".into(),
        }
    }
}

pub struct BasicTimeline {
    pixels_per_day: f32,
    scale: TimeScale,
    precision: TimePrecision,
    calendar: HolidayCalendar,
    timezone: Option<i32>,
    tick_style: TickStyle,
    current_animation: Option<ZoomAnimation>,
}

impl BasicTimeline {
    pub fn new(pixels_per_day: f32, scale: TimeScale) -> Self {
        Self {
            pixels_per_day,
            scale,
            precision: TimePrecision::Days,
            calendar: HolidayCalendar::new(),
            timezone: None,
            tick_style: TickStyle::default(),
            current_animation: None,
        }
    }

    pub fn with_tick_style(mut self, style: TickStyle) -> Self {
        self.tick_style = style;
        self
    }

    pub fn start_zoom(&mut self, target: TimeScale) {
        self.current_animation = Some(ZoomAnimation::new(target, self.scale));
    }

    pub fn update_animation(&mut self, delta: Duration) -> bool {
        if let Some(anim) = &mut self.current_animation {
            if anim.update(delta) {
                self.scale = anim.current_scale();
                self.current_animation = None;
                true
            } else {
                self.scale = anim.current_scale();
                false
            }
        } else {
            true
        }
    }

    /// 使用自定义工作日历
    pub fn with_calendar(mut self, calendar: HolidayCalendar) -> Self {
        self.calendar = calendar;
        self
    }

    /// 设置时间精度
    pub fn with_precision(mut self, precision: TimePrecision) -> Self {
        self.precision = precision;
        self
    }

    /// 设置时区(UTC偏移分钟数)
    pub fn with_timezone(mut self, offset_minutes: i32) -> Self {
        self.timezone = Some(offset_minutes);
        self
    }

    /// 应用时间精度处理
    /// 检查是否为工作日
    pub fn is_workday(&self, date: NaiveDate) -> bool {
        self.calendar.is_workday(date)
    }

    fn apply_precision(&self, time: DateTime<Utc>) -> DateTime<Utc> {
        match self.precision {
            TimePrecision::Seconds => time,
            TimePrecision::Minutes => time.with_second(0).unwrap(),
            TimePrecision::Hours => time.with_minute(0).unwrap().with_second(0).unwrap(),
            TimePrecision::Days => time
                .with_hour(0)
                .unwrap()
                .with_minute(0)
                .unwrap()
                .with_second(0)
                .unwrap(),
            TimePrecision::Custom(secs) => {
                let total_secs = time.timestamp();
                let rounded = (total_secs / secs as i64) * secs as i64;
                Utc.timestamp(rounded, 0)
            }
        }
    }
}

impl Timeline for BasicTimeline {
    fn calculate_ticks(&self, range: Range<DateTime<Utc>>) -> Vec<Tick> {
        let mut ticks = Vec::new();
        let start = self.apply_precision(range.start);
        let end = self.apply_precision(range.end);
        let duration = end - start;

        match self.scale {
            TimeScale::Days => {
                // 天级刻度
                let days = duration.num_days();
                for day in 0..=days {
                    let date = start + Duration::days(day);
                    if !self.is_workday(date.date_naive()) {
                        continue;
                    }

                    let position = day as f32 * self.pixels_per_day;
                    ticks.push(Tick {
                        position,
                        label: date.format("%m-%d").to_string(),
                        level: TickLevel::Major,
                    });
                }
            }
            TimeScale::Weeks => {
                // 周级刻度
                let weeks = duration.num_weeks();
                for week in 0..=weeks {
                    let date = start + Duration::weeks(week);
                    let position = week as f32 * self.pixels_per_day * 7.0;
                    ticks.push(Tick {
                        position,
                        label: date.format("%Y-W%W").to_string(),
                        level: TickLevel::Major,
                    });
                }
            }
            _ => unimplemented!(),
        }

        ticks
    }

    fn time_to_position(&self, time: DateTime<Utc>) -> f32 {
        let processed_time = self.apply_precision(time);
        let duration = processed_time
            - self.apply_precision(processed_time.date().and_hms_opt(0, 0, 0).unwrap());

        match self.scale {
            TimeScale::Days => duration.num_seconds() as f32 / 86400.0 * self.pixels_per_day,
            TimeScale::Weeks => {
                let days = duration.num_days();
                days as f32 * self.pixels_per_day
            }
            _ => unimplemented!(),
        }
    }

    fn time_scale(&self) -> TimeScale {
        self.scale
    }
}
