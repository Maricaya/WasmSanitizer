# rust-wasm-instrumentation

Compile: `cargo build --bin test`

Use:  `./target/debug/test input.wasm output.wasm [func]`

Function list: [return value] [function name]

>- 11 null_reference
>- 12 implicit_signed_integer_truncation
>- 13 implicit_unsigned_integer_truncation
>- 14 float_cast_overflow
>- 15 non_return
>- 16 implicit_integer_sign_change
>- 17 shift
>- 18 unsigned_integer_overflow
>- 19 signed_integer_overflow
>- 20 float_divide_by_zero
>- 21 heap_canary
>- 22 stack_canary

