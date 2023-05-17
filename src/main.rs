use futures::future::join_all;
use rand::Rng;
use std::env;
use std::fs;

async fn create_random_code(length: usize) -> String {
    (0..length)
        .map(|_| rand::thread_rng().gen_range(1..=6).to_string())
        .collect()
}
fn pick_words_by_codes<'a, 'b>(
    codes: &'a Vec<String>,
    codes_and_words: &'b Vec<(String, String)>,
) -> Vec<&'b str> {
    codes
        .iter()
        .filter_map(|code| {
            codes_and_words
                .iter()
                .find(|(c, _)| c == code)
                .map(|(_, word)| word.as_str())
        })
        .collect()
}

#[tokio::main]
async fn main() {
    let args: Vec<String> = env::args().collect();
    let passphrase_length_arg = args.get(1);
    let passphrase_separator_arg = args.get(2);

    match passphrase_length_arg {
        Some(passphrase_length_arg) => {
            if !passphrase_length_arg.chars().all(char::is_numeric) {
                println!("Passphrase length must be a number");
                return;
            }
        }
        None => {
            println!("Passphrase length must be a number");
            return;
        }
    }
    match passphrase_separator_arg {
        Some(passphrase_separator_arg) => {
            if passphrase_separator_arg.len() != 1 {
                println!("Passphrase separator must be a single character");
                return;
            }
        }
        None => {
            println!("Passphrase separator must be a single character");
            return;
        }
    }

    // Load the file
    let mut path = env::current_dir().unwrap();
    path.push("src/lists/en.txt");
    let contents = fs::read_to_string(path).expect("Unable to read file");

    let passphrase_length = passphrase_length_arg.unwrap().parse::<i32>().unwrap();
    let codes: Vec<String> = join_all((0..passphrase_length).map(|_| create_random_code(5)))
        .await
        .into_iter()
        .collect();

    println!("You rolled: {}", codes.join(", "));

    let phrases_with_codes: Vec<(String, String)> = contents
        .lines()
        .map(|line| {
            let fields: Vec<&str> = line.split_whitespace().collect();
            (fields[0].to_string(), fields[1].to_string())
        })
        .collect();

    let phrases = pick_words_by_codes(&codes, &phrases_with_codes);

    let passphrase_separator = passphrase_separator_arg.unwrap();
    let passphrase = phrases.join(&passphrase_separator);

    println!("Passphrase: {}", passphrase)
}
