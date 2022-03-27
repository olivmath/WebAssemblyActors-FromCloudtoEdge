(module
    (func $add (param $x i32) (param $y i32) (result i32)
        (i32.add
            (local.get $x)
            (local.get $y)
        )
    )
    (export "add" (func $add))

    (func $sub (param $x i32) (param $y i32) (result i32)
        (i32.sub
            (local.get $x)
            (local.get $y)
        )
    )
    (export "sub" (func $sub))

    (func $mul (param $x i32) (param $y i32) (result i32)
        (i32.mul
            (local.get $x)
            (local.get $y)
        )
    )
    (export "mul" (func $mul))

    (func $div (param $x i32) (param $y i32) (result i32)
        (i32.div_s
            (local.get $x)
            (local.get $y)
        )
    )
    (export "div" (func $div))
)
