mod generic;
mod lifetime;
mod traits;

fn main() {
    // generische Typen
    generic::run();
    println!("---------------------------------------");
    // gültigkeiten
    lifetime::run();
    println!("---------------------------------------");
    // merkmale
    traits::run();
    println!("---------------------------------------");
}
