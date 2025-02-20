// this program was a mistake. what am i doing
use rand::Rng;

use clap::Parser;

#[derive(Parser)]
#[command(version, about, arg_required_else_help(true))]
struct Cli {
    count: i32,
}

fn main() {
    let meows = vec![
        "meow",
        "maow",
        "mrrrow",
        "mrrrp",
        "nyaaa",
        "nya",
        "mew",
        "meowwww",
        "prrp",
        "nyaaaaaaaaaa",
    ];
    let args = Cli::parse();

    let amount = args.count;
    let mut available_indices: Vec<usize> = (0..meows.len()).collect();
    let mut rng = rand::thread_rng();

    for _ in 0..amount {
        if available_indices.is_empty() {
            available_indices = (0..meows.len()).collect();
        }

        let index = rng.gen_range(0..available_indices.len());
        let meow_index = available_indices.remove(index);
        print!("{} ", meows[meow_index]);
    }

    if rng.gen_range(0..10) >= 5 {
        print!(":3");
    }
    println!();
}
