(module
 (import "wasi_unstable" "fd_write" (func $fd_write (param i32 i32 i32 i32) (result i32)))
 (memory 1)
 (export "memory" (memory 0))
 (type $none_=>_i32 (func (result i32)))
 (type $i32_=>_i32 (func (param i32) (result i32)))
 (type $f64_=>_i32 (func (param f64) (result i32)))
 (global $~rt/heapTop (mut i32) (i32.const 8000))
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
 (data (i32.const 8001) "hello world\n")
 (func (export "_start")
  (local $0 i32)
  (local.set $0
   (call $~rt/createValue/number
    (i32.const 53)
   )
  )

  (i32.store (i32.const 0) (i32.const 8004))  ;; iov.iov_base - This is a pointer to the start of the 'hello world\n' string
  (i32.store (i32.const 4) (i32.const 1))  ;; iov.iov_len - The length of the 'hello world\n' string

  (call $fd_write
   (i32.const 1) ;; file_descriptor - 1 for stdout
   (i32.const 0) ;; *iovs - The pointer to the iov array, which is stored at memory location 0
   (i32.const 1) ;; iovs_len - We're printing 1 string stored in an iov - so one.
   (i32.const 20) ;; nwritten - A place in memory to store the number of bytes written
  )
  drop
 )
)