module chi_squared (N0, N1, N2, alpha, beta_1, beta_2, beta_3);
  wire tmp_0, tmp_1, tmp_2, tmp_3, tmp_4, tmp_5, tmp_6, tmp_7, tmp_8, tmp_9;
  input [15:0] N0;
  input [15:0] N1;
  input [15:0] N2;
  output [15:0] alpha;
  output [15:0] beta_1;
  output [15:0] beta_2;
  output [15:0] beta_3;
  mult m1(4, N0, tmp_0);
  mult m2(tmp_0, N2, tmp_2);
  mult m3(N1, N1, tmp_1);
  sub s4(tmp_2, tmp_1, tmp_9);
  mult m5(tmp_9, tmp_9, alpha);
  mult m6(2, N0, tmp_3);
  add a7(tmp_3, N1, tmp_4);
  mult m8(tmp_4, tmp_4, tmp_5);
  mult m9(tmp_5, 2, beta_1);
  mult m10(2, N2, tmp_6);
  add a11(tmp_6, N1, tmp_7);
  mult m12(tmp_4, tmp_7, beta_2);
  mult m13(tmp_7, tmp_7, tmp_8);
  mult m14(tmp_8, 2, beta_3);

endmodule
