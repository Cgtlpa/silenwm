mod compositor;
mod config;
mod window;

fn main() {
    let settings = match config::load("config/silenwm.toml") {
        Ok(config) => config,
        Err(error) => {
            eprintln!("Failed to load configuration: {error}");
            return;
        }
    };

    println!("Starting {}", settings.general.name);
    println!("Layout: {}", settings.layout.mode);
    println!("Master ratio: {}", settings.layout.master_ratio);

    println!("Focused border: {}", settings.appearance.focused_border);

    println!("Terminal shortcut: {}", settings.keybindings.terminal);

    // Start the graphical compositor
    if let Err(error) = compositor::run() {
        eprintln!("Compositor error: {error}");
    }
}
