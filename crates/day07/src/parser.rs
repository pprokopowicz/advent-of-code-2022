use std::{cell::RefCell, rc::Rc};

use reader::{read_file, File as InputFile};

pub fn parse() -> Rc<RefCell<Node>> {
    let input = read_file(InputFile::Day07);

    let root = Rc::new(RefCell::new(Node::new(FileType::Directory(
        "/".to_string(),
    ))));
    let mut current = Rc::clone(&root);

    let lines = input.lines().skip(1).collect::<Vec<&str>>();

    for line in lines {
        if line.starts_with("$") {
            let command = parse_command(&line);

            match &command {
                Command::List => {}
                Command::PreviousDirectory => {
                    let current_clone = Rc::clone(&current);
                    current = Rc::clone(current_clone.borrow().parent.as_ref().unwrap());
                }
                Command::NextDirectory(name) => {
                    let current_clone = Rc::clone(&current);
                    let borrowed_current = current_clone.borrow();
                    let node = borrowed_current
                        .children
                        .iter()
                        .filter(|child| match &child.borrow().value {
                            FileType::Directory(file_name) => file_name == name,
                            FileType::File(_, _) => false,
                        })
                        .next()
                        .unwrap();

                    current = Rc::clone(node);
                }
            }
        } else {
            let file = parse_file(&line);
            let child = Rc::new(RefCell::new(Node::new(file)));
            {
                let mut mut_child = child.borrow_mut();
                mut_child.parent = Some(Rc::clone(&current));
            }
            current.borrow_mut().children.push(child);
        }
    }

    root
}

fn parse_command(input: &str) -> Command {
    if input.starts_with("$ cd") {
        let split = input.split_whitespace().collect::<Vec<&str>>();
        if split[split.len() - 1] == ".." {
            return Command::PreviousDirectory;
        } else {
            return Command::NextDirectory(split[split.len() - 1].to_string());
        }
    } else if input == "$ ls".to_string() {
        return Command::List;
    } else {
        panic!("Unknown command: {}", input)
    }
}

fn parse_file(input: &str) -> FileType {
    let split = input.split_whitespace().collect::<Vec<&str>>();
    if split[0] == "dir".to_string() {
        return FileType::Directory(split[1].to_string());
    } else {
        let size = split[0].parse::<usize>().unwrap();
        let name = split[1].to_string();

        return FileType::File(name, size);
    }
}

#[derive(Debug)]
enum Command {
    NextDirectory(String),
    PreviousDirectory,
    List,
}

#[derive(Debug)]
pub enum FileType {
    Directory(String),
    File(String, usize),
}

#[derive(Debug)]
pub struct Node {
    pub value: FileType,
    pub children: Vec<Rc<RefCell<Node>>>,
    pub parent: Option<Rc<RefCell<Node>>>,
}

impl Node {
    fn new(value: FileType) -> Node {
        Node {
            value: value,
            children: vec![],
            parent: None,
        }
    }

    pub fn size(&self) -> usize {
        match self.value {
            FileType::File(_, size) => size,
            FileType::Directory(_) => self
                .children
                .iter()
                .map(|child| child.borrow().size())
                .sum::<usize>(),
        }
    }
}
