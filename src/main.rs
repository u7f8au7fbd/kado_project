use bevy::{prelude::*, window::*};
use bevy_screen_diagnostics::*;
mod barrage;
mod bg;
mod debug;
mod kado;
mod player;
mod provatheus;
mod stage;
mod text;

fn main() {
    App::new()
        .add_plugins(
            DefaultPlugins
                .set(WindowPlugin {
                    primary_window: Some(Window {
                        title: "Kado Project".into(),                                  //タイトル
                        resolution: (1280.0, 960.0).into(), //ウィンドウサイズ
                        position: WindowPosition::Centered(MonitorSelection::Primary), //ウィンドウの生成座標を中心に設定
                        present_mode: PresentMode::Fifo,                               //Vsync有効
                        resizable: false, //サイズ変更不可
                        enabled_buttons: bevy::window::EnabledButtons {
                            minimize: false, //最小化無効
                            maximize: false, //最大化無効
                            close: true,     //閉じる有効
                        },
                        visible: false, //非表示
                        ..default()
                    }),
                    ..default()
                })
                .set(ImagePlugin::default_nearest()),
        )
        .insert_resource(ClearColor(Color::rgb_u8(0, 0, 0))) //デフォルトの背景色を設定
        .add_systems(Update, (provatheus::enable_visible, provatheus::gizmos_xyz)) //Provatheus用の開発用ライブラリ
        .add_plugins(ScreenDiagnosticsPlugin::default())
        .add_plugins(ScreenFrameDiagnosticsPlugin)
        .add_systems(Startup, set_camera)
        //以上が固定用
        .add_systems(Startup, player::spawn)
        .add_systems(Update, player::wasd)
        .add_systems(Startup, stage::spawn)
        .add_systems(Update, stage::change)
        .add_systems(Startup, kado::spawn)
        .add_systems(Update, kado::animation)
        .add_systems(Startup, text::spawn)
        .add_systems(Update, text::text_system)
        .insert_resource(GameState {
            battle: true,
            fase: 0,
            text_num: 1,
        })
        .run();
}

#[derive(Resource)]
pub struct GameState {
    battle: bool,
    fase: usize,
    text_num: usize,
}

#[derive(Component)]
struct MainCamera;

fn set_camera(mut commands: Commands) {
    commands.spawn((MainCamera, Camera2dBundle { ..default() }));
}
