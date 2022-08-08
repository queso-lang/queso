(module
  (memory 1)
  (export "memory" (memory 0))
  (type $i32_i32_=>_i32 (func (param i32 i32) (result i32)))
  (func (export "_start") (result i32)
    (local $res i32)
    (local $unused f64)
    (local $unused2 f64)
    (local.tee $res
      (i32.add
        (local.get $a)
        (local.get $b)
      )
    )
  )
)