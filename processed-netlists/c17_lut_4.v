module c17(G1, G16, G17, G2, G3, G4, G5);
  wire 00, 01, 02, 03, 04, 05, 06, 07, 08, 09, 10;
  input G1, G2, G3, G4, G5;
  output G16, G17;
  lut lut_gate1(0xbfa0, G2, G3, G4, G1, G16);
  lut lut_gate2(0x7770, G5, G2, G3, G4, G17);

endmodule
