
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
use crate::player_controller::MoveDirection::{Left, Right};
use crate::reverse_direction::ReverseDirection;
use crate::terrain_effect::TerrainEffect;

#[derive(Visit, Reflect, Default, Debug, Clone)]
pub struct PlayerController {
    accel_force: f32,
    max_speed: f32,
    jump_force: f32,
    direction: MoveDirection,

    collider: Handle<Node>,
}

#[derive(Debug, Visit, Reflect, Clone, AsRefStr, EnumString, EnumVariantNames)]
pub enum MoveDirection {
    Left,
    Right
}

impl Default for MoveDirection {
    fn default() -> Self {
        Right
    }
}

impl_component_provider!(PlayerController);

impl TypeUuidProvider for PlayerController {
    fn type_uuid() -> Uuid {
        uuid!("acfa0dd9-8a87-4663-bc1f-e0dd8370a09b")
    }
}

pub struct ContactFlags {
    ground_contact: bool,
    terrain_effects: (f32, f32),
    reverse_direction: bool
}

impl Default for ContactFlags {
    fn default() -> Self {
        Self {
            ground_contact: false,
            terrain_effects: (0.0f32, 0.0f32),
            reverse_direction: false
        }
    }
}

impl PlayerController {

    pub fn process_collisions(&self, graph: &Graph) -> ContactFlags {
        let mut flags = ContactFlags::default();
        if let Some(collider) = graph
            .try_get(self.collider)
            .and_then(|n| n.cast::<Collider>())
        {
            for contact in collider.contacts(&graph.physics) {
                let opposing_handle = if contact.collider1.eq(&self.collider) { contact.collider2 } else { contact.collider1 };
                if let Some(opposing_collider) = graph.try_get_of_type::<Collider>(opposing_handle) {
                    if opposing_collider.has_script::<ReverseDirection>() {
                        flags.reverse_direction = true;
                    }
                }
                for manifold in contact.manifolds.iter() {
                    if manifold.local_n1.y.abs() > 0.7 || manifold.local_n2.y.abs() > 0.7 {
                        flags.ground_contact = true;
                    }
                }
            }

            for contact in collider.intersects(&graph.physics) {
                if contact.has_any_active_contact {
                    let opposing_handle = if contact.collider1.eq(&self.collider) { contact.collider2 } else { contact.collider1 };
                    if let Some(opposing_collider) = graph.try_get_of_type::<Collider>(opposing_handle) {
                        if opposing_collider.has_script::<TerrainEffect>() {
                            if let Some(terrain_effect) = opposing_collider.try_get_script::<TerrainEffect>() {
                                flags.terrain_effects.0 += terrain_effect.accel_modifier;
                                flags.terrain_effects.1 += terrain_effect.max_speed_mod;
                            }
                        }
                    }
                }
            }
        }

        flags
    }

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

    pub fn has_terrain_effect(&self, graph: &Graph) -> (f32, f32) {
        let mut res = (0.0f32, 0.0f32);
        if let Some(collider) = graph
            .try_get_of_type::<Collider>(self.collider)
        {
            for contact in collider.intersects(&graph.physics) {
                if contact.has_any_active_contact {
                    let opposing_handle = if contact.collider1.eq(&self.collider) { contact.collider2 } else { contact.collider1 };
                    if let Some(opposing_collider) = graph.try_get_of_type::<Collider>(opposing_handle) {
                        if opposing_collider.has_script::<TerrainEffect>() {
                            if let Some(terrain_effect) = opposing_collider.try_get_script::<TerrainEffect>() {
                                res.0 += terrain_effect.accel_modifier;
                                res.1 += terrain_effect.max_speed_mod;
                            }
                        }
                    }
                }
            }
        }
        res
    }
}

impl ScriptTrait for PlayerController {
    fn on_init(&mut self, _context: &mut ScriptContext) {}

    fn on_start(&mut self, _context: &mut ScriptContext) {}

    fn on_deinit(&mut self, _context: &mut ScriptDeinitContext) {}

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
        let flags = self.process_collisions(&context.scene.graph);
        if flags.reverse_direction {
            self.direction = match self.direction {
                Left => Right,
                Right => Left
            };
        }
        if let Some(rigid_body) = context.scene.graph[context.handle].cast_mut::<RigidBody>() {
            let vel = rigid_body.lin_vel();
            Log::info(format!("{}", vel.x));
            if vel.x.abs() == self.max_speed + flags.terrain_effects.1 {
                return;
            }
            if vel.x.abs() > self.max_speed + flags.terrain_effects.1 {
                rigid_body.set_lin_vel(Vector3::new(match self.direction {
                    MoveDirection::Left => self.max_speed + flags.terrain_effects.1,
                    MoveDirection::Right => -self.max_speed - flags.terrain_effects.1
                }, vel.y, vel.z));
                return;
            }
            rigid_body.apply_force(Vector3::new(match self.direction {
                MoveDirection::Left => self.accel_force + flags.terrain_effects.0,
                MoveDirection::Right => -self.accel_force - flags.terrain_effects.0,
            }, 0.0, 0.0));
        }
    }

    fn id(&self) -> Uuid {
        Self::type_uuid()
    }
}
