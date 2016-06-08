extern crate rayon;

fn partition<T:PartialOrd+Send>(v: &mut [T]) -> usize {
    let pivot = v.len() - 1;
    let mut i = 0;
    for j in 0..pivot {
        if v[j] <= v[pivot] {
            v.swap(i, j);
            i += 1;
        }
    }
    v.swap(i, pivot);
    i
}

pub fn qsort(vec: &mut [i32]) {
    if vec.len() <= 1 {
        return;
    }
    let mid = partition(vec);
    let (less, greater) = vec.split_at_mut(mid);
    qsort(less);
    qsort(greater);
}

pub fn parallel_qsort(vec: &mut [i32]) {
    if vec.len() <= 1 {
        return;
    }
    let mid = partition(vec);
    let (less, greater) = vec.split_at_mut(mid);
    rayon::join(
        || parallel_qsort(less),
        || parallel_qsort(greater)
    );
}
