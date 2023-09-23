
use fyrox::{
    core::{uuid::{Uuid, uuid}, visitor::prelude::*, reflect::prelude::*, TypeUuidProvider},
    event::Event, impl_component_provider,
    script::{ScriptContext, ScriptDeinitContext, ScriptTrait},
};
use fyrox::core::algebra::Vector3;
use fyrox::core::log::Log;
use fyrox::core::pool::Handle;
use fyrox::event::{ElementState, VirtualKeyCode, WindowEvent};
use fyrox::scene::collider::Collider;
use fyrox::scene::graph::Graph;
use fyrox::scene::node::Node;
use fyrox::scene::rigidbody::RigidBody;

use strum_macros::{AsRefStr, EnumString, EnumVariantNames};

#[derive(Visit, Reflect, Default, Debug, Clone)]
pub struct PlayerController {
    accel_force: f32,
    max_speed: f32,
    jump_force: f32,
    direction: MoveDirection,

    collider: Handle<Node>,

    // #[visit(skip)]
    // #[reflect(hidden)]
    // is_jumping: bool
}

#[derive(Debug, Visit, Reflect, Clone, AsRefStr, EnumString, EnumVariantNames)]
pub enum MoveDirection {
    Left,
    Right
}

impl Default for MoveDirection {
    fn default() -> Self {
        MoveDirection::Right
    }
}

impl_component_provider!(PlayerController);

impl TypeUuidProvider for PlayerController {
    fn type_uuid() -> Uuid {
        uuid!("acfa0dd9-8a87-4663-bc1f-e0dd8370a09b")
    }
}

impl PlayerController {
    pub fn has_ground_contact(&self, graph: &Graph) -> bool {
        if let Some(collider) = graph
            .try_get(self.collider)
            .and_then(|n| n.cast::<Collider>())
        {
            for contact in collider.contacts(&graph.physics) {
                for manifold in contact.manifolds.iter() {
                    if manifold.local_n1.y.abs() > 0.7 || manifold.local_n2.y.abs() > 0.7 {
                        return true;
                    }
                }
            }
        }
        false
    }
}

impl ScriptTrait for PlayerController {
    fn on_init(&mut self, _context: &mut ScriptContext) {
        // Put initialization logic here.
    }

    fn on_start(&mut self, _context: &mut ScriptContext) {
        // There should be a logic that depends on other scripts in scene.
        // It is called right after **all** scripts were initialized.
    }

    fn on_deinit(&mut self, _context: &mut ScriptDeinitContext) {
        // Put de-initialization logic here.
    }

    fn on_os_event(&mut self, event: &Event<()>, context: &mut ScriptContext) {
        if let Event::WindowEvent { event, .. } = event {
            if let WindowEvent::KeyboardInput { input, .. } = event {
                if let Some(keycode) = input.virtual_keycode {
                    let is_pressed = input.state == ElementState::Pressed;

                    match keycode {
                        VirtualKeyCode::Space => {
                            if is_pressed && self.has_ground_contact(&context.scene.graph) {
                                if let Some(rigid_body) = context.scene.graph[context.handle].cast_mut::<RigidBody>() {
                                    Log::info("Applying jump force");
                                    //rigid_body.apply_force(Vector3::new(0.0, self.jump_force, 0.0));
                                    let vel = rigid_body.lin_vel();
                                    rigid_body.set_lin_vel(Vector3::new(vel.x, self.jump_force, 0.0));
                                }
                            }
                        }
                        _ => (),
                    }
                }
            }
        }
    }

    fn on_update(&mut self, context: &mut ScriptContext) {
        if let Some(rigid_body) = context.scene.graph[context.handle].cast_mut::<RigidBody>() {
            let vel = rigid_body.lin_vel();
            if vel.x.abs() == self.max_speed {
                return;
            }
            if vel.x.abs() > self.max_speed {
                rigid_body.set_lin_vel(Vector3::new(match self.direction {
                    MoveDirection::Left => self.max_speed,
                    MoveDirection::Right => -self.max_speed
                }, vel.y, vel.z));
                return;
            }
            rigid_body.apply_force(Vector3::new(match self.direction {
                MoveDirection::Left => self.accel_force,
                MoveDirection::Right => -self.accel_force,
            }, 0.0, 0.0));
        }
    }

    fn id(&self) -> Uuid {
        Self::type_uuid()
    }
}
