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
        
    movq $5, %rbx                 # Load number '5' into `temp_1`
    movq $8, %rcx                 # Load number '8' into `temp_2`
    movq %rbx, %rax               # Prepare `temp_0` for addition
    addq %rcx, %rax               # Add `temp_1` and `temp_2` into `temp_0`
    movq $printi_format, %rdi     # Load `printi_format` as argument 0
    movq %rax, %rsi               # Load `temp_0` as argument 1
    call printf                   # Call function `printf`
    movq $0, %rsi                 # Load number '0' into `temp_3`
    movq %rsi, %rax               # Return `temp_3`

    # Epilogue
    # leave
    movq %rbp, %rsp
    pop %rbp
    ret
        