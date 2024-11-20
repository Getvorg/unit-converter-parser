use pest::Parser;
use pest_derive::Parser;
use thiserror::Error;

#[derive(Parser)]
#[grammar = "grammar.pest"]
pub struct UnitParser;

#[derive(Error, Debug)]
pub enum UnitConversionError {
    #[error("Invalid unit: {0}")]
    InvalidUnit(String),

    #[error("Invalid number format: {0}")]
    InvalidNumberFormat(String),

    #[error("Conversion failed: {0}")]
    ConversionFailed(String),
}

pub fn convert_units(input: &str) -> Result<String, UnitConversionError> {
    let pairs = UnitParser::parse(Rule::conversion, input)
        .map_err(|_| UnitConversionError::ConversionFailed("Failed to parse input".into()))?;

    for pair in pairs {
        match pair.as_rule() {
            Rule::conversion => {
                let mut inner_pairs = pair.into_inner();

                let value_str = inner_pairs.next().unwrap().as_str();
                let value: f64 = value_str
                    .parse()
                    .map_err(|_| UnitConversionError::InvalidNumberFormat(value_str.into()))?;

                let unit = inner_pairs.next().unwrap().as_str();

                return Ok(match unit {
                    "miles" => format!("{} miles = {} km", value, value * 1.60934),
                    "feet" => format!("{} feet = {} meters", value, value * 0.3048),
                    "inches" => format!("{} inches = {} cm", value, value * 2.54),
                    "km" => format!("{} km = {} miles", value, value / 1.60934),
                    "m" => format!("{} meters = {} feet", value, value / 0.3048),
                    "cm" => format!("{} cm = {} inches", value, value / 2.54),
                    _ => return Err(UnitConversionError::InvalidUnit(unit.into())),
                });
            }
            _ => {}
        }
    }
    Err(UnitConversionError::ConversionFailed(
        "Invalid input".into(),
    ))
}
