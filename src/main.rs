use rand;

use structopt;

mod default_word_lists;

use structopt::StructOpt;

#[derive(StructOpt, Debug)]
#[structopt()]
struct Args {
    /// Word list to use (one of [long, short, shortest])
    #[structopt(short = "w", long = "wordlist", default_value = "long")]
    wordlist: WordList,

    /// Be Verbose.
    #[structopt(short = "v", long = "verbose")]
    verbose: bool,

    /// Length of the passphrase
    #[structopt(short = "n", long = "length", default_value = "6")]
    length: usize,
}

use crate::default_word_lists::WordList;

fn main() {
    let args = Args::from_args();

    let (dices, word_list) = args.wordlist.get();

    if args.verbose {
        println!(
            "Generating passphrase of length {} using {} dices...\n",
            args.length, dices
        );
    }

    let (passphrase, rolls_history) = generate(args.length, dices, &word_list);

    let result = passphrase.join(" ");
    if args.verbose {
        println!("Rolls: => {:?}", rolls_history);
        println!("Passphrase => {}", result);
    } else {
        println!("{}", result);
    }
}

fn generate(
    target_length: usize,
    dices: usize,
    word_list: &[(u32, &'static str)],
) -> (Vec<&'static str>, Vec<u32>) {
    let mut os_rng =
        rand::os::OsRng::new().expect("Could not create secure random number generator");

    let mut passphrase = Vec::with_capacity(target_length);
    let mut rolls_history = Vec::with_capacity(target_length);

    for _ in 0..target_length {
        let rolls = roll_n_dices(dices, &mut os_rng);
        let key = key_from_dices(rolls);
        rolls_history.push(key);

        if let Ok(index) = word_list.binary_search_by_key(&key, |&(k, _)| k) {
            passphrase.push(word_list[index].1);
        } else {
            println!("Error! The provided word list has no value for {}", key);
            std::process::exit(-1);
        }
    }
    (passphrase, rolls_history)
}

fn roll_dice<R: rand::Rng>(rng: &mut R) -> u8 {
    use rand::distributions::{Distribution, Range};
    Range::new(1, 7).sample(rng)
}

fn roll_n_dices<R: rand::Rng>(dices: usize, rng: &mut R) -> Vec<u8> {
    (0..dices).map(|_| roll_dice(rng)).collect()
}

fn key_from_dices(roll: Vec<u8>) -> u32 {
    let mut multiplier = 10u32.pow(roll.len() as u32 - 1);
    let mut result = 0;

    for num in roll {
        result += u32::from(num) * multiplier;
        multiplier /= 10;
    }
    result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_key_from_dices() {
        assert_eq!(2354, key_from_dices(vec![2, 3, 5, 4]));
        assert_eq!(1111, key_from_dices(vec![1, 1, 1, 1]));
        assert_eq!(24, key_from_dices(vec![2, 4]));
        assert_eq!(23544564, key_from_dices(vec![2, 3, 5, 4, 4, 5, 6, 4]));
    }

    #[test]
    fn test_roll_n_dice() {
        let mut os_rng =
            rand::os::OsRng::new().expect("Could not create secure random number generator");

        for i in 1..10 {
            assert_eq!(i, roll_n_dices(i, &mut os_rng).len())
        }
    }
}
