
use bevy::prelude::*;

struct  ButtonProps {
    normal: Handle<ColorMaterial>,
    hovered: Handle<ColorMaterial>,
    pressed: Handle<ColorMaterial>,
}

impl FromWorld for ButtonProps {
    fn from_world(world: &mut World) -> Self {
        let mut  materials = world.get_resource_mut::<Assets<ColorMaterial>>().unwrap();
        println!("======= printing from impl FromWorld =============");
        println!("materials: {:?}", materials);
        ButtonProps {
            normal: materials.add(Color::rgb(0.15, 0.15, 0.15).into()),
            hovered: materials.add(Color::rgb(0.25, 0.25, 0.25).into()),
            pressed: materials.add(Color::rgb(0.35, 0.75, 0.35).into()),
        }
    }
}

fn button_system(
    button: Res<ButtonProps>,
    mut interaction_query: Query<(&Interaction, &mut Handle<ColorMaterial>, &Children), (Changed<Interaction>, With<Button>)>,
    mut text_query: Query<&mut Text>
) {
    for (interaction, mut material, children) in interaction_query.iter_mut() {

        println!("=========== printing from button_system ===========");
        println!("interaction: {:?}", interaction);
        println!("material: {:?}", material);
        println!("children:{:?}", children);

        let mut text = text_query.get_mut(children[0]).unwrap();

        println!("text: {:?}", text);

        match *interaction {
            Interaction::Clicked => {
                text.sections[0].value = "press".to_string();
                *material = button.pressed.clone();
            }
            Interaction::Hovered => {
                text.sections[0].value = "hover".to_string();
                *material = button.hovered.clone();
            }
            Interaction::None => {
                text.sections[0].value = "button".to_string();
                *material = button.normal.clone();
            }
        }
    }
}

fn setup(
    mut commmands: Commands,
    asset_server: Res<AssetServer>,
    button: Res<ButtonProps>
) {
    println!("========= printing from setup =============");



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
                    "Test button",
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
        .init_resource::<ButtonProps>()
        .add_startup_system(setup.system())
        .add_system(button_system.system())
        .run();
}