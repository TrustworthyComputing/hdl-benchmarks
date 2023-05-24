use std::fs::File;
use std::io::prelude::*;
use clap::{Arg, Command};

pub fn generate_adder_nbit(n: usize) -> String {
    let mut verilog = String::new();

    // Define inputs and outputs
    verilog += &format!("module adder_{}bit(a, b, cin, sum, cout);\n", n);
    verilog += &format!("  input [{}:0] a;\n", n - 1);
    verilog += &format!("  input [{}:0] b;\n", n - 1);
    verilog += "  input cin;\n";
    verilog += &format!("  output [{}:0] sum;\n", n - 1);
    verilog += "  output cout;\n\n";

    // Define internal wires
    for i in 0..2 * n - 1 {
        verilog += &format!("  wire c{};\n", i);
    }
    for i in 0..n {
        verilog += &format!("  wire i{};\n", i);
    }
    for i in 0..n - 1 {
        verilog += &format!("  wire s{};\n", i);
    }
    verilog += &format!("  wire s{};\n\n", n - 1);

    // Define gates
    verilog += "  // First bit addition\n";
    verilog += "  xor x1(a[0], b[0], s0);\n";
    verilog += "  and a1(a[0], b[0], c0);\n";
    verilog += "  xor x2(s0, cin, sum[0]);\n";
    verilog += "  and a2(s0, cin, i0);\n";
    verilog += "  or o1(i0, c0, c1);\n\n";
    let mut c = 1;
    for i in 1..n - 1 {
        verilog += &format!("  // Bit {} addition\n", i);
        verilog += &format!("  xor x{}(a[{}], b[{}], s{});\n", 2 * i + 1, i, i, i);
        verilog += &format!("  and a{}(a[{}], b[{}], c{});\n", 2 * i + 1, i, i, c + 1);
        verilog += &format!("  xor x{}(s{}, c{}, sum[{}]);\n", 2 * i + 2, i, c, i);
        verilog += &format!("  and a{}(s{}, c{}, i{});\n", 2 * i + 2, i, c, i);
        verilog += &format!("  or o{}(i{}, c{}, c{});\n\n", i + 1, i, c + 1, c + 2);
        c += 2;
    }
    verilog += "  // Last bit addition\n";
    verilog += &format!(
        "  xor x{}(a[{}], b[{}], s{});\n",
        2 * (n - 1) + 1,
        n - 1,
        n - 1,
        n - 1
    );
    verilog += &format!(
        "  and a{}(a[{}], b[{}], c{});\n",
        2 * (n - 1) + 1,
        n - 1,
        n - 1,
        c + 1
    );
    verilog += &format!(
        "  xor x{}(s{}, c{}, sum[{}]);\n",
        2 * (n - 1) + 2,
        n - 1,
        c,
        n - 1
    );
    verilog += &format!(
        "  and a{}(s{}, c{}, i{});\n",
        2 * (n - 1) + 2,
        n - 1,
        c,
        n - 1
    );
    verilog += &format!("  or o{}(i{}, c{}, cout);\n", n, n - 1, c + 1);

    // End module
    verilog += "endmodule\n";

    verilog
}

fn main() {
    let matches = Command::new("HDL Preprocessor")
        .about("Preprocess Yosys outputs to cleaner structural Verilog")
        .arg(
            Arg::new("bits")
                .long("bits")
                .value_name("NUMBER")
                .help("Number of bits for the adder")
                .required(true)
                .value_parser(clap::value_parser!(usize)),
        )
        .get_matches();

    let bits = matches.get_one::<usize>("bits").expect("required").to_owned();
    let circuit = generate_adder_nbit(bits);
    
    let file = File::create(
        "./processed-netlists/".to_owned() + &bits.to_string() + "-bit-adder.v"
    );

    match file {
        Ok(mut file) => {
            if let Err(err) = file.write_all(circuit.as_bytes()) {
                eprintln!("Failed to write to file: {}", err);
            }
        }
        Err(err) => {
            eprintln!("Failed to create file: {}", err);
        }
    }
}
