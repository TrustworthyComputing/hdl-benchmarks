module chi_squared (A, B, C, alpha, beta_1, beta_2, beta_3);

input [15:0] A;
input [15:0] B;
input [15:0] C;
output [15:0] alpha;
output [15:0] beta_1;
output [15:0] beta_2;
output [15:0] beta_3;
wire [15:0] tmp_0;
wire [15:0] tmp_1;
wire [15:0] tmp_2;
wire [15:0] tmp_3;
wire [15:0] tmp_4;
wire [15:0] tmp_5;
wire [15:0] tmp_6;
wire [15:0] tmp_7;
wire [15:0] tmp_8;

// Compute alpha
assign tmp_0 = 4 * A;
assign tmp_2 = tmp_0 * C;
assign tmp_1 = B * B;
assign alpha = tmp_2 - tmp_1;

// Compute beta_1
assign tmp_3 = 2 * A;
assign tmp_4 = tmp_3 + B;
assign tmp_5 = tmp_4 * tmp_4;
assign beta_1 = tmp_5 * 2;

// Compute beta_2
assign tmp_6 = 2 * C;
assign tmp_7 = tmp_6 + B;
assign beta_2 = tmp_4 * tmp_7;

// Compute beta_3
assign tmp_8 = tmp_7 * tmp_7;
assign beta_3 = tmp_0 * 2;

endmodule
