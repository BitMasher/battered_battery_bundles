
use fyrox::{
    core::{uuid::{Uuid, uuid}, visitor::prelude::*, reflect::prelude::*, TypeUuidProvider},
    event::Event, impl_component_provider,
    script::{ScriptContext, ScriptDeinitContext, ScriptTrait},
};

#[derive(Visit, Reflect, Default, Debug, Clone)]
pub struct TerrainEffect {
    pub accel_modifier: f32,
    pub max_speed_mod: f32
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

    fn on_update(&mut self, _context: &mut ScriptContext) {}

    fn id(&self) -> Uuid {
        Self::type_uuid()
    }
}
    