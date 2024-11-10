use rand::Rng;

fn main() {
    let random_string: String = (0..7)
        .map(|_| {
            let mut rng = rand::thread_rng();
            let char_code = rng.gen_range(65..=90);
            char::from_u32(char_code).unwrap()
        })
        .collect();

    println!("{}", random_string);
}
