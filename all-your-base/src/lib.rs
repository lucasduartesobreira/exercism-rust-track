#[derive(Debug, PartialEq)]
pub enum Error {
    InvalidInputBase,
    InvalidOutputBase,
    InvalidDigit(u32),
}

///
/// Convert a number between two bases.
///
/// A number is any slice of digits.
/// A digit is any unsigned integer (e.g. u8, u16, u32, u64, or usize).
/// Bases are specified as unsigned integers.
///
/// Return an `Err(.)` if the conversion is impossible.
/// The tests do not test for specific values inside the `Err(.)`.
///
///
/// You are allowed to change the function signature as long as all test still pass.
///
///
/// Example:
/// Input
///   number: &[4, 2]
///   from_base: 10
///   to_base: 2
/// Result
///   Ok(vec![1, 0, 1, 0, 1, 0])
///
/// The example corresponds to converting the number 42 from decimal
/// which is equivalent to 101010 in binary.
///
///
/// Notes:
///  * The empty slice ( "[]" ) is equal to the number 0.
///  * Never output leading 0 digits, unless the input number is 0, in which the output must be `[0]`.
///    However, your function must be able to process input with leading 0 digits.
///
pub fn convert(number: &[u32], from_base: u32, to_base: u32) -> Result<Vec<u32>, Error> {
    if from_base <= 1 {
        return Err(Error::InvalidInputBase);
    }

    if to_base <= 1 {
        return Err(Error::InvalidOutputBase);
    }

    if number.is_empty() {
        return Ok(vec![0]);
    }

    let mut number_at_base =
        number
            .iter()
            .rev()
            .enumerate()
            .try_fold(0u32, |acc, (index, &k)| {
                if k >= from_base {
                    Err(Error::InvalidDigit(k))
                } else {
                    Ok(acc + k * (from_base.pow(index as u32)))
                }
            })?;

    let mut number_at_new_base = vec![];

    while number_at_base >= 1 {
        let result = number_at_base / to_base;
        let remainder = number_at_base % to_base;

        number_at_new_base.push(remainder);
        number_at_base = result;
    }

    if number_at_new_base.is_empty() {
        Ok(vec![0])
    } else {
        number_at_new_base.reverse();
        Ok(number_at_new_base)
    }
}

