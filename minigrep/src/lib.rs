use std::{fs,error::Error,env};
/*Experiment with Lifetimes- Working */
// fn parse_config<'a>(args: &'a Vec<String>)->(&'a str,&'a str){
//     // let directory= &args[0];
//     let query =args[1].as_str();
//     let filename = args[2].as_str();
//     (query,filename)
// }
pub struct Commandline{
    pub query : String,
    pub filename : String,
    pub case_sensitive : bool,
}

impl Commandline {
    pub fn new( args: &[String]) -> Result<Commandline, &'static str>{
        if args.len()<3{
            return Err("not enought arguments")
        }
        let query = args[1].clone();
        let filename = args[2].clone();
        let case_sensitive = env::var("CASE_INSENSITIVE").is_err();
        Ok(Commandline { query, filename, case_sensitive })

    }
}
/*Works like this but doesn't print correctly the reislt  */
// fn run(cmd: Commandline)->Result<String, Box<dyn Error>>{
//     let content =fs::read_to_string(cmd.filename)?;

/*
    Ok(content)
}
*/
pub fn run(cmd: Commandline)->Result<(), Box<dyn Error>>{
    let content =fs::read_to_string(cmd.filename)?;
    if cmd.case_sensitive == true{
        for line in search_case_sensitive(&cmd.query, &content){
            println!("{}",line)
        }
    }
    else {
        for line in search_case_insensitive(&cmd.query, &content){
            println!("{}",line)
        }
    }

    Ok(())
}
pub fn search_case_sensitive<'a>(query:&str,content:&'a str)->Vec<&'a str>{
   // Lifetime 'a specify that the argument "content" is connected to the
   // lifetime of the return value.
    let mut result= Vec::new();
    for lines in content.lines(){
        if lines.contains(query){
            result.push(lines)
        }
    }
    result
}

pub fn search_case_insensitive<'a>(query:&str,content:&'a str)->Vec<&'a str>{
    // Lifetime 'a specify that the argument "content" is connected to the
   // lifetime of the return value.
    let query=query.to_lowercase();
    let mut result= Vec::new();
    for lines in content.lines(){
        if lines.to_lowercase().contains(&query){
            result.push(lines)
        }
    }
    result
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    #[ignore]
    fn one_result(){
        let query = "duct";
        let content = "\
Rust:
safe, fast, productive.
Pick three.";
        assert_eq!(vec!["safe, fast, productive."], search_case_sensitive(query,content));
    }
    #[test]
    fn case_sensitive(){
        let query = "duct";
        let content = "\
Rust:
safe, fast, productive.
Pick three.
Duct tape.";
        assert_eq!(vec!["safe, fast, productive."], search_case_sensitive(query,content));

    }

    #[test]
    fn case_insensitive(){
        let query = "rUsT";
        let content = "\
Rust:
safe, fast, productive.
Pick three.
Trust me.";
    assert_eq!(
        vec!["Rust:","Trust me."],
        search_case_insensitive(query, content)
    );
    }

}