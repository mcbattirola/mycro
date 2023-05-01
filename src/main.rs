use core::time::Duration;
use lazy_static::lazy_static;
use rdev::{listen, simulate, Event, EventType, Key};
use std::collections::HashMap;
use std::fs;
use std::sync::RwLock;

pub mod parser;

#[derive(Debug)]
enum CommandState {
    ReadingStarter(usize), // reading, holds current index
    ReadingCommand,        // found the starter, now we're reading the command
}

struct Mycro {
    starter: String,
    // comands is a hashmap from the macro to the list of keys that should
    // emited.
    // Commands are a vector of vectors of keys because some characters
    // may result in multiple keys, like a '@' that needs a left shift and
    // a '2' pressed together.
    commands: HashMap<String, Vec<Vec<Key>>>,
    buffer: String,
    state: CommandState,
}

impl Mycro {
    fn handle_event(&mut self, event: Event) {
        if let Some(key) = event.name {
            match self.state {
                CommandState::ReadingCommand => {
                    // restart reading command on white space
                    if key == " " {
                        self.state = CommandState::ReadingStarter(0);
                        self.buffer = String::new();
                        return;
                    }

                    // backspace
                    if key == "\u{8}" {
                        self.buffer.remove(self.buffer.len() - 1);
                        return;
                    }

                    self.buffer += &key;

                    // for now, lets search the hashmap on each keystroke
                    let cmd = match self.commands.get(&self.buffer) {
                        Some(c) => c,
                        None => {
                            return;
                        }
                    };

                    // delete what is in the buffer and emit the command
                    delete_keys(self.buffer.len() + self.starter.len());
                    send_keys(cmd);

                    self.buffer = String::new();
                    self.state = CommandState::ReadingStarter(0);
                }
                CommandState::ReadingStarter(index) => {
                    let c = match self.starter.chars().nth(index) {
                        Some(c) => c,
                        None => return,
                    };

                    if key == c.to_string() {
                        // if found the last starter char, start reading command
                        if index + 1 == self.starter.len() {
                            self.state = CommandState::ReadingCommand;
                            return;
                        }

                        self.state = CommandState::ReadingStarter(index + 1);
                        return;
                    }
                    self.state = CommandState::ReadingStarter(0);
                }
            }
        }
    }
}

lazy_static! {
    static ref MYCRO: RwLock<Mycro> = RwLock::new(read_config("/home/matheus/.config/mycro")); // TODO
}

fn handle_event(event: Event) {
    let mut mycro = MYCRO.write().unwrap();
    mycro.handle_event(event);
}

const DEFAULT_STARTER: &str = "|>";

fn main() {
    match listen(handle_event) {
        Ok(_) => println!("Listening for events..."),
        Err(e) => println!("Error: {:?}", e),
    }
}

fn read_config(filepath: &str) -> Mycro {
    let contents = fs::read_to_string(filepath).expect("Config file not found");

    let (commands, mut starter) = parser::parse_commands(&contents);
    if starter.is_empty() {
        starter = String::from(DEFAULT_STARTER);
    }

    Mycro {
        starter,
        commands,
        buffer: String::new(),
        state: CommandState::ReadingStarter(0),
    }
}

fn send(event_type: &EventType) {
    let delay = Duration::from_millis(20);
    match simulate(event_type) {
        Ok(()) => (),
        Err(_simulate_error) => {
            println!("We could not send {:?}", event_type);
        }
    }
    // Let the OS catchup
    std::thread::sleep(delay);
}

fn send_keys(cmd: &[Vec<Key>]) {
    for k in cmd.iter() {
        // press all keys
        for sk in k.iter() {
            send(&EventType::KeyPress(*sk));
        }
        // release all keys
        for sk in k.iter() {
            send(&EventType::KeyRelease(*sk));
        }
    }
}

fn delete_keys(qty: usize) {
    for _ in 0..qty {
        send(&EventType::KeyPress(Key::Backspace));
        send(&EventType::KeyRelease(Key::Backspace));
    }
}
