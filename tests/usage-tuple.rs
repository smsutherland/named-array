use named_array::named_array;

#[derive(named_array)]
struct Arr(u32, u32, u32);

#[test]
fn use_arr() {
    let arr = Arr(1, 2, 3);
    assert_eq!(arr.0, arr[0]);
    assert_eq!(arr.1, arr[1]);
    assert_eq!(arr.2, arr[2]);
}

#[test]
#[should_panic(expected = "index out of bounds: the len is 3 but the index is 3")]
fn fail_arr() {
    let arr = Arr(1, 2, 3);
    let _ = arr[3];
}
