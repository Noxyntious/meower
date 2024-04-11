// this program was a mistake. what am i doing
use rand::Rng;
use std::env;
use std::process::ExitCode;

fn main() -> ExitCode {
    let meows = ["meow", "mrrrow", "mrrrp", "nyaaa", "nya", "mew", "meowwww"];
    let amount = if let Some(amount) = env::args().skip(1).next() {
        amount.parse::<usize>().unwrap()
    } else {
        println!(
            "meower help:\n\
            \n\
            usage: meower <amount>\n\
            example: meower 10"
        );
        return ExitCode::FAILURE;
    };
    let mut prevnum: Option<usize> = None;
    for _ in 0..amount {
        let mut num = rand::thread_rng().gen_range(0..meows.len());
        if let Some(prevnum) = prevnum {
            if num == prevnum {
                num = if num > (meows.len() / 2) {
                    num - 1
                } else {
                    num + 1
                };
            }
            print!(" {}", meows[num]);
        } else {
            print!("{}", meows[num]);
        }
        prevnum = Some(num);
    }
    if rand::thread_rng().gen_bool(0.5) {
        print!(" :3");
    }
    return ExitCode::SUCCESS;
}
