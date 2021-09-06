mod afolder;
mod generic;
mod lifetime;
mod traits;

use afolder::afile;

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
    // folder
    afile::afn();
    println!("---------------------------------------");
}
