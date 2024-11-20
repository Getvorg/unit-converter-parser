use anyhow::Result;
use unit_converter_parser::convert_units;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_number_positive() -> Result<()> {
        let result = convert_units("1234 miles")?;
        assert_eq!(result, "1234 miles = 1982.754 km");
        Ok(())
    }

    #[test]
    fn test_number_negative() -> Result<()> {
        let result = convert_units("-1234 miles")?;
        assert_eq!(result, "-1234 miles = -1982.754 km");
        Ok(())
    }

    #[test]
    fn test_number_decimal() -> Result<()> {
        let result = convert_units("3.14 miles")?;
        assert_eq!(result, "3.14 miles = 5.0534 km");
        Ok(())
    }

    #[test]
    fn test_number_signed_decimal() -> Result<()> {
        let result = convert_units("-3.14 miles")?;
        assert_eq!(result, "-3.14 miles = -5.0534 km");
        Ok(())
    }

    #[test]
    fn test_valid_unit_miles() -> Result<()> {
        let result = convert_units("10 miles")?;
        assert_eq!(result, "10 miles = 16.0934 km");
        Ok(())
    }

    #[test]
    fn test_valid_unit_feet() -> Result<()> {
        let result = convert_units("5 feet")?;
        assert_eq!(result, "5 feet = 1.524 meters");
        Ok(())
    }

    #[test]
    fn test_valid_unit_inches() -> Result<()> {
        let result = convert_units("10 inches")?;
        assert_eq!(result, "10 inches = 25.4 cm");
        Ok(())
    }

    #[test]
    fn test_invalid_unit() -> Result<()> {
        let result = convert_units("10 yards");
        assert!(result.is_err());
        Ok(())
    }
}
