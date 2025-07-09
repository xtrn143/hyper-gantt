# Hyper Gantt 设计文档

## 1. 项目目标
- 基于WebAssembly和Bevy ECS的高性能甘特图组件
- 完全可定制化设计
- 简洁易用的API接口

## 2. 技术架构
### 2.1 核心框架
- **Bevy ECS**: 实体组件系统架构
- **WASM**: wasm-bindgen + web-sys
- **渲染方案**: 双模式支持
  - Bevy UI (基础版)
  - WebGL (高性能版)

### 2.2 模块设计
```rust
// ECS核心组件
#[derive(Component)]
pub struct GanttConfig {
    pub time_scale: TimeScale,
    pub show_critical_path: bool,
}

#[derive(Component)] 
pub struct TaskView {
    pub color: Color,
    pub height: f32,
}

## 3. 交互系统设计
### 3.1 输入处理
- **事件分发系统**: 基于Bevy的Event系统
- **输入类型支持**:
  - 鼠标交互 (点击/拖拽/悬停)
  - 键盘快捷键 (WASD导航, +/-缩放)
  - 触摸手势 (捏合缩放/滑动)

### 3.2 操作历史
- **Command模式实现**:
```rust
trait Command {
    fn execute(&mut self);
    fn undo(&mut self);
}
```
- **操作堆栈**: 双向链表存储历史记录
- **撤销/重做**: Ctrl+Z/Ctrl+Y快捷键绑定

### 3.3 上下文菜单
- **动态生成**: 基于当前选中元素类型
- **层级结构**: 支持嵌套子菜单
- **国际化**: 多语言文本支持

## 4. 样式系统设计
### 4.1 主题配置
```rust
struct Theme {
    palette: ColorPalette,
    typography: TypographySettings,
    spacing: SpacingRules,
    animations: AnimationPresets,
}
```

### 4.2 条件样式
- **规则引擎**:
```rust
enum StyleCondition {
    TaskStatus(TaskStatus),
    TimeRange(TimeRange),
    Custom(Box<dyn Fn(&Task) -> bool>)
}
```

### 4.3 图标系统
- **多来源支持**:
  - 内置SVG图标库
  - 外部URL加载
  - 字体图标集成

## 5. 性能优化
### 5.1 渲染策略
- **虚拟滚动**: 仅渲染可视区域任务
- **增量更新**: 脏矩形检测
- **WASM内存**: 预分配对象池

### 5.2 数据管理
- **分页加载**: 动态任务数据获取
- **变更检测**: 细粒度状态监听
- **缓存策略**: 常用数据内存缓存

## 6. API设计
### 6.1 核心接口
```rust
pub trait GanttCore {
    fn add_task(&mut self, task: Task) -> TaskId;
    fn render(&self) -> Result<Canvas, GanttError>;
}
```

### 6.2 构建器模式
```rust
let chart = GanttBuilder::new()
    .with_time_range(start, end)
    .with_theme(Theme::dark())
    .build();
```

### 6.3 错误处理
- **错误分类**:
  - 配置错误
  - 渲染错误  
  - 交互错误
- **错误恢复**: 自动回滚机制

## 7. 详细组件设计
### 7.1 时间轴组件
```rust
struct Timeline {
    visible_range: Range<DateTime>,
    scale: TimeScale, // 天/周/月/季度
    work_days: BitSet, // 工作日历
    holidays: HashSet<Date>,
    // 刻度计算算法
    fn calculate_ticks(&self) -> Vec<(DateTime, TickLevel)> {
        // 根据scale自动计算主/次刻度位置
    }
}
```

### 7.2 任务依赖关系
```rust
struct DependencyRenderer {
    line_style: PolylineStyle,
    arrow_style: ArrowStyle,
    // 贝塞尔曲线路径计算
    fn calculate_path(&self, from: Rect, to: Rect) -> Path {
        // 自动避开障碍物的智能连线算法
    }
}
```

### 7.3 关键路径计算
```rust
fn find_critical_path(tasks: &[Task]) -> Vec<TaskId> {
    // 基于拓扑排序和动态规划
    // 计算最早/最晚开始时间
    // 识别零浮动时间任务
}

## 8. 测试方案设计
### 8.1 单元测试架构
```rust
#[cfg(test)]
mod tests {
    use super::*;
    use wasm_bindgen_test::*;

    #[wasm_bindgen_test]
    fn test_timeline_calculation() {
        let timeline = Timeline::new(...);
        let ticks = timeline.calculate_ticks();
        assert_eq!(ticks.len(), 10);
    }
}
```

### 8.2 浏览器测试
- **测试框架**: wasm-pack + Headless Chrome
- **覆盖范围**:
  - 主流浏览器(Chrome/Firefox/Safari/Edge)
  - 移动端触控测试
  - 高DPI屏幕适配测试

### 8.3 性能测试
```rust
#[bench]
fn bench_critical_path(b: &mut Bencher) {
    let tasks = generate_test_tasks(10_000);
    b.iter(|| find_critical_path(&tasks));
}
```

### 8.4 测试数据生成
- **随机数据**: proptest策略生成
- **边界条件**:
  - 空任务集
  - 单任务场景
  - 最大任务数(10万+)
  - 极端时间范围(跨多年)
```

## 9. 项目结构与ECS组织
### 9.1 项目目录结构
```
hyper-gantt/
├── core/              # 核心逻辑
│   ├── model/        # 数据模型
│   ├── algorithm/    # 核心算法
│   └── render/       # 渲染引擎
├── web/              # WASM相关
│   ├── bindings/     # wasm-bindgen绑定
│   └── adapter/      # 浏览器适配层
├── native/           # 原生平台支持
│   ├── bevy_ui/      # Bevy UI实现
│   └── webgl/        # WebGL实现
└── api/              # 公共API
    ├── builder/      # 构建器模式
    └── error/       # 错误处理
```

### 9.2 ECS框架组织
#### 组件定义示例：
```rust
// core/ecs/components/task.rs
#[derive(Component)]
pub struct Task {
    pub id: TaskId,
    pub start: DateTime,
    pub end: DateTime,
    pub progress: f32,
}
```

#### 系统实现示例：
```rust
// core/ecs/systems/layout/timeline.rs
pub fn update_timeline(
    mut query: Query<(&mut Transform, &TimelineMarker)>,
    time: Res<Time>,
    config: Res<GanttConfig>
) {
    // 时间轴布局逻辑
}
```

#### 跨平台集成：
```rust
// web/adapter/ecs_bridge.rs
pub fn setup_wasm_ecs(app: &mut App) {
    app.add_system(wasm_input_system)
       .add_plugin(WebRendererPlugin);
}

## 10. 补充设计内容
### 10.1 时间轴锁定功能
```rust
#[derive(Component)]
pub struct TimelineLock {
    pub locked_ranges: Vec<Range<DateTime>>,
    pub lock_visual: LockVisualStyle, // 条纹/半透明/纯色
}

// 系统实现
fn enforce_time_locks(
    mut query: Query<&mut Transform, With<TaskBar>>,
    locks: Query<&TimelineLock>
) {
    // 禁止在锁定区域创建/移动任务
}
```

### 10.2 操作历史可视化
```rust
#[derive(Component)]
pub struct HistoryVisualizer {
    pub max_visible: usize, // 最大显示历史条目
    pub style: HistoryStyle
}

// 系统实现
fn update_history_visualization(
    history: Res<OperationHistory>,
    mut visuals: Query<&mut HistoryVisualizer>
) {
    // 更新历史可视化状态
}
```

### 10.3 键盘焦点管理
```rust
#[derive(Component)]
pub struct KeyboardFocus {
    pub style: FocusStyle, // 发光/边框/背景色
    pub z_index: i32,
    pub is_active: bool
}
```

### 10.4 故障排查指南
```
docs/troubleshooting/
├── rendering.md      # 渲染问题
├── performance.md    # 性能问题
└── wasm.md          # WASM相关问题
```
```
