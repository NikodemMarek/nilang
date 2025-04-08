.data
string__text: .asciz "Hello World!"

.data
printi_format: .asciz "%d\n"
printc_format: .asciz "%c\n"
print_format: .asciz "%s\n"

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
    movq $12, %rbx                # Load number '12' into `temp_0`
    movq $string__text, %rax      # Load 'string__text' string pointer into `text`
    movq %rax, %rcx               # Copy `text` into `temp_1`
    movq $print_format, %rdi      # Load `print_format` as argument 0
    movq %rcx, %rsi               # Load `temp_1` as argument 1
    call printf                   # Call function `printf`
    movq $0, %rdx                 # Load number '0' into `temp_2`
    movq %rdx, %rax               # Return `temp_2`
    # Epilogue
    # leave
    movq %rbp, %rsp
    pop %rbp
    ret
