use std::{env, fs, process::ExitCode};

mod model;
mod server;
use model::*;

fn index_files() -> Collection {
    let mut collection = Collection::new();
    let path = "/home/srkn/projects/rust/search_engine/examples";
    for file in fs::read_dir(path).unwrap() {
        collection.add_doc(file.unwrap().path())
    }

    collection
}

fn index() {
    index_files();}

fn help(program: String) {
    println!("Usage: {program} [OPTION]\n");
    println!("Options:");
    println!("      help    Display this message");
    println!("      index   Index all files");
    println!("      serve   Start search server at http://localhost:7878");
}

fn entry() -> Result<(), ()> {
    let mut args = env::args();
    let program = args.next().expect("Path to program is expected!");

    match args.next() {
        Some(arg) => match arg.as_str() {
            "index" => index(),
            "serve" => {
                let collection = index_files();
                server::serve(&collection);
            }
            "help" => help(program),
            _ => println!("Not valid argument"),
        },
        None => help(program),
    }
    Ok(())
}

fn main() {
    match entry() {
        Ok(()) => ExitCode::SUCCESS,
        Err(()) => ExitCode::FAILURE,
    };
}
