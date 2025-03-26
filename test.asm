.data
print_format: .asciz "%d\n"

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
        
    movq $5, %rax                 # Load number '5' into `temp_0`
    movq $print_format, %rdi      # Load `print_format` as argument 0
    movq %rax, %rsi               # Load `temp_0` as argument 1
    call printf                   # Call function `printf`
    movq %rax, %rbx               # Move result of `printf` to return register
    movq $0, %rcx                 # Load number '0' into `temp_1`
    movq %rcx, %rax               # Return `temp_1`

    # Epilogue
    # leave
    movq %rbp, %rsp
    pop %rbp
    ret
        