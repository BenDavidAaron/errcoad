use clipboard::{ClipboardContext, ClipboardProvider};
use rand::Rng;
use std::{thread, time::Duration};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut ctx: ClipboardContext = ClipboardProvider::new()?;
    let mut rng = rand::thread_rng();

    println!("Generating random strings to clipboard. Press Ctrl+C to stop...");

    loop {
        let random_string: String = (0..7)
            .map(|_| {
                let char_code = rng.gen_range(65..=90);
                char::from_u32(char_code).unwrap()
            })
            .collect();

        ctx.set_contents(random_string.clone())?;
        println!("Copied to clipboard: {}", random_string);

        thread::sleep(Duration::from_secs(1));
    }
}
