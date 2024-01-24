pub use plugon_macros;
pub use dlopen::wrapper::{Container, WrapperApi};

pub struct PluginLoader {}

impl PluginLoader {
    pub fn load<PluginType: WrapperApi>(&mut self, path: &str) -> Container<PluginType> {
        unsafe { Container::load(path) }.expect("Could not open library or load symbols")
    }
}
