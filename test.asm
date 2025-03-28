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
        
    movq %rdi, %rdi               # Load `p` as argument 0

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
        
    movq $'H', %rax               # Load character 'H' into `temp_0`
    movq $printc_format, %rdi     # Load `printc_format` as argument 0
    movq %rax, %rsi               # Load `temp_0` as argument 1
    call printf                   # Call function `printf`
    movq $'e', %rsi               # Load character 'e' into `temp_1`
    xchgq %rsi, %rcx              # Swap temp_1 and @swap_temp_1
    movq $printc_format, %rdi     # Load `printc_format` as argument 0
    movq %rcx, %rsi               # Load `temp_1` as argument 1
    call printf                   # Call function `printf`
    movq $'l', %rsi               # Load character 'l' into `temp_2`
    xchgq %rsi, %rbx              # Swap temp_2 and @swap_temp_1
    movq $printc_format, %rdi     # Load `printc_format` as argument 0
    movq %rbx, %rsi               # Load `temp_2` as argument 1
    call printf                   # Call function `printf`
    movq $'l', %rsi               # Load character 'l' into `temp_3`
    xchgq %rsi, %rdx              # Swap temp_3 and @swap_temp_1
    movq $printc_format, %rdi     # Load `printc_format` as argument 0
    movq %rdx, %rsi               # Load `temp_3` as argument 1
    call printf                   # Call function `printf`
    movq $'o', %rsi               # Load character 'o' into `temp_4`
    xchgq %rsi, %r8               # Swap temp_4 and @swap_temp_1
    movq $printc_format, %rdi     # Load `printc_format` as argument 0
    movq %r8, %rsi                # Load `temp_4` as argument 1
    call printf                   # Call function `printf`
    movq $' ', %rsi               # Load character ' ' into `temp_5`
    xchgq %rsi, %r9               # Swap temp_5 and @swap_temp_1
    movq $printc_format, %rdi     # Load `printc_format` as argument 0
    movq %r9, %rsi                # Load `temp_5` as argument 1
    call printf                   # Call function `printf`
    movq $'W', %rsi               # Load character 'W' into `temp_6`
    xchgq %rsi, %r10              # Swap temp_6 and @swap_temp_1
    movq $printc_format, %rdi     # Load `printc_format` as argument 0
    movq %r10, %rsi               # Load `temp_6` as argument 1
    call printf                   # Call function `printf`
    movq $'o', %rsi               # Load character 'o' into `temp_7`
    xchgq %rsi, %r11              # Swap temp_7 and @swap_temp_1
    movq $printc_format, %rdi     # Load `printc_format` as argument 0
    movq %r11, %rsi               # Load `temp_7` as argument 1
    call printf                   # Call function `printf`
    movq $'r', %rsi               # Load character 'r' into `temp_8`
    xchgq %rsi, %r12              # Swap temp_8 and @swap_temp_1
    movq $printc_format, %rdi     # Load `printc_format` as argument 0
    movq %r12, %rsi               # Load `temp_8` as argument 1
    call printf                   # Call function `printf`
    movq $'l', %rsi               # Load character 'l' into `temp_9`
    xchgq %rsi, %r13              # Swap temp_9 and @swap_temp_1
    movq $printc_format, %rdi     # Load `printc_format` as argument 0
    movq %r13, %rsi               # Load `temp_9` as argument 1
    call printf                   # Call function `printf`
    movq $'d', %rsi               # Load character 'd' into `temp_10`
    xchgq %rsi, %r14              # Swap temp_10 and @swap_temp_1
    movq $printc_format, %rdi     # Load `printc_format` as argument 0
    movq %r14, %rsi               # Load `temp_10` as argument 1
    call printf                   # Call function `printf`
    movq $'!', %rsi               # Load character '!' into `temp_11`
    xchgq %rsi, %r15              # Swap temp_11 and @swap_temp_1
    movq $printc_format, %rdi     # Load `printc_format` as argument 0
    movq %r15, %rsi               # Load `temp_11` as argument 1
    call printf                   # Call function `printf`
    movq $0, %rsi                 # Load number '0' into `temp_12`
    movq %rsi, %rax               # Return `temp_12`

    # Epilogue
    # leave
    movq %rbp, %rsp
    pop %rbp
    ret
        