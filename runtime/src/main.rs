use plugon::PluginLoader;

fn main() {
    println!("Load Plugin...");

    let loader: PluginLoader;

    loader.load("./plugin.so");
}
