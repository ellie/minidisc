use clap::Parser;

#[derive(Parser, Debug)]
pub struct Cmd {
    pub path: String,
}

impl Cmd {
    pub fn run(&self) {
        println!("{}", self.path);
    }
}
