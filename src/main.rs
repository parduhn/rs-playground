mod generic;
mod lifetime;
mod traits;

fn main() {
    // generische Typen
    generic::run();
    // gültigkeiten
    lifetime::run();
    // merkmale
    traits::run();
}
