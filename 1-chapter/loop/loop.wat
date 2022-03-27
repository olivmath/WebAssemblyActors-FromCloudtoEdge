(module
    (func $forLoop (param $x i32) (result i32)
        (local $counter i32)
        (local $result i32)

        (local.set $counter (i32.const 0))
        (local.set $result (i32.const 0))

        (block
            (loop
                ;; Increment the counter in loop
                (local.set $counter (call $increment (local.get $counter)))
                (local.set $result (i32.add (local.get $result) (local.get $counter)))
                ;; Comparing values to exit the loop
                (br_if 1 (i32.eq (local.get $counter) (local.get $x)))
                (br 0)
            )
        )
        (local.get $x)
    )

    (func $increment (param $x i32) (result i32)
        (i32.add (local.get $x) (i32.const 1))
    )

    (export "forLoop" (func $forLoop))
    (export "increment" (func $increment))
)