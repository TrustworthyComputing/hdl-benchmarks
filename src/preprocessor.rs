use clap::{Arg, ArgAction, Command};
use std::collections::{HashMap, HashSet};
use std::fs::File;
use std::io::{BufRead, BufReader, BufWriter, Write};
use std::path::Path;

fn parse_args() -> (String, String, bool) {
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
        .arg(
            Arg::new("arithmetic")
                .long("arithmetic")
                .help("Toggle arithmetic mode")
                .required(false)
                .action(ArgAction::SetTrue),
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

    (in_file_name, out_file_name, matches.get_flag("arithmetic"))
}

fn postprocess_assign_dict(
    assign_dict: &mut HashMap<String, String>,
    wire_to_port: HashMap<String, String>,
) {
    let mut keys_to_modify = Vec::new();
    let mut wire_to_port_map: HashMap<String, String> = HashMap::new();
    for (key, value) in assign_dict.iter() {
        let is_port = wire_to_port.contains_key(value);
        if assign_dict.contains_key(value) || is_port {
            keys_to_modify.push(key.clone());
        }
        if is_port {
            let tmp_val = wire_to_port[value].clone();
            let mut tmp_key = value.clone();
            wire_to_port_map.insert(tmp_key.clone(), tmp_val.clone());
            while assign_dict.contains_key(&tmp_key) {
                tmp_key = assign_dict[&tmp_key].clone();
                wire_to_port_map.insert(tmp_key.clone(), tmp_val.clone());
            }
        }
    }
    for (key, value) in wire_to_port_map.iter() {
        if assign_dict.contains_key(key) && assign_dict[key].contains("'h") {
            assign_dict.insert(value.clone(), assign_dict[key].clone());
            continue;
        }
        assign_dict.insert(key.clone(), value.clone());
    }
}

fn get_multibit_assign_wires(line: &str) -> (Vec<String>, Vec<String>) {
    let mut left_side = Vec::new();
    let mut right_side = Vec::new();

    // Split the line by '='
    let s = line.replace(&[';', ' ', '{', '}'][..], "");
    let parts: Vec<&str> = s.split('=').collect();

    assert_eq!(parts.len(), 2);

    // Extract the left and right sides
    let left = parts[0];
    let right = parts[1];

    // Split the brace content by commas
    let wire_names: Vec<&str> = left.split(',').map(|s| s.trim()).collect();
    for wire_name in wire_names {
        if let Some(left_bracket) = wire_name.find('[') {
            if let Some(right_bracket) = wire_name.rfind(']') {
                let name = &wire_name[0..left_bracket];
                let range = &wire_name[(left_bracket + 1)..right_bracket];
                if range.contains(':') {
                    let (start, end) = parse_range(range);
                    for i in (end..=start).rev() {
                        left_side.push(format!("{}[{}]", name, i));
                    }
                } else {
                    left_side.push(wire_name.to_string());
                }
            }
        } else {
            left_side.push(wire_name.to_string());
        }
    }

    // Split the brace content by commas
    let wire_names_right: Vec<&str> = right.split(',').map(|s| s.trim()).collect();
    for wire_name in wire_names_right {
        if let Some(left_bracket) = wire_name.find('[') {
            if let Some(right_bracket) = wire_name.rfind(']') {
                let name = &wire_name[0..left_bracket];
                let range = &wire_name[(left_bracket + 1)..right_bracket];
                if range.contains(':') {
                    let (start, end) = parse_range(range);
                    for i in (end..=start).rev() {
                        right_side.push(format!("{}[{}]", name, i));
                    }
                } else {
                    right_side.push(wire_name.to_string());
                }
            }
        } else if wire_name.contains("'h") {
            // foo[1:0] = 2'h0; => foo[1], foo[0] = 1'h0, 1'h0;
            let wire_name_split: Vec<&str> = wire_name.split('\'').collect();
            let bit_len = wire_name_split[0].parse::<i32>().unwrap_or(0);
            for _ in 0..bit_len {
                right_side.push("1'".to_owned() + wire_name_split[1]);
            }
        } else {
            right_side.push(wire_name.to_string());
        }
    }

    (left_side, right_side)
}

fn parse_range(range: &str) -> (i32, i32) {
    let parts: Vec<&str> = range.split(':').collect();
    if parts.len() == 2 {
        if let Ok(start) = parts[0].parse::<i32>() {
            if let Ok(end) = parts[1].parse::<i32>() {
                return (start, end);
            }
        }
    }

    (0, 0) // Default range if parsing fails
}

fn build_assign_dict(
    in_file_name: &String,
) -> (HashMap<String, String>, HashMap<String, Vec<String>>) {
    let in_file = File::open(in_file_name).expect("Failed to open file");
    let reader = BufReader::new(in_file);
    let mut assign_dict: HashMap<String, String> = HashMap::new();
    let mut wire_to_port: HashMap<String, String> = HashMap::new();
    let mut output_ports = HashSet::new();
    let mut leftover_maps: HashMap<String, Vec<String>> = HashMap::new();
    for line in reader.lines() {
        let line = line.expect("Failed to read line").trim().to_owned();
        if line.contains("output") {
            if line.contains('[') {
                // Find the positions of '[' and ':'
                let open_bracket_pos = line.find('[');
                let colon_pos = line.find(':');

                // Ensure both '[' and ':' are present in the string
                if let (Some(open_bracket), Some(colon)) = (open_bracket_pos, colon_pos) {
                    // Extract the substring between '[' and ':'
                    let number_str = &line[open_bracket + 1..colon];
                    // Parse the substring to an integer
                    if let Ok(number) = number_str.parse::<i32>() {
                        for curr_wire in 0..number + 1 {
                            output_ports.insert(
                                line.split(' ')
                                    .nth(2)
                                    .unwrap()
                                    .trim_end_matches(';')
                                    .to_string()
                                    + "["
                                    + &curr_wire.to_string()
                                    + "]",
                            );
                        }
                    } else {
                        println!("Error: Invalid number format.");
                    }
                } else {
                    println!("Error: '[' and/or ':' not found in the string.");
                }
            } else {
                output_ports.insert(
                    line.split(' ')
                        .nth(1)
                        .unwrap()
                        .trim_end_matches(';')
                        .to_string(),
                );
            }
        } else if line.contains("assign")
            && !line.contains(">>")
            && !line.contains('+')
            && !line.contains('-')
            && !line.contains('*')
            && !line.contains('/')
        {
            // assign dest = source;
            // assign dest[WIRE_ID] = source;
            // assign dest [WIRE_ID] = source;
            // assign dest = source [WIRE_ID];
            // assign dest = source[WIRE_ID];
            // assign dest [WIRE_ID] = source [WIRE_ID];
            // assign dest [WIRE_ID] = source[WIRE_ID];
            // assign dest[WIRE_ID] = source [WIRE_ID];
            // assign dest[WIRE_ID] = source[WIRE_ID];

            let cleaned_line: String = line.replace("assign", "").replace([';', ' '], "");

            // multi-bit assign
            if line.contains(':') || line.contains('{') {
                let (left, right) = get_multibit_assign_wires(&cleaned_line);
                assert_eq!(
                    left.len(),
                    right.len(),
                    "assign statement LHS is not the same size as RHS!"
                );
                for i in 0..left.len() {
                    let output = left[i].clone();
                    let input = right[i].clone();

                    if output_ports.contains(&output) {
                        if wire_to_port.contains_key(&input) {
                            leftover_maps
                                .entry(input.clone())
                                .or_insert(Vec::new())
                                .push(output.clone());
                        } else {
                            wire_to_port.insert(input.clone(), output.clone());
                        }
                    }
                    if let std::collections::hash_map::Entry::Vacant(e) =
                        assign_dict.entry(output.clone())
                    {
                        e.insert(input);
                    } else {
                        leftover_maps
                            .entry(output.clone())
                            .or_insert(Vec::new())
                            .push(input.clone());
                    }
                }
            } else {
                // single-bit assign
                let tokens: Vec<&str> = cleaned_line.split('=').collect();
                let output = tokens[0].to_string();
                let input = tokens[1].to_string();
                if output_ports.contains(&output) {
                    if wire_to_port.contains_key(&input) {
                        leftover_maps
                            .entry(input.clone())
                            .or_insert(Vec::new())
                            .push(output.clone());
                    } else {
                        wire_to_port.insert(input.clone(), output.clone());
                    }
                }
                if let std::collections::hash_map::Entry::Vacant(e) =
                    assign_dict.entry(output.clone())
                {
                    e.insert(input);
                } else {
                    leftover_maps
                        .entry(output.clone())
                        .or_insert(Vec::new())
                        .push(input.clone());
                }
            }
        }
    }

    postprocess_assign_dict(&mut assign_dict, wire_to_port);
    (assign_dict, leftover_maps)
}

fn convert_verilog(
    in_file_name: &String,
    out_file_name: &String,
    wire_dict: &HashMap<String, String>,
    leftover_dict: &HashMap<String, Vec<String>>,
    arith: bool,
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
    let supported_gates = [
        "AND", "DFF", "MUX", "NAND", "NOR", "NOT", "OR", "XOR", "XNOR", "lut",
    ];

    let mut lines = reader.lines();
    loop {
        let line = lines.next();
        if line.is_none() {
            break;
        }
        let mut line = line
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
                    if token.contains('[') {
                        continue;
                    }
                    wires.push(String::from(
                        token
                            .trim_matches(',')
                            // .trim_matches('_')
                            .trim_end_matches(';'),
                    ));
                }
            }
            "input" => {
                if line.contains('[') {
                    multi_bit_inputs.push("  ".to_string() + &line.to_string());
                    if !arith {
                        // Find the start and end of the number substring
                        let start_index = line.find('[').unwrap_or(0) + 1;
                        let end_index = line.find(':').unwrap_or(0);
                        // Extract the number substring
                        let number_str = &line[start_index..end_index];
                        // Convert the substring to a u32
                        let number: u32 = number_str.parse().unwrap_or(0);
                        for i in 0..number + 1 {
                            inputs.push(
                                tokens[2].trim_end_matches(';').to_string()
                                    + "["
                                    + &i.to_string()
                                    + "]",
                            );
                        }
                    }
                } else {
                    for token in tokens.iter().skip(1) {
                        inputs.push(String::from(token.trim_matches(',').trim_end_matches(';')));
                    }
                }
            }
            "output" => {
                if line.contains('[') {
                    multi_bit_outputs.push("  ".to_string() + &line.to_string());
                    if !arith {
                        // Find the start and end of the number substring
                        let start_index = line.find('[').unwrap_or(0) + 1;
                        let end_index = line.find(':').unwrap_or(0);
                        // Extract the number substring
                        let number_str = &line[start_index..end_index];
                        // Convert the substring to a u32
                        let number: u32 = number_str.parse().unwrap_or(0);
                        for i in 0..number + 1 {
                            outputs.push(
                                tokens[2].trim_end_matches(';').to_string()
                                    + "["
                                    + &i.to_string()
                                    + "]",
                            );
                        }
                    }
                } else {
                    for token in tokens.iter().skip(1) {
                        outputs.push(String::from(token.trim_matches(',').trim_end_matches(';')));
                    }
                }
            }
            "assign" => {
                if line.contains(">>") && line.contains('{') {
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
                    if !line.contains('}') {
                        // 1 input LUT
                        curr_token = tokens[5];
                        curr_token = curr_token.trim_end_matches(';');

                        if wire_dict.contains_key(curr_token) {
                            lut_line += &wire_dict[curr_token];
                        } else {
                            lut_line += curr_token;
                        }
                        lut_line += ", ";
                        // end of lut statement
                        curr_token = tokens[1];

                        if wire_dict.contains_key(curr_token) {
                            lut_line += &wire_dict[curr_token];
                        } else {
                            lut_line += curr_token;
                        }
                        lut_line += ");";
                    } else {
                        loop {
                            if !curr_token.contains(">>") && !curr_token.contains('{') {
                                curr_token = curr_token.trim_end_matches(',');
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
                                curr_token = tokens[1];
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
                } else if line.contains('+')
                    || line.contains('*')
                    || line.contains('-')
                    || line.contains('/')
                    || line.contains("<<")
                    || line.contains(">>")
                {
                    let mut arith_line = "".to_owned();
                    if tokens[4] == "+" {
                        arith_line += "add a";
                    } else if tokens[4] == "-" {
                        arith_line += "sub s";
                    } else if tokens[4] == "*" {
                        arith_line += "mult m";
                    } else if tokens[4] == "/" {
                        arith_line += "div d";
                    } else if tokens[4] == "<<" {
                        arith_line += "shl l";
                    } else if tokens[4] == ">>" {
                        arith_line += "shr r";
                    }
                    arith_line += &lut_id.to_string();
                    lut_id += 1;
                    arith_line += "(";
                    arith_line += tokens[3];
                    arith_line += ", ";
                    arith_line += tokens[5].trim_end_matches(';');
                    arith_line += ", ";
                    arith_line += tokens[1];
                    arith_line += ");";
                    gates.push(arith_line.to_string());
                } else if line.contains('=') && arith {
                    let mut arith_line = "copy c".to_owned();
                    arith_line += &lut_id.to_string();
                    lut_id += 1;
                    arith_line += "(";
                    arith_line += tokens[3].trim_end_matches(';');
                    arith_line += ", ";
                    arith_line += tokens[1];
                    arith_line += ");";
                    gates.push(arith_line.to_string());
                }
            }
            _ => {
                // Gate, module
                // If it's a gate
                if supported_gates.iter().any(|gate| line.contains(gate)) {
                    let is_dff = line.contains("DFF");
                    let is_lut = line.contains("lut");
                    if is_lut {
                        let mut lut_line = "lut ".to_owned();
                        lut_line += "lut_gate";
                        lut_line += &lut_id.to_string();
                        lut_line += "(";
                        lut_id += 1;
                        while !line.contains(");") {
                            if line.contains(".LUT(") {
                                if line.contains('h') {
                                    lut_line += "0x";
                                }
                                let mut extracted_lut_const = String::new();
                                if let Some(index_h) = line.find('h') {
                                    if let Some(index_close_paren) = line.find(')') {
                                        let index_separator = index_h;
                                        extracted_lut_const = line
                                            [index_separator + 1..index_close_paren]
                                            .to_string();
                                    }
                                } else if let Some(index_d) = line.find('d') {
                                    if let Some(index_close_paren) = line.find(')') {
                                        let index_separator = index_d;
                                        extracted_lut_const = line
                                            [index_separator + 1..index_close_paren]
                                            .to_string();
                                    }
                                }
                                if !extracted_lut_const.is_empty() {
                                    lut_line += &extracted_lut_const;
                                } else {
                                    println!("WARNING: No constant for LUT");
                                }
                                lut_line += ", ";
                            } else if line.contains(".A(") {
                                let (start_idx, end_idx) = {
                                    if !line.contains('{') {
                                        // single val
                                        (
                                            line.find('(').unwrap_or(0) + 1,
                                            line.find(')').unwrap_or(0),
                                        )
                                    } else {
                                        (
                                            line.find('{').unwrap_or(0) + 1,
                                            line.find('}').unwrap_or(0),
                                        )
                                    }
                                };
                                let in_vals_vec: Vec<String> = line[start_idx..end_idx]
                                    .split(',')
                                    .map(|v| v.trim().to_string())
                                    .collect();

                                for in_val in in_vals_vec {
                                    let mut tmp_name = in_val;
                                    while wire_dict.contains_key(&tmp_name) {
                                        tmp_name = wire_dict[&tmp_name].to_string();
                                        if outputs.contains(&tmp_name.to_string()) {
                                            break;
                                        }
                                    }
                                    lut_line += tmp_name.as_str();
                                    lut_line += ", ";
                                }
                            } else if line.contains(".Y(") {
                                let start_idx = line.find('(').unwrap_or(0) + 1;
                                let end_idx = line.find(')').unwrap_or(0);
                                let out_value_str = &line[start_idx..end_idx];
                                let mut out_value = &out_value_str.trim().to_string();
                                while wire_dict.contains_key(out_value) {
                                    out_value = &wire_dict[out_value];
                                    if outputs.contains(&out_value.to_string()) {
                                        break;
                                    }
                                }
                                lut_line += out_value;
                                lut_line += ");";
                            }
                            line = lines
                                .next()
                                .expect("Failed to read line")
                                .unwrap()
                                .trim()
                                .to_owned();
                        }
                        gates.push(lut_line.to_string());
                    } else {
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
                                let mut wire_name = line
                                    [line.find('(').unwrap() + 1..line.find(')').unwrap()]
                                    .trim_end_matches(';');

                                while wire_dict.contains_key(wire_name) {
                                    wire_name = &wire_dict[wire_name];
                                    if outputs.contains(&wire_name.to_string()) {
                                        break;
                                    }
                                }
                                gate_line += wire_name;
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
                    }
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
    if !multi_bit_outputs.is_empty() && arith {
        out_writer
            .write_all((multi_bit_outputs.join("\n") + "\n").as_bytes())
            .expect("Failed to write line");
    }
    if !outputs.is_empty() && !arith {
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

    // Check for direct Input -> Output connections and constants
    let mut buf_idx = 0;
    for (in_wire, out_wire_vec) in leftover_dict.iter() {
        for out_wire in out_wire_vec {
            if inputs.contains(in_wire) && outputs.contains(out_wire) {
                out_writer
                    .write_all(
                        ("  buf b_".to_owned()
                            + &buf_idx.to_string()
                            + "_("
                            + in_wire
                            + ", "
                            + out_wire
                            + ");\n")
                            .as_bytes(),
                    )
                    .expect("Failed to write line");
                buf_idx += 1;
            } else if in_wire.contains("'h") {
                if in_wire.contains('0') {
                    out_writer
                        .write_all(
                            ("  czero b_".to_owned()
                                + &buf_idx.to_string()
                                + "_("
                                + out_wire
                                + ");\n")
                                .as_bytes(),
                        )
                        .expect("Failed to write line");
                } else {
                    out_writer
                        .write_all(
                            ("  cone b_".to_owned()
                                + &buf_idx.to_string()
                                + "_("
                                + out_wire
                                + ");\n")
                                .as_bytes(),
                        )
                        .expect("Failed to write line");
                }
                buf_idx += 1;
            }
        }
    }
    for (in_wire, out_wire) in wire_dict.iter() {
        if inputs.contains(in_wire) && outputs.contains(out_wire) {
            out_writer
                .write_all(
                    ("  buf b_".to_owned()
                        + &buf_idx.to_string()
                        + "_("
                        + in_wire
                        + ", "
                        + out_wire
                        + ");\n")
                        .as_bytes(),
                )
                .expect("Failed to write line");
            buf_idx += 1;
        } else if out_wire.contains("'h") {
            if out_wire.contains('0') {
                out_writer
                    .write_all(
                        ("  czero b_".to_owned() + &buf_idx.to_string() + "_(" + in_wire + ");\n")
                            .as_bytes(),
                    )
                    .expect("Failed to write line");
            } else {
                out_writer
                    .write_all(
                        ("  cone b_".to_owned() + &buf_idx.to_string() + "_(" + in_wire + ");\n")
                            .as_bytes(),
                    )
                    .expect("Failed to write line");
            }
            buf_idx += 1;
        } else if in_wire.contains("'h") {
            if in_wire.contains('0') {
                out_writer
                    .write_all(
                        ("  czero b_".to_owned() + &buf_idx.to_string() + "_(" + out_wire + ");\n")
                            .as_bytes(),
                    )
                    .expect("Failed to write line");
            } else {
                out_writer
                    .write_all(
                        ("  cone b_".to_owned() + &buf_idx.to_string() + "_(" + out_wire + ");\n")
                            .as_bytes(),
                    )
                    .expect("Failed to write line");
            }
            buf_idx += 1;
        }
    }

    out_writer
        .write_all(b"\nendmodule\n")
        .expect("Failed to write line");
}

fn main() {
    let (in_file_name, out_file_name, arith) = parse_args();

    let (wire_dict, leftover_maps) = build_assign_dict(&in_file_name);
    convert_verilog(
        &in_file_name,
        &out_file_name,
        &wire_dict,
        &leftover_maps,
        arith,
    );
}

#[test]
fn test_get_multibit_assign_wires() {
    let prompts = vec![
        "assign alpha[1:0] = { 1'h0, N1[0] };",
        r#"assign { t [31] , t [ 30 ] , t[ 29] , t[28] , t[27] , t[26] , t[25], t[24] , t[23] ,
             t[22] , t[21] , t[20] , t[19] , t[18] , t[17], t[16] , t[15] , t[14] , t[13] , t[12] ,
             t[11] , t[10] , t[9] , t[8] , t[7] , t[6] , t[5] , t[4] , t[3] , t[2] , t[1] , t[0]  } 
             =  { N0 [ 29 : 0 ], 2'h0 };"#,
        r#"assign { t[31] , t[30] , t[29] , t[28] , t[27] , t[26] , t[25] , t[24] , t[23] , t[22] ,
             t[21] , t[20] , t[19] , t[18] , t[17] , t[16] , t[15] , t[14] , t[13] , t[12] , t[11] ,
              t[10] , t[9] , t[8] , t[7] , t[6] , t[5] , t[4] , t[3] , t[2] , t[1] , t[0]  } = 
              { 1'hx, beta_3[31:3], 1'h0, N1[0] };"#,
    ];
    let left_result = vec![
        vec!["alpha[1]", "alpha[0]"],
        vec![
            "t[31]", "t[30]", "t[29]", "t[28]", "t[27]", "t[26]", "t[25]", "t[24]", "t[23]",
            "t[22]", "t[21]", "t[20]", "t[19]", "t[18]", "t[17]", "t[16]", "t[15]", "t[14]",
            "t[13]", "t[12]", "t[11]", "t[10]", "t[9]", "t[8]", "t[7]", "t[6]", "t[5]", "t[4]",
            "t[3]", "t[2]", "t[1]", "t[0]",
        ],
        vec![
            "t[31]", "t[30]", "t[29]", "t[28]", "t[27]", "t[26]", "t[25]", "t[24]", "t[23]",
            "t[22]", "t[21]", "t[20]", "t[19]", "t[18]", "t[17]", "t[16]", "t[15]", "t[14]",
            "t[13]", "t[12]", "t[11]", "t[10]", "t[9]", "t[8]", "t[7]", "t[6]", "t[5]", "t[4]",
            "t[3]", "t[2]", "t[1]", "t[0]",
        ],
    ];
    let right_result = vec![
        vec!["1'h0", "N1[0]"],
        vec![
            "N0[29]", "N0[28]", "N0[27]", "N0[26]", "N0[25]", "N0[24]", "N0[23]", "N0[22]",
            "N0[21]", "N0[20]", "N0[19]", "N0[18]", "N0[17]", "N0[16]", "N0[15]", "N0[14]",
            "N0[13]", "N0[12]", "N0[11]", "N0[10]", "N0[9]", "N0[8]", "N0[7]", "N0[6]", "N0[5]",
            "N0[4]", "N0[3]", "N0[2]", "N0[1]", "N0[0]", "1'h0", "1'h0",
        ],
        vec![
            "1'hx",
            "beta_3[31]",
            "beta_3[30]",
            "beta_3[29]",
            "beta_3[28]",
            "beta_3[27]",
            "beta_3[26]",
            "beta_3[25]",
            "beta_3[24]",
            "beta_3[23]",
            "beta_3[22]",
            "beta_3[21]",
            "beta_3[20]",
            "beta_3[19]",
            "beta_3[18]",
            "beta_3[17]",
            "beta_3[16]",
            "beta_3[15]",
            "beta_3[14]",
            "beta_3[13]",
            "beta_3[12]",
            "beta_3[11]",
            "beta_3[10]",
            "beta_3[9]",
            "beta_3[8]",
            "beta_3[7]",
            "beta_3[6]",
            "beta_3[5]",
            "beta_3[4]",
            "beta_3[3]",
            "1'h0",
            "N1[0]",
        ],
    ];

    for (p, (left, right)) in prompts
        .iter()
        .zip(left_result.iter().zip(right_result.iter()))
    {
        let cleaned_prompt = p.replace("assign", "").replace(";", "").replace(" ", "");
        let (l, r) = get_multibit_assign_wires(&cleaned_prompt);

        for (l, expected_l) in l.iter().zip(left.iter()) {
            assert_eq!(*l, *expected_l);
        }
        for (r, expected_r) in r.iter().zip(right.iter()) {
            assert_eq!(*r, *expected_r);
        }
    }
}
