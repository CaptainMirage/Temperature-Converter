fn main() {
    // a smaller set of test cases
    let _test_cases_mini = ["100 F", "257 F", "999999999 F", "000 F", "01 F", "-1 F", "0 F"];

    // these test cases also check for edge cases like negative, zero, large numbers
    let _test_cases = [
        // Normal cases
        "100 F", "0 C", "273.15 K",
        // Edge cases
        "-459.67 F", // Absolute zero in Fahrenheit
        "-273.15 C", // Absolute zero in Celsius
        "0 K",       // Absolute zero in Kelvin
        "5_000_000 F", // Extremely high temperature
        // Decimal cases
        "98.6 F", "37.0 C", "310.15 K",
        // Leading zeros
        "000.5 F", "01.0 C", "0273.15 K",
        // Negative temperatures
        "-40 F", "-40 C", "-100 K",
        // Zero
        "0 F", "0 C", "0 K",
        // Invalid cases
        "100", "F", "100F", "100 X", "ABC", "12.34.56 F",
        // Random words that should be Invalid
        "hello everybody my name is markiplier", "mark100plier_f", "mark100plier f",
        // Cases with extra spaces
        "  100   F  ", " 0  C ", "273.15    K   ",
        // Cases with different casings
        "100 f", "0 c", "273.15 k",
        // Very small numbers
        "0.000001 F", "0.0000001 C", "0.00000001 K",
        // Very large numbers
        "9999999999999999 F", "9999999999999999 C", "9999999999999999 K",
        // Scientific notation (note: this will not work with the current implementation
        //                      and I can't be f^$#ed to implement that)
        "1e2 F", "1e-2 C", "1e+2 K",
    ];

    // goes through each element of array and
    for case in _test_cases {
        match smart_convert_temp(case) {
            Some((_char , (result1, temp1), (result2, temp2))) => println!(
                "temperature: {}\n    temp_val: {}\n    Conversion to {}: {}\n    Conversion to {}: {}\n",
                case, _char, temp1, result1, temp2, result2,
            ),
            None => println!("Input: {}, No valid letter found or couldn't parse number\n", case),
        }
    }
}

fn smart_convert_temp(input: &str) -> Option<(char, (String, char), (String, char))> {
    // turns everything into uppercase for no reason
    let input_upper = input.to_uppercase();

    // tries to find the letters 'F', 'C' and 'K'
    let temp_val = match input_upper.chars()
        .find(|&c| c == 'F' || c == 'C' || c == 'K') {
        Some(l) => l,
        None => return None, // returns None if nothing is found
    };

    // removes any character other than numbers
    // also excludes '.' and '-' for floating and negative values
    let tempt: f64 = input_upper
        .chars()
        .filter(|c| c.is_digit(10) || *c == '.' || *c == '-')
        .collect::<String>()
        .parse()
        .ok()?;

    // converts the temperature into the other two formats and returns the value
    let (result1, result2) = match temp_val {
        'F' => ((format!("{:.3} K", (tempt - 32.0) * 5.0/9.0 + 273.15), 'K')
                , (format!("{:.3} C", (tempt - 32.0) * 5.0/9.0), 'C')
        ),
        'C' => ((format!("{:.3} K", tempt + 273.15), 'K')
                , (format!("{:.3} F", (tempt * 9.0/5.0) + 32.0), 'F')
        ),
        'K' => ((format!("{:.3} C", tempt - 273.15), 'C')
                , (format!("{:.3} F", (tempt - 273.15) * 9.0/5.0 + 32.0), 'F')
        ),
        _ => unreachable!(), // This case should never happen if it does the code panics
    };
    Some((temp_val ,result1, result2)) // gives values to some to return in the top
}