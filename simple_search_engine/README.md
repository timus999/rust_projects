✅ Core Idea of the Project

You’re building a local search engine that can scan a folder full of .txt files, analyze their text, and allow the user to search for words or phrases.
It’s basically a simplified version of Google, but for local text files.

🎯 Goal Features (Detailed)

Below are the core features your search engine should have.
Then I’ll show bonus advanced features to challenge yourself further.

🚀 1. Directory Scanning

Your search engine should:

✔ Load All Text Files

User provides a folder path

Program uses std::fs::read_dir() to iterate

Only loads .txt files

Ignores binary files, hidden files, other formats

✔ Store Metadata

For each file:

file name

full path

file size

number of words

This can be stored in a struct:

```
struct Document {
    name: String,
    path: String,
    content: String,
}
```

🚀 2. Tokenization

Tokenization = breaking text into words.

Your program should:

✔ Normalize text before processing

Convert to lowercase

Remove punctuation (, . ! ? ( ) : ; etc)

Split by whitespace

Example input:

Hello, world! This is Rust.


Tokenized result:

```
["hello", "world", "this", "is", "rust"]
```

This teaches you:

string manipulation

iterators

text processing

🚀 3. Build an Inverted Index

This is the heart of your search engine.

An inverted index is a mapping:

word → list of files containing that word


Use a HashMap<String, Vec<String>>

Example:
```
{
    "rust" -> ["file1.txt", "file3.txt"],
    "hello" -> ["file1.txt"],
    "world" -> ["file1.txt", "file2.txt"]
}

```
This makes searching fast.

🚀 4. Search System

User types:

search rust programming


Your program should:

✔ Split the query into words
✔ Find all files containing all query words
✔ Rank results (simple scoring)

Example rules:

File with more matches appears higher

Smaller files may score higher

Exact word matches ranked higher

Output:

```
Found 2 matching files:
1. notes/rust_intro.txt   (score: 7)
2. docs/programming.txt   (score: 3)

```
🚀 5. Interactive CLI Interface

Your search engine should have a small shell:

search-engine> load ./documents
Indexed 14 files in 0.03s

search-engine> search rust
results...

search-engine> stats
search-engine> exit


Commands to support:

✔ load <directory>

Load and index files

✔ search <text>

Return matching files

✔ stats

Show:

number of files

number of words indexed

top 10 most common words

✔ help

List commands

✔ exit

Quit program
