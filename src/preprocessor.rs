use clap::{Arg, Command};
use std::collections::{HashMap, HashSet};
use std::fs::File;
use std::io::{BufRead, BufReader, BufWriter, Write};
use std::path::Path;

fn parse_args() -> (String, String) {
    let matches = Command::new("HDL Preprocessor")
        .about("Preprocess Yosys outputs to cleaner structural Verilog")
        .arg(
            Arg::new("input")
                .long("input")
                .value_name("FILE")
                .help("Sets the input file to use")
                .required(true),
        )
        .arg(
            Arg::new("output")
                .long("output")
                .value_name("FILE")
                .help("Sets the output file to use")
                .required(false),
        )
        .get_matches();

    let in_file_name = matches
        .get_one::<String>("input")
        .expect("required")
        .to_owned();
    let out_file_name = {
        if matches.contains_id("output") {
            matches.get_one::<String>("output").unwrap().to_owned()
        } else {
            let mut out_file_name = "./processed-netlists/".to_owned();
            let path = Path::new(&in_file_name);
            if let Some(file_name) = path.file_name() {
                out_file_name += file_name.to_str().unwrap();
            }
            out_file_name
        }
    };

    (in_file_name, out_file_name)
}

fn postprocess_assign_dict(assign_dict: &mut HashMap<String, String>) {
    let mut keys_to_modify = Vec::new();
    for (key, value) in assign_dict.iter() {
        if assign_dict.contains_key(value) {
            keys_to_modify.push(key.clone());
        }
    }
    for key in keys_to_modify {
        let mut value = assign_dict[&key].clone();
        while assign_dict.contains_key(&value) {
            value = assign_dict[&value].clone();
        }
        assign_dict.insert(key, value);
    }
}

fn build_assign_dict(in_file_name: &String) -> HashMap<String, String> {
    let in_file = File::open(in_file_name).expect("Failed to open file");
    let reader = BufReader::new(in_file);
    let mut assign_dict: HashMap<String, String> = HashMap::new();
    let mut assign_dict_for_outputs: HashMap<String, String> = HashMap::new();
    let mut output_ports = HashSet::new();
    for line in reader.lines() {
        let line = line.expect("Failed to read line").trim().to_owned();

        if line.contains("output") {
            if line.contains('[') {
                output_ports.insert(
                    line.split(' ')
                        .nth(2)
                        .unwrap()
                        .trim_end_matches(';')
                        .to_string(),
                );
            } else {
                output_ports.insert(
                    line.split(' ')
                        .nth(1)
                        .unwrap()
                        .trim_end_matches(';')
                        .to_string(),
                );
            }
        } else if line.contains("assign") && !line.contains(">>") {
            let tokens: Vec<&str> = line.split(' ').collect();
            let output = tokens[1]
                .trim_end_matches(';')
                .trim_start_matches('_')
                .trim_end_matches('_')
                .to_string();
            let input = tokens[3]
                .trim_end_matches(';')
                .trim_start_matches('_')
                .trim_end_matches('_')
                .to_string();

// TODO
// assign x = input
// assign y = x
// assign z = y
// assign output = z

// assign_dict{ x, input };
// assign_dict{ y, x };
// assign_dict{ z, y };
// assign_dict{ z, output};

            if output_ports.contains(&output) {
                println!("input {}", input);
                println!("output {}", output);

                // assign_dict.insert(input, output);
                assign_dict_for_outputs.insert(input, output);
            } else {
                assign_dict.insert(output, input);
            }
        }
    }

    postprocess_assign_dict(&mut assign_dict);

    assign_dict.extend(assign_dict_for_outputs);

    assign_dict
}

fn convert_verilog(
    in_file_name: &String,
    out_file_name: &String,
    wire_dict: &HashMap<String, String>,
) {
    let in_file = File::open(in_file_name).expect("Failed to open file");
    let out_file = File::create(out_file_name).expect("Failed to create file");

    let reader = BufReader::new(in_file);
    let mut out_writer = BufWriter::new(out_file);
    let mut wires = Vec::new();
    let mut inputs = Vec::new();
    let mut outputs = Vec::new();
    let mut multi_bit_inputs = Vec::new();
    let mut multi_bit_outputs = Vec::new();
    let mut gates = Vec::new();
    let mut lut_id = 1;
    let supported_gates = vec![
        "AND", "DFF", "MUX", "NAND", "NOR", "NOT", "OR", "XOR", "XNOR",
    ];

    let mut lines = reader.lines();
    loop {
        let line = lines.next();
        if line.is_none() {
            break;
        }
        let line = line
            .expect("Failed to read line")
            .unwrap()
            .trim()
            .to_owned();
        if line.is_empty()
            || line.starts_with("//")
            || line.starts_with("(*")
            || line.starts_with("/*")
        {
            continue;
        }

        let tokens: Vec<&str> = line
            .split([',', ' '].as_ref())
            .filter(|s| !s.is_empty())
            .collect();
        match tokens[0] {
            "wire" => {
                for token in tokens.iter().skip(1) {
                    wires.push(String::from(
                        token
                            .trim_matches(',')
                            .trim_end_matches(';')
                            .trim_start_matches('_')
                            .trim_end_matches('_'),
                    ));
                }
            }
            "input" => {
                if line.contains('[') {
                    multi_bit_inputs.push("  ".to_string() + &line.to_string());
                } else {
                    for token in tokens.iter().skip(1) {
                        inputs.push(String::from(token.trim_matches(',').trim_end_matches(';')));
                    }
                }
            }
            "output" => {
                if line.contains('[') {
                    multi_bit_outputs.push("  ".to_string() + &line.to_string());
                } else {
                    for token in tokens.iter().skip(1) {
                        outputs.push(String::from(token.trim_matches(',').trim_end_matches(';')));
                    }
                }
            }
            "assign" => {
                if line.contains(">>") {
                    // LUT
                    let mut lut_line = "lut ".to_owned();
                    lut_line += "lut_gate";
                    lut_line += &lut_id.to_string();
                    lut_line += "(";
                    lut_id += 1;
                    if line.contains('h') {
                        lut_line += "0x";
                    }
                    lut_line += tokens[3]
                        .split(['h', 'd'])
                        .filter(|s| !s.is_empty())
                        .collect::<Vec<_>>()[1];
                    lut_line += ", ";
                    let mut curr_token = tokens[4];
                    let mut token_idx = 4;
                    if !line.contains("}") { // 1 input LUT
                        curr_token = tokens[5];
                        curr_token = curr_token.trim_start_matches('_').trim_end_matches(';');
                        curr_token = curr_token.trim_end_matches('_');

                        if wire_dict.contains_key(curr_token) {
                            lut_line += &wire_dict[curr_token];
                        } else {
                            lut_line += curr_token;
                        }
                        lut_line += ", ";
                        // end of lut statement
                        curr_token = tokens[1].trim_start_matches('_').trim_end_matches('_');
                        if wire_dict.contains_key(curr_token) {
                            lut_line += &wire_dict[curr_token];
                        } else {
                            lut_line += curr_token;
                        }
                        lut_line += ");";
                    }
                    else {
                        loop {
                            if !curr_token.contains(">>") && !curr_token.contains('{') {
                                curr_token = curr_token.trim_start_matches('_').trim_end_matches(',');
                                curr_token = curr_token.trim_end_matches('_');
                                if wire_dict.contains_key(curr_token) {
                                    lut_line += &wire_dict[curr_token];
                                } else {
                                    lut_line += curr_token;
                                }
                                lut_line += ", ";
                            }
                            token_idx += 1;
                            curr_token = tokens[token_idx];
                            if curr_token.contains("};") {
                                // end of lut statement
                                curr_token = tokens[1].trim_start_matches('_').trim_end_matches('_');
                                if wire_dict.contains_key(curr_token) {
                                    lut_line += &wire_dict[curr_token];
                                } else {
                                    lut_line += curr_token;
                                }
                                lut_line += ");";
                                break;
                            }
                        }
                    }
                    gates.push(lut_line.to_string());
                }
            }
            _ => {
                // Gate, module
                // If it's a gate
                if supported_gates.iter().any(|gate| line.contains(gate)) {
                    let is_dff = line.contains("DFF");
                    let mut gate_line = tokens[0][3..tokens[0].len() - 1].to_lowercase();
                    if is_dff {
                        gate_line.truncate(gate_line.len() - 4);
                        lines.next();
                    }
                    gate_line += &(" g".to_owned() + tokens[1] + "(");
                    // Continue reading all inputs until you find a ');'
                    loop {
                        let line = lines.next();
                        if line.is_none() {
                            break;
                        }
                        let line = line
                            .expect("Failed to read line")
                            .unwrap()
                            .trim()
                            .to_owned();
                        if line.starts_with(");") {
                            break;
                        } else {
                            let wire_name = line
                                [line.find('(').unwrap() + 1..line.find(')').unwrap()]
                                .trim_start_matches('_')
                                .trim_end_matches('_');

                            if wire_dict.contains_key(wire_name) {
                                gate_line += &wire_dict[wire_name];
                            } else {
                                gate_line += wire_name;
                            }
                            gate_line += ", ";
                        }
                    }
                    gate_line.pop();
                    gate_line.pop();
                    if is_dff {
                        let mut parts: Vec<&str> = gate_line.split(',').collect();
                        parts.pop(); // remove the last element from the vector
                        gate_line = parts.join(","); // join the remaining elements with commas
                    }
                    gate_line += ");";
                    gates.push(gate_line.to_string());
                } else if line.starts_with("module") {
                    // module or end_module
                    out_writer
                        .write_all((line + "\n").as_bytes())
                        .expect("Failed to write line");
                }
            }
        }
    }

    // Write wires
    out_writer
        .write_all(b"  wire ")
        .expect("Failed to write line");
    out_writer
        .write_all(wires.join(", ").as_bytes())
        .expect("Failed to write line");
    out_writer.write_all(b";\n").expect("Failed to write line");

    // Write inputs
    if !multi_bit_inputs.is_empty() {
        out_writer
            .write_all((multi_bit_inputs.join("\n") + "\n").as_bytes())
            .expect("Failed to write line");
    }
    if !inputs.is_empty() {
        out_writer
            .write_all(b"  input ")
            .expect("Failed to write line");
        out_writer
            .write_all(inputs.join(", ").as_bytes())
            .expect("Failed to write line");
        out_writer.write_all(b";\n").expect("Failed to write line");
    }

    // Write outputs
    if !multi_bit_outputs.is_empty() {
        out_writer
            .write_all((multi_bit_outputs.join("\n") + "\n").as_bytes())
            .expect("Failed to write line");
    }
    if !outputs.is_empty() {
        out_writer
            .write_all(b"  output ")
            .expect("Failed to write line");
        out_writer
            .write_all(outputs.join(", ").as_bytes())
            .expect("Failed to write line");
        out_writer.write_all(b";\n").expect("Failed to write line");
    }

    // Write gates
    for gate in gates {
        out_writer
            .write_all(("  ".to_owned() + &gate + "\n").as_bytes())
            .expect("Failed to write line");
    }

    out_writer
        .write_all(b"\nendmodule\n")
        .expect("Failed to write line");
}

fn main() {
    let (in_file_name, out_file_name) = parse_args();

    let wire_dict = build_assign_dict(&in_file_name);
    convert_verilog(&in_file_name, &out_file_name, &wire_dict);
}