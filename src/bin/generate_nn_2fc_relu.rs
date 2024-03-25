use clap::{Arg, Command};
use rand::Rng;
use std::fs::File;
use std::io::prelude::*;

const NUM_LAYERS: usize = 2;
const NUM_CLASSES: usize = 10;
const NUM_HIDDEN: usize = 100;
static OUT_NEURONS: [usize; NUM_LAYERS] = [NUM_HIDDEN, NUM_CLASSES];

fn generate_inputs(size: usize) -> String {
    let mut inputs = String::from("wire, value\n");
    let mut rng = rand::thread_rng();

    for r in 0..size {
        let value: u8 = rng.gen();
        inputs.push_str(&format!("in_{}, {}\n", r, value));    
    }

    for l in 0..NUM_LAYERS {
        // Get the number of input neurons, size if layer is 0, output of layer
        // l-1 otherwise
        let in_size: usize = if l == 0 {
            size
        } else {
            OUT_NEURONS[l - 1]
        };
        for r in 0..in_size {
            for c in 0..OUT_NEURONS[l] {
                let value: u8 = rng.gen();
                inputs.push_str(&format!("weights_{}_{}_{}, {}\n", l, r, c, value));    
            }
        }
    }
    inputs
}

fn generate_nn_module(size: usize) -> String {
    let mut module_code = String::from("module nn (");
    for r in 0..size {
        module_code.push_str(&format!("in_{}, ", r));    
    }
    for l in 0..NUM_LAYERS {
        // Get the number of input neurons, size if layer is 0, output of layer
        // l-1 otherwise
        let in_size: usize = if l == 0 {
            size
        } else {
            OUT_NEURONS[l - 1]
        };
        for r in 0..in_size {
            for c in 0..OUT_NEURONS[l] {
                module_code.push_str(&format!("weights_{}_{}_{}, ", l, r, c));    
            }
        }
    }
    for r in 0..NUM_CLASSES {
        module_code.push_str(&format!("res_{}, ", r));
    }
    module_code.truncate(module_code.len() - 2); // remove last space and comma
    module_code.push_str(");\n\n");

    for r in 0..size {
        module_code.push_str(&format!("  input [15:0] in_{};\n", r));
    }
    for l in 0..NUM_LAYERS {
        // Get the number of input neurons, size if layer is 0, output of layer
        // l-1 otherwise
        let in_size: usize = if l == 0 {
            size
        } else {
            OUT_NEURONS[l - 1]
        };
        for r in 0..in_size {
            for c in 0..OUT_NEURONS[l] {
                module_code.push_str(&format!("  input [15:0] weights_{}_{}_{};\n", l, r, c));
            }
        }
    }
    module_code.push('\n');
    for r in 0..NUM_CLASSES {
        module_code.push_str(&format!("  output [15:0] res_{};\n", r));
    }
    module_code.push('\n');

    // temp variables for relu activation
    for l in 0..(NUM_LAYERS-1) {
        for r in 0..OUT_NEURONS[l] {
            module_code.push_str(&format!("  wire [15:0] t_relu_0_l{}_r{};\n", l, r));
            module_code.push_str(&format!("  wire [15:0] t_relu_1_l{}_r{};\n", l, r));
        }
    }

    for l in 0..NUM_LAYERS {
        let in_size: usize = if l == 0 {
            size
        } else {
            OUT_NEURONS[l - 1]
        };
        for r in 0..OUT_NEURONS[l] {
            for c in 0..in_size {
                module_code.push_str(&format!("  wire [15:0] t0_l{}_r{}_c{};\n", l, r, c));
            }
            let log_size = (f64::log2(size as f64) as usize) + 1;
            let mut add_size = size;
            for depth in 0..log_size {
                let mut i = 0;
                for _ in (0..add_size - 1).step_by(2) {
                    module_code.push_str(&format!(
                        "  wire [15:0] t{}_l{}_r{}_c{};\n",
                        depth + 1,
                        l,
                        r,
                        i
                    ));
                    i += 1;
                }
                // if the dimension is odd
                if add_size % 2 == 1 {
                    module_code.push_str(&format!(
                        "  wire [15:0] t{}_l{}_r{}_c{};\n",
                        depth + 1,
                        l,
                        r,
                        i
                    ));
                }
                add_size = (add_size as f32 / 2.0).ceil() as usize;
            }
        }
    }
    module_code.push('\n');

    // a s d f g
    //  b   y  g
    //    w  x
    //      s
    for l in 0..NUM_LAYERS {
        let in_size: usize = if l == 0 {
            size
        } else {
            OUT_NEURONS[l - 1]
        };
        for r in 0..OUT_NEURONS[l] {
            for c in 0..in_size {
                if l == 0 {
                    module_code.push_str(&format!(
                        "  assign t0_l{}_r{}_c{} = weights_{}_{}_{} * in_{};\n",
                        l, r, c, l, c, r, c
                    ));
                } else {
                    module_code.push_str(&format!(
                        "  assign t0_l{}_r{}_c{} = weights_{}_{}_{} * t_relu_1_l{}_r{};\n",
                        l, r, c, l, c, r, l - 1, c
                    ));
                }
            }
            let log_size = (f64::log2(size as f64) as usize) + 1;
            let mut add_size = size;

            for depth in 0..log_size {
                let mut i = 0;

                for c in (0..add_size - 1).step_by(2) {
                    module_code.push_str(&format!(
                        "  assign t{}_l{}_r{}_c{} = t{}_l{}_r{}_c{} + t{}_l{}_r{}_c{};\n",
                        depth + 1,
                        l,
                        r,
                        i,
                        depth,
                        l,
                        r,
                        c,
                        depth,
                        l,
                        r,
                        c + 1
                    ));
                    i += 1;
                }
                // if the dimension is odd
                if add_size % 2 == 1 {
                    module_code.push_str(&format!(
                        "  assign t{}_l{}_r{}_c{} = t{}_l{}_r{}_c{};\n",
                        depth + 1,
                        l,
                        r,
                        i,
                        depth,
                        l,
                        r,
                        add_size - 1,
                    ));
                }
                add_size = (add_size as f32 / 2.0).ceil() as usize;
                module_code.push('\n');
            }
            if l < (NUM_LAYERS - 1) {
                module_code.push_str(&format!(
                        "  assign t_relu_0_l{}_r{} = t{}_l{}_r{}_c{} * t{}_l{}_r{}_c{};\n",
                        l, r, log_size, l, r, 0, log_size, l, r, 0
                ));
                module_code.push_str(&format!(
                    "  assign t_relu_1_l{}_r{} = t{}_l{}_r{}_c{} + t_relu_0_l{}_r{};\n",
                    l, r, log_size, l, r, 0, l, r
            ));
            } else {
                module_code.push_str(&format!(
                    "  assign res_{} = t{}_l{}_r{}_c{};\n",
                    r, log_size, l, r, 0
                ));    
            }
        }
    }

    module_code.push_str("endmodule\n");
    module_code
}

fn main() {
    let matches = Command::new("Generate NN Inference")
        .about("Produces verilog code for a two-layer NN")
        .arg(
            Arg::new("input_size")
                .long("input_size")
                .value_name("NUMBER")
                .help("Number of elements/pixels per input")
                .required(true)
                .value_parser(clap::value_parser!(usize)),
        )
        .get_matches();

    let size = *matches.get_one::<usize>("input_size").unwrap();

    let filename = size.to_string()
        + "-invec"
        + "-nn";
    let verilog_filename = "./designs/".to_owned() + filename.as_str() + ".v";
    let inputs_filename = "./test-cases/".to_owned() + filename.as_str() + ".inputs.csv";

    let inputs = generate_inputs(size);
    let verilog_code = generate_nn_module(size);

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
