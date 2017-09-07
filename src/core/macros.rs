#![allow(unused_macros)]

macro_rules! gen_tuple {

    {$($item: expr),*} => {

        ($($item, )*)
    }
}

macro_rules! gen_hashmap {

    {$($key: expr => $value: expr),*} => {

        {
            let mut hashmap = HashMap::new();

            $(
                hashmap.insert($key, $value);
            )*

            hashmap
        }
    }

}