use crate::*;


#[test]
fn test() {
    let a: Vec3<f32> = vec3(1.0, 2.0, 3.0);
    assert_eq!(StaticIndexFromEnd::<_, Count0>::static_index_from_end_owned(a), 3.0);
    assert_eq!(StaticIndexFromEnd::<_, Count1>::static_index_from_end_owned(a), 2.0);
    assert_eq!(StaticIndexFromEnd::<_, Count2>::static_index_from_end_owned(a), 1.0);
    
    assert_eq!(a.get::<Count0>(), 1.0);
    assert_eq!(a.get::<Count1>(), 2.0);
    assert_eq!(a.get::<Count2>(), 3.0);
}

