# How to use

## 1. Write `.wat` text file

```wat
(module
    (func $sum (param $x i32) (param $y i32) (result i32)
        (i32.add
            (local.get $x)
            (local.get $y)
        )
    )
    (export "sum" (func $sum))
)
```


## 2. wat to wasm

`wat2wasm sum.wat`

## 2. Run your functions

`wasmtime sum.wasm --invoke sum 2 45`