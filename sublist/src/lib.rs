#[derive(Debug, PartialEq)]
pub enum Comparison {
    Equal,
    Sublist,
    Superlist,
    Unequal,
}

pub fn sublist<T: PartialEq>(first_list: &[T], second_list: &[T]) -> Comparison {
    if is_equal(first_list, second_list) {
        return Comparison::Equal;
    }

    if is_an_superlist(first_list, second_list) {
        return Comparison::Superlist;
    }

    if is_an_sublist(first_list, second_list) {
        return Comparison::Sublist;
    }

    Comparison::Unequal
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
            || second_list.iter().enumerate().any(|(index, value)| {
                if *value == first_list[0] && index + first_list.len() <= second_list.len() {
                    return first_list == &second_list[index..(index + first_list.len())];
                }
                false
            });
    }
    false
}
