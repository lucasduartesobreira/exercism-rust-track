#[derive(Debug, PartialEq)]
pub enum Comparison {
    Equal,
    Sublist,
    Superlist,
    Unequal,
}

pub fn sublist<T: PartialEq>(first_list: &[T], second_list: &[T]) -> Comparison {
    match (first_list, second_list) {
        (x, y) if is_equal(x, y) => Comparison::Equal,
        (x, y) if is_an_superlist(x, y) => Comparison::Superlist,
        (x, y) if is_an_sublist(x, y) => Comparison::Sublist,
        _ => Comparison::Unequal,
    }
}

fn is_equal<T: PartialEq>(first_list: &[T], second_list: &[T]) -> bool {
    if first_list.len() == second_list.len() {
        return first_list == second_list;
    }
    false
}

fn is_an_superlist<T: PartialEq>(first_list: &[T], second_list: &[T]) -> bool {
    if first_list.len() > second_list.len() {
        return second_list.is_empty() || is_an_sublist(second_list, first_list);
    }
    false
}

fn is_an_sublist<T: PartialEq>(first_list: &[T], second_list: &[T]) -> bool {
    if first_list.len() < second_list.len() {
        return first_list.is_empty()
            || second_list
                .windows(first_list.len())
                .any(|x| x[0] == first_list[0] && x[1..] == first_list[1..]);
    }
    false
}
