//! Editor with your game connected to it as a plugin.
use fyrox::event_loop::EventLoop;
use fyroxed_base::{Editor, StartupData};
use battered_battery_bundles::GameConstructor;
use battered_battery_bundles::player_controller::MoveDirection;

fn main() {
    let event_loop = EventLoop::new();
    let mut editor = Editor::new(
        &event_loop,
        Some(StartupData {
            working_directory: Default::default(),
            scene: "data/scene.rgs".into(),
        }),
    );

    let editors = &editor.inspector.property_editors;
    editors.register_inheritable_enum::<MoveDirection, _>();

    editor.add_game_plugin(GameConstructor);
    editor.run(event_loop)
}
