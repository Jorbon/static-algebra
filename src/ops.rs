
// Element-wise
pub trait ElementNeg                { type Output; fn element_neg   (self          ) -> Self::Output; }
pub trait ElementNot                { type Output; fn element_not   (self          ) -> Self::Output; }
pub trait ElementAdd   <Rhs = Self> { type Output; fn element_add   (self, rhs: Rhs) -> Self::Output; }
pub trait ElementSub   <Rhs = Self> { type Output; fn element_sub   (self, rhs: Rhs) -> Self::Output; }
pub trait ElementMul   <Rhs = Self> { type Output; fn element_mul   (self, rhs: Rhs) -> Self::Output; }
pub trait ElementDiv   <Rhs = Self> { type Output; fn element_div   (self, rhs: Rhs) -> Self::Output; }
pub trait ElementRem   <Rhs = Self> { type Output; fn element_rem   (self, rhs: Rhs) -> Self::Output; }
pub trait ElementBitAnd<Rhs = Self> { type Output; fn element_bitand(self, rhs: Rhs) -> Self::Output; }
pub trait ElementBitOr <Rhs = Self> { type Output; fn element_bitor (self, rhs: Rhs) -> Self::Output; }
pub trait ElementBitXor<Rhs = Self> { type Output; fn element_bitxor(self, rhs: Rhs) -> Self::Output; }
pub trait ElementShl   <Rhs = Self> { type Output; fn element_shl   (self, rhs: Rhs) -> Self::Output; }
pub trait ElementShr   <Rhs = Self> { type Output; fn element_shr   (self, rhs: Rhs) -> Self::Output; }

// Scalar
pub trait ScalarMul   <Rhs = Self> { type Output; fn scalar_mul   (self, rhs: Rhs) -> Self::Output; }
pub trait ScalarDiv   <Rhs = Self> { type Output; fn scalar_div   (self, rhs: Rhs) -> Self::Output; }
pub trait ScalarRem   <Rhs = Self> { type Output; fn scalar_rem   (self, rhs: Rhs) -> Self::Output; }
pub trait ScalarBitAnd<Rhs = Self> { type Output; fn scalar_bitand(self, rhs: Rhs) -> Self::Output; }
pub trait ScalarBitOr <Rhs = Self> { type Output; fn scalar_bitor (self, rhs: Rhs) -> Self::Output; }
pub trait ScalarBitXor<Rhs = Self> { type Output; fn scalar_bitxor(self, rhs: Rhs) -> Self::Output; }
pub trait ScalarShl   <Rhs = Self> { type Output; fn scalar_shl   (self, rhs: Rhs) -> Self::Output; }
pub trait ScalarShr   <Rhs = Self> { type Output; fn scalar_shr   (self, rhs: Rhs) -> Self::Output; }

// Custom
pub trait MatrixMul<Rhs = Self> { type Output; fn matrix_mul(self, rhs: Rhs) -> Self::Output; }
pub trait Dot      <Rhs = Self> { type Output; fn dot       (self, rhs: Rhs) -> Self::Output; }
// Inner
// Outer
// Kronecker
pub trait Cross    <Rhs = Self> { type Output; fn cross     (self, rhs: Rhs) -> Self::Output; }

