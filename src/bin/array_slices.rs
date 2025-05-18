fn main() {
    let values = [4, 8, 15, 16, 23, 42];

    let my_slice = &values[0..4];
    println!("{my_slice:?}");

    let my_slice = &values[2..4];
    println!("{my_slice:?}");

    let my_slice = &values[2..];
    println!("{my_slice:?}");

    let my_slice = &values[..];
    println!("{my_slice:?}");

    let my_slice = &values;
    println!("{my_slice:?}");
}