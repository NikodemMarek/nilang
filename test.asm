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
    movq $6, %rax
    call _test
    movq %rax, %rbx
    movq %rbx, %rax
    leave
    ret

.globl _test
_test:
    pushq %rbp
    movq %rsp, %rbp
    movq %r9, %rax
    leave
    ret
