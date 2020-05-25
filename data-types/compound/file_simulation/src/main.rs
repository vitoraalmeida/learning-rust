extern crate rand;
use rand::Rng;

use std::fmt;
use std::fmt::{Display};

fn one_in(n: u32) -> bool {
    rand::thread_rng().gen_weighted_bool(n)
}

#[derive(Debug, PartialEq)]
enum FileState {
    Open,
    Closed,
}

#[derive(Debug)] //permite a impressão do struct usando {:?}
struct File {
    name: String,
    data: Vec<u8>,
    state: FileState,
}

impl Display for FileState { //para ser impresso sem passar {:?} debug
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            FileState::Open => write!(f, "OPEN"),
            FileState::Closed => write!(f, "CLOSED"),
        }
    }
}

impl Display for File { //para ser impresso sem passar {:?} debug
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "<{} ({})>", self.name, self.state)
    }
}


impl File {
    fn new(name: &str) -> File {
        File {
            name: String::from(name),
            data: Vec::new(),
            state: FileState::Closed
        }
    }

    fn new_with_data(name: &str, data: &Vec<u8>) -> File {
        let mut f = File::new(name);
        f.data = data.clone();
        f.state = FileState::Closed;
        f
    }

    fn read(self: &File, save_to: &mut Vec<u8>) -> Result<usize, String> {
        if self.state != FileState::Open {
            return Err(String::from("File must be open for reading"));
        }
        let mut tmp = self.data.clone();
        let read_length = tmp.len();
        save_to.reserve(read_length);
        save_to.append(&mut tmp);
        Ok(read_length)
    }
}


fn open(mut f: File) -> Result<File, String> {
    if one_in(10) {
        let err_msg = String::from("Permission denied");
        return Err(err_msg);
    }
    f.state = FileState::Open;
    Ok(f)
}

fn close(mut f:  File) -> Result<File, String> {
    if one_in(10) {
        let err_msg = String::from("Interrupted by signal");
        return Err(err_msg);
    }
    f.state = FileState::Closed;
    Ok(f)
}

fn main() {
    /*
    let mut f1 = File {
        name: String::from("f1.txt"),
        data: vec![114, 117, 115, 116, 33],
    };
    */
    let f1_data: Vec<u8> = vec![114, 117, 115, 116, 33];
    let mut f1 = File::new_with_data("f1.txt", &f1_data);

    let mut buffer: Vec<u8> = vec![];

    f1 = open(f1).unwrap();
    let f1_length = f1.read(&mut buffer).unwrap();
    f1 = close(f1).unwrap();

    let text = String::from_utf8_lossy(&buffer); //converte Vec<u8> pra String
   
    //debug
    println!("{:?}", f1);
    //display implementado explicitamente
    println!("{}", f1);
    println!("{} is {} bytes long", &f1.name, &f1_length);
    println!("{}", text)
}
