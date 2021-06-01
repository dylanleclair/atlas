use std::collections::HashMap; // used to parse command line args
use std::env; // used to parse command line args
use image::io::Reader as ImageReader; // imported to read images while binding them to atlas
use image::{GenericImage}; // imported to support in textures

use std::path::Path;
use std::fs;

const DEFAULT_WIDTH : &str = "32";
const DEFAULT_HEIGHT : &str = "32";
const DEFAULT_COLS : &str = "0";

fn main() {

    let args : Vec<String> = env::args().collect();

    // iterate over, mapping pairs

    let parsed_args = parse_cmd_line(&args);

    // given target directory of files to stitch together into a texture
    // given width & height of the atlas
    // given width & height of each tile

    let w = parsed_args.get("-w").unwrap_or(&DEFAULT_WIDTH); // specifies width of tile
    let h = parsed_args.get("-h").unwrap_or(&DEFAULT_HEIGHT); // specifies height of tile
    let c = parsed_args.get("-c").unwrap_or(&DEFAULT_COLS); // specifies the desired # columns in atlas

    let width = w.parse::<u32>().unwrap();
    let height = h.parse::<u32>().unwrap();

    let dirname = parsed_args.get("-dir").unwrap(); // specifies the input directory

    // crawl through the directory, adding each file to the next calculated area
    let path = Path::new(dirname); // directory as specified
    let file_count = get_file_count(&path); // counts the files in the specified directory

    let mut cols = c.parse::<u32>().unwrap();

    if cols == 0 {
        cols = (file_count as f32).sqrt().ceil() as u32;
    }

    // first, calculate the expected number of rows depending on size of dir 
    let rows = ((file_count as f32) / (cols as f32).ceil()) as u32; // the number of rows needed to fit all images in

    let mut images_placed = 0;

    let mut atlas = image::ImageBuffer::new(width * cols, height * rows);

    // step through the files in the directory
    for entry in fs::read_dir(path).unwrap() {
        // ignore directories, process images
        let p = entry.unwrap().path();
        if !p.is_dir() { // if it's a file, we want to merge it into the atlas
            // perform the insertion
            let img = ImageReader::open(&p).unwrap().decode().unwrap();
            
            let x = (images_placed % cols) * width;
            let y = (images_placed / cols) * height;
            
            // place sub image into atlas
            atlas.copy_from(&img,x, y).unwrap();
            images_placed+=1;
        }
    }

    atlas.save("atlas.png").unwrap();

}


/// Parses the command line arguments into a HashMap
fn parse_cmd_line(args: &Vec<String>) -> HashMap<String,&str> {

    let mut parsed_args = HashMap::new();

    if (args.len() % 2) != 1 {
        panic!("Invalid input! Key without a pair identified.");
    }

    for i in (1..args.len()-1).step_by(2) {
        parsed_args.insert(args[i].clone(), args[i+1].as_ref()); // insert into map
    }

    parsed_args

}

fn get_file_count(path: &Path) -> i32 {
    let mut file_count = 0;
    // step through the files in the directory
    for entry in fs::read_dir(path).unwrap() {
        // ignore directories, process images
        let p = entry.unwrap().path();
        if !p.is_dir() { // if it's a file, we want to merge it into the atlas
            // perform the insertion
            file_count +=1;
        }
    }
    file_count

}
