module foo(in1, in2, in3, o1, o2, o3);

    input [1:0] in1;
    input [1:0] in2;
    input [1:0] in3;
    output [1:0] o1;
    output [1:0] o2;
    output [1:0] o3;

    wire _0743713_;
    wire _0674559_;
    wire _0684774_;

    \$lut  #(
      .LUT(4'h8),
      .WIDTH(32'd2)
    ) _1556823_ (
      .A(in1[1:0]),
      .Y(_0743713_)
    );

    \$lut  #(
      .LUT(16'h8),
      .WIDTH(32'd2)
    ) _1494339_ (
      .A(in2[1:0], in1[1:0]),
      .Y(_0674559_)
    );

    \$lut  #(
      .LUT(4'h8),
      .WIDTH(32'd2)
    ) _1503582_ (
      .A(in3[1:0]),
      .Y(_0684774_)
    );

    assign o1 = _0743713_;
    assign o2 = _0674559_;
    assign o3 = _0684774_;
endmodule



