use std::{
    env::{self},
    process::ExitCode,
};

struct Index {
    name: String,
    frequency: i32,
}

fn index() {
    println!("You are in index")
}

fn serve() {
    println!("You are in serve")
}

fn help(program: String) {
    println!("Usage: {program} [OPTION]\n");
    println!("Options:");
    println!("      help    Display this message");
    println!("      index   Index all files");
    println!("      serve   Start search server");
}

fn entry() -> Result<(), ()> {
    let mut args = env::args();
    let program = args.next().expect("Path to program is expected");

    match args.next() {
        Some(arg) => match arg.as_str() {
            "index" => index(),
            "serve" => serve(),
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
