<h1 align="center">HDL Benchmarks <a href="https://github.com/jimouris/helm/blob/main/LICENSE"><img src="https://img.shields.io/badge/license-MIT-blue.svg"></a> </h1>

<h2 align="center">Verilog designs and their synthesized netlists</h2>


## [Designs](./designs/)
These are behavioral Verilog files that can serve as inputs to a logic synthesis
tool to generate a corresponding netlist.
* ISCAS'85 benchmarks: `c*.v`
* ISCAS'89 benchmarks: `s*.v`


## [Netlists](./netlists/)
Our preprocessor expects a netlist written in structural Verilog and will clean up the
netlist to make it easier for frameworks to read. The structural verilog can be
generated with the following [Yosys](https://github.com/YosysHQ/yosys) commands:

For gates:
```shell
read_verilog [MY_DESIGN].v
synth
abc -g simple,-MUX 
splitnets
write_verilog -noexpr [MY_DESIGN].struct.v
```

For 2:1 LUTs:
```shell
read_verilog [MY_DESIGN].v
synth
abc -luts 2
nlutmap -luts 0,10000000,0 (only applicable to LUTs with LBB)
splitnets
write_verilog -noexpr [MY_DESIGN].struct.v
```


## [Processed Netlists](./processed-netlists/)
These files are generated automatically by the preprocessor. To run our 
preprocessor run: `cargo run --bin preprocessor --release -- --input ./netlists/c17.v`


## [Test Cases](./test-cases/)
These files represent test cases for a corresponding netlist that can be useful
for simulating circuits in software.
* Inputs: `filename.inputs.csv`
* Outputs: `filename.outputs.csv`



<p align="center">
    <img src="./logos/twc.png" height="20%" width="20%">
</p>
<h4 align="center">Trustworthy Computing Group</h4>
