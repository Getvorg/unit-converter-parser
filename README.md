unit-converter-parser
 
This is a unit conversion parser built using Rust and the Pest parsing library. It converts various units of measurement (e.g., miles, feet, inches, kilometers, meters, and centimeters) based on user input. The input string follows a simple format where a number is followed by a unit, and the program returns a conversion result.

Parsing Process

The parser uses a custom grammar defined in the `grammar.pest` file to match input strings. The grammar specifies the structure of the input, such as the number and the unit of measurement. Here is the basic structure of the grammar:

- `conversion`: This rule matches a number followed by a unit, such as `10 miles` or `5 km`.
- `number`: This rule matches a sequence of digits, which represents the numeric part of the input.
- `unit`: This rule matches one of the supported units, including `miles`, `feet`, `inches`, `km`, `m`, and `cm`.

Once the input string is parsed, the `convert_units` function performs the unit conversion by using predefined conversion rates, such as:
- 1 mile = 1.60934 kilometers
- 1 foot = 0.3048 meters
- 1 inch = 2.54 centimeters
- 1 kilometer = 0.621371 miles
- 1 meter = 3.28084 feet
- 1 centimeter = 0.393701 inches

Example

For example, if the user inputs `10 miles`, the output will be `10 miles = 16.0934 km`.

The program handles errors gracefully by reporting invalid input or unsupported units.

Usage

To run the program, simply execute it with the unit measurement as an argument. For example:

cargo run "10 miles" 10 miles = 16.0934 km

