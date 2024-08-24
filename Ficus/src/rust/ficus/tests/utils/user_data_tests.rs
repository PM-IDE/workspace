use std::rc::Rc;

use ficus::utils::user_data::{
    keys::DefaultKey,
    user_data::{UserData, UserDataImpl},
};

#[test]
fn test_user_data() {
    let key = DefaultKey::<usize>::new("asdasdasda".to_string());
    let mut user_data = UserDataImpl::new();
    let b = 123;
    user_data.put_concrete(&key, b);

    assert_eq!(*user_data.concrete(&key).unwrap(), 123);

    *user_data.concrete_mut(&key).unwrap() = 321;
    assert_eq!(*user_data.concrete(&key).unwrap(), 321);
}

#[test]
fn test_user_data_two_keys() {
    let first_key = DefaultKey::<Rc<Box<usize>>>::new("1".to_string());
    let second_key = DefaultKey::<Rc<Box<usize>>>::new("2".to_string());

    let first_value = Rc::new(Box::new(123));
    let second_value = Rc::new(Box::new(321));
    let box1 = Rc::clone(&first_value);
    let box2 = Rc::clone(&second_value);

    let mut user_data = UserDataImpl::new();

    user_data.put_any(&first_key, box1);
    user_data.put_any(&second_key, box2);

    assert_eq!(user_data.concrete(&first_key).unwrap(), &first_value);
    assert_eq!(user_data.concrete(&second_key).unwrap(), &second_value);

    assert!(Rc::ptr_eq(user_data.get(&first_key).unwrap(), &first_value));
    assert!(Rc::ptr_eq(user_data.get(&second_key).unwrap(), &second_value));
}

#[test]
fn test_remove_from_user_data() {
    let key = DefaultKey::<usize>::new("1".to_string());

    let value = 123usize;
    let mut user_data = UserDataImpl::new();

    user_data.put_any(&key, value);

    assert_eq!(*user_data.concrete(&key).unwrap(), 123);

    user_data.remove(&key);

    assert_eq!(user_data.concrete(&key), None);
}
