.data
string__text: .asciz "Hello World!"
printd_format: .asciz "%d\n"
print_format: .asciz "%s\n"
printc_format: .asciz "%c\n"
.text                         # 
.globl _start                 # 
_start:                       # 
call main                     # 
movq $60, %rax                # 
xorq %rdi, %rdi               # 
syscall                       # 
.globl main
main:
    # Prologue
    pushq %rbp
    movq %rsp, %rbp
    movq $12, %rbx                # Load number '12' into `temp_0`
    movq $string__text, %rax      # Load 'string__text' string pointer into `text`
    movq $1, %rcx                 # Load boolean 'true' into `temp_1`
    testq %rcx, %rcx              # Test if `temp_1` is `0`
    je .label_0                   # Jump to label `label_0` if `temp_1` test passed
    movq %rax, %rdx               # Copy `text` into `temp_2`
    xchgq %rdi, %rsi              # Swap @swap_temp_1 and @swap_temp_0
    movq $print_format, %rdi      # Load `print_format` as argument 0
    movq %rdx, %rsi               # Load `temp_2` as argument 1
    call printf                   # Call function `printf`
    .label_0:                     # Create label `label_0`
    movq $0, %rsi                 # Load number '0' into `temp_3`
    movq %rsi, %rax               # Return `temp_3`
    # Epilogue
    # leave
    movq %rbp, %rsp
    pop %rbp
    ret
