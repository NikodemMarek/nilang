
.data
printi_format: .asciz "%d\n"
printc_format: .asciz "%c\n"

.text
.globl _start
_start:
    call main
    movq $60, %rax
    xorq %rdi, %rdi
    syscall
        
.globl test
test:
    # Prologue
    pushq %rbp
    movq %rsp, %rbp
    movq %rdi, %rdi               # Load `a` as argument 0
    movq $44, %rbx                # Load number '44' into `p.x`
    movq %rdi, %rcx               # Copy `a` into `p.y`
    movq %rbx, %rdx               # Copy `p.x` into `temp_0`
    movq %rdx, %rax               # Return `temp_0`
    # Epilogue
    # leave
    movq %rbp, %rsp
    pop %rbp
    ret

.globl main
main:
    # Prologue
    pushq %rbp
    movq %rsp, %rbp
    movq $5, %rbx                 # Load number '5' into `temp_0`
    movq %rbx, %rdi               # Load `temp_0` as argument 0
    call test                     # Call function `test`
    movq %rax, %rax               # Move result of `test` to return register
    movq %rax, %rcx               # Copy `test` into `temp_2`
    movq $8, %rdx                 # Load number '8' into `temp_3`
    movq %rcx, %rdi               # Prepare `temp_1` for addition
    addq %rdx, %rdi               # Add `temp_2` and `temp_3` into `temp_1`
    xchgq %rdi, %rsi              # Swap temp_1 and @swap_temp_0
    xchgq %rsi, %r8               # Swap temp_1 and @swap_temp_1
    movq $printi_format, %rdi     # Load `printi_format` as argument 0
    movq %r8, %rsi                # Load `temp_1` as argument 1
    call printf                   # Call function `printf`
    movq $0, %rsi                 # Load number '0' into `temp_4`
    movq %rsi, %rax               # Return `temp_4`
    # Epilogue
    # leave
    movq %rbp, %rsp
    pop %rbp
    ret
