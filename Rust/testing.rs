
fn main() {
    let mut array = Vec::new();
    array.push(1);
    array.push(2);
    
    assert_eq!(array.len(), 2);
    assert_eq!(array[0], 1);
    
    assert_eq!(array.pop(), Some(2));
    assert_eq!(array.len(), 1);
    
    array[0] = 7;
    assert_eq!(array[0], 7);
    
    array.extend([1, 2, 3].iter().copied());
    
    for x in &array {
        println!("{}", x);
    }
    assert_eq!(vec, [7, 1, 2, 3]);
}
