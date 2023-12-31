use std::fs;
use std::env;
use std::error::Error;

pub struct MinigrepArgs {
    query : String,
    file_path : String,
    ignore_case: bool,
}

impl MinigrepArgs {
    pub fn build(mut args: impl Iterator<Item = String>) -> Result<MinigrepArgs, &'static str> {
        args.next(); //name of the program

        let query = match args.next() {
            Some(arg) => arg,
            None => return Err("no query string")
        };

        let file_path = match args.next() {
            Some(arg) => arg,
            None => return Err("no file path")
        };
        

        let ignore_case = env::var("IGNORE_CASE").is_ok();
        Ok(MinigrepArgs { query, file_path, ignore_case })
    }
}

pub fn run(command_line_args: MinigrepArgs) -> Result<(), Box<dyn Error>>{
    let file_content = fs::read_to_string(command_line_args.file_path)?;

    let lines = if command_line_args.ignore_case { 
        search_case_insensitive(&command_line_args.query, &file_content)
    } else {
        search_case_insensitive(&command_line_args.query, &file_content)
    };
    
    for line in lines {
        println!("{line}");
    } 
    Ok(())
}

pub fn search_case_sensitive<'a>(text_to_search: &str, contents: &'a str) -> Vec<&'a str> {
    let mut filtered_contents = Vec::new();

    for line in contents.lines(){
        if line.contains(text_to_search){
            filtered_contents.push(line);
        }
    }
    filtered_contents
}

pub fn search_case_insensitive<'a>(text_to_search: &str, contents: &'a str) -> Vec<&'a str> {
    let lowercase_query = text_to_search.to_lowercase(); //String
    contents.lines().filter(|line| line.contains(&lowercase_query)).collect()
}

//test driven development TDD!
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_search_case_insensitive(){
        let text_to_search = "IN";
        let contents = "/
facciamo un viaggio
in mezzo al mar
facciamo un viaggio
in fondo al mar";
        assert_eq!(vec!["in mezzo al mar","in fondo al mar"],search_case_insensitive(text_to_search, contents));
    }

    #[test]
    fn test_search_case_sensitive(){
        let text_to_search = "in";
        let contents = "/
facciamo un viaggio
in mezzo al mar
facciamo un viaggio
in fondo al mar";
        assert_eq!(vec!["in mezzo al mar","in fondo al mar"],search_case_sensitive(text_to_search, contents));
    }
}