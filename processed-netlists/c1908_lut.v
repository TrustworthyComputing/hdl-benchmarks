module c1908(G1, G10, G11, G12, G13, G14, G15, G16, G17, G18, G1884, G1885, G1886, G1887, G1888, G1889, G1890, G1891, G1892, G1893, G1894, G1895, G1896, G1897, G1898, G1899, G19, G1900, G1901, G1902, G1903, G1904, G1905, G1906, G1907, G1908, G2, G20, G21, G22, G23, G24, G25, G26, G27, G28, G29, G3, G30, G31, G32, G33, G4, G5, G6, G7, G8, G9);
  wire 000, 001, 002, 003, 004, 005, 006, 007, 008, 009, 010, 011, 012, 013, 014, 015, 016, 017, 018, 019, 020, 021, 022, 023, 024, 025, 026, 027, 028, 029, 030, 031, 032, 033, 034, 035, 036, 037, 038, 039, 040, 041, 042, 043, 044, 045, 046, 047, 048, 049, 050, 051, 052, 053, 054, 055, 056, 057, 058, 059, 060, 061, 062, 063, 064, 065, 066, 067, 068, 069, 070, 071, 072, 073, 074, 075, 076, 077, 078, 079, 080, 081, 082, 083, 084, 085, 086, 087, 088, 089, 090, 091, 092, 093, 094, 095, 096, 097, 098, 099, 100, 101, 102, 103, 104, 105, 106, 107, 108, 109, 110, 111, 112, 113, 114, 115, 116, 117, 118, 119, 120, 121, 122, 123, 124, 125, 126, 127, 128, 129, 130, 131, 132, 133, 134, 135, 136, 137, 138, 139, 140, 141, 142, 143, 144, 145, 146, 147, 148, 149, 150, 151, 152, 153, 154, 155, 156, 157, 158, 159, 160, 161, 162, 163, 164, 165, 166, 167, 168, 169, 170, 171, 172, 173, 174, 175, 176, 177, 178, 179, 180, 181, 182, 183, 184, 185, 186, 187, 188, 189, 190, 191, 192, 193, 194, 195, 196, 197, 198, 199, 200, 201, 202, 203, 204, 205, 206, 207, 208, 209, 210, 211, 212, 213, 214, 215, 216, 217, 218, 219, 220, 221, 222, 223, 224, 225, 226, 227, 228, 229, 230, 231, 232, 233, 234, 235, 236, 237, 238, 239, 240, 241, 242, 243, 244, 245, 246, 247, 248, 249, 250, 251, 252, 253, 254, 255, 256, 257, 258, 259, 260, 261, 262, 263, 264, 265, 266, 267, 268, 269, 270, 271, 272, 273, 274, 275, 276, 277, 278, 279, 280, 281, 282, 283, 284, 285, 286, 287, 288, 289, 290, G313, G314, G315, G316, G317, G318, G319, G320, G321, G322, G323, G324, G325, G326, G327, G328, G335, G338, G341, G344, G347, G350, G356, G359, G362, G368, G374, G377, G395, G398, G401, G404, G407, G410, G413, G416, G419, G425, G428, G431, G437, G440, G443, G446, G449, G452, G455, G458, G467, G470, G473, G476, G479, G482, G485, G488, G491, G494, G497, G500, G506, G509, G512, G515, G518, G521, G524, G527, G530, G715, G716, G718, G719, G720, G721, G723, G728, G772, G774, G776, G778, G780, G782, G784, G786, G787, G789, G790, G792, G795, G805, G807, G808;
  input G1, G10, G11, G12, G13, G14, G15, G16, G17, G18, G19, G2, G20, G21, G22, G23, G24, G25, G26, G27, G28, G29, G3, G30, G31, G32, G33, G4, G5, G6, G7, G8, G9;
  output G1884, G1885, G1886, G1887, G1888, G1889, G1890, G1891, G1892, G1893, G1894, G1895, G1896, G1897, G1898, G1899, G1900, G1901, G1902, G1903, G1904, G1905, G1906, G1907, G1908;
  lut lut_gate1(0x1e, 252, 229, 231, G1906);
  lut lut_gate2(0xe0, 221, 253, 290, 252);
  lut lut_gate3(0x80, 282, 268, 254, 253);
  lut lut_gate4(0x90, 255, 267, 261, 254);
  lut lut_gate5(0x9, G28, 256, 255);
  lut lut_gate6(0x41, 257, 260, G31, 256);
  lut lut_gate7(0x96, G12, 259, 258, 257);
  lut lut_gate8(0x6, G10, G15, 258);
  lut lut_gate9(0x10, G19, G23, G33, 259);
  lut lut_gate10(0x69, G6, G8, G3, 260);
  lut lut_gate11(0x41, 262, 266, G31, 261);
  lut lut_gate12(0x96, G13, 265, 263, 262);
  lut lut_gate13(0x6, G16, 264, 263);
  lut lut_gate14(0x9, G9, G14, 264);
  lut lut_gate15(0x10, G20, G23, G33, 265);
  lut lut_gate16(0x69, G4, G7, G10, 266);
  lut lut_gate17(0xe0, G19, G31, G23, 267);
  lut lut_gate18(0x41, G27, 269, 274, 268);
  lut lut_gate19(0x14, 273, 270, G31, 269);
  lut lut_gate20(0x96, 272, 271, 263, 270);
  lut lut_gate21(0x10, G18, G24, G33, 271);
  lut lut_gate22(0x9, G11, G15, 272);
  lut lut_gate23(0x96, G2, G5, G8, 273);
  lut lut_gate24(0x6, G26, 275, 274);
  lut lut_gate25(0x14, 279, 276, G31, 275);
  lut lut_gate26(0x9, 278, 277, 276);
  lut lut_gate27(0x69, G13, G11, G12, 277);
  lut lut_gate28(0x69, G16, G10, G15, 278);
  lut lut_gate29(0x96, G1, 281, 280, 279);
  lut lut_gate30(0x96, G7, G5, G6, 280);
  lut lut_gate31(0x10, G17, G24, G33, 281);
  lut lut_gate32(0x9, 289, 283, 282);
  lut lut_gate33(0x14, 287, 284, G31, 283);
  lut lut_gate34(0x9, 286, 285, 284);
  lut lut_gate35(0x69, G4, G8, 280, 285);
  lut lut_gate36(0x96, G1, G2, G3, 286);
  lut lut_gate37(0x9, 288, 278, 287);
  lut lut_gate38(0x4b, G9, G21, G33, 288);
  lut lut_gate39(0xe0, G17, G31, G24, 289);
  lut lut_gate40(0x40, 254, 268, 220, 290);
  lut lut_gate41(0xe0, G18, G31, G24, 220);
  lut lut_gate42(0x0, 226, 222, 230, 221);
  lut lut_gate43(0x4b, G25, 223, G31, 222);
  lut lut_gate44(0x96, 286, 224, 276, 223);
  lut lut_gate45(0xb4, 225, G22, G33, 224);
  lut lut_gate46(0x9, G4, G14, 225);
  lut lut_gate47(0x0b, 227, G24, G23, 226);
  lut lut_gate48(0x0b, 228, 229, G31, 227);
  lut lut_gate49(0x1, G32, G33, 228);
  lut lut_gate50(0x1, G33, G29, 229);
  lut lut_gate51(0xe0, G20, G31, G23, 230);
  lut lut_gate52(0xe1, 284, G21, G33, 231);
  lut lut_gate53(0x9, G1, 252, G1884);
  lut lut_gate54(0x9, G2, 252, G1885);
  lut lut_gate55(0x9, G3, 252, G1886);
  lut lut_gate56(0x9, G4, 252, G1887);
  lut lut_gate57(0x9, G10, 232, G1888);
  lut lut_gate58(0xe0, 233, 253, 290, 232);
  lut lut_gate59(0x0, 234, 222, 230, 233);
  lut lut_gate60(0x0b, 235, G24, G23, 234);
  lut lut_gate61(0x0b, 228, 236, G31, 235);
  lut lut_gate62(0x1, G30, G33, 236);
  lut lut_gate63(0x9, G15, 232, G1889);
  lut lut_gate64(0x9, G16, 232, G1890);
  lut lut_gate65(0x87, G5, 226, 237, G1891);
  lut lut_gate66(0x8, 222, 238, 237);
  lut lut_gate67(0xe0, 230, 253, 290, 238);
  lut lut_gate68(0x87, G6, 226, 237, G1892);
  lut lut_gate69(0x87, G7, 226, 237, G1893);
  lut lut_gate70(0x87, G8, 226, 237, G1894);
  lut lut_gate71(0x87, G9, 234, 237, G1895);
  lut lut_gate72(0x9, G11, 239, G1896);
  lut lut_gate73(0x80, 220, 233, 253, 239);
  lut lut_gate74(0x9, G12, 239, G1897);
  lut lut_gate75(0x9, G13, 239, G1898);
  lut lut_gate76(0x9, G14, 239, G1899);
  lut lut_gate77(0xb, 241, 240, G1900);
  lut lut_gate78(0xe0, G32, 252, 232, 240);
  lut lut_gate79(0x07, G33, 290, 242, 241);
  lut lut_gate80(0x40, 282, 222, 230, 242);
  lut lut_gate81(0x1e, 232, 236, 243, G1907);
  lut lut_gate82(0x6, 244, 276, 243);
  lut lut_gate83(0xe1, 264, G22, G33, 244);
  lut lut_gate84(0x14, 246, 245, 228, G1901);
  lut lut_gate85(0xe0, 289, 252, 232, 245);
  lut lut_gate86(0x6, 287, 284, 246);
  lut lut_gate87(0x14, 223, 247, 228, G1902);
  lut lut_gate88(0xe0, G25, 252, 232, 247);
  lut lut_gate89(0x0, 228, 240, 248, G1903);
  lut lut_gate90(0x6, 273, 270, 248);
  lut lut_gate91(0x0e, 228, 240, 249, G1904);
  lut lut_gate92(0x6, 260, 257, 249);
  lut lut_gate93(0x0e, 228, 240, 250, G1905);
  lut lut_gate94(0x6, 266, 262, 250);
  lut lut_gate95(0x10, 251, 240, 228, G1908);
  lut lut_gate96(0x6, 279, 276, 251);

endmodule