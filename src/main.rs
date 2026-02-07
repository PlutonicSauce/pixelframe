mod animation;
mod frame;
mod player;
mod terminal;

use animation::Animation;
use player::AnimationPlayer;
use std::env;
use std::path::Path;

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        println!("Usage: pixelframe <path-to-pixel-image> [frame-delay-ms]");
        println!("\nExample: pixelframe art.png 100");
        println!("         pixelframe ./animations/sprite.png 50");
        std::process::exit(1);
    }

    let image_path = &args[1];
    let frame_delay = if args.len() > 2 {
        args[2].parse::<u64>().unwrap_or(100)
    } else {
        100
    };

    if !Path::new(image_path).exists() {
        eprintln!("Error: File '{}' not found", image_path);
        std::process::exit(1);
    }

    match Animation::load(image_path) {
        Ok(animation) => {
            println!("✓ Loaded animation: {} frames ({}x{})", 
                     animation.frame_count(), 
                     animation.width(), 
                     animation.height());
            println!("  Frame delay: {}ms", frame_delay);
            println!("  Press Ctrl+C to stop\n");

            let player = AnimationPlayer::new(animation, frame_delay);
            if let Err(e) = player.play() {
                eprintln!("Error playing animation: {}", e);
                std::process::exit(1);
            }
        }
        Err(e) => {
            eprintln!("Error loading image: {}", e);
            std::process::exit(1);
        }
    }
}