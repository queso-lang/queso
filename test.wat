(module
  (import "wasi_unstable" "fd_write" (func $fd_write (param i32 i32 i32 i32) (result i32)))
  (memory 1)
  (export "memory" (memory 0))
  (data (i32.const 8) "hello world\n")
  (func (export "_start")
    (elem declare func $b-function (param $a i32) (param $b i32) (result i32)
      (local $res i32)
      (local $unused f64)
      (local.tee $res
      (i32.add
        (local.get $a)
        (local.get $b)
      )
      )
    )
    (i32.store (i32.const 0) (i32.const 8))  ;; iov.iov_base - This is a pointer to the start of the 'hello world\n' string
    (i32.store (i32.const 4) (i32.const 12))  ;; iov.iov_len - The length of the 'hello world\n' string

    (call $fd_write
        (i32.const 1) ;; file_descriptor - 1 for stdout
        (i32.const 0) ;; *iovs - The pointer to the iov array, which is stored at memory location 0
        (i32.const 1) ;; iovs_len - We're printing 1 string stored in an iov - so one.
        (i32.const 20) ;; nwritten - A place in memory to store the number of bytes written
    )
    drop
  )
)