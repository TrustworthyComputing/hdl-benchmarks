module chi_squared (N0, N1, N2, alpha, beta_1, beta_2, beta_3);

input [15:0] N0;
input [15:0] N1;
input [15:0] N2;
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
wire [15:0] tmp_9;

// Compute alpha
assign tmp_0 = 4 * N0;
assign tmp_2 = tmp_0 * N2;
assign tmp_1 = N1 * N1;
assign tmp_9 = tmp_2 - tmp_1;
assign alpha = tmp_9 * tmp_9;

// Compute beta_1
assign tmp_3 = 2 * N0;
assign tmp_4 = tmp_3 + N1;
assign tmp_5 = tmp_4 * tmp_4;
assign beta_1 = tmp_5 * 2;

// Compute beta_2
assign tmp_6 = 2 * N2;
assign tmp_7 = tmp_6 + N1;
assign beta_2 = tmp_4 * tmp_7;

// Compute beta_3
assign tmp_8 = tmp_7 * tmp_7;
assign beta_3 = tmp_8 * 2;

endmodule
