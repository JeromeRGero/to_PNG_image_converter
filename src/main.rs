use image_convert::{ImageResource, InterlaceType, identify_read, PNGConfig, to_png};
use std::path::Path;
use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() > 1 {
        // println!("The following arguments were passed:");
        // for arg in &args[1..] {
        //     println!("Argument: [{}]", arg);
        // }
        // Get input file from filepath

        let source_image_path = Path::new(&args[1]);

        let target_image_path = Path::join(
            source_image_path.parent().unwrap(), 
            format!("{}_output.png", source_image_path.file_stem().unwrap().to_str().unwrap()));

        println!("---\nThe path to the image you have requested to convert is: \n{:?}", source_image_path);
        println!("---\nThe new file will be called: \n{:?}", format!("{}_output.png", source_image_path.file_stem().unwrap().to_str().unwrap()));


        let input = ImageResource::from_path(source_image_path);

        let mut id_output = None;

        let id = identify_read(&mut id_output, &input).unwrap();

        let current_format = id.format;
        
        println!("---\nThe current format of the image you have requested to convert is: \n{:?}", current_format);

        let png_config = PNGConfig::new();

        let mut output = ImageResource::from_path(target_image_path.clone());

        to_png(&mut output, &input, &png_config).unwrap();

        if output.into_path_buf().unwrap().exists() {
            println!("---\nSUCCESS: The image has been converted to a png and saved to the following path: \n{:?}", target_image_path);
        } else {
            println!("---\nFAILED: The image has not been converted to a png and saved to the following path: \n{:?}", target_image_path);
        }
        
    } else {
        println!("--- FAILED: You must supply the path to the image file to convert to a PNG.");
    }
}
