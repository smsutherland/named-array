use named_array::named_array;

#[derive(named_array)]
struct Arr {
    a: u32,
    b: u32,
    c: u32,
}

#[test]
fn use_arr() {
    let arr = Arr { a: 1, b: 2, c: 3 };
    assert_eq!(arr.a, arr[0]);
    assert_eq!(arr.b, arr[1]);
    assert_eq!(arr.c, arr[2]);
}

#[test]
#[should_panic]
fn fail_arr() {
    let arr = Arr { a: 1, b: 2, c: 3 };
    let _ = arr[3];
}
