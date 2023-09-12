use hdl_benchmarks::{build_assign_dict, convert_verilog, parse_args};

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
