use std::{env, io::{stdin, stdout, Write}, path::Path, process::Command};

use clap::{command, Arg, Command as Command2};

fn main() {

    let match_result = command!().about("this cli tool lets you edit your Picture in Terminal :)")
    .subcommand(
        Command2::new("blur")
        .arg(
            Arg::new("parameter")
            .long("para")
            .required(true)
            .help("allowed parameters -> 0.1 to 1.0")
        )
    )
    .subcommand(
        Command2::new("rotate")
        .arg(
            Arg::new("parameter")

            .long("para")
            .required(true)
            .help("allowed parameters -> 90, 180, 270")
        )
    )
    // .arg(
    //     Arg::new("path")
    //     .short('p')
    //     .long("path")
    //     .required(true)
    //     .help("enter the path")
    // )
    .get_matches();


    println!("=> Now Select image (use `ls`, `cd <dir>`, and `select <image>`):");
    let mut path;
    loop {
        print!("--> ");
        stdout().flush();

        let mut input = String::new();
        stdin().read_line(&mut input).unwrap();
    
        // print!("{}", command);
        let mut parts = input.trim().split_whitespace();
        let command = parts.next().unwrap();
        let args = parts.clone();

        match command {
            "cd" =>{
                let new_dir = args.peekable().peek().map_or("/", |x| *x);
                let dir = Path::new(new_dir);
                if let Err(e) = env::set_current_dir(&dir){
                    eprintln!("{}", e);
                }
            },

            "select" => {
                if let Some(file_name) = parts.next() {
                    let full_path = env::current_dir().unwrap().join(file_name);
                    if full_path.exists() && full_path.is_file() {
                        path = full_path.to_string_lossy().to_string();
                        break;
                    } else {
                        eprintln!("File not found: {}", file_name);
                    }
                } else {
                    eprintln!("Usage: select <filename>");
                }
            }

            command =>{
                let child =Command::new(command)
                    .args(args)
                    .spawn();

                match child {
                    Ok(mut child) => { child.wait(); },
                    Err(e) => eprintln!("Failed to execute command: {}", e),
                }

            }
        }
    }


    // print!("want to edit {}", match_result.get_one::<String>("edit").unwrap_or(&"No edit options given!".to_string()));
   
    let editOP =  match_result.subcommand_name().unwrap().to_string();
        // .unwrap_or(&"No edit options given!".to_string());
    // let path =  match_result.get_one::<String>("path");
        // .unwrap_or(&"path given!".to_string());
    let para =  match_result.subcommand_matches(match_result.subcommand_name().unwrap()).unwrap().get_one::<String>("parameter");

    edit(editOP, para, path,);
}

fn edit(editOP : String, para : Option<&String>, path : String, ){

    match editOP.as_str() {
        "blur" => {

            print!("editOP is {} and para is {} with path {}", editOP, para.unwrap(), path)

        }

        "rotate" => {

            print!("2editOP is {} and para is {} with path {}", editOP, para.unwrap(), path)


        }
        _ => {
            println!("Unknown operation: {}", editOP);
        }
    }

}