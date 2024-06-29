use clap::Parser;
use std::io::{BufRead, BufReader, Cursor};
use murmur3::murmur3_32;
use std::collections::{HashMap, HashSet};
use std::collections::hash_map::Entry;
use std::fs::File;

#[derive(Parser)]
#[command(version, about, long_about = None)]
struct Args {
    /// Input file to read
    #[arg(short, long)]
    file: String,
    /// Number of queries to read
    #[arg(short, long, default_value_t = 0)]
    queries: usize,
}

#[derive(Debug)]
struct Statement {
    collisions: HashSet<String>,
}

impl Statement {
    fn new(query: String) -> Self {
        let mut collisions = HashSet::new();
        collisions.insert(query);
        Self {
            collisions,
        }
    }

    fn add_collision(&mut self, query: String) {
        self.collisions.insert(query);
    }

    fn get_collision(&self) -> &HashSet<String> {
        &self.collisions
    }

    fn has_collision(&self) -> bool {
        self.collisions.len() > 1
    }
}

fn new_reader(file: &str) -> BufReader<File> {
    BufReader::new(File::open(file).expect("Could not open file"))
}

fn main() {
    let args = Args::parse();
    let mut buf = Vec::new();
    let mut counter = 0;

    let mut map = HashMap::new();

    let reader = new_reader(&args.file);
    for line in reader.lines().flatten() {
        if line.contains(";") {
            buf.push(line);
            let mut cursor = Cursor::new(buf.join(" "));
            let key = murmur3_32(&mut cursor, 0).unwrap();
            match map.entry(key) {
                Entry::Occupied(mut entry) => {
                    let value: &mut Statement = entry.get_mut();
                    value.add_collision(buf.join(" "));
                }
                Entry::Vacant(entry) => {
                    entry.insert(Statement::new(buf.join(" ")));
                }
            }
            buf.clear();
            counter += 1;
        } else {
            buf.push(line);
        }
        if args.queries > 0 && counter == args.queries {
            break;
        }
    }

    let total = map.len();
    for (key, value) in map.drain() {
        if value.has_collision() {
            let collision = value.get_collision();
            println!("key: {key}");
            for (n, query) in collision.iter().enumerate() {
                println!("collision {n}: {query}");
                println!();
            }
            println!();
        }
    }

    println!("Total queries: {}", total);
}
