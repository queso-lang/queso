(module
 (type $i32_=>_i32 (func (param i32) (result i32)))
 (type $i32_i32_=>_i32 (func (param i32 i32) (result i32)))
 (type $none_=>_i32 (func (result i32)))
 (type $i32_=>_none (func (param i32)))
 (type $i32_i32_i32_i32_=>_i32 (func (param i32 i32 i32 i32) (result i32)))
 (type $none_=>_none (func))
 (import "wasi_unstable" "fd_write" (func $~rt/fd_write (param i32 i32 i32 i32) (result i32)))
 (global $~rt/heapTop (mut i32) (i32.const 8000))
 (memory $0 2 2)
 (data (i32.const 0) "")
 (table $0 2 2 funcref)
 (elem $0 (i32.const 0) $fn_fuiw8 $fn_u4e87k)
 (export "memory" (memory $0))
 (export "_start" (func $_start))
 (func $~rt/alloc (param $0 i32) (result i32)
  (local $1 i32)
  (global.set $~rt/heapTop
   (i32.add
    (local.tee $1
     (global.get $~rt/heapTop)
    )
    (local.get $0)
   )
  )
  (return
   (local.get $1)
  )
 )
 (func $~rt/createValue/null (result i32)
  (local $0 i32)
  (local.set $0
   (call $~rt/alloc
    (i32.const 4)
   )
  )
  (i32.store
   (local.get $0)
   (i32.const 0)
  )
  (return
   (local.get $0)
  )
 )
 (func $~rt/createValue/number (param $0 i32) (result i32)
  (local $1 i32)
  (local.set $1
   (call $~rt/alloc
    (i32.const 8)
   )
  )
  (i32.store
   (local.get $1)
   (i32.const 1)
  )
  (i32.store offset=4
   (local.get $1)
   (local.get $0)
  )
  (return
   (local.get $1)
  )
 )
 (func $~rt/createValue/string (param $0 i32) (param $1 i32) (result i32)
  (local $2 i32)
  (local.set $2
   (call $~rt/alloc
    (i32.const 12)
   )
  )
  (i32.store
   (local.get $2)
   (i32.const 2)
  )
  (i32.store offset=4
   (local.get $2)
   (local.get $0)
  )
  (i32.store offset=8
   (local.get $2)
   (local.get $1)
  )
  (return
   (local.get $2)
  )
 )
 (func $~rt/createValue/closure (param $0 i32) (param $1 i32) (result i32)
  (local $2 i32)
  (local.set $2
   (call $~rt/alloc
    (i32.add
     (i32.const 8)
     (local.get $1)
    )
   )
  )
  (i32.store
   (local.get $2)
   (i32.const 3)
  )
  (i32.store offset=4
   (local.get $2)
   (local.get $1)
  )
  (return
   (local.get $2)
  )
 )
 (func $~rt/numberToAscii (param $0 i32) (result i32)
  (i32.add
   (local.get $0)
   (i32.const 30)
  )
 )
 (func $~rt/toString (param $0 i32) (result i32)
  (local $1 i32)
  (local.set $1
   (i32.load
    (local.get $0)
   )
  )
  (if (result i32)
   (i32.eq
    (local.get $1)
    (i32.const 2)
   )
   (local.get $0)
   (i32.const 0)
  )
 )
 (func $~rt/print (param $0 i32)
  (local $1 i32)
  (local $2 i32)
  (local.set $1
   (local.get $0)
  )
  (local.set $2
   (call $~rt/alloc
    (i32.const 8)
   )
  )
  (i32.store
   (local.get $2)
   (i32.load offset=8
    (local.get $1)
   )
  )
  (i32.store offset=4
   (local.get $2)
   (i32.load offset=4
    (local.get $1)
   )
  )
  (drop
   (call $~rt/fd_write
    (i32.const 1)
    (local.get $2)
    (i32.const 1)
    (call $~rt/alloc
     (i32.const 4)
    )
   )
  )
 )
 (func $fn_fuiw8 (param $0 i32) (result i32)
  (local $1 i32)
  (local $2 i32)
  (drop
   (local.tee $1
    (call $~rt/createValue/null)
   )
  )
  (drop
   (i32.add
    (i32.const 0)
    (local.get $0)
   )
  )
  (drop
   (i32.add
    (i32.const 4)
    (local.get $0)
   )
  )
  (local.get $1)
 )
 (func $fn_u4e87k (param $0 i32) (result i32)
  (local $1 i32)
  (local $2 i32)
  (local $3 i32)
  (drop
   (local.tee $1
    (call $~rt/createValue/null)
   )
  )
  (drop
   (local.tee $2
    (call $~rt/createValue/null)
   )
  )
  (block
   (local.set $3
    (call $~rt/createValue/closure
     (i32.const 0)
     (i32.const 2)
    )
   )
   (i32.store offset=8
    (i32.const 3)
    (local.get $2)
   )
   (i32.store offset=12
    (i32.const 3)
    (local.get $1)
   )
   (return
    (local.get $3)
   )
  )
 )
 (func $~program (param $0 i32) (result i32)
  (local $1 i32)
  (local $2 i32)
  (drop
   (local.tee $1
    (block
     (local.set $2
      (call $~rt/createValue/closure
       (i32.const 1)
       (i32.const 0)
      )
     )
     (return
      (local.get $2)
     )
    )
   )
  )
  (block (result i32)
   (local.set $2
    (local.get $1)
   )
   (call $fn_fuiw8)
  )
 )
 (func $_start
  (call $~rt/print
   (call $~program
    (i32.const -1)
   )
  )
 )
)