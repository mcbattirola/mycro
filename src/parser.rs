use rdev::Key;
use std::collections::HashMap;

pub fn parse_commands(content: &str) -> (HashMap<String, Vec<Key>>, String) {
    let mut commands = HashMap::new();
    let mut starter = String::new();

    content.lines().for_each(|line| {
        // ignore comments
        if line.starts_with("#") {
            return;
        }

        let parts: Vec<&str> = line.split("=").collect();
        if parts.len() < 2 {
            println!("malformed line '{}'", line);
            return;
        }

        if parts[0] == "starter" {
            starter = parts[1].to_string();
        }

        commands.insert(String::from(parts[0]), str_to_keys(parts[1]));
    });
    commands.insert(
        String::from("hw"),
        vec![
            Key::KeyH,
            Key::KeyE,
            Key::KeyL,
            Key::KeyL,
            Key::KeyO,
            Key::Space,
            Key::KeyW,
            Key::KeyO,
            Key::KeyR,
            Key::KeyL,
            Key::KeyD,
        ],
    );
    return (commands, String::from("|>"));
}

fn str_to_keys(s: &str) -> Vec<Key> {
    // TODO: can't use a vector of keys.
    // Should use something that allows us to press
    // two different keys at the same time in order to key a uppercase
    // os things like !@#.
    let mut keys: Vec<Key> = vec![];
    for c in s.chars() {
        match c {
            'a' => keys.push(Key::KeyA),
            'b' => keys.push(Key::KeyB),
            'c' => keys.push(Key::KeyC),
            'd' => keys.push(Key::KeyD),
            'e' => keys.push(Key::KeyE),
            'f' => keys.push(Key::KeyF),
            'g' => keys.push(Key::KeyG),
            'h' => keys.push(Key::KeyH),
            'i' => keys.push(Key::KeyI),
            'j' => keys.push(Key::KeyJ),
            'k' => keys.push(Key::KeyK),
            'l' => keys.push(Key::KeyL),
            'm' => keys.push(Key::KeyM),
            'n' => keys.push(Key::KeyN),
            'o' => keys.push(Key::KeyO),
            'p' => keys.push(Key::KeyP),
            'q' => keys.push(Key::KeyQ),
            'r' => keys.push(Key::KeyR),
            's' => keys.push(Key::KeyS),
            't' => keys.push(Key::KeyT),
            'u' => keys.push(Key::KeyU),
            'v' => keys.push(Key::KeyV),
            'w' => keys.push(Key::KeyW),
            'x' => keys.push(Key::KeyX),
            'y' => keys.push(Key::KeyY),
            'z' => keys.push(Key::KeyZ),
            '@' => {
                keys.push(Key::ShiftLeft);
                keys.push(Key::Num2);
            }
            '.' => keys.push(Key::Dot),
            _ => continue,
        }
    }

    return keys;
}
