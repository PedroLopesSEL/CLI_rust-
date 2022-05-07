use std::{env,fs,process,error::Error};
fn main() {

    let args = env::args().collect::<Vec<String>>();
    /* Experiment with Matching- Working */
    // for (i,s) in args.iter().enumerate() {
    //     match i {
    //         0 =>println!("Program name {}",s),
    //         1 =>println!("Searching for {}",s),
    //         2 =>println!("In file {}",s),
    //         _ =>panic!("Out of range")
    //     };
    // }
    let config =
    Commandline::new(&args).unwrap_or_else(|err|{
        println!("Problem parsing arguments: {}", err);
        process::exit(1);
    });
    // println!("Program name {}",directory);
    println!("Searching for {}",config.query);
    println!("In file {}",config.filename);
    if let Err(a) = run(config){
        print!("Application error \n{}",a);
        process::exit(1);
    }
}
/*Experiment with Lifetimes- Working */
// fn parse_config<'a>(args: &'a Vec<String>)->(&'a str,&'a str){
//     // let directory= &args[0];
//     let query =args[1].as_str();
//     let filename = args[2].as_str();
//     (query,filename)
// }
struct Commandline{
    query : String,
    filename : String,
}

impl Commandline {
    fn new( args: &[String]) -> Result<Commandline, &'static str>{
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

//     Ok(content)
// }
fn run(cmd: Commandline)->Result<(), Box<dyn Error>>{
    let content =fs::read_to_string(cmd.filename)?;
    println!("With text:\n{}", content);
    Ok(())
}