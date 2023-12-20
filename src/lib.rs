use std::fs;
use std::error::Error;

pub struct MinigrepArgs {
    query : String,
    file_path : String
}

impl MinigrepArgs {
    pub fn build(args: &[String]) -> Result<MinigrepArgs, &'static str> {
        if (args.len() < 3 ){
            return Err("Not enough arguments");
        }

        let query = args[1].clone();
        let file_path = args[2].clone();

        Ok(MinigrepArgs { query, file_path })
    }
}

pub fn run(minigrepArgs: MinigrepArgs) -> Result<(), Box<dyn Error>>{
    let file_content = fs::read_to_string(minigrepArgs.file_path)?;
    println!("{:?}",file_content);
    Ok(())
}