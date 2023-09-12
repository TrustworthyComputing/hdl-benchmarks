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
    get_multibit_assign_wires(&prompt, &multibit_ports, &mut wire_to_port, &mut leftover_maps, &mut assign_dict, &output_ports);
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
    get_multibit_assign_wires(&prompt, &multibit_ports, &mut wire_to_port, &mut leftover_maps, &mut assign_dict, &output_ports);
    println!("assign dict: {:?}", assign_dict);

    // TODO: should panic: catch the panic as correct test.
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
    get_multibit_assign_wires(&prompt, &multibit_ports, &mut wire_to_port, &mut leftover_maps, &mut assign_dict, &output_ports);
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
    get_multibit_assign_wires(&prompt, &multibit_ports, &mut wire_to_port, &mut leftover_maps, &mut assign_dict, &output_ports);

    // assert_eq!()
}

/// assign { dest0, dest1, ..., dest7 } = source[7:0];
#[test]
fn one_to_many_assignment_2() {
    let mut multibit_ports: HashMap<String, u32> = HashMap::new();
    multibit_ports.insert("source".to_string(), 8);
    let mut wire_to_port: HashMap<String, String> = HashMap::new();
    let mut leftover_maps: HashMap<String, Vec<String>> = HashMap::new();
    let mut assign_dict: HashMap<String, String> = HashMap::new();
    let output_ports: HashSet<String> = HashSet::new();
    let prompt = "assign { d[7] , d[6] , d[5] , d[4] , d[3] , d[2] , d[1] , d[0] } = source[7:0];";
    get_multibit_assign_wires(&prompt, &multibit_ports, &mut wire_to_port, &mut leftover_maps, &mut assign_dict, &output_ports);

    // assert_eq!()
}

/// assign { dest0, dest1, ..., dest7 } = { source0, source1, ..., source7 };
#[test]
fn many_to_many_assignment_1() {
    let mut multibit_ports: HashMap<String, u32> = HashMap::new();
    multibit_ports.insert("s".to_string(), 8);
    let prompt = r#"assign { d[7], d[6], d[5], d[4], d[3], d[2], d[1], d[0] } = 
        { s[7], s[6], s[5], s[4], s[3], s[2], s[1], s[0] };"#;

    // assert_eq!()

}

/// assign { dest0, dest1, ..., dest7 } = { source0, source1, source[2:7] };
#[test]
fn many_to_many_assignment_2() {
    let mut multibit_ports: HashMap<String, u32> = HashMap::new();
    multibit_ports.insert("N0".to_string(), 8);
    let prompt = r#"assign { d[7], d[6], d[5], d[4], d[3], d[2], d[1], d[0] } = 
        { s[7], 1'h0, s[5:0] };"#;

    // assert_eq!()
}


// #[test]
// fn test_get_multibit_assign_wires() {
//     let prompts = vec![
//         // output dest[31:0];
//         // input source[31:0];
//         // ...
//         // assign dest = source;
//         "assign alpha[1:0] = { 1'h0, N1[0] };",
//         r#"assign { t [31] , t [ 30 ] , t[ 29] , t[28] , t[27] , t[26] , t[25], t[24] , t[23] ,
//              t[22] , t[21] , t[20] , t[19] , t[18] , t[17], t[16] , t[15] , t[14] , t[13] , t[12] ,
//              t[11] , t[10] , t[9] , t[8] , t[7] , t[6] , t[5] , t[4] , t[3] , t[2] , t[1] , t[0]  } 
//              =  { N0 [ 29 : 0 ], 2'h0 };"#,
//         r#"assign { t[31] , t[30] , t[29] , t[28] , t[27] , t[26] , t[25] , t[24] , t[23] , t[22] ,
//              t[21] , t[20] , t[19] , t[18] , t[17] , t[16] , t[15] , t[14] , t[13] , t[12] , t[11] ,
//               t[10] , t[9] , t[8] , t[7] , t[6] , t[5] , t[4] , t[3] , t[2] , t[1] , t[0]  } = 
//               { 1'hx, beta_3[31:3], 1'h0, N1[0] };"#,
//     ];
//     let left_result = vec![
//         vec!["alpha[1]", "alpha[0]"],
//         vec![
//             "t[31]", "t[30]", "t[29]", "t[28]", "t[27]", "t[26]", "t[25]", "t[24]", "t[23]",
//             "t[22]", "t[21]", "t[20]", "t[19]", "t[18]", "t[17]", "t[16]", "t[15]", "t[14]",
//             "t[13]", "t[12]", "t[11]", "t[10]", "t[9]", "t[8]", "t[7]", "t[6]", "t[5]", "t[4]",
//             "t[3]", "t[2]", "t[1]", "t[0]",
//         ],
//         vec![
//             "t[31]", "t[30]", "t[29]", "t[28]", "t[27]", "t[26]", "t[25]", "t[24]", "t[23]",
//             "t[22]", "t[21]", "t[20]", "t[19]", "t[18]", "t[17]", "t[16]", "t[15]", "t[14]",
//             "t[13]", "t[12]", "t[11]", "t[10]", "t[9]", "t[8]", "t[7]", "t[6]", "t[5]", "t[4]",
//             "t[3]", "t[2]", "t[1]", "t[0]",
//         ],
//     ];
//     let right_result = vec![
//         vec!["1'h0", "N1[0]"],
//         vec![
//             "N0[29]", "N0[28]", "N0[27]", "N0[26]", "N0[25]", "N0[24]", "N0[23]", "N0[22]",
//             "N0[21]", "N0[20]", "N0[19]", "N0[18]", "N0[17]", "N0[16]", "N0[15]", "N0[14]",
//             "N0[13]", "N0[12]", "N0[11]", "N0[10]", "N0[9]", "N0[8]", "N0[7]", "N0[6]", "N0[5]",
//             "N0[4]", "N0[3]", "N0[2]", "N0[1]", "N0[0]", "1'h0", "1'h0",
//         ],
//         vec![
//             "1'hx",
//             "beta_3[31]",
//             "beta_3[30]",
//             "beta_3[29]",
//             "beta_3[28]",
//             "beta_3[27]",
//             "beta_3[26]",
//             "beta_3[25]",
//             "beta_3[24]",
//             "beta_3[23]",
//             "beta_3[22]",
//             "beta_3[21]",
//             "beta_3[20]",
//             "beta_3[19]",
//             "beta_3[18]",
//             "beta_3[17]",
//             "beta_3[16]",
//             "beta_3[15]",
//             "beta_3[14]",
//             "beta_3[13]",
//             "beta_3[12]",
//             "beta_3[11]",
//             "beta_3[10]",
//             "beta_3[9]",
//             "beta_3[8]",
//             "beta_3[7]",
//             "beta_3[6]",
//             "beta_3[5]",
//             "beta_3[4]",
//             "beta_3[3]",
//             "1'h0",
//             "N1[0]",
//         ],
//     ];

//     for (p, (left, right)) in prompts
//         .iter()
//         .zip(left_result.iter().zip(right_result.iter()))
//     {
//         let cleaned_prompt = p.replace("assign", "").replace(";", "").replace(" ", "");
//         let (l, r) = get_multibit_assign_wires(&cleaned_prompt);

//         for (l, expected_l) in l.iter().zip(left.iter()) {
//             assert_eq!(*l, *expected_l);
//         }
//         for (r, expected_r) in r.iter().zip(right.iter()) {
//             assert_eq!(*r, *expected_r);
//         }
//     }
// }
