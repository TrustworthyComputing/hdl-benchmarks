use std::collections::{HashMap, HashSet};

use hdl_benchmarks::get_multibit_assign_wires;

/// output dest; // single-bit source, single-bit dest
/// assign dest = source;
#[test]
fn one_to_one_assignment_1() {
    let multibit_ports: HashMap<String, u32> = HashMap::new();
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
