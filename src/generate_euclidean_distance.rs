use clap::{Arg, Command};
use rand::Rng;
use std::fs::File;
use std::io::prelude::*;

fn generate_input_vectors(size: usize) -> String {
    let mut inputs = String::from("wire, value\n");
    let mut rng = rand::thread_rng();

    for v in 0..size {
        let mut value: u8 = rng.gen();
        inputs.push_str(&format!("u_{}, {}\n", v, value));
        value = rng.gen();
        inputs.push_str(&format!("v_{}, {}\n", v, value));
    }

    inputs
}

fn generate_distance_module(size: usize) -> String {
    let mut module_code = String::from("module euclidean_distance (");
    for v in 0..size {
        module_code.push_str(&format!("u_{}, ", v));
        module_code.push_str(&format!("v_{}, ", v));
    }
    module_code.push_str("dist, ");
    module_code.truncate(module_code.len() - 2); // remove last space and comma
    module_code.push_str(");\n\n");

    for v in 0..size {
        module_code.push_str(&format!("  input [15:0] u_{};\n", v));
        module_code.push_str(&format!("  input [15:0] v_{};\n", v));
    }
    module_code.push('\n');
    module_code.push_str("  output [15:0] dist;\n");
    module_code.push('\n');

    for v in 0..size {
        module_code.push_str(&format!("  wire [15:0] t_diff_v{};\n", v));
        module_code.push_str(&format!("  wire [15:0] t0_sum_v{};\n", v));
    }
    let log_size = (f64::log2(size as f64) as usize) + 1;
    let mut add_size = size;
    for depth in 0..log_size {
        let mut i = 0;
        for _ in (0..add_size - 1).step_by(2) {
            module_code.push_str(&format!("  wire [15:0] t{}_sum_v{};\n", depth + 1, i,));
            i += 1;
        }
        // if the dimension is odd
        if add_size % 2 == 1 {
            module_code.push_str(&format!("  wire [15:0] t{}_sum_v{};\n", depth + 1, i,));
        }
        add_size = (add_size as f32 / 2.0).ceil() as usize;
    }

    module_code.push('\n');

    for v in 0..size {
        module_code.push_str(&format!("  assign t_diff_v{} = v_{} - u_{};\n", v, v, v));
        module_code.push_str(&format!(
            "  assign t0_sum_v{} = t_diff_v{} * t_diff_v{};\n",
            v, v, v
        ));
    }
    add_size = size;
    for depth in 0..log_size {
        let mut i = 0;

        for r2 in (0..add_size - 1).step_by(2) {
            module_code.push_str(&format!(
                "  assign t{}_sum_v{} = t{}_sum_v{} + t{}_sum_v{};\n",
                depth + 1,
                i,
                depth,
                r2,
                depth,
                r2 + 1
            ));
            i += 1;
        }
        // if the dimension is odd
        if add_size % 2 == 1 {
            module_code.push_str(&format!(
                "  assign t{}_sum_v{} = t{}_sum_v{};\n",
                depth + 1,
                i,
                depth,
                add_size - 1
            ));
        }
        add_size = (add_size as f32 / 2.0).ceil() as usize;
        module_code.push('\n');
    }
    module_code.push_str(&format!("  assign dist = t{}_sum_v{};\n", log_size, 0));

    module_code.push_str("endmodule\n");
    module_code
}

fn main() {
    let matches = Command::new("Generate Euclidean Distance")
        .about("Produces verilog code for euclidean distance between two vectors")
        .arg(
            Arg::new("vector-length")
                .long("vector-length")
                .value_name("NUMBER")
                .help("Number of elements per vector.")
                .required(true)
                .value_parser(clap::value_parser!(usize)),
        )
        .get_matches();

    let size = *matches.get_one::<usize>("vector-length").unwrap();
    println!("vector_length: {}", size);

    let filename = "v".to_owned() + &size.to_string() + "-euclidean-distance";
    let verilog_filename = "./designs/".to_owned() + filename.as_str() + ".v";
    let inputs_filename = "./test-cases/".to_owned() + filename.as_str() + ".inputs.csv";

    let inputs = generate_input_vectors(size);
    let verilog_code = generate_distance_module(size);

    let file = File::create(&verilog_filename);

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

    let file_inputs = File::create(&inputs_filename);
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
