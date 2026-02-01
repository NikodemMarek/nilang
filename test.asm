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
    movq $0, %rax                 # Load boolean 'false' into `is_true`
    movq $12, %rcx                # Load number '12' into `temp_0`
    movq $string__text, %rbx      # Load 'string__text' string pointer into `text`
    movq %rax, %rdx               # Copy `is_true` into `temp_1`
    xchgq %rdi, %rsi              # Swap @swap_temp_1 and @swap_temp_0
    movq $printi_format, %rdi     # Load `printi_format` as argument 0
    movq %rdx, %rsi               # Load `temp_1` as argument 1
    call printf                   # Call function `printf`
    movq $0, %rsi                 # Load number '0' into `temp_2`
    movq %rsi, %rax               # Return `temp_2`
    # Epilogue
    # leave
    movq %rbp, %rsp
    pop %rbp
    ret
