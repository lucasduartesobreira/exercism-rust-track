#[macro_export]
macro_rules! hashmap {
    () => {::std::collections::HashMap::new()};
    ($($key:expr => $val:expr,)+) => { crate::hashmap!($($key => $val),+) };
    ($($key:expr => $val:expr),*) => {
    {
        use ::std::collections::HashMap;
        let mut hm = HashMap::new();
        $(hm.insert( $key, $val );)*
        hm
    }
    };
}
