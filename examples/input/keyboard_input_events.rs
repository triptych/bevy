use bevy::{input::keyboard::KeyboardInput, prelude::*};

fn main() {
    App::build()
        .add_default_plugins()
        .init_resource::<State>()
        .add_system(print_keyboard_event_system.system())
        .run();
}

#[derive(Default)]
struct State {
    event_reader: EventReader<KeyboardInput>,
}

/// This system prints out all mouse events as they come in
fn print_keyboard_event_system(
    mut state: ResMut<State>,
    keyboard_input_events: Res<Events<KeyboardInput>>,
) {
    for event in state.event_reader.iter(&keyboard_input_events) {
        println!("{:?}", event);
    }
}
