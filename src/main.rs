mod Image_fns;

use std::{env, io::{stdin, stdout, Write}, path::Path, process::Command};
use clap::{command, Arg, Command as Command2};
use image::{DynamicImage, ImageReader};
use Image_fns::{edit_rotate, edit_blur};

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
            // "exit" => {break;}

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
    let para =  match_result.subcommand_matches(match_result.subcommand_name().unwrap()).unwrap().get_one::<String>("parameter").unwrap().as_str();

    let rt = tokio::runtime::Runtime::new().unwrap();
    rt.block_on(edit(editOP, para, path.as_str()));
}

async fn edit(editOP : String, para : &str, path : &str, ){
    match editOP.as_str() {
        "blur" => {
            // print!("editOP is {} and para is {} with path {}", editOP, para, path)
            match edit_blur(para, path).await {
                Ok(img_data) => {
                    println!("Blur operation completed successfully.");
                    let output_path = format!("{}_blured.jpg", path.trim_end_matches(".jpeg").trim_end_matches(".jpg"));
                    
                    // Save the image to disk
                    std::fs::write(&output_path, img_data).expect("Failed to write image file");
                    println!("Blured image saved to: {}", output_path);
                },
                Err(e)=>{
                    eprintln!("error processing image : {}", e)
                }
            }
        }

        "rotate" => {
            // print!("2editOP is {} and para is {} with path {}", editOP, para, path)
            match edit_rotate(para, path).await {
                Ok(img_data) => {
                    let output_path = format!("{}_rotated.jpg", path.trim_end_matches(".jpeg").trim_end_matches(".jpg"));
                    
                    // Save the image to disk
                    std::fs::write(&output_path, img_data).expect("Failed to write image file");
                    println!("Rotated image saved to: {}", output_path);
                },
                Err(e) => {
                    eprintln!("Error processing image: {}", e);
                }
            }
        }
        _ => {
            println!("Unknown operation: {}", editOP);
        }
    }

}