mod hash_map;
mod rc;
mod stack;

fn main() {
    let stack = stack::SimpleStack::new();
    stack.push(1);
    stack.push(2);
    stack.push(3);
    assert_eq!(stack.pop(), Some(3));
    assert_eq!(stack.pop(), Some(2));
    assert_eq!(stack.pop(), Some(1));
    assert_eq!(stack.pop(), None);

    let map = hash_map! {
        "one" => 1,
        "two" => 2,
        "three" => 3,
    };
    assert_eq!(map["one"], 1);
    assert_eq!(map["two"], 2);
    assert_eq!(map["three"], 3);

    let data = vec![1, 2, 3];
    let my_rc = rc::MyRc::new(data.clone());
    let my_rc2 = my_rc.clone();

    println!("rc: {:?}  rc2: {:?}", *my_rc, *my_rc2);
}
