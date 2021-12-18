#[macro_export]
macro_rules! hashmap {
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
