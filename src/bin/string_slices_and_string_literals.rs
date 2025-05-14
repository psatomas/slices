fn main() {
    let first_name = {
        let action_hero = "Arnold Schwarzenegger";
        &action_hero[0..6]
    };

    println!("{first_name}");
}