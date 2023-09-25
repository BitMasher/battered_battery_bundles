
use fyrox::{
    core::{uuid::{Uuid, uuid}, visitor::prelude::*, reflect::prelude::*, TypeUuidProvider},
    event::Event, impl_component_provider,
    script::{ScriptContext, ScriptDeinitContext, ScriptTrait},
};
use fyrox::core::log::Log;
use fyrox::core::pool::Handle;
use fyrox::scene::collider::Collider;
use fyrox::scene::graph::Graph;
use fyrox::scene::node::Node;
use crate::player_controller::PlayerController;

#[derive(Debug, Clone)]
pub struct DamageMessage {
    pub player_damage: u8,
    pub package_damage: u8,
}

#[derive(Visit, Reflect, Default, Debug, Clone)]
pub struct TerrainEffect {
    pub accel_modifier: f32,
    pub max_speed_mod: f32,
    pub player_damage: u8,
    pub package_damage: u8,

    #[visit(skip)]
    #[reflect(hidden)]
    hit: Vec<Handle<Node>>,
}

impl TerrainEffect {
    pub fn has_player_contact(&self, handle: Handle<Node>, graph: &Graph) -> Option<Handle<Node>> {
        if let Some(collider) = graph.try_get_of_type::<Collider>(handle) {
            for contact in collider.intersects(&graph.physics) {
                if contact.has_any_active_contact {
                    let opposing_handle = if contact.collider1.eq(&handle) { contact.collider2 } else { contact.collider1 };
                    if let Some(parent) = graph.try_get(opposing_handle).map(|n| n.parent()) {
                        if let Some((player, _)) = graph.find_up(parent, &mut |n| {
                            return n.script().and_then(|s|s.cast::<PlayerController>()).is_some();
                        }) {
                            return Some(player);
                        }
                    }
                }
            }
        }
        None
    }
}

impl_component_provider!(TerrainEffect);

impl TypeUuidProvider for TerrainEffect {
    fn type_uuid() -> Uuid {
        uuid!("3c4f8742-38b8-4d26-a1ae-9c4a552eb5d1")
    }
}

impl ScriptTrait for TerrainEffect {
    fn on_init(&mut self, _context: &mut ScriptContext) {}

    fn on_start(&mut self, _context: &mut ScriptContext) {}

    fn on_deinit(&mut self, _context: &mut ScriptDeinitContext) {}

    fn on_os_event(&mut self, _event: &Event<()>, _context: &mut ScriptContext) {}

    fn on_update(&mut self, context: &mut ScriptContext) {
        let graph = &mut context.scene.graph;
        if let Some(player) = self.has_player_contact(context.handle, &graph) {
            Log::info("has player contact");
            if !self.hit.contains(&player) {
                Log::info("not seen");
                self.hit.push(player);
                context.message_sender.send_global(DamageMessage {
                    player_damage: self.player_damage,
                    package_damage: self.package_damage,
                });
            }
        } else {
            self.hit.clear();
        }
    }

    fn id(&self) -> Uuid {
        Self::type_uuid()
    }
}
    