macro_rules! test_syntax {
    ($name:ident: $input:literal => $output:literal) => {
        #[test]
        fn $name() {
            let input_expr = $crate::compiler::parser::parse_source($input.to_string());
            let output_expr = $crate::compiler::parser::parse_source($output.to_string());
            println!("{}: {} => {}", stringify!($name), input_expr, output_expr);
            let input_expr = super::evaluate_constants(input_expr);
            let output_expr = super::evaluate_constants(output_expr);
            assert_eq!(input_expr, output_expr);
        }
    };
}

// Multiplication
test_syntax!( mul_zero_left:  "x * 0.0"   => "0.0" );
test_syntax!( mul_zero_right: "0.0 * x"   => "0.0" );
test_syntax!( mul_one_left:   "x * 1.0"   => "x"   );
test_syntax!( mul_one_right:  "1.0 * x"   => "x"   );
test_syntax!( mul_num:        "2.0 * 2.0" => "4.0" );

// Division
test_syntax!( div_zero_left:  "x / 0.0"     => "0.0" );
test_syntax!( div_zero_right: "0.0 / x"     => "0.0" );
test_syntax!( div_one_left:   "x / 1.0"     => "x"   );
test_syntax!( div_equal_x:    "x / x"       => "1.0" );
test_syntax!( div_equal_y:    "y / y"       => "1.0" );
test_syntax!( div_equal_num:  "10.0 / 10.0" => "1.0" );
test_syntax!( div_num:        "10.0 / 2.0"  => "5.0" );

// Addition
test_syntax!( add_zero_left:  "x + 0.0"   => "x"   );
test_syntax!( add_zero_right: "0.0 + x"   => "x"   );
test_syntax!( add_num:        "2.0 + 2.0" => "4.0" );

// Subtraction
test_syntax!( sub_zero_left:  "x - 0.0"   => "x"   );
test_syntax!( sub_zero_right: "0.0 - x"   => "-x"  );

// Modulo
test_syntax!( mod_zero_left:  "x % 0.0"     => "0.0" );
test_syntax!( mod_zero_right: "0.0 % x"     => "0.0" );
test_syntax!( mod_num:        "10.0 % 3.0"  => "1.0" );

// Power
test_syntax!( pow_zero_left:  "0.0 ^ x"   => "0.0" );
test_syntax!( pow_zero_right: "x ^ 0.0"   => "1.0" );
test_syntax!( pow_one_left:   "1.0 ^ x"   => "1.0" );
test_syntax!( pow_one_right:  "x ^ 1.0"   => "x"   );
test_syntax!( pow_num:        "2.0 ^ 2.0" => "4.0" );

// Trig Functions
test_syntax!( sin: "sin(0.0)" => "0.0" );
test_syntax!( cos: "cos(0.0)" => "1.0" );

// Parentheses
test_syntax!( paren:     "(1.0)"     => "1.0" );
test_syntax!( paren_bin: "(1.0/2.0)" => "0.5" );

// Absolute Value
test_syntax!( abs_pos: "|1.0|"  => "1.0" );
test_syntax!( abs_neg: "|-1.0|" => "1.0" );

// Boolean Operations (less, greater, or/max, and/min)
test_syntax!( less_true:     "1.0 < 2.0" => "1.0"  );
test_syntax!( less_false:    "2.0 < 1.0" => "-1.0" );
test_syntax!( greater_true:  "2.0 > 1.0" => "1.0"  );
test_syntax!( greater_false: "1.0 > 2.0" => "-1.0" );
test_syntax!( or:            "1.0 | 0.0" => "1.0"  );
test_syntax!( and:           "1.0 & 0.0" => "0.0"  );
