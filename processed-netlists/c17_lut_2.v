module c17(G1, G16, G17, G2, G3, G4, G5);
  wire 00, 01, 02, 03, 04, 05, 06, 07, 08, 09, 10, 11, 12, 13;
  input G1, G2, G3, G4, G5;
  output G16, G17;
  lut lut_gate1(0x8, G4, G3, 11);
  lut lut_gate2(0x1, 12, 11, G17);
  lut lut_gate3(0x1, G5, G2, 12);
  lut lut_gate4(0xbc, G2, G3, G4, 13);
  lut lut_gate5(0xbc, 13, G3, G1, G16);

endmodule
