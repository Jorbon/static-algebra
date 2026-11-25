use core::ops::*;

use crate::{recursive_list::RecursiveList, static_list::{Mappable, StaticMap, StaticZip}, vector::{Vec0, VecPush, Vector}};


// Unary

impl<       T> Neg for Vec0   <T    > where                       T: Neg, Self: StaticMap<T, T::Output> { type Output = <Self as Mappable<T::Output>>::Mapped; #[inline] fn neg(self) -> Self::Output { self.map(&Neg::neg) } }
impl<       T> Not for Vec0   <T    > where                       T: Not, Self: StaticMap<T, T::Output> { type Output = <Self as Mappable<T::Output>>::Mapped; #[inline] fn not(self) -> Self::Output { self.map(&Not::not) } }

impl<Inner, T> Neg for VecPush<Inner> where Inner: Vector<T = T>, T: Neg, Self: StaticMap<T, T::Output> { type Output = <Self as Mappable<T::Output>>::Mapped; #[inline] fn neg(self) -> Self::Output { self.map(&Neg::neg) } }
impl<Inner, T> Not for VecPush<Inner> where Inner: Vector<T = T>, T: Not, Self: StaticMap<T, T::Output> { type Output = <Self as Mappable<T::Output>>::Mapped; #[inline] fn not(self) -> Self::Output { self.map(&Not::not) } }


// Element-wise

impl<       Rhs, T> Add   <Rhs> for Vec0   <T    > where                       Rhs: RecursiveList, T: Add   <Rhs::T>, Self: StaticZip<Rhs, T, Rhs::T, T::Output> { type Output = <Self as Mappable<T::Output>>::Mapped; #[inline] fn add   (self, rhs: Rhs) -> Self::Output { self.zip(rhs, &Add   ::add   ) } }
impl<       Rhs, T> Sub   <Rhs> for Vec0   <T    > where                       Rhs: RecursiveList, T: Sub   <Rhs::T>, Self: StaticZip<Rhs, T, Rhs::T, T::Output> { type Output = <Self as Mappable<T::Output>>::Mapped; #[inline] fn sub   (self, rhs: Rhs) -> Self::Output { self.zip(rhs, &Sub   ::sub   ) } }
impl<       Rhs, T> BitAnd<Rhs> for Vec0   <T    > where                       Rhs: RecursiveList, T: BitAnd<Rhs::T>, Self: StaticZip<Rhs, T, Rhs::T, T::Output> { type Output = <Self as Mappable<T::Output>>::Mapped; #[inline] fn bitand(self, rhs: Rhs) -> Self::Output { self.zip(rhs, &BitAnd::bitand) } }
impl<       Rhs, T> BitOr <Rhs> for Vec0   <T    > where                       Rhs: RecursiveList, T: BitOr <Rhs::T>, Self: StaticZip<Rhs, T, Rhs::T, T::Output> { type Output = <Self as Mappable<T::Output>>::Mapped; #[inline] fn bitor (self, rhs: Rhs) -> Self::Output { self.zip(rhs, &BitOr ::bitor ) } }
impl<       Rhs, T> BitXor<Rhs> for Vec0   <T    > where                       Rhs: RecursiveList, T: BitXor<Rhs::T>, Self: StaticZip<Rhs, T, Rhs::T, T::Output> { type Output = <Self as Mappable<T::Output>>::Mapped; #[inline] fn bitxor(self, rhs: Rhs) -> Self::Output { self.zip(rhs, &BitXor::bitxor) } }
impl<       Rhs, T> Shl   <Rhs> for Vec0   <T    > where                       Rhs: RecursiveList, T: Shl   <Rhs::T>, Self: StaticZip<Rhs, T, Rhs::T, T::Output> { type Output = <Self as Mappable<T::Output>>::Mapped; #[inline] fn shl   (self, rhs: Rhs) -> Self::Output { self.zip(rhs, &Shl   ::shl   ) } }
impl<       Rhs, T> Shr   <Rhs> for Vec0   <T    > where                       Rhs: RecursiveList, T: Shr   <Rhs::T>, Self: StaticZip<Rhs, T, Rhs::T, T::Output> { type Output = <Self as Mappable<T::Output>>::Mapped; #[inline] fn shr   (self, rhs: Rhs) -> Self::Output { self.zip(rhs, &Shr   ::shr   ) } }

impl<Inner, Rhs, T> Add   <Rhs> for VecPush<Inner> where Inner: Vector<T = T>, Rhs: RecursiveList, T: Add   <Rhs::T>, Self: StaticZip<Rhs, T, Rhs::T, T::Output> { type Output = <Self as Mappable<T::Output>>::Mapped; #[inline] fn add   (self, rhs: Rhs) -> Self::Output { self.zip(rhs, &Add   ::add   ) } }
impl<Inner, Rhs, T> Sub   <Rhs> for VecPush<Inner> where Inner: Vector<T = T>, Rhs: RecursiveList, T: Sub   <Rhs::T>, Self: StaticZip<Rhs, T, Rhs::T, T::Output> { type Output = <Self as Mappable<T::Output>>::Mapped; #[inline] fn sub   (self, rhs: Rhs) -> Self::Output { self.zip(rhs, &Sub   ::sub   ) } }
impl<Inner, Rhs, T> BitAnd<Rhs> for VecPush<Inner> where Inner: Vector<T = T>, Rhs: RecursiveList, T: BitAnd<Rhs::T>, Self: StaticZip<Rhs, T, Rhs::T, T::Output> { type Output = <Self as Mappable<T::Output>>::Mapped; #[inline] fn bitand(self, rhs: Rhs) -> Self::Output { self.zip(rhs, &BitAnd::bitand) } }
impl<Inner, Rhs, T> BitOr <Rhs> for VecPush<Inner> where Inner: Vector<T = T>, Rhs: RecursiveList, T: BitOr <Rhs::T>, Self: StaticZip<Rhs, T, Rhs::T, T::Output> { type Output = <Self as Mappable<T::Output>>::Mapped; #[inline] fn bitor (self, rhs: Rhs) -> Self::Output { self.zip(rhs, &BitOr ::bitor ) } }
impl<Inner, Rhs, T> BitXor<Rhs> for VecPush<Inner> where Inner: Vector<T = T>, Rhs: RecursiveList, T: BitXor<Rhs::T>, Self: StaticZip<Rhs, T, Rhs::T, T::Output> { type Output = <Self as Mappable<T::Output>>::Mapped; #[inline] fn bitxor(self, rhs: Rhs) -> Self::Output { self.zip(rhs, &BitXor::bitxor) } }
impl<Inner, Rhs, T> Shl   <Rhs> for VecPush<Inner> where Inner: Vector<T = T>, Rhs: RecursiveList, T: Shl   <Rhs::T>, Self: StaticZip<Rhs, T, Rhs::T, T::Output> { type Output = <Self as Mappable<T::Output>>::Mapped; #[inline] fn shl   (self, rhs: Rhs) -> Self::Output { self.zip(rhs, &Shl   ::shl   ) } }
impl<Inner, Rhs, T> Shr   <Rhs> for VecPush<Inner> where Inner: Vector<T = T>, Rhs: RecursiveList, T: Shr   <Rhs::T>, Self: StaticZip<Rhs, T, Rhs::T, T::Output> { type Output = <Self as Mappable<T::Output>>::Mapped; #[inline] fn shr   (self, rhs: Rhs) -> Self::Output { self.zip(rhs, &Shr   ::shr   ) } }


// Scalar

impl<       Rhs, T> Mul<Rhs> for Vec0   <T    > where                       T: Mul<Rhs>, Self: StaticMap<T, T::Output>, Rhs: Copy { type Output = <Self as Mappable<T::Output>>::Mapped; #[inline] fn mul(self, rhs: Rhs) -> Self::Output { self.map(&|t| T::mul(t, rhs)) } }
impl<       Rhs, T> Div<Rhs> for Vec0   <T    > where                       T: Div<Rhs>, Self: StaticMap<T, T::Output>, Rhs: Copy { type Output = <Self as Mappable<T::Output>>::Mapped; #[inline] fn div(self, rhs: Rhs) -> Self::Output { self.map(&|t| T::div(t, rhs)) } }
impl<       Rhs, T> Rem<Rhs> for Vec0   <T    > where                       T: Rem<Rhs>, Self: StaticMap<T, T::Output>, Rhs: Copy { type Output = <Self as Mappable<T::Output>>::Mapped; #[inline] fn rem(self, rhs: Rhs) -> Self::Output { self.map(&|t| T::rem(t, rhs)) } }

impl<Inner, Rhs, T> Mul<Rhs> for VecPush<Inner> where Inner: Vector<T = T>, T: Mul<Rhs>, Self: StaticMap<T, T::Output>, Rhs: Copy { type Output = <Self as Mappable<T::Output>>::Mapped; #[inline] fn mul(self, rhs: Rhs) -> Self::Output { self.map(&|t| T::mul(t, rhs)) } }
impl<Inner, Rhs, T> Div<Rhs> for VecPush<Inner> where Inner: Vector<T = T>, T: Div<Rhs>, Self: StaticMap<T, T::Output>, Rhs: Copy { type Output = <Self as Mappable<T::Output>>::Mapped; #[inline] fn div(self, rhs: Rhs) -> Self::Output { self.map(&|t| T::div(t, rhs)) } }
impl<Inner, Rhs, T> Rem<Rhs> for VecPush<Inner> where Inner: Vector<T = T>, T: Rem<Rhs>, Self: StaticMap<T, T::Output>, Rhs: Copy { type Output = <Self as Mappable<T::Output>>::Mapped; #[inline] fn rem(self, rhs: Rhs) -> Self::Output { self.map(&|t| T::rem(t, rhs)) } }


// Assign element-wise

impl<       Rhs, T> AddAssign   <Rhs> for Vec0   <T    > where                       Rhs: RecursiveList, T: AddAssign   <Rhs::T>, for <'a> &'a mut Self: StaticZip<Rhs, &'a mut T, Rhs::T, ()> + Mappable<()> { #[inline] fn add_assign   (&mut self, rhs: Rhs) { self.zip(rhs, &AddAssign   ::add_assign   ); } }
impl<       Rhs, T> SubAssign   <Rhs> for Vec0   <T    > where                       Rhs: RecursiveList, T: SubAssign   <Rhs::T>, for <'a> &'a mut Self: StaticZip<Rhs, &'a mut T, Rhs::T, ()> + Mappable<()> { #[inline] fn sub_assign   (&mut self, rhs: Rhs) { self.zip(rhs, &SubAssign   ::sub_assign   ); } }
impl<       Rhs, T> BitAndAssign<Rhs> for Vec0   <T    > where                       Rhs: RecursiveList, T: BitAndAssign<Rhs::T>, for <'a> &'a mut Self: StaticZip<Rhs, &'a mut T, Rhs::T, ()> + Mappable<()> { #[inline] fn bitand_assign(&mut self, rhs: Rhs) { self.zip(rhs, &BitAndAssign::bitand_assign); } }
impl<       Rhs, T> BitOrAssign <Rhs> for Vec0   <T    > where                       Rhs: RecursiveList, T: BitOrAssign <Rhs::T>, for <'a> &'a mut Self: StaticZip<Rhs, &'a mut T, Rhs::T, ()> + Mappable<()> { #[inline] fn bitor_assign (&mut self, rhs: Rhs) { self.zip(rhs, &BitOrAssign ::bitor_assign ); } }
impl<       Rhs, T> BitXorAssign<Rhs> for Vec0   <T    > where                       Rhs: RecursiveList, T: BitXorAssign<Rhs::T>, for <'a> &'a mut Self: StaticZip<Rhs, &'a mut T, Rhs::T, ()> + Mappable<()> { #[inline] fn bitxor_assign(&mut self, rhs: Rhs) { self.zip(rhs, &BitXorAssign::bitxor_assign); } }
impl<       Rhs, T> ShlAssign   <Rhs> for Vec0   <T    > where                       Rhs: RecursiveList, T: ShlAssign   <Rhs::T>, for <'a> &'a mut Self: StaticZip<Rhs, &'a mut T, Rhs::T, ()> + Mappable<()> { #[inline] fn shl_assign   (&mut self, rhs: Rhs) { self.zip(rhs, &ShlAssign   ::shl_assign   ); } }
impl<       Rhs, T> ShrAssign   <Rhs> for Vec0   <T    > where                       Rhs: RecursiveList, T: ShrAssign   <Rhs::T>, for <'a> &'a mut Self: StaticZip<Rhs, &'a mut T, Rhs::T, ()> + Mappable<()> { #[inline] fn shr_assign   (&mut self, rhs: Rhs) { self.zip(rhs, &ShrAssign   ::shr_assign   ); } }

impl<Inner, Rhs, T> AddAssign   <Rhs> for VecPush<Inner> where Inner: Vector<T = T>, Rhs: RecursiveList, T: AddAssign   <Rhs::T>, for <'a> &'a mut Self: StaticZip<Rhs, &'a mut T, Rhs::T, ()> + Mappable<()> { #[inline] fn add_assign   (&mut self, rhs: Rhs) { self.zip(rhs, &AddAssign   ::add_assign   ); } }
impl<Inner, Rhs, T> SubAssign   <Rhs> for VecPush<Inner> where Inner: Vector<T = T>, Rhs: RecursiveList, T: SubAssign   <Rhs::T>, for <'a> &'a mut Self: StaticZip<Rhs, &'a mut T, Rhs::T, ()> + Mappable<()> { #[inline] fn sub_assign   (&mut self, rhs: Rhs) { self.zip(rhs, &SubAssign   ::sub_assign   ); } }
impl<Inner, Rhs, T> BitAndAssign<Rhs> for VecPush<Inner> where Inner: Vector<T = T>, Rhs: RecursiveList, T: BitAndAssign<Rhs::T>, for <'a> &'a mut Self: StaticZip<Rhs, &'a mut T, Rhs::T, ()> + Mappable<()> { #[inline] fn bitand_assign(&mut self, rhs: Rhs) { self.zip(rhs, &BitAndAssign::bitand_assign); } }
impl<Inner, Rhs, T> BitOrAssign <Rhs> for VecPush<Inner> where Inner: Vector<T = T>, Rhs: RecursiveList, T: BitOrAssign <Rhs::T>, for <'a> &'a mut Self: StaticZip<Rhs, &'a mut T, Rhs::T, ()> + Mappable<()> { #[inline] fn bitor_assign (&mut self, rhs: Rhs) { self.zip(rhs, &BitOrAssign ::bitor_assign ); } }
impl<Inner, Rhs, T> BitXorAssign<Rhs> for VecPush<Inner> where Inner: Vector<T = T>, Rhs: RecursiveList, T: BitXorAssign<Rhs::T>, for <'a> &'a mut Self: StaticZip<Rhs, &'a mut T, Rhs::T, ()> + Mappable<()> { #[inline] fn bitxor_assign(&mut self, rhs: Rhs) { self.zip(rhs, &BitXorAssign::bitxor_assign); } }
impl<Inner, Rhs, T> ShlAssign   <Rhs> for VecPush<Inner> where Inner: Vector<T = T>, Rhs: RecursiveList, T: ShlAssign   <Rhs::T>, for <'a> &'a mut Self: StaticZip<Rhs, &'a mut T, Rhs::T, ()> + Mappable<()> { #[inline] fn shl_assign   (&mut self, rhs: Rhs) { self.zip(rhs, &ShlAssign   ::shl_assign   ); } }
impl<Inner, Rhs, T> ShrAssign   <Rhs> for VecPush<Inner> where Inner: Vector<T = T>, Rhs: RecursiveList, T: ShrAssign   <Rhs::T>, for <'a> &'a mut Self: StaticZip<Rhs, &'a mut T, Rhs::T, ()> + Mappable<()> { #[inline] fn shr_assign   (&mut self, rhs: Rhs) { self.zip(rhs, &ShrAssign   ::shr_assign   ); } }


// Assign scalar

impl<       Rhs, T> MulAssign<Rhs> for Vec0   <T    > where                       T: MulAssign<Rhs>, for <'a> &'a mut Self: StaticMap<&'a mut T, ()> + Mappable<()>, Rhs: Copy { #[inline] fn mul_assign(&mut self, rhs: Rhs) { self.map(&|t| MulAssign::mul_assign(t, rhs)); } }
impl<       Rhs, T> DivAssign<Rhs> for Vec0   <T    > where                       T: DivAssign<Rhs>, for <'a> &'a mut Self: StaticMap<&'a mut T, ()> + Mappable<()>, Rhs: Copy { #[inline] fn div_assign(&mut self, rhs: Rhs) { self.map(&|t| DivAssign::div_assign(t, rhs)); } }
impl<       Rhs, T> RemAssign<Rhs> for Vec0   <T    > where                       T: RemAssign<Rhs>, for <'a> &'a mut Self: StaticMap<&'a mut T, ()> + Mappable<()>, Rhs: Copy { #[inline] fn rem_assign(&mut self, rhs: Rhs) { self.map(&|t| RemAssign::rem_assign(t, rhs)); } }

impl<Inner, Rhs, T> MulAssign<Rhs> for VecPush<Inner> where Inner: Vector<T = T>, T: MulAssign<Rhs>, for <'a> &'a mut Self: StaticMap<&'a mut T, ()> + Mappable<()>, Rhs: Copy { #[inline] fn mul_assign(&mut self, rhs: Rhs) { self.map(&|t| MulAssign::mul_assign(t, rhs)); } }
impl<Inner, Rhs, T> DivAssign<Rhs> for VecPush<Inner> where Inner: Vector<T = T>, T: DivAssign<Rhs>, for <'a> &'a mut Self: StaticMap<&'a mut T, ()> + Mappable<()>, Rhs: Copy { #[inline] fn div_assign(&mut self, rhs: Rhs) { self.map(&|t| DivAssign::div_assign(t, rhs)); } }
impl<Inner, Rhs, T> RemAssign<Rhs> for VecPush<Inner> where Inner: Vector<T = T>, T: RemAssign<Rhs>, for <'a> &'a mut Self: StaticMap<&'a mut T, ()> + Mappable<()>, Rhs: Copy { #[inline] fn rem_assign(&mut self, rhs: Rhs) { self.map(&|t| RemAssign::rem_assign(t, rhs)); } }

