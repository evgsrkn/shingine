use std::{
    collections::HashMap,
    env,
    fs,
    process::ExitCode,
};

// Find words by file
fn find_file_words(path: &str) -> HashMap<String, Vec<String>> {
    let mut file_words: HashMap<String, Vec<String>> = HashMap::new();
    for file in fs::read_dir(path).unwrap() {
        let file = file.unwrap();
        let text: String = fs::read_to_string(file.path()).unwrap();

        let words: Vec<String> = text.split_whitespace()
            .map(String::from)
            .collect();

        file_words.insert(file.file_name().into_string().expect("1"), words);
    }
    file_words
}

// Find word frequency by file
fn find_file_word_frequency(file_words: HashMap<String, Vec<String>>) -> HashMap<String, HashMap<String, f32>> {
    let mut frequencies: HashMap<String, HashMap<String, f32>> = HashMap::new();
    for (file, words) in file_words {
        for word in words {
            if frequencies.contains_key(&file) {
                if frequencies.get_mut(&file).unwrap().contains_key(&word) {
                    *frequencies.get_mut(&file).unwrap().get_mut(&word).unwrap() += 1.0;
                } else {
                    frequencies.get_mut(&file).unwrap().insert(word, 1.0);
                }
            } else {
                frequencies.insert(file.clone(), HashMap::new());
            }
        }
    }
    frequencies
}

// TF is the result of diviion of the frequency of a word
// in a file by the total number of words in a file
fn find_tf(indexes: &mut HashMap<String, HashMap<String, f32>>) -> &HashMap<String, HashMap<String, f32>> {
    for (file, word_freq) in indexes.clone() {
        let len = word_freq.len();
        for (word, _) in word_freq.clone() {
            *indexes.get_mut(&file).unwrap().get_mut(&word).unwrap() /= len as f32;
        }
    }
    indexes
}

// TODO find idf of words
// IDF is the result of division of the total number of files_count
// in a folder by the number of files containing the word
fn find_idf() {

}

fn index_files() {
    let path = "/home/srkn/projects/rust/search_engine/examples";
    let files_count = fs::read_dir(path).unwrap().count();
    let file_words = find_file_words(path);

    // Find word frequency in files
    let mut indexes = find_file_word_frequency(file_words);

    // Find TF (frequency / number of words) of words
    find_tf(&mut indexes);

    // Find IDF (number of files / )

    for (file, words) in indexes {
        println!("---------------{}---------------", file);
        for (word, freq) in words {
            println!("{} - {}", word, freq);
        }
    }
}

fn index() {
    index_files();
}

fn serve() {
    println!("You are in serve")
}

fn help(program: String) {
    println!("Usage: {program} [OPTION]\n");
    println!("Options:");
    println!("      help    Display this message");
    println!("      index   Index all files");
    println!("      serve   Start search server at http://localhost:1298");
}

fn entry() -> Result<(), ()> {
    let mut args = env::args();
    let program = args.next().expect("Path to program is expected!");

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
