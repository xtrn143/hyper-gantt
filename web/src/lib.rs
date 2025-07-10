use bevy::prelude::*;
use hyper_gantt_core::{ecs::components::timeline::TimelineMarker, test_utils::ExtremeTestData};

#[wasm_bindgen::prelude::wasm_bindgen(start)]
pub fn run() {
    App::new()
        .add_plugins(DefaultPlugins)
        .insert_resource(ExtremeTestData::new())
        .add_systems(Startup, setup)
        .run();
}

fn setup(mut commands: Commands, test_data: Res<ExtremeTestData>) {
    // 设置相机
    commands.spawn(Camera2dBundle::default());

    // 显示测试数据信息
    commands.spawn(TextBundle::from_sections([
        TextSection::new(
            format!("共加载{}个测试任务\n", test_data.tasks.len()),
            TextStyle {
                font_size: 24.0,
                color: Color::WHITE,
                ..default()
            },
        ),
        TextSection::new(
            format!("包含{}个节假日", test_data.holidays.len()),
            TextStyle {
                font_size: 24.0,
                color: Color::YELLOW,
                ..default()
            },
        ),
    ]));

    // 添加时间轴标记
    commands.spawn((
        NodeBundle {
            style: Style {
                width: Val::Percent(100.0),
                height: Val::Px(50.0),
                ..default()
            },
            background_color: Color::DARK_GRAY.into(),
            ..default()
        },
        TimelineMarker {
            pixels_per_day: 10.0,
            current_scale: 1.0,
            target_scale: None,
        },
    ));
}
