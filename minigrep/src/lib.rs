use std::{fs,error::Error};
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
}

impl Commandline {
    pub fn new( args: &[String]) -> Result<Commandline, &'static str>{
        if args.len()<3{
            Err("not enought arguments")
        }
        else{
        Ok(Commandline { query: args[1].clone(), filename : args[2].clone()})
        }
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

    for line in search(&cmd.query, &content){
        println!("{}",line)
    }

    Ok(())
}
pub fn search<'a>(query:&str,content:&'a str)->Vec<&'a str>{
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

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn one_result(){
        let query = "duct";
        let content = "\
Rust:
safe, fast, productive.
Pick three.";
        assert_eq!(vec!["safe, fast, productive."], search(query,content));
    }

}