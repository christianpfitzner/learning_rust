

// this tool show you if all repositories are upd to date
use std::process;
use std::process::Command; 
use std::io;
use std::fs::{self, DirEntry};
use std::path::Path;
use std::path::PathBuf;
use std::ops::Not;



fn main()
{

let git_folder = Path::new("/home/chris/workspace/git"); 


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

let files = fs::read_dir(git_folder).unwrap();

for path in files {
    println!("Name: {}", path.unwrap().path().display()); 

 
    // check if the found folders contain a .git folder
    // if this is the case, these folders are repositories


    let appending_thing = Path::new("/.git");
    // append .git/ for the git folder
    let dgit_folder_in_dir = path.unwrap().path().join(&appending_thing); 


}





}