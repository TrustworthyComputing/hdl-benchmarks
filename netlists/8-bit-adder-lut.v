/* Generated by Yosys 0.9+2406 (git sha1 334ec5fa, clang 10.0.0-4ubuntu1 -fPIC -Os) */

(* src = "adder.v:1.1-5.10" *)
module adder(G11, G12, G14);
  wire _00_;
  wire _01_;
  wire _02_;
  wire _03_;
  wire _04_;
  wire _05_;
  wire _06_;
  wire _07_;
  wire _08_;
  wire _09_;
  wire _10_;
  wire _11_;
  wire _12_;
  wire _13_;
  wire _14_;
  wire _15_;
  wire _16_;
  wire _17_;
  wire _18_;
  wire _19_;
  wire _20_;
  wire _21_;
  wire _22_;
  wire _23_;
  wire _24_;
  wire _25_;
  wire _26_;
  wire _27_;
  wire _28_;
  wire _29_;
  wire _30_;
  wire _31_;
  (* src = "adder.v:2.13-2.16" *)
  wire _32_;
  (* src = "adder.v:2.13-2.16" *)
  wire _33_;
  (* src = "adder.v:2.13-2.16" *)
  wire _34_;
  (* src = "adder.v:2.13-2.16" *)
  wire _35_;
  (* src = "adder.v:2.13-2.16" *)
  wire _36_;
  (* src = "adder.v:2.13-2.16" *)
  wire _37_;
  (* src = "adder.v:2.13-2.16" *)
  wire _38_;
  (* src = "adder.v:2.13-2.16" *)
  wire _39_;
  (* src = "adder.v:2.17-2.20" *)
  wire _40_;
  (* src = "adder.v:2.17-2.20" *)
  wire _41_;
  (* src = "adder.v:2.17-2.20" *)
  wire _42_;
  (* src = "adder.v:2.17-2.20" *)
  wire _43_;
  (* src = "adder.v:2.17-2.20" *)
  wire _44_;
  (* src = "adder.v:2.17-2.20" *)
  wire _45_;
  (* src = "adder.v:2.17-2.20" *)
  wire _46_;
  (* src = "adder.v:2.17-2.20" *)
  wire _47_;
  (* src = "adder.v:3.14-3.17" *)
  wire _48_;
  (* src = "adder.v:3.14-3.17" *)
  wire _49_;
  (* src = "adder.v:3.14-3.17" *)
  wire _50_;
  (* src = "adder.v:3.14-3.17" *)
  wire _51_;
  (* src = "adder.v:3.14-3.17" *)
  wire _52_;
  (* src = "adder.v:3.14-3.17" *)
  wire _53_;
  (* src = "adder.v:3.14-3.17" *)
  wire _54_;
  (* src = "adder.v:3.14-3.17" *)
  wire _55_;
  wire _56_;
  wire _57_;
  wire _58_;
  (* src = "adder.v:2.13-2.16" *)
  input [7:0] G11;
  (* src = "adder.v:2.17-2.20" *)
  input [7:0] G12;
  (* src = "adder.v:3.14-3.17" *)
  output [7:0] G14;
  assign _49_ = 16'h8778 >> { _41_, _33_, _40_, _32_ };
  assign _50_ = 64'hf880077f077ff880 >> { _42_, _34_, _41_, _33_, _40_, _32_ };
  assign _56_ = 64'h0317173f173f173f >> { _32_, _40_, _41_, _42_, _34_, _33_ };
  assign _51_ = 8'h69 >> { _43_, _35_, _56_ };
  assign _52_ = 32'd3559599060 >> { _44_, _36_, _43_, _35_, _56_ };
  assign _57_ = 32'd53160767 >> { _43_, _35_, _44_, _36_, _56_ };
  assign _53_ = 8'h69 >> { _45_, _37_, _57_ };
  assign _54_ = 32'd3559599060 >> { _46_, _38_, _45_, _37_, _57_ };
  assign _58_ = 4'h6 >> { _47_, _39_ };
  assign _55_ = 64'h032b2b3ffcd4d4c0 >> { _58_, _45_, _37_, _46_, _38_, _57_ };
  assign _48_ = 4'h6 >> { _40_, _32_ };
  assign _33_ = G11[1];
  assign _41_ = G12[1];
  assign _32_ = G11[0];
  assign _40_ = G12[0];
  assign G14[1] = _49_;
  assign _34_ = G11[2];
  assign _42_ = G12[2];
  assign G14[2] = _50_;
  assign _35_ = G11[3];
  assign _43_ = G12[3];
  assign G14[3] = _51_;
  assign _36_ = G11[4];
  assign _44_ = G12[4];
  assign G14[4] = _52_;
  assign _37_ = G11[5];
  assign _45_ = G12[5];
  assign G14[5] = _53_;
  assign _38_ = G11[6];
  assign _46_ = G12[6];
  assign G14[6] = _54_;
  assign _39_ = G11[7];
  assign _47_ = G12[7];
  assign G14[7] = _55_;
  assign G14[0] = _48_;
endmodule