
use bevy::prelude::*;

struct  Button {
    normal: Handle<ColorMaterial>,
    hovered: Handle<ColorMaterial>,
    pressed: Handle<ColorMaterial>,
}

impl FromWorld for Button {
    fn from_world(world: &mut World) -> Self {
        let mut  materials = world.get_resource_mut::<Assets<ColorMaterial>>().unwrap();

        Button {
            normal: materials.add(Color::rgb(0.15, 0.15, 0.15).into()),
            hovered: materials.add(Color::rgb(0.25, 0.25, 0.25).into()),
            pressed: materials.add(Color::rgb(0.35, 0.75, 0.35).into()),
        }
    }
}

fn setup(
    mut commmands: Commands,
    asset_server: Res<AssetServer>,
    button: Res<Button>
) {
    // camera setup
    commmands.spawn_bundle(UiCameraBundle::default());
    commmands
        .spawn_bundle(ButtonBundle {
        style: Style {
            // size of button
            size: Size::new(Val::Px(150.0), Val::Px(65.0)),
            // center button
            margin: Rect::all(Val::Auto),
            //   horozontal central align hild text
            justify_content: JustifyContent::Center,
            //     vertical centre child text
            align_items: AlignItems::Center,
            ..Default::default()
        },
        material: button.normal.clone(),
        ..Default::default()
    })
        .with_children(|parent| {
            parent.spawn_bundle(TextBundle {
                text: Text::with_section(
                    "Test buttong",
                    TextStyle {
                        font: asset_server.load("fonts/FiraSans-Bold.ttf"),
                        font_size: 40.0,
                        color: Color::rgb(0.9, 0.9,0.9),
                    },
                    Default::default(),
                ),
                ..Default::default()
            });
        });

}

fn main() {
    App::build()
        .add_plugins(DefaultPlugins)
        .init_resource::<Button>()
        .run();
}