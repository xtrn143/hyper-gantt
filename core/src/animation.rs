//! 动画系统实现
//!
//! 提供平滑过渡动画效果

use super::timeline::TimeScale;
use chrono::Duration;

/// 缩放动画状态
pub struct ZoomAnimation {
    duration: f32, // 总时长(秒)
    elapsed: f32,  // 已进行时间
    start_scale: TimeScale,
    target_scale: TimeScale,
}

impl ZoomAnimation {
    /// 创建新的缩放动画
    pub fn new(target: TimeScale, current: TimeScale) -> Self {
        Self {
            duration: 0.5, // 默认500ms平滑过渡
            elapsed: 0.0,
            start_scale: current,
            target_scale: target,
        }
    }

    /// 更新动画状态
    pub fn update(&mut self, delta: Duration) -> bool {
        self.elapsed += delta.num_seconds() as f32;
        self.elapsed >= self.duration
    }

    /// 获取当前缩放值(0.0-1.0)
    pub fn progress(&self) -> f32 {
        (self.elapsed / self.duration).min(1.0)
    }

    /// 获取当前时间单位
    pub fn current_scale(&self) -> TimeScale {
        // 线性插值
        let t = ease_in_out_quad(self.progress());
        interpolate_time_scale(self.start_scale, self.target_scale, t)
    }
}

/// 二次缓动函数
fn ease_in_out_quad(t: f32) -> f32 {
    if t < 0.5 {
        2.0 * t * t
    } else {
        -1.0 + (4.0 - 2.0 * t) * t
    }
}

/// 时间单位插值
fn interpolate_time_scale(start: TimeScale, end: TimeScale, t: f32) -> TimeScale {
    match (start, end) {
        (TimeScale::Days, TimeScale::Weeks) if t < 0.5 => TimeScale::Days,
        (TimeScale::Days, TimeScale::Weeks) => TimeScale::Weeks,
        // 其他单位转换逻辑
        _ => {
            if t < 0.5 {
                start
            } else {
                end
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_animation_progress() {
        use chrono::Duration;
        let mut anim = ZoomAnimation::new(TimeScale::Weeks, TimeScale::Days);
        anim.update(Duration::milliseconds(250));
        assert!(!anim.update(Duration::milliseconds(100)));
        assert!(anim.update(Duration::milliseconds(200)));
    }
}
