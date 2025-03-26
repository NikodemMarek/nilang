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
        
    movq $'H', %rax               # Load character 'H' into `temp_0`
    movq $printc_format, %rdi     # Load `printc_format` as argument 0
    movq %rax, %rsi               # Load `temp_0` as argument 1
    call printf                   # Call function `printf`
    movq %rax, %rbx               # Move result of `printf` to return register
    movq $'e', %rcx               # Load character 'e' into `temp_1`
    movq $printc_format, %rdi     # Load `printc_format` as argument 0
    movq %rcx, %rsi               # Load `temp_1` as argument 1
    call printf                   # Call function `printf`
    movq %rax, %rdx               # Move result of `printf` to return register
    movq $'l', %rsi               # Load character 'l' into `temp_2`
    movq %rsi, %r8                # Move `temp_2` to a free location
    movq $printc_format, %rdi     # Load `printc_format` as argument 0
    movq %r8, %rsi                # Load `temp_2` as argument 1
    call printf                   # Call function `printf`
    movq %rax, %rdi               # Move result of `printf` to return register
    movq $'l', %rsi               # Load character 'l' into `temp_3`
    movq %rdi, %r11               # Move `` to a free location
    movq %rsi, %r10               # Move `temp_3` to a free location
    movq $printc_format, %rdi     # Load `printc_format` as argument 0
    movq %r10, %rsi               # Load `temp_3` as argument 1
    call printf                   # Call function `printf`
    movq %rax, %r9                # Move result of `printf` to return register
    movq $'o', %rdi               # Load character 'o' into `temp_4`
    movq %rdi, %r12               # Move `temp_4` to a free location
    movq $printc_format, %rdi     # Load `printc_format` as argument 0
    movq %r12, %rsi               # Load `temp_4` as argument 1
    call printf                   # Call function `printf`
    movq %rax, %rsi               # Move result of `printf` to return register
    movq $0, %rdi                 # Load number '0' into `temp_5`
    movq %rdi, %rax               # Return `temp_5`

    # Epilogue
    # leave
    movq %rbp, %rsp
    pop %rbp
    ret
        