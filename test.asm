.data
string__bye: .asciz "Bye World!"
string__wait: .asciz "I'm also here!"
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
    movq %rax, %rbx               # Copy `is_true` into `temp_0`
    testq %rbx, %rbx              # Test if `temp_0` is `0`
    je .label_0                   # Jump to label `label_0` if `temp_0` test passed
    movq $10, %rdx                # Load number '10' into `temp_1`
    movq $string__bye, %rcx       # Load 'string__bye' string pointer into `bye`
    movq %rcx, %rsi               # Copy `bye` into `temp_2`
    xchgq %rsi, %r8               # Swap temp_2 and @swap_temp_1
    movq $print_format, %rdi      # Load `print_format` as argument 0
    movq %r8, %rsi                # Load `temp_2` as argument 1
    call printf                   # Call function `printf`
    jmp .label_1                  # Jump to label `label_1`
    .label_0:                     # Create label `label_0`
    movq $1, %rsi                 # Load boolean 'true' into `temp_3`
    testq %rsi, %rsi              # Test if `temp_3` is `0`
    je .label_2                   # Jump to label `label_2` if `temp_3` test passed
    movq $14, %r9                 # Load number '14' into `temp_4`
    movq $string__wait, %rdi      # Load 'string__wait' string pointer into `wait`
    movq %rdi, %r10               # Copy `wait` into `temp_5`
    xchgq %rdi, %r11              # Swap wait and @swap_temp_0
    xchgq %rsi, %r12              # Swap temp_3 and @swap_temp_1
    movq $print_format, %rdi      # Load `print_format` as argument 0
    movq %r10, %rsi               # Load `temp_5` as argument 1
    call printf                   # Call function `printf`
    jmp .label_3                  # Jump to label `label_3`
    .label_2:                     # Create label `label_2`
    movq $1, %rsi                 # Load boolean 'true' into `temp_6`
    testq %rsi, %rsi              # Test if `temp_6` is `0`
    je .label_4                   # Jump to label `label_4` if `temp_6` test passed
    movq $12, %r13                # Load number '12' into `temp_7`
    movq $string__text, %rdi      # Load 'string__text' string pointer into `text`
    movq %rdi, %r14               # Copy `text` into `temp_8`
    xchgq %rdi, %r15              # Swap text and @swap_temp_0
    xchgq %rsi, -0(%rax)          # Swap temp_6 and @swap_temp_1
    movq $print_format, %rdi      # Load `print_format` as argument 0
    movq %r14, %rsi               # Load `temp_8` as argument 1
    call printf                   # Call function `printf`
    .label_4:                     # Create label `label_4`
    .label_3:                     # Create label `label_3`
    .label_1:                     # Create label `label_1`
    movq $0, %rsi                 # Load number '0' into `temp_9`
    movq %rsi, %rax               # Return `temp_9`
    # Epilogue
    # leave
    movq %rbp, %rsp
    pop %rbp
    ret
