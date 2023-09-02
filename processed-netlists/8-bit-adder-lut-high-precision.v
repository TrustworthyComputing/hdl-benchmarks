module adder(G11, G12, G14);
  wire _00_, _01_, _02_, _03_, _04_, _05_, _06_, _07_, _08_, _09_, _10_, _11_, _12_, _13_, _14_, _15_, _16_, _17_, _18_, _19_, _20_, _21_, _22_, _23_, _24_, _25_, _26_, _27_, _28_, _29_, _30_, _31_, _32_, _33_, _34_, _35_, _36_, _37_, _38_, _39_, _40_, _41_, _42_, _43_, _44_, _45_, _46_, _47_, _48_, _49_, _50_, _51_;
  input [7:0] G11;
  input [7:0] G12;
  output G14[0], G14[1], G14[2], G14[3], G14[4], G14[5], G14[6], G14[7];
  lut lut_gate1(0x8778, G11[1], G12[1], G11[0], G12[0], G14[1]);
  lut lut_gate2(0xf880077f077ff880, G11[2], G12[2], G11[1], G12[1], G11[0], G12[0], G14[2]);
  lut lut_gate3(0x69, G11[3], G12[3], _49_, G14[3]);
  lut lut_gate4(0x0317173f173f173f, G12[0], G11[0], G11[1], G11[2], G12[2], G12[1], _49_);
  lut lut_gate5(3559599060, G11[4], G12[4], G11[3], G12[3], _49_, G14[4]);
  lut lut_gate6(0x69, G11[5], G12[5], _50_, G14[5]);
  lut lut_gate7(53160767, G11[3], G12[3], G11[4], G12[4], _49_, _50_);
  lut lut_gate8(3559599060, G11[6], G12[6], G11[5], G12[5], _50_, G14[6]);
  lut lut_gate9(0x032b2b3ffcd4d4c0, _51_, G11[5], G12[5], G11[6], G12[6], _50_, G14[7]);
  lut lut_gate10(0x6, G11[7], G12[7], _51_);
  lut lut_gate11(0x6, G11[0], G12[0], G14[0]);

endmodule
