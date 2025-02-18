use crate::JsonSchema;
use nalgebra::geometry::{Isometry, OPoint};
use nalgebra::DefaultAllocator;
use nalgebra::{Const, Matrix};

// covers matrices and vectors
forward_impl!(
    (<T: JsonSchema, const N: usize, S> JsonSchema for Matrix<T, Const<N>, Const<1>, S>)
    => alloc::vec::Vec<T>
);

forward_impl!(
    (<T: JsonSchema + nalgebra::Scalar, D:  nalgebra::DimName> JsonSchema for OPoint<T, D>  where DefaultAllocator: nalgebra::allocator::Allocator<D>) => alloc::vec::Vec<T>
);

forward_impl!(
    (<T: JsonSchema, R, const D: usize> JsonSchema for Isometry<T, R, D>) => alloc::vec::Vec<T>
);
