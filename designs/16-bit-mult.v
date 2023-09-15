module multiplier(G11,G12,G14);
input [15:0] G11,G12;
output [15:0] G14;
assign G14 = G11 * G12;
endmodule
