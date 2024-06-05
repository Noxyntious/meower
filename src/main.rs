// this program was a mistake. what am i doing
use rand::Rng;

use clap::Parser;

#[derive(Parser)]
#[command(version, about, arg_required_else_help(true))]
struct Cli {
    count: i32,
}

fn main() {
    let meows = vec!["meow", "mrrrow", "mrrrp", "nyaaa", "nya", "mew", "meowwww", "nyaaaaaaaaaa"];
    let args = Cli::parse();

    let amount = args.count;
    let mut i: i32 = 0;
    let mut prevnum = 9999;
    while i < amount {
        let mut num = rand::thread_rng().gen_range(0..meows.len());
        if num == prevnum {
            if num > 3 {
                num = num - 1
            } else if num < 3 {
                num = num + 1
            }
        }
        print!("{} ", meows[num]);
        prevnum = num;
        i = i + 1;
    }
    let colonthree = rand::thread_rng().gen_range(0..10);
    if colonthree >= 5 {
        print!(":3");
    }
    print!("\n");
}
