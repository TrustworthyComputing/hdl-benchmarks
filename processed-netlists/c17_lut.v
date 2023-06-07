module c17(G1, G16, G17, G2, G3, G4, G5);
  wire 00, 01, 02, 03, 04, 05, 06, 07, 08, 09, 10, 11, 12;
  input G1, G2, G3, G4, G5;
  output G16, G17;
  lut lut_gate1(0xf8, 11, G1, G3, G16);
  lut lut_gate2(0x70, G2, G3, G4, 11);
  lut lut_gate3(0x0e, 12, G2, G5, G17);
  lut lut_gate4(0x8, G4, G3, 12);

endmodule
