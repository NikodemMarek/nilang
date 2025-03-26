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
        
.globl _test
_test:

    # Prologue
    pushq %rbp
    movq %rsp, %rbp
        
    movq %rdi, %rdi               # Load `a` as argument 0
    movq %rsi, %rsi               # Load `b.x` as argument 1
    movq %rdx, %rdx               # Load `b.y` as argument 2
    movq %rdi, %rax               # Copy `a` into `temp_0`
    movq %rax, %rax               # Return `temp_0`

    # Epilogue
    # leave
    movq %rbp, %rsp
    pop %rbp
    ret
        

.globl _main
_main:

    # Prologue
    pushq %rbp
    movq %rsp, %rbp
        
    movq $5, %rax                 # Load number '5' into `p.y`
    movq $6, %rbx                 # Load number '6' into `p.x`
    movq $9, %rcx                 # Load number '9' into `temp_1`
    movq %rbx, %rdx               # Copy `p.x` into `temp_2.x`
    movq %rax, %rsi               # Copy `p.y` into `temp_2.y`
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
        