
use fyrox::{
    core::{uuid::{Uuid, uuid}, visitor::prelude::*, reflect::prelude::*, TypeUuidProvider},
    event::Event, impl_component_provider,
    script::{ScriptContext, ScriptDeinitContext, ScriptTrait},
};

#[derive(Visit, Reflect, Default, Debug, Clone)]
pub struct ReverseDirection {}

impl_component_provider!(ReverseDirection);

impl TypeUuidProvider for ReverseDirection {
    fn type_uuid() -> Uuid {
        uuid!("4cf1d5a0-8820-472c-b5ff-893b82c3f15b")
    }
}

impl ScriptTrait for ReverseDirection {
    fn on_init(&mut self, _context: &mut ScriptContext) {}

    fn on_start(&mut self, _context: &mut ScriptContext) {}

    fn on_deinit(&mut self, _context: &mut ScriptDeinitContext) {}

    fn on_os_event(&mut self, _event: &Event<()>, _context: &mut ScriptContext) {}

    fn on_update(&mut self, _context: &mut ScriptContext) {}

    fn id(&self) -> Uuid {
        Self::type_uuid()
    }
}
    