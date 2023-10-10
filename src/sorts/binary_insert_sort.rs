use std::{ptr, mem::MaybeUninit, cmp::Ordering};

pub fn binary_search_insert_point_by_lt<T, F>(
    array: &[T],
    target: &T,
    mut lt: F,
) -> usize
where F: FnMut(&T, &T) -> bool,
{
    let [mut i, mut j] = [0, array.len()];
    while i < j {
        let mid = i + ((j - i) >> 1);
        let mid_data = unsafe { array.get_unchecked(mid) };
        if lt(target, mid_data) {
            j = mid
        } else {
            i = mid + 1
        }
    }
    debug_assert_eq!(i, j);
    i
}

/// 将array最后一个元素插入到首部
pub fn insert_last_to_first<T>(array: &mut [T]) {
    if array.len() < 2 { return }
    let mut data = MaybeUninit::uninit();
    unsafe {
        ptr::copy_nonoverlapping(
            array.last().unwrap_unchecked(),
            data.as_mut_ptr(),
            1,
        );
        ptr::copy(
            array.as_ptr(),
            array.as_mut_ptr().add(1),
            array.len() - 1,
        );
        ptr::copy_nonoverlapping(
            data.as_ptr(),
            array.as_mut_ptr(),
            1,
        );
    }
}

pub fn binary_insert_sort_by_lt<T, F>(array: &mut [T], mut lt: F)
where F: FnMut(&T, &T) -> bool,
{
    for i in 1..array.len() { unsafe {
        let x = array.get_unchecked(i);
        if cmp!(lt => array.get_unchecked(i-1), (<=), x) {
            continue
        }
        let insert_point = binary_search_insert_point_by_lt(
            array.get_unchecked(..i-1),
            array.get_unchecked(i),
            &mut lt
        );
        insert_last_to_first(
            array.get_unchecked_mut(insert_point..=i)
        )
    } }
}

pub fn binary_insert_sort<T: Ord>(array: &mut [T]) {
    binary_insert_sort_by_lt(array, T::lt)
}

pub fn binary_insert_sort_by<T, F>(array: &mut [T], mut cmp: F)
where F: FnMut(&T, &T) -> Ordering
{
    binary_insert_sort_by_lt(array, |a, b| cmp(a, b).is_lt())
}

pub fn binary_insert_sort_by_key<T, K, F>(array: &mut [T], mut f: F)
where F: FnMut(&T) -> K,
      K: Ord,
{
    binary_insert_sort_by_lt(array, |a, b| f(a).lt(&f(b)))
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::data::Data;
    use rand::prelude::SliceRandom;

    #[test]
    fn binary_search_insert_point_test() {
        let arr = [0, 1, 1, 3, 5, 6];
        assert_eq!(binary_search_insert_point_by_lt(&arr, &-1, i32::lt), 0);
        assert_eq!(binary_search_insert_point_by_lt(&arr, &0, i32::lt), 1);
        assert_eq!(binary_search_insert_point_by_lt(&arr, &1, i32::lt), 3);
        assert_eq!(binary_search_insert_point_by_lt(&arr, &3, i32::lt), 4);
        assert_eq!(binary_search_insert_point_by_lt(&arr, &4, i32::lt), 4);
        assert_eq!(binary_search_insert_point_by_lt(&arr, &8, i32::lt), 6);
    }

    #[test]
    fn insert_last_to_first_test() {
        let mut arr = [0, 1, 2, 3, 4, 5];
        insert_last_to_first(&mut arr);
        assert_eq!(arr, [5, 0, 1, 2, 3, 4]);

        insert_last_to_first::<i32>(&mut []);
        insert_last_to_first(&mut [0]);
    }

    #[test]
    fn binary_insert_sort_test() {
        let mut arr = Data::new_vec(1000);
        arr.shuffle(&mut rand::thread_rng());
        let mut arr1 = arr.clone();
        arr.sort();
        binary_insert_sort(&mut arr1);
        assert_eq!(arr, arr1);
    }
}
