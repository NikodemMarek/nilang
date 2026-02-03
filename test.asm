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
    movq $1, %rax                 # Load boolean 'true' into `is_true`
    movq $12, %rcx                # Load number '12' into `temp_0`
    movq $string__text, %rbx      # Load 'string__text' string pointer into `text`
    .label_0:                     # Create label `label_0`
    movq %rax, %rdx               # Copy `is_true` into `temp_1`
    testq %rdx, %rdx              # Test if `temp_1` is `0`
    je .label_1                   # Jump to label `label_1` if `temp_1` test passed
    movq %rbx, %rsi               # Copy `text` into `temp_2`
    xchgq %rsi, %r8               # Swap temp_2 and @swap_temp_1
    movq $print_format, %rdi      # Load `print_format` as argument 0
    movq %r8, %rsi                # Load `temp_2` as argument 1
    call printf                   # Call function `printf`
    movq %rax, %rsi               # Copy `is_true` into `temp_3`
    testq %rsi, %rsi              # Test if `temp_3` is `0`
    je .label_2                   # Jump to label `label_2` if `temp_3` test passed
    movq $0, %rax                 # Load boolean 'false' into `is_true`
    .label_2:                     # Create label `label_2`
    jmp .label_0                  # Jump to label `label_0`
    .label_1:                     # Create label `label_1`
    movq $0, %rdi                 # Load number '0' into `temp_4`
    movq %rdi, %rax               # Return `temp_4`
    # Epilogue
    # leave
    movq %rbp, %rsp
    pop %rbp
    ret
