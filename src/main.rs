mod model;

fn main() {
    println!("Hello, world!");

    let n1 = model::base_types::Number::new(245);

    let n2 = model::base_types::Number::new(30);

    println!("2 + 3 = {}",(n2-n1).to_string())
}
