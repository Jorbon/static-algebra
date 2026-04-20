use crate::{number::{Num0, Num1, Num2, Num3}, recursive_list::RecursiveList, static_list::StaticGet, vector::Vec0};

extern crate std;
use std::dbg;


#[test]
fn test() {
    
    
    let a = Vec0.push(3.5).push(2.0).push(-9.0);
    dbg!(StaticGet::<_, Num2>::static_get(a));
    
    
    // let a: Vec3<f32> = vec3(1.0, 2.0, 3.0);
    
    // let _ = a.iter();
    
    // assert_eq!(*StaticIndexFromEnd::<_, Num0>::static_index_from_end(&a), 3.0);
    // assert_eq!(*StaticIndexFromEnd::<_, Num1>::static_index_from_end(&a), 2.0);
    // assert_eq!(*StaticIndexFromEnd::<_, Num2>::static_index_from_end(&a), 1.0);
    
    // assert_eq!(a.get_owned::<Num0>(), 1.0);
    // assert_eq!(a.get_owned::<Num1>(), 2.0);
    // assert_eq!(a.get_owned::<Num2>(), 3.0);
    
    // let b = vec3(1.0, 3.0, 2.0f32);
    // dbg!(b.dot(vec3(3.5, 4.2, 1000.0f32)));
    
}

