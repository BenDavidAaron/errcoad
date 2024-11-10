use clipboard::{ClipboardContext, ClipboardProvider};
use rand::Rng;
use std::{env, process, thread, time::Duration};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args: Vec<String> = env::args().collect();
    if args.len() != 2 {
        eprintln!("Usage: {} <template>", args[0]);
        eprintln!("Example: {} 'pre-####-post'", args[0]);
        eprintln!("         # characters will be replaced with random ASCII");
        process::exit(1);
    }

    let template = &args[1];
    let mut ctx: ClipboardContext = ClipboardProvider::new()?;
    let mut rng = rand::thread_rng();

    println!("Generating random strings using template: {}", template);
    println!("Press Ctrl+C to stop...");

    loop {
        let random_string = template
            .chars()
            .map(|c| {
                if c == '#' {
                    let char_code = rng.gen_range(33..=126);
                    char::from_u32(char_code).unwrap()
                } else {
                    c
                }
            })
            .collect::<String>();

        ctx.set_contents(random_string.clone())?;
        println!("Copied to clipboard: {}", random_string);

        thread::sleep(Duration::from_secs(1));
    }
}
