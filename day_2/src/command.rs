pub enum CommandType {
    Forward,
    Down,
    Up,
}

pub struct Command {
    pub command_type: CommandType,
    pub amount: i32,
}

impl Command {
    pub fn new(command_line: String) -> Self {
        let split_command_line: Vec<&str> = command_line.split(" ").collect();
        let command_type = match split_command_line[0] {
            "forward" => CommandType::Forward,
            "up" => CommandType::Up,
            "down" => CommandType::Down,
            _ => panic!("Command not found!"),
        };
        Self {
            command_type,
            amount: split_command_line[1].parse::<i32>().unwrap(),
        }
    }
}
