use std::io::{BufRead, BufReader};
use std::fs::File;
use std::option::Option;

extern crate rand;
use rand::{thread_rng, Rng};

pub fn start() {
    let words_vec = match read_words_file() {
        Some(vec) => vec,
        None => {
            println!("Failed to read word data file");
            return
        }
    };

    for word in words_vec.iter() {
        println!("{}", word);
    }
}

fn read_words_file() -> Option<Vec<String>> {
    let mut words_vec: Vec<String> = Vec::new();

    let file = match File::open("word_dict.txt") {
        Ok(f) => f,
        Err(e) => {
            println!("Failed to open file: {:#?}", e);
            return None;
        }
    };
    let file = BufReader::new(file);

    for line in file.lines() {
        let line = line.unwrap();

        if line.trim().len() == 0 {
            continue;
        }
        words_vec.push(line);
    }

    Some(words_vec)
}

fn shuffle_words_vec_idx_map(words_vec_idx_map: &mut Vec<usize>) {
    assert_eq!(words_vec_idx_map.len(), words_vec_idx_map.capacity());

    for i in (0..words_vec_idx_map.len()).rev() {
        let mut random_index = generate_random_num(words_vec_idx_map.len());

        if random_index == i {
            random_index += 1 % words_vec_idx_map.len();
        }

        let last_temp = words_vec_idx_map[i];
        words_vec_idx_map[i] = words_vec_idx_map[random_index];
        words_vec_idx_map[random_index] = last_temp;
    }
}

fn generate_random_num(max_range: usize) -> usize {
    thread_rng().gen_range(0..max_range)
}
