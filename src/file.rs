use std::fmt;
use std::fmt::{Display};
#[allow(dead_code)]
#[derive(Debug, PartialEq)]
enum FileState {
    Open,
    Closed,
}
#[allow(dead_code)]
#[derive(Debug)]
struct File {
    name: String,
    data: Vec<u8>,
    state: FileState,
}

impl Display for FileState {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            FileState::Open => write!(f, "OPEN"),
            FileState::Closed => write!(f, "CLOSED"),
        }
    }
}

impl Display for File {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "<{} ({})>", self.name, self.state)
    }
}

impl File {
    #[allow(dead_code)]
    fn new(name: String) -> File {
        File {
            name,
            data: Vec::new(),
            state: FileState::Closed,
        }
    }
}

fn _use_file() {
    let file1 = File::new("file1.txt".to_string());
    println!("{:?}", file1);
    println!("{}", file1);
}