use clap::{command, Arg, Command};

fn main() {
    let match_result = command!().about("this cli tool lets you edit your Picture in Terminal :)")
    .subcommand(
        Command::new("blur")
        .arg(
            Arg::new("parameter")
            .long("para")
            .required(true)
            .help("allowed parameters -> 0.1 to 1.0")
        )
    )
    .subcommand(
        Command::new("rotate")
        .arg(
            Arg::new("parameter")

            .long("para")
            .required(true)
            .help("allowed parameters -> 90, 180, 270")
        )
    )
    .arg(
        Arg::new("path")
        .short('p')
        .long("path")
        .required(true)
        .help("enter the path")
    )
    .get_matches();

    // print!("want to edit {}", match_result.get_one::<String>("edit").unwrap_or(&"No edit options given!".to_string()));
   
    let editOP =  match_result.subcommand_name().unwrap().to_string();
        // .unwrap_or(&"No edit options given!".to_string());
    let path =  match_result.get_one::<String>("path");
        // .unwrap_or(&"path given!".to_string());
    let para =  match_result.subcommand_matches(match_result.subcommand_name().unwrap()).unwrap().get_one::<String>("parameter");
    // print!("para is {}", para.unwrap() );
    

    edit(editOP, para, path,);
}

fn edit(editOP : String, para : Option<&String>, path : Option<&String>, ){

    match editOP.as_str() {
        "blur" => {

            print!("editOP is {} and para is {}", editOP, para.unwrap())

        }

        "rotate" => {

            print!("2editOP is {} and para is {}", editOP, para.unwrap())


        }
        _ => {
            println!("Unknown operation: {}", editOP);
        }
    }

}