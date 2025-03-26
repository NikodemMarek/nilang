.text

.globl _start
_start:
    call _main
    # movq $60, %rax
    # xorq %rdi, %rdi
    # syscall
    movq %rax, %rbx
    movq $1, %rax
    int $0x80
    ret
        
.globl _main
_main:

    # Prologue
    pushq %rbp
    movq %rsp, %rbp
        
    movq $6, %rax                 # Load number '6' into `p.x`
    movq $5, %rbx                 # Load number '5' into `p.y`
    movq $5, %rcx                 # Load number '5' into `temp_1`
    movq %rax, %rdx               # Copy `p.x` into `temp_2.x`
    movq %rbx, %rsi               # Copy `p.y` into `temp_2.y`
    movq %rsi, %r9                # Move `temp_2.y` to a free location
    movq %rdx, %r8                # Move `temp_2.x` to a free location
    movq %rcx, %rdi               # Load `temp_1` as argument 0
    movq %r8, %rsi                # Load `temp_2.x` as argument 1
    movq %r9, %rdx                # Load `temp_2.y` as argument 2
    call _test                    # Call function `test`
    movq %rax, %rdi               # Move result of `test` to return register
    movq %rdi, %rax               # Return `temp_0`

    # Epilogue
    # leave
    movq %rbp, %rsp
    pop %rbp
    ret
        

.globl _test
_test:

    # Prologue
    pushq %rbp
    movq %rsp, %rbp
        
    movq %rdi, %rdi               # Load `a` as argument 0
    movq %rsi, %rsi               # Load `b.x` as argument 1
    movq %rdx, %rdx               # Load `b.y` as argument 2
    movq %rdi, %rax               # Copy `a` into `temp_2`
    movq %rsi, %rbx               # Copy `b.x` into `temp_3`
    movq %rax, %rcx               # Prepare `temp_1` for multiplication
    imulq %rbx, %rcx              # Multiply `temp_2` and `temp_3` into `temp_1`
    movq $5, %r8                  # Load number '5' into `temp_4`
    movq %rcx, %r9                # Prepare `temp_0` for addition
    addq %r8, %r9                 # Add `temp_1` and `temp_4` into `temp_0`
    movq %r9, %rax                # Return `temp_0`

    # Epilogue
    # leave
    movq %rbp, %rsp
    pop %rbp
    ret
        