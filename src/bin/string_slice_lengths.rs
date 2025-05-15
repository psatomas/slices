fn main() {
    let food = "🍕";
    println!("{}", food.len());
    let pizza_slice = &food[0..4];
    println!("{}", pizza_slice.len());
}