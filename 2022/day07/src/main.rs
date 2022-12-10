use std::{collections::HashMap, io, iter::Peekable};

enum Command {
    ChangeDirectory(String),
    List(Vec<String>),
}

fn parse_command<I>(iter: &mut Peekable<I>) -> Option<Command>
where
    I: Iterator<Item = String>,
{
    let nc = iter.next();
    if nc.is_none() {
        return None;
    }
    let c = nc.unwrap();
    let mut data = String::new();
    loop {
        if let Some(s) = iter.peek() {
            if !s.starts_with("$ ") {
                let s = iter.next().unwrap();
                data += s.as_str();
                data += "\n";
                continue;
            }
        }
        break;
    }
    if c == "$ ls" {
        Some(Command::List(
            data.trim().split("\n").map(|t| t.to_owned()).collect(),
        ))
    } else if c.starts_with("$ cd ") {
        Some(Command::ChangeDirectory(
            c.strip_prefix("$ cd ").unwrap().to_string(),
        ))
    } else {
        panic!("{}", format!("Unknown command {}", c))
    }
}

fn parse_input<I>(iter: &mut I) -> Vec<Command>
where
    I: Iterator<Item = String>,
{
    let mut commands = Vec::<Command>::new();
    let mut peekable = iter.peekable();
    loop {
        match parse_command(&mut peekable) {
            Some(c) => commands.push(c),
            _ => break,
        }
    }
    commands
}

enum Item {
    File(u64),
    Dir(String),
}
type Items = HashMap<String, Item>;
type FileSystem = HashMap<String, Items>;

struct Shell {
    filesystem: FileSystem,
    current_path: Vec<String>,
}

impl Default for Shell {
    fn default() -> Self {
        Self {
            filesystem: FileSystem::default(),
            current_path: vec![],
        }
    }
}

impl Shell {
    fn change_dir(&mut self, dir: &str) {
        match dir {
            "/" => {
                self.current_path = vec![];
                self.ensure_current_dir_exists();
            }
            ".." => {
                self.current_path.pop();
            }
            _ => {
                self.current_path.push(dir.to_string());
                self.ensure_current_dir_exists();
            }
        }
    }

    fn ensure_current_dir_exists(&mut self) {
        let path = self.current_dir();
        self.ensure_dir_exists(path);
    }

    fn ensure_dir_exists(&mut self, path: String) {
        if !self.filesystem.contains_key(&path) {
            self.filesystem.insert(path, Items::default());
        }
    }

    fn current_dir(&self) -> String {
        format!("/{}", self.current_path.join("/"))
    }

    fn sub_dir(&self, dir: &str) -> String {
        let cd = self.current_dir();
        match cd.as_str() {
            "/" => format!("/{}", dir),
            _ => format!("{}/{}", self.current_dir(), dir),
        }
    }

    fn add_file(&mut self, file: String, size: u64) {
        let path = self.current_dir();
        let dir = self.filesystem.get_mut(&path).unwrap();
        dir.insert(file, Item::File(size));
    }

    fn add_dir(&mut self, name: String) {
        let path = self.current_dir();
        let sub_path = self.sub_dir(name.as_str());

        self.ensure_dir_exists(sub_path.clone());

        let parent = self.filesystem.get_mut(&path).unwrap();
        parent.insert(name, Item::Dir(sub_path));
    }

    fn list(&mut self, lines: &Vec<String>) {
        for l in lines {
            let t = l.split(" ").collect::<Vec<&str>>();
            match t[0] {
                "dir" => self.add_dir(t[1].to_string()),
                _ => self.add_file(t[1].to_string(), t[0].parse::<u64>().unwrap()),
            }
        }
    }

    fn execute(&mut self, c: &Command) {
        match c {
            Command::ChangeDirectory(dir) => self.change_dir(dir),
            Command::List(list) => self.list(list),
        }
    }
}

fn dir_size(fs: &FileSystem, path: &String) -> u64 {
    let items = fs.get(path).unwrap().values();
    items.fold(0_u64, |a, item| {
        let size = match item {
            Item::File(s) => *s,
            Item::Dir(path) => dir_size(fs, path),
        };
        a + size
    })
}

fn main() {
    let mut lines = io::stdin().lines().map(|l| l.unwrap());
    let commands = parse_input(&mut lines);

    let mut shell = Shell::default();
    for c in commands {
        shell.execute(&c);
    }
    let dirs = shell.filesystem.keys().collect::<Vec<&String>>();

    let r1 = dirs
        .iter()
        .map(|d| dir_size(&shell.filesystem, *d))
        .filter(|s| *s <= 100000_u64)
        .sum::<u64>();
    println!("{}", r1);
}
