use std::env;
use std::process;
use minigrep::MinigrepArgs;

fn main() {
    println!("-------- minigrep --------");
    // let args: Vec<String> = env::args().collect(); //cloning this value inside build VS sending iterator
    // dbg!(args);

    let minigrep_args = MinigrepArgs::build(env::args()).unwrap_or_else(|_err| {
        println!("{_err}");
        process::exit(1);
    });

    if let Err(_err) = minigrep::run(minigrep_args){
        println!("{_err}");
        process::exit(1);
    };
    
}



