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
    movq %rax, %rcx
    movq %rbx, %rdx
    movq $9, %rsi
    movq $7, %rdi
    movq %rsi, %r8
    movq %rdi, %r9
    movq %r8, %r10
    movq %r10, %rax
    leave
    ret
