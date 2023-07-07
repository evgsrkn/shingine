use std::{
    collections::HashMap,
    fs,
    path::PathBuf,
};

type TermFreq = HashMap<String, usize>;
type DocFreq = HashMap<String, usize>;
type Docs = HashMap<PathBuf, Doc>;
type Result = Vec<(PathBuf, f32)>;

#[derive(Debug)]
pub struct Collection {
    pub docs: Docs,
    words_df: DocFreq,
}

#[derive(Debug)]
pub struct Doc {
    words_tf: TermFreq,
}

impl Doc {
    pub fn new(path: &PathBuf) -> Self {
        let words: Vec<String> = fs::read_to_string(path)
            .unwrap()
            .to_lowercase()
            .split_whitespace()
            .map(String::from)
            .collect();

        Doc {
            words_tf: find_tf(words),
        }
    }
}

impl Collection {
    pub fn new() -> Self {
        Collection {
            docs: Docs::new(),
            words_df: DocFreq::new(),
        }
    }

    pub fn add_doc(&mut self, path: PathBuf) {
        self.remove_doc(&path);

        let doc = Doc::new(&path);

        for word in doc.words_tf.keys() {
            match self.words_df.get_mut(word) {
                Some(df) => *df += 1,
                None => {
                    self.words_df.insert(word.to_owned(), 1);
                }
            }
        }

        self.docs.insert(path, doc);
    }

    fn remove_doc(&mut self, path: &PathBuf) {
        if let Some(doc) = self.docs.remove(path) {
            for word in doc.words_tf.keys() {
                if let Some(df) = self.words_df.get_mut(word) {
                    *df -= 1;
                }
            }
        }
    }

    pub fn search(&self, query: &str) -> String {
        let mut res = Result::new();

        for (path, doc) in &self.docs {
            let mut index = 0f32;

            for word in query.to_lowercase().split_whitespace() {
                let tf = doc.words_tf.get(word).cloned().unwrap_or(0) as f32;
                let idf = find_idf(word, self.docs.len(), &self.words_df);
                index += tf * idf;
            }

            res.push((path.to_owned(), index));
        }

        res.sort_by(|a, b| b.1.partial_cmp(&a.1).unwrap());

        to_json(res)
    }
}

fn to_json(res: Result) -> String {
    let mut json = String::from("{");

    for (path, index) in res.iter() {
        let file = path.to_str().unwrap();
        json.push_str(format!("\"{file}\": \"{index}\",").as_str())
    }

    json.pop().unwrap();
    json.push('}');

    json
}

fn find_tf(words: Vec<String>) -> TermFreq {
    let mut tf = TermFreq::new();
    for word in words {
        match tf.get_mut(&word) {
            Some(x) => *x += 1,
            None => {
                tf.insert(word, 1);
            }
        };
    }

    tf
}

fn find_idf(word: &str, docs_count: usize, df: &DocFreq) -> f32 {
    (docs_count as f32 / df.get(word).cloned().unwrap_or(1) as f32).log10()
}
