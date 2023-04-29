use std::collections::HashMap;

struct Mycro {
    starter: String,
    commands: HashMap<String, String>,
}

impl Mycro {
    fn run(&self) {}
}

const DEFAULT_STARTER: &str = "|>";

fn main() {
    // read .config/mycro
    let mycro = read_config("$HOME/.config/mycro");
    mycro.run();
}

fn read_config(filepath: &str) -> Mycro {
    let mut commands = HashMap::new();
    commands.insert(String::from("hw"), String::from("hello, world"));
    return Mycro {
        starter: String::from(DEFAULT_STARTER),
        commands: commands,
    };
}
