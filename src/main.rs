use std::env;
use std::process;
use minigrep::MinigrepArgs;

fn main() {
    println!("-------- minigrep --------");
    let args: Vec<String> = env::args().collect();
    // dbg!(args);

    let minigrepArgs = MinigrepArgs::build(&args).unwrap_or_else(|err| {
        println!("{err}");
        process::exit(1);
    });

    if let Err(err) = minigrep::run(minigrepArgs){
        println!("{err}");
        process::exit(1);
    };
    
}



