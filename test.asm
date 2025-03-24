.text
.globl _start
_start:
    call _main
    movq %rax, %rbx
    movq $1, %rax
    int $0x80
    ret

.globl _test
_test:
    pushq %rbp
    movq %rsp, %rbp
    movq %r9, %rax
    movq %rax, %rbx
    movq %rbx, %rax
    leave
    ret

.globl _main
_main:
    pushq %rbp
    movq %rsp, %rbp
    movq $5, %rcx
    movq $13, %rdx
    movq $7, %rsi
    movq $9, %rdi
    movq %rcx, %r8
    movq %rdx, %r9
    movq %rdi, %r10
    movq %rsi, %r11
    movq $111, %r12
    movq %r12, %r13
    call _test
    movq %r14, %r15
    movq %rax, -8(%rbp)
    movq -8(%rbp), %rax
    leave
    ret
