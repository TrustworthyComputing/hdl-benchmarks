// Parallel CRC32 (Koopman) 
// Polynomial: x^32 + x^30 + x^29 + x^28 + x^26 + x^20 + x^19 + x^17 + 
// + x^16 + x^15 + x^11 + x^10 + x^7 + x^6 + x^4 + x^2 + x^1 + 1

module CRC32_K_Parallel 
	#(	
	// Parameters
	parameter INITIAL_STATES = 32'h00000000)

	(	
	// FPGA Signals
	input clk, rst_n,
	
	// Data Signals
	input [7:0] 	i_RX_Byte,
	input 		i_Init,
	input		i_RX_En,
	output reg 	o_CRC_Ready,
	output [31:0] 	o_CRC,
	output [31:0] 	o_CRC_Reversed,
	output [31:0] 	o_CRC_Xor,
	output [31:0] 	o_CRC_Xor_Reserved
	);

	reg [31:0]	r_CRC;
	reg [7:0] 	r_RX_Byte;
	reg 		r_RX_En;	


	// Initiate Frame Exchange
	always @(posedge clk or negedge rst_n)
	begin
		if (!rst_n)
		begin
			r_CRC		<= INITIAL_STATES;	// Reset CRC regs to initial conditions 	
			o_CRC_Ready	<= 1'b0;
		end	
		else 
		begin 
			if (i_Init)
			begin
				r_CRC	<= INITIAL_STATES;	// Reset CRC regs to initial conditions 
				o_CRC_Ready	<= 1'b0;	
			end
			else if (r_RX_En)
			begin			
				r_CRC[0]  <= r_RX_Byte[7] ^ r_RX_Byte[6] ^ r_RX_Byte[3] ^ r_RX_Byte[2] ^ r_RX_Byte[0] ^ r_CRC[24] ^ r_CRC[26] ^ r_CRC[27] ^ r_CRC[30] ^ r_CRC[31];
    				r_CRC[1]  <= r_RX_Byte[6] ^ r_RX_Byte[4] ^ r_RX_Byte[2] ^ r_RX_Byte[1] ^ r_RX_Byte[0] ^ r_CRC[24] ^ r_CRC[25] ^ r_CRC[26] ^ r_CRC[28] ^ r_CRC[30];
   				r_CRC[2]  <= r_RX_Byte[6] ^ r_RX_Byte[5] ^ r_RX_Byte[1] ^ r_RX_Byte[0] ^ r_CRC[24] ^ r_CRC[25] ^ r_CRC[29] ^ r_CRC[30];
    				r_CRC[3]  <= r_RX_Byte[7] ^ r_RX_Byte[6] ^ r_RX_Byte[2] ^ r_RX_Byte[1] ^ r_CRC[25] ^ r_CRC[26] ^ r_CRC[30] ^ r_CRC[31];
				r_CRC[4]  <= r_RX_Byte[6] ^ r_RX_Byte[0] ^ r_CRC[24] ^ r_CRC[30];
				r_CRC[5]  <= r_RX_Byte[7] ^ r_RX_Byte[1] ^ r_CRC[25] ^ r_CRC[31];
				r_CRC[6]  <= r_RX_Byte[7] ^ r_RX_Byte[6] ^ r_RX_Byte[3] ^ r_RX_Byte[0] ^ r_CRC[24] ^ r_CRC[27] ^ r_CRC[30] ^ r_CRC[31];
				r_CRC[7]  <= r_RX_Byte[6] ^ r_RX_Byte[4] ^ r_RX_Byte[3] ^ r_RX_Byte[2] ^ r_RX_Byte[1] ^ r_RX_Byte[0] ^ r_CRC[24] ^ r_CRC[25] ^ r_CRC[26] ^ r_CRC[27] ^ r_CRC[28] ^ r_CRC[30];
				r_CRC[8]  <= r_RX_Byte[7] ^ r_RX_Byte[5] ^ r_RX_Byte[4] ^ r_RX_Byte[3] ^ r_RX_Byte[2] ^ r_RX_Byte[1] ^ r_CRC[0] ^ r_CRC[25] ^ r_CRC[26] ^ r_CRC[27] ^ r_CRC[28] ^ r_CRC[29] ^ r_CRC[31];
				r_CRC[9]  <= r_RX_Byte[6] ^ r_RX_Byte[5] ^ r_RX_Byte[4] ^ r_RX_Byte[3] ^ r_RX_Byte[2] ^ r_CRC[1] ^ r_CRC[26] ^ r_CRC[27] ^ r_CRC[28] ^ r_CRC[29] ^ r_CRC[30];
				r_CRC[10] <= r_RX_Byte[5] ^ r_RX_Byte[4] ^ r_RX_Byte[2] ^ r_RX_Byte[0] ^ r_CRC[2] ^ r_CRC[24] ^ r_CRC[26] ^ r_CRC[28] ^ r_CRC[29];
				r_CRC[11] <= r_RX_Byte[7] ^ r_RX_Byte[5] ^ r_RX_Byte[2] ^ r_RX_Byte[1] ^ r_RX_Byte[0] ^ r_CRC[3] ^ r_CRC[24] ^ r_CRC[25] ^ r_CRC[26] ^ r_CRC[29] ^ r_CRC[31];
				r_CRC[12] <= r_RX_Byte[6] ^ r_RX_Byte[3] ^ r_RX_Byte[2] ^ r_RX_Byte[1] ^ r_CRC[4] ^ r_CRC[25] ^ r_CRC[26] ^ r_CRC[27] ^ r_CRC[30];
				r_CRC[13] <= r_RX_Byte[7] ^ r_RX_Byte[4] ^ r_RX_Byte[3] ^ r_RX_Byte[2] ^ r_CRC[5] ^ r_CRC[26] ^ r_CRC[27] ^ r_CRC[28] ^ r_CRC[31];
				r_CRC[14] <= r_RX_Byte[5] ^ r_RX_Byte[4] ^ r_RX_Byte[3] ^ r_CRC[6] ^ r_CRC[27] ^ r_CRC[28] ^ r_CRC[29];
				r_CRC[15] <= r_RX_Byte[7] ^ r_RX_Byte[5] ^ r_RX_Byte[4] ^ r_RX_Byte[3] ^ r_RX_Byte[2] ^ r_RX_Byte[0] ^ r_CRC[7] ^ r_CRC[24] ^ r_CRC[26] ^ r_CRC[27] ^ r_CRC[28] ^ r_CRC[29] ^ r_CRC[31];
				r_CRC[16] <= r_RX_Byte[7] ^ r_RX_Byte[5] ^ r_RX_Byte[4] ^ r_RX_Byte[2] ^ r_RX_Byte[1] ^ r_RX_Byte[0] ^ r_CRC[8] ^ r_CRC[24] ^ r_CRC[25] ^ r_CRC[26] ^ r_CRC[28] ^ r_CRC[29] ^ r_CRC[31];
				r_CRC[17] <= r_RX_Byte[7] ^ r_RX_Byte[5] ^ r_RX_Byte[1] ^ r_RX_Byte[0] ^ r_CRC[9] ^ r_CRC[24] ^ r_CRC[25] ^ r_CRC[29] ^ r_CRC[31];
				r_CRC[18] <= r_RX_Byte[6] ^ r_RX_Byte[2] ^ r_RX_Byte[1] ^ r_CRC[10] ^ r_CRC[25] ^ r_CRC[26] ^ r_CRC[30];
				r_CRC[19] <= r_RX_Byte[6] ^ r_RX_Byte[0] ^ r_CRC[11] ^ r_CRC[24] ^ r_CRC[30];
				r_CRC[20] <= r_RX_Byte[6] ^ r_RX_Byte[3] ^ r_RX_Byte[2] ^ r_RX_Byte[1] ^ r_RX_Byte[0] ^ r_CRC[12] ^ r_CRC[24] ^ r_CRC[25] ^ r_CRC[26] ^ r_CRC[27] ^ r_CRC[30];
				r_CRC[21] <= r_RX_Byte[7] ^ r_RX_Byte[4] ^ r_RX_Byte[3] ^ r_RX_Byte[2] ^ r_RX_Byte[1] ^ r_CRC[13] ^ r_CRC[25] ^ r_CRC[26] ^ r_CRC[27] ^ r_CRC[28] ^ r_CRC[31];
				r_CRC[22] <= r_RX_Byte[5] ^ r_RX_Byte[4] ^ r_RX_Byte[3] ^ r_RX_Byte[2] ^ r_CRC[14] ^ r_CRC[26] ^ r_CRC[27] ^ r_CRC[28] ^ r_CRC[29];
				r_CRC[23] <= r_RX_Byte[6] ^ r_RX_Byte[5] ^ r_RX_Byte[4] ^ r_RX_Byte[3] ^ r_CRC[15] ^ r_CRC[27] ^ r_CRC[28] ^ r_CRC[29] ^ r_CRC[30];
				r_CRC[24] <= r_RX_Byte[7] ^ r_RX_Byte[6] ^ r_RX_Byte[5] ^ r_RX_Byte[4] ^ r_CRC[16] ^ r_CRC[28] ^ r_CRC[29] ^ r_CRC[30] ^ r_CRC[31];
				r_CRC[25] <= r_RX_Byte[7] ^ r_RX_Byte[6] ^ r_RX_Byte[5] ^ r_CRC[17] ^ r_CRC[29] ^ r_CRC[30] ^ r_CRC[31];
				r_CRC[26] <= r_RX_Byte[3] ^ r_RX_Byte[2] ^ r_RX_Byte[0] ^ r_CRC[18] ^ r_CRC[24] ^ r_CRC[26] ^ r_CRC[27];
				r_CRC[27] <= r_RX_Byte[4] ^ r_RX_Byte[3] ^ r_RX_Byte[1] ^ r_CRC[19] ^ r_CRC[25] ^ r_CRC[27] ^ r_CRC[28];
				r_CRC[28] <= r_RX_Byte[7] ^ r_RX_Byte[6] ^ r_RX_Byte[5] ^ r_RX_Byte[4] ^ r_RX_Byte[3] ^ r_RX_Byte[0] ^ r_CRC[20] ^ r_CRC[24] ^ r_CRC[27] ^ r_CRC[28] ^ r_CRC[29] ^ r_CRC[30] ^ r_CRC[31];
				r_CRC[29] <= r_RX_Byte[5] ^ r_RX_Byte[4] ^ r_RX_Byte[3] ^ r_RX_Byte[2] ^ r_RX_Byte[1] ^ r_RX_Byte[0] ^ r_CRC[21] ^ r_CRC[24] ^ r_CRC[25] ^ r_CRC[26] ^ r_CRC[27] ^ r_CRC[28] ^ r_CRC[29];
				r_CRC[30] <= r_RX_Byte[7] ^ r_RX_Byte[5] ^ r_RX_Byte[4] ^ r_RX_Byte[1] ^ r_RX_Byte[0] ^ r_CRC[22] ^ r_CRC[24] ^ r_CRC[25] ^ r_CRC[28] ^ r_CRC[29] ^ r_CRC[31];
				r_CRC[31] <= r_RX_Byte[6] ^ r_RX_Byte[5] ^ r_RX_Byte[2] ^ r_RX_Byte[1] ^ r_CRC[23] ^ r_CRC[25] ^ r_CRC[26] ^ r_CRC[29] ^ r_CRC[30];
				r_RX_En		<= 1'b0;
				o_CRC_Ready	<= 1'b1;
			end
		end
	end
	
	// Register input byte	
	always @(posedge clk or negedge rst_n)
	begin
		if (!rst_n)
		begin
			r_RX_Byte	<= 8'h00;
		end
		else
		begin
			if (i_RX_En)
			begin
				r_RX_En 	<= i_RX_En;			
				r_RX_Byte 	<= i_RX_Byte;
				o_CRC_Ready	<= 1'b0;
			end	
		end		
	end			


	assign o_CRC = r_CRC;
	assign o_CRC_Reversed = {r_CRC[0], r_CRC[1], r_CRC[2], r_CRC[3], r_CRC[4],  r_CRC[5], r_CRC[6], r_CRC[7], 
							r_CRC[8], r_CRC[9], r_CRC[10], r_CRC[11],r_CRC[12], r_CRC[13], r_CRC[14], r_CRC[15], 
							r_CRC[16], r_CRC[17], r_CRC[18], r_CRC[19], r_CRC[20], r_CRC[21], r_CRC[22], r_CRC[23], 
							r_CRC[24], r_CRC[25], r_CRC[26], r_CRC[27], r_CRC[28], r_CRC[29], r_CRC[30], r_CRC[31]};

	assign o_CRC_Xor = o_CRC ^ 32'hFFFFFFFF;
  	assign o_CRC_Xor_Reserved = o_CRC_Reversed ^ 32'hFFFFFFFF;
endmodule
