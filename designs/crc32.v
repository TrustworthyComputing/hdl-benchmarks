module crc32(data, polynomial, crc_out);

input [31:0] data;
input [31:0] polynomial;
output [31:0] crc_out;

wire [31:0] crc_register;
wire [31:0] crc_register_2;
wire [31:0] crc_register_3;
wire [31:0] crc_register_4;
wire [31:0] crc_register_5;
wire [31:0] crc_register_6;
wire [31:0] crc_register_7;
wire [31:0] crc_register_8;
wire [31:0] crc_register_9;


// Initialize the CRC register
assign crc_register = 32'hFFFFFFFF ^ data;

assign crc_register_2 = (crc_register) ? (crc_register >>> 1) ^ polynomial : crc_register >>> 1;
assign crc_register_3 = (crc_register_2) ? (crc_register_2 >>> 1) ^ polynomial : crc_register_2 >>> 1;
assign crc_register_4 = (crc_register_3) ? (crc_register_3 >>> 1) ^ polynomial : crc_register_3 >>> 1;
assign crc_register_5 = (crc_register_4) ? (crc_register_4 >>> 1) ^ polynomial : crc_register_4 >>> 1;
assign crc_register_6 = (crc_register_5) ? (crc_register_5 >>> 1) ^ polynomial : crc_register_5 >>> 1;
assign crc_register_7 = (crc_register_6) ? (crc_register_6 >>> 1) ^ polynomial : crc_register_6 >>> 1;
assign crc_register_8 = (crc_register_7) ? (crc_register_7 >>> 1) ^ polynomial : crc_register_7 >>> 1;
assign crc_register_9 = (crc_register_8) ? (crc_register_8 >>> 1) ^ polynomial : crc_register_8 >>> 1;

assign crc_out = ~crc_register_9;

endmodule
