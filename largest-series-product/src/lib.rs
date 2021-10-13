#[derive(Debug, PartialEq)]
pub enum Error {
    SpanTooLong,
    InvalidDigit(char),
}

pub fn lsp(string_digits: &str, span: usize) -> Result<u64, Error> {
    match (string_digits.len(), span) {
        (x, y) if y > x => Err(Error::SpanTooLong),
        (_, 0) => Ok(1),
        _ => {
            let normalized_vec_of_windows = string_digits
                .chars()
                .collect::<Vec<_>>()
                .windows(span)
                .map(|chars| {
                    chars
                        .iter()
                        .map(|&c| c.to_digit(10).ok_or(Error::InvalidDigit(c)))
                        .collect::<Result<Vec<_>, _>>()
                })
                .collect::<Result<Vec<_>, Error>>()?;

            Ok(normalized_vec_of_windows
                .into_iter()
                .map(|vecs| vecs.into_iter().map(u64::from).product())
                .max()
                .unwrap())
        }
    }
    /*
     *unimplemented!(
     *    "largest series product of a span of {} digits in {}",
     *    span,
     *    string_digits
     *);
     */
}
