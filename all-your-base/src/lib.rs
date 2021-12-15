#[derive(Debug, PartialEq)]
pub enum Error {
    InvalidInputBase,
    InvalidOutputBase,
    InvalidDigit(u32),
}

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

    let number_at_base = number
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

    Ok(get_number_at_new_base(vec![], number_at_base, to_base))
}

fn get_number_at_new_base(mut remainder_stack: Vec<u32>, actual: u32, to_base: u32) -> Vec<u32> {
    if actual == 0 {
        return match remainder_stack.is_empty() {
            true => {
                vec![0]
            }
            false => {
                remainder_stack.reverse();
                remainder_stack
            }
        };
    }

    let remainder = actual % to_base;
    let next = actual / to_base;

    remainder_stack.push(remainder);

    get_number_at_new_base(remainder_stack, next, to_base)
}
