//! Operations that broadly apply to list types.


// pub trait Neg { type Output; fn neg(self) -> Self::Output; }
// pub trait Not { type Output; fn not(self) -> Self::Output; }





// Element-wise
// pub trait ElementAdd   <Rhs = Self> { type Output; fn element_add   (self, rhs: Rhs) -> Self::Output; }
// pub trait ElementSub   <Rhs = Self> { type Output; fn element_sub   (self, rhs: Rhs) -> Self::Output; }
// pub trait ElementMul   <Rhs = Self> { type Output; fn element_mul   (self, rhs: Rhs) -> Self::Output; }
// pub trait ElementDiv   <Rhs = Self> { type Output; fn element_div   (self, rhs: Rhs) -> Self::Output; }
// pub trait ElementRem   <Rhs = Self> { type Output; fn element_rem   (self, rhs: Rhs) -> Self::Output; }
// pub trait ElementBitAnd<Rhs = Self> { type Output; fn element_bitand(self, rhs: Rhs) -> Self::Output; }
// pub trait ElementBitOr <Rhs = Self> { type Output; fn element_bitor (self, rhs: Rhs) -> Self::Output; }
// pub trait ElementBitXor<Rhs = Self> { type Output; fn element_bitxor(self, rhs: Rhs) -> Self::Output; }
// pub trait ElementShl   <Rhs = Self> { type Output; fn element_shl   (self, rhs: Rhs) -> Self::Output; }
// pub trait ElementShr   <Rhs = Self> { type Output; fn element_shr   (self, rhs: Rhs) -> Self::Output; }

// Scalar
// pub trait ScalarAdd   <Rhs = Self> { type Output; fn scalar_add   (self, rhs: Rhs) -> Self::Output; }
// pub trait ScalarSub   <Rhs = Self> { type Output; fn scalar_sub   (self, rhs: Rhs) -> Self::Output; }
// pub trait ScalarMul   <Rhs = Self> { type Output; fn scalar_mul   (self, rhs: Rhs) -> Self::Output; }
// pub trait ScalarDiv   <Rhs = Self> { type Output; fn scalar_div   (self, rhs: Rhs) -> Self::Output; }
// pub trait ScalarRem   <Rhs = Self> { type Output; fn scalar_rem   (self, rhs: Rhs) -> Self::Output; }
// pub trait ScalarBitAnd<Rhs = Self> { type Output; fn scalar_bitand(self, rhs: Rhs) -> Self::Output; }
// pub trait ScalarBitOr <Rhs = Self> { type Output; fn scalar_bitor (self, rhs: Rhs) -> Self::Output; }
// pub trait ScalarBitXor<Rhs = Self> { type Output; fn scalar_bitxor(self, rhs: Rhs) -> Self::Output; }
// pub trait ScalarShl   <Rhs = Self> { type Output; fn scalar_shl   (self, rhs: Rhs) -> Self::Output; }
// pub trait ScalarShr   <Rhs = Self> { type Output; fn scalar_shr   (self, rhs: Rhs) -> Self::Output; }


// Vector-specific
// pub trait CrossProduct<Rhs = Self> { type Output; fn cross_product(self, rhs: Rhs) -> Self::Output; }
// pub trait InnerProduct<Rhs = Self> { type Output; fn inner_product(self, rhs: Rhs) -> Self::Output; }
// pub trait OuterProduct<Rhs = Self> { type Output; fn outer_product(self, rhs: Rhs) -> Self::Output; }

// Matrix-specific
// pub trait Transpose   { type Output; fn transpose  (self) -> Self::Output; }
// pub trait Adjoint     { type Output; fn adjoint    (self) -> Self::Output; }
// pub trait Adjugate    { type Output; fn adjugate   (self) -> Self::Output; }
// pub trait Minors      { type Output; fn minors     (self) -> Self::Output; }
// pub trait Cofactors   { type Output; fn cofactors  (self) -> Self::Output; }
// pub trait Determinant { type Output; fn determinant(self) -> Self::Output; }
// pub trait Inverse     { type Output; fn inverse    (self) -> Self::Output; }
// pub trait MatrixMul   <Rhs = Self> { type Output; fn matrix_mul   (self, rhs: Rhs) -> Self::Output; }
// TODO: Kronecker

