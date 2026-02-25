use crate::{ConstIndex, ConstIndexFromEnd, Count0, Count1, Count2, Count3, Vec3, Vector, vec1};


#[test]
fn test() {
    let a: Vec3<f32> = Vector(Vector(vec1(1.0), 2.0), 3.0);
    dbg!(ConstIndexFromEnd::<_, Count0>::index_from_end(&a));
    dbg!(ConstIndexFromEnd::<_, Count1>::index_from_end(&a));
    dbg!(ConstIndexFromEnd::<_, Count2>::index_from_end(&a));
    // dbg!(ConstIndexFromEnd::<_, Count3>::index_from_end(&a)); // doesn't compile
    
    dbg!(ConstIndex::<_, Count0>::index(&a));
    dbg!(ConstIndex::<_, Count1>::index(&a));
    dbg!(ConstIndex::<_, Count2>::index(&a));
    // dbg!(ConstIndex::<_, Count3>::index(&a)); // doesn't compile
}

