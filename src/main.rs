use bevy::{
    prelude::*,
};
use tts::{Backends, TTS};
use bevy::input::keyboard::KeyboardInput;

struct Speaker {
    pub tts: TTS,
}

fn main() {
    let mut tts = TTS::new(Backends::AppKit).unwrap();

    App::build()
        .add_startup_system(setup.system())
        .add_resource(MyInputState::default())
        .add_resource(Speaker { tts })
        .add_system(update_letters.system())
        .add_plugins(DefaultPlugins)
        .run();
}

struct Letter;

#[derive(Default)]
pub struct MyInputState {
    keys: EventReader<KeyboardInput>,
}

fn setup(
    commands: &mut Commands, asset_server: Res<AssetServer>
) {
    commands
        // 2d camera
        .spawn(CameraUiBundle::default())
        // texture
        .spawn(TextBundle {
            style: Style {
                align_self: AlignSelf::FlexEnd,
                ..Default::default()
            },
            text: Text {
                value: "---".to_string(),
                font: asset_server.load("fonts/LondrinaSolid-Black.ttf"),
                style: TextStyle {
                    font_size: 240.0,
                    color: Color::WHITE,
                    alignment: TextAlignment {
                        horizontal: HorizontalAlign::Center,
                        vertical: VerticalAlign::Center,
                    }
                },
            },
            ..Default::default()
        })
        .with(Letter);
}

fn update_letters(
    mut state: ResMut<MyInputState>,
    ev_keys: Res<Events<KeyboardInput>>,
    mut speaker: ResMut<Speaker>,
    mut query: Query<&mut Text, With<Letter>>
) {
    for ev in state.keys.iter(&ev_keys) {
        if ev.state.is_pressed() {
            for mut text in query.iter_mut() {
                text.value = format!("{:?}", ev.key_code.unwrap());

                speaker.tts.speak(text.value.clone(), true).unwrap();
            }
        }
    }
}