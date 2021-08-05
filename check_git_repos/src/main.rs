

// this tool show you if all repositories are upd to date
use std::process;
use std::process::Command; 
use std::io;
use std::fs::{self, DirEntry};
use std::path::Path;

use std::ops::Not;


fn main()
{

let git_folder = Path::new("/home/chris/workspace/gi"); 


if git_folder.exists().not() 
{
    println!("path does not exist. Abortin program. ");
    process::exit(0x100);
}

if git_folder.is_dir()
{
    println!("path is directory");
}


// output of the path to the terminal
println!("{}", git_folder.display());





}