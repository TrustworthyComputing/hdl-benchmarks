use clap::{Arg, Command, ArgAction};
use std::fs::File;
use std::io::prelude::*;
use rand::Rng;
use image::{DynamicImage, GenericImageView, GrayImage};
use csv::ReaderBuilder;

fn compress_pixel_data(pixel_data: u8) -> u8 {
    (pixel_data as u16 * 15 / 255) as u8
}

fn generate_input_wires(height: usize, width: usize, compress: bool) -> String {
    let mut inputs = String::from("wire, value\n");
    let mut rng = rand::thread_rng();

    for r in 0..height {
        for c in 0..width {
            let mut value: u8 = rng.gen_range(0..=255);
            if compress {
                value = compress_pixel_data(value);
            }
            inputs.push_str(&format!("p_{}_{}, {}\n", r, c, value));
        }
    }

    inputs
}

fn generate_input_wires_from_image(image: &DynamicImage, padding: u8, compress: bool) -> (String, usize, usize) {
    let (width, height) = image.dimensions();
    let mut inputs = String::from("wire, value\n");

    for r in 0..height {
        for c in 0..width {
            let mut pixel_value = image.get_pixel(c, r)[0]; // Get the red channel (assuming grayscale)
            if compress {
                pixel_value = compress_pixel_data(pixel_value);
            }
            inputs.push_str(&format!("p_{}_{}, {}\n", r+1, c+1, pixel_value));
        }
    }

    // Add padding
    for r in 0..(height+1) {
        inputs.push_str(&format!("p_{}_0, {}\n", r, padding));
        inputs.push_str(&format!("p_{}_{}, {}\n", r, width+1, padding));
    }
    for c in 0..(width+1) {
        inputs.push_str(&format!("p_0_{}, {}\n", c+1, padding));
        inputs.push_str(&format!("p_{}_{}, {}\n", height+1, c+1, padding));
    }
    inputs.push_str(&format!("p_{}_{}, {}\n", height+1, 0, padding)); // extra corner case

    (inputs, (height + 2) as usize, (width + 2) as usize)
}

fn generate_convolution_module(height: usize, width: usize) -> String {
    let mut module_code = String::from("module image_blur (");
    for r in 0..height {
        for c in 0..width {
            module_code.push_str(&format!("p_{}_{}, ", r, c));
        }
    }
    for r in 1..height-1 {
        for c in 1..width-1 {
            module_code.push_str(&format!("out_{}_{}, ", r, c));
        }
    }
    module_code.truncate(module_code.len() - 2); // remove last space and comma
    module_code.push_str(");\n\n");

    for r in 0..height {
        for c in 0..width {
            module_code.push_str(&format!("  input [7:0] p_{}_{};\n", r, c));
        }
    }
    module_code.push_str("\n");
    for r in 1..height-1 {
        for c in 1..width-1 {
            module_code.push_str(&format!("  output [7:0] out_{}_{};\n", r, c));
        }
    }
    module_code.push_str("\n");

    for r in 0..height {
        for c in 0..width {
            module_code.push_str(&format!("  wire [7:0] t_r{}_c{}_0;\n", r, c));
            module_code.push_str(&format!("  wire [7:0] t_r{}_c{}_1;\n", r, c));
            module_code.push_str(&format!("  wire [7:0] t_r{}_c{}_2;\n", r, c));
            module_code.push_str(&format!("  wire [7:0] t_r{}_c{}_3;\n", r, c));
            module_code.push_str(&format!("  wire [7:0] t_r{}_c{}_4;\n", r, c));
            module_code.push_str(&format!("  wire [7:0] t_r{}_c{}_5;\n", r, c));
            module_code.push_str(&format!("  wire [7:0] t_r{}_c{}_6;\n", r, c));
            module_code.push_str(&format!("  wire [7:0] t_r{}_c{}_7;\n", r, c));
        }
    }
    module_code.push_str("\n");
    
    for r in 1..height - 1 {
        for c in 1..width - 1 {
            module_code.push_str(&format!(
                "  assign t_r{}_c{}_0 = p_{}_{} + p_{}_{};\n",
                r, c,
                r - 1, c, // N
                r, c - 1  // W
            ));
            module_code.push_str(&format!(
                "  assign t_r{}_c{}_1 = p_{}_{} + p_{}_{};\n",
                r, c,
                r + 1, c, // S
                r, c + 1  // E
            ));
            module_code.push_str(&format!(
                "  assign t_r{}_c{}_2 = p_{}_{} + p_{}_{};\n",
                r, c,
                r - 1, c - 1, // NW
                r - 1, c + 1  // NE
            ));
            module_code.push_str(&format!(
                "  assign t_r{}_c{}_3 = p_{}_{} + p_{}_{};\n",
                r, c,
                r + 1, c - 1, // SW
                r + 1, c + 1  // SE
            ));

            module_code.push_str(&format!(
                "  assign t_r{}_c{}_4 = t_r{}_c{}_0 + p_{}_{};\n",
                r, c,
                r, c, // N + W
                r, c  // C
            ));

            module_code.push_str(&format!(
                "  assign t_r{}_c{}_5 = t_r{}_c{}_2 + t_r{}_c{}_3;\n",
                r, c,
                r, c, // NW + NE
                r, c  // SW + SE
            ));
            module_code.push_str(&format!(
                "  assign t_r{}_c{}_6 = t_r{}_c{}_1 + t_r{}_c{}_4;\n",
                r, c,
                r, c, // S + E
                r, c  // (N + W) + C
            ));
            module_code.push_str(&format!(
                "  assign t_r{}_c{}_7 = t_r{}_c{}_5 + t_r{}_c{}_6;\n",
                r, c,
                r, c, // (NW + NE) + (SW + SE)
                r, c  // (S + E) + (N + W) + C
            ));

            module_code.push_str(&format!(
                "  assign out_{}_{} = t_r{}_c{}_7 / 9;\n\n",
                r, c,
                r, c
            ));
        }
    }

    module_code.push_str("endmodule\n");
    module_code
}

fn main() {
    let matches = Command::new("Generate Blur Filter")
        .about("Preprocess Yosys outputs to cleaner structural Verilog")
        .arg(
            Arg::new("height")
                .long("height")
                .value_name("NUMBER")
                .help("Number of height of the input image (including padding)")
                .required_unless_present("image")
                .conflicts_with("image")
                .value_parser(clap::value_parser!(usize)),
        )
        .arg(
            Arg::new("width")
                .long("width")
                .value_name("NUMBER")
                .help("Number of width for the input image (including padding)")
                .required_unless_present("image")
                .conflicts_with("image")
                .value_parser(clap::value_parser!(usize)),
        )
        .arg(
            Arg::new("image")
                .long("image")
                .value_name("STRING")
                .help("Path to image")
                .required_unless_present_any(&["height", "width"])
                .value_parser(clap::value_parser!(String)),
        )
        .arg(
            Arg::new("compress")
                .short('c')
                .long("compress")
                .help("Compress pixel data to 5 bits of precision")
                .action(ArgAction::SetTrue),
        )
        .get_matches();

    let compress = matches.get_flag("compress");
    let (verilog_code, height, width, inputs) = {
        if let Some(image_path) = matches.get_one::<String>("image") {
            println!("Image path: {}", image_path);

            let image = image::open(image_path).expect("Failed to open image");
            let (inputs, height, width) = generate_input_wires_from_image(&image, 0, compress);
            let verilog_code = generate_convolution_module(height, width);

            (verilog_code, height, width, inputs)
        } else {
            println!("Using random pixels");

            let height = matches.get_one::<usize>("height").unwrap();
            let width = matches.get_one::<usize>("width").unwrap();
            let inputs = generate_input_wires(*height, *width, compress);
            let verilog_code = generate_convolution_module(*height, *width);

            (verilog_code, *height, *width, inputs)
        }
    };
    println!("height: {}, width: {}", height, width);

    let filename = "r".to_owned() + &height.to_string() + "_c" + &width.to_string() + "-blur";
    let verilog_filename = "./designs/".to_owned() + filename.as_str() + ".v";
    let inputs_filename = "./test-cases/".to_owned() + filename.as_str() + ".inputs.csv";

    let file = File::create(&verilog_filename);
    let file_inputs = File::create(&inputs_filename);

    match file {
        Ok(mut file) => {
            if let Err(err) = file.write_all(verilog_code.as_bytes()) {
                eprintln!("Failed to write to file: {}", err);
            } else {
                println!("File generated at {}", verilog_filename);
            }
        }
        Err(err) => {
            eprintln!("Failed to create file: {}", err);
        }
    }

    match file_inputs {
        Ok(mut file_inputs) => {
            if let Err(err) = file_inputs.write_all(inputs.as_bytes()) {
                eprintln!("Failed to write to inputs: {}", err);
            } else {
                println!("File generated at {}", inputs_filename);
            }
        }
        Err(err) => {
            eprintln!("Failed to create inputs: {}", err);
        }
    }
}
