
use fyrox::{
    core::{uuid::{Uuid, uuid}, visitor::prelude::*, reflect::prelude::*, TypeUuidProvider},
    event::Event, impl_component_provider,
    script::{ScriptContext, ScriptDeinitContext, ScriptTrait},
};
use fyrox::core::algebra::Vector3;
use fyrox::core::pool::Handle;
use fyrox::scene::camera::Camera;

use fyrox::scene::node::Node;

#[derive(Visit, Reflect, Default, Debug, Clone)]
pub struct CameraController {
    follow_ref: Handle<Node>
}

impl_component_provider!(CameraController);

impl TypeUuidProvider for CameraController {
    fn type_uuid() -> Uuid {
        uuid!("2c2dcccf-d771-477e-a785-3b110fa349de")
    }
}

impl ScriptTrait for CameraController {
    fn on_init(&mut self, context: &mut ScriptContext) {
        let graph = &mut context.scene.graph;
        let target_x = graph[self.follow_ref].local_transform().position().x;
        let camera = graph[context.handle].cast_mut::<Camera>().expect("Camera expected but got other");
        let camera_transform = camera.local_transform_mut();
        let pos = camera_transform.position();
        camera_transform.set_position(Vector3::new(target_x, pos.y, pos.z));
    }

    fn on_start(&mut self, _context: &mut ScriptContext) {}

    fn on_deinit(&mut self, _context: &mut ScriptDeinitContext) {}

    fn on_os_event(&mut self, _event: &Event<()>, _context: &mut ScriptContext) {}

    fn on_update(&mut self, context: &mut ScriptContext) {
        let graph = &mut context.scene.graph;
        let target_x = graph[self.follow_ref].local_transform().position().x;
        let camera = graph[context.handle].cast_mut::<Camera>().expect("Camera expected but got other");
        let camera_transform = camera.local_transform_mut();
        let pos = camera_transform.position();
        camera_transform.set_position(Vector3::new(target_x, pos.y, pos.z));
    }

    fn id(&self) -> Uuid {
        Self::type_uuid()
    }
}
    