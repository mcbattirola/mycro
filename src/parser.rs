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
        for (i, part) in parts.iter().enumerate() {
            // add back the '=' that was splitted
            if i > 0 {
                cmd_result += "="
            }
            cmd_result += part;
        }
        // join remaining parts as they may contain a '=' that got splited

        commands.insert(String::from(cmd_macro), str_to_keys(&cmd_result));
    });
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
            'A' => keys.push(vec![Key::ShiftLeft, Key::KeyA]),
            'B' => keys.push(vec![Key::ShiftLeft, Key::KeyB]),
            'C' => keys.push(vec![Key::ShiftLeft, Key::KeyC]),
            'D' => keys.push(vec![Key::ShiftLeft, Key::KeyD]),
            'E' => keys.push(vec![Key::ShiftLeft, Key::KeyE]),
            'F' => keys.push(vec![Key::ShiftLeft, Key::KeyF]),
            'G' => keys.push(vec![Key::ShiftLeft, Key::KeyG]),
            'H' => keys.push(vec![Key::ShiftLeft, Key::KeyH]),
            'I' => keys.push(vec![Key::ShiftLeft, Key::KeyI]),
            'J' => keys.push(vec![Key::ShiftLeft, Key::KeyJ]),
            'K' => keys.push(vec![Key::ShiftLeft, Key::KeyK]),
            'L' => keys.push(vec![Key::ShiftLeft, Key::KeyL]),
            'M' => keys.push(vec![Key::ShiftLeft, Key::KeyM]),
            'N' => keys.push(vec![Key::ShiftLeft, Key::KeyN]),
            'O' => keys.push(vec![Key::ShiftLeft, Key::KeyO]),
            'P' => keys.push(vec![Key::ShiftLeft, Key::KeyP]),
            'Q' => keys.push(vec![Key::ShiftLeft, Key::KeyQ]),
            'R' => keys.push(vec![Key::ShiftLeft, Key::KeyR]),
            'S' => keys.push(vec![Key::ShiftLeft, Key::KeyS]),
            'T' => keys.push(vec![Key::ShiftLeft, Key::KeyT]),
            'U' => keys.push(vec![Key::ShiftLeft, Key::KeyU]),
            'V' => keys.push(vec![Key::ShiftLeft, Key::KeyV]),
            'W' => keys.push(vec![Key::ShiftLeft, Key::KeyW]),
            'X' => keys.push(vec![Key::ShiftLeft, Key::KeyX]),
            'Y' => keys.push(vec![Key::ShiftLeft, Key::KeyY]),
            'Z' => keys.push(vec![Key::ShiftLeft, Key::KeyZ]),
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
            ' ' => keys.push(vec![Key::Space]),
            _ => continue,
        }
    }
    keys
}
