#[derive(Debug, Clone, Copy, PartialEq)]
pub enum CommandType {
    A,
    C,
    L
}

pub struct Parser {
    commands: Vec<String>,
    next: usize,
    current: String
}

impl Parser {
    pub fn new(path: &str) -> std::io::Result<Parser> {
        let contents = std::fs::read_to_string(path)?;
        let mut commands = Vec::new();

        for line in contents.lines() {
            // Remove comment
            let no_comment = match line.split_once("//") {
                Some((before, _)) => before,
                None => line,
            };
            let cleaned: String = no_comment.chars().filter(|c| !c.is_whitespace()).collect();

            if cleaned.is_empty() {
                continue;
            }

            commands.push(cleaned);
        }

        Ok(Parser {
            commands,
            next: 0,
            current: String::new()
        })
    }

    pub fn has_more_commands(&self) -> bool {
        self.next < self.commands.len()
    }

    pub fn advance(&mut self) {
        self.current = self.commands[self.next].clone();
        self.next += 1;
    }

    pub fn reset(&mut self) {
        self.next = 0;
    }

    pub fn command_type(&self) -> CommandType {
        if self.current.starts_with('@') {
            CommandType::A
        } else if self.current.starts_with('(') {
            CommandType::L
        } else {
            CommandType::C
        }
    }

    pub fn symbol(&self) -> &str {
        match self.command_type() {
            CommandType::A => &self.current[1..],
            CommandType::L => &self.current[1..self.current.len() - 1],
            CommandType::C => ""
        }
    }

    pub fn dest(&self) -> &str {
        match self.current.split_once('=') {
            Some((d, _)) => d,
            None => ""
        }
    }

    pub fn jump(&self) -> &str {
        match self.current.split_once(';') {
            Some((_, j)) => j,
            None => ""
        }
    }

    pub fn comp(&self) -> &str {
        let after_dest = match self.current.split_once('=') {
            Some((_, rest)) => rest,
            None => &self.current,
        };
        match after_dest.split_once(';') {
            Some((c, _)) => c,
            None => after_dest
        }
    }
}