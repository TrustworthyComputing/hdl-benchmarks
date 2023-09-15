use std::collections::{HashMap, HashSet};
use std::fs::File;
use std::io::{BufRead, BufReader};

use hdl_benchmarks::{build_assign_dict, convert_verilog, get_multibit_assign_wires};

/// output dest; // single-bit source, single-bit dest
/// assign dest = source;
#[test]
fn one_to_one_assignment_1() {
    let multibit_ports: HashMap<String, u32> = HashMap::new();
    let mut wire_to_port: HashMap<String, String> = HashMap::new();
    let mut leftover_maps: HashMap<String, Vec<String>> = HashMap::new();
    let mut assign_dict: HashMap<String, String> = HashMap::new();
    let output_ports: HashSet<String> = HashSet::new();
    let prompt = "assign dest = source;";
    get_multibit_assign_wires(
        &prompt,
        &multibit_ports,
        &mut wire_to_port,
        &mut leftover_maps,
        &mut assign_dict,
        &output_ports,
    );
    println!("assign dict: {:?}", assign_dict);

    assert_eq!(assign_dict["dest"], "source");
}

/// output dest; // single-bit source, single-bit dest
/// assign dest = source;
#[test]
#[should_panic(expected = "Can't assign multi-bit port to single-bit port!")]
fn one_to_one_assignment_incorrect() {
    let mut multibit_ports: HashMap<String, u32> = HashMap::new();
    // define that source is multi-bit, while dest is single bit
    multibit_ports.insert("source".to_string(), 8);
    let mut wire_to_port: HashMap<String, String> = HashMap::new();
    let mut leftover_maps: HashMap<String, Vec<String>> = HashMap::new();
    let mut assign_dict: HashMap<String, String> = HashMap::new();
    let output_ports: HashSet<String> = HashSet::new();
    //output_ports.insert("dest".to_string());
    let prompt = "assign dest = source;";
    get_multibit_assign_wires(
        &prompt,
        &multibit_ports,
        &mut wire_to_port,
        &mut leftover_maps,
        &mut assign_dict,
        &output_ports,
    );
}

/// output dest[7:0]; // multi-bit source, multi-bit dest
/// assign dest = source;
#[test]
fn one_to_one_assignment_2() {
    let bit_width: u32 = 8;
    let mut multibit_ports: HashMap<String, u32> = HashMap::new();
    multibit_ports.insert("source".to_string(), bit_width);
    multibit_ports.insert("dest".to_string(), bit_width);
    let mut wire_to_port: HashMap<String, String> = HashMap::new();
    let mut leftover_maps: HashMap<String, Vec<String>> = HashMap::new();
    let mut assign_dict: HashMap<String, String> = HashMap::new();
    let mut output_ports: HashSet<String> = HashSet::new();
    output_ports.insert("dest".to_string());
    let prompt = "assign dest = source;";
    get_multibit_assign_wires(
        &prompt,
        &multibit_ports,
        &mut wire_to_port,
        &mut leftover_maps,
        &mut assign_dict,
        &output_ports,
    );
    for i in 0..bit_width {
        let single_bit_in = "source".to_owned() + "[" + &i.to_string() + "]";
        let single_bit_out = "dest".to_owned() + "[" + &i.to_string() + "]";
        assert_eq!(assign_dict[&single_bit_out], single_bit_in);
    }
}

/// assign { dest0, dest1, ..., dest7 } = source;
#[test]
fn one_to_many_assignment_1() {
    let mut multibit_ports: HashMap<String, u32> = HashMap::new();
    multibit_ports.insert("source".to_string(), 8);
    let mut wire_to_port: HashMap<String, String> = HashMap::new();
    let mut leftover_maps: HashMap<String, Vec<String>> = HashMap::new();
    let mut assign_dict: HashMap<String, String> = HashMap::new();
    let output_ports: HashSet<String> = HashSet::new();
    let prompt = "assign { d[7] , d[6] , d[5] , d[4] , d[3] , d[2] , d[1] , d[0] } = source;";
    get_multibit_assign_wires(
        &prompt,
        &multibit_ports,
        &mut wire_to_port,
        &mut leftover_maps,
        &mut assign_dict,
        &output_ports,
    );
    for i in 0..8 {
        let single_bit_in = "source".to_owned() + "[" + &i.to_string() + "]";
        let single_bit_out = "d".to_owned() + "[" + &i.to_string() + "]";
        assert_eq!(assign_dict[&single_bit_out], single_bit_in);
    }
}

/// assign { dest0, dest1, ..., dest7 } = source[7:0];
#[test]
fn one_to_many_assignment_2() {
    let multibit_ports: HashMap<String, u32> = HashMap::new();
    let mut wire_to_port: HashMap<String, String> = HashMap::new();
    let mut leftover_maps: HashMap<String, Vec<String>> = HashMap::new();
    let mut assign_dict: HashMap<String, String> = HashMap::new();
    let output_ports: HashSet<String> = HashSet::new();
    let prompt = "assign { d[7] , d[6] , d[5] , d[4] , d[3] , d[2] , d[1] , d[0] } = source[7:0];";
    get_multibit_assign_wires(
        &prompt,
        &multibit_ports,
        &mut wire_to_port,
        &mut leftover_maps,
        &mut assign_dict,
        &output_ports,
    );
    for i in 0..8 {
        let single_bit_in = "source".to_owned() + "[" + &i.to_string() + "]";
        let single_bit_out = "d".to_owned() + "[" + &i.to_string() + "]";
        assert_eq!(assign_dict[&single_bit_out], single_bit_in);
    }
}

/// assign { dest0, dest1, ..., dest7 } = { source0, source1, ..., source7 };
#[test]
fn many_to_many_assignment_1() {
    let multibit_ports: HashMap<String, u32> = HashMap::new();
    let mut wire_to_port: HashMap<String, String> = HashMap::new();
    let mut leftover_maps: HashMap<String, Vec<String>> = HashMap::new();
    let mut assign_dict: HashMap<String, String> = HashMap::new();
    let output_ports: HashSet<String> = HashSet::new();
    let prompt = "assign { d[7], d[6], d[5], d[4], d[3], d[2], d[1], d[0] } =
        { s[7], s[6], s[5], s[4], s[3], s[2], s[1], s[0] };";
    get_multibit_assign_wires(
        &prompt,
        &multibit_ports,
        &mut wire_to_port,
        &mut leftover_maps,
        &mut assign_dict,
        &output_ports,
    );
    for i in 0..8 {
        let single_bit_in = "s".to_owned() + "[" + &i.to_string() + "]";
        let single_bit_out = "d".to_owned() + "[" + &i.to_string() + "]";
        assert_eq!(assign_dict[&single_bit_out], single_bit_in);
    }
}

/// assign { dest0, dest1, ..., dest7 } = { source0, source1, source[2:7] };
#[test]
fn many_to_many_assignment_2() {
    let multibit_ports: HashMap<String, u32> = HashMap::new();
    let mut wire_to_port: HashMap<String, String> = HashMap::new();
    let mut leftover_maps: HashMap<String, Vec<String>> = HashMap::new();
    let mut assign_dict: HashMap<String, String> = HashMap::new();
    let output_ports: HashSet<String> = HashSet::new();
    let prompt = "assign { d[7], d[6], d[5], d[4], d[3], d[2], d[1], d[0] } =
        { s[7], 1'h0, s[5:0] };";
    get_multibit_assign_wires(
        &prompt,
        &multibit_ports,
        &mut wire_to_port,
        &mut leftover_maps,
        &mut assign_dict,
        &output_ports,
    );
    assert_eq!(assign_dict["d[7]"], "s[7]");
    assert_eq!(assign_dict["d[6]"], "1'h0");
    for i in 0..6 {
        let single_bit_in = "s".to_owned() + "[" + &i.to_string() + "]";
        let single_bit_out = "d".to_owned() + "[" + &i.to_string() + "]";
        assert_eq!(assign_dict[&single_bit_out], single_bit_in);
    }
}

/// assign dest[1:0]  = { 1'h0, source[0] };
#[test]
fn many_to_many_assignment_3() {
    let multibit_ports: HashMap<String, u32> = HashMap::new();
    let mut wire_to_port: HashMap<String, String> = HashMap::new();
    let mut leftover_maps: HashMap<String, Vec<String>> = HashMap::new();
    let mut assign_dict: HashMap<String, String> = HashMap::new();
    let output_ports: HashSet<String> = HashSet::new();
    let prompt = "assign d[1:0]  = { 1'h0, s [0] };";
    get_multibit_assign_wires(
        &prompt,
        &multibit_ports,
        &mut wire_to_port,
        &mut leftover_maps,
        &mut assign_dict,
        &output_ports,
    );
    assert_eq!(assign_dict["d[1]"], "1'h0");
    assert_eq!(assign_dict["d[0]"], "s[0]");
}

/// assign dest  = { 1'h0, source[0] };
#[test]
fn many_to_many_assignment_4() {
    let mut multibit_ports: HashMap<String, u32> = HashMap::new();
    multibit_ports.insert("o_CRC".to_string(), 32);
    let mut wire_to_port: HashMap<String, String> = HashMap::new();
    let mut leftover_maps: HashMap<String, Vec<String>> = HashMap::new();
    let mut assign_dict: HashMap<String, String> = HashMap::new();
    let output_ports: HashSet<String> = HashSet::new();
    let prompt = "  assign o_CRC = { r_CRC[31] , r_CRC[30] , r_CRC[29] , r_CRC[28] , r_CRC[27] , r_CRC[26] , r_CRC[25] , r_CRC[24] , r_CRC[23] , r_CRC[22] , r_CRC[21] , r_CRC[20] , r_CRC[19] , r_CRC[18] , r_CRC[17] , r_CRC[16] , r_CRC[15] , r_CRC[14] , r_CRC[13] , r_CRC[12] , r_CRC[11] , r_CRC[10] , r_CRC[9] , r_CRC[8] , r_CRC[7] , r_CRC[6] , r_CRC[5] , r_CRC[4] , r_CRC[3] , r_CRC[2] , r_CRC[1] , r_CRC[0]  };";
    get_multibit_assign_wires(
        &prompt,
        &multibit_ports,
        &mut wire_to_port,
        &mut leftover_maps,
        &mut assign_dict,
        &output_ports,
    );
    assert_eq!(assign_dict["o_CRC[31]"], "r_CRC[31]");
    assert_eq!(assign_dict["o_CRC[12]"], "r_CRC[12]")
}

/// Test for build_assign_dict function for a file with gates.
#[test]
fn build_assign_dict_v2_euclidean_gates() {
    let (assign_dict, leftover_maps) =
        build_assign_dict(&"./netlists/v2_euclidean_gates.v".to_string());

    assert_eq!(assign_dict.len(), 96);
    assert_eq!(leftover_maps.len(), 0);

    assert_eq!(assign_dict["_3119_"], "v_1[1]");
    assert_eq!(assign_dict["_1345_"], "\\dist[3]");
    assert_eq!(assign_dict["_3070_"], "u_0[15]");
}

/// Test for build_assign_dict function for a file with arithmetic operations.
#[test]
fn build_assign_dict_v2_euclidean_arithmetic() {
    let (assign_dict, leftover_maps) =
        build_assign_dict(&"./designs/v2-euclidean-distance.v".to_string());

    // Maps should be empty for arithmetic mode
    assert_eq!(assign_dict.len(), 0);
    assert_eq!(leftover_maps.len(), 0);
}

/// \$lut  #(.LUT(4'h8),.WIDTH(32'd2)) _1556823_ (.A(b_4_1[1:0]),.Y(_0743713_));
/// => lut lut_gate[GATE_ID](0x8, b_4_1[1], b_4_1[0], _0743713_);
#[test]
fn lut_array_in_argument() {
    let prompt = "\\$lut  #(.LUT(4'h8),.WIDTH(32'd2)) _1556823_ (.A(b_4_1[1:0]),.Y(_0743713_));";

    println!("prompt: {:?}", prompt);

    let wire_dict = HashMap::new();
    let leftover_dict = HashMap::new();
    let output_filename = "./netlists/test_output.v".to_string();

    convert_verilog(
        &"./netlists/lut-multibit-input-test.v".to_string(),
        &output_filename,
        &wire_dict,
        &leftover_dict,
        false,
    );
    let converted_file = File::open(output_filename).expect("Failed to open file");
    let reader = BufReader::new(converted_file);
    let lines: Vec<String> = reader
        .lines()
        .map(|line| line.expect("Failed to read line"))
        .collect();

    assert!(lines.contains(&"  lut lut_gate1(0x8, in1[1], in1[0], _0743713_);".to_string()));
    assert!(lines
        .contains(&"  lut lut_gate2(0x8, in2[1], in2[0], in1[1], in1[0], _0674559_);".to_string()));
}
