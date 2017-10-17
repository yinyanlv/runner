#![allow(unused_macros)]

macro_rules! gen_hashmap {

    (@single $($e: tt)*) => (());

    (@count $($e: expr)*) => (<[()]>::len(&[$(gen_hashmap!(@single $e)),*]));

    ($($key: expr => $value: expr,)+) => (gen_hashmap!($($key => $value),+));

    {$($key: expr => $value: expr),*} => {

        {
            let count = gen_hashmap!(@count $($key)*);
            let mut hashmap = ::std::collections::HashMap::with_capacity(count);

            $(
                hashmap.insert($key, $value);
            )*

            hashmap
        }
    }
}
