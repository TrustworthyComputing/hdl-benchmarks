/* Generated by Yosys 0.9 (git sha1 1979e0b) */

(* src = "designs/c17.v:1" *)
module c17(G1, G16, G17, G2, G3, G4, G5);
  wire _00_;
  wire _01_;
  wire _02_;
  wire _03_;
  (* src = "designs/c17.v:2" *)
  wire _04_;
  (* src = "designs/c17.v:3" *)
  wire _05_;
  (* src = "designs/c17.v:3" *)
  wire _06_;
  (* src = "designs/c17.v:2" *)
  wire _07_;
  (* src = "designs/c17.v:2" *)
  wire _08_;
  (* src = "designs/c17.v:2" *)
  wire _09_;
  (* src = "designs/c17.v:2" *)
  wire _10_;
  wire _11_;
  wire _12_;
  wire _13_;
  (* src = "designs/c17.v:2" *)
  input G1;
  (* src = "designs/c17.v:3" *)
  output G16;
  (* src = "designs/c17.v:3" *)
  output G17;
  (* src = "designs/c17.v:2" *)
  input G2;
  (* src = "designs/c17.v:2" *)
  input G3;
  (* src = "designs/c17.v:2" *)
  input G4;
  (* src = "designs/c17.v:2" *)
  input G5;
  assign _11_ = 4'h8 >> { _09_, _08_ };
  assign _06_ = 4'h1 >> { _12_, _11_ };
  assign _12_ = 4'h1 >> { _10_, _07_ };
  assign _13_ = 8'hbc >> { _07_, _08_, _09_ };
  assign _05_ = 8'hbc >> { _13_, _08_, _04_ };
  assign _07_ = G2;
  assign _08_ = G3;
  assign _04_ = G1;
  assign _09_ = G4;
  assign G16 = _05_;
  assign _10_ = G5;
  assign G17 = _06_;
endmodule