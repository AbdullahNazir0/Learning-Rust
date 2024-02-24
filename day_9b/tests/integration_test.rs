use day_9b;
#[test]
fn positive_ok() {
    let res = day_9b::calculate_area(8, 4);
    assert_eq!(res, 32, "Result should be 32, but is {}", res);
}