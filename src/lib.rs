use clap::{Arg, ArgAction, Command};
use std::collections::{hash_map::Entry, HashMap, HashSet};
use std::fs::File;
use std::io::{BufRead, BufReader, BufWriter, Write};
use std::path::Path;

pub fn parse_args() -> (String, String, bool) {
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

/// Single bit assign:
/// 1) assign dest = source;
/// 2) assign dest[WIRE_ID] = source;
/// 3) assign dest [WIRE_ID] = source;
/// 4) assign dest = source [WIRE_ID];
/// 5) assign dest = source[WIRE_ID];
/// 6) assign dest [WIRE_ID] = source [WIRE_ID];
/// 7) assign dest [WIRE_ID] = source[WIRE_ID];
/// 8) assign dest[WIRE_ID] = source [WIRE_ID];
/// 9) assign dest[WIRE_ID] = source[WIRE_ID];
///
/// Multi-bit assign:
/// 1) assign { w1, w2, w3, ..., w16 } = { x1, x2, x3, ..., x16 };
/// 2) assign { w1, w2, w3, ..., w16 } = x[15:0];
/// 3) assign { w1, w2, w3, ..., w16 } = { x[13:0], 1h'0, x[0]};
/// 4) assign w[15:0] = { x1, x2, x3, ..., x16 };
/// 5) assign { w[13:0], w[15:14] } = { x1, x2, x3, ..., x16 };
/// 6) assign w = { x1, x2, x3, ..., x16 };
/// 7) assign w = x;
/// 8) assign w[15:0] = x;
/// 9) assign w = x[15:0];
pub fn get_multibit_assign_wires(
    unsanitized_line: &str,
    multibit_ports: &HashMap<String, u32>,
    wire_to_port: &mut HashMap<String, String>,
    leftover_maps: &mut HashMap<String, Vec<String>>,
    assign_dict: &mut HashMap<String, String>,
    output_ports: &HashSet<String>,
) {
    let line: String = unsanitized_line
        .replace("assign", "")
        .replace([';', ' '], "");

    if line.contains(':') || line.contains('{') {
        let mut left_side = Vec::new();
        let mut right_side = Vec::new();

        // Split the line by '='
        let s = line.replace(&[';', ' ', '{', '}'][..], "");

        // Extract the left and right sides
        let parts: Vec<&str> = s.split('=').collect();
        let left = parts[0];
        let right = parts[1];

        assert_eq!(parts.len(), 2);

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

        // (left_side, right_side)
        let mut new_left_side = Vec::new();
        let mut new_right_side = Vec::new();
        // if left_side.len() != right_side.len() {
        for j in 0..left_side.len() {
            if !left_side[j].contains('[') && multibit_ports.contains_key(&left_side[j]) {
                let tmp_id = left_side[j].clone();
                for i in (0..multibit_ports[&tmp_id]).rev() {
                    new_left_side.push(tmp_id.clone() + "[" + &i.to_string() + "]");
                }
            } else {
                new_left_side.push(left_side[j].clone());
            }
        }
        for j in 0..right_side.len() {
            if !right_side[j].contains('[') && multibit_ports.contains_key(&right_side[j]) {
                let tmp_id = right_side[j].clone();
                for i in (0..multibit_ports[&tmp_id]).rev() {
                    new_right_side.push(tmp_id.clone() + "[" + &i.to_string() + "]");
                }
            } else {
                new_right_side.push(right_side[j].clone());
            }
        }
        assert_eq!(
            new_left_side.len(),
            new_right_side.len(),
            "assign statement LHS is not the same size as RHS!"
        );
        for i in 0..new_left_side.len() {
            let output = new_left_side[i].clone();
            let input = new_right_side[i].clone();

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
            if let Entry::Vacant(e) = assign_dict.entry(output.clone()) {
                e.insert(input);
            } else {
                leftover_maps
                    .entry(output.clone())
                    .or_insert(Vec::new())
                    .push(input.clone());
            }
        }
    } else {
        // single-bit or single-port assign
        let tokens: Vec<&str> = line.split('=').collect();
        let output = tokens[0].to_string();
        let input = tokens[1].to_string();
        let is_multibit_port = if !multibit_ports.contains_key(&output) {
            assert!(
                !multibit_ports.contains_key(&input),
                "Can't assign multi-bit port to single-bit port!"
            );
            false
        } else {
            true
        };
        if output_ports.contains(&output) {
            if !is_multibit_port {
                // add mapping to convert wires directly to output ports
                // if mapping already exists, save in leftover_maps
                // assign y = x;
                // assign z = x;
                if wire_to_port.contains_key(&input) {
                    leftover_maps
                        .entry(input.clone())
                        .or_insert(Vec::new())
                        .push(output.clone());
                } else {
                    wire_to_port.insert(input.clone(), output.clone());
                }
            } else {
                assert!(
                    multibit_ports.contains_key(&input),
                    "Can't assign single-bit port to multi-bit port!"
                );
                assert_eq!(
                    multibit_ports[&output], multibit_ports[&input],
                    "Multi-bit ports need to be the same length!"
                );
                for i in 0..multibit_ports[&output] {
                    let single_wire_out = output.clone() + "[" + &i.to_string() + "]";
                    let single_wire_in = input.clone() + "[" + &i.to_string() + "]";
                    if wire_to_port.contains_key(&single_wire_in) {
                        leftover_maps
                            .entry(single_wire_in.clone())
                            .or_insert(Vec::new())
                            .push(single_wire_out.clone());
                    } else {
                        wire_to_port.insert(single_wire_in.clone(), single_wire_out.clone());
                    }
                    if let Entry::Vacant(e) = assign_dict.entry(single_wire_out.clone()) {
                        e.insert(single_wire_in);
                    } else {
                        leftover_maps
                            .entry(single_wire_out.clone())
                            .or_insert(Vec::new())
                            .push(single_wire_in.clone());
                    }
                }
            }
        }

        // assign x = 1'h0
        // assign y = 1'h0
        if !is_multibit_port {
            if let Entry::Vacant(e) = assign_dict.entry(output.clone()) {
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

pub fn build_assign_dict(
    in_file_name: &String,
) -> (HashMap<String, String>, HashMap<String, Vec<String>>) {
    let in_file = File::open(in_file_name).expect("Failed to open file");
    let reader = BufReader::new(in_file);
    let mut assign_dict: HashMap<String, String> = HashMap::new();
    let mut wire_to_port: HashMap<String, String> = HashMap::new();
    let mut output_ports = HashSet::new();
    let mut multibit_ports: HashMap<String, u32> = HashMap::new();
    let mut leftover_maps: HashMap<String, Vec<String>> = HashMap::new();
    for line in reader.lines() {
        let line = line.expect("Failed to read line").trim().to_owned();
        if line.contains("output") || line.contains("input") || line.contains("wire") {
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
                        let tmp_name = line
                            .split(' ')
                            .nth(2)
                            .unwrap()
                            .trim_end_matches(';')
                            .to_string();
                        multibit_ports.insert(tmp_name.clone(), number as u32 + 1);
                        if line.contains("output") {
                            for curr_wire in 0..number + 1 {
                                output_ports
                                    .insert(tmp_name.clone() + "[" + &curr_wire.to_string() + "]");
                            }
                        }
                    } else {
                        println!("Error: Invalid number format.");
                    }
                }
            } else if line.contains("output") {
                // only do that for output ports
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
            let cleaned_line: String = line.replace("assign", "").replace([';', ' '], "");
            get_multibit_assign_wires(
                &cleaned_line,
                &multibit_ports,
                &mut wire_to_port,
                &mut leftover_maps,
                &mut assign_dict,
                &output_ports,
            );
        }
    }
    postprocess_assign_dict(&mut assign_dict, wire_to_port);

    (assign_dict, leftover_maps)
}

pub fn convert_verilog(
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
                    wires.push(String::from(token.trim_matches(',').trim_end_matches(';')));
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
                                    if in_val.contains(':') {
                                        // Initialize variables to store the extracted numbers
                                        let mut extracted_number1: u32 = 0;
                                        let mut extracted_number2: u32 = 0;
                                        // foo[15:13]
                                        // Find the index of '[' and ':'
                                        if let Some(open_bracket_idx) = in_val.find('[') {
                                            if let Some(colon_idx) = in_val.find(':') {
                                                // Check if '[' appears before ':'
                                                if open_bracket_idx < colon_idx {
                                                    // Extract the substring between '[' and ':'
                                                    let number1_str =
                                                        &in_val[open_bracket_idx + 1..colon_idx];

                                                    // Attempt to parse the first extracted substring as a u32
                                                    if let Ok(number) = number1_str.parse::<u32>() {
                                                        extracted_number1 = number;
                                                    }

                                                    // Find the index of ':]' (':' followed by ']')
                                                    if let Some(closing_bracket_idx) =
                                                        in_val.find(']')
                                                    {
                                                        // Extract the substring between ':' and ']'
                                                        let number2_str = &in_val
                                                            [colon_idx + 1..closing_bracket_idx];

                                                        // Attempt to parse the second extracted substring as a u32
                                                        if let Ok(number) =
                                                            number2_str.parse::<u32>()
                                                        {
                                                            extracted_number2 = number;
                                                        }
                                                    }
                                                }
                                            }
                                        }

                                        let wire_id: String =
                                            in_val.split('[').next().unwrap_or("").to_string();
                                        for i in (extracted_number2..=extracted_number1).rev() {
                                            let mut tmp_name =
                                                wire_id.clone() + "[" + &i.to_string() + "]";
                                            while wire_dict.contains_key(&tmp_name) {
                                                tmp_name = wire_dict[&tmp_name].to_string();
                                                if outputs.contains(&tmp_name.to_string()) {
                                                    break;
                                                }
                                            }
                                            lut_line += tmp_name.as_str();
                                            lut_line += ", ";
                                        }
                                    } else {
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
                                lut_line += &out_value.replace(' ', "");
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
                        let mut temp_line = String::new();
                        if is_dff {
                            let mut parts: Vec<&str> = gate_line.split(',').collect();
                            parts.pop(); // remove the last element from the vector
                            for part in parts {
                                let mut wire_name = part.trim().to_string();
                                while wire_dict.contains_key(&wire_name) {
                                    wire_name = wire_dict[&wire_name].clone();
                                    if outputs.contains(&wire_name) {
                                        break;
                                    }
                                }
                                temp_line += &(wire_name + ", ");
                            }
                            gate_line = temp_line.trim_end_matches(", ").to_string();
                            // gate_line = parts.join(","); // join the remaining elements with commas
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
