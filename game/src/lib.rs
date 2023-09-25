//! Game project.
use fyrox::{
    core::pool::Handle,
    event::Event,
    event_loop::ControlFlow,
    gui::message::UiMessage,
    plugin::{Plugin, PluginConstructor, PluginContext, PluginRegistrationContext},
    scene::{Scene, loader::AsyncSceneLoader},
    core::log::Log,
};
use fyrox::core::algebra::Vector2;
use fyrox::engine::GraphicsContext;
use fyrox::gui::message::MessageDirection;
use fyrox::gui::text::{TextBuilder, TextMessage};
use fyrox::gui::UiNode;
use fyrox::gui::widget::WidgetBuilder;
use crate::camera_controller::CameraController;
use crate::package_pickup_point::PackagePickupPoint;
use crate::player_controller::PlayerController;
use crate::reverse_direction::ReverseDirection;
use crate::terrain_effect::TerrainEffect;

pub mod camera_controller;
pub mod player_controller;
pub mod terrain_effect;
pub mod reverse_direction;
pub mod package_pickup_point;

pub struct GameConstructor;

impl PluginConstructor for GameConstructor {
    fn register(&self, context: PluginRegistrationContext) {
        context.serialization_context.script_constructors.add::<CameraController>("Camera Controller");
        context.serialization_context.script_constructors.add::<PlayerController>("Player Controller");
        context.serialization_context.script_constructors.add::<TerrainEffect>("Terrain Effects");
        context.serialization_context.script_constructors.add::<ReverseDirection>("Reverse Direction");
        context.serialization_context.script_constructors.add::<PackagePickupPoint>("Package Pickup Point");
    }

    fn create_instance(
        &self,
        override_scene: Handle<Scene>,
        context: PluginContext,
    ) -> Box<dyn Plugin> {
        Box::new(Game::new(override_scene, context))
    }
}

pub struct Game {
    scene: Handle<Scene>,
    loader: Option<AsyncSceneLoader>,
    health_ui: Handle<UiNode>,
    package_ui: Handle<UiNode>,
}

impl Game {
    pub fn new(override_scene: Handle<Scene>, context: PluginContext) -> Self {
        let mut loader = None;
        let scene = if override_scene.is_some() {
            override_scene
        } else {
            loader = Some(AsyncSceneLoader::begin_loading(
                "data/scene.rgs".into(),
                context.serialization_context.clone(),
                context.resource_manager.clone(),
            ));
            Default::default()
        };

        let health_text = TextBuilder::new(WidgetBuilder::new()
            .with_desired_position(Vector2::new(10.0, 10.0)))
            .build(&mut context.user_interface.build_ctx());
        let package_text = TextBuilder::new(WidgetBuilder::new()
            .with_desired_position(Vector2::new(10.0, 25.0)))
            .build(&mut context.user_interface.build_ctx());

        Self { scene, loader, health_ui: health_text, package_ui: package_text }
    }
}

impl Plugin for Game {
    fn on_deinit(&mut self, _context: PluginContext) {
        // Do a cleanup here.
    }

    fn update(&mut self, context: &mut PluginContext, _control_flow: &mut ControlFlow) {
        if let Some(loader) = self.loader.as_ref() {
            if let Some(result) = loader.fetch_result() {
                match result {
                    Ok(scene) => {
                        self.scene = context.scenes.add(scene);
                    }
                    Err(err) => Log::err(err),
                }
            }
        }

        // Add your global update code here.

        if let GraphicsContext::Initialized(ref graphics_context) = context.graphics_context {
            let player = context.scenes[self.scene].graph.find_from_root(&mut |n| n.has_script::<PlayerController>()).unwrap().1.script().unwrap().cast::<PlayerController>().unwrap();
            context.user_interface.send_message(TextMessage::text(
                self.health_ui,
                MessageDirection::ToWidget,
                format!("Player health {}", player.actual_player_health)
            ));
            context.user_interface.send_message(TextMessage::text(
                self.package_ui,
                MessageDirection::ToWidget,
                format!("Package health: {}", player.actual_package_health)
            ));
        }
    }

    fn on_os_event(
        &mut self,
        _event: &Event<()>,
        _context: PluginContext,
        _control_flow: &mut ControlFlow,
    ) {
        // Do something on OS event here.
    }

    fn on_ui_message(
        &mut self,
        _context: &mut PluginContext,
        _message: &UiMessage,
        _control_flow: &mut ControlFlow,
    ) {
        // Handle UI events here.
    }
}
