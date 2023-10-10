use std::{
    mem,
    ptr::copy_nonoverlapping as copy_nonol, cmp::Ordering,
};

/// 清晰的实现, 但是只是清晰
#[allow(dead_code)]
pub(crate) fn insert_sort_by_lt_example<T, F>(array: &mut [T], mut lt: F)
where F: FnMut(&T, &T) -> bool,
      T: Copy
{
    #[warn(dead_code)]
    for i in 1..array.len() {
        if cmp!(lt => &array[i-1], (<=), &array[i]) {
            // 已经排好
            continue
        }
        let target = array[i];
        let mut j = i - 1;
        array[i] = array[j];
        while j > 0 && cmp!(lt => &array[j-1], (>), &target) {
            array[j] = array[j-1];
            j -= 1
        }
        array[j] = target
    }
}

fn insert_sort_by_lt<T, F>(array: &mut [T], mut lt: F)
where F: FnMut(&T, &T) -> bool
{
    for i in 1..array.len() { unsafe {
        let target_ref = array.get_unchecked(i);
        if cmp!(lt => array.get_unchecked(i-1), (<=), target_ref) {
            continue
        }
        let mut target = mem::MaybeUninit::uninit();
        copy_nonol(target_ref, target.as_mut_ptr(), 1);
        let dst = array.get_unchecked_mut(i) as *mut T;
        let mut insert_ptr = dst.sub(1);
        copy_nonol(
            insert_ptr,
            dst,
            1,
        );
        while insert_ptr > array.as_mut_ptr()
            && cmp!(lt =>
                insert_ptr.sub(1).as_ref().unwrap_unchecked(),
                (>),
                target.as_ptr().as_ref().unwrap_unchecked(),
            )
        {
            copy_nonol(insert_ptr.sub(1), insert_ptr, 1);
            insert_ptr = insert_ptr.sub(1)
        }
        copy_nonol(target.as_ptr(), insert_ptr, 1)
    } }
}

pub fn insert_sort<T: Ord>(array: &mut [T]) {
    insert_sort_by_lt(array, T::lt)
}

pub fn insert_sort_by<T, F>(array: &mut [T], mut cmp: F)
where F: FnMut(&T, &T) -> Ordering
{
    insert_sort_by_lt(array, |a, b| cmp(a, b).is_lt())
}

pub fn insert_sort_by_key<T, K, F>(array: &mut [T], mut f: F)
where F: FnMut(&T) -> K,
      K: Ord,
{
    insert_sort_by_lt(array, |a, b| f(a).lt(&f(b)))
}
