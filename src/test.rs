use crate::*;


#[test]
fn test() {
    
    let a: Vec3<f32> = vec3(1.0, 2.0, 3.0);
    
    let _ = a.iter();
    
    assert_eq!(*StaticIndexFromEnd::<_, Number0>::static_index_from_end(&a), 3.0);
    assert_eq!(*StaticIndexFromEnd::<_, Number1>::static_index_from_end(&a), 2.0);
    assert_eq!(*StaticIndexFromEnd::<_, Number2>::static_index_from_end(&a), 1.0);
    
    assert_eq!(a.get_owned::<Number0>(), 1.0);
    assert_eq!(a.get_owned::<Number1>(), 2.0);
    assert_eq!(a.get_owned::<Number2>(), 3.0);
}

