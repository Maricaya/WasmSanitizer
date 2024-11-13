(module $mul_check.wasm
  (type (;0;) (func (result i32)))
  (type (;1;) (func))
  (type (;2;) (func (param i32)))
  (type (;3;) (func (param i32 i32) (result i32)))
  (type (;4;) (func (param i64 i64) (result i32)))
  (type (;5;) (func (param i32) (result i32)))
  (import "wasi_snapshot_preview1" "proc_exit" (func $__wasi_proc_exit (type 2)))
  (func $__wasm_call_ctors (type 1)
    call $emscripten_stack_init)
  (func $i32_is_mul_overflow (type 3) (param i32 i32) (result i32)
    (local i32 i32 i32 i32 i32 i32 i32 i32 i32 i32 i32 i32 i32 i32 i32 i32 i32 i32 i32 i32 i32 i32 i32 i32 i32 i32 i32 i32 i32 i32 i32 i32 i32 i32 i32 i32 i32 i32 i32 i32 i32 i32 i32 i32 i32 i32 i32 i32 i32 i32 i32 i32 i32 i32 i32 i32 i32 i32 i32 i32 i32 i32 i32 i32 i32 i32 i32 i32 i32 i32 i32 i32 i32 i32 i32 i32 i32 i32 i32 i32 i32)
    global.get $__stack_pointer
    local.set 2
    i32.const 16
    local.set 3
    local.get 2
    local.get 3
    i32.sub
    local.set 4
    local.get 4
    global.set $__stack_pointer
    local.get 4
    local.get 0
    i32.store offset=8
    local.get 4
    local.get 1
    i32.store offset=4
    local.get 4
    i32.load offset=8
    local.set 5
    i32.const 0
    local.set 6
    local.get 5
    local.set 7
    local.get 6
    local.set 8
    local.get 7
    local.get 8
    i32.ge_s
    local.set 9
    i32.const 1
    local.set 10
    local.get 9
    local.get 10
    i32.and
    local.set 11
    block  ;; label = @1
      block  ;; label = @2
        local.get 11
        i32.eqz
        br_if 0 (;@2;)
        local.get 4
        i32.load offset=4
        local.set 12
        i32.const 0
        local.set 13
        local.get 12
        local.set 14
        local.get 13
        local.set 15
        local.get 14
        local.get 15
        i32.ge_s
        local.set 16
        i32.const 1
        local.set 17
        local.get 16
        local.get 17
        i32.and
        local.set 18
        local.get 18
        i32.eqz
        br_if 0 (;@2;)
        local.get 4
        i32.load offset=8
        local.set 19
        i32.const 2147483647
        local.set 20
        local.get 20
        local.get 19
        i32.div_s
        local.set 21
        local.get 4
        i32.load offset=4
        local.set 22
        local.get 21
        local.set 23
        local.get 22
        local.set 24
        local.get 23
        local.get 24
        i32.lt_s
        local.set 25
        i32.const 1
        local.set 26
        local.get 25
        local.get 26
        i32.and
        local.set 27
        local.get 4
        local.get 27
        i32.store offset=12
        br 1 (;@1;)
      end
      local.get 4
      i32.load offset=8
      local.set 28
      i32.const 0
      local.set 29
      local.get 28
      local.set 30
      local.get 29
      local.set 31
      local.get 30
      local.get 31
      i32.lt_s
      local.set 32
      i32.const 1
      local.set 33
      local.get 32
      local.get 33
      i32.and
      local.set 34
      block  ;; label = @2
        local.get 34
        i32.eqz
        br_if 0 (;@2;)
        local.get 4
        i32.load offset=4
        local.set 35
        i32.const 0
        local.set 36
        local.get 35
        local.set 37
        local.get 36
        local.set 38
        local.get 37
        local.get 38
        i32.lt_s
        local.set 39
        i32.const 1
        local.set 40
        local.get 39
        local.get 40
        i32.and
        local.set 41
        local.get 41
        i32.eqz
        br_if 0 (;@2;)
        local.get 4
        i32.load offset=8
        local.set 42
        i32.const 2147483647
        local.set 43
        local.get 43
        local.get 42
        i32.div_s
        local.set 44
        local.get 4
        i32.load offset=4
        local.set 45
        local.get 44
        local.set 46
        local.get 45
        local.set 47
        local.get 46
        local.get 47
        i32.gt_s
        local.set 48
        i32.const 1
        local.set 49
        local.get 48
        local.get 49
        i32.and
        local.set 50
        local.get 4
        local.get 50
        i32.store offset=12
        br 1 (;@1;)
      end
      local.get 4
      i32.load offset=8
      local.set 51
      local.get 4
      i32.load offset=4
      local.set 52
      local.get 51
      local.get 52
      i32.mul
      local.set 53
      i32.const -2147483648
      local.set 54
      local.get 53
      local.set 55
      local.get 54
      local.set 56
      local.get 55
      local.get 56
      i32.eq
      local.set 57
      i32.const 1
      local.set 58
      local.get 57
      local.get 58
      i32.and
      local.set 59
      block  ;; label = @2
        local.get 59
        i32.eqz
        br_if 0 (;@2;)
        i32.const 0
        local.set 60
        local.get 4
        local.get 60
        i32.store offset=12
        br 1 (;@1;)
      end
      local.get 4
      i32.load offset=8
      local.set 61
      i32.const 0
      local.set 62
      local.get 61
      local.set 63
      local.get 62
      local.set 64
      local.get 63
      local.get 64
      i32.lt_s
      local.set 65
      i32.const 1
      local.set 66
      local.get 65
      local.get 66
      i32.and
      local.set 67
      block  ;; label = @2
        block  ;; label = @3
          local.get 67
          i32.eqz
          br_if 0 (;@3;)
          local.get 4
          i32.load offset=8
          local.set 68
          i32.const 0
          local.set 69
          local.get 69
          local.get 68
          i32.sub
          local.set 70
          local.get 4
          i32.load offset=4
          local.set 71
          local.get 70
          local.get 71
          call $i32_is_mul_overflow
          local.set 72
          local.get 72
          local.set 73
          br 1 (;@2;)
        end
        local.get 4
        i32.load offset=8
        local.set 74
        local.get 4
        i32.load offset=4
        local.set 75
        i32.const 0
        local.set 76
        local.get 76
        local.get 75
        i32.sub
        local.set 77
        local.get 74
        local.get 77
        call $i32_is_mul_overflow
        local.set 78
        local.get 78
        local.set 73
      end
      local.get 73
      local.set 79
      local.get 4
      local.get 79
      i32.store offset=12
    end
    local.get 4
    i32.load offset=12
    local.set 80
    i32.const 16
    local.set 81
    local.get 4
    local.get 81
    i32.add
    local.set 82
    local.get 82
    global.set $__stack_pointer
    local.get 80
    return)
  (func $i64_is_mul_overflow (type 4) (param i64 i64) (result i32)
    (local i32 i32 i32 i64 i64 i64 i64 i32 i32 i32 i64 i64 i64 i64 i32 i32 i32 i64 i64 i64 i64 i64 i64 i32 i32 i32 i64 i64 i64 i64 i32 i32 i32 i64 i64 i64 i64 i32 i32 i32 i64 i64 i64 i64 i64 i64 i32 i32 i32 i64 i64 i64 i64 i64 i64 i32 i32 i32 i32 i64 i64 i64 i64 i32 i32 i32 i64 i64 i64 i64 i32 i32 i64 i64 i64 i64 i32 i32 i32 i32 i32)
    global.get $__stack_pointer
    local.set 2
    i32.const 32
    local.set 3
    local.get 2
    local.get 3
    i32.sub
    local.set 4
    local.get 4
    global.set $__stack_pointer
    local.get 4
    local.get 0
    i64.store offset=16
    local.get 4
    local.get 1
    i64.store offset=8
    local.get 4
    i64.load offset=16
    local.set 5
    i64.const 0
    local.set 6
    local.get 5
    local.set 7
    local.get 6
    local.set 8
    local.get 7
    local.get 8
    i64.ge_s
    local.set 9
    i32.const 1
    local.set 10
    local.get 9
    local.get 10
    i32.and
    local.set 11
    block  ;; label = @1
      block  ;; label = @2
        local.get 11
        i32.eqz
        br_if 0 (;@2;)
        local.get 4
        i64.load offset=8
        local.set 12
        i64.const 0
        local.set 13
        local.get 12
        local.set 14
        local.get 13
        local.set 15
        local.get 14
        local.get 15
        i64.ge_s
        local.set 16
        i32.const 1
        local.set 17
        local.get 16
        local.get 17
        i32.and
        local.set 18
        local.get 18
        i32.eqz
        br_if 0 (;@2;)
        local.get 4
        i64.load offset=16
        local.set 19
        i64.const 9223372036854775807
        local.set 20
        local.get 20
        local.get 19
        i64.div_s
        local.set 21
        local.get 4
        i64.load offset=8
        local.set 22
        local.get 21
        local.set 23
        local.get 22
        local.set 24
        local.get 23
        local.get 24
        i64.lt_s
        local.set 25
        i32.const 1
        local.set 26
        local.get 25
        local.get 26
        i32.and
        local.set 27
        local.get 4
        local.get 27
        i32.store offset=28
        br 1 (;@1;)
      end
      local.get 4
      i64.load offset=16
      local.set 28
      i64.const 0
      local.set 29
      local.get 28
      local.set 30
      local.get 29
      local.set 31
      local.get 30
      local.get 31
      i64.lt_s
      local.set 32
      i32.const 1
      local.set 33
      local.get 32
      local.get 33
      i32.and
      local.set 34
      block  ;; label = @2
        local.get 34
        i32.eqz
        br_if 0 (;@2;)
        local.get 4
        i64.load offset=8
        local.set 35
        i64.const 0
        local.set 36
        local.get 35
        local.set 37
        local.get 36
        local.set 38
        local.get 37
        local.get 38
        i64.lt_s
        local.set 39
        i32.const 1
        local.set 40
        local.get 39
        local.get 40
        i32.and
        local.set 41
        local.get 41
        i32.eqz
        br_if 0 (;@2;)
        local.get 4
        i64.load offset=16
        local.set 42
        i64.const 9223372036854775807
        local.set 43
        local.get 43
        local.get 42
        i64.div_s
        local.set 44
        local.get 4
        i64.load offset=8
        local.set 45
        local.get 44
        local.set 46
        local.get 45
        local.set 47
        local.get 46
        local.get 47
        i64.gt_s
        local.set 48
        i32.const 1
        local.set 49
        local.get 48
        local.get 49
        i32.and
        local.set 50
        local.get 4
        local.get 50
        i32.store offset=28
        br 1 (;@1;)
      end
      local.get 4
      i64.load offset=16
      local.set 51
      local.get 4
      i64.load offset=8
      local.set 52
      local.get 51
      local.get 52
      i64.mul
      local.set 53
      i64.const -9223372036854775808
      local.set 54
      local.get 53
      local.set 55
      local.get 54
      local.set 56
      local.get 55
      local.get 56
      i64.eq
      local.set 57
      i32.const 1
      local.set 58
      local.get 57
      local.get 58
      i32.and
      local.set 59
      block  ;; label = @2
        local.get 59
        i32.eqz
        br_if 0 (;@2;)
        i32.const 0
        local.set 60
        local.get 4
        local.get 60
        i32.store offset=28
        br 1 (;@1;)
      end
      local.get 4
      i64.load offset=16
      local.set 61
      i64.const 0
      local.set 62
      local.get 61
      local.set 63
      local.get 62
      local.set 64
      local.get 63
      local.get 64
      i64.lt_s
      local.set 65
      i32.const 1
      local.set 66
      local.get 65
      local.get 66
      i32.and
      local.set 67
      block  ;; label = @2
        block  ;; label = @3
          local.get 67
          i32.eqz
          br_if 0 (;@3;)
          local.get 4
          i64.load offset=16
          local.set 68
          i64.const 0
          local.set 69
          local.get 69
          local.get 68
          i64.sub
          local.set 70
          local.get 4
          i64.load offset=8
          local.set 71
          local.get 70
          local.get 71
          call $i64_is_mul_overflow
          local.set 72
          local.get 72
          local.set 73
          br 1 (;@2;)
        end
        local.get 4
        i64.load offset=16
        local.set 74
        local.get 4
        i64.load offset=8
        local.set 75
        i64.const 0
        local.set 76
        local.get 76
        local.get 75
        i64.sub
        local.set 77
        local.get 74
        local.get 77
        call $i64_is_mul_overflow
        local.set 78
        local.get 78
        local.set 73
      end
      local.get 73
      local.set 79
      local.get 4
      local.get 79
      i32.store offset=28
    end
    local.get 4
    i32.load offset=28
    local.set 80
    i32.const 32
    local.set 81
    local.get 4
    local.get 81
    i32.add
    local.set 82
    local.get 82
    global.set $__stack_pointer
    local.get 80
    return)
  (func $__original_main (type 0) (result i32)
    (local i32 i32 i64 i64 i32)
    i32.const 2147483647
    local.set 0
    i32.const 2
    local.set 1
    local.get 0
    local.get 1
    call $i32_is_mul_overflow
    drop
    i64.const 9223372036854775807
    local.set 2
    i64.const 2
    local.set 3
    local.get 2
    local.get 3
    call $i64_is_mul_overflow
    drop
    i32.const 0
    local.set 4
    local.get 4
    return)
  (func $_start (type 1)
    block  ;; label = @1
      i32.const 1
      i32.eqz
      br_if 0 (;@1;)
      call $__wasm_call_ctors
    end
    call $__original_main
    call $exit
    unreachable)
  (func $dummy (type 1))
  (func $libc_exit_fini (type 1)
    (local i32)
    i32.const 0
    local.set 0
    block  ;; label = @1
      i32.const 0
      i32.const 0
      i32.le_u
      br_if 0 (;@1;)
      loop  ;; label = @2
        local.get 0
        i32.const -4
        i32.add
        local.tee 0
        i32.load
        call_indirect (type 1)
        local.get 0
        i32.const 0
        i32.gt_u
        br_if 0 (;@2;)
      end
    end
    call $dummy)
  (func $exit (type 2) (param i32)
    call $dummy
    call $libc_exit_fini
    call $dummy
    local.get 0
    call $_Exit
    unreachable)
  (func $_Exit (type 2) (param i32)
    local.get 0
    call $__wasi_proc_exit
    unreachable)
  (func $__errno_location (type 0) (result i32)
    i32.const 65536)
  (func $emscripten_stack_init (type 1)
    i32.const 65536
    global.set $__stack_base
    i32.const 0
    i32.const 15
    i32.add
    i32.const -16
    i32.and
    global.set $__stack_end)
  (func $emscripten_stack_get_free (type 0) (result i32)
    global.get $__stack_pointer
    global.get $__stack_end
    i32.sub)
  (func $emscripten_stack_get_base (type 0) (result i32)
    global.get $__stack_base)
  (func $emscripten_stack_get_end (type 0) (result i32)
    global.get $__stack_end)
  (func $stackSave (type 0) (result i32)
    global.get $__stack_pointer)
  (func $stackRestore (type 2) (param i32)
    local.get 0
    global.set $__stack_pointer)
  (func $stackAlloc (type 5) (param i32) (result i32)
    (local i32 i32)
    global.get $__stack_pointer
    local.get 0
    i32.sub
    i32.const -16
    i32.and
    local.tee 1
    global.set $__stack_pointer
    local.get 1)
  (func $emscripten_stack_get_current (type 0) (result i32)
    global.get $__stack_pointer)
  (table (;0;) 2 2 funcref)
  (memory (;0;) 256 256)
  (global $__stack_pointer (mut i32) (i32.const 65536))
  (global $__stack_end (mut i32) (i32.const 0))
  (global $__stack_base (mut i32) (i32.const 0))
  (export "memory" (memory 0))
  (export "__indirect_function_table" (table 0))
  (export "_start" (func $_start))
  (export "__errno_location" (func $__errno_location))
  (export "emscripten_stack_init" (func $emscripten_stack_init))
  (export "emscripten_stack_get_free" (func $emscripten_stack_get_free))
  (export "emscripten_stack_get_base" (func $emscripten_stack_get_base))
  (export "emscripten_stack_get_end" (func $emscripten_stack_get_end))
  (export "stackSave" (func $stackSave))
  (export "stackRestore" (func $stackRestore))
  (export "stackAlloc" (func $stackAlloc))
  (export "emscripten_stack_get_current" (func $emscripten_stack_get_current))
  (elem (;0;) (i32.const 1) func $__wasm_call_ctors))
