use std::collections::HashMap;
use rand::Rng;

fn main() {
    let mut map = HashMap::new();
    map.insert(1, 2);

    let secret_number = rand::thread_rng().gen_range(1, 101);
}