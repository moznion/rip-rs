/// Parsed is a tuple type which has a T-typed value end a cursor for bytes reading.
pub type Parsed<T> = (T, usize);
