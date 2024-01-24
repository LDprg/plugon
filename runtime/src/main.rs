#[macro_use]
extern crate dlopen_derive;

use plugon::{PluginLoader, WrapperApi};

#[derive(WrapperApi)]
struct Api {
    test: fn(),
}

fn main() {
    println!("Load Plugin...");

    let mut loader = PluginLoader {};

    let dll = loader.load::<Api>("libplugin.so");

    dll.test();
}
