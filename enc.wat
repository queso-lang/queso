(module
 (import "wasi_snapshot_preview1" "fd_write"
         (func $fd_write (param i32 i32 i32 i32) (result i32)))
 (data $d0 "\c3\84\0a")
 (global $offset i32 (i32.const 8))  ;; where to place data in memory
 (global $skip   i32 (i32.const 0))  ;; where to start copying from
 (global $length i32 (i32.const 3)) ;; how many bytes to copy
 (memory (export "memory") 1)
 (func (export "_start")
       ;; step 1: copy the passive data segment into memory
       (memory.init $d0
        (global.get $offset)
        (global.get $skip)
        (global.get $length))
       ;; step 2: store the string's address and length in memory
       (i32.store (i32.const 0)         ;; iov_base location in memory
                  (global.get $offset)) ;; iov_base value
       (i32.store (i32.const 4)         ;; iov_size location in memory
                  (global.get $length)) ;; iov_size value
       ;; step 3: write the string from memory
       (drop (call $fd_write
                   (i32.const 1)        ;; fd
                   (i32.const 0)        ;; iovs location in memory
                   (i32.const 1)        ;; iovs_len
                   (i32.const 0)))))    ;; retptr0 location in memory