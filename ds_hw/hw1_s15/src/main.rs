//http://www.cs.rpi.edu/academics/courses/spring15/csci1200/hw/01_image_processing/hw.pdf

extern crate clap;
use clap::{App, Arg, SubCommand,};

#[allow(unused_variables)]
fn main() {
    let matches = App::new("hw1_s15")
                    //necessary positional args:
                    .arg(Arg::with_name("input")
                         .help("file containing starting board")
                         .index(1)
                         .required(true)
                         )
                    .arg(Arg::with_name("output")
                         .help("where to write resulting board")
                         .index(2)
                         .required(true))
                    //would be nice to put these in a group (exactly one is necessary),
                    .subcommand(SubCommand::with_name("replace")
                                .arg(Arg::with_name("old").required(true))
                                .arg(Arg::with_name("new").required(true)))
                    .subcommand(SubCommand::with_name("dilation")
                                .arg(Arg::with_name("old").required(true)))
                    .subcommand(SubCommand::with_name("erosion")
                                .arg(Arg::with_name("old").required(true))
                                .arg(Arg::with_name("new").required(true)))
                    .subcommand(SubCommand::with_name("floodfill")
                                .arg(Arg::with_name("x").required(true))
                                .arg(Arg::with_name("y").required(true))
                                .arg(Arg::with_name("new").required(true)))
                    .get_matches();
    
    let input  = matches.value_of("input");
    let output = matches.value_of("output");
    
    if let Some(m) = matches.subcommand_matches("replace"){
        if let (Some(new), Some(old)) = (m.value_of("new"), m.value_of("old")){
            println!("new: '{}'; old: '{}'", new, old);
        }
        else { assert!(false); } //clap should prevent this from ever being triggered, right?
    } else if let Some(m) = matches.subcommand_matches("dilation"){
        if let Some(old) = m.value_of("old"){
        }
        else { assert!(false); }
    } else if let Some(m) = matches.subcommand_matches("erosion"){
        if let (Some(new), Some(old)) = (m.value_of("new"), m.value_of("old")){
        }
        else { assert!(false); }
    } else if let Some(m) = matches.subcommand_matches("floodfill"){
        if let (Some(new), Some(x), Some(y)) = 
            (m.value_of("new"), m.value_of("x"), m.value_of("y")){
        }
        else { assert!(false); }
    } else {
        println!("No command given :(");
        println!("Commands include `replace`, `dilation`, `erosion`, and `floodfill`");
        std::process::exit(1);  //exit code clap uses
    }

}
