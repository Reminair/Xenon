use std::{
    env,
    io,
    fs,
    path::PathBuf
};
use crate::cli;

pub fn run() -> io::Result<()> {
    let mut currentDir = env::current_dir();
    println!("{}", currentDir);
}
