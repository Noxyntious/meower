// this program was a mistake. what am i doing
use rand::Rng;
use std::env;
use std::process;

fn main() {
    let meows = vec!["meow", "mrrrow", "mrrrp", "nyaaa", "nya", "mew", "meowwww"];
    let args: Vec<_> = env::args().collect();
    if args.len() <= 1 {
        println!("meower help:\n\nusage: meower <amount>\nexample: meower 10");
        process::exit(0x1);
    }
    let amount = args[1].parse::<i32>().unwrap()-1;
    let mut i: i32 = 0;
    let mut prevnum = 9999;
    while i <= amount {
        let mut num = rand::thread_rng().gen_range(0..meows.len());
        if num == prevnum {
            if num > 3 {
                num = num-1
            } else if num < 3 {
                num = num+1
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
}
