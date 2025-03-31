
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
        
.globl main
main:
    # Prologue
    pushq %rbp
    movq %rsp, %rbp
    movq $9, %rbx                 # Load number '9' into `temp_1`
    movq $5, %rcx                 # Load number '5' into `temp_2`
    xchgq %rax, %rsi              # Swap temp_0 and @swap_temp_0
    xchgq %rdx, %rdi              # Swap @empty and @swap_temp_1
    movq %rbx, %rax               # Load `temp_1` as argument 0
    movq %rdi, %rdx               # Load `@empty` as argument 1
    movq $0, %rdx                 # Prepare `temp_0` for modulo
    movq %rbx, %rax               # Prepare `temp_0` for modulo
    idivq %rcx                    # Divide `temp_1` by `temp_2`
    movq %rdx, %rsi               # Move result of modulo into `temp_0`
    xchgq %rsi, %rdx              # Swap temp_0 and @swap_temp_1
    movq $printi_format, %rdi     # Load `printi_format` as argument 0
    movq %rdx, %rsi               # Load `temp_0` as argument 1
    call printf                   # Call function `printf`
    movq $0, %rax                 # Load number '0' into `temp_3`
    movq %rax, %rax               # Return `temp_3`
    # Epilogue
    # leave
    movq %rbp, %rsp
    pop %rbp
    ret
