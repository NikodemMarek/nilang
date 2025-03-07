.text
.globl _start
_start:
    call _main
    movq %rax, %rbx
    movq $1, %rax
    int $0x80
    ret

.globl _main
_main:
    pushq %rbp
    movq %rsp, %rbp
    movq $5, %rax
    movq $13, %rbx
    movq %rbx, %rcx
    movq %rcx, %rax
    leave
    ret
