// Crate-Specific libraries
use chip8::Chip8;
use chip8_emu::Chip8App;

// Standard Libaries
use std::io;

// External Libraries

fn cpu_main() {
    // Set up render system and register input callbacks
    // Initialize the Chip8 system and load the game into the memory
    // Emulation Loop

    // Emulate one cycle
    // If DrawFlag is set, Update the Screen
    // Store key press state ( Press and Release )
    
}

fn main() {
    // Log to stdout (if you run with `RUST_LOG=debug`)
    tracing_subscriber::fmt::init();

    let native_options = eframe::NativeOptions::default();
    
    eframe::run_native(
        "chip8_emu",
        native_options,
        Box::new(|cc| Box::new(chip8_emu::Chip8App::new(cc))),
    );
}
