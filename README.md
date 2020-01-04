# cgmath-std140

Conversions [cgmath](https://crates.io/crates/cgmath) types to [std140](https://crates.io/crates/std140) types.

## Example

```rust
use cgmath_std140::AsStd140;
use cgmath::Vector4;

let vector: Vector4 = Vector4::new(1.0, 2.0, 3.0, 4.0);
let converted: std140::vec4 = matrix.as_std140();
```

## Licence

MIT
