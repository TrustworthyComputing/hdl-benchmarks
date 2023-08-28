module invalid (A, B, C);
  wire tmp_0,  tmp_1,  tmp_2,  tmp_3;
  input [15:0] A, B;
  output [15:0] C;
  mult m1(5, A, tmp_0);
  mult m2(10, B, tmp_1);
  lut l3(0x5, tmp_0, C);
endmodule
