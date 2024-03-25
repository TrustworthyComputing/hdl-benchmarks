use clap::{Arg, Command};
use rand::Rng;
use std::fs::File;
use std::io::prelude::*;

fn generate_inputs(size: usize) -> String {
    let mut inputs = String::from("wire, value\n");
    let mut rng = rand::thread_rng();

    for r in 0..size {
        for c in 0..size {
            let value: u8 = rng.gen();
            inputs.push_str(&format!("a_{}_{}, {}\n", r, c, value));
        }
        let value: u8 = rng.gen();
        inputs.push_str(&format!("b_{}, {}\n", r, value));
    }
    inputs
}

fn generate_matvec_module(size: usize) -> String {
    let mut module_code = String::from("module lr (");
    for r in 0..size {
        for c in 0..size {
            module_code.push_str(&format!("a_{}_{}, ", r, c));
        }
        module_code.push_str(&format!("b_{}, ", r));
    }
    for r in 0..size {
        module_code.push_str(&format!("c_{}, ", r));
    }
    module_code.truncate(module_code.len() - 2); // remove last space and comma
    module_code.push_str(");\n\n");

    for r in 0..size {
        for c in 0..size {
            module_code.push_str(&format!("  input [15:0] a_{}_{};\n", r, c));
        }
        module_code.push_str(&format!("  input [15:0] b_{};\n", r));
    }
    module_code.push('\n');
    for r in 0..size {
        module_code.push_str(&format!("  output [15:0] c_{};\n", r));
    }
    module_code.push('\n');

    for r in 0..size {
        for c in 0..4 {
            module_code.push_str(&format!("  wire [15:0] t_sig_r{}_c{};\n", r, c));
        }
    }

    for r in 0..size {
        for c in 0..size {
            module_code.push_str(&format!("  wire [15:0] t0_r{}_c{};\n", r, c));
        }
        let log_size = (f64::log2(size as f64) as usize) + 1;
        let mut add_size = size;
        for depth in 0..log_size {
            let mut i = 0;
            for _ in (0..add_size - 1).step_by(2) {
                module_code.push_str(&format!(
                    "  wire [15:0] t{}_r{}_c{};\n",
                    depth + 1,
                    r,
                    i
                ));
                i += 1;
            }
            // if the dimension is odd
            if add_size % 2 == 1 {
                module_code.push_str(&format!(
                    "  wire [15:0] t{}_r{}_c{};\n",
                    depth + 1,
                    r,
                    i
                ));
            }
            add_size = (add_size as f32 / 2.0).ceil() as usize;
        }
    }
    module_code.push('\n');

    // a s d f g
    //  b   y  g
    //    w  x
    //      s

    for r in 0..size {
        for c in 0..size {
            module_code.push_str(&format!(
                "  assign t0_r{}_c{} = a_{}_{} * b_{};\n",
                r, c, r, c, c
            ));
        }
        let log_size = (f64::log2(size as f64) as usize) + 1;
        let mut add_size = size;

        for depth in 0..log_size {
            let mut i = 0;

            for c in (0..add_size - 1).step_by(2) {
                module_code.push_str(&format!(
                    "  assign t{}_r{}_c{} = t{}_r{}_c{} + t{}_r{}_c{};\n",
                    depth + 1,
                    r,
                    i,
                    depth,
                    r,
                    c,
                    depth,
                    r,
                    c + 1
                ));
                i += 1;
            }
            // if the dimension is odd
            if add_size % 2 == 1 {
                module_code.push_str(&format!(
                    "  assign t{}_r{}_c{} = t{}_r{}_c{};\n",
                    depth + 1,
                    r,
                    i,
                    depth,
                    r,
                    add_size - 1,
                ));
            }
            add_size = (add_size as f32 / 2.0).ceil() as usize;
            module_code.push('\n');
        }
        module_code.push_str(&format!(
                "  assign t_sig_r{}_c0 = t{}_r{}_c{} * t{}_r{}_c{};\n",
                r, log_size, r, 0, log_size, r, 0
        ));
        module_code.push_str(&format!(
            "  assign t_sig_r{}_c1 = t{}_r{}_c{} * t_sig_r{}_c0;\n",
            r, log_size, r, 0, r
        ));
        module_code.push_str(&format!(
            "  assign t_sig_r{}_c2 = t{}_r{}_c{} * 12;\n",
            r, log_size, r, 0
        ));
        module_code.push_str(&format!(
            "  assign t_sig_r{}_c3 = t_sig_r{}_c1 + t_sig_r{}_c2;\n",
            r, r, r
        ));
        module_code.push_str(&format!(
            "  assign c_{} = t_sig_r{}_c3 + 24;\n",
            r, r
        ));
    }

    module_code.push_str("endmodule\n");
    module_code
}

fn main() {
    let matches = Command::new("Generate LR Inference")
        .about("Produces verilog code for logistic regression inference")
        .arg(
            Arg::new("rows")
                .long("rows")
                .value_name("NUMBER")
                .help("Number of rows of the square matrix. This is also the number of columns.")
                .required(true)
                .value_parser(clap::value_parser!(usize)),
        )
        .get_matches();

    let size = *matches.get_one::<usize>("rows").unwrap();
    println!("rows: {}, columns: {}", size, size);

    let filename = size.to_string()
        + "-attribute"
        + "-lr";
    let verilog_filename = "./designs/".to_owned() + filename.as_str() + ".v";
    let inputs_filename = "./test-cases/".to_owned() + filename.as_str() + ".inputs.csv";

    let inputs = generate_inputs(size);
    let verilog_code = generate_matvec_module(size);

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
