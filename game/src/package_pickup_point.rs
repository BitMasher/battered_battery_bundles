
use fyrox::{
    core::{uuid::{Uuid, uuid}, visitor::prelude::*, reflect::prelude::*, TypeUuidProvider},
    event::Event, impl_component_provider,
    script::{ScriptContext, ScriptDeinitContext, ScriptTrait},
};

#[derive(Visit, Reflect, Default, Debug, Clone)]
pub struct PackagePickupPoint {
    pub is_drop_off: bool
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

    fn on_update(&mut self, _context: &mut ScriptContext) {}

    fn id(&self) -> Uuid {
        Self::type_uuid()
    }
}
    