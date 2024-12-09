.data

.text
.globl _start
_start:
pushq %rbp
movq %rsp, %rbp
    call _main
    movl $1, %eax
    int $0x80
    ret

.globl _main
_main:
pushq %rbp
movq %rsp, %rbp
    movq $1, %r15
    movq $2, %r14
    movq $3, %r13
    movq $4, %r12
    movq $5, %r11
    movq $6, %r10
    movq $7, %r9
    movq $8, %r8
    movq $9, %rdi
    movq $10, %rsi
    movq $11, %rdx
    movq $12, %rcx
    movq $13, %rbx
    movq $14, %rax
    movq $15, -8(%rbp)
    movq $16, -16(%rbp)
    movq $17, -24(%rbp)
    movq $18, -32(%rbp)
    movq $19, -40(%rbp)
    movq -40(%rbp), %rbx
    leave
    ret
