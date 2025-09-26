use bevy::{
    prelude::*
};

pub struct HelloPlugin;

impl Plugin for HelloPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, start);
    }
}

fn start() {
    println!("started")
}
        
fn main() {
    App::new()
    .add_plugins(DefaultPlugins.set(WindowPlugin {
        primary_window: Some(Window {
            title: String::from("Graphing?"),
            decorations: false,
            ..default()
        }),
        ..default()
    }))
    .add_plugins(HelloPlugin)
    .add_systems(Startup, setup)
    .add_systems(Update, text_color_system)
    .run();
}

#[derive(Component)]
struct AnimatedText;

fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    commands.spawn(Camera2d);

    commands.spawn((
        Text::new("hello world!"),
        TextFont {
            font_size: 50.0,
            ..default()
        },
        TextShadow::default(),
        TextLayout::new_with_justify(JustifyText::Center),
        BorderRadius::all(Val::Px(5.0)),
        BorderColor(Color::linear_rgb(1.0, 0.0, 0.0)),
        Node {
            position_type: PositionType::Absolute,
            align_self: AlignSelf::Center,
            justify_self: JustifySelf::Center,
            ..default()
        },
        AnimatedText,
    ));
    
    commands.spawn((
        Mesh2d(meshes.add(Circle::new(50.0))),
        MeshMaterial2d(materials.add(Color::hsl(0.0, 1.0, 0.5))),
        Transform::from_xyz(200.0, 0.0, 0.0),
    ));
}

fn text_color_system(time: Res<Time>, mut query: Query<&mut TextColor, With<AnimatedText>>) {
    for mut text_color in &mut query {
        let seconds = time.elapsed_secs();

        text_color.0 = Color::hsl(
            ops::sin(seconds)*180.0 + 180.0, 
            1.0,
            0.5,
        );
    }
}