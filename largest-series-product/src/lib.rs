#[derive(Debug, PartialEq)]
pub enum Error {
    SpanTooLong,
    InvalidDigit(char),
}

pub fn lsp(string_digits: &str, span: usize) -> Result<u64, Error> {
    if span > string_digits.len() {
        return Err(Error::SpanTooLong);
    }
    if string_digits.is_empty() || span == 0 {
        return Ok(1);
    }
    Ok(string_digits
        .chars()
        .collect::<Vec<_>>()
        .windows(span)
        .map(|chars| {
            chars
                .iter()
                .map(|&c| c.to_digit(10).ok_or(Error::InvalidDigit(c)))
                .collect::<Result<Vec<_>, _>>()
                .into_iter()
                .flat_map(|vec| vec.into_iter().map(u64::from))
                .product()
        })
        .max()
        .unwrap())
    /*
     *unimplemented!(
     *    "largest series product of a span of {} digits in {}",
     *    span,
     *    string_digits
     *);
     */
}
