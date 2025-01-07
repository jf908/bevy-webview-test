use std::f32::consts::TAU;

use bevy::{
    color::palettes::css::YELLOW_GREEN, prelude::*, window::PrimaryWindow, winit::WinitWindows,
};
use wry::{raw_window_handle::HasWindowHandle, WebViewBuilder};

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                title: "Bevy & Wry".to_string(),
                // https://github.com/bevyengine/bevy/pull/16545
                // clip_children: false,
                ..default()
            }),
            ..default()
        }))
        .add_systems(Startup, setup)
        .add_systems(Startup, setup_webview)
        .add_systems(Update, update)
        .run();
}

fn setup(mut commands: Commands) {
    commands.spawn(Camera2d);
}

fn update(mut gizmos: Gizmos, time: Res<Time>) {
    gizmos.ellipse_2d(
        Rot2::radians(time.elapsed_secs() % TAU),
        Vec2::new(100., 200.),
        YELLOW_GREEN,
    );
}

fn setup_webview(world: &mut World) {
    let primary_window_entity = world
        .query_filtered::<Entity, With<PrimaryWindow>>()
        .single(world);
    let primary_window = world
        .get_non_send_resource::<WinitWindows>()
        .expect("Missing window")
        .get_window(primary_window_entity)
        .expect("Missing window");

    let mut size = primary_window.inner_size();
    size.width /= 2;

    let webview = WebViewBuilder::new()
        .with_devtools(true)
        .with_transparent(true)
        .with_bounds(wry::Rect {
            size: size.into(),
            ..default()
        })
        .with_html(
            r#"<html>
            <body style="background-color: rgba(0.5,0.5,0.5,0.5); color: white;"></body>
            <script>
            window.onload = function() {
                document.body.innerText = `hello, ${navigator.userAgent}`;
            };
            </script>
            </html>"#,
        )
        .build_as_child(&primary_window.window_handle().unwrap())
        .unwrap();

    world.insert_non_send_resource(webview);
}
