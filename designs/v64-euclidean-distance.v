module euclidean_distance (u_0, v_0, u_1, v_1, u_2, v_2, u_3, v_3, u_4, v_4, u_5, v_5, u_6, v_6, u_7, v_7, u_8, v_8, u_9, v_9, u_10, v_10, u_11, v_11, u_12, v_12, u_13, v_13, u_14, v_14, u_15, v_15, u_16, v_16, u_17, v_17, u_18, v_18, u_19, v_19, u_20, v_20, u_21, v_21, u_22, v_22, u_23, v_23, u_24, v_24, u_25, v_25, u_26, v_26, u_27, v_27, u_28, v_28, u_29, v_29, u_30, v_30, u_31, v_31, u_32, v_32, u_33, v_33, u_34, v_34, u_35, v_35, u_36, v_36, u_37, v_37, u_38, v_38, u_39, v_39, u_40, v_40, u_41, v_41, u_42, v_42, u_43, v_43, u_44, v_44, u_45, v_45, u_46, v_46, u_47, v_47, u_48, v_48, u_49, v_49, u_50, v_50, u_51, v_51, u_52, v_52, u_53, v_53, u_54, v_54, u_55, v_55, u_56, v_56, u_57, v_57, u_58, v_58, u_59, v_59, u_60, v_60, u_61, v_61, u_62, v_62, u_63, v_63, dist);
  input [31:0] u_0;
  input [31:0] v_0;
  input [31:0] u_1;
  input [31:0] v_1;
  input [31:0] u_2;
  input [31:0] v_2;
  input [31:0] u_3;
  input [31:0] v_3;
  input [31:0] u_4;
  input [31:0] v_4;
  input [31:0] u_5;
  input [31:0] v_5;
  input [31:0] u_6;
  input [31:0] v_6;
  input [31:0] u_7;
  input [31:0] v_7;
  input [31:0] u_8;
  input [31:0] v_8;
  input [31:0] u_9;
  input [31:0] v_9;
  input [31:0] u_10;
  input [31:0] v_10;
  input [31:0] u_11;
  input [31:0] v_11;
  input [31:0] u_12;
  input [31:0] v_12;
  input [31:0] u_13;
  input [31:0] v_13;
  input [31:0] u_14;
  input [31:0] v_14;
  input [31:0] u_15;
  input [31:0] v_15;
  input [31:0] u_16;
  input [31:0] v_16;
  input [31:0] u_17;
  input [31:0] v_17;
  input [31:0] u_18;
  input [31:0] v_18;
  input [31:0] u_19;
  input [31:0] v_19;
  input [31:0] u_20;
  input [31:0] v_20;
  input [31:0] u_21;
  input [31:0] v_21;
  input [31:0] u_22;
  input [31:0] v_22;
  input [31:0] u_23;
  input [31:0] v_23;
  input [31:0] u_24;
  input [31:0] v_24;
  input [31:0] u_25;
  input [31:0] v_25;
  input [31:0] u_26;
  input [31:0] v_26;
  input [31:0] u_27;
  input [31:0] v_27;
  input [31:0] u_28;
  input [31:0] v_28;
  input [31:0] u_29;
  input [31:0] v_29;
  input [31:0] u_30;
  input [31:0] v_30;
  input [31:0] u_31;
  input [31:0] v_31;
  input [31:0] u_32;
  input [31:0] v_32;
  input [31:0] u_33;
  input [31:0] v_33;
  input [31:0] u_34;
  input [31:0] v_34;
  input [31:0] u_35;
  input [31:0] v_35;
  input [31:0] u_36;
  input [31:0] v_36;
  input [31:0] u_37;
  input [31:0] v_37;
  input [31:0] u_38;
  input [31:0] v_38;
  input [31:0] u_39;
  input [31:0] v_39;
  input [31:0] u_40;
  input [31:0] v_40;
  input [31:0] u_41;
  input [31:0] v_41;
  input [31:0] u_42;
  input [31:0] v_42;
  input [31:0] u_43;
  input [31:0] v_43;
  input [31:0] u_44;
  input [31:0] v_44;
  input [31:0] u_45;
  input [31:0] v_45;
  input [31:0] u_46;
  input [31:0] v_46;
  input [31:0] u_47;
  input [31:0] v_47;
  input [31:0] u_48;
  input [31:0] v_48;
  input [31:0] u_49;
  input [31:0] v_49;
  input [31:0] u_50;
  input [31:0] v_50;
  input [31:0] u_51;
  input [31:0] v_51;
  input [31:0] u_52;
  input [31:0] v_52;
  input [31:0] u_53;
  input [31:0] v_53;
  input [31:0] u_54;
  input [31:0] v_54;
  input [31:0] u_55;
  input [31:0] v_55;
  input [31:0] u_56;
  input [31:0] v_56;
  input [31:0] u_57;
  input [31:0] v_57;
  input [31:0] u_58;
  input [31:0] v_58;
  input [31:0] u_59;
  input [31:0] v_59;
  input [31:0] u_60;
  input [31:0] v_60;
  input [31:0] u_61;
  input [31:0] v_61;
  input [31:0] u_62;
  input [31:0] v_62;
  input [31:0] u_63;
  input [31:0] v_63;

  output [31:0] dist;

  wire [31:0] t_diff_v0;
  wire [31:0] t0_sum_v0;
  wire [31:0] t_diff_v1;
  wire [31:0] t0_sum_v1;
  wire [31:0] t_diff_v2;
  wire [31:0] t0_sum_v2;
  wire [31:0] t_diff_v3;
  wire [31:0] t0_sum_v3;
  wire [31:0] t_diff_v4;
  wire [31:0] t0_sum_v4;
  wire [31:0] t_diff_v5;
  wire [31:0] t0_sum_v5;
  wire [31:0] t_diff_v6;
  wire [31:0] t0_sum_v6;
  wire [31:0] t_diff_v7;
  wire [31:0] t0_sum_v7;
  wire [31:0] t_diff_v8;
  wire [31:0] t0_sum_v8;
  wire [31:0] t_diff_v9;
  wire [31:0] t0_sum_v9;
  wire [31:0] t_diff_v10;
  wire [31:0] t0_sum_v10;
  wire [31:0] t_diff_v11;
  wire [31:0] t0_sum_v11;
  wire [31:0] t_diff_v12;
  wire [31:0] t0_sum_v12;
  wire [31:0] t_diff_v13;
  wire [31:0] t0_sum_v13;
  wire [31:0] t_diff_v14;
  wire [31:0] t0_sum_v14;
  wire [31:0] t_diff_v15;
  wire [31:0] t0_sum_v15;
  wire [31:0] t_diff_v16;
  wire [31:0] t0_sum_v16;
  wire [31:0] t_diff_v17;
  wire [31:0] t0_sum_v17;
  wire [31:0] t_diff_v18;
  wire [31:0] t0_sum_v18;
  wire [31:0] t_diff_v19;
  wire [31:0] t0_sum_v19;
  wire [31:0] t_diff_v20;
  wire [31:0] t0_sum_v20;
  wire [31:0] t_diff_v21;
  wire [31:0] t0_sum_v21;
  wire [31:0] t_diff_v22;
  wire [31:0] t0_sum_v22;
  wire [31:0] t_diff_v23;
  wire [31:0] t0_sum_v23;
  wire [31:0] t_diff_v24;
  wire [31:0] t0_sum_v24;
  wire [31:0] t_diff_v25;
  wire [31:0] t0_sum_v25;
  wire [31:0] t_diff_v26;
  wire [31:0] t0_sum_v26;
  wire [31:0] t_diff_v27;
  wire [31:0] t0_sum_v27;
  wire [31:0] t_diff_v28;
  wire [31:0] t0_sum_v28;
  wire [31:0] t_diff_v29;
  wire [31:0] t0_sum_v29;
  wire [31:0] t_diff_v30;
  wire [31:0] t0_sum_v30;
  wire [31:0] t_diff_v31;
  wire [31:0] t0_sum_v31;
  wire [31:0] t_diff_v32;
  wire [31:0] t0_sum_v32;
  wire [31:0] t_diff_v33;
  wire [31:0] t0_sum_v33;
  wire [31:0] t_diff_v34;
  wire [31:0] t0_sum_v34;
  wire [31:0] t_diff_v35;
  wire [31:0] t0_sum_v35;
  wire [31:0] t_diff_v36;
  wire [31:0] t0_sum_v36;
  wire [31:0] t_diff_v37;
  wire [31:0] t0_sum_v37;
  wire [31:0] t_diff_v38;
  wire [31:0] t0_sum_v38;
  wire [31:0] t_diff_v39;
  wire [31:0] t0_sum_v39;
  wire [31:0] t_diff_v40;
  wire [31:0] t0_sum_v40;
  wire [31:0] t_diff_v41;
  wire [31:0] t0_sum_v41;
  wire [31:0] t_diff_v42;
  wire [31:0] t0_sum_v42;
  wire [31:0] t_diff_v43;
  wire [31:0] t0_sum_v43;
  wire [31:0] t_diff_v44;
  wire [31:0] t0_sum_v44;
  wire [31:0] t_diff_v45;
  wire [31:0] t0_sum_v45;
  wire [31:0] t_diff_v46;
  wire [31:0] t0_sum_v46;
  wire [31:0] t_diff_v47;
  wire [31:0] t0_sum_v47;
  wire [31:0] t_diff_v48;
  wire [31:0] t0_sum_v48;
  wire [31:0] t_diff_v49;
  wire [31:0] t0_sum_v49;
  wire [31:0] t_diff_v50;
  wire [31:0] t0_sum_v50;
  wire [31:0] t_diff_v51;
  wire [31:0] t0_sum_v51;
  wire [31:0] t_diff_v52;
  wire [31:0] t0_sum_v52;
  wire [31:0] t_diff_v53;
  wire [31:0] t0_sum_v53;
  wire [31:0] t_diff_v54;
  wire [31:0] t0_sum_v54;
  wire [31:0] t_diff_v55;
  wire [31:0] t0_sum_v55;
  wire [31:0] t_diff_v56;
  wire [31:0] t0_sum_v56;
  wire [31:0] t_diff_v57;
  wire [31:0] t0_sum_v57;
  wire [31:0] t_diff_v58;
  wire [31:0] t0_sum_v58;
  wire [31:0] t_diff_v59;
  wire [31:0] t0_sum_v59;
  wire [31:0] t_diff_v60;
  wire [31:0] t0_sum_v60;
  wire [31:0] t_diff_v61;
  wire [31:0] t0_sum_v61;
  wire [31:0] t_diff_v62;
  wire [31:0] t0_sum_v62;
  wire [31:0] t_diff_v63;
  wire [31:0] t0_sum_v63;
  wire [31:0] t1_sum_v0;
  wire [31:0] t1_sum_v1;
  wire [31:0] t1_sum_v2;
  wire [31:0] t1_sum_v3;
  wire [31:0] t1_sum_v4;
  wire [31:0] t1_sum_v5;
  wire [31:0] t1_sum_v6;
  wire [31:0] t1_sum_v7;
  wire [31:0] t1_sum_v8;
  wire [31:0] t1_sum_v9;
  wire [31:0] t1_sum_v10;
  wire [31:0] t1_sum_v11;
  wire [31:0] t1_sum_v12;
  wire [31:0] t1_sum_v13;
  wire [31:0] t1_sum_v14;
  wire [31:0] t1_sum_v15;
  wire [31:0] t1_sum_v16;
  wire [31:0] t1_sum_v17;
  wire [31:0] t1_sum_v18;
  wire [31:0] t1_sum_v19;
  wire [31:0] t1_sum_v20;
  wire [31:0] t1_sum_v21;
  wire [31:0] t1_sum_v22;
  wire [31:0] t1_sum_v23;
  wire [31:0] t1_sum_v24;
  wire [31:0] t1_sum_v25;
  wire [31:0] t1_sum_v26;
  wire [31:0] t1_sum_v27;
  wire [31:0] t1_sum_v28;
  wire [31:0] t1_sum_v29;
  wire [31:0] t1_sum_v30;
  wire [31:0] t1_sum_v31;
  wire [31:0] t2_sum_v0;
  wire [31:0] t2_sum_v1;
  wire [31:0] t2_sum_v2;
  wire [31:0] t2_sum_v3;
  wire [31:0] t2_sum_v4;
  wire [31:0] t2_sum_v5;
  wire [31:0] t2_sum_v6;
  wire [31:0] t2_sum_v7;
  wire [31:0] t2_sum_v8;
  wire [31:0] t2_sum_v9;
  wire [31:0] t2_sum_v10;
  wire [31:0] t2_sum_v11;
  wire [31:0] t2_sum_v12;
  wire [31:0] t2_sum_v13;
  wire [31:0] t2_sum_v14;
  wire [31:0] t2_sum_v15;
  wire [31:0] t3_sum_v0;
  wire [31:0] t3_sum_v1;
  wire [31:0] t3_sum_v2;
  wire [31:0] t3_sum_v3;
  wire [31:0] t3_sum_v4;
  wire [31:0] t3_sum_v5;
  wire [31:0] t3_sum_v6;
  wire [31:0] t3_sum_v7;
  wire [31:0] t4_sum_v0;
  wire [31:0] t4_sum_v1;
  wire [31:0] t4_sum_v2;
  wire [31:0] t4_sum_v3;
  wire [31:0] t5_sum_v0;
  wire [31:0] t5_sum_v1;
  wire [31:0] t6_sum_v0;
  wire [31:0] t7_sum_v0;

  assign t_diff_v0 = v_0 - u_0;
  assign t0_sum_v0 = t_diff_v0 * t_diff_v0;
  assign t_diff_v1 = v_1 - u_1;
  assign t0_sum_v1 = t_diff_v1 * t_diff_v1;
  assign t_diff_v2 = v_2 - u_2;
  assign t0_sum_v2 = t_diff_v2 * t_diff_v2;
  assign t_diff_v3 = v_3 - u_3;
  assign t0_sum_v3 = t_diff_v3 * t_diff_v3;
  assign t_diff_v4 = v_4 - u_4;
  assign t0_sum_v4 = t_diff_v4 * t_diff_v4;
  assign t_diff_v5 = v_5 - u_5;
  assign t0_sum_v5 = t_diff_v5 * t_diff_v5;
  assign t_diff_v6 = v_6 - u_6;
  assign t0_sum_v6 = t_diff_v6 * t_diff_v6;
  assign t_diff_v7 = v_7 - u_7;
  assign t0_sum_v7 = t_diff_v7 * t_diff_v7;
  assign t_diff_v8 = v_8 - u_8;
  assign t0_sum_v8 = t_diff_v8 * t_diff_v8;
  assign t_diff_v9 = v_9 - u_9;
  assign t0_sum_v9 = t_diff_v9 * t_diff_v9;
  assign t_diff_v10 = v_10 - u_10;
  assign t0_sum_v10 = t_diff_v10 * t_diff_v10;
  assign t_diff_v11 = v_11 - u_11;
  assign t0_sum_v11 = t_diff_v11 * t_diff_v11;
  assign t_diff_v12 = v_12 - u_12;
  assign t0_sum_v12 = t_diff_v12 * t_diff_v12;
  assign t_diff_v13 = v_13 - u_13;
  assign t0_sum_v13 = t_diff_v13 * t_diff_v13;
  assign t_diff_v14 = v_14 - u_14;
  assign t0_sum_v14 = t_diff_v14 * t_diff_v14;
  assign t_diff_v15 = v_15 - u_15;
  assign t0_sum_v15 = t_diff_v15 * t_diff_v15;
  assign t_diff_v16 = v_16 - u_16;
  assign t0_sum_v16 = t_diff_v16 * t_diff_v16;
  assign t_diff_v17 = v_17 - u_17;
  assign t0_sum_v17 = t_diff_v17 * t_diff_v17;
  assign t_diff_v18 = v_18 - u_18;
  assign t0_sum_v18 = t_diff_v18 * t_diff_v18;
  assign t_diff_v19 = v_19 - u_19;
  assign t0_sum_v19 = t_diff_v19 * t_diff_v19;
  assign t_diff_v20 = v_20 - u_20;
  assign t0_sum_v20 = t_diff_v20 * t_diff_v20;
  assign t_diff_v21 = v_21 - u_21;
  assign t0_sum_v21 = t_diff_v21 * t_diff_v21;
  assign t_diff_v22 = v_22 - u_22;
  assign t0_sum_v22 = t_diff_v22 * t_diff_v22;
  assign t_diff_v23 = v_23 - u_23;
  assign t0_sum_v23 = t_diff_v23 * t_diff_v23;
  assign t_diff_v24 = v_24 - u_24;
  assign t0_sum_v24 = t_diff_v24 * t_diff_v24;
  assign t_diff_v25 = v_25 - u_25;
  assign t0_sum_v25 = t_diff_v25 * t_diff_v25;
  assign t_diff_v26 = v_26 - u_26;
  assign t0_sum_v26 = t_diff_v26 * t_diff_v26;
  assign t_diff_v27 = v_27 - u_27;
  assign t0_sum_v27 = t_diff_v27 * t_diff_v27;
  assign t_diff_v28 = v_28 - u_28;
  assign t0_sum_v28 = t_diff_v28 * t_diff_v28;
  assign t_diff_v29 = v_29 - u_29;
  assign t0_sum_v29 = t_diff_v29 * t_diff_v29;
  assign t_diff_v30 = v_30 - u_30;
  assign t0_sum_v30 = t_diff_v30 * t_diff_v30;
  assign t_diff_v31 = v_31 - u_31;
  assign t0_sum_v31 = t_diff_v31 * t_diff_v31;
  assign t_diff_v32 = v_32 - u_32;
  assign t0_sum_v32 = t_diff_v32 * t_diff_v32;
  assign t_diff_v33 = v_33 - u_33;
  assign t0_sum_v33 = t_diff_v33 * t_diff_v33;
  assign t_diff_v34 = v_34 - u_34;
  assign t0_sum_v34 = t_diff_v34 * t_diff_v34;
  assign t_diff_v35 = v_35 - u_35;
  assign t0_sum_v35 = t_diff_v35 * t_diff_v35;
  assign t_diff_v36 = v_36 - u_36;
  assign t0_sum_v36 = t_diff_v36 * t_diff_v36;
  assign t_diff_v37 = v_37 - u_37;
  assign t0_sum_v37 = t_diff_v37 * t_diff_v37;
  assign t_diff_v38 = v_38 - u_38;
  assign t0_sum_v38 = t_diff_v38 * t_diff_v38;
  assign t_diff_v39 = v_39 - u_39;
  assign t0_sum_v39 = t_diff_v39 * t_diff_v39;
  assign t_diff_v40 = v_40 - u_40;
  assign t0_sum_v40 = t_diff_v40 * t_diff_v40;
  assign t_diff_v41 = v_41 - u_41;
  assign t0_sum_v41 = t_diff_v41 * t_diff_v41;
  assign t_diff_v42 = v_42 - u_42;
  assign t0_sum_v42 = t_diff_v42 * t_diff_v42;
  assign t_diff_v43 = v_43 - u_43;
  assign t0_sum_v43 = t_diff_v43 * t_diff_v43;
  assign t_diff_v44 = v_44 - u_44;
  assign t0_sum_v44 = t_diff_v44 * t_diff_v44;
  assign t_diff_v45 = v_45 - u_45;
  assign t0_sum_v45 = t_diff_v45 * t_diff_v45;
  assign t_diff_v46 = v_46 - u_46;
  assign t0_sum_v46 = t_diff_v46 * t_diff_v46;
  assign t_diff_v47 = v_47 - u_47;
  assign t0_sum_v47 = t_diff_v47 * t_diff_v47;
  assign t_diff_v48 = v_48 - u_48;
  assign t0_sum_v48 = t_diff_v48 * t_diff_v48;
  assign t_diff_v49 = v_49 - u_49;
  assign t0_sum_v49 = t_diff_v49 * t_diff_v49;
  assign t_diff_v50 = v_50 - u_50;
  assign t0_sum_v50 = t_diff_v50 * t_diff_v50;
  assign t_diff_v51 = v_51 - u_51;
  assign t0_sum_v51 = t_diff_v51 * t_diff_v51;
  assign t_diff_v52 = v_52 - u_52;
  assign t0_sum_v52 = t_diff_v52 * t_diff_v52;
  assign t_diff_v53 = v_53 - u_53;
  assign t0_sum_v53 = t_diff_v53 * t_diff_v53;
  assign t_diff_v54 = v_54 - u_54;
  assign t0_sum_v54 = t_diff_v54 * t_diff_v54;
  assign t_diff_v55 = v_55 - u_55;
  assign t0_sum_v55 = t_diff_v55 * t_diff_v55;
  assign t_diff_v56 = v_56 - u_56;
  assign t0_sum_v56 = t_diff_v56 * t_diff_v56;
  assign t_diff_v57 = v_57 - u_57;
  assign t0_sum_v57 = t_diff_v57 * t_diff_v57;
  assign t_diff_v58 = v_58 - u_58;
  assign t0_sum_v58 = t_diff_v58 * t_diff_v58;
  assign t_diff_v59 = v_59 - u_59;
  assign t0_sum_v59 = t_diff_v59 * t_diff_v59;
  assign t_diff_v60 = v_60 - u_60;
  assign t0_sum_v60 = t_diff_v60 * t_diff_v60;
  assign t_diff_v61 = v_61 - u_61;
  assign t0_sum_v61 = t_diff_v61 * t_diff_v61;
  assign t_diff_v62 = v_62 - u_62;
  assign t0_sum_v62 = t_diff_v62 * t_diff_v62;
  assign t_diff_v63 = v_63 - u_63;
  assign t0_sum_v63 = t_diff_v63 * t_diff_v63;
  assign t1_sum_v0 = t0_sum_v0 + t0_sum_v1;
  assign t1_sum_v1 = t0_sum_v2 + t0_sum_v3;
  assign t1_sum_v2 = t0_sum_v4 + t0_sum_v5;
  assign t1_sum_v3 = t0_sum_v6 + t0_sum_v7;
  assign t1_sum_v4 = t0_sum_v8 + t0_sum_v9;
  assign t1_sum_v5 = t0_sum_v10 + t0_sum_v11;
  assign t1_sum_v6 = t0_sum_v12 + t0_sum_v13;
  assign t1_sum_v7 = t0_sum_v14 + t0_sum_v15;
  assign t1_sum_v8 = t0_sum_v16 + t0_sum_v17;
  assign t1_sum_v9 = t0_sum_v18 + t0_sum_v19;
  assign t1_sum_v10 = t0_sum_v20 + t0_sum_v21;
  assign t1_sum_v11 = t0_sum_v22 + t0_sum_v23;
  assign t1_sum_v12 = t0_sum_v24 + t0_sum_v25;
  assign t1_sum_v13 = t0_sum_v26 + t0_sum_v27;
  assign t1_sum_v14 = t0_sum_v28 + t0_sum_v29;
  assign t1_sum_v15 = t0_sum_v30 + t0_sum_v31;
  assign t1_sum_v16 = t0_sum_v32 + t0_sum_v33;
  assign t1_sum_v17 = t0_sum_v34 + t0_sum_v35;
  assign t1_sum_v18 = t0_sum_v36 + t0_sum_v37;
  assign t1_sum_v19 = t0_sum_v38 + t0_sum_v39;
  assign t1_sum_v20 = t0_sum_v40 + t0_sum_v41;
  assign t1_sum_v21 = t0_sum_v42 + t0_sum_v43;
  assign t1_sum_v22 = t0_sum_v44 + t0_sum_v45;
  assign t1_sum_v23 = t0_sum_v46 + t0_sum_v47;
  assign t1_sum_v24 = t0_sum_v48 + t0_sum_v49;
  assign t1_sum_v25 = t0_sum_v50 + t0_sum_v51;
  assign t1_sum_v26 = t0_sum_v52 + t0_sum_v53;
  assign t1_sum_v27 = t0_sum_v54 + t0_sum_v55;
  assign t1_sum_v28 = t0_sum_v56 + t0_sum_v57;
  assign t1_sum_v29 = t0_sum_v58 + t0_sum_v59;
  assign t1_sum_v30 = t0_sum_v60 + t0_sum_v61;
  assign t1_sum_v31 = t0_sum_v62 + t0_sum_v63;

  assign t2_sum_v0 = t1_sum_v0 + t1_sum_v1;
  assign t2_sum_v1 = t1_sum_v2 + t1_sum_v3;
  assign t2_sum_v2 = t1_sum_v4 + t1_sum_v5;
  assign t2_sum_v3 = t1_sum_v6 + t1_sum_v7;
  assign t2_sum_v4 = t1_sum_v8 + t1_sum_v9;
  assign t2_sum_v5 = t1_sum_v10 + t1_sum_v11;
  assign t2_sum_v6 = t1_sum_v12 + t1_sum_v13;
  assign t2_sum_v7 = t1_sum_v14 + t1_sum_v15;
  assign t2_sum_v8 = t1_sum_v16 + t1_sum_v17;
  assign t2_sum_v9 = t1_sum_v18 + t1_sum_v19;
  assign t2_sum_v10 = t1_sum_v20 + t1_sum_v21;
  assign t2_sum_v11 = t1_sum_v22 + t1_sum_v23;
  assign t2_sum_v12 = t1_sum_v24 + t1_sum_v25;
  assign t2_sum_v13 = t1_sum_v26 + t1_sum_v27;
  assign t2_sum_v14 = t1_sum_v28 + t1_sum_v29;
  assign t2_sum_v15 = t1_sum_v30 + t1_sum_v31;

  assign t3_sum_v0 = t2_sum_v0 + t2_sum_v1;
  assign t3_sum_v1 = t2_sum_v2 + t2_sum_v3;
  assign t3_sum_v2 = t2_sum_v4 + t2_sum_v5;
  assign t3_sum_v3 = t2_sum_v6 + t2_sum_v7;
  assign t3_sum_v4 = t2_sum_v8 + t2_sum_v9;
  assign t3_sum_v5 = t2_sum_v10 + t2_sum_v11;
  assign t3_sum_v6 = t2_sum_v12 + t2_sum_v13;
  assign t3_sum_v7 = t2_sum_v14 + t2_sum_v15;

  assign t4_sum_v0 = t3_sum_v0 + t3_sum_v1;
  assign t4_sum_v1 = t3_sum_v2 + t3_sum_v3;
  assign t4_sum_v2 = t3_sum_v4 + t3_sum_v5;
  assign t4_sum_v3 = t3_sum_v6 + t3_sum_v7;

  assign t5_sum_v0 = t4_sum_v0 + t4_sum_v1;
  assign t5_sum_v1 = t4_sum_v2 + t4_sum_v3;

  assign t6_sum_v0 = t5_sum_v0 + t5_sum_v1;

  assign t7_sum_v0 = t6_sum_v0;

  assign dist = t7_sum_v0;
endmodule
