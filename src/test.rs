use crate::{number::Num2, recursive_list::RecursiveList, static_list::StaticZip, vector::helper::{vec0, vec3}};

extern crate std;
use std::dbg;


#[test]
fn test() {
    
    
    let mut a = vec0().push(3.5).push(2.0).push(-9.0);
    let b = vec3(1.0, 2.0, 5.0);
    dbg!((a + b).x());
    
    // let c = vec0::<f32>();
    
    a += b;
    
    a *= 1.5;
    
    dbg!(a.x(), a.y(), a.z());
    
    
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

