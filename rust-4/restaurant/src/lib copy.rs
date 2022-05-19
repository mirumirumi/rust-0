
mod front_of_house {
    pub mod hosting {
        pub fn add_to_waitlist() {}

        fn seat_at_table() {}
    }

    mod serving {
        fn take_order() {}

        fn serve_order() {}

        fn take_payment() {}
    }
}

use crate::front_of_house::hosting;

pub fn eat_at_restaurant() {
    crate::hosting::add_to_waitlist();

    hosting::add_to_waitlist();

    // こうやって１つ上から書くのが慣例。その関数がローカルで定義されていないことを明らかにできるから。
    
    // 反対に、構造体やenumをuseするときはフルパスで書くのは習わしだとのこと。
    // まさかの理由はないということで、自然に発生してみんなそう書くようになったかららしい。なんでそこだけ適当なんだ…。
}

use std::fmt::Result;
use std::io::Result as IoResult;


use std::{cmp::Ordering, io};

// ↓

use std::cmp::Ordering;
use std::io;



use std::io::{self, Write};
// ↓
use std::io;
use std::io::Write;



use std::collections::*;





#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        let result = 2 + 2;
        assert_eq!(result, 4);
    }
}
