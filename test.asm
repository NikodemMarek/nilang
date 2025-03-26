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
    movq $5, %rcx                 # Load number '5' into `temp_2`
    movq %rcx, %rdi               # Load `temp_2` as argument 0
    call test                     # Call function `test`
    movq $9, %rdx                 # Load number '9' into `temp_3`
    movq $printi_format, %rdi     # Load `printi_format` as argument 0
    movq %rdx, %rsi               # Load `temp_3` as argument 1
    call printf                   # Call function `printf`
    movq $0, %rsi                 # Load number '0' into `temp_4`
    movq %rsi, %rax               # Return `temp_4`

    # Epilogue
    # leave
    movq %rbp, %rsp
    pop %rbp
    ret
        