.data
string__text: .asciz "Hello World!"
string__bye: .asciz "Bye World!"

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
    movq $1, %rax                 # Load boolean 'true' into `is_true`
    movq $12, %rcx                # Load number '12' into `temp_0`
    movq $string__text, %rbx      # Load 'string__text' string pointer into `text`
    movq %rbx, %rdx               # Copy `text` into `temp_1`
    xchgq %rdi, %rsi              # Swap @swap_temp_1 and @swap_temp_0
    movq $print_format, %rdi      # Load `print_format` as argument 0
    movq %rdx, %rsi               # Load `temp_1` as argument 1
    call printf                   # Call function `printf`
    movq %rax, %rsi               # Copy `is_true` into `temp_2`
    testq %rsi, %rsi              # Test if `temp_2` is `0`
    je .label_0                   # Jump to label `label_0` if `temp_2` test passed
    movq $10, %r8                 # Load number '10' into `temp_3`
    movq $string__bye, %rdi       # Load 'string__bye' string pointer into `bye`
    movq %rdi, %r9                # Copy `bye` into `temp_4`
    xchgq %rdi, %r10              # Swap bye and @swap_temp_0
    xchgq %rsi, %r11              # Swap temp_2 and @swap_temp_1
    movq $print_format, %rdi      # Load `print_format` as argument 0
    movq %r9, %rsi                # Load `temp_4` as argument 1
    call printf                   # Call function `printf`
    .label_0:                     # Create label `label_0`
    movq $0, %rsi                 # Load number '0' into `temp_5`
    movq %rsi, %rax               # Return `temp_5`
    # Epilogue
    # leave
    movq %rbp, %rsp
    pop %rbp
    ret
