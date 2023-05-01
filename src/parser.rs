use rdev::Key;
use std::collections::HashMap;

pub fn parse_commands(content: &str) -> (HashMap<String, Vec<Vec<Key>>>, String) {
    let mut commands = HashMap::new();
    let mut starter = String::new();

    content.lines().for_each(|line| {
        // ignore comments
        if line.starts_with('#') {
            return;
        }

        let mut parts: Vec<&str> = line.split('=').collect();
        if parts.len() < 2 {
            println!("malformed line '{}'", line);
            return;
        }

        if parts[0] == "starter" {
            starter = parts[1].to_string();
        }

        let cmd_macro = parts.remove(0);
        let mut cmd_result = String::new();
        for i in 0..parts.len() {
            // add back the '=' that was splitted
            if i > 0 {
                cmd_result += "="
            }
            cmd_result += parts[i];
        }
        // join remaining parts as they may contain a '=' that got splited

        commands.insert(String::from(cmd_macro), str_to_keys(&cmd_result));
    });
    commands.insert(
        String::from("hw"),
        vec![
            vec![Key::KeyH],
            vec![Key::KeyE],
            vec![Key::KeyL],
            vec![Key::KeyL],
            vec![Key::KeyO],
            vec![Key::Space],
            vec![Key::KeyW],
            vec![Key::KeyO],
            vec![Key::KeyR],
            vec![Key::KeyL],
            vec![Key::KeyD],
        ],
    );
    (commands, starter)
}

fn str_to_keys(s: &str) -> Vec<Vec<Key>> {
    // TODO: can't use a vector of keys.
    // Should use something that allows us to press
    // two different keys at the same time in order to key a uppercase
    // os things like !@#.
    let mut keys: Vec<Vec<Key>> = vec![];
    for c in s.chars() {
        match c {
            'a' => keys.push(vec![Key::KeyA]),
            'b' => keys.push(vec![Key::KeyB]),
            'c' => keys.push(vec![Key::KeyC]),
            'd' => keys.push(vec![Key::KeyD]),
            'e' => keys.push(vec![Key::KeyE]),
            'f' => keys.push(vec![Key::KeyF]),
            'g' => keys.push(vec![Key::KeyG]),
            'h' => keys.push(vec![Key::KeyH]),
            'i' => keys.push(vec![Key::KeyI]),
            'j' => keys.push(vec![Key::KeyJ]),
            'k' => keys.push(vec![Key::KeyK]),
            'l' => keys.push(vec![Key::KeyL]),
            'm' => keys.push(vec![Key::KeyM]),
            'n' => keys.push(vec![Key::KeyN]),
            'o' => keys.push(vec![Key::KeyO]),
            'p' => keys.push(vec![Key::KeyP]),
            'q' => keys.push(vec![Key::KeyQ]),
            'r' => keys.push(vec![Key::KeyR]),
            's' => keys.push(vec![Key::KeyS]),
            't' => keys.push(vec![Key::KeyT]),
            'u' => keys.push(vec![Key::KeyU]),
            'v' => keys.push(vec![Key::KeyV]),
            'w' => keys.push(vec![Key::KeyW]),
            'x' => keys.push(vec![Key::KeyX]),
            'y' => keys.push(vec![Key::KeyY]),
            'z' => keys.push(vec![Key::KeyZ]),
            '!' => keys.push(vec![Key::ShiftLeft, Key::Num1]),
            '@' => keys.push(vec![Key::ShiftLeft, Key::Num2]),
            '#' => keys.push(vec![Key::ShiftLeft, Key::Num3]),
            '$' => keys.push(vec![Key::ShiftLeft, Key::Num4]),
            '%' => keys.push(vec![Key::ShiftLeft, Key::Num5]),
            '^' => keys.push(vec![Key::ShiftLeft, Key::Num6]),
            '&' => keys.push(vec![Key::ShiftLeft, Key::Num7]),
            '*' => keys.push(vec![Key::ShiftLeft, Key::Num8]),
            '(' => keys.push(vec![Key::ShiftLeft, Key::Num9]),
            ')' => keys.push(vec![Key::ShiftLeft, Key::Num0]),
            ',' => keys.push(vec![Key::Comma]),
            '<' => keys.push(vec![Key::ShiftLeft, Key::Comma]),
            '.' => keys.push(vec![Key::Dot]),
            '>' => keys.push(vec![Key::ShiftLeft, Key::Dot]),
            '/' => keys.push(vec![Key::Slash]),
            '?' => keys.push(vec![Key::ShiftLeft, Key::Slash]),
            '\'' => keys.push(vec![Key::Quote]),
            '"' => keys.push(vec![Key::ShiftLeft, Key::Quote]),
            '=' => keys.push(vec![Key::Equal]),
            '+' => keys.push(vec![Key::ShiftLeft, Key::Equal]),
            '`' => keys.push(vec![Key::BackQuote]),
            '~' => keys.push(vec![Key::ShiftLeft, Key::BackQuote]),
            '\\' => keys.push(vec![Key::BackSlash]),
            '|' => keys.push(vec![Key::ShiftLeft, Key::BackSlash]),
            '-' => keys.push(vec![Key::Minus]),
            '_' => keys.push(vec![Key::ShiftLeft, Key::Minus]),
            _ => continue,
        }
    }
    keys
}
