//! 这是我用来写排序算法示例程序的

/// 提供同种类型的`lt`比较函数,
/// 将其扩展到`<` `<=` `>` `>=`
///
/// 这可以以更可读的方式使用`lt`
#[macro_export]
macro_rules! cmp {
    ($cmp:expr => $a:expr, (<), $b:expr $(,)?) => {
        $cmp($a, $b)
    };
    ($cmp:expr => $a:expr, (<=), $b:expr $(,)?) => {
        ! $cmp($b, $a)
    };
    ($cmp:expr => $a:expr, (>), $b:expr $(,)?) => {
        $cmp($b, $a)
    };
    ($cmp:expr => $a:expr, (>=), $b:expr $(,)?) => {
        ! $cmp($a, $b)
    };
}

pub mod sorts;

#[cfg(test)]
mod tests {
    use rand::seq::SliceRandom;
    use super::data::Data;
    use crate::sorts::{
        insert_sort::*,
    };

    #[test]
    fn ptr_cmp_test() {
        let arr = [2, 1];
        let [a, b]: [*const i32; 2] = [&arr[0], &arr[1]];
        assert!(a < b);
        assert_eq!(a, a);
        assert_eq!(b, b);

        assert!(! (a >= b));

        let arr = Box::new([2, 1]);
        let [a, b]: [*const i32; 2] = [&arr[0], &arr[1]];
        assert!(a < b);
        assert_eq!(a, a);
        assert_eq!(b, b);

        assert!(! (a >= b));
    }

    #[test]
    fn cmp_macro_test() {
        let lt = i32::lt;

        assert!(cmp!(lt => &1, (<), &2));
        assert!(cmp!(lt => &1, (<=), &2));
        assert!(cmp!(lt => &1, (<=), &1));
        assert!(cmp!(lt => &2, (>), &1));
        assert!(cmp!(lt => &2, (>=), &1));
        assert!(cmp!(lt => &1, (>=), &1));

        assert!(! cmp!(lt => &1, (<), &1));
        assert!(! cmp!(lt => &1, (<=), &0));
        assert!(! cmp!(lt => &1, (>), &1));
        assert!(! cmp!(lt => &0, (>=), &1));
    }

    #[test]
    fn insert_sort_example_test() {
        let mut arr = Data::new_vec(1000);
        arr.shuffle(&mut rand::thread_rng());
        let mut arr1 = arr.clone();
        arr.sort();
        insert_sort_by_lt_example(&mut arr1, Data::lt);
        assert_eq!(arr, arr1);
    }

    #[test]
    fn insert_sort_test() {
        let mut arr = Data::new_vec(1000);
        arr.shuffle(&mut rand::thread_rng());
        let mut arr1 = arr.clone();
        arr.sort();
        insert_sort(&mut arr1);
        assert_eq!(arr, arr1);
    }
}
#[cfg(test)]
pub(crate) mod data {
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Ord)]
    pub(crate) struct Data {
        num: i32,
        id: i32,
    }
    impl Data {
        pub fn new(num: i32, id: i32) -> Self { Self { num, id } }
        pub fn new_vec(count: usize) -> Vec<Self> {
            let mut id = 0;
            Vec::from_iter((0..count).map(|n| {
                let res = Self::new((n >> 3) as i32, id);
                id += 1;
                res
            }))
        }
    }
    impl PartialOrd for Data {
        fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
            self.num.partial_cmp(&other.num)
        }
    }
}
