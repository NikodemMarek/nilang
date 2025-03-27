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
    movq $'e', %rbx               # Load character 'e' into `temp_1`
    movq $printc_format, %rdi     # Load `printc_format` as argument 0
    movq %rbx, %rsi               # Load `temp_1` as argument 1
    call printf                   # Call function `printf`
    movq $'l', %rcx               # Load character 'l' into `temp_2`
    movq $printc_format, %rdi     # Load `printc_format` as argument 0
    movq %rcx, %rsi               # Load `temp_2` as argument 1
    call printf                   # Call function `printf`
    movq $'l', %rdx               # Load character 'l' into `temp_3`
    movq $printc_format, %rdi     # Load `printc_format` as argument 0
    movq %rdx, %rsi               # Load `temp_3` as argument 1
    call printf                   # Call function `printf`
    movq $'o', %rsi               # Load character 'o' into `temp_4`
    movq %rsi, %r8                # Move `temp_4` to a free location
    movq $printc_format, %rdi     # Load `printc_format` as argument 0
    movq %r8, %rsi                # Load `temp_4` as argument 1
    call printf                   # Call function `printf`
    movq $' ', %rdi               # Load character ' ' into `temp_5`
    movq %rdi, %r9                # Move `temp_5` to a free location
    movq $printc_format, %rdi     # Load `printc_format` as argument 0
    movq %r9, %rsi                # Load `temp_5` as argument 1
    call printf                   # Call function `printf`
    movq $'W', %rsi               # Load character 'W' into `temp_6`
    movq %rsi, %r10               # Move `temp_6` to a free location
    movq $printc_format, %rdi     # Load `printc_format` as argument 0
    movq %r10, %rsi               # Load `temp_6` as argument 1
    call printf                   # Call function `printf`
    movq $'o', %rdi               # Load character 'o' into `temp_7`
    movq %rdi, %r11               # Move `temp_7` to a free location
    movq $printc_format, %rdi     # Load `printc_format` as argument 0
    movq %r11, %rsi               # Load `temp_7` as argument 1
    call printf                   # Call function `printf`
    movq $'r', %rsi               # Load character 'r' into `temp_8`
    movq %rsi, %r12               # Move `temp_8` to a free location
    movq $printc_format, %rdi     # Load `printc_format` as argument 0
    movq %r12, %rsi               # Load `temp_8` as argument 1
    call printf                   # Call function `printf`
    movq $'l', %rdi               # Load character 'l' into `temp_9`
    movq %rdi, %r13               # Move `temp_9` to a free location
    movq $printc_format, %rdi     # Load `printc_format` as argument 0
    movq %r13, %rsi               # Load `temp_9` as argument 1
    call printf                   # Call function `printf`
    movq $'d', %rsi               # Load character 'd' into `temp_10`
    movq %rsi, %r14               # Move `temp_10` to a free location
    movq $printc_format, %rdi     # Load `printc_format` as argument 0
    movq %r14, %rsi               # Load `temp_10` as argument 1
    call printf                   # Call function `printf`
    movq $'!', %rdi               # Load character '!' into `temp_11`
    movq %rdi, %r15               # Move `temp_11` to a free location
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
        