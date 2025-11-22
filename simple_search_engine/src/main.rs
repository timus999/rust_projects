use std::any::type_name;
use std::collections::HashMap;
use std::path::Path;
use std::sync::{LazyLock, RwLock};
use std::{fs, io};

static INVERTED_INDEX: LazyLock<RwLock<HashMap<String, Vec<String>>>> =
    LazyLock::new(|| RwLock::new(HashMap::new()));

static INDEX: LazyLock<RwLock<HashMap<String, Vec<String>>>> =
    LazyLock::new(|| RwLock::new(HashMap::new()));
#[derive(Debug)]
pub struct Document {
    name: String,
    path: String,
    content: String,
}

impl Document {
    pub fn new(name: String, path: String, content: String) -> Self {
        Document {
            name,
            path,
            content,
        }
    }
}

fn type_of<T>(_: T) -> &'static str {
    type_name::<T>()
}

fn main() -> io::Result<()> {
    println!("Welcome to our Simple Search Engine program...");

    println!("Enter the folder you want to scan: ");
    let mut folder_path: String = String::new();

    io::stdin()
        .read_line(&mut folder_path)
        .expect("Failed to read folder path....");

    let folder_path = folder_path.trim();
    let directories = fs::read_dir(folder_path);

    if let Ok(dir) = directories {
        let list_of_directories: Vec<_> = dir.collect();
        println!("Here are all the files: ");
        for dir in list_of_directories {
            let dir = dir?;
            println!("{:?}", dir.path());
            let dir_path = dir.path();
            let file_path = Path::new(&dir_path);
            if let Some(extension) = file_path.extension() {
                match extension.to_string_lossy().as_ref() {
                    "txt" => {
                        let file_name = file_path.file_name().unwrap().to_os_string().into_string();
                        if let Ok(file_name) = file_name {
                            let documents = Document::new(
                                file_name.clone(),
                                file_path.to_str().unwrap().to_string(),
                                fs::read_to_string(&file_path).unwrap().to_string(),
                            );

                            println!("Metadata: {:?}", documents);
                            let normalized_text =
                                normalize_text(fs::read_to_string(&file_path).unwrap().to_string());
                            if let Ok(normalized_text) = normalized_text {
                                create_index(file_name, normalized_text);
                                create_inverted_index();
                            }
                        }
                    }
                    _ => println!("Does not support this kind of files"),
                }
            }
        }
    }
    search_query();
    Ok(())
}

fn normalize_text(content: String) -> std::io::Result<Vec<String>> {
    let content_char = content.chars();
    let mut final_content: String = String::new();
    for mut c in content_char {
        if c.is_ascii_alphanumeric() || c.is_ascii_whitespace() {
            if c.is_ascii_uppercase() {
                c = ' ';
            }
        } else {
            c = ' ';
        }
        final_content.push(c);
    }
    println!("{}", final_content);
    let tokenized: Vec<String> = final_content
        .split_whitespace()
        .map(|s| s.to_string())
        .collect();
    println!("{:?}", tokenized);
    Ok(tokenized)
}

fn create_index(file_name: String, list_of_words: Vec<String>) {
    let mut index = INDEX.write().unwrap();
    index.insert(file_name, list_of_words);
    println!("{:?}", index);
}

fn create_inverted_index() {
    let mut inverted_index = INVERTED_INDEX.write().unwrap();
    let index = INDEX.read().unwrap();

    for (file_name, words) in index.iter() {
        println!("{} file contains: ", file_name);
        for word in words {
            println!("Word: {}, filename: {}", word, file_name);
            if inverted_index.contains_key(word.as_str()) {
                inverted_index
                    .get_mut(word.as_str())
                    .unwrap()
                    .push(file_name.to_string());
            } else {
                let mut file_list: Vec<String> = Vec::new();
                inverted_index.insert(word.to_string(), file_list);
            }
        }
    }
    println!("Inverted Index: {:?}", inverted_index);
}

fn search_query() {
    println!("Enter keyword to search: ");
    let mut query: String = String::new();
    io::stdin()
        .read_line(&mut query)
        .expect("Failed to read the query");

    let query = query.trim().to_ascii_lowercase();

    let mut result: HashMap<String, u32> = HashMap::new();

    let inverted_index = INVERTED_INDEX.read().unwrap();

    if inverted_index.contains_key(&query) {
        let file_name_list = inverted_index.get(&query).unwrap();

        for file_name in file_name_list {
            if result.contains_key(file_name) {
                *result.get_mut(file_name).unwrap() += 1;
            } else {
                result.insert(file_name.to_string(), 1u32);
            }
        }
    }

    println!("Result : {:?}", result);
}
