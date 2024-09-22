# Temperature Converter

This Rust program provides a simple temperature conversion utility (i did it for practice). It can convert temperatures between Fahrenheit, Celsius, and Kelvin scales.

## Features

- Converts temperatures between Fahrenheit, Celsius, and Kelvin
- Handles a variety of input formats, including decimals and negative temperatures
- Includes error handling for invalid inputs
- Provides comprehensive test cases to ensure accuracy

## How it works

The main function `smart_convert_temp` takes a string input and attempts to parse it as a temperature. It then:

1. Identifies the temperature scale (F, C, or K)
2. Parses the numerical value
3. Converts the temperature to the other two scales (i dont wanna ask the user cuz frankly i dont care)
4. Returns the results formatted to three decimal places

## Usage

Run the program to see conversions for a predefined set of test cases. These cases cover various scenarios including:

- Normal temperatures
- Edge cases (e.g., absolute zero)
- Decimal values
- Negative temperatures
- Invalid inputs

## Note

This implementation does not currently support scientific notation for temperature inputs.
It also freaks out when you try to say words like i said markiplier there, it thought its a Kelvin scale becuase the first letter it found was K in mark 

Feel free to modify the test cases or integrate the function into your own projects!

(if the whole readme feels weird i asked claude to write it for me cuz i cant be f#!&ed)
