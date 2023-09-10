#[macro_export]
macro_rules! my_hash_map {
    ($( $key:expr => $value: expr),*) => {
        {
            let mut my_map = HashMap::new();
            $(
                my_map.insert($key, $value);
            )*
            my_map
        }
    };
}