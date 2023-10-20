use clap::Parser;
use std::fs::File;
use std::io::Read;
use unsvg::Image;
mod parser;

/// A simple program to parse four arguments using clap.
#[derive(Parser)]
struct Args {
    /// Path to a file
    file_path: std::path::PathBuf,

    /// Path to an svg or png image
    image_path: std::path::PathBuf,

    /// Height
    height: u32,

    /// Width
    width: u32,
}

#[allow(unused)]
fn parse_file(filepath: &std::path::PathBuf) -> Vec<String> {
    let mut logo_file = match File::open(&filepath) {
        Ok(f) => f,
        Err(_) => std::process::exit(1),
    };

    let mut s = String::new();
    match logo_file.read_to_string(&mut s) {
        Ok(_) => {
            return s.lines().map(|x| String::from(x)).collect();
        }
        Err(_) => {
            std::process::exit(1);
        }
    };
}

fn main() -> Result<(), ()> {
    let args: Args = Args::parse();

    // Access the parsed arguments
    let file_path = args.file_path;
    let image_path = args.image_path;
    let height = args.height;
    let width = args.width;

    let mut image = Image::new(width, height);
    let file_to_vec = parse_file(&file_path);

    let mut turtle_6991 =
        parser::turtle::Turtle::new((width / 2) as f32, (height / 2) as f32, 7, 0);
    let finish_draw = parser::turtle_move(
        &file_to_vec,
        &mut turtle_6991,
        &mut image,
        0,
        file_to_vec.len(),
    );

    match finish_draw {
        Ok(_) => {}
        Err(_) => return Err(()),
    };

    match image_path.extension().map(|s| s.to_str()).flatten() {
        Some("svg") => {
            let res = image.save_svg(&image_path);
            if let Err(e) = res {
                eprintln!("Error saving svg: {e}");
                return Err(());
            }
        }
        Some("png") => {
            let res = image.save_png(&image_path);
            if let Err(e) = res {
                eprintln!("Error saving png: {e}");
                return Err(());
            }
        }
        _ => {
            eprintln!("File extension not supported");
            return Err(());
        }
    }

    Ok(())
}
