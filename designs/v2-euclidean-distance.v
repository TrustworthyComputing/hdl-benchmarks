module euclidean_distance (u_0, v_0, u_1, v_1, dist);

  input [15:0] u_0;
  input [15:0] v_0;
  input [15:0] u_1;
  input [15:0] v_1;

  output [15:0] dist;

  wire [15:0] t_diff_v0;
  wire [15:0] t0_sum_v0;
  wire [15:0] t_diff_v1;
  wire [15:0] t0_sum_v1;
  wire [15:0] t1_sum_v0;
  wire [15:0] t2_sum_v0;

  assign t_diff_v0 = v_0 - u_0;
  assign t0_sum_v0 = t_diff_v0 * t_diff_v0;
  assign t_diff_v1 = v_1 - u_1;
  assign t0_sum_v1 = t_diff_v1 * t_diff_v1;
  assign t1_sum_v0 = t0_sum_v0 + t0_sum_v1;
  assign dist = t1_sum_v0;
endmodule
