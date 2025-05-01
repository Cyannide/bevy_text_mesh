use bevy::prelude::*;
use bevy_text_mesh::TextMeshPlugin;

fn main() {
    App::new()
        .add_plugins((DefaultPlugins, TextMeshPlugin)) // TextMeshPlugin for interop check
        .add_systems(Startup, setup)
        .add_systems(Update, animate_rotation)
        .run();
}

#[derive(Component)]
struct AnimateRotation;

fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    let font = asset_server.load("fonts/FiraMono-Medium.ttf");
    let text_font = TextFont {
        font: font.clone(),
        font_size: 60.0,
        ..default()
    };
    let text_alignment = JustifyText::Center;

    commands.spawn(Camera2d);
    commands.spawn((
        Text2d::new("standard 2d text works too"),
        text_font.clone(),
		TextColor::WHITE,
        TextLayout::new_with_justify(text_alignment),
        AnimateRotation,
    ));

}

fn animate_rotation(
    time: Res<Time>,
    mut query: Query<&mut Transform, (With<Text2d>, With<AnimateRotation>)>,
) {
    for mut transform in &mut query {
        transform.rotation = Quat::from_rotation_z(time.elapsed_secs_f64().cos() as f32);
    }
}
