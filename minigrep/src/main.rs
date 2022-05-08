use std::env;
use std::process;
use minigrep::Commandline;
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
        eprintln!("Problem parsing arguments: {}", err);
        process::exit(1);
    });
    // println!("Program name {}",directory);
    println!("Searching for {}",config.query);
    println!("In file {}",config.filename);
    if let Err(a) = minigrep::run(config){
        eprint!("Application error \n{}",a);
        process::exit(1);
    }
}
