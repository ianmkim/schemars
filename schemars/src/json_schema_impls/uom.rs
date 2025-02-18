use crate::JsonSchema;

use uom::si::Quantity;

// This implementation works for any uom::Quantity
// where V (the storage type) implements JsonSchema.
forward_impl!((<D: uom::si::Dimension + ?Sized, U: uom::si::Units<V> + ?Sized, V: JsonSchema + uom::Conversion<V> + uom::num_traits::Num + Sized> JsonSchema for Quantity<D, U, V>) => V);
