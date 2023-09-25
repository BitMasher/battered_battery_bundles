
use fyrox::{
    core::{uuid::{Uuid, uuid}, visitor::prelude::*, reflect::prelude::*, TypeUuidProvider},
    event::Event, impl_component_provider,
    script::{ScriptContext, ScriptDeinitContext, ScriptTrait},
};
use fyrox::core::pool::Handle;
use fyrox::material::SharedMaterial;
use fyrox::scene::collider::Collider;
use fyrox::scene::graph::Graph;
use fyrox::scene::mesh::Mesh;
use fyrox::scene::node::Node;
use crate::player_controller::PlayerController;

#[derive(Visit, Reflect, Default, Debug, Clone)]
pub struct PackagePickupPoint {
    pub is_drop_off: bool,

    point_mesh: Handle<Node>,

    deactivated_material: SharedMaterial,
}

impl PackagePickupPoint {
    pub fn has_player_contact(&self, handle: Handle<Node>, graph: &Graph) -> bool {
        if let Some(collider) = graph.try_get_of_type::<Collider>(handle) {
            for contact in collider.intersects(&graph.physics) {
                if contact.has_any_active_contact {
                    let opposing_handle = if contact.collider1.eq(&handle) { contact.collider2 } else { contact.collider1 };
                    if let Some(parent) = graph.try_get(opposing_handle).map(|n| n.parent()) {
                        if graph.find_up(parent, &mut |n| {
                            return n.script().and_then(|s|s.cast::<PlayerController>()).is_some();
                        }).is_some() {
                            return true;
                        }
                    }
                }
            }
        }
        false
    }
}

impl_component_provider!(PackagePickupPoint);

impl TypeUuidProvider for PackagePickupPoint {
    fn type_uuid() -> Uuid {
        uuid!("77112419-6979-41bc-84f5-f3894490a388")
    }
}

impl ScriptTrait for PackagePickupPoint {
    fn on_init(&mut self, _context: &mut ScriptContext) {}

    fn on_start(&mut self, _context: &mut ScriptContext) {}

    fn on_deinit(&mut self, _context: &mut ScriptDeinitContext) {}

    fn on_os_event(&mut self, _event: &Event<()>, _context: &mut ScriptContext) {}

    fn on_update(&mut self, context: &mut ScriptContext) {
        let mut graph = &mut context.scene.graph;
        if self.has_player_contact(context.handle, &graph) {
            graph[context.handle].set_enabled(false);
            if let Some(mesh) = graph[self.point_mesh].cast_mut::<Mesh>() {
                mesh.surfaces_mut()[0].set_material(self.deactivated_material.clone())
            }
        }
    }

    fn id(&self) -> Uuid {
        Self::type_uuid()
    }
}
    