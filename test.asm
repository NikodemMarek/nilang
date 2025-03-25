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
    movq $13, %rax
    movq $5, %rbx
    movq %rbx, %rcx
    movq %rax, %rdx
    movq $7, %rsi
    movq $9, %rdi
    movq %rdi, %r8
    movq %rsi, %r9
    movq %r8, %r10
    movq %r9, %r11
    call _test
    movq %r12, %r14
    movq %r13, %r15
    movq %rax, -8(%rbp)
    movq -8(%rbp), %rax
    leave
    ret

.globl _test
_test:
    pushq %rbp
    movq %rsp, %rbp
    movq %r9, -16(%rbp)
    movq %r8, -24(%rbp)
    movq -16(%rbp), -32(%rbp)
    movq -32(%rbp), %rax
    leave
    ret
