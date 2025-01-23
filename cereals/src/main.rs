#[derive(Debug)]  // grants debug trait, allowing use of {:?}
enum Cereal {
    Barley, Millet, Rice,
    Rye, Spelt, Wheat,
}

fn main() {
    let mut grains: Vec<Cereal> = vec![]; // init empty vector
    grains.push(Cereal::Rye);
    drop(grains);
    // illegal borrow of moved val, does not compile!!!
    println!("{:?}", grains);
}
