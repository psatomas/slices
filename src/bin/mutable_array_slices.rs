fn main() {
    let mut my_array = [10, 15, 20, 25, 30];
    let my_slice = &mut my_array[2..4];
    println!("My slice: {:?}", my_slice);

    my_slice[0] = 100;
    println!("My slice: {:?}", my_slice);
    println!("My array: {:?}", my_array);
}