#[derive(Debug, PartialEq)]
pub enum Error {
    SpanTooLong,
    InvalidDigit(char),
}

pub fn lsp(string_digits: &str, span: usize) -> Result<u64, Error> {
    match span {
        0 => Ok(1),
        _ => Ok(u64::from(
            string_digits
                .chars()
                .map(|c| c.to_digit(10).ok_or(Error::InvalidDigit(c)))
                .collect::<Result<Vec<u32>, Error>>()?
                .windows(span)
                .map(|window| window.iter().product::<u32>())
                .max()
                .ok_or(Error::SpanTooLong)?,
        )),
    }
}
