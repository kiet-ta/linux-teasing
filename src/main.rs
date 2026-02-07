mod detector;
mod state;
mod renderer;
mod message;

use crossterm::style::{Color, Stylize};
use std::process::exit;

fn main() {
    // 1. Run Detector
    match detector::judge() {
        detector::Judgment::None => {
            // Ensure state is synced if it was UTC or same timezone (optional, prompt says "Update state silently" for UTC)
            // But logic says: IF UTC -> Do nothing. Update state silently.
            // IF Same -> Do nothing.
            
            // Let's just exit. State update on "Silent" is good practice to track "I saw you were good".
            // But to save IO, we might skip.
            // Prompt: "IF current_timezone == "UTC": Do nothing. Update state silently. Exit(0)."
            // Prompt: "IF current_timezone == last_known_timezone: Do nothing. Exit(0)."
            
            // So we need to distinguish UTC vs Same.
            // Re-evaluating detector logic?
            // Detector returns enum. Let's handle generic update.
             detector::update_state_after_judgment();
             exit(0);
        },
        detector::Judgment::Guilty => {
            // 2. Render Penguin
            let _ = renderer::render();

            // 3. Print Message
            println!(); 
            println!("{}", message::get_message().with(Color::Rgb { r: 255, g: 174, b: 0 }));
            println!();

            // 4. Update State
            detector::update_state_after_judgment();
            exit(0);
        }
    }
}
