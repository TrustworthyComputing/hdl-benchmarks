module c499(G1, G10, G11, G12, G13, G14, G15, G16, G17, G18, G19, G2, G20, G21, G22, G23, G24, G25, G26, G27, G28, G29, G3, G30, G31, G32, G33, G34, G35, G36, G37, G38, G39, G4, G40, G41, G468, G469, G470, G471, G472, G473, G474, G475, G476, G477, G478, G479, G480, G481, G482, G483, G484, G485, G486, G487, G488, G489, G490, G491, G492, G493, G494, G495, G496, G497, G498, G499, G5, G6, G7, G8, G9);
  wire 000, 001, 002, 003, 004, 005, 006, 007, 008, 009, 010, 011, 012, 013, 014, 015, 016, 017, 018, 019, 020, 021, 022, 023, 024, 025, 026, 027, 028, 029, 030, 031, 032, 033, 034, 035, 036, 037, 038, 039, 040, 041, 042, 043, 044, 045, 046, 047, 048, 049, 050, 051, 052, 053, 054, 055, 056, 057, 058, 059, 060, 061, 062, 063, 064, 065, 066, 067, 068, 069, 070, 071, 072, 073, 074, 075, 076, 077, 078, 079, 080, 081, 082, 083, 084, 085, 086, 087, 088, 089, 090, 091, 092, 093, 094, 095, 096, 097, 098, 099, 100, 101, 102, 103, 104, 105, 106, 107, 108, 109, 110, 111, 112, 113, 114, 115, 116, 117, 118, 119, 120, 121, 122, 123, 124, 125, 126, 127, 128, 129, 130, 131, 132, 133, 134, 135, 136, 137, 138, 139, 140, 141, 142, 143, 144, 145, 146, 147, 148, 149, 150, 151, 152, 153, 154, 155, 156, 157, 158, 159, 160, 161, 162, 163, 164, 165, 166, 167, 168, 169, 170, 171, 172, 173, 174, 175, 176, 177, 178, 179, 180, 181, 182, 183, 184, 185, 186, 187, 188, 189, 190, 191, 192, 193, 194, 195, 196, 197, 198, 199, 200, 201, 202, 203, 204, 205, 206, 207, 208, 209, 210, 211, 212, 213, 214, 215, 216, 217, 218, 219, 220, 221, 222, 223, 224, 225, 226, 227, 228, 229, 230, 231, 232, 233, 234, 235, 236, 237, 238, 239, 240, 241, 242, 243, 244, 245, 246, 247, 248, 249, 250, 251, 252, 253, 254, 255, 256, 257, 258, 259, 260, 261, 262, 263, 264, 265, 266, 267, 268, 269, 270, 271, 272, 273, 274, 275, 276, 277, 278, 279, 280, 281, 282, 283, 284, 285, 286, 287, 288, 289, 290, 291, 292, 293, 294, 295, 296, 297, 298, 299, 300, 301, 302, 303, 304, 305, 306, 307, 308, 309, 310, 311, 312, 313, 314, 315, 316, 317, 318, 319, 320, 321, 322, 323, 324, 325, 326, 327, 328;
  input G1, G10, G11, G12, G13, G14, G15, G16, G17, G18, G19, G2, G20, G21, G22, G23, G24, G25, G26, G27, G28, G29, G3, G30, G31, G32, G33, G34, G35, G36, G37, G38, G39, G4, G40, G41, G5, G6, G7, G8, G9;
  output G468, G469, G470, G471, G472, G473, G474, G475, G476, G477, G478, G479, G480, G481, G482, G483, G484, G485, G486, G487, G488, G489, G490, G491, G492, G493, G494, G495, G496, G497, G498, G499;
  lut lut_gate1(0xb4, G1, 305, 327, G468);
  lut lut_gate2(0x40, 262, 244, 306, 305);
  lut lut_gate3(0x1, 240, 307, 306);
  lut lut_gate4(0x60, 308, 235, 327, 307);
  lut lut_gate5(0x4, 318, 309, 308);
  lut lut_gate6(0x69, 316, 312, 310, 309);
  lut lut_gate7(0x96, G32, G31, 311, 310);
  lut lut_gate8(0x9, G30, G29, 311);
  lut lut_gate9(0x96, 315, 314, 313, 312);
  lut lut_gate10(0x6, G23, G24, 313);
  lut lut_gate11(0x9, G21, G22, 314);
  lut lut_gate12(0x8, G36, G41, 315);
  lut lut_gate13(0x96, G16, G12, 317, 316);
  lut lut_gate14(0x9, G8, G4, 317);
  lut lut_gate15(0x69, 323, 321, 319, 318);
  lut lut_gate16(0x96, G19, G20, 320, 319);
  lut lut_gate17(0x9, G17, G18, 320);
  lut lut_gate18(0x96, G28, G27, 322, 321);
  lut lut_gate19(0x9, G26, G25, 322);
  lut lut_gate20(0x96, 326, 325, 324, 323);
  lut lut_gate21(0x8, G35, G41, 324);
  lut lut_gate22(0x9, G15, G11, 325);
  lut lut_gate23(0x9, G7, G3, 326);
  lut lut_gate24(0x69, 319, 234, 328, 327);
  lut lut_gate25(0x96, 233, 232, 231, 328);
  lut lut_gate26(0x8, G33, G41, 231);
  lut lut_gate27(0x9, G9, G13, 232);
  lut lut_gate28(0x9, G1, G5, 233);
  lut lut_gate29(0x6, 314, 313, 234);
  lut lut_gate30(0x69, 321, 310, 236, 235);
  lut lut_gate31(0x96, 239, 238, 237, 236);
  lut lut_gate32(0x8, G34, G41, 237);
  lut lut_gate33(0x9, G14, G10, 238);
  lut lut_gate34(0x9, G6, G2, 239);
  lut lut_gate35(0xe0, 243, 241, 242, 240);
  lut lut_gate36(0x8, 318, 309, 241);
  lut lut_gate37(0x1, 318, 309, 242);
  lut lut_gate38(0x8, 235, 327, 243);
  lut lut_gate39(0x4, 254, 245, 244);
  lut lut_gate40(0x9, 249, 246, 245);
  lut lut_gate41(0x69, G21, G17, 247, 246);
  lut lut_gate42(0x78, 248, G37, G41, 247);
  lut lut_gate43(0x9, G25, G29, 248);
  lut lut_gate44(0x96, 253, 252, 250, 249);
  lut lut_gate45(0x96, G3, G4, 251, 250);
  lut lut_gate46(0x9, G2, G1, 251);
  lut lut_gate47(0x6, G7, G8, 252);
  lut lut_gate48(0x9, G6, G5, 253);
  lut lut_gate49(0x69, 260, 258, 255, 254);
  lut lut_gate50(0x69, G22, G18, 256, 255);
  lut lut_gate51(0x78, 257, G38, G41, 256);
  lut lut_gate52(0x9, G26, G30, 257);
  lut lut_gate53(0x96, G15, G16, 259, 258);
  lut lut_gate54(0x9, G14, G13, 259);
  lut lut_gate55(0x96, G11, G12, 261, 260);
  lut lut_gate56(0x9, G10, G9, 261);
  lut lut_gate57(0x1, 268, 263, 262);
  lut lut_gate58(0x69, 266, 264, 258, 263);
  lut lut_gate59(0x96, 265, 253, 252, 264);
  lut lut_gate60(0x8, G40, G41, 265);
  lut lut_gate61(0x96, G28, G32, 267, 266);
  lut lut_gate62(0x9, G24, G20, 267);
  lut lut_gate63(0x69, 269, 260, 250, 268);
  lut lut_gate64(0x96, 272, 271, 270, 269);
  lut lut_gate65(0x8, G39, G41, 270);
  lut lut_gate66(0x9, G27, G31, 271);
  lut lut_gate67(0x9, G23, G19, 272);
  lut lut_gate68(0xb4, G2, 305, 235, G469);
  lut lut_gate69(0xb4, G3, 305, 318, G470);
  lut lut_gate70(0x78, G4, 309, 305, G471);
  lut lut_gate71(0xb4, G5, 273, 327, G472);
  lut lut_gate72(0x8, 244, 274, 273);
  lut lut_gate73(0xe0, 275, 307, 240, 274);
  lut lut_gate74(0x8, 268, 263, 275);
  lut lut_gate75(0xb4, G6, 273, 235, G473);
  lut lut_gate76(0xb4, G7, 273, 318, G474);
  lut lut_gate77(0x6, G8, 276, G475);
  lut lut_gate78(0x80, 309, 244, 274, 276);
  lut lut_gate79(0xb4, G9, 277, 327, G476);
  lut lut_gate80(0x40, 262, 278, 306, 277);
  lut lut_gate81(0x4, 245, 254, 278);
  lut lut_gate82(0xb4, G10, 277, 235, G477);
  lut lut_gate83(0xb4, G11, 277, 318, G478);
  lut lut_gate84(0x78, G12, 309, 277, G479);
  lut lut_gate85(0xb4, G13, 279, 327, G480);
  lut lut_gate86(0x8, 278, 274, 279);
  lut lut_gate87(0xb4, G14, 279, 235, G481);
  lut lut_gate88(0xb4, G15, 279, 318, G482);
  lut lut_gate89(0x6, G16, 280, G483);
  lut lut_gate90(0x80, 309, 278, 274, 280);
  lut lut_gate91(0x6, G17, 281, G484);
  lut lut_gate92(0x40, 242, 282, 245, 281);
  lut lut_gate93(0xe0, 287, 283, 285, 282);
  lut lut_gate94(0x40, 245, 254, 284, 283);
  lut lut_gate95(0x6, 268, 263, 284);
  lut lut_gate96(0x60, 286, 254, 245, 285);
  lut lut_gate97(0x4, 268, 263, 286);
  lut lut_gate98(0x4, 235, 327, 287);
  lut lut_gate99(0x6, G18, 288, G485);
  lut lut_gate100(0x40, 242, 282, 254, 288);
  lut lut_gate101(0x6, G19, 289, G486);
  lut lut_gate102(0x40, 282, 242, 268, 289);
  lut lut_gate103(0x6, G20, 290, G487);
  lut lut_gate104(0x80, 263, 242, 282, 290);
  lut lut_gate105(0x6, G21, 291, G488);
  lut lut_gate106(0x40, 241, 282, 245, 291);
  lut lut_gate107(0x6, G22, 292, G489);
  lut lut_gate108(0x40, 241, 282, 254, 292);
  lut lut_gate109(0x6, G23, 293, G490);
  lut lut_gate110(0x40, 282, 241, 268, 293);
  lut lut_gate111(0x6, G24, 294, G491);
  lut lut_gate112(0x80, 263, 241, 282, 294);
  lut lut_gate113(0x6, G25, 295, G492);
  lut lut_gate114(0x40, 242, 296, 245, 295);
  lut lut_gate115(0xe0, 297, 283, 285, 296);
  lut lut_gate116(0x4, 327, 235, 297);
  lut lut_gate117(0x6, G26, 298, G493);
  lut lut_gate118(0x40, 242, 296, 254, 298);
  lut lut_gate119(0x6, G27, 299, G494);
  lut lut_gate120(0x40, 296, 242, 268, 299);
  lut lut_gate121(0x6, G28, 300, G495);
  lut lut_gate122(0x80, 263, 242, 296, 300);
  lut lut_gate123(0x6, G29, 301, G496);
  lut lut_gate124(0x40, 241, 296, 245, 301);
  lut lut_gate125(0x6, G30, 302, G497);
  lut lut_gate126(0x40, 241, 296, 254, 302);
  lut lut_gate127(0x6, G31, 303, G498);
  lut lut_gate128(0x40, 296, 241, 268, 303);
  lut lut_gate129(0x6, G32, 304, G499);
  lut lut_gate130(0x80, 263, 241, 296, 304);

endmodule
