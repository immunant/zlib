#include <ia2.h>
#include <scrub_registers.h>
asm(
    /* Wrapper for indirect call(int, int) -> int: */
    ".text\n"
    ".global __ia2_indirect_callgate__ZTSPF11block_stateP14internal_stateiE_pkey_1\n"
    ".type __ia2_indirect_callgate__ZTSPF11block_stateP14internal_stateiE_pkey_1, @function\n"
    "__ia2_indirect_callgate__ZTSPF11block_stateP14internal_stateiE_pkey_1:\n"
    "pushq %rbp\n"
    "movq %rsp, %rbp\n"
    "pushq %rbx\n"
    "pushq %r12\n"
    "pushq %r13\n"
    "pushq %r14\n"
    "pushq %r15\n"
    ASSERT_PKRU(0xfffffffffffffff0) "\n"
    /* Set PKRU to the intermediate value to move arguments */
    "movq %rcx, %r10\n"
    "movq %rdx, %r11\n"
    "xorl %ecx, %ecx\n"
    "xorl %edx, %edx\n"
    "movl $0xfffffff0, %eax\n"
    "wrpkru\n"
    "movq %r10, %rcx\n"
    "movq %r11, %rdx\n"
    /* Compute location to save old stack pointer (using r11) */
    "mov ia2_stackptr_1@GOTTPOFF(%rip), %r11\n"
    /* Write the old stack pointer to memory */
    "movq %rsp, %fs:(%r11)\n"
    /* Compute location to load new stack pointer (using r11) */
    "mov ia2_stackptr_0@GOTTPOFF(%rip), %r11\n"
    /* Read the new stack pointer from memory */
    "movq %fs:(%r11), %rsp\n"
    "subq $8, %rsp\n"
    /* Copy arguments into the correct registers */
    "movq %rdi, %r12\n"
    "movq %rsi, %rdi\n"
    "movq %rdx, %rsi\n"
    /* Preserve essential regs on stack */
    "pushq %rdi\n"
    "pushq %rsi\n"
    "pushq %r12\n"
    /* Scrub non-essential regs */
    "call __libia2_scrub_registers\n"
    /* Restore preserved regs */
    "popq %r12\n"
    "popq %rsi\n"
    "popq %rdi\n"
    /* Set PKRU to the compartment's value */
    "movq %rcx, %r10\n"
    "movq %rdx, %r11\n"
    "xorl %ecx, %ecx\n"
    "xorl %edx, %edx\n"
    "movl $0xfffffffc, %eax\n"
    "wrpkru\n"
    "movq %r10, %rcx\n"
    "movq %r11, %rdx\n"
    /* Call wrapped function */
    "call *%r12\n"
    /* Set PKRU to the intermediate value to move arguments */
    "movq %rax, %r10\n"
    "movq %rdx, %r11\n"
    "xorl %ecx, %ecx\n"
    "xorl %edx, %edx\n"
    "movl $0xfffffff0, %eax\n"
    "wrpkru\n"
    "movq %r10, %rax\n"
    "movq %r11, %rdx\n"
    /* Free stack space used for stack args */
    "addq $8, %rsp\n"
    /* Compute location to save old stack pointer (using r11) */
    "mov ia2_stackptr_0@GOTTPOFF(%rip), %r11\n"
    /* Write the old stack pointer to memory */
    "movq %rsp, %fs:(%r11)\n"
    /* Compute location to load new stack pointer (using r11) */
    "mov ia2_stackptr_1@GOTTPOFF(%rip), %r11\n"
    /* Read the new stack pointer from memory */
    "movq %fs:(%r11), %rsp\n"
    /* Set PKRU to the caller's value */
    "movq %rax, %r10\n"
    "movq %rdx, %r11\n"
    "xorl %ecx, %ecx\n"
    "xorl %edx, %edx\n"
    "movl $0xfffffff0, %eax\n"
    "wrpkru\n"
    "movq %r10, %rax\n"
    "movq %r11, %rdx\n"
    "popq %r15\n"
    "popq %r14\n"
    "popq %r13\n"
    "popq %r12\n"
    "popq %rbx\n"
    "popq %rbp\n"
    /* Return to the caller */
    "ret\n"
    ".size __ia2_indirect_callgate__ZTSPF11block_stateP14internal_stateiE_pkey_1, .-__ia2_indirect_callgate__ZTSPF11block_stateP14internal_stateiE_pkey_1\n"
    ".previous\n"
);
asm(
    /* Wrapper for indirect call(int, int, int) -> int: */
    ".text\n"
    ".global __ia2_indirect_callgate__ZTSPFPvS_jjE_pkey_1\n"
    ".type __ia2_indirect_callgate__ZTSPFPvS_jjE_pkey_1, @function\n"
    "__ia2_indirect_callgate__ZTSPFPvS_jjE_pkey_1:\n"
    "pushq %rbp\n"
    "movq %rsp, %rbp\n"
    "pushq %rbx\n"
    "pushq %r12\n"
    "pushq %r13\n"
    "pushq %r14\n"
    "pushq %r15\n"
    ASSERT_PKRU(0xfffffffffffffff0) "\n"
    /* Set PKRU to the intermediate value to move arguments */
    "movq %rcx, %r10\n"
    "movq %rdx, %r11\n"
    "xorl %ecx, %ecx\n"
    "xorl %edx, %edx\n"
    "movl $0xfffffff0, %eax\n"
    "wrpkru\n"
    "movq %r10, %rcx\n"
    "movq %r11, %rdx\n"
    /* Compute location to save old stack pointer (using r11) */
    "mov ia2_stackptr_1@GOTTPOFF(%rip), %r11\n"
    /* Write the old stack pointer to memory */
    "movq %rsp, %fs:(%r11)\n"
    /* Compute location to load new stack pointer (using r11) */
    "mov ia2_stackptr_0@GOTTPOFF(%rip), %r11\n"
    /* Read the new stack pointer from memory */
    "movq %fs:(%r11), %rsp\n"
    "subq $8, %rsp\n"
    /* Copy arguments into the correct registers */
    "movq %rdi, %r12\n"
    "movq %rsi, %rdi\n"
    "movq %rdx, %rsi\n"
    "movq %rcx, %rdx\n"
    /* Preserve essential regs on stack */
    "pushq %rdi\n"
    "pushq %rsi\n"
    "pushq %rdx\n"
    "pushq %r12\n"
    /* Scrub non-essential regs */
    "call __libia2_scrub_registers\n"
    /* Restore preserved regs */
    "popq %r12\n"
    "popq %rdx\n"
    "popq %rsi\n"
    "popq %rdi\n"
    /* Set PKRU to the compartment's value */
    "movq %rcx, %r10\n"
    "movq %rdx, %r11\n"
    "xorl %ecx, %ecx\n"
    "xorl %edx, %edx\n"
    "movl $0xfffffffc, %eax\n"
    "wrpkru\n"
    "movq %r10, %rcx\n"
    "movq %r11, %rdx\n"
    /* Call wrapped function */
    "call *%r12\n"
    /* Set PKRU to the intermediate value to move arguments */
    "movq %rax, %r10\n"
    "movq %rdx, %r11\n"
    "xorl %ecx, %ecx\n"
    "xorl %edx, %edx\n"
    "movl $0xfffffff0, %eax\n"
    "wrpkru\n"
    "movq %r10, %rax\n"
    "movq %r11, %rdx\n"
    /* Free stack space used for stack args */
    "addq $8, %rsp\n"
    /* Compute location to save old stack pointer (using r11) */
    "mov ia2_stackptr_0@GOTTPOFF(%rip), %r11\n"
    /* Write the old stack pointer to memory */
    "movq %rsp, %fs:(%r11)\n"
    /* Compute location to load new stack pointer (using r11) */
    "mov ia2_stackptr_1@GOTTPOFF(%rip), %r11\n"
    /* Read the new stack pointer from memory */
    "movq %fs:(%r11), %rsp\n"
    /* Set PKRU to the caller's value */
    "movq %rax, %r10\n"
    "movq %rdx, %r11\n"
    "xorl %ecx, %ecx\n"
    "xorl %edx, %edx\n"
    "movl $0xfffffff0, %eax\n"
    "wrpkru\n"
    "movq %r10, %rax\n"
    "movq %r11, %rdx\n"
    "popq %r15\n"
    "popq %r14\n"
    "popq %r13\n"
    "popq %r12\n"
    "popq %rbx\n"
    "popq %rbp\n"
    /* Return to the caller */
    "ret\n"
    ".size __ia2_indirect_callgate__ZTSPFPvS_jjE_pkey_1, .-__ia2_indirect_callgate__ZTSPFPvS_jjE_pkey_1\n"
    ".previous\n"
);
asm(
    /* Wrapper for indirect call(int, int, int) -> int: */
    ".text\n"
    ".global __ia2_indirect_callgate__ZTSPFiPvPhjE_pkey_1\n"
    ".type __ia2_indirect_callgate__ZTSPFiPvPhjE_pkey_1, @function\n"
    "__ia2_indirect_callgate__ZTSPFiPvPhjE_pkey_1:\n"
    "pushq %rbp\n"
    "movq %rsp, %rbp\n"
    "pushq %rbx\n"
    "pushq %r12\n"
    "pushq %r13\n"
    "pushq %r14\n"
    "pushq %r15\n"
    ASSERT_PKRU(0xfffffffffffffff0) "\n"
    /* Set PKRU to the intermediate value to move arguments */
    "movq %rcx, %r10\n"
    "movq %rdx, %r11\n"
    "xorl %ecx, %ecx\n"
    "xorl %edx, %edx\n"
    "movl $0xfffffff0, %eax\n"
    "wrpkru\n"
    "movq %r10, %rcx\n"
    "movq %r11, %rdx\n"
    /* Compute location to save old stack pointer (using r11) */
    "mov ia2_stackptr_1@GOTTPOFF(%rip), %r11\n"
    /* Write the old stack pointer to memory */
    "movq %rsp, %fs:(%r11)\n"
    /* Compute location to load new stack pointer (using r11) */
    "mov ia2_stackptr_0@GOTTPOFF(%rip), %r11\n"
    /* Read the new stack pointer from memory */
    "movq %fs:(%r11), %rsp\n"
    "subq $8, %rsp\n"
    /* Copy arguments into the correct registers */
    "movq %rdi, %r12\n"
    "movq %rsi, %rdi\n"
    "movq %rdx, %rsi\n"
    "movq %rcx, %rdx\n"
    /* Preserve essential regs on stack */
    "pushq %rdi\n"
    "pushq %rsi\n"
    "pushq %rdx\n"
    "pushq %r12\n"
    /* Scrub non-essential regs */
    "call __libia2_scrub_registers\n"
    /* Restore preserved regs */
    "popq %r12\n"
    "popq %rdx\n"
    "popq %rsi\n"
    "popq %rdi\n"
    /* Set PKRU to the compartment's value */
    "movq %rcx, %r10\n"
    "movq %rdx, %r11\n"
    "xorl %ecx, %ecx\n"
    "xorl %edx, %edx\n"
    "movl $0xfffffffc, %eax\n"
    "wrpkru\n"
    "movq %r10, %rcx\n"
    "movq %r11, %rdx\n"
    /* Call wrapped function */
    "call *%r12\n"
    /* Set PKRU to the intermediate value to move arguments */
    "movq %rax, %r10\n"
    "movq %rdx, %r11\n"
    "xorl %ecx, %ecx\n"
    "xorl %edx, %edx\n"
    "movl $0xfffffff0, %eax\n"
    "wrpkru\n"
    "movq %r10, %rax\n"
    "movq %r11, %rdx\n"
    /* Free stack space used for stack args */
    "addq $8, %rsp\n"
    /* Compute location to save old stack pointer (using r11) */
    "mov ia2_stackptr_0@GOTTPOFF(%rip), %r11\n"
    /* Write the old stack pointer to memory */
    "movq %rsp, %fs:(%r11)\n"
    /* Compute location to load new stack pointer (using r11) */
    "mov ia2_stackptr_1@GOTTPOFF(%rip), %r11\n"
    /* Read the new stack pointer from memory */
    "movq %fs:(%r11), %rsp\n"
    /* Set PKRU to the caller's value */
    "movq %rax, %r10\n"
    "movq %rdx, %r11\n"
    "xorl %ecx, %ecx\n"
    "xorl %edx, %edx\n"
    "movl $0xfffffff0, %eax\n"
    "wrpkru\n"
    "movq %r10, %rax\n"
    "movq %r11, %rdx\n"
    "popq %r15\n"
    "popq %r14\n"
    "popq %r13\n"
    "popq %r12\n"
    "popq %rbx\n"
    "popq %rbp\n"
    /* Return to the caller */
    "ret\n"
    ".size __ia2_indirect_callgate__ZTSPFiPvPhjE_pkey_1, .-__ia2_indirect_callgate__ZTSPFiPvPhjE_pkey_1\n"
    ".previous\n"
);
asm(
    /* Wrapper for indirect call(int, int) -> int: */
    ".text\n"
    ".global __ia2_indirect_callgate__ZTSPFjPvPPhE_pkey_1\n"
    ".type __ia2_indirect_callgate__ZTSPFjPvPPhE_pkey_1, @function\n"
    "__ia2_indirect_callgate__ZTSPFjPvPPhE_pkey_1:\n"
    "pushq %rbp\n"
    "movq %rsp, %rbp\n"
    "pushq %rbx\n"
    "pushq %r12\n"
    "pushq %r13\n"
    "pushq %r14\n"
    "pushq %r15\n"
    ASSERT_PKRU(0xfffffffffffffff0) "\n"
    /* Set PKRU to the intermediate value to move arguments */
    "movq %rcx, %r10\n"
    "movq %rdx, %r11\n"
    "xorl %ecx, %ecx\n"
    "xorl %edx, %edx\n"
    "movl $0xfffffff0, %eax\n"
    "wrpkru\n"
    "movq %r10, %rcx\n"
    "movq %r11, %rdx\n"
    /* Compute location to save old stack pointer (using r11) */
    "mov ia2_stackptr_1@GOTTPOFF(%rip), %r11\n"
    /* Write the old stack pointer to memory */
    "movq %rsp, %fs:(%r11)\n"
    /* Compute location to load new stack pointer (using r11) */
    "mov ia2_stackptr_0@GOTTPOFF(%rip), %r11\n"
    /* Read the new stack pointer from memory */
    "movq %fs:(%r11), %rsp\n"
    "subq $8, %rsp\n"
    /* Copy arguments into the correct registers */
    "movq %rdi, %r12\n"
    "movq %rsi, %rdi\n"
    "movq %rdx, %rsi\n"
    /* Preserve essential regs on stack */
    "pushq %rdi\n"
    "pushq %rsi\n"
    "pushq %r12\n"
    /* Scrub non-essential regs */
    "call __libia2_scrub_registers\n"
    /* Restore preserved regs */
    "popq %r12\n"
    "popq %rsi\n"
    "popq %rdi\n"
    /* Set PKRU to the compartment's value */
    "movq %rcx, %r10\n"
    "movq %rdx, %r11\n"
    "xorl %ecx, %ecx\n"
    "xorl %edx, %edx\n"
    "movl $0xfffffffc, %eax\n"
    "wrpkru\n"
    "movq %r10, %rcx\n"
    "movq %r11, %rdx\n"
    /* Call wrapped function */
    "call *%r12\n"
    /* Set PKRU to the intermediate value to move arguments */
    "movq %rax, %r10\n"
    "movq %rdx, %r11\n"
    "xorl %ecx, %ecx\n"
    "xorl %edx, %edx\n"
    "movl $0xfffffff0, %eax\n"
    "wrpkru\n"
    "movq %r10, %rax\n"
    "movq %r11, %rdx\n"
    /* Free stack space used for stack args */
    "addq $8, %rsp\n"
    /* Compute location to save old stack pointer (using r11) */
    "mov ia2_stackptr_0@GOTTPOFF(%rip), %r11\n"
    /* Write the old stack pointer to memory */
    "movq %rsp, %fs:(%r11)\n"
    /* Compute location to load new stack pointer (using r11) */
    "mov ia2_stackptr_1@GOTTPOFF(%rip), %r11\n"
    /* Read the new stack pointer from memory */
    "movq %fs:(%r11), %rsp\n"
    /* Set PKRU to the caller's value */
    "movq %rax, %r10\n"
    "movq %rdx, %r11\n"
    "xorl %ecx, %ecx\n"
    "xorl %edx, %edx\n"
    "movl $0xfffffff0, %eax\n"
    "wrpkru\n"
    "movq %r10, %rax\n"
    "movq %r11, %rdx\n"
    "popq %r15\n"
    "popq %r14\n"
    "popq %r13\n"
    "popq %r12\n"
    "popq %rbx\n"
    "popq %rbp\n"
    /* Return to the caller */
    "ret\n"
    ".size __ia2_indirect_callgate__ZTSPFjPvPPhE_pkey_1, .-__ia2_indirect_callgate__ZTSPFjPvPPhE_pkey_1\n"
    ".previous\n"
);
asm(
    /* Wrapper for indirect call(int, int): */
    ".text\n"
    ".global __ia2_indirect_callgate__ZTSPFvPvS_E_pkey_1\n"
    ".type __ia2_indirect_callgate__ZTSPFvPvS_E_pkey_1, @function\n"
    "__ia2_indirect_callgate__ZTSPFvPvS_E_pkey_1:\n"
    "pushq %rbp\n"
    "movq %rsp, %rbp\n"
    "pushq %rbx\n"
    "pushq %r12\n"
    "pushq %r13\n"
    "pushq %r14\n"
    "pushq %r15\n"
    ASSERT_PKRU(0xfffffffffffffff0) "\n"
    /* Set PKRU to the intermediate value to move arguments */
    "movq %rcx, %r10\n"
    "movq %rdx, %r11\n"
    "xorl %ecx, %ecx\n"
    "xorl %edx, %edx\n"
    "movl $0xfffffff0, %eax\n"
    "wrpkru\n"
    "movq %r10, %rcx\n"
    "movq %r11, %rdx\n"
    /* Compute location to save old stack pointer (using r11) */
    "mov ia2_stackptr_1@GOTTPOFF(%rip), %r11\n"
    /* Write the old stack pointer to memory */
    "movq %rsp, %fs:(%r11)\n"
    /* Compute location to load new stack pointer (using r11) */
    "mov ia2_stackptr_0@GOTTPOFF(%rip), %r11\n"
    /* Read the new stack pointer from memory */
    "movq %fs:(%r11), %rsp\n"
    "subq $8, %rsp\n"
    /* Copy arguments into the correct registers */
    "movq %rdi, %r12\n"
    "movq %rsi, %rdi\n"
    "movq %rdx, %rsi\n"
    /* Preserve essential regs on stack */
    "pushq %rdi\n"
    "pushq %rsi\n"
    "pushq %r12\n"
    /* Scrub non-essential regs */
    "call __libia2_scrub_registers\n"
    /* Restore preserved regs */
    "popq %r12\n"
    "popq %rsi\n"
    "popq %rdi\n"
    /* Set PKRU to the compartment's value */
    "movq %rcx, %r10\n"
    "movq %rdx, %r11\n"
    "xorl %ecx, %ecx\n"
    "xorl %edx, %edx\n"
    "movl $0xfffffffc, %eax\n"
    "wrpkru\n"
    "movq %r10, %rcx\n"
    "movq %r11, %rdx\n"
    /* Call wrapped function */
    "call *%r12\n"
    /* Set PKRU to the intermediate value to move arguments */
    "movq %rax, %r10\n"
    "movq %rdx, %r11\n"
    "xorl %ecx, %ecx\n"
    "xorl %edx, %edx\n"
    "movl $0xfffffff0, %eax\n"
    "wrpkru\n"
    "movq %r10, %rax\n"
    "movq %r11, %rdx\n"
    /* Free stack space used for stack args */
    "addq $8, %rsp\n"
    /* Compute location to save old stack pointer (using r11) */
    "mov ia2_stackptr_0@GOTTPOFF(%rip), %r11\n"
    /* Write the old stack pointer to memory */
    "movq %rsp, %fs:(%r11)\n"
    /* Compute location to load new stack pointer (using r11) */
    "mov ia2_stackptr_1@GOTTPOFF(%rip), %r11\n"
    /* Read the new stack pointer from memory */
    "movq %fs:(%r11), %rsp\n"
    /* Set PKRU to the caller's value */
    "movq %rax, %r10\n"
    "movq %rdx, %r11\n"
    "xorl %ecx, %ecx\n"
    "xorl %edx, %edx\n"
    "movl $0xfffffff0, %eax\n"
    "wrpkru\n"
    "movq %r10, %rax\n"
    "movq %r11, %rdx\n"
    "popq %r15\n"
    "popq %r14\n"
    "popq %r13\n"
    "popq %r12\n"
    "popq %rbx\n"
    "popq %rbp\n"
    /* Return to the caller */
    "ret\n"
    ".size __ia2_indirect_callgate__ZTSPFvPvS_E_pkey_1, .-__ia2_indirect_callgate__ZTSPFvPvS_E_pkey_1\n"
    ".previous\n"
);
asm(
    /* Wrapper for indirect call(int, int) -> int: */
    ".text\n"
    ".global __ia2_indirect_callgate__ZTSPF11block_stateP14internal_stateiE_pkey_2\n"
    ".type __ia2_indirect_callgate__ZTSPF11block_stateP14internal_stateiE_pkey_2, @function\n"
    "__ia2_indirect_callgate__ZTSPF11block_stateP14internal_stateiE_pkey_2:\n"
    "pushq %rbp\n"
    "movq %rsp, %rbp\n"
    "pushq %rbx\n"
    "pushq %r12\n"
    "pushq %r13\n"
    "pushq %r14\n"
    "pushq %r15\n"
    ASSERT_PKRU(0xffffffffffffffcc) "\n"
    /* Set PKRU to the intermediate value to move arguments */
    "movq %rcx, %r10\n"
    "movq %rdx, %r11\n"
    "xorl %ecx, %ecx\n"
    "xorl %edx, %edx\n"
    "movl $0xffffffcc, %eax\n"
    "wrpkru\n"
    "movq %r10, %rcx\n"
    "movq %r11, %rdx\n"
    /* Compute location to save old stack pointer (using r11) */
    "mov ia2_stackptr_2@GOTTPOFF(%rip), %r11\n"
    /* Write the old stack pointer to memory */
    "movq %rsp, %fs:(%r11)\n"
    /* Compute location to load new stack pointer (using r11) */
    "mov ia2_stackptr_0@GOTTPOFF(%rip), %r11\n"
    /* Read the new stack pointer from memory */
    "movq %fs:(%r11), %rsp\n"
    "subq $8, %rsp\n"
    /* Copy arguments into the correct registers */
    "movq %rdi, %r12\n"
    "movq %rsi, %rdi\n"
    "movq %rdx, %rsi\n"
    /* Preserve essential regs on stack */
    "pushq %rdi\n"
    "pushq %rsi\n"
    "pushq %r12\n"
    /* Scrub non-essential regs */
    "call __libia2_scrub_registers\n"
    /* Restore preserved regs */
    "popq %r12\n"
    "popq %rsi\n"
    "popq %rdi\n"
    /* Set PKRU to the compartment's value */
    "movq %rcx, %r10\n"
    "movq %rdx, %r11\n"
    "xorl %ecx, %ecx\n"
    "xorl %edx, %edx\n"
    "movl $0xfffffffc, %eax\n"
    "wrpkru\n"
    "movq %r10, %rcx\n"
    "movq %r11, %rdx\n"
    /* Call wrapped function */
    "call *%r12\n"
    /* Set PKRU to the intermediate value to move arguments */
    "movq %rax, %r10\n"
    "movq %rdx, %r11\n"
    "xorl %ecx, %ecx\n"
    "xorl %edx, %edx\n"
    "movl $0xffffffcc, %eax\n"
    "wrpkru\n"
    "movq %r10, %rax\n"
    "movq %r11, %rdx\n"
    /* Free stack space used for stack args */
    "addq $8, %rsp\n"
    /* Compute location to save old stack pointer (using r11) */
    "mov ia2_stackptr_0@GOTTPOFF(%rip), %r11\n"
    /* Write the old stack pointer to memory */
    "movq %rsp, %fs:(%r11)\n"
    /* Compute location to load new stack pointer (using r11) */
    "mov ia2_stackptr_2@GOTTPOFF(%rip), %r11\n"
    /* Read the new stack pointer from memory */
    "movq %fs:(%r11), %rsp\n"
    /* Set PKRU to the caller's value */
    "movq %rax, %r10\n"
    "movq %rdx, %r11\n"
    "xorl %ecx, %ecx\n"
    "xorl %edx, %edx\n"
    "movl $0xffffffcc, %eax\n"
    "wrpkru\n"
    "movq %r10, %rax\n"
    "movq %r11, %rdx\n"
    "popq %r15\n"
    "popq %r14\n"
    "popq %r13\n"
    "popq %r12\n"
    "popq %rbx\n"
    "popq %rbp\n"
    /* Return to the caller */
    "ret\n"
    ".size __ia2_indirect_callgate__ZTSPF11block_stateP14internal_stateiE_pkey_2, .-__ia2_indirect_callgate__ZTSPF11block_stateP14internal_stateiE_pkey_2\n"
    ".previous\n"
);
asm(
    /* Wrapper for indirect call(int, int, int) -> int: */
    ".text\n"
    ".global __ia2_indirect_callgate__ZTSPFPvS_jjE_pkey_2\n"
    ".type __ia2_indirect_callgate__ZTSPFPvS_jjE_pkey_2, @function\n"
    "__ia2_indirect_callgate__ZTSPFPvS_jjE_pkey_2:\n"
    "pushq %rbp\n"
    "movq %rsp, %rbp\n"
    "pushq %rbx\n"
    "pushq %r12\n"
    "pushq %r13\n"
    "pushq %r14\n"
    "pushq %r15\n"
    ASSERT_PKRU(0xffffffffffffffcc) "\n"
    /* Set PKRU to the intermediate value to move arguments */
    "movq %rcx, %r10\n"
    "movq %rdx, %r11\n"
    "xorl %ecx, %ecx\n"
    "xorl %edx, %edx\n"
    "movl $0xffffffcc, %eax\n"
    "wrpkru\n"
    "movq %r10, %rcx\n"
    "movq %r11, %rdx\n"
    /* Compute location to save old stack pointer (using r11) */
    "mov ia2_stackptr_2@GOTTPOFF(%rip), %r11\n"
    /* Write the old stack pointer to memory */
    "movq %rsp, %fs:(%r11)\n"
    /* Compute location to load new stack pointer (using r11) */
    "mov ia2_stackptr_0@GOTTPOFF(%rip), %r11\n"
    /* Read the new stack pointer from memory */
    "movq %fs:(%r11), %rsp\n"
    "subq $8, %rsp\n"
    /* Copy arguments into the correct registers */
    "movq %rdi, %r12\n"
    "movq %rsi, %rdi\n"
    "movq %rdx, %rsi\n"
    "movq %rcx, %rdx\n"
    /* Preserve essential regs on stack */
    "pushq %rdi\n"
    "pushq %rsi\n"
    "pushq %rdx\n"
    "pushq %r12\n"
    /* Scrub non-essential regs */
    "call __libia2_scrub_registers\n"
    /* Restore preserved regs */
    "popq %r12\n"
    "popq %rdx\n"
    "popq %rsi\n"
    "popq %rdi\n"
    /* Set PKRU to the compartment's value */
    "movq %rcx, %r10\n"
    "movq %rdx, %r11\n"
    "xorl %ecx, %ecx\n"
    "xorl %edx, %edx\n"
    "movl $0xfffffffc, %eax\n"
    "wrpkru\n"
    "movq %r10, %rcx\n"
    "movq %r11, %rdx\n"
    /* Call wrapped function */
    "call *%r12\n"
    /* Set PKRU to the intermediate value to move arguments */
    "movq %rax, %r10\n"
    "movq %rdx, %r11\n"
    "xorl %ecx, %ecx\n"
    "xorl %edx, %edx\n"
    "movl $0xffffffcc, %eax\n"
    "wrpkru\n"
    "movq %r10, %rax\n"
    "movq %r11, %rdx\n"
    /* Free stack space used for stack args */
    "addq $8, %rsp\n"
    /* Compute location to save old stack pointer (using r11) */
    "mov ia2_stackptr_0@GOTTPOFF(%rip), %r11\n"
    /* Write the old stack pointer to memory */
    "movq %rsp, %fs:(%r11)\n"
    /* Compute location to load new stack pointer (using r11) */
    "mov ia2_stackptr_2@GOTTPOFF(%rip), %r11\n"
    /* Read the new stack pointer from memory */
    "movq %fs:(%r11), %rsp\n"
    /* Set PKRU to the caller's value */
    "movq %rax, %r10\n"
    "movq %rdx, %r11\n"
    "xorl %ecx, %ecx\n"
    "xorl %edx, %edx\n"
    "movl $0xffffffcc, %eax\n"
    "wrpkru\n"
    "movq %r10, %rax\n"
    "movq %r11, %rdx\n"
    "popq %r15\n"
    "popq %r14\n"
    "popq %r13\n"
    "popq %r12\n"
    "popq %rbx\n"
    "popq %rbp\n"
    /* Return to the caller */
    "ret\n"
    ".size __ia2_indirect_callgate__ZTSPFPvS_jjE_pkey_2, .-__ia2_indirect_callgate__ZTSPFPvS_jjE_pkey_2\n"
    ".previous\n"
);
asm(
    /* Wrapper for indirect call(int, int, int) -> int: */
    ".text\n"
    ".global __ia2_indirect_callgate__ZTSPFiPvPhjE_pkey_2\n"
    ".type __ia2_indirect_callgate__ZTSPFiPvPhjE_pkey_2, @function\n"
    "__ia2_indirect_callgate__ZTSPFiPvPhjE_pkey_2:\n"
    "pushq %rbp\n"
    "movq %rsp, %rbp\n"
    "pushq %rbx\n"
    "pushq %r12\n"
    "pushq %r13\n"
    "pushq %r14\n"
    "pushq %r15\n"
    ASSERT_PKRU(0xffffffffffffffcc) "\n"
    /* Set PKRU to the intermediate value to move arguments */
    "movq %rcx, %r10\n"
    "movq %rdx, %r11\n"
    "xorl %ecx, %ecx\n"
    "xorl %edx, %edx\n"
    "movl $0xffffffcc, %eax\n"
    "wrpkru\n"
    "movq %r10, %rcx\n"
    "movq %r11, %rdx\n"
    /* Compute location to save old stack pointer (using r11) */
    "mov ia2_stackptr_2@GOTTPOFF(%rip), %r11\n"
    /* Write the old stack pointer to memory */
    "movq %rsp, %fs:(%r11)\n"
    /* Compute location to load new stack pointer (using r11) */
    "mov ia2_stackptr_0@GOTTPOFF(%rip), %r11\n"
    /* Read the new stack pointer from memory */
    "movq %fs:(%r11), %rsp\n"
    "subq $8, %rsp\n"
    /* Copy arguments into the correct registers */
    "movq %rdi, %r12\n"
    "movq %rsi, %rdi\n"
    "movq %rdx, %rsi\n"
    "movq %rcx, %rdx\n"
    /* Preserve essential regs on stack */
    "pushq %rdi\n"
    "pushq %rsi\n"
    "pushq %rdx\n"
    "pushq %r12\n"
    /* Scrub non-essential regs */
    "call __libia2_scrub_registers\n"
    /* Restore preserved regs */
    "popq %r12\n"
    "popq %rdx\n"
    "popq %rsi\n"
    "popq %rdi\n"
    /* Set PKRU to the compartment's value */
    "movq %rcx, %r10\n"
    "movq %rdx, %r11\n"
    "xorl %ecx, %ecx\n"
    "xorl %edx, %edx\n"
    "movl $0xfffffffc, %eax\n"
    "wrpkru\n"
    "movq %r10, %rcx\n"
    "movq %r11, %rdx\n"
    /* Call wrapped function */
    "call *%r12\n"
    /* Set PKRU to the intermediate value to move arguments */
    "movq %rax, %r10\n"
    "movq %rdx, %r11\n"
    "xorl %ecx, %ecx\n"
    "xorl %edx, %edx\n"
    "movl $0xffffffcc, %eax\n"
    "wrpkru\n"
    "movq %r10, %rax\n"
    "movq %r11, %rdx\n"
    /* Free stack space used for stack args */
    "addq $8, %rsp\n"
    /* Compute location to save old stack pointer (using r11) */
    "mov ia2_stackptr_0@GOTTPOFF(%rip), %r11\n"
    /* Write the old stack pointer to memory */
    "movq %rsp, %fs:(%r11)\n"
    /* Compute location to load new stack pointer (using r11) */
    "mov ia2_stackptr_2@GOTTPOFF(%rip), %r11\n"
    /* Read the new stack pointer from memory */
    "movq %fs:(%r11), %rsp\n"
    /* Set PKRU to the caller's value */
    "movq %rax, %r10\n"
    "movq %rdx, %r11\n"
    "xorl %ecx, %ecx\n"
    "xorl %edx, %edx\n"
    "movl $0xffffffcc, %eax\n"
    "wrpkru\n"
    "movq %r10, %rax\n"
    "movq %r11, %rdx\n"
    "popq %r15\n"
    "popq %r14\n"
    "popq %r13\n"
    "popq %r12\n"
    "popq %rbx\n"
    "popq %rbp\n"
    /* Return to the caller */
    "ret\n"
    ".size __ia2_indirect_callgate__ZTSPFiPvPhjE_pkey_2, .-__ia2_indirect_callgate__ZTSPFiPvPhjE_pkey_2\n"
    ".previous\n"
);
asm(
    /* Wrapper for indirect call(int, int) -> int: */
    ".text\n"
    ".global __ia2_indirect_callgate__ZTSPFjPvPPhE_pkey_2\n"
    ".type __ia2_indirect_callgate__ZTSPFjPvPPhE_pkey_2, @function\n"
    "__ia2_indirect_callgate__ZTSPFjPvPPhE_pkey_2:\n"
    "pushq %rbp\n"
    "movq %rsp, %rbp\n"
    "pushq %rbx\n"
    "pushq %r12\n"
    "pushq %r13\n"
    "pushq %r14\n"
    "pushq %r15\n"
    ASSERT_PKRU(0xffffffffffffffcc) "\n"
    /* Set PKRU to the intermediate value to move arguments */
    "movq %rcx, %r10\n"
    "movq %rdx, %r11\n"
    "xorl %ecx, %ecx\n"
    "xorl %edx, %edx\n"
    "movl $0xffffffcc, %eax\n"
    "wrpkru\n"
    "movq %r10, %rcx\n"
    "movq %r11, %rdx\n"
    /* Compute location to save old stack pointer (using r11) */
    "mov ia2_stackptr_2@GOTTPOFF(%rip), %r11\n"
    /* Write the old stack pointer to memory */
    "movq %rsp, %fs:(%r11)\n"
    /* Compute location to load new stack pointer (using r11) */
    "mov ia2_stackptr_0@GOTTPOFF(%rip), %r11\n"
    /* Read the new stack pointer from memory */
    "movq %fs:(%r11), %rsp\n"
    "subq $8, %rsp\n"
    /* Copy arguments into the correct registers */
    "movq %rdi, %r12\n"
    "movq %rsi, %rdi\n"
    "movq %rdx, %rsi\n"
    /* Preserve essential regs on stack */
    "pushq %rdi\n"
    "pushq %rsi\n"
    "pushq %r12\n"
    /* Scrub non-essential regs */
    "call __libia2_scrub_registers\n"
    /* Restore preserved regs */
    "popq %r12\n"
    "popq %rsi\n"
    "popq %rdi\n"
    /* Set PKRU to the compartment's value */
    "movq %rcx, %r10\n"
    "movq %rdx, %r11\n"
    "xorl %ecx, %ecx\n"
    "xorl %edx, %edx\n"
    "movl $0xfffffffc, %eax\n"
    "wrpkru\n"
    "movq %r10, %rcx\n"
    "movq %r11, %rdx\n"
    /* Call wrapped function */
    "call *%r12\n"
    /* Set PKRU to the intermediate value to move arguments */
    "movq %rax, %r10\n"
    "movq %rdx, %r11\n"
    "xorl %ecx, %ecx\n"
    "xorl %edx, %edx\n"
    "movl $0xffffffcc, %eax\n"
    "wrpkru\n"
    "movq %r10, %rax\n"
    "movq %r11, %rdx\n"
    /* Free stack space used for stack args */
    "addq $8, %rsp\n"
    /* Compute location to save old stack pointer (using r11) */
    "mov ia2_stackptr_0@GOTTPOFF(%rip), %r11\n"
    /* Write the old stack pointer to memory */
    "movq %rsp, %fs:(%r11)\n"
    /* Compute location to load new stack pointer (using r11) */
    "mov ia2_stackptr_2@GOTTPOFF(%rip), %r11\n"
    /* Read the new stack pointer from memory */
    "movq %fs:(%r11), %rsp\n"
    /* Set PKRU to the caller's value */
    "movq %rax, %r10\n"
    "movq %rdx, %r11\n"
    "xorl %ecx, %ecx\n"
    "xorl %edx, %edx\n"
    "movl $0xffffffcc, %eax\n"
    "wrpkru\n"
    "movq %r10, %rax\n"
    "movq %r11, %rdx\n"
    "popq %r15\n"
    "popq %r14\n"
    "popq %r13\n"
    "popq %r12\n"
    "popq %rbx\n"
    "popq %rbp\n"
    /* Return to the caller */
    "ret\n"
    ".size __ia2_indirect_callgate__ZTSPFjPvPPhE_pkey_2, .-__ia2_indirect_callgate__ZTSPFjPvPPhE_pkey_2\n"
    ".previous\n"
);
asm(
    /* Wrapper for indirect call(int, int): */
    ".text\n"
    ".global __ia2_indirect_callgate__ZTSPFvPvS_E_pkey_2\n"
    ".type __ia2_indirect_callgate__ZTSPFvPvS_E_pkey_2, @function\n"
    "__ia2_indirect_callgate__ZTSPFvPvS_E_pkey_2:\n"
    "pushq %rbp\n"
    "movq %rsp, %rbp\n"
    "pushq %rbx\n"
    "pushq %r12\n"
    "pushq %r13\n"
    "pushq %r14\n"
    "pushq %r15\n"
    ASSERT_PKRU(0xffffffffffffffcc) "\n"
    /* Set PKRU to the intermediate value to move arguments */
    "movq %rcx, %r10\n"
    "movq %rdx, %r11\n"
    "xorl %ecx, %ecx\n"
    "xorl %edx, %edx\n"
    "movl $0xffffffcc, %eax\n"
    "wrpkru\n"
    "movq %r10, %rcx\n"
    "movq %r11, %rdx\n"
    /* Compute location to save old stack pointer (using r11) */
    "mov ia2_stackptr_2@GOTTPOFF(%rip), %r11\n"
    /* Write the old stack pointer to memory */
    "movq %rsp, %fs:(%r11)\n"
    /* Compute location to load new stack pointer (using r11) */
    "mov ia2_stackptr_0@GOTTPOFF(%rip), %r11\n"
    /* Read the new stack pointer from memory */
    "movq %fs:(%r11), %rsp\n"
    "subq $8, %rsp\n"
    /* Copy arguments into the correct registers */
    "movq %rdi, %r12\n"
    "movq %rsi, %rdi\n"
    "movq %rdx, %rsi\n"
    /* Preserve essential regs on stack */
    "pushq %rdi\n"
    "pushq %rsi\n"
    "pushq %r12\n"
    /* Scrub non-essential regs */
    "call __libia2_scrub_registers\n"
    /* Restore preserved regs */
    "popq %r12\n"
    "popq %rsi\n"
    "popq %rdi\n"
    /* Set PKRU to the compartment's value */
    "movq %rcx, %r10\n"
    "movq %rdx, %r11\n"
    "xorl %ecx, %ecx\n"
    "xorl %edx, %edx\n"
    "movl $0xfffffffc, %eax\n"
    "wrpkru\n"
    "movq %r10, %rcx\n"
    "movq %r11, %rdx\n"
    /* Call wrapped function */
    "call *%r12\n"
    /* Set PKRU to the intermediate value to move arguments */
    "movq %rax, %r10\n"
    "movq %rdx, %r11\n"
    "xorl %ecx, %ecx\n"
    "xorl %edx, %edx\n"
    "movl $0xffffffcc, %eax\n"
    "wrpkru\n"
    "movq %r10, %rax\n"
    "movq %r11, %rdx\n"
    /* Free stack space used for stack args */
    "addq $8, %rsp\n"
    /* Compute location to save old stack pointer (using r11) */
    "mov ia2_stackptr_0@GOTTPOFF(%rip), %r11\n"
    /* Write the old stack pointer to memory */
    "movq %rsp, %fs:(%r11)\n"
    /* Compute location to load new stack pointer (using r11) */
    "mov ia2_stackptr_2@GOTTPOFF(%rip), %r11\n"
    /* Read the new stack pointer from memory */
    "movq %fs:(%r11), %rsp\n"
    /* Set PKRU to the caller's value */
    "movq %rax, %r10\n"
    "movq %rdx, %r11\n"
    "xorl %ecx, %ecx\n"
    "xorl %edx, %edx\n"
    "movl $0xffffffcc, %eax\n"
    "wrpkru\n"
    "movq %r10, %rax\n"
    "movq %r11, %rdx\n"
    "popq %r15\n"
    "popq %r14\n"
    "popq %r13\n"
    "popq %r12\n"
    "popq %rbx\n"
    "popq %rbp\n"
    /* Return to the caller */
    "ret\n"
    ".size __ia2_indirect_callgate__ZTSPFvPvS_E_pkey_2, .-__ia2_indirect_callgate__ZTSPFvPvS_E_pkey_2\n"
    ".previous\n"
);
asm(
    /* Wrapper for adler32(int, int, int) -> int: */
    ".text\n"
    ".global __wrap_adler32\n"
    ".type __wrap_adler32, @function\n"
    "__wrap_adler32:\n"
    "pushq %rbp\n"
    "movq %rsp, %rbp\n"
    "pushq %rbx\n"
    "pushq %r12\n"
    "pushq %r13\n"
    "pushq %r14\n"
    "pushq %r15\n"
    ASSERT_PKRU(0xfffffffffffffff0) "\n"
    /* Set PKRU to the intermediate value to move arguments */
    "movq %rcx, %r10\n"
    "movq %rdx, %r11\n"
    "xorl %ecx, %ecx\n"
    "xorl %edx, %edx\n"
    "movl $0xffffffc0, %eax\n"
    "wrpkru\n"
    "movq %r10, %rcx\n"
    "movq %r11, %rdx\n"
    /* Compute location to save old stack pointer (using r11) */
    "mov ia2_stackptr_1@GOTTPOFF(%rip), %r11\n"
    /* Write the old stack pointer to memory */
    "movq %rsp, %fs:(%r11)\n"
    /* Compute location to load new stack pointer (using r11) */
    "mov ia2_stackptr_2@GOTTPOFF(%rip), %r11\n"
    /* Read the new stack pointer from memory */
    "movq %fs:(%r11), %rsp\n"
    "subq $8, %rsp\n"
    /* Preserve essential regs on stack */
    "pushq %rdi\n"
    "pushq %rsi\n"
    "pushq %rdx\n"
    /* Scrub non-essential regs */
    "call __libia2_scrub_registers\n"
    /* Restore preserved regs */
    "popq %rdx\n"
    "popq %rsi\n"
    "popq %rdi\n"
    /* Set PKRU to the compartment's value */
    "movq %rcx, %r10\n"
    "movq %rdx, %r11\n"
    "xorl %ecx, %ecx\n"
    "xorl %edx, %edx\n"
    "movl $0xffffffcc, %eax\n"
    "wrpkru\n"
    "movq %r10, %rcx\n"
    "movq %r11, %rdx\n"
    /* Call wrapped function */
    "call adler32\n"
    /* Set PKRU to the intermediate value to move arguments */
    "movq %rax, %r10\n"
    "movq %rdx, %r11\n"
    "xorl %ecx, %ecx\n"
    "xorl %edx, %edx\n"
    "movl $0xffffffc0, %eax\n"
    "wrpkru\n"
    "movq %r10, %rax\n"
    "movq %r11, %rdx\n"
    /* Free stack space used for stack args */
    "addq $8, %rsp\n"
    /* Compute location to save old stack pointer (using r11) */
    "mov ia2_stackptr_2@GOTTPOFF(%rip), %r11\n"
    /* Write the old stack pointer to memory */
    "movq %rsp, %fs:(%r11)\n"
    /* Compute location to load new stack pointer (using r11) */
    "mov ia2_stackptr_1@GOTTPOFF(%rip), %r11\n"
    /* Read the new stack pointer from memory */
    "movq %fs:(%r11), %rsp\n"
    /* Preserve essential regs on stack */
    "pushq %rax\n"
    /* Scrub non-essential regs */
    "call __libia2_scrub_registers\n"
    /* Restore preserved regs */
    "popq %rax\n"
    /* Set PKRU to the caller's value */
    "movq %rax, %r10\n"
    "movq %rdx, %r11\n"
    "xorl %ecx, %ecx\n"
    "xorl %edx, %edx\n"
    "movl $0xfffffff0, %eax\n"
    "wrpkru\n"
    "movq %r10, %rax\n"
    "movq %r11, %rdx\n"
    "popq %r15\n"
    "popq %r14\n"
    "popq %r13\n"
    "popq %r12\n"
    "popq %rbx\n"
    "popq %rbp\n"
    /* Return to the caller */
    "ret\n"
    ".size __wrap_adler32, .-__wrap_adler32\n"
    ".previous\n"
);
asm(
    /* Wrapper for adler32_combine(int, int, int) -> int: */
    ".text\n"
    ".global __wrap_adler32_combine\n"
    ".type __wrap_adler32_combine, @function\n"
    "__wrap_adler32_combine:\n"
    "pushq %rbp\n"
    "movq %rsp, %rbp\n"
    "pushq %rbx\n"
    "pushq %r12\n"
    "pushq %r13\n"
    "pushq %r14\n"
    "pushq %r15\n"
    ASSERT_PKRU(0xfffffffffffffff0) "\n"
    /* Set PKRU to the intermediate value to move arguments */
    "movq %rcx, %r10\n"
    "movq %rdx, %r11\n"
    "xorl %ecx, %ecx\n"
    "xorl %edx, %edx\n"
    "movl $0xffffffc0, %eax\n"
    "wrpkru\n"
    "movq %r10, %rcx\n"
    "movq %r11, %rdx\n"
    /* Compute location to save old stack pointer (using r11) */
    "mov ia2_stackptr_1@GOTTPOFF(%rip), %r11\n"
    /* Write the old stack pointer to memory */
    "movq %rsp, %fs:(%r11)\n"
    /* Compute location to load new stack pointer (using r11) */
    "mov ia2_stackptr_2@GOTTPOFF(%rip), %r11\n"
    /* Read the new stack pointer from memory */
    "movq %fs:(%r11), %rsp\n"
    "subq $8, %rsp\n"
    /* Preserve essential regs on stack */
    "pushq %rdi\n"
    "pushq %rsi\n"
    "pushq %rdx\n"
    /* Scrub non-essential regs */
    "call __libia2_scrub_registers\n"
    /* Restore preserved regs */
    "popq %rdx\n"
    "popq %rsi\n"
    "popq %rdi\n"
    /* Set PKRU to the compartment's value */
    "movq %rcx, %r10\n"
    "movq %rdx, %r11\n"
    "xorl %ecx, %ecx\n"
    "xorl %edx, %edx\n"
    "movl $0xffffffcc, %eax\n"
    "wrpkru\n"
    "movq %r10, %rcx\n"
    "movq %r11, %rdx\n"
    /* Call wrapped function */
    "call adler32_combine\n"
    /* Set PKRU to the intermediate value to move arguments */
    "movq %rax, %r10\n"
    "movq %rdx, %r11\n"
    "xorl %ecx, %ecx\n"
    "xorl %edx, %edx\n"
    "movl $0xffffffc0, %eax\n"
    "wrpkru\n"
    "movq %r10, %rax\n"
    "movq %r11, %rdx\n"
    /* Free stack space used for stack args */
    "addq $8, %rsp\n"
    /* Compute location to save old stack pointer (using r11) */
    "mov ia2_stackptr_2@GOTTPOFF(%rip), %r11\n"
    /* Write the old stack pointer to memory */
    "movq %rsp, %fs:(%r11)\n"
    /* Compute location to load new stack pointer (using r11) */
    "mov ia2_stackptr_1@GOTTPOFF(%rip), %r11\n"
    /* Read the new stack pointer from memory */
    "movq %fs:(%r11), %rsp\n"
    /* Preserve essential regs on stack */
    "pushq %rax\n"
    /* Scrub non-essential regs */
    "call __libia2_scrub_registers\n"
    /* Restore preserved regs */
    "popq %rax\n"
    /* Set PKRU to the caller's value */
    "movq %rax, %r10\n"
    "movq %rdx, %r11\n"
    "xorl %ecx, %ecx\n"
    "xorl %edx, %edx\n"
    "movl $0xfffffff0, %eax\n"
    "wrpkru\n"
    "movq %r10, %rax\n"
    "movq %r11, %rdx\n"
    "popq %r15\n"
    "popq %r14\n"
    "popq %r13\n"
    "popq %r12\n"
    "popq %rbx\n"
    "popq %rbp\n"
    /* Return to the caller */
    "ret\n"
    ".size __wrap_adler32_combine, .-__wrap_adler32_combine\n"
    ".previous\n"
);
asm(
    /* Wrapper for adler32_combine64(int, int, int) -> int: */
    ".text\n"
    ".global __wrap_adler32_combine64\n"
    ".type __wrap_adler32_combine64, @function\n"
    "__wrap_adler32_combine64:\n"
    "pushq %rbp\n"
    "movq %rsp, %rbp\n"
    "pushq %rbx\n"
    "pushq %r12\n"
    "pushq %r13\n"
    "pushq %r14\n"
    "pushq %r15\n"
    ASSERT_PKRU(0xfffffffffffffff0) "\n"
    /* Set PKRU to the intermediate value to move arguments */
    "movq %rcx, %r10\n"
    "movq %rdx, %r11\n"
    "xorl %ecx, %ecx\n"
    "xorl %edx, %edx\n"
    "movl $0xffffffc0, %eax\n"
    "wrpkru\n"
    "movq %r10, %rcx\n"
    "movq %r11, %rdx\n"
    /* Compute location to save old stack pointer (using r11) */
    "mov ia2_stackptr_1@GOTTPOFF(%rip), %r11\n"
    /* Write the old stack pointer to memory */
    "movq %rsp, %fs:(%r11)\n"
    /* Compute location to load new stack pointer (using r11) */
    "mov ia2_stackptr_2@GOTTPOFF(%rip), %r11\n"
    /* Read the new stack pointer from memory */
    "movq %fs:(%r11), %rsp\n"
    "subq $8, %rsp\n"
    /* Preserve essential regs on stack */
    "pushq %rdi\n"
    "pushq %rsi\n"
    "pushq %rdx\n"
    /* Scrub non-essential regs */
    "call __libia2_scrub_registers\n"
    /* Restore preserved regs */
    "popq %rdx\n"
    "popq %rsi\n"
    "popq %rdi\n"
    /* Set PKRU to the compartment's value */
    "movq %rcx, %r10\n"
    "movq %rdx, %r11\n"
    "xorl %ecx, %ecx\n"
    "xorl %edx, %edx\n"
    "movl $0xffffffcc, %eax\n"
    "wrpkru\n"
    "movq %r10, %rcx\n"
    "movq %r11, %rdx\n"
    /* Call wrapped function */
    "call adler32_combine64\n"
    /* Set PKRU to the intermediate value to move arguments */
    "movq %rax, %r10\n"
    "movq %rdx, %r11\n"
    "xorl %ecx, %ecx\n"
    "xorl %edx, %edx\n"
    "movl $0xffffffc0, %eax\n"
    "wrpkru\n"
    "movq %r10, %rax\n"
    "movq %r11, %rdx\n"
    /* Free stack space used for stack args */
    "addq $8, %rsp\n"
    /* Compute location to save old stack pointer (using r11) */
    "mov ia2_stackptr_2@GOTTPOFF(%rip), %r11\n"
    /* Write the old stack pointer to memory */
    "movq %rsp, %fs:(%r11)\n"
    /* Compute location to load new stack pointer (using r11) */
    "mov ia2_stackptr_1@GOTTPOFF(%rip), %r11\n"
    /* Read the new stack pointer from memory */
    "movq %fs:(%r11), %rsp\n"
    /* Preserve essential regs on stack */
    "pushq %rax\n"
    /* Scrub non-essential regs */
    "call __libia2_scrub_registers\n"
    /* Restore preserved regs */
    "popq %rax\n"
    /* Set PKRU to the caller's value */
    "movq %rax, %r10\n"
    "movq %rdx, %r11\n"
    "xorl %ecx, %ecx\n"
    "xorl %edx, %edx\n"
    "movl $0xfffffff0, %eax\n"
    "wrpkru\n"
    "movq %r10, %rax\n"
    "movq %r11, %rdx\n"
    "popq %r15\n"
    "popq %r14\n"
    "popq %r13\n"
    "popq %r12\n"
    "popq %rbx\n"
    "popq %rbp\n"
    /* Return to the caller */
    "ret\n"
    ".size __wrap_adler32_combine64, .-__wrap_adler32_combine64\n"
    ".previous\n"
);
asm(
    /* Wrapper for adler32_z(int, int, int) -> int: */
    ".text\n"
    ".global __wrap_adler32_z\n"
    ".type __wrap_adler32_z, @function\n"
    "__wrap_adler32_z:\n"
    "pushq %rbp\n"
    "movq %rsp, %rbp\n"
    "pushq %rbx\n"
    "pushq %r12\n"
    "pushq %r13\n"
    "pushq %r14\n"
    "pushq %r15\n"
    ASSERT_PKRU(0xfffffffffffffff0) "\n"
    /* Set PKRU to the intermediate value to move arguments */
    "movq %rcx, %r10\n"
    "movq %rdx, %r11\n"
    "xorl %ecx, %ecx\n"
    "xorl %edx, %edx\n"
    "movl $0xffffffc0, %eax\n"
    "wrpkru\n"
    "movq %r10, %rcx\n"
    "movq %r11, %rdx\n"
    /* Compute location to save old stack pointer (using r11) */
    "mov ia2_stackptr_1@GOTTPOFF(%rip), %r11\n"
    /* Write the old stack pointer to memory */
    "movq %rsp, %fs:(%r11)\n"
    /* Compute location to load new stack pointer (using r11) */
    "mov ia2_stackptr_2@GOTTPOFF(%rip), %r11\n"
    /* Read the new stack pointer from memory */
    "movq %fs:(%r11), %rsp\n"
    "subq $8, %rsp\n"
    /* Preserve essential regs on stack */
    "pushq %rdi\n"
    "pushq %rsi\n"
    "pushq %rdx\n"
    /* Scrub non-essential regs */
    "call __libia2_scrub_registers\n"
    /* Restore preserved regs */
    "popq %rdx\n"
    "popq %rsi\n"
    "popq %rdi\n"
    /* Set PKRU to the compartment's value */
    "movq %rcx, %r10\n"
    "movq %rdx, %r11\n"
    "xorl %ecx, %ecx\n"
    "xorl %edx, %edx\n"
    "movl $0xffffffcc, %eax\n"
    "wrpkru\n"
    "movq %r10, %rcx\n"
    "movq %r11, %rdx\n"
    /* Call wrapped function */
    "call adler32_z\n"
    /* Set PKRU to the intermediate value to move arguments */
    "movq %rax, %r10\n"
    "movq %rdx, %r11\n"
    "xorl %ecx, %ecx\n"
    "xorl %edx, %edx\n"
    "movl $0xffffffc0, %eax\n"
    "wrpkru\n"
    "movq %r10, %rax\n"
    "movq %r11, %rdx\n"
    /* Free stack space used for stack args */
    "addq $8, %rsp\n"
    /* Compute location to save old stack pointer (using r11) */
    "mov ia2_stackptr_2@GOTTPOFF(%rip), %r11\n"
    /* Write the old stack pointer to memory */
    "movq %rsp, %fs:(%r11)\n"
    /* Compute location to load new stack pointer (using r11) */
    "mov ia2_stackptr_1@GOTTPOFF(%rip), %r11\n"
    /* Read the new stack pointer from memory */
    "movq %fs:(%r11), %rsp\n"
    /* Preserve essential regs on stack */
    "pushq %rax\n"
    /* Scrub non-essential regs */
    "call __libia2_scrub_registers\n"
    /* Restore preserved regs */
    "popq %rax\n"
    /* Set PKRU to the caller's value */
    "movq %rax, %r10\n"
    "movq %rdx, %r11\n"
    "xorl %ecx, %ecx\n"
    "xorl %edx, %edx\n"
    "movl $0xfffffff0, %eax\n"
    "wrpkru\n"
    "movq %r10, %rax\n"
    "movq %r11, %rdx\n"
    "popq %r15\n"
    "popq %r14\n"
    "popq %r13\n"
    "popq %r12\n"
    "popq %rbx\n"
    "popq %rbp\n"
    /* Return to the caller */
    "ret\n"
    ".size __wrap_adler32_z, .-__wrap_adler32_z\n"
    ".previous\n"
);
asm(
    /* Wrapper for compress(int, int, int, int) -> int: */
    ".text\n"
    ".global __wrap_compress\n"
    ".type __wrap_compress, @function\n"
    "__wrap_compress:\n"
    "pushq %rbp\n"
    "movq %rsp, %rbp\n"
    "pushq %rbx\n"
    "pushq %r12\n"
    "pushq %r13\n"
    "pushq %r14\n"
    "pushq %r15\n"
    ASSERT_PKRU(0xfffffffffffffff0) "\n"
    /* Set PKRU to the intermediate value to move arguments */
    "movq %rcx, %r10\n"
    "movq %rdx, %r11\n"
    "xorl %ecx, %ecx\n"
    "xorl %edx, %edx\n"
    "movl $0xffffffc0, %eax\n"
    "wrpkru\n"
    "movq %r10, %rcx\n"
    "movq %r11, %rdx\n"
    /* Compute location to save old stack pointer (using r11) */
    "mov ia2_stackptr_1@GOTTPOFF(%rip), %r11\n"
    /* Write the old stack pointer to memory */
    "movq %rsp, %fs:(%r11)\n"
    /* Compute location to load new stack pointer (using r11) */
    "mov ia2_stackptr_2@GOTTPOFF(%rip), %r11\n"
    /* Read the new stack pointer from memory */
    "movq %fs:(%r11), %rsp\n"
    "subq $8, %rsp\n"
    /* Preserve essential regs on stack */
    "pushq %rdi\n"
    "pushq %rsi\n"
    "pushq %rdx\n"
    "pushq %rcx\n"
    /* Scrub non-essential regs */
    "call __libia2_scrub_registers\n"
    /* Restore preserved regs */
    "popq %rcx\n"
    "popq %rdx\n"
    "popq %rsi\n"
    "popq %rdi\n"
    /* Set PKRU to the compartment's value */
    "movq %rcx, %r10\n"
    "movq %rdx, %r11\n"
    "xorl %ecx, %ecx\n"
    "xorl %edx, %edx\n"
    "movl $0xffffffcc, %eax\n"
    "wrpkru\n"
    "movq %r10, %rcx\n"
    "movq %r11, %rdx\n"
    /* Call wrapped function */
    "call compress\n"
    /* Set PKRU to the intermediate value to move arguments */
    "movq %rax, %r10\n"
    "movq %rdx, %r11\n"
    "xorl %ecx, %ecx\n"
    "xorl %edx, %edx\n"
    "movl $0xffffffc0, %eax\n"
    "wrpkru\n"
    "movq %r10, %rax\n"
    "movq %r11, %rdx\n"
    /* Free stack space used for stack args */
    "addq $8, %rsp\n"
    /* Compute location to save old stack pointer (using r11) */
    "mov ia2_stackptr_2@GOTTPOFF(%rip), %r11\n"
    /* Write the old stack pointer to memory */
    "movq %rsp, %fs:(%r11)\n"
    /* Compute location to load new stack pointer (using r11) */
    "mov ia2_stackptr_1@GOTTPOFF(%rip), %r11\n"
    /* Read the new stack pointer from memory */
    "movq %fs:(%r11), %rsp\n"
    /* Preserve essential regs on stack */
    "pushq %rax\n"
    /* Scrub non-essential regs */
    "call __libia2_scrub_registers\n"
    /* Restore preserved regs */
    "popq %rax\n"
    /* Set PKRU to the caller's value */
    "movq %rax, %r10\n"
    "movq %rdx, %r11\n"
    "xorl %ecx, %ecx\n"
    "xorl %edx, %edx\n"
    "movl $0xfffffff0, %eax\n"
    "wrpkru\n"
    "movq %r10, %rax\n"
    "movq %r11, %rdx\n"
    "popq %r15\n"
    "popq %r14\n"
    "popq %r13\n"
    "popq %r12\n"
    "popq %rbx\n"
    "popq %rbp\n"
    /* Return to the caller */
    "ret\n"
    ".size __wrap_compress, .-__wrap_compress\n"
    ".previous\n"
);
asm(
    /* Wrapper for compress2(int, int, int, int, int) -> int: */
    ".text\n"
    ".global __wrap_compress2\n"
    ".type __wrap_compress2, @function\n"
    "__wrap_compress2:\n"
    "pushq %rbp\n"
    "movq %rsp, %rbp\n"
    "pushq %rbx\n"
    "pushq %r12\n"
    "pushq %r13\n"
    "pushq %r14\n"
    "pushq %r15\n"
    ASSERT_PKRU(0xfffffffffffffff0) "\n"
    /* Set PKRU to the intermediate value to move arguments */
    "movq %rcx, %r10\n"
    "movq %rdx, %r11\n"
    "xorl %ecx, %ecx\n"
    "xorl %edx, %edx\n"
    "movl $0xffffffc0, %eax\n"
    "wrpkru\n"
    "movq %r10, %rcx\n"
    "movq %r11, %rdx\n"
    /* Compute location to save old stack pointer (using r11) */
    "mov ia2_stackptr_1@GOTTPOFF(%rip), %r11\n"
    /* Write the old stack pointer to memory */
    "movq %rsp, %fs:(%r11)\n"
    /* Compute location to load new stack pointer (using r11) */
    "mov ia2_stackptr_2@GOTTPOFF(%rip), %r11\n"
    /* Read the new stack pointer from memory */
    "movq %fs:(%r11), %rsp\n"
    "subq $8, %rsp\n"
    /* Preserve essential regs on stack */
    "pushq %rdi\n"
    "pushq %rsi\n"
    "pushq %rdx\n"
    "pushq %rcx\n"
    "pushq %r8\n"
    /* Scrub non-essential regs */
    "call __libia2_scrub_registers\n"
    /* Restore preserved regs */
    "popq %r8\n"
    "popq %rcx\n"
    "popq %rdx\n"
    "popq %rsi\n"
    "popq %rdi\n"
    /* Set PKRU to the compartment's value */
    "movq %rcx, %r10\n"
    "movq %rdx, %r11\n"
    "xorl %ecx, %ecx\n"
    "xorl %edx, %edx\n"
    "movl $0xffffffcc, %eax\n"
    "wrpkru\n"
    "movq %r10, %rcx\n"
    "movq %r11, %rdx\n"
    /* Call wrapped function */
    "call compress2\n"
    /* Set PKRU to the intermediate value to move arguments */
    "movq %rax, %r10\n"
    "movq %rdx, %r11\n"
    "xorl %ecx, %ecx\n"
    "xorl %edx, %edx\n"
    "movl $0xffffffc0, %eax\n"
    "wrpkru\n"
    "movq %r10, %rax\n"
    "movq %r11, %rdx\n"
    /* Free stack space used for stack args */
    "addq $8, %rsp\n"
    /* Compute location to save old stack pointer (using r11) */
    "mov ia2_stackptr_2@GOTTPOFF(%rip), %r11\n"
    /* Write the old stack pointer to memory */
    "movq %rsp, %fs:(%r11)\n"
    /* Compute location to load new stack pointer (using r11) */
    "mov ia2_stackptr_1@GOTTPOFF(%rip), %r11\n"
    /* Read the new stack pointer from memory */
    "movq %fs:(%r11), %rsp\n"
    /* Preserve essential regs on stack */
    "pushq %rax\n"
    /* Scrub non-essential regs */
    "call __libia2_scrub_registers\n"
    /* Restore preserved regs */
    "popq %rax\n"
    /* Set PKRU to the caller's value */
    "movq %rax, %r10\n"
    "movq %rdx, %r11\n"
    "xorl %ecx, %ecx\n"
    "xorl %edx, %edx\n"
    "movl $0xfffffff0, %eax\n"
    "wrpkru\n"
    "movq %r10, %rax\n"
    "movq %r11, %rdx\n"
    "popq %r15\n"
    "popq %r14\n"
    "popq %r13\n"
    "popq %r12\n"
    "popq %rbx\n"
    "popq %rbp\n"
    /* Return to the caller */
    "ret\n"
    ".size __wrap_compress2, .-__wrap_compress2\n"
    ".previous\n"
);
asm(
    /* Wrapper for compressBound(int) -> int: */
    ".text\n"
    ".global __wrap_compressBound\n"
    ".type __wrap_compressBound, @function\n"
    "__wrap_compressBound:\n"
    "pushq %rbp\n"
    "movq %rsp, %rbp\n"
    "pushq %rbx\n"
    "pushq %r12\n"
    "pushq %r13\n"
    "pushq %r14\n"
    "pushq %r15\n"
    ASSERT_PKRU(0xfffffffffffffff0) "\n"
    /* Set PKRU to the intermediate value to move arguments */
    "movq %rcx, %r10\n"
    "movq %rdx, %r11\n"
    "xorl %ecx, %ecx\n"
    "xorl %edx, %edx\n"
    "movl $0xffffffc0, %eax\n"
    "wrpkru\n"
    "movq %r10, %rcx\n"
    "movq %r11, %rdx\n"
    /* Compute location to save old stack pointer (using r11) */
    "mov ia2_stackptr_1@GOTTPOFF(%rip), %r11\n"
    /* Write the old stack pointer to memory */
    "movq %rsp, %fs:(%r11)\n"
    /* Compute location to load new stack pointer (using r11) */
    "mov ia2_stackptr_2@GOTTPOFF(%rip), %r11\n"
    /* Read the new stack pointer from memory */
    "movq %fs:(%r11), %rsp\n"
    "subq $8, %rsp\n"
    /* Preserve essential regs on stack */
    "pushq %rdi\n"
    /* Scrub non-essential regs */
    "call __libia2_scrub_registers\n"
    /* Restore preserved regs */
    "popq %rdi\n"
    /* Set PKRU to the compartment's value */
    "movq %rcx, %r10\n"
    "movq %rdx, %r11\n"
    "xorl %ecx, %ecx\n"
    "xorl %edx, %edx\n"
    "movl $0xffffffcc, %eax\n"
    "wrpkru\n"
    "movq %r10, %rcx\n"
    "movq %r11, %rdx\n"
    /* Call wrapped function */
    "call compressBound\n"
    /* Set PKRU to the intermediate value to move arguments */
    "movq %rax, %r10\n"
    "movq %rdx, %r11\n"
    "xorl %ecx, %ecx\n"
    "xorl %edx, %edx\n"
    "movl $0xffffffc0, %eax\n"
    "wrpkru\n"
    "movq %r10, %rax\n"
    "movq %r11, %rdx\n"
    /* Free stack space used for stack args */
    "addq $8, %rsp\n"
    /* Compute location to save old stack pointer (using r11) */
    "mov ia2_stackptr_2@GOTTPOFF(%rip), %r11\n"
    /* Write the old stack pointer to memory */
    "movq %rsp, %fs:(%r11)\n"
    /* Compute location to load new stack pointer (using r11) */
    "mov ia2_stackptr_1@GOTTPOFF(%rip), %r11\n"
    /* Read the new stack pointer from memory */
    "movq %fs:(%r11), %rsp\n"
    /* Preserve essential regs on stack */
    "pushq %rax\n"
    /* Scrub non-essential regs */
    "call __libia2_scrub_registers\n"
    /* Restore preserved regs */
    "popq %rax\n"
    /* Set PKRU to the caller's value */
    "movq %rax, %r10\n"
    "movq %rdx, %r11\n"
    "xorl %ecx, %ecx\n"
    "xorl %edx, %edx\n"
    "movl $0xfffffff0, %eax\n"
    "wrpkru\n"
    "movq %r10, %rax\n"
    "movq %r11, %rdx\n"
    "popq %r15\n"
    "popq %r14\n"
    "popq %r13\n"
    "popq %r12\n"
    "popq %rbx\n"
    "popq %rbp\n"
    /* Return to the caller */
    "ret\n"
    ".size __wrap_compressBound, .-__wrap_compressBound\n"
    ".previous\n"
);
asm(
    /* Wrapper for crc32(int, int, int) -> int: */
    ".text\n"
    ".global __wrap_crc32\n"
    ".type __wrap_crc32, @function\n"
    "__wrap_crc32:\n"
    "pushq %rbp\n"
    "movq %rsp, %rbp\n"
    "pushq %rbx\n"
    "pushq %r12\n"
    "pushq %r13\n"
    "pushq %r14\n"
    "pushq %r15\n"
    ASSERT_PKRU(0xfffffffffffffff0) "\n"
    /* Set PKRU to the intermediate value to move arguments */
    "movq %rcx, %r10\n"
    "movq %rdx, %r11\n"
    "xorl %ecx, %ecx\n"
    "xorl %edx, %edx\n"
    "movl $0xffffffc0, %eax\n"
    "wrpkru\n"
    "movq %r10, %rcx\n"
    "movq %r11, %rdx\n"
    /* Compute location to save old stack pointer (using r11) */
    "mov ia2_stackptr_1@GOTTPOFF(%rip), %r11\n"
    /* Write the old stack pointer to memory */
    "movq %rsp, %fs:(%r11)\n"
    /* Compute location to load new stack pointer (using r11) */
    "mov ia2_stackptr_2@GOTTPOFF(%rip), %r11\n"
    /* Read the new stack pointer from memory */
    "movq %fs:(%r11), %rsp\n"
    "subq $8, %rsp\n"
    /* Preserve essential regs on stack */
    "pushq %rdi\n"
    "pushq %rsi\n"
    "pushq %rdx\n"
    /* Scrub non-essential regs */
    "call __libia2_scrub_registers\n"
    /* Restore preserved regs */
    "popq %rdx\n"
    "popq %rsi\n"
    "popq %rdi\n"
    /* Set PKRU to the compartment's value */
    "movq %rcx, %r10\n"
    "movq %rdx, %r11\n"
    "xorl %ecx, %ecx\n"
    "xorl %edx, %edx\n"
    "movl $0xffffffcc, %eax\n"
    "wrpkru\n"
    "movq %r10, %rcx\n"
    "movq %r11, %rdx\n"
    /* Call wrapped function */
    "call crc32\n"
    /* Set PKRU to the intermediate value to move arguments */
    "movq %rax, %r10\n"
    "movq %rdx, %r11\n"
    "xorl %ecx, %ecx\n"
    "xorl %edx, %edx\n"
    "movl $0xffffffc0, %eax\n"
    "wrpkru\n"
    "movq %r10, %rax\n"
    "movq %r11, %rdx\n"
    /* Free stack space used for stack args */
    "addq $8, %rsp\n"
    /* Compute location to save old stack pointer (using r11) */
    "mov ia2_stackptr_2@GOTTPOFF(%rip), %r11\n"
    /* Write the old stack pointer to memory */
    "movq %rsp, %fs:(%r11)\n"
    /* Compute location to load new stack pointer (using r11) */
    "mov ia2_stackptr_1@GOTTPOFF(%rip), %r11\n"
    /* Read the new stack pointer from memory */
    "movq %fs:(%r11), %rsp\n"
    /* Preserve essential regs on stack */
    "pushq %rax\n"
    /* Scrub non-essential regs */
    "call __libia2_scrub_registers\n"
    /* Restore preserved regs */
    "popq %rax\n"
    /* Set PKRU to the caller's value */
    "movq %rax, %r10\n"
    "movq %rdx, %r11\n"
    "xorl %ecx, %ecx\n"
    "xorl %edx, %edx\n"
    "movl $0xfffffff0, %eax\n"
    "wrpkru\n"
    "movq %r10, %rax\n"
    "movq %r11, %rdx\n"
    "popq %r15\n"
    "popq %r14\n"
    "popq %r13\n"
    "popq %r12\n"
    "popq %rbx\n"
    "popq %rbp\n"
    /* Return to the caller */
    "ret\n"
    ".size __wrap_crc32, .-__wrap_crc32\n"
    ".previous\n"
);
asm(
    /* Wrapper for crc32_combine(int, int, int) -> int: */
    ".text\n"
    ".global __wrap_crc32_combine\n"
    ".type __wrap_crc32_combine, @function\n"
    "__wrap_crc32_combine:\n"
    "pushq %rbp\n"
    "movq %rsp, %rbp\n"
    "pushq %rbx\n"
    "pushq %r12\n"
    "pushq %r13\n"
    "pushq %r14\n"
    "pushq %r15\n"
    ASSERT_PKRU(0xfffffffffffffff0) "\n"
    /* Set PKRU to the intermediate value to move arguments */
    "movq %rcx, %r10\n"
    "movq %rdx, %r11\n"
    "xorl %ecx, %ecx\n"
    "xorl %edx, %edx\n"
    "movl $0xffffffc0, %eax\n"
    "wrpkru\n"
    "movq %r10, %rcx\n"
    "movq %r11, %rdx\n"
    /* Compute location to save old stack pointer (using r11) */
    "mov ia2_stackptr_1@GOTTPOFF(%rip), %r11\n"
    /* Write the old stack pointer to memory */
    "movq %rsp, %fs:(%r11)\n"
    /* Compute location to load new stack pointer (using r11) */
    "mov ia2_stackptr_2@GOTTPOFF(%rip), %r11\n"
    /* Read the new stack pointer from memory */
    "movq %fs:(%r11), %rsp\n"
    "subq $8, %rsp\n"
    /* Preserve essential regs on stack */
    "pushq %rdi\n"
    "pushq %rsi\n"
    "pushq %rdx\n"
    /* Scrub non-essential regs */
    "call __libia2_scrub_registers\n"
    /* Restore preserved regs */
    "popq %rdx\n"
    "popq %rsi\n"
    "popq %rdi\n"
    /* Set PKRU to the compartment's value */
    "movq %rcx, %r10\n"
    "movq %rdx, %r11\n"
    "xorl %ecx, %ecx\n"
    "xorl %edx, %edx\n"
    "movl $0xffffffcc, %eax\n"
    "wrpkru\n"
    "movq %r10, %rcx\n"
    "movq %r11, %rdx\n"
    /* Call wrapped function */
    "call crc32_combine\n"
    /* Set PKRU to the intermediate value to move arguments */
    "movq %rax, %r10\n"
    "movq %rdx, %r11\n"
    "xorl %ecx, %ecx\n"
    "xorl %edx, %edx\n"
    "movl $0xffffffc0, %eax\n"
    "wrpkru\n"
    "movq %r10, %rax\n"
    "movq %r11, %rdx\n"
    /* Free stack space used for stack args */
    "addq $8, %rsp\n"
    /* Compute location to save old stack pointer (using r11) */
    "mov ia2_stackptr_2@GOTTPOFF(%rip), %r11\n"
    /* Write the old stack pointer to memory */
    "movq %rsp, %fs:(%r11)\n"
    /* Compute location to load new stack pointer (using r11) */
    "mov ia2_stackptr_1@GOTTPOFF(%rip), %r11\n"
    /* Read the new stack pointer from memory */
    "movq %fs:(%r11), %rsp\n"
    /* Preserve essential regs on stack */
    "pushq %rax\n"
    /* Scrub non-essential regs */
    "call __libia2_scrub_registers\n"
    /* Restore preserved regs */
    "popq %rax\n"
    /* Set PKRU to the caller's value */
    "movq %rax, %r10\n"
    "movq %rdx, %r11\n"
    "xorl %ecx, %ecx\n"
    "xorl %edx, %edx\n"
    "movl $0xfffffff0, %eax\n"
    "wrpkru\n"
    "movq %r10, %rax\n"
    "movq %r11, %rdx\n"
    "popq %r15\n"
    "popq %r14\n"
    "popq %r13\n"
    "popq %r12\n"
    "popq %rbx\n"
    "popq %rbp\n"
    /* Return to the caller */
    "ret\n"
    ".size __wrap_crc32_combine, .-__wrap_crc32_combine\n"
    ".previous\n"
);
asm(
    /* Wrapper for crc32_combine64(int, int, int) -> int: */
    ".text\n"
    ".global __wrap_crc32_combine64\n"
    ".type __wrap_crc32_combine64, @function\n"
    "__wrap_crc32_combine64:\n"
    "pushq %rbp\n"
    "movq %rsp, %rbp\n"
    "pushq %rbx\n"
    "pushq %r12\n"
    "pushq %r13\n"
    "pushq %r14\n"
    "pushq %r15\n"
    ASSERT_PKRU(0xfffffffffffffff0) "\n"
    /* Set PKRU to the intermediate value to move arguments */
    "movq %rcx, %r10\n"
    "movq %rdx, %r11\n"
    "xorl %ecx, %ecx\n"
    "xorl %edx, %edx\n"
    "movl $0xffffffc0, %eax\n"
    "wrpkru\n"
    "movq %r10, %rcx\n"
    "movq %r11, %rdx\n"
    /* Compute location to save old stack pointer (using r11) */
    "mov ia2_stackptr_1@GOTTPOFF(%rip), %r11\n"
    /* Write the old stack pointer to memory */
    "movq %rsp, %fs:(%r11)\n"
    /* Compute location to load new stack pointer (using r11) */
    "mov ia2_stackptr_2@GOTTPOFF(%rip), %r11\n"
    /* Read the new stack pointer from memory */
    "movq %fs:(%r11), %rsp\n"
    "subq $8, %rsp\n"
    /* Preserve essential regs on stack */
    "pushq %rdi\n"
    "pushq %rsi\n"
    "pushq %rdx\n"
    /* Scrub non-essential regs */
    "call __libia2_scrub_registers\n"
    /* Restore preserved regs */
    "popq %rdx\n"
    "popq %rsi\n"
    "popq %rdi\n"
    /* Set PKRU to the compartment's value */
    "movq %rcx, %r10\n"
    "movq %rdx, %r11\n"
    "xorl %ecx, %ecx\n"
    "xorl %edx, %edx\n"
    "movl $0xffffffcc, %eax\n"
    "wrpkru\n"
    "movq %r10, %rcx\n"
    "movq %r11, %rdx\n"
    /* Call wrapped function */
    "call crc32_combine64\n"
    /* Set PKRU to the intermediate value to move arguments */
    "movq %rax, %r10\n"
    "movq %rdx, %r11\n"
    "xorl %ecx, %ecx\n"
    "xorl %edx, %edx\n"
    "movl $0xffffffc0, %eax\n"
    "wrpkru\n"
    "movq %r10, %rax\n"
    "movq %r11, %rdx\n"
    /* Free stack space used for stack args */
    "addq $8, %rsp\n"
    /* Compute location to save old stack pointer (using r11) */
    "mov ia2_stackptr_2@GOTTPOFF(%rip), %r11\n"
    /* Write the old stack pointer to memory */
    "movq %rsp, %fs:(%r11)\n"
    /* Compute location to load new stack pointer (using r11) */
    "mov ia2_stackptr_1@GOTTPOFF(%rip), %r11\n"
    /* Read the new stack pointer from memory */
    "movq %fs:(%r11), %rsp\n"
    /* Preserve essential regs on stack */
    "pushq %rax\n"
    /* Scrub non-essential regs */
    "call __libia2_scrub_registers\n"
    /* Restore preserved regs */
    "popq %rax\n"
    /* Set PKRU to the caller's value */
    "movq %rax, %r10\n"
    "movq %rdx, %r11\n"
    "xorl %ecx, %ecx\n"
    "xorl %edx, %edx\n"
    "movl $0xfffffff0, %eax\n"
    "wrpkru\n"
    "movq %r10, %rax\n"
    "movq %r11, %rdx\n"
    "popq %r15\n"
    "popq %r14\n"
    "popq %r13\n"
    "popq %r12\n"
    "popq %rbx\n"
    "popq %rbp\n"
    /* Return to the caller */
    "ret\n"
    ".size __wrap_crc32_combine64, .-__wrap_crc32_combine64\n"
    ".previous\n"
);
asm(
    /* Wrapper for crc32_combine_gen(int) -> int: */
    ".text\n"
    ".global __wrap_crc32_combine_gen\n"
    ".type __wrap_crc32_combine_gen, @function\n"
    "__wrap_crc32_combine_gen:\n"
    "pushq %rbp\n"
    "movq %rsp, %rbp\n"
    "pushq %rbx\n"
    "pushq %r12\n"
    "pushq %r13\n"
    "pushq %r14\n"
    "pushq %r15\n"
    ASSERT_PKRU(0xfffffffffffffff0) "\n"
    /* Set PKRU to the intermediate value to move arguments */
    "movq %rcx, %r10\n"
    "movq %rdx, %r11\n"
    "xorl %ecx, %ecx\n"
    "xorl %edx, %edx\n"
    "movl $0xffffffc0, %eax\n"
    "wrpkru\n"
    "movq %r10, %rcx\n"
    "movq %r11, %rdx\n"
    /* Compute location to save old stack pointer (using r11) */
    "mov ia2_stackptr_1@GOTTPOFF(%rip), %r11\n"
    /* Write the old stack pointer to memory */
    "movq %rsp, %fs:(%r11)\n"
    /* Compute location to load new stack pointer (using r11) */
    "mov ia2_stackptr_2@GOTTPOFF(%rip), %r11\n"
    /* Read the new stack pointer from memory */
    "movq %fs:(%r11), %rsp\n"
    "subq $8, %rsp\n"
    /* Preserve essential regs on stack */
    "pushq %rdi\n"
    /* Scrub non-essential regs */
    "call __libia2_scrub_registers\n"
    /* Restore preserved regs */
    "popq %rdi\n"
    /* Set PKRU to the compartment's value */
    "movq %rcx, %r10\n"
    "movq %rdx, %r11\n"
    "xorl %ecx, %ecx\n"
    "xorl %edx, %edx\n"
    "movl $0xffffffcc, %eax\n"
    "wrpkru\n"
    "movq %r10, %rcx\n"
    "movq %r11, %rdx\n"
    /* Call wrapped function */
    "call crc32_combine_gen\n"
    /* Set PKRU to the intermediate value to move arguments */
    "movq %rax, %r10\n"
    "movq %rdx, %r11\n"
    "xorl %ecx, %ecx\n"
    "xorl %edx, %edx\n"
    "movl $0xffffffc0, %eax\n"
    "wrpkru\n"
    "movq %r10, %rax\n"
    "movq %r11, %rdx\n"
    /* Free stack space used for stack args */
    "addq $8, %rsp\n"
    /* Compute location to save old stack pointer (using r11) */
    "mov ia2_stackptr_2@GOTTPOFF(%rip), %r11\n"
    /* Write the old stack pointer to memory */
    "movq %rsp, %fs:(%r11)\n"
    /* Compute location to load new stack pointer (using r11) */
    "mov ia2_stackptr_1@GOTTPOFF(%rip), %r11\n"
    /* Read the new stack pointer from memory */
    "movq %fs:(%r11), %rsp\n"
    /* Preserve essential regs on stack */
    "pushq %rax\n"
    /* Scrub non-essential regs */
    "call __libia2_scrub_registers\n"
    /* Restore preserved regs */
    "popq %rax\n"
    /* Set PKRU to the caller's value */
    "movq %rax, %r10\n"
    "movq %rdx, %r11\n"
    "xorl %ecx, %ecx\n"
    "xorl %edx, %edx\n"
    "movl $0xfffffff0, %eax\n"
    "wrpkru\n"
    "movq %r10, %rax\n"
    "movq %r11, %rdx\n"
    "popq %r15\n"
    "popq %r14\n"
    "popq %r13\n"
    "popq %r12\n"
    "popq %rbx\n"
    "popq %rbp\n"
    /* Return to the caller */
    "ret\n"
    ".size __wrap_crc32_combine_gen, .-__wrap_crc32_combine_gen\n"
    ".previous\n"
);
asm(
    /* Wrapper for crc32_combine_gen64(int) -> int: */
    ".text\n"
    ".global __wrap_crc32_combine_gen64\n"
    ".type __wrap_crc32_combine_gen64, @function\n"
    "__wrap_crc32_combine_gen64:\n"
    "pushq %rbp\n"
    "movq %rsp, %rbp\n"
    "pushq %rbx\n"
    "pushq %r12\n"
    "pushq %r13\n"
    "pushq %r14\n"
    "pushq %r15\n"
    ASSERT_PKRU(0xfffffffffffffff0) "\n"
    /* Set PKRU to the intermediate value to move arguments */
    "movq %rcx, %r10\n"
    "movq %rdx, %r11\n"
    "xorl %ecx, %ecx\n"
    "xorl %edx, %edx\n"
    "movl $0xffffffc0, %eax\n"
    "wrpkru\n"
    "movq %r10, %rcx\n"
    "movq %r11, %rdx\n"
    /* Compute location to save old stack pointer (using r11) */
    "mov ia2_stackptr_1@GOTTPOFF(%rip), %r11\n"
    /* Write the old stack pointer to memory */
    "movq %rsp, %fs:(%r11)\n"
    /* Compute location to load new stack pointer (using r11) */
    "mov ia2_stackptr_2@GOTTPOFF(%rip), %r11\n"
    /* Read the new stack pointer from memory */
    "movq %fs:(%r11), %rsp\n"
    "subq $8, %rsp\n"
    /* Preserve essential regs on stack */
    "pushq %rdi\n"
    /* Scrub non-essential regs */
    "call __libia2_scrub_registers\n"
    /* Restore preserved regs */
    "popq %rdi\n"
    /* Set PKRU to the compartment's value */
    "movq %rcx, %r10\n"
    "movq %rdx, %r11\n"
    "xorl %ecx, %ecx\n"
    "xorl %edx, %edx\n"
    "movl $0xffffffcc, %eax\n"
    "wrpkru\n"
    "movq %r10, %rcx\n"
    "movq %r11, %rdx\n"
    /* Call wrapped function */
    "call crc32_combine_gen64\n"
    /* Set PKRU to the intermediate value to move arguments */
    "movq %rax, %r10\n"
    "movq %rdx, %r11\n"
    "xorl %ecx, %ecx\n"
    "xorl %edx, %edx\n"
    "movl $0xffffffc0, %eax\n"
    "wrpkru\n"
    "movq %r10, %rax\n"
    "movq %r11, %rdx\n"
    /* Free stack space used for stack args */
    "addq $8, %rsp\n"
    /* Compute location to save old stack pointer (using r11) */
    "mov ia2_stackptr_2@GOTTPOFF(%rip), %r11\n"
    /* Write the old stack pointer to memory */
    "movq %rsp, %fs:(%r11)\n"
    /* Compute location to load new stack pointer (using r11) */
    "mov ia2_stackptr_1@GOTTPOFF(%rip), %r11\n"
    /* Read the new stack pointer from memory */
    "movq %fs:(%r11), %rsp\n"
    /* Preserve essential regs on stack */
    "pushq %rax\n"
    /* Scrub non-essential regs */
    "call __libia2_scrub_registers\n"
    /* Restore preserved regs */
    "popq %rax\n"
    /* Set PKRU to the caller's value */
    "movq %rax, %r10\n"
    "movq %rdx, %r11\n"
    "xorl %ecx, %ecx\n"
    "xorl %edx, %edx\n"
    "movl $0xfffffff0, %eax\n"
    "wrpkru\n"
    "movq %r10, %rax\n"
    "movq %r11, %rdx\n"
    "popq %r15\n"
    "popq %r14\n"
    "popq %r13\n"
    "popq %r12\n"
    "popq %rbx\n"
    "popq %rbp\n"
    /* Return to the caller */
    "ret\n"
    ".size __wrap_crc32_combine_gen64, .-__wrap_crc32_combine_gen64\n"
    ".previous\n"
);
asm(
    /* Wrapper for crc32_combine_op(int, int, int) -> int: */
    ".text\n"
    ".global __wrap_crc32_combine_op\n"
    ".type __wrap_crc32_combine_op, @function\n"
    "__wrap_crc32_combine_op:\n"
    "pushq %rbp\n"
    "movq %rsp, %rbp\n"
    "pushq %rbx\n"
    "pushq %r12\n"
    "pushq %r13\n"
    "pushq %r14\n"
    "pushq %r15\n"
    ASSERT_PKRU(0xfffffffffffffff0) "\n"
    /* Set PKRU to the intermediate value to move arguments */
    "movq %rcx, %r10\n"
    "movq %rdx, %r11\n"
    "xorl %ecx, %ecx\n"
    "xorl %edx, %edx\n"
    "movl $0xffffffc0, %eax\n"
    "wrpkru\n"
    "movq %r10, %rcx\n"
    "movq %r11, %rdx\n"
    /* Compute location to save old stack pointer (using r11) */
    "mov ia2_stackptr_1@GOTTPOFF(%rip), %r11\n"
    /* Write the old stack pointer to memory */
    "movq %rsp, %fs:(%r11)\n"
    /* Compute location to load new stack pointer (using r11) */
    "mov ia2_stackptr_2@GOTTPOFF(%rip), %r11\n"
    /* Read the new stack pointer from memory */
    "movq %fs:(%r11), %rsp\n"
    "subq $8, %rsp\n"
    /* Preserve essential regs on stack */
    "pushq %rdi\n"
    "pushq %rsi\n"
    "pushq %rdx\n"
    /* Scrub non-essential regs */
    "call __libia2_scrub_registers\n"
    /* Restore preserved regs */
    "popq %rdx\n"
    "popq %rsi\n"
    "popq %rdi\n"
    /* Set PKRU to the compartment's value */
    "movq %rcx, %r10\n"
    "movq %rdx, %r11\n"
    "xorl %ecx, %ecx\n"
    "xorl %edx, %edx\n"
    "movl $0xffffffcc, %eax\n"
    "wrpkru\n"
    "movq %r10, %rcx\n"
    "movq %r11, %rdx\n"
    /* Call wrapped function */
    "call crc32_combine_op\n"
    /* Set PKRU to the intermediate value to move arguments */
    "movq %rax, %r10\n"
    "movq %rdx, %r11\n"
    "xorl %ecx, %ecx\n"
    "xorl %edx, %edx\n"
    "movl $0xffffffc0, %eax\n"
    "wrpkru\n"
    "movq %r10, %rax\n"
    "movq %r11, %rdx\n"
    /* Free stack space used for stack args */
    "addq $8, %rsp\n"
    /* Compute location to save old stack pointer (using r11) */
    "mov ia2_stackptr_2@GOTTPOFF(%rip), %r11\n"
    /* Write the old stack pointer to memory */
    "movq %rsp, %fs:(%r11)\n"
    /* Compute location to load new stack pointer (using r11) */
    "mov ia2_stackptr_1@GOTTPOFF(%rip), %r11\n"
    /* Read the new stack pointer from memory */
    "movq %fs:(%r11), %rsp\n"
    /* Preserve essential regs on stack */
    "pushq %rax\n"
    /* Scrub non-essential regs */
    "call __libia2_scrub_registers\n"
    /* Restore preserved regs */
    "popq %rax\n"
    /* Set PKRU to the caller's value */
    "movq %rax, %r10\n"
    "movq %rdx, %r11\n"
    "xorl %ecx, %ecx\n"
    "xorl %edx, %edx\n"
    "movl $0xfffffff0, %eax\n"
    "wrpkru\n"
    "movq %r10, %rax\n"
    "movq %r11, %rdx\n"
    "popq %r15\n"
    "popq %r14\n"
    "popq %r13\n"
    "popq %r12\n"
    "popq %rbx\n"
    "popq %rbp\n"
    /* Return to the caller */
    "ret\n"
    ".size __wrap_crc32_combine_op, .-__wrap_crc32_combine_op\n"
    ".previous\n"
);
asm(
    /* Wrapper for crc32_z(int, int, int) -> int: */
    ".text\n"
    ".global __wrap_crc32_z\n"
    ".type __wrap_crc32_z, @function\n"
    "__wrap_crc32_z:\n"
    "pushq %rbp\n"
    "movq %rsp, %rbp\n"
    "pushq %rbx\n"
    "pushq %r12\n"
    "pushq %r13\n"
    "pushq %r14\n"
    "pushq %r15\n"
    ASSERT_PKRU(0xfffffffffffffff0) "\n"
    /* Set PKRU to the intermediate value to move arguments */
    "movq %rcx, %r10\n"
    "movq %rdx, %r11\n"
    "xorl %ecx, %ecx\n"
    "xorl %edx, %edx\n"
    "movl $0xffffffc0, %eax\n"
    "wrpkru\n"
    "movq %r10, %rcx\n"
    "movq %r11, %rdx\n"
    /* Compute location to save old stack pointer (using r11) */
    "mov ia2_stackptr_1@GOTTPOFF(%rip), %r11\n"
    /* Write the old stack pointer to memory */
    "movq %rsp, %fs:(%r11)\n"
    /* Compute location to load new stack pointer (using r11) */
    "mov ia2_stackptr_2@GOTTPOFF(%rip), %r11\n"
    /* Read the new stack pointer from memory */
    "movq %fs:(%r11), %rsp\n"
    "subq $8, %rsp\n"
    /* Preserve essential regs on stack */
    "pushq %rdi\n"
    "pushq %rsi\n"
    "pushq %rdx\n"
    /* Scrub non-essential regs */
    "call __libia2_scrub_registers\n"
    /* Restore preserved regs */
    "popq %rdx\n"
    "popq %rsi\n"
    "popq %rdi\n"
    /* Set PKRU to the compartment's value */
    "movq %rcx, %r10\n"
    "movq %rdx, %r11\n"
    "xorl %ecx, %ecx\n"
    "xorl %edx, %edx\n"
    "movl $0xffffffcc, %eax\n"
    "wrpkru\n"
    "movq %r10, %rcx\n"
    "movq %r11, %rdx\n"
    /* Call wrapped function */
    "call crc32_z\n"
    /* Set PKRU to the intermediate value to move arguments */
    "movq %rax, %r10\n"
    "movq %rdx, %r11\n"
    "xorl %ecx, %ecx\n"
    "xorl %edx, %edx\n"
    "movl $0xffffffc0, %eax\n"
    "wrpkru\n"
    "movq %r10, %rax\n"
    "movq %r11, %rdx\n"
    /* Free stack space used for stack args */
    "addq $8, %rsp\n"
    /* Compute location to save old stack pointer (using r11) */
    "mov ia2_stackptr_2@GOTTPOFF(%rip), %r11\n"
    /* Write the old stack pointer to memory */
    "movq %rsp, %fs:(%r11)\n"
    /* Compute location to load new stack pointer (using r11) */
    "mov ia2_stackptr_1@GOTTPOFF(%rip), %r11\n"
    /* Read the new stack pointer from memory */
    "movq %fs:(%r11), %rsp\n"
    /* Preserve essential regs on stack */
    "pushq %rax\n"
    /* Scrub non-essential regs */
    "call __libia2_scrub_registers\n"
    /* Restore preserved regs */
    "popq %rax\n"
    /* Set PKRU to the caller's value */
    "movq %rax, %r10\n"
    "movq %rdx, %r11\n"
    "xorl %ecx, %ecx\n"
    "xorl %edx, %edx\n"
    "movl $0xfffffff0, %eax\n"
    "wrpkru\n"
    "movq %r10, %rax\n"
    "movq %r11, %rdx\n"
    "popq %r15\n"
    "popq %r14\n"
    "popq %r13\n"
    "popq %r12\n"
    "popq %rbx\n"
    "popq %rbp\n"
    /* Return to the caller */
    "ret\n"
    ".size __wrap_crc32_z, .-__wrap_crc32_z\n"
    ".previous\n"
);
asm(
    /* Wrapper for deflate(int, int) -> int: */
    ".text\n"
    ".global __wrap_deflate\n"
    ".type __wrap_deflate, @function\n"
    "__wrap_deflate:\n"
    "pushq %rbp\n"
    "movq %rsp, %rbp\n"
    "pushq %rbx\n"
    "pushq %r12\n"
    "pushq %r13\n"
    "pushq %r14\n"
    "pushq %r15\n"
    ASSERT_PKRU(0xfffffffffffffff0) "\n"
    /* Set PKRU to the intermediate value to move arguments */
    "movq %rcx, %r10\n"
    "movq %rdx, %r11\n"
    "xorl %ecx, %ecx\n"
    "xorl %edx, %edx\n"
    "movl $0xffffffc0, %eax\n"
    "wrpkru\n"
    "movq %r10, %rcx\n"
    "movq %r11, %rdx\n"
    /* Compute location to save old stack pointer (using r11) */
    "mov ia2_stackptr_1@GOTTPOFF(%rip), %r11\n"
    /* Write the old stack pointer to memory */
    "movq %rsp, %fs:(%r11)\n"
    /* Compute location to load new stack pointer (using r11) */
    "mov ia2_stackptr_2@GOTTPOFF(%rip), %r11\n"
    /* Read the new stack pointer from memory */
    "movq %fs:(%r11), %rsp\n"
    "subq $8, %rsp\n"
    /* Preserve essential regs on stack */
    "pushq %rdi\n"
    "pushq %rsi\n"
    /* Scrub non-essential regs */
    "call __libia2_scrub_registers\n"
    /* Restore preserved regs */
    "popq %rsi\n"
    "popq %rdi\n"
    /* Set PKRU to the compartment's value */
    "movq %rcx, %r10\n"
    "movq %rdx, %r11\n"
    "xorl %ecx, %ecx\n"
    "xorl %edx, %edx\n"
    "movl $0xffffffcc, %eax\n"
    "wrpkru\n"
    "movq %r10, %rcx\n"
    "movq %r11, %rdx\n"
    /* Call wrapped function */
    "call deflate\n"
    /* Set PKRU to the intermediate value to move arguments */
    "movq %rax, %r10\n"
    "movq %rdx, %r11\n"
    "xorl %ecx, %ecx\n"
    "xorl %edx, %edx\n"
    "movl $0xffffffc0, %eax\n"
    "wrpkru\n"
    "movq %r10, %rax\n"
    "movq %r11, %rdx\n"
    /* Free stack space used for stack args */
    "addq $8, %rsp\n"
    /* Compute location to save old stack pointer (using r11) */
    "mov ia2_stackptr_2@GOTTPOFF(%rip), %r11\n"
    /* Write the old stack pointer to memory */
    "movq %rsp, %fs:(%r11)\n"
    /* Compute location to load new stack pointer (using r11) */
    "mov ia2_stackptr_1@GOTTPOFF(%rip), %r11\n"
    /* Read the new stack pointer from memory */
    "movq %fs:(%r11), %rsp\n"
    /* Preserve essential regs on stack */
    "pushq %rax\n"
    /* Scrub non-essential regs */
    "call __libia2_scrub_registers\n"
    /* Restore preserved regs */
    "popq %rax\n"
    /* Set PKRU to the caller's value */
    "movq %rax, %r10\n"
    "movq %rdx, %r11\n"
    "xorl %ecx, %ecx\n"
    "xorl %edx, %edx\n"
    "movl $0xfffffff0, %eax\n"
    "wrpkru\n"
    "movq %r10, %rax\n"
    "movq %r11, %rdx\n"
    "popq %r15\n"
    "popq %r14\n"
    "popq %r13\n"
    "popq %r12\n"
    "popq %rbx\n"
    "popq %rbp\n"
    /* Return to the caller */
    "ret\n"
    ".size __wrap_deflate, .-__wrap_deflate\n"
    ".previous\n"
);
asm(
    /* Wrapper for deflateBound(int, int) -> int: */
    ".text\n"
    ".global __wrap_deflateBound\n"
    ".type __wrap_deflateBound, @function\n"
    "__wrap_deflateBound:\n"
    "pushq %rbp\n"
    "movq %rsp, %rbp\n"
    "pushq %rbx\n"
    "pushq %r12\n"
    "pushq %r13\n"
    "pushq %r14\n"
    "pushq %r15\n"
    ASSERT_PKRU(0xfffffffffffffff0) "\n"
    /* Set PKRU to the intermediate value to move arguments */
    "movq %rcx, %r10\n"
    "movq %rdx, %r11\n"
    "xorl %ecx, %ecx\n"
    "xorl %edx, %edx\n"
    "movl $0xffffffc0, %eax\n"
    "wrpkru\n"
    "movq %r10, %rcx\n"
    "movq %r11, %rdx\n"
    /* Compute location to save old stack pointer (using r11) */
    "mov ia2_stackptr_1@GOTTPOFF(%rip), %r11\n"
    /* Write the old stack pointer to memory */
    "movq %rsp, %fs:(%r11)\n"
    /* Compute location to load new stack pointer (using r11) */
    "mov ia2_stackptr_2@GOTTPOFF(%rip), %r11\n"
    /* Read the new stack pointer from memory */
    "movq %fs:(%r11), %rsp\n"
    "subq $8, %rsp\n"
    /* Preserve essential regs on stack */
    "pushq %rdi\n"
    "pushq %rsi\n"
    /* Scrub non-essential regs */
    "call __libia2_scrub_registers\n"
    /* Restore preserved regs */
    "popq %rsi\n"
    "popq %rdi\n"
    /* Set PKRU to the compartment's value */
    "movq %rcx, %r10\n"
    "movq %rdx, %r11\n"
    "xorl %ecx, %ecx\n"
    "xorl %edx, %edx\n"
    "movl $0xffffffcc, %eax\n"
    "wrpkru\n"
    "movq %r10, %rcx\n"
    "movq %r11, %rdx\n"
    /* Call wrapped function */
    "call deflateBound\n"
    /* Set PKRU to the intermediate value to move arguments */
    "movq %rax, %r10\n"
    "movq %rdx, %r11\n"
    "xorl %ecx, %ecx\n"
    "xorl %edx, %edx\n"
    "movl $0xffffffc0, %eax\n"
    "wrpkru\n"
    "movq %r10, %rax\n"
    "movq %r11, %rdx\n"
    /* Free stack space used for stack args */
    "addq $8, %rsp\n"
    /* Compute location to save old stack pointer (using r11) */
    "mov ia2_stackptr_2@GOTTPOFF(%rip), %r11\n"
    /* Write the old stack pointer to memory */
    "movq %rsp, %fs:(%r11)\n"
    /* Compute location to load new stack pointer (using r11) */
    "mov ia2_stackptr_1@GOTTPOFF(%rip), %r11\n"
    /* Read the new stack pointer from memory */
    "movq %fs:(%r11), %rsp\n"
    /* Preserve essential regs on stack */
    "pushq %rax\n"
    /* Scrub non-essential regs */
    "call __libia2_scrub_registers\n"
    /* Restore preserved regs */
    "popq %rax\n"
    /* Set PKRU to the caller's value */
    "movq %rax, %r10\n"
    "movq %rdx, %r11\n"
    "xorl %ecx, %ecx\n"
    "xorl %edx, %edx\n"
    "movl $0xfffffff0, %eax\n"
    "wrpkru\n"
    "movq %r10, %rax\n"
    "movq %r11, %rdx\n"
    "popq %r15\n"
    "popq %r14\n"
    "popq %r13\n"
    "popq %r12\n"
    "popq %rbx\n"
    "popq %rbp\n"
    /* Return to the caller */
    "ret\n"
    ".size __wrap_deflateBound, .-__wrap_deflateBound\n"
    ".previous\n"
);
asm(
    /* Wrapper for deflateCopy(int, int) -> int: */
    ".text\n"
    ".global __wrap_deflateCopy\n"
    ".type __wrap_deflateCopy, @function\n"
    "__wrap_deflateCopy:\n"
    "pushq %rbp\n"
    "movq %rsp, %rbp\n"
    "pushq %rbx\n"
    "pushq %r12\n"
    "pushq %r13\n"
    "pushq %r14\n"
    "pushq %r15\n"
    ASSERT_PKRU(0xfffffffffffffff0) "\n"
    /* Set PKRU to the intermediate value to move arguments */
    "movq %rcx, %r10\n"
    "movq %rdx, %r11\n"
    "xorl %ecx, %ecx\n"
    "xorl %edx, %edx\n"
    "movl $0xffffffc0, %eax\n"
    "wrpkru\n"
    "movq %r10, %rcx\n"
    "movq %r11, %rdx\n"
    /* Compute location to save old stack pointer (using r11) */
    "mov ia2_stackptr_1@GOTTPOFF(%rip), %r11\n"
    /* Write the old stack pointer to memory */
    "movq %rsp, %fs:(%r11)\n"
    /* Compute location to load new stack pointer (using r11) */
    "mov ia2_stackptr_2@GOTTPOFF(%rip), %r11\n"
    /* Read the new stack pointer from memory */
    "movq %fs:(%r11), %rsp\n"
    "subq $8, %rsp\n"
    /* Preserve essential regs on stack */
    "pushq %rdi\n"
    "pushq %rsi\n"
    /* Scrub non-essential regs */
    "call __libia2_scrub_registers\n"
    /* Restore preserved regs */
    "popq %rsi\n"
    "popq %rdi\n"
    /* Set PKRU to the compartment's value */
    "movq %rcx, %r10\n"
    "movq %rdx, %r11\n"
    "xorl %ecx, %ecx\n"
    "xorl %edx, %edx\n"
    "movl $0xffffffcc, %eax\n"
    "wrpkru\n"
    "movq %r10, %rcx\n"
    "movq %r11, %rdx\n"
    /* Call wrapped function */
    "call deflateCopy\n"
    /* Set PKRU to the intermediate value to move arguments */
    "movq %rax, %r10\n"
    "movq %rdx, %r11\n"
    "xorl %ecx, %ecx\n"
    "xorl %edx, %edx\n"
    "movl $0xffffffc0, %eax\n"
    "wrpkru\n"
    "movq %r10, %rax\n"
    "movq %r11, %rdx\n"
    /* Free stack space used for stack args */
    "addq $8, %rsp\n"
    /* Compute location to save old stack pointer (using r11) */
    "mov ia2_stackptr_2@GOTTPOFF(%rip), %r11\n"
    /* Write the old stack pointer to memory */
    "movq %rsp, %fs:(%r11)\n"
    /* Compute location to load new stack pointer (using r11) */
    "mov ia2_stackptr_1@GOTTPOFF(%rip), %r11\n"
    /* Read the new stack pointer from memory */
    "movq %fs:(%r11), %rsp\n"
    /* Preserve essential regs on stack */
    "pushq %rax\n"
    /* Scrub non-essential regs */
    "call __libia2_scrub_registers\n"
    /* Restore preserved regs */
    "popq %rax\n"
    /* Set PKRU to the caller's value */
    "movq %rax, %r10\n"
    "movq %rdx, %r11\n"
    "xorl %ecx, %ecx\n"
    "xorl %edx, %edx\n"
    "movl $0xfffffff0, %eax\n"
    "wrpkru\n"
    "movq %r10, %rax\n"
    "movq %r11, %rdx\n"
    "popq %r15\n"
    "popq %r14\n"
    "popq %r13\n"
    "popq %r12\n"
    "popq %rbx\n"
    "popq %rbp\n"
    /* Return to the caller */
    "ret\n"
    ".size __wrap_deflateCopy, .-__wrap_deflateCopy\n"
    ".previous\n"
);
asm(
    /* Wrapper for deflateEnd(int) -> int: */
    ".text\n"
    ".global __wrap_deflateEnd\n"
    ".type __wrap_deflateEnd, @function\n"
    "__wrap_deflateEnd:\n"
    "pushq %rbp\n"
    "movq %rsp, %rbp\n"
    "pushq %rbx\n"
    "pushq %r12\n"
    "pushq %r13\n"
    "pushq %r14\n"
    "pushq %r15\n"
    ASSERT_PKRU(0xfffffffffffffff0) "\n"
    /* Set PKRU to the intermediate value to move arguments */
    "movq %rcx, %r10\n"
    "movq %rdx, %r11\n"
    "xorl %ecx, %ecx\n"
    "xorl %edx, %edx\n"
    "movl $0xffffffc0, %eax\n"
    "wrpkru\n"
    "movq %r10, %rcx\n"
    "movq %r11, %rdx\n"
    /* Compute location to save old stack pointer (using r11) */
    "mov ia2_stackptr_1@GOTTPOFF(%rip), %r11\n"
    /* Write the old stack pointer to memory */
    "movq %rsp, %fs:(%r11)\n"
    /* Compute location to load new stack pointer (using r11) */
    "mov ia2_stackptr_2@GOTTPOFF(%rip), %r11\n"
    /* Read the new stack pointer from memory */
    "movq %fs:(%r11), %rsp\n"
    "subq $8, %rsp\n"
    /* Preserve essential regs on stack */
    "pushq %rdi\n"
    /* Scrub non-essential regs */
    "call __libia2_scrub_registers\n"
    /* Restore preserved regs */
    "popq %rdi\n"
    /* Set PKRU to the compartment's value */
    "movq %rcx, %r10\n"
    "movq %rdx, %r11\n"
    "xorl %ecx, %ecx\n"
    "xorl %edx, %edx\n"
    "movl $0xffffffcc, %eax\n"
    "wrpkru\n"
    "movq %r10, %rcx\n"
    "movq %r11, %rdx\n"
    /* Call wrapped function */
    "call deflateEnd\n"
    /* Set PKRU to the intermediate value to move arguments */
    "movq %rax, %r10\n"
    "movq %rdx, %r11\n"
    "xorl %ecx, %ecx\n"
    "xorl %edx, %edx\n"
    "movl $0xffffffc0, %eax\n"
    "wrpkru\n"
    "movq %r10, %rax\n"
    "movq %r11, %rdx\n"
    /* Free stack space used for stack args */
    "addq $8, %rsp\n"
    /* Compute location to save old stack pointer (using r11) */
    "mov ia2_stackptr_2@GOTTPOFF(%rip), %r11\n"
    /* Write the old stack pointer to memory */
    "movq %rsp, %fs:(%r11)\n"
    /* Compute location to load new stack pointer (using r11) */
    "mov ia2_stackptr_1@GOTTPOFF(%rip), %r11\n"
    /* Read the new stack pointer from memory */
    "movq %fs:(%r11), %rsp\n"
    /* Preserve essential regs on stack */
    "pushq %rax\n"
    /* Scrub non-essential regs */
    "call __libia2_scrub_registers\n"
    /* Restore preserved regs */
    "popq %rax\n"
    /* Set PKRU to the caller's value */
    "movq %rax, %r10\n"
    "movq %rdx, %r11\n"
    "xorl %ecx, %ecx\n"
    "xorl %edx, %edx\n"
    "movl $0xfffffff0, %eax\n"
    "wrpkru\n"
    "movq %r10, %rax\n"
    "movq %r11, %rdx\n"
    "popq %r15\n"
    "popq %r14\n"
    "popq %r13\n"
    "popq %r12\n"
    "popq %rbx\n"
    "popq %rbp\n"
    /* Return to the caller */
    "ret\n"
    ".size __wrap_deflateEnd, .-__wrap_deflateEnd\n"
    ".previous\n"
);
asm(
    /* Wrapper for deflateGetDictionary(int, int, int) -> int: */
    ".text\n"
    ".global __wrap_deflateGetDictionary\n"
    ".type __wrap_deflateGetDictionary, @function\n"
    "__wrap_deflateGetDictionary:\n"
    "pushq %rbp\n"
    "movq %rsp, %rbp\n"
    "pushq %rbx\n"
    "pushq %r12\n"
    "pushq %r13\n"
    "pushq %r14\n"
    "pushq %r15\n"
    ASSERT_PKRU(0xfffffffffffffff0) "\n"
    /* Set PKRU to the intermediate value to move arguments */
    "movq %rcx, %r10\n"
    "movq %rdx, %r11\n"
    "xorl %ecx, %ecx\n"
    "xorl %edx, %edx\n"
    "movl $0xffffffc0, %eax\n"
    "wrpkru\n"
    "movq %r10, %rcx\n"
    "movq %r11, %rdx\n"
    /* Compute location to save old stack pointer (using r11) */
    "mov ia2_stackptr_1@GOTTPOFF(%rip), %r11\n"
    /* Write the old stack pointer to memory */
    "movq %rsp, %fs:(%r11)\n"
    /* Compute location to load new stack pointer (using r11) */
    "mov ia2_stackptr_2@GOTTPOFF(%rip), %r11\n"
    /* Read the new stack pointer from memory */
    "movq %fs:(%r11), %rsp\n"
    "subq $8, %rsp\n"
    /* Preserve essential regs on stack */
    "pushq %rdi\n"
    "pushq %rsi\n"
    "pushq %rdx\n"
    /* Scrub non-essential regs */
    "call __libia2_scrub_registers\n"
    /* Restore preserved regs */
    "popq %rdx\n"
    "popq %rsi\n"
    "popq %rdi\n"
    /* Set PKRU to the compartment's value */
    "movq %rcx, %r10\n"
    "movq %rdx, %r11\n"
    "xorl %ecx, %ecx\n"
    "xorl %edx, %edx\n"
    "movl $0xffffffcc, %eax\n"
    "wrpkru\n"
    "movq %r10, %rcx\n"
    "movq %r11, %rdx\n"
    /* Call wrapped function */
    "call deflateGetDictionary\n"
    /* Set PKRU to the intermediate value to move arguments */
    "movq %rax, %r10\n"
    "movq %rdx, %r11\n"
    "xorl %ecx, %ecx\n"
    "xorl %edx, %edx\n"
    "movl $0xffffffc0, %eax\n"
    "wrpkru\n"
    "movq %r10, %rax\n"
    "movq %r11, %rdx\n"
    /* Free stack space used for stack args */
    "addq $8, %rsp\n"
    /* Compute location to save old stack pointer (using r11) */
    "mov ia2_stackptr_2@GOTTPOFF(%rip), %r11\n"
    /* Write the old stack pointer to memory */
    "movq %rsp, %fs:(%r11)\n"
    /* Compute location to load new stack pointer (using r11) */
    "mov ia2_stackptr_1@GOTTPOFF(%rip), %r11\n"
    /* Read the new stack pointer from memory */
    "movq %fs:(%r11), %rsp\n"
    /* Preserve essential regs on stack */
    "pushq %rax\n"
    /* Scrub non-essential regs */
    "call __libia2_scrub_registers\n"
    /* Restore preserved regs */
    "popq %rax\n"
    /* Set PKRU to the caller's value */
    "movq %rax, %r10\n"
    "movq %rdx, %r11\n"
    "xorl %ecx, %ecx\n"
    "xorl %edx, %edx\n"
    "movl $0xfffffff0, %eax\n"
    "wrpkru\n"
    "movq %r10, %rax\n"
    "movq %r11, %rdx\n"
    "popq %r15\n"
    "popq %r14\n"
    "popq %r13\n"
    "popq %r12\n"
    "popq %rbx\n"
    "popq %rbp\n"
    /* Return to the caller */
    "ret\n"
    ".size __wrap_deflateGetDictionary, .-__wrap_deflateGetDictionary\n"
    ".previous\n"
);
asm(
    /* Wrapper for deflateInit2_(int, int, int, int, int, int, int, int) -> int: */
    ".text\n"
    ".global __wrap_deflateInit2_\n"
    ".type __wrap_deflateInit2_, @function\n"
    "__wrap_deflateInit2_:\n"
    "pushq %rbp\n"
    "movq %rsp, %rbp\n"
    "pushq %rbx\n"
    "pushq %r12\n"
    "pushq %r13\n"
    "pushq %r14\n"
    "pushq %r15\n"
    ASSERT_PKRU(0xfffffffffffffff0) "\n"
    /* Set PKRU to the intermediate value to move arguments */
    "movq %rcx, %r10\n"
    "movq %rdx, %r11\n"
    "xorl %ecx, %ecx\n"
    "xorl %edx, %edx\n"
    "movl $0xffffffc0, %eax\n"
    "wrpkru\n"
    "movq %r10, %rcx\n"
    "movq %r11, %rdx\n"
    /* Compute location to save old stack pointer (using r11) */
    "mov ia2_stackptr_1@GOTTPOFF(%rip), %r11\n"
    /* Write the old stack pointer to memory */
    "movq %rsp, %fs:(%r11)\n"
    /* Compute location to load new stack pointer (using r11) */
    "mov ia2_stackptr_2@GOTTPOFF(%rip), %r11\n"
    /* Read the new stack pointer from memory */
    "movq %fs:(%r11), %rsp\n"
    "subq $8, %rsp\n"
    /* Copy stack arguments from the caller's stack to the compartment */
    "mov ia2_stackptr_1@GOTTPOFF(%rip), %r12\n"
    "movq %fs:(%r12), %rax\n"
    "pushq 64(%rax)\n"
    "pushq 56(%rax)\n"
    /* Preserve essential regs on stack */
    "pushq %rdi\n"
    "pushq %rsi\n"
    "pushq %rdx\n"
    "pushq %rcx\n"
    "pushq %r8\n"
    "pushq %r9\n"
    /* Scrub non-essential regs */
    "call __libia2_scrub_registers\n"
    /* Restore preserved regs */
    "popq %r9\n"
    "popq %r8\n"
    "popq %rcx\n"
    "popq %rdx\n"
    "popq %rsi\n"
    "popq %rdi\n"
    /* Set PKRU to the compartment's value */
    "movq %rcx, %r10\n"
    "movq %rdx, %r11\n"
    "xorl %ecx, %ecx\n"
    "xorl %edx, %edx\n"
    "movl $0xffffffcc, %eax\n"
    "wrpkru\n"
    "movq %r10, %rcx\n"
    "movq %r11, %rdx\n"
    /* Call wrapped function */
    "call deflateInit2_\n"
    /* Set PKRU to the intermediate value to move arguments */
    "movq %rax, %r10\n"
    "movq %rdx, %r11\n"
    "xorl %ecx, %ecx\n"
    "xorl %edx, %edx\n"
    "movl $0xffffffc0, %eax\n"
    "wrpkru\n"
    "movq %r10, %rax\n"
    "movq %r11, %rdx\n"
    /* Free stack space used for stack args */
    "addq $24, %rsp\n"
    /* Compute location to save old stack pointer (using r11) */
    "mov ia2_stackptr_2@GOTTPOFF(%rip), %r11\n"
    /* Write the old stack pointer to memory */
    "movq %rsp, %fs:(%r11)\n"
    /* Compute location to load new stack pointer (using r11) */
    "mov ia2_stackptr_1@GOTTPOFF(%rip), %r11\n"
    /* Read the new stack pointer from memory */
    "movq %fs:(%r11), %rsp\n"
    /* Preserve essential regs on stack */
    "pushq %rax\n"
    /* Scrub non-essential regs */
    "call __libia2_scrub_registers\n"
    /* Restore preserved regs */
    "popq %rax\n"
    /* Set PKRU to the caller's value */
    "movq %rax, %r10\n"
    "movq %rdx, %r11\n"
    "xorl %ecx, %ecx\n"
    "xorl %edx, %edx\n"
    "movl $0xfffffff0, %eax\n"
    "wrpkru\n"
    "movq %r10, %rax\n"
    "movq %r11, %rdx\n"
    "popq %r15\n"
    "popq %r14\n"
    "popq %r13\n"
    "popq %r12\n"
    "popq %rbx\n"
    "popq %rbp\n"
    /* Return to the caller */
    "ret\n"
    ".size __wrap_deflateInit2_, .-__wrap_deflateInit2_\n"
    ".previous\n"
);
asm(
    /* Wrapper for deflateInit_(int, int, int, int) -> int: */
    ".text\n"
    ".global __wrap_deflateInit_\n"
    ".type __wrap_deflateInit_, @function\n"
    "__wrap_deflateInit_:\n"
    "pushq %rbp\n"
    "movq %rsp, %rbp\n"
    "pushq %rbx\n"
    "pushq %r12\n"
    "pushq %r13\n"
    "pushq %r14\n"
    "pushq %r15\n"
    ASSERT_PKRU(0xfffffffffffffff0) "\n"
    /* Set PKRU to the intermediate value to move arguments */
    "movq %rcx, %r10\n"
    "movq %rdx, %r11\n"
    "xorl %ecx, %ecx\n"
    "xorl %edx, %edx\n"
    "movl $0xffffffc0, %eax\n"
    "wrpkru\n"
    "movq %r10, %rcx\n"
    "movq %r11, %rdx\n"
    /* Compute location to save old stack pointer (using r11) */
    "mov ia2_stackptr_1@GOTTPOFF(%rip), %r11\n"
    /* Write the old stack pointer to memory */
    "movq %rsp, %fs:(%r11)\n"
    /* Compute location to load new stack pointer (using r11) */
    "mov ia2_stackptr_2@GOTTPOFF(%rip), %r11\n"
    /* Read the new stack pointer from memory */
    "movq %fs:(%r11), %rsp\n"
    "subq $8, %rsp\n"
    /* Preserve essential regs on stack */
    "pushq %rdi\n"
    "pushq %rsi\n"
    "pushq %rdx\n"
    "pushq %rcx\n"
    /* Scrub non-essential regs */
    "call __libia2_scrub_registers\n"
    /* Restore preserved regs */
    "popq %rcx\n"
    "popq %rdx\n"
    "popq %rsi\n"
    "popq %rdi\n"
    /* Set PKRU to the compartment's value */
    "movq %rcx, %r10\n"
    "movq %rdx, %r11\n"
    "xorl %ecx, %ecx\n"
    "xorl %edx, %edx\n"
    "movl $0xffffffcc, %eax\n"
    "wrpkru\n"
    "movq %r10, %rcx\n"
    "movq %r11, %rdx\n"
    /* Call wrapped function */
    "call deflateInit_\n"
    /* Set PKRU to the intermediate value to move arguments */
    "movq %rax, %r10\n"
    "movq %rdx, %r11\n"
    "xorl %ecx, %ecx\n"
    "xorl %edx, %edx\n"
    "movl $0xffffffc0, %eax\n"
    "wrpkru\n"
    "movq %r10, %rax\n"
    "movq %r11, %rdx\n"
    /* Free stack space used for stack args */
    "addq $8, %rsp\n"
    /* Compute location to save old stack pointer (using r11) */
    "mov ia2_stackptr_2@GOTTPOFF(%rip), %r11\n"
    /* Write the old stack pointer to memory */
    "movq %rsp, %fs:(%r11)\n"
    /* Compute location to load new stack pointer (using r11) */
    "mov ia2_stackptr_1@GOTTPOFF(%rip), %r11\n"
    /* Read the new stack pointer from memory */
    "movq %fs:(%r11), %rsp\n"
    /* Preserve essential regs on stack */
    "pushq %rax\n"
    /* Scrub non-essential regs */
    "call __libia2_scrub_registers\n"
    /* Restore preserved regs */
    "popq %rax\n"
    /* Set PKRU to the caller's value */
    "movq %rax, %r10\n"
    "movq %rdx, %r11\n"
    "xorl %ecx, %ecx\n"
    "xorl %edx, %edx\n"
    "movl $0xfffffff0, %eax\n"
    "wrpkru\n"
    "movq %r10, %rax\n"
    "movq %r11, %rdx\n"
    "popq %r15\n"
    "popq %r14\n"
    "popq %r13\n"
    "popq %r12\n"
    "popq %rbx\n"
    "popq %rbp\n"
    /* Return to the caller */
    "ret\n"
    ".size __wrap_deflateInit_, .-__wrap_deflateInit_\n"
    ".previous\n"
);
asm(
    /* Wrapper for deflateParams(int, int, int) -> int: */
    ".text\n"
    ".global __wrap_deflateParams\n"
    ".type __wrap_deflateParams, @function\n"
    "__wrap_deflateParams:\n"
    "pushq %rbp\n"
    "movq %rsp, %rbp\n"
    "pushq %rbx\n"
    "pushq %r12\n"
    "pushq %r13\n"
    "pushq %r14\n"
    "pushq %r15\n"
    ASSERT_PKRU(0xfffffffffffffff0) "\n"
    /* Set PKRU to the intermediate value to move arguments */
    "movq %rcx, %r10\n"
    "movq %rdx, %r11\n"
    "xorl %ecx, %ecx\n"
    "xorl %edx, %edx\n"
    "movl $0xffffffc0, %eax\n"
    "wrpkru\n"
    "movq %r10, %rcx\n"
    "movq %r11, %rdx\n"
    /* Compute location to save old stack pointer (using r11) */
    "mov ia2_stackptr_1@GOTTPOFF(%rip), %r11\n"
    /* Write the old stack pointer to memory */
    "movq %rsp, %fs:(%r11)\n"
    /* Compute location to load new stack pointer (using r11) */
    "mov ia2_stackptr_2@GOTTPOFF(%rip), %r11\n"
    /* Read the new stack pointer from memory */
    "movq %fs:(%r11), %rsp\n"
    "subq $8, %rsp\n"
    /* Preserve essential regs on stack */
    "pushq %rdi\n"
    "pushq %rsi\n"
    "pushq %rdx\n"
    /* Scrub non-essential regs */
    "call __libia2_scrub_registers\n"
    /* Restore preserved regs */
    "popq %rdx\n"
    "popq %rsi\n"
    "popq %rdi\n"
    /* Set PKRU to the compartment's value */
    "movq %rcx, %r10\n"
    "movq %rdx, %r11\n"
    "xorl %ecx, %ecx\n"
    "xorl %edx, %edx\n"
    "movl $0xffffffcc, %eax\n"
    "wrpkru\n"
    "movq %r10, %rcx\n"
    "movq %r11, %rdx\n"
    /* Call wrapped function */
    "call deflateParams\n"
    /* Set PKRU to the intermediate value to move arguments */
    "movq %rax, %r10\n"
    "movq %rdx, %r11\n"
    "xorl %ecx, %ecx\n"
    "xorl %edx, %edx\n"
    "movl $0xffffffc0, %eax\n"
    "wrpkru\n"
    "movq %r10, %rax\n"
    "movq %r11, %rdx\n"
    /* Free stack space used for stack args */
    "addq $8, %rsp\n"
    /* Compute location to save old stack pointer (using r11) */
    "mov ia2_stackptr_2@GOTTPOFF(%rip), %r11\n"
    /* Write the old stack pointer to memory */
    "movq %rsp, %fs:(%r11)\n"
    /* Compute location to load new stack pointer (using r11) */
    "mov ia2_stackptr_1@GOTTPOFF(%rip), %r11\n"
    /* Read the new stack pointer from memory */
    "movq %fs:(%r11), %rsp\n"
    /* Preserve essential regs on stack */
    "pushq %rax\n"
    /* Scrub non-essential regs */
    "call __libia2_scrub_registers\n"
    /* Restore preserved regs */
    "popq %rax\n"
    /* Set PKRU to the caller's value */
    "movq %rax, %r10\n"
    "movq %rdx, %r11\n"
    "xorl %ecx, %ecx\n"
    "xorl %edx, %edx\n"
    "movl $0xfffffff0, %eax\n"
    "wrpkru\n"
    "movq %r10, %rax\n"
    "movq %r11, %rdx\n"
    "popq %r15\n"
    "popq %r14\n"
    "popq %r13\n"
    "popq %r12\n"
    "popq %rbx\n"
    "popq %rbp\n"
    /* Return to the caller */
    "ret\n"
    ".size __wrap_deflateParams, .-__wrap_deflateParams\n"
    ".previous\n"
);
asm(
    /* Wrapper for deflatePending(int, int, int) -> int: */
    ".text\n"
    ".global __wrap_deflatePending\n"
    ".type __wrap_deflatePending, @function\n"
    "__wrap_deflatePending:\n"
    "pushq %rbp\n"
    "movq %rsp, %rbp\n"
    "pushq %rbx\n"
    "pushq %r12\n"
    "pushq %r13\n"
    "pushq %r14\n"
    "pushq %r15\n"
    ASSERT_PKRU(0xfffffffffffffff0) "\n"
    /* Set PKRU to the intermediate value to move arguments */
    "movq %rcx, %r10\n"
    "movq %rdx, %r11\n"
    "xorl %ecx, %ecx\n"
    "xorl %edx, %edx\n"
    "movl $0xffffffc0, %eax\n"
    "wrpkru\n"
    "movq %r10, %rcx\n"
    "movq %r11, %rdx\n"
    /* Compute location to save old stack pointer (using r11) */
    "mov ia2_stackptr_1@GOTTPOFF(%rip), %r11\n"
    /* Write the old stack pointer to memory */
    "movq %rsp, %fs:(%r11)\n"
    /* Compute location to load new stack pointer (using r11) */
    "mov ia2_stackptr_2@GOTTPOFF(%rip), %r11\n"
    /* Read the new stack pointer from memory */
    "movq %fs:(%r11), %rsp\n"
    "subq $8, %rsp\n"
    /* Preserve essential regs on stack */
    "pushq %rdi\n"
    "pushq %rsi\n"
    "pushq %rdx\n"
    /* Scrub non-essential regs */
    "call __libia2_scrub_registers\n"
    /* Restore preserved regs */
    "popq %rdx\n"
    "popq %rsi\n"
    "popq %rdi\n"
    /* Set PKRU to the compartment's value */
    "movq %rcx, %r10\n"
    "movq %rdx, %r11\n"
    "xorl %ecx, %ecx\n"
    "xorl %edx, %edx\n"
    "movl $0xffffffcc, %eax\n"
    "wrpkru\n"
    "movq %r10, %rcx\n"
    "movq %r11, %rdx\n"
    /* Call wrapped function */
    "call deflatePending\n"
    /* Set PKRU to the intermediate value to move arguments */
    "movq %rax, %r10\n"
    "movq %rdx, %r11\n"
    "xorl %ecx, %ecx\n"
    "xorl %edx, %edx\n"
    "movl $0xffffffc0, %eax\n"
    "wrpkru\n"
    "movq %r10, %rax\n"
    "movq %r11, %rdx\n"
    /* Free stack space used for stack args */
    "addq $8, %rsp\n"
    /* Compute location to save old stack pointer (using r11) */
    "mov ia2_stackptr_2@GOTTPOFF(%rip), %r11\n"
    /* Write the old stack pointer to memory */
    "movq %rsp, %fs:(%r11)\n"
    /* Compute location to load new stack pointer (using r11) */
    "mov ia2_stackptr_1@GOTTPOFF(%rip), %r11\n"
    /* Read the new stack pointer from memory */
    "movq %fs:(%r11), %rsp\n"
    /* Preserve essential regs on stack */
    "pushq %rax\n"
    /* Scrub non-essential regs */
    "call __libia2_scrub_registers\n"
    /* Restore preserved regs */
    "popq %rax\n"
    /* Set PKRU to the caller's value */
    "movq %rax, %r10\n"
    "movq %rdx, %r11\n"
    "xorl %ecx, %ecx\n"
    "xorl %edx, %edx\n"
    "movl $0xfffffff0, %eax\n"
    "wrpkru\n"
    "movq %r10, %rax\n"
    "movq %r11, %rdx\n"
    "popq %r15\n"
    "popq %r14\n"
    "popq %r13\n"
    "popq %r12\n"
    "popq %rbx\n"
    "popq %rbp\n"
    /* Return to the caller */
    "ret\n"
    ".size __wrap_deflatePending, .-__wrap_deflatePending\n"
    ".previous\n"
);
asm(
    /* Wrapper for deflatePrime(int, int, int) -> int: */
    ".text\n"
    ".global __wrap_deflatePrime\n"
    ".type __wrap_deflatePrime, @function\n"
    "__wrap_deflatePrime:\n"
    "pushq %rbp\n"
    "movq %rsp, %rbp\n"
    "pushq %rbx\n"
    "pushq %r12\n"
    "pushq %r13\n"
    "pushq %r14\n"
    "pushq %r15\n"
    ASSERT_PKRU(0xfffffffffffffff0) "\n"
    /* Set PKRU to the intermediate value to move arguments */
    "movq %rcx, %r10\n"
    "movq %rdx, %r11\n"
    "xorl %ecx, %ecx\n"
    "xorl %edx, %edx\n"
    "movl $0xffffffc0, %eax\n"
    "wrpkru\n"
    "movq %r10, %rcx\n"
    "movq %r11, %rdx\n"
    /* Compute location to save old stack pointer (using r11) */
    "mov ia2_stackptr_1@GOTTPOFF(%rip), %r11\n"
    /* Write the old stack pointer to memory */
    "movq %rsp, %fs:(%r11)\n"
    /* Compute location to load new stack pointer (using r11) */
    "mov ia2_stackptr_2@GOTTPOFF(%rip), %r11\n"
    /* Read the new stack pointer from memory */
    "movq %fs:(%r11), %rsp\n"
    "subq $8, %rsp\n"
    /* Preserve essential regs on stack */
    "pushq %rdi\n"
    "pushq %rsi\n"
    "pushq %rdx\n"
    /* Scrub non-essential regs */
    "call __libia2_scrub_registers\n"
    /* Restore preserved regs */
    "popq %rdx\n"
    "popq %rsi\n"
    "popq %rdi\n"
    /* Set PKRU to the compartment's value */
    "movq %rcx, %r10\n"
    "movq %rdx, %r11\n"
    "xorl %ecx, %ecx\n"
    "xorl %edx, %edx\n"
    "movl $0xffffffcc, %eax\n"
    "wrpkru\n"
    "movq %r10, %rcx\n"
    "movq %r11, %rdx\n"
    /* Call wrapped function */
    "call deflatePrime\n"
    /* Set PKRU to the intermediate value to move arguments */
    "movq %rax, %r10\n"
    "movq %rdx, %r11\n"
    "xorl %ecx, %ecx\n"
    "xorl %edx, %edx\n"
    "movl $0xffffffc0, %eax\n"
    "wrpkru\n"
    "movq %r10, %rax\n"
    "movq %r11, %rdx\n"
    /* Free stack space used for stack args */
    "addq $8, %rsp\n"
    /* Compute location to save old stack pointer (using r11) */
    "mov ia2_stackptr_2@GOTTPOFF(%rip), %r11\n"
    /* Write the old stack pointer to memory */
    "movq %rsp, %fs:(%r11)\n"
    /* Compute location to load new stack pointer (using r11) */
    "mov ia2_stackptr_1@GOTTPOFF(%rip), %r11\n"
    /* Read the new stack pointer from memory */
    "movq %fs:(%r11), %rsp\n"
    /* Preserve essential regs on stack */
    "pushq %rax\n"
    /* Scrub non-essential regs */
    "call __libia2_scrub_registers\n"
    /* Restore preserved regs */
    "popq %rax\n"
    /* Set PKRU to the caller's value */
    "movq %rax, %r10\n"
    "movq %rdx, %r11\n"
    "xorl %ecx, %ecx\n"
    "xorl %edx, %edx\n"
    "movl $0xfffffff0, %eax\n"
    "wrpkru\n"
    "movq %r10, %rax\n"
    "movq %r11, %rdx\n"
    "popq %r15\n"
    "popq %r14\n"
    "popq %r13\n"
    "popq %r12\n"
    "popq %rbx\n"
    "popq %rbp\n"
    /* Return to the caller */
    "ret\n"
    ".size __wrap_deflatePrime, .-__wrap_deflatePrime\n"
    ".previous\n"
);
asm(
    /* Wrapper for deflateReset(int) -> int: */
    ".text\n"
    ".global __wrap_deflateReset\n"
    ".type __wrap_deflateReset, @function\n"
    "__wrap_deflateReset:\n"
    "pushq %rbp\n"
    "movq %rsp, %rbp\n"
    "pushq %rbx\n"
    "pushq %r12\n"
    "pushq %r13\n"
    "pushq %r14\n"
    "pushq %r15\n"
    ASSERT_PKRU(0xfffffffffffffff0) "\n"
    /* Set PKRU to the intermediate value to move arguments */
    "movq %rcx, %r10\n"
    "movq %rdx, %r11\n"
    "xorl %ecx, %ecx\n"
    "xorl %edx, %edx\n"
    "movl $0xffffffc0, %eax\n"
    "wrpkru\n"
    "movq %r10, %rcx\n"
    "movq %r11, %rdx\n"
    /* Compute location to save old stack pointer (using r11) */
    "mov ia2_stackptr_1@GOTTPOFF(%rip), %r11\n"
    /* Write the old stack pointer to memory */
    "movq %rsp, %fs:(%r11)\n"
    /* Compute location to load new stack pointer (using r11) */
    "mov ia2_stackptr_2@GOTTPOFF(%rip), %r11\n"
    /* Read the new stack pointer from memory */
    "movq %fs:(%r11), %rsp\n"
    "subq $8, %rsp\n"
    /* Preserve essential regs on stack */
    "pushq %rdi\n"
    /* Scrub non-essential regs */
    "call __libia2_scrub_registers\n"
    /* Restore preserved regs */
    "popq %rdi\n"
    /* Set PKRU to the compartment's value */
    "movq %rcx, %r10\n"
    "movq %rdx, %r11\n"
    "xorl %ecx, %ecx\n"
    "xorl %edx, %edx\n"
    "movl $0xffffffcc, %eax\n"
    "wrpkru\n"
    "movq %r10, %rcx\n"
    "movq %r11, %rdx\n"
    /* Call wrapped function */
    "call deflateReset\n"
    /* Set PKRU to the intermediate value to move arguments */
    "movq %rax, %r10\n"
    "movq %rdx, %r11\n"
    "xorl %ecx, %ecx\n"
    "xorl %edx, %edx\n"
    "movl $0xffffffc0, %eax\n"
    "wrpkru\n"
    "movq %r10, %rax\n"
    "movq %r11, %rdx\n"
    /* Free stack space used for stack args */
    "addq $8, %rsp\n"
    /* Compute location to save old stack pointer (using r11) */
    "mov ia2_stackptr_2@GOTTPOFF(%rip), %r11\n"
    /* Write the old stack pointer to memory */
    "movq %rsp, %fs:(%r11)\n"
    /* Compute location to load new stack pointer (using r11) */
    "mov ia2_stackptr_1@GOTTPOFF(%rip), %r11\n"
    /* Read the new stack pointer from memory */
    "movq %fs:(%r11), %rsp\n"
    /* Preserve essential regs on stack */
    "pushq %rax\n"
    /* Scrub non-essential regs */
    "call __libia2_scrub_registers\n"
    /* Restore preserved regs */
    "popq %rax\n"
    /* Set PKRU to the caller's value */
    "movq %rax, %r10\n"
    "movq %rdx, %r11\n"
    "xorl %ecx, %ecx\n"
    "xorl %edx, %edx\n"
    "movl $0xfffffff0, %eax\n"
    "wrpkru\n"
    "movq %r10, %rax\n"
    "movq %r11, %rdx\n"
    "popq %r15\n"
    "popq %r14\n"
    "popq %r13\n"
    "popq %r12\n"
    "popq %rbx\n"
    "popq %rbp\n"
    /* Return to the caller */
    "ret\n"
    ".size __wrap_deflateReset, .-__wrap_deflateReset\n"
    ".previous\n"
);
asm(
    /* Wrapper for deflateResetKeep(int) -> int: */
    ".text\n"
    ".global __wrap_deflateResetKeep\n"
    ".type __wrap_deflateResetKeep, @function\n"
    "__wrap_deflateResetKeep:\n"
    "pushq %rbp\n"
    "movq %rsp, %rbp\n"
    "pushq %rbx\n"
    "pushq %r12\n"
    "pushq %r13\n"
    "pushq %r14\n"
    "pushq %r15\n"
    ASSERT_PKRU(0xfffffffffffffff0) "\n"
    /* Set PKRU to the intermediate value to move arguments */
    "movq %rcx, %r10\n"
    "movq %rdx, %r11\n"
    "xorl %ecx, %ecx\n"
    "xorl %edx, %edx\n"
    "movl $0xffffffc0, %eax\n"
    "wrpkru\n"
    "movq %r10, %rcx\n"
    "movq %r11, %rdx\n"
    /* Compute location to save old stack pointer (using r11) */
    "mov ia2_stackptr_1@GOTTPOFF(%rip), %r11\n"
    /* Write the old stack pointer to memory */
    "movq %rsp, %fs:(%r11)\n"
    /* Compute location to load new stack pointer (using r11) */
    "mov ia2_stackptr_2@GOTTPOFF(%rip), %r11\n"
    /* Read the new stack pointer from memory */
    "movq %fs:(%r11), %rsp\n"
    "subq $8, %rsp\n"
    /* Preserve essential regs on stack */
    "pushq %rdi\n"
    /* Scrub non-essential regs */
    "call __libia2_scrub_registers\n"
    /* Restore preserved regs */
    "popq %rdi\n"
    /* Set PKRU to the compartment's value */
    "movq %rcx, %r10\n"
    "movq %rdx, %r11\n"
    "xorl %ecx, %ecx\n"
    "xorl %edx, %edx\n"
    "movl $0xffffffcc, %eax\n"
    "wrpkru\n"
    "movq %r10, %rcx\n"
    "movq %r11, %rdx\n"
    /* Call wrapped function */
    "call deflateResetKeep\n"
    /* Set PKRU to the intermediate value to move arguments */
    "movq %rax, %r10\n"
    "movq %rdx, %r11\n"
    "xorl %ecx, %ecx\n"
    "xorl %edx, %edx\n"
    "movl $0xffffffc0, %eax\n"
    "wrpkru\n"
    "movq %r10, %rax\n"
    "movq %r11, %rdx\n"
    /* Free stack space used for stack args */
    "addq $8, %rsp\n"
    /* Compute location to save old stack pointer (using r11) */
    "mov ia2_stackptr_2@GOTTPOFF(%rip), %r11\n"
    /* Write the old stack pointer to memory */
    "movq %rsp, %fs:(%r11)\n"
    /* Compute location to load new stack pointer (using r11) */
    "mov ia2_stackptr_1@GOTTPOFF(%rip), %r11\n"
    /* Read the new stack pointer from memory */
    "movq %fs:(%r11), %rsp\n"
    /* Preserve essential regs on stack */
    "pushq %rax\n"
    /* Scrub non-essential regs */
    "call __libia2_scrub_registers\n"
    /* Restore preserved regs */
    "popq %rax\n"
    /* Set PKRU to the caller's value */
    "movq %rax, %r10\n"
    "movq %rdx, %r11\n"
    "xorl %ecx, %ecx\n"
    "xorl %edx, %edx\n"
    "movl $0xfffffff0, %eax\n"
    "wrpkru\n"
    "movq %r10, %rax\n"
    "movq %r11, %rdx\n"
    "popq %r15\n"
    "popq %r14\n"
    "popq %r13\n"
    "popq %r12\n"
    "popq %rbx\n"
    "popq %rbp\n"
    /* Return to the caller */
    "ret\n"
    ".size __wrap_deflateResetKeep, .-__wrap_deflateResetKeep\n"
    ".previous\n"
);
asm(
    /* Wrapper for deflateSetDictionary(int, int, int) -> int: */
    ".text\n"
    ".global __wrap_deflateSetDictionary\n"
    ".type __wrap_deflateSetDictionary, @function\n"
    "__wrap_deflateSetDictionary:\n"
    "pushq %rbp\n"
    "movq %rsp, %rbp\n"
    "pushq %rbx\n"
    "pushq %r12\n"
    "pushq %r13\n"
    "pushq %r14\n"
    "pushq %r15\n"
    ASSERT_PKRU(0xfffffffffffffff0) "\n"
    /* Set PKRU to the intermediate value to move arguments */
    "movq %rcx, %r10\n"
    "movq %rdx, %r11\n"
    "xorl %ecx, %ecx\n"
    "xorl %edx, %edx\n"
    "movl $0xffffffc0, %eax\n"
    "wrpkru\n"
    "movq %r10, %rcx\n"
    "movq %r11, %rdx\n"
    /* Compute location to save old stack pointer (using r11) */
    "mov ia2_stackptr_1@GOTTPOFF(%rip), %r11\n"
    /* Write the old stack pointer to memory */
    "movq %rsp, %fs:(%r11)\n"
    /* Compute location to load new stack pointer (using r11) */
    "mov ia2_stackptr_2@GOTTPOFF(%rip), %r11\n"
    /* Read the new stack pointer from memory */
    "movq %fs:(%r11), %rsp\n"
    "subq $8, %rsp\n"
    /* Preserve essential regs on stack */
    "pushq %rdi\n"
    "pushq %rsi\n"
    "pushq %rdx\n"
    /* Scrub non-essential regs */
    "call __libia2_scrub_registers\n"
    /* Restore preserved regs */
    "popq %rdx\n"
    "popq %rsi\n"
    "popq %rdi\n"
    /* Set PKRU to the compartment's value */
    "movq %rcx, %r10\n"
    "movq %rdx, %r11\n"
    "xorl %ecx, %ecx\n"
    "xorl %edx, %edx\n"
    "movl $0xffffffcc, %eax\n"
    "wrpkru\n"
    "movq %r10, %rcx\n"
    "movq %r11, %rdx\n"
    /* Call wrapped function */
    "call deflateSetDictionary\n"
    /* Set PKRU to the intermediate value to move arguments */
    "movq %rax, %r10\n"
    "movq %rdx, %r11\n"
    "xorl %ecx, %ecx\n"
    "xorl %edx, %edx\n"
    "movl $0xffffffc0, %eax\n"
    "wrpkru\n"
    "movq %r10, %rax\n"
    "movq %r11, %rdx\n"
    /* Free stack space used for stack args */
    "addq $8, %rsp\n"
    /* Compute location to save old stack pointer (using r11) */
    "mov ia2_stackptr_2@GOTTPOFF(%rip), %r11\n"
    /* Write the old stack pointer to memory */
    "movq %rsp, %fs:(%r11)\n"
    /* Compute location to load new stack pointer (using r11) */
    "mov ia2_stackptr_1@GOTTPOFF(%rip), %r11\n"
    /* Read the new stack pointer from memory */
    "movq %fs:(%r11), %rsp\n"
    /* Preserve essential regs on stack */
    "pushq %rax\n"
    /* Scrub non-essential regs */
    "call __libia2_scrub_registers\n"
    /* Restore preserved regs */
    "popq %rax\n"
    /* Set PKRU to the caller's value */
    "movq %rax, %r10\n"
    "movq %rdx, %r11\n"
    "xorl %ecx, %ecx\n"
    "xorl %edx, %edx\n"
    "movl $0xfffffff0, %eax\n"
    "wrpkru\n"
    "movq %r10, %rax\n"
    "movq %r11, %rdx\n"
    "popq %r15\n"
    "popq %r14\n"
    "popq %r13\n"
    "popq %r12\n"
    "popq %rbx\n"
    "popq %rbp\n"
    /* Return to the caller */
    "ret\n"
    ".size __wrap_deflateSetDictionary, .-__wrap_deflateSetDictionary\n"
    ".previous\n"
);
asm(
    /* Wrapper for deflateSetHeader(int, int) -> int: */
    ".text\n"
    ".global __wrap_deflateSetHeader\n"
    ".type __wrap_deflateSetHeader, @function\n"
    "__wrap_deflateSetHeader:\n"
    "pushq %rbp\n"
    "movq %rsp, %rbp\n"
    "pushq %rbx\n"
    "pushq %r12\n"
    "pushq %r13\n"
    "pushq %r14\n"
    "pushq %r15\n"
    ASSERT_PKRU(0xfffffffffffffff0) "\n"
    /* Set PKRU to the intermediate value to move arguments */
    "movq %rcx, %r10\n"
    "movq %rdx, %r11\n"
    "xorl %ecx, %ecx\n"
    "xorl %edx, %edx\n"
    "movl $0xffffffc0, %eax\n"
    "wrpkru\n"
    "movq %r10, %rcx\n"
    "movq %r11, %rdx\n"
    /* Compute location to save old stack pointer (using r11) */
    "mov ia2_stackptr_1@GOTTPOFF(%rip), %r11\n"
    /* Write the old stack pointer to memory */
    "movq %rsp, %fs:(%r11)\n"
    /* Compute location to load new stack pointer (using r11) */
    "mov ia2_stackptr_2@GOTTPOFF(%rip), %r11\n"
    /* Read the new stack pointer from memory */
    "movq %fs:(%r11), %rsp\n"
    "subq $8, %rsp\n"
    /* Preserve essential regs on stack */
    "pushq %rdi\n"
    "pushq %rsi\n"
    /* Scrub non-essential regs */
    "call __libia2_scrub_registers\n"
    /* Restore preserved regs */
    "popq %rsi\n"
    "popq %rdi\n"
    /* Set PKRU to the compartment's value */
    "movq %rcx, %r10\n"
    "movq %rdx, %r11\n"
    "xorl %ecx, %ecx\n"
    "xorl %edx, %edx\n"
    "movl $0xffffffcc, %eax\n"
    "wrpkru\n"
    "movq %r10, %rcx\n"
    "movq %r11, %rdx\n"
    /* Call wrapped function */
    "call deflateSetHeader\n"
    /* Set PKRU to the intermediate value to move arguments */
    "movq %rax, %r10\n"
    "movq %rdx, %r11\n"
    "xorl %ecx, %ecx\n"
    "xorl %edx, %edx\n"
    "movl $0xffffffc0, %eax\n"
    "wrpkru\n"
    "movq %r10, %rax\n"
    "movq %r11, %rdx\n"
    /* Free stack space used for stack args */
    "addq $8, %rsp\n"
    /* Compute location to save old stack pointer (using r11) */
    "mov ia2_stackptr_2@GOTTPOFF(%rip), %r11\n"
    /* Write the old stack pointer to memory */
    "movq %rsp, %fs:(%r11)\n"
    /* Compute location to load new stack pointer (using r11) */
    "mov ia2_stackptr_1@GOTTPOFF(%rip), %r11\n"
    /* Read the new stack pointer from memory */
    "movq %fs:(%r11), %rsp\n"
    /* Preserve essential regs on stack */
    "pushq %rax\n"
    /* Scrub non-essential regs */
    "call __libia2_scrub_registers\n"
    /* Restore preserved regs */
    "popq %rax\n"
    /* Set PKRU to the caller's value */
    "movq %rax, %r10\n"
    "movq %rdx, %r11\n"
    "xorl %ecx, %ecx\n"
    "xorl %edx, %edx\n"
    "movl $0xfffffff0, %eax\n"
    "wrpkru\n"
    "movq %r10, %rax\n"
    "movq %r11, %rdx\n"
    "popq %r15\n"
    "popq %r14\n"
    "popq %r13\n"
    "popq %r12\n"
    "popq %rbx\n"
    "popq %rbp\n"
    /* Return to the caller */
    "ret\n"
    ".size __wrap_deflateSetHeader, .-__wrap_deflateSetHeader\n"
    ".previous\n"
);
asm(
    /* Wrapper for deflateTune(int, int, int, int, int) -> int: */
    ".text\n"
    ".global __wrap_deflateTune\n"
    ".type __wrap_deflateTune, @function\n"
    "__wrap_deflateTune:\n"
    "pushq %rbp\n"
    "movq %rsp, %rbp\n"
    "pushq %rbx\n"
    "pushq %r12\n"
    "pushq %r13\n"
    "pushq %r14\n"
    "pushq %r15\n"
    ASSERT_PKRU(0xfffffffffffffff0) "\n"
    /* Set PKRU to the intermediate value to move arguments */
    "movq %rcx, %r10\n"
    "movq %rdx, %r11\n"
    "xorl %ecx, %ecx\n"
    "xorl %edx, %edx\n"
    "movl $0xffffffc0, %eax\n"
    "wrpkru\n"
    "movq %r10, %rcx\n"
    "movq %r11, %rdx\n"
    /* Compute location to save old stack pointer (using r11) */
    "mov ia2_stackptr_1@GOTTPOFF(%rip), %r11\n"
    /* Write the old stack pointer to memory */
    "movq %rsp, %fs:(%r11)\n"
    /* Compute location to load new stack pointer (using r11) */
    "mov ia2_stackptr_2@GOTTPOFF(%rip), %r11\n"
    /* Read the new stack pointer from memory */
    "movq %fs:(%r11), %rsp\n"
    "subq $8, %rsp\n"
    /* Preserve essential regs on stack */
    "pushq %rdi\n"
    "pushq %rsi\n"
    "pushq %rdx\n"
    "pushq %rcx\n"
    "pushq %r8\n"
    /* Scrub non-essential regs */
    "call __libia2_scrub_registers\n"
    /* Restore preserved regs */
    "popq %r8\n"
    "popq %rcx\n"
    "popq %rdx\n"
    "popq %rsi\n"
    "popq %rdi\n"
    /* Set PKRU to the compartment's value */
    "movq %rcx, %r10\n"
    "movq %rdx, %r11\n"
    "xorl %ecx, %ecx\n"
    "xorl %edx, %edx\n"
    "movl $0xffffffcc, %eax\n"
    "wrpkru\n"
    "movq %r10, %rcx\n"
    "movq %r11, %rdx\n"
    /* Call wrapped function */
    "call deflateTune\n"
    /* Set PKRU to the intermediate value to move arguments */
    "movq %rax, %r10\n"
    "movq %rdx, %r11\n"
    "xorl %ecx, %ecx\n"
    "xorl %edx, %edx\n"
    "movl $0xffffffc0, %eax\n"
    "wrpkru\n"
    "movq %r10, %rax\n"
    "movq %r11, %rdx\n"
    /* Free stack space used for stack args */
    "addq $8, %rsp\n"
    /* Compute location to save old stack pointer (using r11) */
    "mov ia2_stackptr_2@GOTTPOFF(%rip), %r11\n"
    /* Write the old stack pointer to memory */
    "movq %rsp, %fs:(%r11)\n"
    /* Compute location to load new stack pointer (using r11) */
    "mov ia2_stackptr_1@GOTTPOFF(%rip), %r11\n"
    /* Read the new stack pointer from memory */
    "movq %fs:(%r11), %rsp\n"
    /* Preserve essential regs on stack */
    "pushq %rax\n"
    /* Scrub non-essential regs */
    "call __libia2_scrub_registers\n"
    /* Restore preserved regs */
    "popq %rax\n"
    /* Set PKRU to the caller's value */
    "movq %rax, %r10\n"
    "movq %rdx, %r11\n"
    "xorl %ecx, %ecx\n"
    "xorl %edx, %edx\n"
    "movl $0xfffffff0, %eax\n"
    "wrpkru\n"
    "movq %r10, %rax\n"
    "movq %r11, %rdx\n"
    "popq %r15\n"
    "popq %r14\n"
    "popq %r13\n"
    "popq %r12\n"
    "popq %rbx\n"
    "popq %rbp\n"
    /* Return to the caller */
    "ret\n"
    ".size __wrap_deflateTune, .-__wrap_deflateTune\n"
    ".previous\n"
);
asm(
    /* Wrapper for deflateUsed(int, int) -> int: */
    ".text\n"
    ".global __wrap_deflateUsed\n"
    ".type __wrap_deflateUsed, @function\n"
    "__wrap_deflateUsed:\n"
    "pushq %rbp\n"
    "movq %rsp, %rbp\n"
    "pushq %rbx\n"
    "pushq %r12\n"
    "pushq %r13\n"
    "pushq %r14\n"
    "pushq %r15\n"
    ASSERT_PKRU(0xfffffffffffffff0) "\n"
    /* Set PKRU to the intermediate value to move arguments */
    "movq %rcx, %r10\n"
    "movq %rdx, %r11\n"
    "xorl %ecx, %ecx\n"
    "xorl %edx, %edx\n"
    "movl $0xffffffc0, %eax\n"
    "wrpkru\n"
    "movq %r10, %rcx\n"
    "movq %r11, %rdx\n"
    /* Compute location to save old stack pointer (using r11) */
    "mov ia2_stackptr_1@GOTTPOFF(%rip), %r11\n"
    /* Write the old stack pointer to memory */
    "movq %rsp, %fs:(%r11)\n"
    /* Compute location to load new stack pointer (using r11) */
    "mov ia2_stackptr_2@GOTTPOFF(%rip), %r11\n"
    /* Read the new stack pointer from memory */
    "movq %fs:(%r11), %rsp\n"
    "subq $8, %rsp\n"
    /* Preserve essential regs on stack */
    "pushq %rdi\n"
    "pushq %rsi\n"
    /* Scrub non-essential regs */
    "call __libia2_scrub_registers\n"
    /* Restore preserved regs */
    "popq %rsi\n"
    "popq %rdi\n"
    /* Set PKRU to the compartment's value */
    "movq %rcx, %r10\n"
    "movq %rdx, %r11\n"
    "xorl %ecx, %ecx\n"
    "xorl %edx, %edx\n"
    "movl $0xffffffcc, %eax\n"
    "wrpkru\n"
    "movq %r10, %rcx\n"
    "movq %r11, %rdx\n"
    /* Call wrapped function */
    "call deflateUsed\n"
    /* Set PKRU to the intermediate value to move arguments */
    "movq %rax, %r10\n"
    "movq %rdx, %r11\n"
    "xorl %ecx, %ecx\n"
    "xorl %edx, %edx\n"
    "movl $0xffffffc0, %eax\n"
    "wrpkru\n"
    "movq %r10, %rax\n"
    "movq %r11, %rdx\n"
    /* Free stack space used for stack args */
    "addq $8, %rsp\n"
    /* Compute location to save old stack pointer (using r11) */
    "mov ia2_stackptr_2@GOTTPOFF(%rip), %r11\n"
    /* Write the old stack pointer to memory */
    "movq %rsp, %fs:(%r11)\n"
    /* Compute location to load new stack pointer (using r11) */
    "mov ia2_stackptr_1@GOTTPOFF(%rip), %r11\n"
    /* Read the new stack pointer from memory */
    "movq %fs:(%r11), %rsp\n"
    /* Preserve essential regs on stack */
    "pushq %rax\n"
    /* Scrub non-essential regs */
    "call __libia2_scrub_registers\n"
    /* Restore preserved regs */
    "popq %rax\n"
    /* Set PKRU to the caller's value */
    "movq %rax, %r10\n"
    "movq %rdx, %r11\n"
    "xorl %ecx, %ecx\n"
    "xorl %edx, %edx\n"
    "movl $0xfffffff0, %eax\n"
    "wrpkru\n"
    "movq %r10, %rax\n"
    "movq %r11, %rdx\n"
    "popq %r15\n"
    "popq %r14\n"
    "popq %r13\n"
    "popq %r12\n"
    "popq %rbx\n"
    "popq %rbp\n"
    /* Return to the caller */
    "ret\n"
    ".size __wrap_deflateUsed, .-__wrap_deflateUsed\n"
    ".previous\n"
);
asm(
    /* Wrapper for get_crc_table() -> int: */
    ".text\n"
    ".global __wrap_get_crc_table\n"
    ".type __wrap_get_crc_table, @function\n"
    "__wrap_get_crc_table:\n"
    "pushq %rbp\n"
    "movq %rsp, %rbp\n"
    "pushq %rbx\n"
    "pushq %r12\n"
    "pushq %r13\n"
    "pushq %r14\n"
    "pushq %r15\n"
    ASSERT_PKRU(0xfffffffffffffff0) "\n"
    /* Set PKRU to the intermediate value to move arguments */
    "movq %rcx, %r10\n"
    "movq %rdx, %r11\n"
    "xorl %ecx, %ecx\n"
    "xorl %edx, %edx\n"
    "movl $0xffffffc0, %eax\n"
    "wrpkru\n"
    "movq %r10, %rcx\n"
    "movq %r11, %rdx\n"
    /* Compute location to save old stack pointer (using r11) */
    "mov ia2_stackptr_1@GOTTPOFF(%rip), %r11\n"
    /* Write the old stack pointer to memory */
    "movq %rsp, %fs:(%r11)\n"
    /* Compute location to load new stack pointer (using r11) */
    "mov ia2_stackptr_2@GOTTPOFF(%rip), %r11\n"
    /* Read the new stack pointer from memory */
    "movq %fs:(%r11), %rsp\n"
    "subq $8, %rsp\n"
    /* Scrub non-essential regs */
    "call __libia2_scrub_registers\n"
    /* Set PKRU to the compartment's value */
    "movq %rcx, %r10\n"
    "movq %rdx, %r11\n"
    "xorl %ecx, %ecx\n"
    "xorl %edx, %edx\n"
    "movl $0xffffffcc, %eax\n"
    "wrpkru\n"
    "movq %r10, %rcx\n"
    "movq %r11, %rdx\n"
    /* Call wrapped function */
    "call get_crc_table\n"
    /* Set PKRU to the intermediate value to move arguments */
    "movq %rax, %r10\n"
    "movq %rdx, %r11\n"
    "xorl %ecx, %ecx\n"
    "xorl %edx, %edx\n"
    "movl $0xffffffc0, %eax\n"
    "wrpkru\n"
    "movq %r10, %rax\n"
    "movq %r11, %rdx\n"
    /* Free stack space used for stack args */
    "addq $8, %rsp\n"
    /* Compute location to save old stack pointer (using r11) */
    "mov ia2_stackptr_2@GOTTPOFF(%rip), %r11\n"
    /* Write the old stack pointer to memory */
    "movq %rsp, %fs:(%r11)\n"
    /* Compute location to load new stack pointer (using r11) */
    "mov ia2_stackptr_1@GOTTPOFF(%rip), %r11\n"
    /* Read the new stack pointer from memory */
    "movq %fs:(%r11), %rsp\n"
    /* Preserve essential regs on stack */
    "pushq %rax\n"
    /* Scrub non-essential regs */
    "call __libia2_scrub_registers\n"
    /* Restore preserved regs */
    "popq %rax\n"
    /* Set PKRU to the caller's value */
    "movq %rax, %r10\n"
    "movq %rdx, %r11\n"
    "xorl %ecx, %ecx\n"
    "xorl %edx, %edx\n"
    "movl $0xfffffff0, %eax\n"
    "wrpkru\n"
    "movq %r10, %rax\n"
    "movq %r11, %rdx\n"
    "popq %r15\n"
    "popq %r14\n"
    "popq %r13\n"
    "popq %r12\n"
    "popq %rbx\n"
    "popq %rbp\n"
    /* Return to the caller */
    "ret\n"
    ".size __wrap_get_crc_table, .-__wrap_get_crc_table\n"
    ".previous\n"
);
asm(
    /* Wrapper for gzbuffer(int, int) -> int: */
    ".text\n"
    ".global __wrap_gzbuffer\n"
    ".type __wrap_gzbuffer, @function\n"
    "__wrap_gzbuffer:\n"
    "pushq %rbp\n"
    "movq %rsp, %rbp\n"
    "pushq %rbx\n"
    "pushq %r12\n"
    "pushq %r13\n"
    "pushq %r14\n"
    "pushq %r15\n"
    ASSERT_PKRU(0xfffffffffffffff0) "\n"
    /* Set PKRU to the intermediate value to move arguments */
    "movq %rcx, %r10\n"
    "movq %rdx, %r11\n"
    "xorl %ecx, %ecx\n"
    "xorl %edx, %edx\n"
    "movl $0xffffffc0, %eax\n"
    "wrpkru\n"
    "movq %r10, %rcx\n"
    "movq %r11, %rdx\n"
    /* Compute location to save old stack pointer (using r11) */
    "mov ia2_stackptr_1@GOTTPOFF(%rip), %r11\n"
    /* Write the old stack pointer to memory */
    "movq %rsp, %fs:(%r11)\n"
    /* Compute location to load new stack pointer (using r11) */
    "mov ia2_stackptr_2@GOTTPOFF(%rip), %r11\n"
    /* Read the new stack pointer from memory */
    "movq %fs:(%r11), %rsp\n"
    "subq $8, %rsp\n"
    /* Preserve essential regs on stack */
    "pushq %rdi\n"
    "pushq %rsi\n"
    /* Scrub non-essential regs */
    "call __libia2_scrub_registers\n"
    /* Restore preserved regs */
    "popq %rsi\n"
    "popq %rdi\n"
    /* Set PKRU to the compartment's value */
    "movq %rcx, %r10\n"
    "movq %rdx, %r11\n"
    "xorl %ecx, %ecx\n"
    "xorl %edx, %edx\n"
    "movl $0xffffffcc, %eax\n"
    "wrpkru\n"
    "movq %r10, %rcx\n"
    "movq %r11, %rdx\n"
    /* Call wrapped function */
    "call gzbuffer\n"
    /* Set PKRU to the intermediate value to move arguments */
    "movq %rax, %r10\n"
    "movq %rdx, %r11\n"
    "xorl %ecx, %ecx\n"
    "xorl %edx, %edx\n"
    "movl $0xffffffc0, %eax\n"
    "wrpkru\n"
    "movq %r10, %rax\n"
    "movq %r11, %rdx\n"
    /* Free stack space used for stack args */
    "addq $8, %rsp\n"
    /* Compute location to save old stack pointer (using r11) */
    "mov ia2_stackptr_2@GOTTPOFF(%rip), %r11\n"
    /* Write the old stack pointer to memory */
    "movq %rsp, %fs:(%r11)\n"
    /* Compute location to load new stack pointer (using r11) */
    "mov ia2_stackptr_1@GOTTPOFF(%rip), %r11\n"
    /* Read the new stack pointer from memory */
    "movq %fs:(%r11), %rsp\n"
    /* Preserve essential regs on stack */
    "pushq %rax\n"
    /* Scrub non-essential regs */
    "call __libia2_scrub_registers\n"
    /* Restore preserved regs */
    "popq %rax\n"
    /* Set PKRU to the caller's value */
    "movq %rax, %r10\n"
    "movq %rdx, %r11\n"
    "xorl %ecx, %ecx\n"
    "xorl %edx, %edx\n"
    "movl $0xfffffff0, %eax\n"
    "wrpkru\n"
    "movq %r10, %rax\n"
    "movq %r11, %rdx\n"
    "popq %r15\n"
    "popq %r14\n"
    "popq %r13\n"
    "popq %r12\n"
    "popq %rbx\n"
    "popq %rbp\n"
    /* Return to the caller */
    "ret\n"
    ".size __wrap_gzbuffer, .-__wrap_gzbuffer\n"
    ".previous\n"
);
asm(
    /* Wrapper for gzclearerr(int): */
    ".text\n"
    ".global __wrap_gzclearerr\n"
    ".type __wrap_gzclearerr, @function\n"
    "__wrap_gzclearerr:\n"
    "pushq %rbp\n"
    "movq %rsp, %rbp\n"
    "pushq %rbx\n"
    "pushq %r12\n"
    "pushq %r13\n"
    "pushq %r14\n"
    "pushq %r15\n"
    ASSERT_PKRU(0xfffffffffffffff0) "\n"
    /* Set PKRU to the intermediate value to move arguments */
    "movq %rcx, %r10\n"
    "movq %rdx, %r11\n"
    "xorl %ecx, %ecx\n"
    "xorl %edx, %edx\n"
    "movl $0xffffffc0, %eax\n"
    "wrpkru\n"
    "movq %r10, %rcx\n"
    "movq %r11, %rdx\n"
    /* Compute location to save old stack pointer (using r11) */
    "mov ia2_stackptr_1@GOTTPOFF(%rip), %r11\n"
    /* Write the old stack pointer to memory */
    "movq %rsp, %fs:(%r11)\n"
    /* Compute location to load new stack pointer (using r11) */
    "mov ia2_stackptr_2@GOTTPOFF(%rip), %r11\n"
    /* Read the new stack pointer from memory */
    "movq %fs:(%r11), %rsp\n"
    "subq $8, %rsp\n"
    /* Preserve essential regs on stack */
    "pushq %rdi\n"
    /* Scrub non-essential regs */
    "call __libia2_scrub_registers\n"
    /* Restore preserved regs */
    "popq %rdi\n"
    /* Set PKRU to the compartment's value */
    "movq %rcx, %r10\n"
    "movq %rdx, %r11\n"
    "xorl %ecx, %ecx\n"
    "xorl %edx, %edx\n"
    "movl $0xffffffcc, %eax\n"
    "wrpkru\n"
    "movq %r10, %rcx\n"
    "movq %r11, %rdx\n"
    /* Call wrapped function */
    "call gzclearerr\n"
    /* Set PKRU to the intermediate value to move arguments */
    "movq %rax, %r10\n"
    "movq %rdx, %r11\n"
    "xorl %ecx, %ecx\n"
    "xorl %edx, %edx\n"
    "movl $0xffffffc0, %eax\n"
    "wrpkru\n"
    "movq %r10, %rax\n"
    "movq %r11, %rdx\n"
    /* Free stack space used for stack args */
    "addq $8, %rsp\n"
    /* Compute location to save old stack pointer (using r11) */
    "mov ia2_stackptr_2@GOTTPOFF(%rip), %r11\n"
    /* Write the old stack pointer to memory */
    "movq %rsp, %fs:(%r11)\n"
    /* Compute location to load new stack pointer (using r11) */
    "mov ia2_stackptr_1@GOTTPOFF(%rip), %r11\n"
    /* Read the new stack pointer from memory */
    "movq %fs:(%r11), %rsp\n"
    /* Scrub non-essential regs */
    "call __libia2_scrub_registers\n"
    /* Set PKRU to the caller's value */
    "movq %rax, %r10\n"
    "movq %rdx, %r11\n"
    "xorl %ecx, %ecx\n"
    "xorl %edx, %edx\n"
    "movl $0xfffffff0, %eax\n"
    "wrpkru\n"
    "movq %r10, %rax\n"
    "movq %r11, %rdx\n"
    "popq %r15\n"
    "popq %r14\n"
    "popq %r13\n"
    "popq %r12\n"
    "popq %rbx\n"
    "popq %rbp\n"
    /* Return to the caller */
    "ret\n"
    ".size __wrap_gzclearerr, .-__wrap_gzclearerr\n"
    ".previous\n"
);
asm(
    /* Wrapper for gzclose(int) -> int: */
    ".text\n"
    ".global __wrap_gzclose\n"
    ".type __wrap_gzclose, @function\n"
    "__wrap_gzclose:\n"
    "pushq %rbp\n"
    "movq %rsp, %rbp\n"
    "pushq %rbx\n"
    "pushq %r12\n"
    "pushq %r13\n"
    "pushq %r14\n"
    "pushq %r15\n"
    ASSERT_PKRU(0xfffffffffffffff0) "\n"
    /* Set PKRU to the intermediate value to move arguments */
    "movq %rcx, %r10\n"
    "movq %rdx, %r11\n"
    "xorl %ecx, %ecx\n"
    "xorl %edx, %edx\n"
    "movl $0xffffffc0, %eax\n"
    "wrpkru\n"
    "movq %r10, %rcx\n"
    "movq %r11, %rdx\n"
    /* Compute location to save old stack pointer (using r11) */
    "mov ia2_stackptr_1@GOTTPOFF(%rip), %r11\n"
    /* Write the old stack pointer to memory */
    "movq %rsp, %fs:(%r11)\n"
    /* Compute location to load new stack pointer (using r11) */
    "mov ia2_stackptr_2@GOTTPOFF(%rip), %r11\n"
    /* Read the new stack pointer from memory */
    "movq %fs:(%r11), %rsp\n"
    "subq $8, %rsp\n"
    /* Preserve essential regs on stack */
    "pushq %rdi\n"
    /* Scrub non-essential regs */
    "call __libia2_scrub_registers\n"
    /* Restore preserved regs */
    "popq %rdi\n"
    /* Set PKRU to the compartment's value */
    "movq %rcx, %r10\n"
    "movq %rdx, %r11\n"
    "xorl %ecx, %ecx\n"
    "xorl %edx, %edx\n"
    "movl $0xffffffcc, %eax\n"
    "wrpkru\n"
    "movq %r10, %rcx\n"
    "movq %r11, %rdx\n"
    /* Call wrapped function */
    "call gzclose\n"
    /* Set PKRU to the intermediate value to move arguments */
    "movq %rax, %r10\n"
    "movq %rdx, %r11\n"
    "xorl %ecx, %ecx\n"
    "xorl %edx, %edx\n"
    "movl $0xffffffc0, %eax\n"
    "wrpkru\n"
    "movq %r10, %rax\n"
    "movq %r11, %rdx\n"
    /* Free stack space used for stack args */
    "addq $8, %rsp\n"
    /* Compute location to save old stack pointer (using r11) */
    "mov ia2_stackptr_2@GOTTPOFF(%rip), %r11\n"
    /* Write the old stack pointer to memory */
    "movq %rsp, %fs:(%r11)\n"
    /* Compute location to load new stack pointer (using r11) */
    "mov ia2_stackptr_1@GOTTPOFF(%rip), %r11\n"
    /* Read the new stack pointer from memory */
    "movq %fs:(%r11), %rsp\n"
    /* Preserve essential regs on stack */
    "pushq %rax\n"
    /* Scrub non-essential regs */
    "call __libia2_scrub_registers\n"
    /* Restore preserved regs */
    "popq %rax\n"
    /* Set PKRU to the caller's value */
    "movq %rax, %r10\n"
    "movq %rdx, %r11\n"
    "xorl %ecx, %ecx\n"
    "xorl %edx, %edx\n"
    "movl $0xfffffff0, %eax\n"
    "wrpkru\n"
    "movq %r10, %rax\n"
    "movq %r11, %rdx\n"
    "popq %r15\n"
    "popq %r14\n"
    "popq %r13\n"
    "popq %r12\n"
    "popq %rbx\n"
    "popq %rbp\n"
    /* Return to the caller */
    "ret\n"
    ".size __wrap_gzclose, .-__wrap_gzclose\n"
    ".previous\n"
);
asm(
    /* Wrapper for gzclose_r(int) -> int: */
    ".text\n"
    ".global __wrap_gzclose_r\n"
    ".type __wrap_gzclose_r, @function\n"
    "__wrap_gzclose_r:\n"
    "pushq %rbp\n"
    "movq %rsp, %rbp\n"
    "pushq %rbx\n"
    "pushq %r12\n"
    "pushq %r13\n"
    "pushq %r14\n"
    "pushq %r15\n"
    ASSERT_PKRU(0xfffffffffffffff0) "\n"
    /* Set PKRU to the intermediate value to move arguments */
    "movq %rcx, %r10\n"
    "movq %rdx, %r11\n"
    "xorl %ecx, %ecx\n"
    "xorl %edx, %edx\n"
    "movl $0xffffffc0, %eax\n"
    "wrpkru\n"
    "movq %r10, %rcx\n"
    "movq %r11, %rdx\n"
    /* Compute location to save old stack pointer (using r11) */
    "mov ia2_stackptr_1@GOTTPOFF(%rip), %r11\n"
    /* Write the old stack pointer to memory */
    "movq %rsp, %fs:(%r11)\n"
    /* Compute location to load new stack pointer (using r11) */
    "mov ia2_stackptr_2@GOTTPOFF(%rip), %r11\n"
    /* Read the new stack pointer from memory */
    "movq %fs:(%r11), %rsp\n"
    "subq $8, %rsp\n"
    /* Preserve essential regs on stack */
    "pushq %rdi\n"
    /* Scrub non-essential regs */
    "call __libia2_scrub_registers\n"
    /* Restore preserved regs */
    "popq %rdi\n"
    /* Set PKRU to the compartment's value */
    "movq %rcx, %r10\n"
    "movq %rdx, %r11\n"
    "xorl %ecx, %ecx\n"
    "xorl %edx, %edx\n"
    "movl $0xffffffcc, %eax\n"
    "wrpkru\n"
    "movq %r10, %rcx\n"
    "movq %r11, %rdx\n"
    /* Call wrapped function */
    "call gzclose_r\n"
    /* Set PKRU to the intermediate value to move arguments */
    "movq %rax, %r10\n"
    "movq %rdx, %r11\n"
    "xorl %ecx, %ecx\n"
    "xorl %edx, %edx\n"
    "movl $0xffffffc0, %eax\n"
    "wrpkru\n"
    "movq %r10, %rax\n"
    "movq %r11, %rdx\n"
    /* Free stack space used for stack args */
    "addq $8, %rsp\n"
    /* Compute location to save old stack pointer (using r11) */
    "mov ia2_stackptr_2@GOTTPOFF(%rip), %r11\n"
    /* Write the old stack pointer to memory */
    "movq %rsp, %fs:(%r11)\n"
    /* Compute location to load new stack pointer (using r11) */
    "mov ia2_stackptr_1@GOTTPOFF(%rip), %r11\n"
    /* Read the new stack pointer from memory */
    "movq %fs:(%r11), %rsp\n"
    /* Preserve essential regs on stack */
    "pushq %rax\n"
    /* Scrub non-essential regs */
    "call __libia2_scrub_registers\n"
    /* Restore preserved regs */
    "popq %rax\n"
    /* Set PKRU to the caller's value */
    "movq %rax, %r10\n"
    "movq %rdx, %r11\n"
    "xorl %ecx, %ecx\n"
    "xorl %edx, %edx\n"
    "movl $0xfffffff0, %eax\n"
    "wrpkru\n"
    "movq %r10, %rax\n"
    "movq %r11, %rdx\n"
    "popq %r15\n"
    "popq %r14\n"
    "popq %r13\n"
    "popq %r12\n"
    "popq %rbx\n"
    "popq %rbp\n"
    /* Return to the caller */
    "ret\n"
    ".size __wrap_gzclose_r, .-__wrap_gzclose_r\n"
    ".previous\n"
);
asm(
    /* Wrapper for gzclose_w(int) -> int: */
    ".text\n"
    ".global __wrap_gzclose_w\n"
    ".type __wrap_gzclose_w, @function\n"
    "__wrap_gzclose_w:\n"
    "pushq %rbp\n"
    "movq %rsp, %rbp\n"
    "pushq %rbx\n"
    "pushq %r12\n"
    "pushq %r13\n"
    "pushq %r14\n"
    "pushq %r15\n"
    ASSERT_PKRU(0xfffffffffffffff0) "\n"
    /* Set PKRU to the intermediate value to move arguments */
    "movq %rcx, %r10\n"
    "movq %rdx, %r11\n"
    "xorl %ecx, %ecx\n"
    "xorl %edx, %edx\n"
    "movl $0xffffffc0, %eax\n"
    "wrpkru\n"
    "movq %r10, %rcx\n"
    "movq %r11, %rdx\n"
    /* Compute location to save old stack pointer (using r11) */
    "mov ia2_stackptr_1@GOTTPOFF(%rip), %r11\n"
    /* Write the old stack pointer to memory */
    "movq %rsp, %fs:(%r11)\n"
    /* Compute location to load new stack pointer (using r11) */
    "mov ia2_stackptr_2@GOTTPOFF(%rip), %r11\n"
    /* Read the new stack pointer from memory */
    "movq %fs:(%r11), %rsp\n"
    "subq $8, %rsp\n"
    /* Preserve essential regs on stack */
    "pushq %rdi\n"
    /* Scrub non-essential regs */
    "call __libia2_scrub_registers\n"
    /* Restore preserved regs */
    "popq %rdi\n"
    /* Set PKRU to the compartment's value */
    "movq %rcx, %r10\n"
    "movq %rdx, %r11\n"
    "xorl %ecx, %ecx\n"
    "xorl %edx, %edx\n"
    "movl $0xffffffcc, %eax\n"
    "wrpkru\n"
    "movq %r10, %rcx\n"
    "movq %r11, %rdx\n"
    /* Call wrapped function */
    "call gzclose_w\n"
    /* Set PKRU to the intermediate value to move arguments */
    "movq %rax, %r10\n"
    "movq %rdx, %r11\n"
    "xorl %ecx, %ecx\n"
    "xorl %edx, %edx\n"
    "movl $0xffffffc0, %eax\n"
    "wrpkru\n"
    "movq %r10, %rax\n"
    "movq %r11, %rdx\n"
    /* Free stack space used for stack args */
    "addq $8, %rsp\n"
    /* Compute location to save old stack pointer (using r11) */
    "mov ia2_stackptr_2@GOTTPOFF(%rip), %r11\n"
    /* Write the old stack pointer to memory */
    "movq %rsp, %fs:(%r11)\n"
    /* Compute location to load new stack pointer (using r11) */
    "mov ia2_stackptr_1@GOTTPOFF(%rip), %r11\n"
    /* Read the new stack pointer from memory */
    "movq %fs:(%r11), %rsp\n"
    /* Preserve essential regs on stack */
    "pushq %rax\n"
    /* Scrub non-essential regs */
    "call __libia2_scrub_registers\n"
    /* Restore preserved regs */
    "popq %rax\n"
    /* Set PKRU to the caller's value */
    "movq %rax, %r10\n"
    "movq %rdx, %r11\n"
    "xorl %ecx, %ecx\n"
    "xorl %edx, %edx\n"
    "movl $0xfffffff0, %eax\n"
    "wrpkru\n"
    "movq %r10, %rax\n"
    "movq %r11, %rdx\n"
    "popq %r15\n"
    "popq %r14\n"
    "popq %r13\n"
    "popq %r12\n"
    "popq %rbx\n"
    "popq %rbp\n"
    /* Return to the caller */
    "ret\n"
    ".size __wrap_gzclose_w, .-__wrap_gzclose_w\n"
    ".previous\n"
);
asm(
    /* Wrapper for gzdirect(int) -> int: */
    ".text\n"
    ".global __wrap_gzdirect\n"
    ".type __wrap_gzdirect, @function\n"
    "__wrap_gzdirect:\n"
    "pushq %rbp\n"
    "movq %rsp, %rbp\n"
    "pushq %rbx\n"
    "pushq %r12\n"
    "pushq %r13\n"
    "pushq %r14\n"
    "pushq %r15\n"
    ASSERT_PKRU(0xfffffffffffffff0) "\n"
    /* Set PKRU to the intermediate value to move arguments */
    "movq %rcx, %r10\n"
    "movq %rdx, %r11\n"
    "xorl %ecx, %ecx\n"
    "xorl %edx, %edx\n"
    "movl $0xffffffc0, %eax\n"
    "wrpkru\n"
    "movq %r10, %rcx\n"
    "movq %r11, %rdx\n"
    /* Compute location to save old stack pointer (using r11) */
    "mov ia2_stackptr_1@GOTTPOFF(%rip), %r11\n"
    /* Write the old stack pointer to memory */
    "movq %rsp, %fs:(%r11)\n"
    /* Compute location to load new stack pointer (using r11) */
    "mov ia2_stackptr_2@GOTTPOFF(%rip), %r11\n"
    /* Read the new stack pointer from memory */
    "movq %fs:(%r11), %rsp\n"
    "subq $8, %rsp\n"
    /* Preserve essential regs on stack */
    "pushq %rdi\n"
    /* Scrub non-essential regs */
    "call __libia2_scrub_registers\n"
    /* Restore preserved regs */
    "popq %rdi\n"
    /* Set PKRU to the compartment's value */
    "movq %rcx, %r10\n"
    "movq %rdx, %r11\n"
    "xorl %ecx, %ecx\n"
    "xorl %edx, %edx\n"
    "movl $0xffffffcc, %eax\n"
    "wrpkru\n"
    "movq %r10, %rcx\n"
    "movq %r11, %rdx\n"
    /* Call wrapped function */
    "call gzdirect\n"
    /* Set PKRU to the intermediate value to move arguments */
    "movq %rax, %r10\n"
    "movq %rdx, %r11\n"
    "xorl %ecx, %ecx\n"
    "xorl %edx, %edx\n"
    "movl $0xffffffc0, %eax\n"
    "wrpkru\n"
    "movq %r10, %rax\n"
    "movq %r11, %rdx\n"
    /* Free stack space used for stack args */
    "addq $8, %rsp\n"
    /* Compute location to save old stack pointer (using r11) */
    "mov ia2_stackptr_2@GOTTPOFF(%rip), %r11\n"
    /* Write the old stack pointer to memory */
    "movq %rsp, %fs:(%r11)\n"
    /* Compute location to load new stack pointer (using r11) */
    "mov ia2_stackptr_1@GOTTPOFF(%rip), %r11\n"
    /* Read the new stack pointer from memory */
    "movq %fs:(%r11), %rsp\n"
    /* Preserve essential regs on stack */
    "pushq %rax\n"
    /* Scrub non-essential regs */
    "call __libia2_scrub_registers\n"
    /* Restore preserved regs */
    "popq %rax\n"
    /* Set PKRU to the caller's value */
    "movq %rax, %r10\n"
    "movq %rdx, %r11\n"
    "xorl %ecx, %ecx\n"
    "xorl %edx, %edx\n"
    "movl $0xfffffff0, %eax\n"
    "wrpkru\n"
    "movq %r10, %rax\n"
    "movq %r11, %rdx\n"
    "popq %r15\n"
    "popq %r14\n"
    "popq %r13\n"
    "popq %r12\n"
    "popq %rbx\n"
    "popq %rbp\n"
    /* Return to the caller */
    "ret\n"
    ".size __wrap_gzdirect, .-__wrap_gzdirect\n"
    ".previous\n"
);
asm(
    /* Wrapper for gzdopen(int, int) -> int: */
    ".text\n"
    ".global __wrap_gzdopen\n"
    ".type __wrap_gzdopen, @function\n"
    "__wrap_gzdopen:\n"
    "pushq %rbp\n"
    "movq %rsp, %rbp\n"
    "pushq %rbx\n"
    "pushq %r12\n"
    "pushq %r13\n"
    "pushq %r14\n"
    "pushq %r15\n"
    ASSERT_PKRU(0xfffffffffffffff0) "\n"
    /* Set PKRU to the intermediate value to move arguments */
    "movq %rcx, %r10\n"
    "movq %rdx, %r11\n"
    "xorl %ecx, %ecx\n"
    "xorl %edx, %edx\n"
    "movl $0xffffffc0, %eax\n"
    "wrpkru\n"
    "movq %r10, %rcx\n"
    "movq %r11, %rdx\n"
    /* Compute location to save old stack pointer (using r11) */
    "mov ia2_stackptr_1@GOTTPOFF(%rip), %r11\n"
    /* Write the old stack pointer to memory */
    "movq %rsp, %fs:(%r11)\n"
    /* Compute location to load new stack pointer (using r11) */
    "mov ia2_stackptr_2@GOTTPOFF(%rip), %r11\n"
    /* Read the new stack pointer from memory */
    "movq %fs:(%r11), %rsp\n"
    "subq $8, %rsp\n"
    /* Preserve essential regs on stack */
    "pushq %rdi\n"
    "pushq %rsi\n"
    /* Scrub non-essential regs */
    "call __libia2_scrub_registers\n"
    /* Restore preserved regs */
    "popq %rsi\n"
    "popq %rdi\n"
    /* Set PKRU to the compartment's value */
    "movq %rcx, %r10\n"
    "movq %rdx, %r11\n"
    "xorl %ecx, %ecx\n"
    "xorl %edx, %edx\n"
    "movl $0xffffffcc, %eax\n"
    "wrpkru\n"
    "movq %r10, %rcx\n"
    "movq %r11, %rdx\n"
    /* Call wrapped function */
    "call gzdopen\n"
    /* Set PKRU to the intermediate value to move arguments */
    "movq %rax, %r10\n"
    "movq %rdx, %r11\n"
    "xorl %ecx, %ecx\n"
    "xorl %edx, %edx\n"
    "movl $0xffffffc0, %eax\n"
    "wrpkru\n"
    "movq %r10, %rax\n"
    "movq %r11, %rdx\n"
    /* Free stack space used for stack args */
    "addq $8, %rsp\n"
    /* Compute location to save old stack pointer (using r11) */
    "mov ia2_stackptr_2@GOTTPOFF(%rip), %r11\n"
    /* Write the old stack pointer to memory */
    "movq %rsp, %fs:(%r11)\n"
    /* Compute location to load new stack pointer (using r11) */
    "mov ia2_stackptr_1@GOTTPOFF(%rip), %r11\n"
    /* Read the new stack pointer from memory */
    "movq %fs:(%r11), %rsp\n"
    /* Preserve essential regs on stack */
    "pushq %rax\n"
    /* Scrub non-essential regs */
    "call __libia2_scrub_registers\n"
    /* Restore preserved regs */
    "popq %rax\n"
    /* Set PKRU to the caller's value */
    "movq %rax, %r10\n"
    "movq %rdx, %r11\n"
    "xorl %ecx, %ecx\n"
    "xorl %edx, %edx\n"
    "movl $0xfffffff0, %eax\n"
    "wrpkru\n"
    "movq %r10, %rax\n"
    "movq %r11, %rdx\n"
    "popq %r15\n"
    "popq %r14\n"
    "popq %r13\n"
    "popq %r12\n"
    "popq %rbx\n"
    "popq %rbp\n"
    /* Return to the caller */
    "ret\n"
    ".size __wrap_gzdopen, .-__wrap_gzdopen\n"
    ".previous\n"
);
asm(
    /* Wrapper for gzeof(int) -> int: */
    ".text\n"
    ".global __wrap_gzeof\n"
    ".type __wrap_gzeof, @function\n"
    "__wrap_gzeof:\n"
    "pushq %rbp\n"
    "movq %rsp, %rbp\n"
    "pushq %rbx\n"
    "pushq %r12\n"
    "pushq %r13\n"
    "pushq %r14\n"
    "pushq %r15\n"
    ASSERT_PKRU(0xfffffffffffffff0) "\n"
    /* Set PKRU to the intermediate value to move arguments */
    "movq %rcx, %r10\n"
    "movq %rdx, %r11\n"
    "xorl %ecx, %ecx\n"
    "xorl %edx, %edx\n"
    "movl $0xffffffc0, %eax\n"
    "wrpkru\n"
    "movq %r10, %rcx\n"
    "movq %r11, %rdx\n"
    /* Compute location to save old stack pointer (using r11) */
    "mov ia2_stackptr_1@GOTTPOFF(%rip), %r11\n"
    /* Write the old stack pointer to memory */
    "movq %rsp, %fs:(%r11)\n"
    /* Compute location to load new stack pointer (using r11) */
    "mov ia2_stackptr_2@GOTTPOFF(%rip), %r11\n"
    /* Read the new stack pointer from memory */
    "movq %fs:(%r11), %rsp\n"
    "subq $8, %rsp\n"
    /* Preserve essential regs on stack */
    "pushq %rdi\n"
    /* Scrub non-essential regs */
    "call __libia2_scrub_registers\n"
    /* Restore preserved regs */
    "popq %rdi\n"
    /* Set PKRU to the compartment's value */
    "movq %rcx, %r10\n"
    "movq %rdx, %r11\n"
    "xorl %ecx, %ecx\n"
    "xorl %edx, %edx\n"
    "movl $0xffffffcc, %eax\n"
    "wrpkru\n"
    "movq %r10, %rcx\n"
    "movq %r11, %rdx\n"
    /* Call wrapped function */
    "call gzeof\n"
    /* Set PKRU to the intermediate value to move arguments */
    "movq %rax, %r10\n"
    "movq %rdx, %r11\n"
    "xorl %ecx, %ecx\n"
    "xorl %edx, %edx\n"
    "movl $0xffffffc0, %eax\n"
    "wrpkru\n"
    "movq %r10, %rax\n"
    "movq %r11, %rdx\n"
    /* Free stack space used for stack args */
    "addq $8, %rsp\n"
    /* Compute location to save old stack pointer (using r11) */
    "mov ia2_stackptr_2@GOTTPOFF(%rip), %r11\n"
    /* Write the old stack pointer to memory */
    "movq %rsp, %fs:(%r11)\n"
    /* Compute location to load new stack pointer (using r11) */
    "mov ia2_stackptr_1@GOTTPOFF(%rip), %r11\n"
    /* Read the new stack pointer from memory */
    "movq %fs:(%r11), %rsp\n"
    /* Preserve essential regs on stack */
    "pushq %rax\n"
    /* Scrub non-essential regs */
    "call __libia2_scrub_registers\n"
    /* Restore preserved regs */
    "popq %rax\n"
    /* Set PKRU to the caller's value */
    "movq %rax, %r10\n"
    "movq %rdx, %r11\n"
    "xorl %ecx, %ecx\n"
    "xorl %edx, %edx\n"
    "movl $0xfffffff0, %eax\n"
    "wrpkru\n"
    "movq %r10, %rax\n"
    "movq %r11, %rdx\n"
    "popq %r15\n"
    "popq %r14\n"
    "popq %r13\n"
    "popq %r12\n"
    "popq %rbx\n"
    "popq %rbp\n"
    /* Return to the caller */
    "ret\n"
    ".size __wrap_gzeof, .-__wrap_gzeof\n"
    ".previous\n"
);
asm(
    /* Wrapper for gzerror(int, int) -> int: */
    ".text\n"
    ".global __wrap_gzerror\n"
    ".type __wrap_gzerror, @function\n"
    "__wrap_gzerror:\n"
    "pushq %rbp\n"
    "movq %rsp, %rbp\n"
    "pushq %rbx\n"
    "pushq %r12\n"
    "pushq %r13\n"
    "pushq %r14\n"
    "pushq %r15\n"
    ASSERT_PKRU(0xfffffffffffffff0) "\n"
    /* Set PKRU to the intermediate value to move arguments */
    "movq %rcx, %r10\n"
    "movq %rdx, %r11\n"
    "xorl %ecx, %ecx\n"
    "xorl %edx, %edx\n"
    "movl $0xffffffc0, %eax\n"
    "wrpkru\n"
    "movq %r10, %rcx\n"
    "movq %r11, %rdx\n"
    /* Compute location to save old stack pointer (using r11) */
    "mov ia2_stackptr_1@GOTTPOFF(%rip), %r11\n"
    /* Write the old stack pointer to memory */
    "movq %rsp, %fs:(%r11)\n"
    /* Compute location to load new stack pointer (using r11) */
    "mov ia2_stackptr_2@GOTTPOFF(%rip), %r11\n"
    /* Read the new stack pointer from memory */
    "movq %fs:(%r11), %rsp\n"
    "subq $8, %rsp\n"
    /* Preserve essential regs on stack */
    "pushq %rdi\n"
    "pushq %rsi\n"
    /* Scrub non-essential regs */
    "call __libia2_scrub_registers\n"
    /* Restore preserved regs */
    "popq %rsi\n"
    "popq %rdi\n"
    /* Set PKRU to the compartment's value */
    "movq %rcx, %r10\n"
    "movq %rdx, %r11\n"
    "xorl %ecx, %ecx\n"
    "xorl %edx, %edx\n"
    "movl $0xffffffcc, %eax\n"
    "wrpkru\n"
    "movq %r10, %rcx\n"
    "movq %r11, %rdx\n"
    /* Call wrapped function */
    "call gzerror\n"
    /* Set PKRU to the intermediate value to move arguments */
    "movq %rax, %r10\n"
    "movq %rdx, %r11\n"
    "xorl %ecx, %ecx\n"
    "xorl %edx, %edx\n"
    "movl $0xffffffc0, %eax\n"
    "wrpkru\n"
    "movq %r10, %rax\n"
    "movq %r11, %rdx\n"
    /* Free stack space used for stack args */
    "addq $8, %rsp\n"
    /* Compute location to save old stack pointer (using r11) */
    "mov ia2_stackptr_2@GOTTPOFF(%rip), %r11\n"
    /* Write the old stack pointer to memory */
    "movq %rsp, %fs:(%r11)\n"
    /* Compute location to load new stack pointer (using r11) */
    "mov ia2_stackptr_1@GOTTPOFF(%rip), %r11\n"
    /* Read the new stack pointer from memory */
    "movq %fs:(%r11), %rsp\n"
    /* Preserve essential regs on stack */
    "pushq %rax\n"
    /* Scrub non-essential regs */
    "call __libia2_scrub_registers\n"
    /* Restore preserved regs */
    "popq %rax\n"
    /* Set PKRU to the caller's value */
    "movq %rax, %r10\n"
    "movq %rdx, %r11\n"
    "xorl %ecx, %ecx\n"
    "xorl %edx, %edx\n"
    "movl $0xfffffff0, %eax\n"
    "wrpkru\n"
    "movq %r10, %rax\n"
    "movq %r11, %rdx\n"
    "popq %r15\n"
    "popq %r14\n"
    "popq %r13\n"
    "popq %r12\n"
    "popq %rbx\n"
    "popq %rbp\n"
    /* Return to the caller */
    "ret\n"
    ".size __wrap_gzerror, .-__wrap_gzerror\n"
    ".previous\n"
);
asm(
    /* Wrapper for gzflush(int, int) -> int: */
    ".text\n"
    ".global __wrap_gzflush\n"
    ".type __wrap_gzflush, @function\n"
    "__wrap_gzflush:\n"
    "pushq %rbp\n"
    "movq %rsp, %rbp\n"
    "pushq %rbx\n"
    "pushq %r12\n"
    "pushq %r13\n"
    "pushq %r14\n"
    "pushq %r15\n"
    ASSERT_PKRU(0xfffffffffffffff0) "\n"
    /* Set PKRU to the intermediate value to move arguments */
    "movq %rcx, %r10\n"
    "movq %rdx, %r11\n"
    "xorl %ecx, %ecx\n"
    "xorl %edx, %edx\n"
    "movl $0xffffffc0, %eax\n"
    "wrpkru\n"
    "movq %r10, %rcx\n"
    "movq %r11, %rdx\n"
    /* Compute location to save old stack pointer (using r11) */
    "mov ia2_stackptr_1@GOTTPOFF(%rip), %r11\n"
    /* Write the old stack pointer to memory */
    "movq %rsp, %fs:(%r11)\n"
    /* Compute location to load new stack pointer (using r11) */
    "mov ia2_stackptr_2@GOTTPOFF(%rip), %r11\n"
    /* Read the new stack pointer from memory */
    "movq %fs:(%r11), %rsp\n"
    "subq $8, %rsp\n"
    /* Preserve essential regs on stack */
    "pushq %rdi\n"
    "pushq %rsi\n"
    /* Scrub non-essential regs */
    "call __libia2_scrub_registers\n"
    /* Restore preserved regs */
    "popq %rsi\n"
    "popq %rdi\n"
    /* Set PKRU to the compartment's value */
    "movq %rcx, %r10\n"
    "movq %rdx, %r11\n"
    "xorl %ecx, %ecx\n"
    "xorl %edx, %edx\n"
    "movl $0xffffffcc, %eax\n"
    "wrpkru\n"
    "movq %r10, %rcx\n"
    "movq %r11, %rdx\n"
    /* Call wrapped function */
    "call gzflush\n"
    /* Set PKRU to the intermediate value to move arguments */
    "movq %rax, %r10\n"
    "movq %rdx, %r11\n"
    "xorl %ecx, %ecx\n"
    "xorl %edx, %edx\n"
    "movl $0xffffffc0, %eax\n"
    "wrpkru\n"
    "movq %r10, %rax\n"
    "movq %r11, %rdx\n"
    /* Free stack space used for stack args */
    "addq $8, %rsp\n"
    /* Compute location to save old stack pointer (using r11) */
    "mov ia2_stackptr_2@GOTTPOFF(%rip), %r11\n"
    /* Write the old stack pointer to memory */
    "movq %rsp, %fs:(%r11)\n"
    /* Compute location to load new stack pointer (using r11) */
    "mov ia2_stackptr_1@GOTTPOFF(%rip), %r11\n"
    /* Read the new stack pointer from memory */
    "movq %fs:(%r11), %rsp\n"
    /* Preserve essential regs on stack */
    "pushq %rax\n"
    /* Scrub non-essential regs */
    "call __libia2_scrub_registers\n"
    /* Restore preserved regs */
    "popq %rax\n"
    /* Set PKRU to the caller's value */
    "movq %rax, %r10\n"
    "movq %rdx, %r11\n"
    "xorl %ecx, %ecx\n"
    "xorl %edx, %edx\n"
    "movl $0xfffffff0, %eax\n"
    "wrpkru\n"
    "movq %r10, %rax\n"
    "movq %r11, %rdx\n"
    "popq %r15\n"
    "popq %r14\n"
    "popq %r13\n"
    "popq %r12\n"
    "popq %rbx\n"
    "popq %rbp\n"
    /* Return to the caller */
    "ret\n"
    ".size __wrap_gzflush, .-__wrap_gzflush\n"
    ".previous\n"
);
asm(
    /* Wrapper for gzfread(int, int, int, int) -> int: */
    ".text\n"
    ".global __wrap_gzfread\n"
    ".type __wrap_gzfread, @function\n"
    "__wrap_gzfread:\n"
    "pushq %rbp\n"
    "movq %rsp, %rbp\n"
    "pushq %rbx\n"
    "pushq %r12\n"
    "pushq %r13\n"
    "pushq %r14\n"
    "pushq %r15\n"
    ASSERT_PKRU(0xfffffffffffffff0) "\n"
    /* Set PKRU to the intermediate value to move arguments */
    "movq %rcx, %r10\n"
    "movq %rdx, %r11\n"
    "xorl %ecx, %ecx\n"
    "xorl %edx, %edx\n"
    "movl $0xffffffc0, %eax\n"
    "wrpkru\n"
    "movq %r10, %rcx\n"
    "movq %r11, %rdx\n"
    /* Compute location to save old stack pointer (using r11) */
    "mov ia2_stackptr_1@GOTTPOFF(%rip), %r11\n"
    /* Write the old stack pointer to memory */
    "movq %rsp, %fs:(%r11)\n"
    /* Compute location to load new stack pointer (using r11) */
    "mov ia2_stackptr_2@GOTTPOFF(%rip), %r11\n"
    /* Read the new stack pointer from memory */
    "movq %fs:(%r11), %rsp\n"
    "subq $8, %rsp\n"
    /* Preserve essential regs on stack */
    "pushq %rdi\n"
    "pushq %rsi\n"
    "pushq %rdx\n"
    "pushq %rcx\n"
    /* Scrub non-essential regs */
    "call __libia2_scrub_registers\n"
    /* Restore preserved regs */
    "popq %rcx\n"
    "popq %rdx\n"
    "popq %rsi\n"
    "popq %rdi\n"
    /* Set PKRU to the compartment's value */
    "movq %rcx, %r10\n"
    "movq %rdx, %r11\n"
    "xorl %ecx, %ecx\n"
    "xorl %edx, %edx\n"
    "movl $0xffffffcc, %eax\n"
    "wrpkru\n"
    "movq %r10, %rcx\n"
    "movq %r11, %rdx\n"
    /* Call wrapped function */
    "call gzfread\n"
    /* Set PKRU to the intermediate value to move arguments */
    "movq %rax, %r10\n"
    "movq %rdx, %r11\n"
    "xorl %ecx, %ecx\n"
    "xorl %edx, %edx\n"
    "movl $0xffffffc0, %eax\n"
    "wrpkru\n"
    "movq %r10, %rax\n"
    "movq %r11, %rdx\n"
    /* Free stack space used for stack args */
    "addq $8, %rsp\n"
    /* Compute location to save old stack pointer (using r11) */
    "mov ia2_stackptr_2@GOTTPOFF(%rip), %r11\n"
    /* Write the old stack pointer to memory */
    "movq %rsp, %fs:(%r11)\n"
    /* Compute location to load new stack pointer (using r11) */
    "mov ia2_stackptr_1@GOTTPOFF(%rip), %r11\n"
    /* Read the new stack pointer from memory */
    "movq %fs:(%r11), %rsp\n"
    /* Preserve essential regs on stack */
    "pushq %rax\n"
    /* Scrub non-essential regs */
    "call __libia2_scrub_registers\n"
    /* Restore preserved regs */
    "popq %rax\n"
    /* Set PKRU to the caller's value */
    "movq %rax, %r10\n"
    "movq %rdx, %r11\n"
    "xorl %ecx, %ecx\n"
    "xorl %edx, %edx\n"
    "movl $0xfffffff0, %eax\n"
    "wrpkru\n"
    "movq %r10, %rax\n"
    "movq %r11, %rdx\n"
    "popq %r15\n"
    "popq %r14\n"
    "popq %r13\n"
    "popq %r12\n"
    "popq %rbx\n"
    "popq %rbp\n"
    /* Return to the caller */
    "ret\n"
    ".size __wrap_gzfread, .-__wrap_gzfread\n"
    ".previous\n"
);
asm(
    /* Wrapper for gzfwrite(int, int, int, int) -> int: */
    ".text\n"
    ".global __wrap_gzfwrite\n"
    ".type __wrap_gzfwrite, @function\n"
    "__wrap_gzfwrite:\n"
    "pushq %rbp\n"
    "movq %rsp, %rbp\n"
    "pushq %rbx\n"
    "pushq %r12\n"
    "pushq %r13\n"
    "pushq %r14\n"
    "pushq %r15\n"
    ASSERT_PKRU(0xfffffffffffffff0) "\n"
    /* Set PKRU to the intermediate value to move arguments */
    "movq %rcx, %r10\n"
    "movq %rdx, %r11\n"
    "xorl %ecx, %ecx\n"
    "xorl %edx, %edx\n"
    "movl $0xffffffc0, %eax\n"
    "wrpkru\n"
    "movq %r10, %rcx\n"
    "movq %r11, %rdx\n"
    /* Compute location to save old stack pointer (using r11) */
    "mov ia2_stackptr_1@GOTTPOFF(%rip), %r11\n"
    /* Write the old stack pointer to memory */
    "movq %rsp, %fs:(%r11)\n"
    /* Compute location to load new stack pointer (using r11) */
    "mov ia2_stackptr_2@GOTTPOFF(%rip), %r11\n"
    /* Read the new stack pointer from memory */
    "movq %fs:(%r11), %rsp\n"
    "subq $8, %rsp\n"
    /* Preserve essential regs on stack */
    "pushq %rdi\n"
    "pushq %rsi\n"
    "pushq %rdx\n"
    "pushq %rcx\n"
    /* Scrub non-essential regs */
    "call __libia2_scrub_registers\n"
    /* Restore preserved regs */
    "popq %rcx\n"
    "popq %rdx\n"
    "popq %rsi\n"
    "popq %rdi\n"
    /* Set PKRU to the compartment's value */
    "movq %rcx, %r10\n"
    "movq %rdx, %r11\n"
    "xorl %ecx, %ecx\n"
    "xorl %edx, %edx\n"
    "movl $0xffffffcc, %eax\n"
    "wrpkru\n"
    "movq %r10, %rcx\n"
    "movq %r11, %rdx\n"
    /* Call wrapped function */
    "call gzfwrite\n"
    /* Set PKRU to the intermediate value to move arguments */
    "movq %rax, %r10\n"
    "movq %rdx, %r11\n"
    "xorl %ecx, %ecx\n"
    "xorl %edx, %edx\n"
    "movl $0xffffffc0, %eax\n"
    "wrpkru\n"
    "movq %r10, %rax\n"
    "movq %r11, %rdx\n"
    /* Free stack space used for stack args */
    "addq $8, %rsp\n"
    /* Compute location to save old stack pointer (using r11) */
    "mov ia2_stackptr_2@GOTTPOFF(%rip), %r11\n"
    /* Write the old stack pointer to memory */
    "movq %rsp, %fs:(%r11)\n"
    /* Compute location to load new stack pointer (using r11) */
    "mov ia2_stackptr_1@GOTTPOFF(%rip), %r11\n"
    /* Read the new stack pointer from memory */
    "movq %fs:(%r11), %rsp\n"
    /* Preserve essential regs on stack */
    "pushq %rax\n"
    /* Scrub non-essential regs */
    "call __libia2_scrub_registers\n"
    /* Restore preserved regs */
    "popq %rax\n"
    /* Set PKRU to the caller's value */
    "movq %rax, %r10\n"
    "movq %rdx, %r11\n"
    "xorl %ecx, %ecx\n"
    "xorl %edx, %edx\n"
    "movl $0xfffffff0, %eax\n"
    "wrpkru\n"
    "movq %r10, %rax\n"
    "movq %r11, %rdx\n"
    "popq %r15\n"
    "popq %r14\n"
    "popq %r13\n"
    "popq %r12\n"
    "popq %rbx\n"
    "popq %rbp\n"
    /* Return to the caller */
    "ret\n"
    ".size __wrap_gzfwrite, .-__wrap_gzfwrite\n"
    ".previous\n"
);
asm(
    /* Wrapper for gzgetc(int) -> int: */
    ".text\n"
    ".global __wrap_gzgetc\n"
    ".type __wrap_gzgetc, @function\n"
    "__wrap_gzgetc:\n"
    "pushq %rbp\n"
    "movq %rsp, %rbp\n"
    "pushq %rbx\n"
    "pushq %r12\n"
    "pushq %r13\n"
    "pushq %r14\n"
    "pushq %r15\n"
    ASSERT_PKRU(0xfffffffffffffff0) "\n"
    /* Set PKRU to the intermediate value to move arguments */
    "movq %rcx, %r10\n"
    "movq %rdx, %r11\n"
    "xorl %ecx, %ecx\n"
    "xorl %edx, %edx\n"
    "movl $0xffffffc0, %eax\n"
    "wrpkru\n"
    "movq %r10, %rcx\n"
    "movq %r11, %rdx\n"
    /* Compute location to save old stack pointer (using r11) */
    "mov ia2_stackptr_1@GOTTPOFF(%rip), %r11\n"
    /* Write the old stack pointer to memory */
    "movq %rsp, %fs:(%r11)\n"
    /* Compute location to load new stack pointer (using r11) */
    "mov ia2_stackptr_2@GOTTPOFF(%rip), %r11\n"
    /* Read the new stack pointer from memory */
    "movq %fs:(%r11), %rsp\n"
    "subq $8, %rsp\n"
    /* Preserve essential regs on stack */
    "pushq %rdi\n"
    /* Scrub non-essential regs */
    "call __libia2_scrub_registers\n"
    /* Restore preserved regs */
    "popq %rdi\n"
    /* Set PKRU to the compartment's value */
    "movq %rcx, %r10\n"
    "movq %rdx, %r11\n"
    "xorl %ecx, %ecx\n"
    "xorl %edx, %edx\n"
    "movl $0xffffffcc, %eax\n"
    "wrpkru\n"
    "movq %r10, %rcx\n"
    "movq %r11, %rdx\n"
    /* Call wrapped function */
    "call gzgetc\n"
    /* Set PKRU to the intermediate value to move arguments */
    "movq %rax, %r10\n"
    "movq %rdx, %r11\n"
    "xorl %ecx, %ecx\n"
    "xorl %edx, %edx\n"
    "movl $0xffffffc0, %eax\n"
    "wrpkru\n"
    "movq %r10, %rax\n"
    "movq %r11, %rdx\n"
    /* Free stack space used for stack args */
    "addq $8, %rsp\n"
    /* Compute location to save old stack pointer (using r11) */
    "mov ia2_stackptr_2@GOTTPOFF(%rip), %r11\n"
    /* Write the old stack pointer to memory */
    "movq %rsp, %fs:(%r11)\n"
    /* Compute location to load new stack pointer (using r11) */
    "mov ia2_stackptr_1@GOTTPOFF(%rip), %r11\n"
    /* Read the new stack pointer from memory */
    "movq %fs:(%r11), %rsp\n"
    /* Preserve essential regs on stack */
    "pushq %rax\n"
    /* Scrub non-essential regs */
    "call __libia2_scrub_registers\n"
    /* Restore preserved regs */
    "popq %rax\n"
    /* Set PKRU to the caller's value */
    "movq %rax, %r10\n"
    "movq %rdx, %r11\n"
    "xorl %ecx, %ecx\n"
    "xorl %edx, %edx\n"
    "movl $0xfffffff0, %eax\n"
    "wrpkru\n"
    "movq %r10, %rax\n"
    "movq %r11, %rdx\n"
    "popq %r15\n"
    "popq %r14\n"
    "popq %r13\n"
    "popq %r12\n"
    "popq %rbx\n"
    "popq %rbp\n"
    /* Return to the caller */
    "ret\n"
    ".size __wrap_gzgetc, .-__wrap_gzgetc\n"
    ".previous\n"
);
asm(
    /* Wrapper for gzgetc_(int) -> int: */
    ".text\n"
    ".global __wrap_gzgetc_\n"
    ".type __wrap_gzgetc_, @function\n"
    "__wrap_gzgetc_:\n"
    "pushq %rbp\n"
    "movq %rsp, %rbp\n"
    "pushq %rbx\n"
    "pushq %r12\n"
    "pushq %r13\n"
    "pushq %r14\n"
    "pushq %r15\n"
    ASSERT_PKRU(0xfffffffffffffff0) "\n"
    /* Set PKRU to the intermediate value to move arguments */
    "movq %rcx, %r10\n"
    "movq %rdx, %r11\n"
    "xorl %ecx, %ecx\n"
    "xorl %edx, %edx\n"
    "movl $0xffffffc0, %eax\n"
    "wrpkru\n"
    "movq %r10, %rcx\n"
    "movq %r11, %rdx\n"
    /* Compute location to save old stack pointer (using r11) */
    "mov ia2_stackptr_1@GOTTPOFF(%rip), %r11\n"
    /* Write the old stack pointer to memory */
    "movq %rsp, %fs:(%r11)\n"
    /* Compute location to load new stack pointer (using r11) */
    "mov ia2_stackptr_2@GOTTPOFF(%rip), %r11\n"
    /* Read the new stack pointer from memory */
    "movq %fs:(%r11), %rsp\n"
    "subq $8, %rsp\n"
    /* Preserve essential regs on stack */
    "pushq %rdi\n"
    /* Scrub non-essential regs */
    "call __libia2_scrub_registers\n"
    /* Restore preserved regs */
    "popq %rdi\n"
    /* Set PKRU to the compartment's value */
    "movq %rcx, %r10\n"
    "movq %rdx, %r11\n"
    "xorl %ecx, %ecx\n"
    "xorl %edx, %edx\n"
    "movl $0xffffffcc, %eax\n"
    "wrpkru\n"
    "movq %r10, %rcx\n"
    "movq %r11, %rdx\n"
    /* Call wrapped function */
    "call gzgetc_\n"
    /* Set PKRU to the intermediate value to move arguments */
    "movq %rax, %r10\n"
    "movq %rdx, %r11\n"
    "xorl %ecx, %ecx\n"
    "xorl %edx, %edx\n"
    "movl $0xffffffc0, %eax\n"
    "wrpkru\n"
    "movq %r10, %rax\n"
    "movq %r11, %rdx\n"
    /* Free stack space used for stack args */
    "addq $8, %rsp\n"
    /* Compute location to save old stack pointer (using r11) */
    "mov ia2_stackptr_2@GOTTPOFF(%rip), %r11\n"
    /* Write the old stack pointer to memory */
    "movq %rsp, %fs:(%r11)\n"
    /* Compute location to load new stack pointer (using r11) */
    "mov ia2_stackptr_1@GOTTPOFF(%rip), %r11\n"
    /* Read the new stack pointer from memory */
    "movq %fs:(%r11), %rsp\n"
    /* Preserve essential regs on stack */
    "pushq %rax\n"
    /* Scrub non-essential regs */
    "call __libia2_scrub_registers\n"
    /* Restore preserved regs */
    "popq %rax\n"
    /* Set PKRU to the caller's value */
    "movq %rax, %r10\n"
    "movq %rdx, %r11\n"
    "xorl %ecx, %ecx\n"
    "xorl %edx, %edx\n"
    "movl $0xfffffff0, %eax\n"
    "wrpkru\n"
    "movq %r10, %rax\n"
    "movq %r11, %rdx\n"
    "popq %r15\n"
    "popq %r14\n"
    "popq %r13\n"
    "popq %r12\n"
    "popq %rbx\n"
    "popq %rbp\n"
    /* Return to the caller */
    "ret\n"
    ".size __wrap_gzgetc_, .-__wrap_gzgetc_\n"
    ".previous\n"
);
asm(
    /* Wrapper for gzgets(int, int, int) -> int: */
    ".text\n"
    ".global __wrap_gzgets\n"
    ".type __wrap_gzgets, @function\n"
    "__wrap_gzgets:\n"
    "pushq %rbp\n"
    "movq %rsp, %rbp\n"
    "pushq %rbx\n"
    "pushq %r12\n"
    "pushq %r13\n"
    "pushq %r14\n"
    "pushq %r15\n"
    ASSERT_PKRU(0xfffffffffffffff0) "\n"
    /* Set PKRU to the intermediate value to move arguments */
    "movq %rcx, %r10\n"
    "movq %rdx, %r11\n"
    "xorl %ecx, %ecx\n"
    "xorl %edx, %edx\n"
    "movl $0xffffffc0, %eax\n"
    "wrpkru\n"
    "movq %r10, %rcx\n"
    "movq %r11, %rdx\n"
    /* Compute location to save old stack pointer (using r11) */
    "mov ia2_stackptr_1@GOTTPOFF(%rip), %r11\n"
    /* Write the old stack pointer to memory */
    "movq %rsp, %fs:(%r11)\n"
    /* Compute location to load new stack pointer (using r11) */
    "mov ia2_stackptr_2@GOTTPOFF(%rip), %r11\n"
    /* Read the new stack pointer from memory */
    "movq %fs:(%r11), %rsp\n"
    "subq $8, %rsp\n"
    /* Preserve essential regs on stack */
    "pushq %rdi\n"
    "pushq %rsi\n"
    "pushq %rdx\n"
    /* Scrub non-essential regs */
    "call __libia2_scrub_registers\n"
    /* Restore preserved regs */
    "popq %rdx\n"
    "popq %rsi\n"
    "popq %rdi\n"
    /* Set PKRU to the compartment's value */
    "movq %rcx, %r10\n"
    "movq %rdx, %r11\n"
    "xorl %ecx, %ecx\n"
    "xorl %edx, %edx\n"
    "movl $0xffffffcc, %eax\n"
    "wrpkru\n"
    "movq %r10, %rcx\n"
    "movq %r11, %rdx\n"
    /* Call wrapped function */
    "call gzgets\n"
    /* Set PKRU to the intermediate value to move arguments */
    "movq %rax, %r10\n"
    "movq %rdx, %r11\n"
    "xorl %ecx, %ecx\n"
    "xorl %edx, %edx\n"
    "movl $0xffffffc0, %eax\n"
    "wrpkru\n"
    "movq %r10, %rax\n"
    "movq %r11, %rdx\n"
    /* Free stack space used for stack args */
    "addq $8, %rsp\n"
    /* Compute location to save old stack pointer (using r11) */
    "mov ia2_stackptr_2@GOTTPOFF(%rip), %r11\n"
    /* Write the old stack pointer to memory */
    "movq %rsp, %fs:(%r11)\n"
    /* Compute location to load new stack pointer (using r11) */
    "mov ia2_stackptr_1@GOTTPOFF(%rip), %r11\n"
    /* Read the new stack pointer from memory */
    "movq %fs:(%r11), %rsp\n"
    /* Preserve essential regs on stack */
    "pushq %rax\n"
    /* Scrub non-essential regs */
    "call __libia2_scrub_registers\n"
    /* Restore preserved regs */
    "popq %rax\n"
    /* Set PKRU to the caller's value */
    "movq %rax, %r10\n"
    "movq %rdx, %r11\n"
    "xorl %ecx, %ecx\n"
    "xorl %edx, %edx\n"
    "movl $0xfffffff0, %eax\n"
    "wrpkru\n"
    "movq %r10, %rax\n"
    "movq %r11, %rdx\n"
    "popq %r15\n"
    "popq %r14\n"
    "popq %r13\n"
    "popq %r12\n"
    "popq %rbx\n"
    "popq %rbp\n"
    /* Return to the caller */
    "ret\n"
    ".size __wrap_gzgets, .-__wrap_gzgets\n"
    ".previous\n"
);
asm(
    /* Wrapper for gzoffset(int) -> int: */
    ".text\n"
    ".global __wrap_gzoffset\n"
    ".type __wrap_gzoffset, @function\n"
    "__wrap_gzoffset:\n"
    "pushq %rbp\n"
    "movq %rsp, %rbp\n"
    "pushq %rbx\n"
    "pushq %r12\n"
    "pushq %r13\n"
    "pushq %r14\n"
    "pushq %r15\n"
    ASSERT_PKRU(0xfffffffffffffff0) "\n"
    /* Set PKRU to the intermediate value to move arguments */
    "movq %rcx, %r10\n"
    "movq %rdx, %r11\n"
    "xorl %ecx, %ecx\n"
    "xorl %edx, %edx\n"
    "movl $0xffffffc0, %eax\n"
    "wrpkru\n"
    "movq %r10, %rcx\n"
    "movq %r11, %rdx\n"
    /* Compute location to save old stack pointer (using r11) */
    "mov ia2_stackptr_1@GOTTPOFF(%rip), %r11\n"
    /* Write the old stack pointer to memory */
    "movq %rsp, %fs:(%r11)\n"
    /* Compute location to load new stack pointer (using r11) */
    "mov ia2_stackptr_2@GOTTPOFF(%rip), %r11\n"
    /* Read the new stack pointer from memory */
    "movq %fs:(%r11), %rsp\n"
    "subq $8, %rsp\n"
    /* Preserve essential regs on stack */
    "pushq %rdi\n"
    /* Scrub non-essential regs */
    "call __libia2_scrub_registers\n"
    /* Restore preserved regs */
    "popq %rdi\n"
    /* Set PKRU to the compartment's value */
    "movq %rcx, %r10\n"
    "movq %rdx, %r11\n"
    "xorl %ecx, %ecx\n"
    "xorl %edx, %edx\n"
    "movl $0xffffffcc, %eax\n"
    "wrpkru\n"
    "movq %r10, %rcx\n"
    "movq %r11, %rdx\n"
    /* Call wrapped function */
    "call gzoffset\n"
    /* Set PKRU to the intermediate value to move arguments */
    "movq %rax, %r10\n"
    "movq %rdx, %r11\n"
    "xorl %ecx, %ecx\n"
    "xorl %edx, %edx\n"
    "movl $0xffffffc0, %eax\n"
    "wrpkru\n"
    "movq %r10, %rax\n"
    "movq %r11, %rdx\n"
    /* Free stack space used for stack args */
    "addq $8, %rsp\n"
    /* Compute location to save old stack pointer (using r11) */
    "mov ia2_stackptr_2@GOTTPOFF(%rip), %r11\n"
    /* Write the old stack pointer to memory */
    "movq %rsp, %fs:(%r11)\n"
    /* Compute location to load new stack pointer (using r11) */
    "mov ia2_stackptr_1@GOTTPOFF(%rip), %r11\n"
    /* Read the new stack pointer from memory */
    "movq %fs:(%r11), %rsp\n"
    /* Preserve essential regs on stack */
    "pushq %rax\n"
    /* Scrub non-essential regs */
    "call __libia2_scrub_registers\n"
    /* Restore preserved regs */
    "popq %rax\n"
    /* Set PKRU to the caller's value */
    "movq %rax, %r10\n"
    "movq %rdx, %r11\n"
    "xorl %ecx, %ecx\n"
    "xorl %edx, %edx\n"
    "movl $0xfffffff0, %eax\n"
    "wrpkru\n"
    "movq %r10, %rax\n"
    "movq %r11, %rdx\n"
    "popq %r15\n"
    "popq %r14\n"
    "popq %r13\n"
    "popq %r12\n"
    "popq %rbx\n"
    "popq %rbp\n"
    /* Return to the caller */
    "ret\n"
    ".size __wrap_gzoffset, .-__wrap_gzoffset\n"
    ".previous\n"
);
asm(
    /* Wrapper for gzoffset64(int) -> int: */
    ".text\n"
    ".global __wrap_gzoffset64\n"
    ".type __wrap_gzoffset64, @function\n"
    "__wrap_gzoffset64:\n"
    "pushq %rbp\n"
    "movq %rsp, %rbp\n"
    "pushq %rbx\n"
    "pushq %r12\n"
    "pushq %r13\n"
    "pushq %r14\n"
    "pushq %r15\n"
    ASSERT_PKRU(0xfffffffffffffff0) "\n"
    /* Set PKRU to the intermediate value to move arguments */
    "movq %rcx, %r10\n"
    "movq %rdx, %r11\n"
    "xorl %ecx, %ecx\n"
    "xorl %edx, %edx\n"
    "movl $0xffffffc0, %eax\n"
    "wrpkru\n"
    "movq %r10, %rcx\n"
    "movq %r11, %rdx\n"
    /* Compute location to save old stack pointer (using r11) */
    "mov ia2_stackptr_1@GOTTPOFF(%rip), %r11\n"
    /* Write the old stack pointer to memory */
    "movq %rsp, %fs:(%r11)\n"
    /* Compute location to load new stack pointer (using r11) */
    "mov ia2_stackptr_2@GOTTPOFF(%rip), %r11\n"
    /* Read the new stack pointer from memory */
    "movq %fs:(%r11), %rsp\n"
    "subq $8, %rsp\n"
    /* Preserve essential regs on stack */
    "pushq %rdi\n"
    /* Scrub non-essential regs */
    "call __libia2_scrub_registers\n"
    /* Restore preserved regs */
    "popq %rdi\n"
    /* Set PKRU to the compartment's value */
    "movq %rcx, %r10\n"
    "movq %rdx, %r11\n"
    "xorl %ecx, %ecx\n"
    "xorl %edx, %edx\n"
    "movl $0xffffffcc, %eax\n"
    "wrpkru\n"
    "movq %r10, %rcx\n"
    "movq %r11, %rdx\n"
    /* Call wrapped function */
    "call gzoffset64\n"
    /* Set PKRU to the intermediate value to move arguments */
    "movq %rax, %r10\n"
    "movq %rdx, %r11\n"
    "xorl %ecx, %ecx\n"
    "xorl %edx, %edx\n"
    "movl $0xffffffc0, %eax\n"
    "wrpkru\n"
    "movq %r10, %rax\n"
    "movq %r11, %rdx\n"
    /* Free stack space used for stack args */
    "addq $8, %rsp\n"
    /* Compute location to save old stack pointer (using r11) */
    "mov ia2_stackptr_2@GOTTPOFF(%rip), %r11\n"
    /* Write the old stack pointer to memory */
    "movq %rsp, %fs:(%r11)\n"
    /* Compute location to load new stack pointer (using r11) */
    "mov ia2_stackptr_1@GOTTPOFF(%rip), %r11\n"
    /* Read the new stack pointer from memory */
    "movq %fs:(%r11), %rsp\n"
    /* Preserve essential regs on stack */
    "pushq %rax\n"
    /* Scrub non-essential regs */
    "call __libia2_scrub_registers\n"
    /* Restore preserved regs */
    "popq %rax\n"
    /* Set PKRU to the caller's value */
    "movq %rax, %r10\n"
    "movq %rdx, %r11\n"
    "xorl %ecx, %ecx\n"
    "xorl %edx, %edx\n"
    "movl $0xfffffff0, %eax\n"
    "wrpkru\n"
    "movq %r10, %rax\n"
    "movq %r11, %rdx\n"
    "popq %r15\n"
    "popq %r14\n"
    "popq %r13\n"
    "popq %r12\n"
    "popq %rbx\n"
    "popq %rbp\n"
    /* Return to the caller */
    "ret\n"
    ".size __wrap_gzoffset64, .-__wrap_gzoffset64\n"
    ".previous\n"
);
asm(
    /* Wrapper for gzopen(int, int) -> int: */
    ".text\n"
    ".global __wrap_gzopen\n"
    ".type __wrap_gzopen, @function\n"
    "__wrap_gzopen:\n"
    "pushq %rbp\n"
    "movq %rsp, %rbp\n"
    "pushq %rbx\n"
    "pushq %r12\n"
    "pushq %r13\n"
    "pushq %r14\n"
    "pushq %r15\n"
    ASSERT_PKRU(0xfffffffffffffff0) "\n"
    /* Set PKRU to the intermediate value to move arguments */
    "movq %rcx, %r10\n"
    "movq %rdx, %r11\n"
    "xorl %ecx, %ecx\n"
    "xorl %edx, %edx\n"
    "movl $0xffffffc0, %eax\n"
    "wrpkru\n"
    "movq %r10, %rcx\n"
    "movq %r11, %rdx\n"
    /* Compute location to save old stack pointer (using r11) */
    "mov ia2_stackptr_1@GOTTPOFF(%rip), %r11\n"
    /* Write the old stack pointer to memory */
    "movq %rsp, %fs:(%r11)\n"
    /* Compute location to load new stack pointer (using r11) */
    "mov ia2_stackptr_2@GOTTPOFF(%rip), %r11\n"
    /* Read the new stack pointer from memory */
    "movq %fs:(%r11), %rsp\n"
    "subq $8, %rsp\n"
    /* Preserve essential regs on stack */
    "pushq %rdi\n"
    "pushq %rsi\n"
    /* Scrub non-essential regs */
    "call __libia2_scrub_registers\n"
    /* Restore preserved regs */
    "popq %rsi\n"
    "popq %rdi\n"
    /* Set PKRU to the compartment's value */
    "movq %rcx, %r10\n"
    "movq %rdx, %r11\n"
    "xorl %ecx, %ecx\n"
    "xorl %edx, %edx\n"
    "movl $0xffffffcc, %eax\n"
    "wrpkru\n"
    "movq %r10, %rcx\n"
    "movq %r11, %rdx\n"
    /* Call wrapped function */
    "call gzopen\n"
    /* Set PKRU to the intermediate value to move arguments */
    "movq %rax, %r10\n"
    "movq %rdx, %r11\n"
    "xorl %ecx, %ecx\n"
    "xorl %edx, %edx\n"
    "movl $0xffffffc0, %eax\n"
    "wrpkru\n"
    "movq %r10, %rax\n"
    "movq %r11, %rdx\n"
    /* Free stack space used for stack args */
    "addq $8, %rsp\n"
    /* Compute location to save old stack pointer (using r11) */
    "mov ia2_stackptr_2@GOTTPOFF(%rip), %r11\n"
    /* Write the old stack pointer to memory */
    "movq %rsp, %fs:(%r11)\n"
    /* Compute location to load new stack pointer (using r11) */
    "mov ia2_stackptr_1@GOTTPOFF(%rip), %r11\n"
    /* Read the new stack pointer from memory */
    "movq %fs:(%r11), %rsp\n"
    /* Preserve essential regs on stack */
    "pushq %rax\n"
    /* Scrub non-essential regs */
    "call __libia2_scrub_registers\n"
    /* Restore preserved regs */
    "popq %rax\n"
    /* Set PKRU to the caller's value */
    "movq %rax, %r10\n"
    "movq %rdx, %r11\n"
    "xorl %ecx, %ecx\n"
    "xorl %edx, %edx\n"
    "movl $0xfffffff0, %eax\n"
    "wrpkru\n"
    "movq %r10, %rax\n"
    "movq %r11, %rdx\n"
    "popq %r15\n"
    "popq %r14\n"
    "popq %r13\n"
    "popq %r12\n"
    "popq %rbx\n"
    "popq %rbp\n"
    /* Return to the caller */
    "ret\n"
    ".size __wrap_gzopen, .-__wrap_gzopen\n"
    ".previous\n"
);
asm(
    /* Wrapper for gzopen64(int, int) -> int: */
    ".text\n"
    ".global __wrap_gzopen64\n"
    ".type __wrap_gzopen64, @function\n"
    "__wrap_gzopen64:\n"
    "pushq %rbp\n"
    "movq %rsp, %rbp\n"
    "pushq %rbx\n"
    "pushq %r12\n"
    "pushq %r13\n"
    "pushq %r14\n"
    "pushq %r15\n"
    ASSERT_PKRU(0xfffffffffffffff0) "\n"
    /* Set PKRU to the intermediate value to move arguments */
    "movq %rcx, %r10\n"
    "movq %rdx, %r11\n"
    "xorl %ecx, %ecx\n"
    "xorl %edx, %edx\n"
    "movl $0xffffffc0, %eax\n"
    "wrpkru\n"
    "movq %r10, %rcx\n"
    "movq %r11, %rdx\n"
    /* Compute location to save old stack pointer (using r11) */
    "mov ia2_stackptr_1@GOTTPOFF(%rip), %r11\n"
    /* Write the old stack pointer to memory */
    "movq %rsp, %fs:(%r11)\n"
    /* Compute location to load new stack pointer (using r11) */
    "mov ia2_stackptr_2@GOTTPOFF(%rip), %r11\n"
    /* Read the new stack pointer from memory */
    "movq %fs:(%r11), %rsp\n"
    "subq $8, %rsp\n"
    /* Preserve essential regs on stack */
    "pushq %rdi\n"
    "pushq %rsi\n"
    /* Scrub non-essential regs */
    "call __libia2_scrub_registers\n"
    /* Restore preserved regs */
    "popq %rsi\n"
    "popq %rdi\n"
    /* Set PKRU to the compartment's value */
    "movq %rcx, %r10\n"
    "movq %rdx, %r11\n"
    "xorl %ecx, %ecx\n"
    "xorl %edx, %edx\n"
    "movl $0xffffffcc, %eax\n"
    "wrpkru\n"
    "movq %r10, %rcx\n"
    "movq %r11, %rdx\n"
    /* Call wrapped function */
    "call gzopen64\n"
    /* Set PKRU to the intermediate value to move arguments */
    "movq %rax, %r10\n"
    "movq %rdx, %r11\n"
    "xorl %ecx, %ecx\n"
    "xorl %edx, %edx\n"
    "movl $0xffffffc0, %eax\n"
    "wrpkru\n"
    "movq %r10, %rax\n"
    "movq %r11, %rdx\n"
    /* Free stack space used for stack args */
    "addq $8, %rsp\n"
    /* Compute location to save old stack pointer (using r11) */
    "mov ia2_stackptr_2@GOTTPOFF(%rip), %r11\n"
    /* Write the old stack pointer to memory */
    "movq %rsp, %fs:(%r11)\n"
    /* Compute location to load new stack pointer (using r11) */
    "mov ia2_stackptr_1@GOTTPOFF(%rip), %r11\n"
    /* Read the new stack pointer from memory */
    "movq %fs:(%r11), %rsp\n"
    /* Preserve essential regs on stack */
    "pushq %rax\n"
    /* Scrub non-essential regs */
    "call __libia2_scrub_registers\n"
    /* Restore preserved regs */
    "popq %rax\n"
    /* Set PKRU to the caller's value */
    "movq %rax, %r10\n"
    "movq %rdx, %r11\n"
    "xorl %ecx, %ecx\n"
    "xorl %edx, %edx\n"
    "movl $0xfffffff0, %eax\n"
    "wrpkru\n"
    "movq %r10, %rax\n"
    "movq %r11, %rdx\n"
    "popq %r15\n"
    "popq %r14\n"
    "popq %r13\n"
    "popq %r12\n"
    "popq %rbx\n"
    "popq %rbp\n"
    /* Return to the caller */
    "ret\n"
    ".size __wrap_gzopen64, .-__wrap_gzopen64\n"
    ".previous\n"
);
asm(
    /* Wrapper for gzputc(int, int) -> int: */
    ".text\n"
    ".global __wrap_gzputc\n"
    ".type __wrap_gzputc, @function\n"
    "__wrap_gzputc:\n"
    "pushq %rbp\n"
    "movq %rsp, %rbp\n"
    "pushq %rbx\n"
    "pushq %r12\n"
    "pushq %r13\n"
    "pushq %r14\n"
    "pushq %r15\n"
    ASSERT_PKRU(0xfffffffffffffff0) "\n"
    /* Set PKRU to the intermediate value to move arguments */
    "movq %rcx, %r10\n"
    "movq %rdx, %r11\n"
    "xorl %ecx, %ecx\n"
    "xorl %edx, %edx\n"
    "movl $0xffffffc0, %eax\n"
    "wrpkru\n"
    "movq %r10, %rcx\n"
    "movq %r11, %rdx\n"
    /* Compute location to save old stack pointer (using r11) */
    "mov ia2_stackptr_1@GOTTPOFF(%rip), %r11\n"
    /* Write the old stack pointer to memory */
    "movq %rsp, %fs:(%r11)\n"
    /* Compute location to load new stack pointer (using r11) */
    "mov ia2_stackptr_2@GOTTPOFF(%rip), %r11\n"
    /* Read the new stack pointer from memory */
    "movq %fs:(%r11), %rsp\n"
    "subq $8, %rsp\n"
    /* Preserve essential regs on stack */
    "pushq %rdi\n"
    "pushq %rsi\n"
    /* Scrub non-essential regs */
    "call __libia2_scrub_registers\n"
    /* Restore preserved regs */
    "popq %rsi\n"
    "popq %rdi\n"
    /* Set PKRU to the compartment's value */
    "movq %rcx, %r10\n"
    "movq %rdx, %r11\n"
    "xorl %ecx, %ecx\n"
    "xorl %edx, %edx\n"
    "movl $0xffffffcc, %eax\n"
    "wrpkru\n"
    "movq %r10, %rcx\n"
    "movq %r11, %rdx\n"
    /* Call wrapped function */
    "call gzputc\n"
    /* Set PKRU to the intermediate value to move arguments */
    "movq %rax, %r10\n"
    "movq %rdx, %r11\n"
    "xorl %ecx, %ecx\n"
    "xorl %edx, %edx\n"
    "movl $0xffffffc0, %eax\n"
    "wrpkru\n"
    "movq %r10, %rax\n"
    "movq %r11, %rdx\n"
    /* Free stack space used for stack args */
    "addq $8, %rsp\n"
    /* Compute location to save old stack pointer (using r11) */
    "mov ia2_stackptr_2@GOTTPOFF(%rip), %r11\n"
    /* Write the old stack pointer to memory */
    "movq %rsp, %fs:(%r11)\n"
    /* Compute location to load new stack pointer (using r11) */
    "mov ia2_stackptr_1@GOTTPOFF(%rip), %r11\n"
    /* Read the new stack pointer from memory */
    "movq %fs:(%r11), %rsp\n"
    /* Preserve essential regs on stack */
    "pushq %rax\n"
    /* Scrub non-essential regs */
    "call __libia2_scrub_registers\n"
    /* Restore preserved regs */
    "popq %rax\n"
    /* Set PKRU to the caller's value */
    "movq %rax, %r10\n"
    "movq %rdx, %r11\n"
    "xorl %ecx, %ecx\n"
    "xorl %edx, %edx\n"
    "movl $0xfffffff0, %eax\n"
    "wrpkru\n"
    "movq %r10, %rax\n"
    "movq %r11, %rdx\n"
    "popq %r15\n"
    "popq %r14\n"
    "popq %r13\n"
    "popq %r12\n"
    "popq %rbx\n"
    "popq %rbp\n"
    /* Return to the caller */
    "ret\n"
    ".size __wrap_gzputc, .-__wrap_gzputc\n"
    ".previous\n"
);
asm(
    /* Wrapper for gzputs(int, int) -> int: */
    ".text\n"
    ".global __wrap_gzputs\n"
    ".type __wrap_gzputs, @function\n"
    "__wrap_gzputs:\n"
    "pushq %rbp\n"
    "movq %rsp, %rbp\n"
    "pushq %rbx\n"
    "pushq %r12\n"
    "pushq %r13\n"
    "pushq %r14\n"
    "pushq %r15\n"
    ASSERT_PKRU(0xfffffffffffffff0) "\n"
    /* Set PKRU to the intermediate value to move arguments */
    "movq %rcx, %r10\n"
    "movq %rdx, %r11\n"
    "xorl %ecx, %ecx\n"
    "xorl %edx, %edx\n"
    "movl $0xffffffc0, %eax\n"
    "wrpkru\n"
    "movq %r10, %rcx\n"
    "movq %r11, %rdx\n"
    /* Compute location to save old stack pointer (using r11) */
    "mov ia2_stackptr_1@GOTTPOFF(%rip), %r11\n"
    /* Write the old stack pointer to memory */
    "movq %rsp, %fs:(%r11)\n"
    /* Compute location to load new stack pointer (using r11) */
    "mov ia2_stackptr_2@GOTTPOFF(%rip), %r11\n"
    /* Read the new stack pointer from memory */
    "movq %fs:(%r11), %rsp\n"
    "subq $8, %rsp\n"
    /* Preserve essential regs on stack */
    "pushq %rdi\n"
    "pushq %rsi\n"
    /* Scrub non-essential regs */
    "call __libia2_scrub_registers\n"
    /* Restore preserved regs */
    "popq %rsi\n"
    "popq %rdi\n"
    /* Set PKRU to the compartment's value */
    "movq %rcx, %r10\n"
    "movq %rdx, %r11\n"
    "xorl %ecx, %ecx\n"
    "xorl %edx, %edx\n"
    "movl $0xffffffcc, %eax\n"
    "wrpkru\n"
    "movq %r10, %rcx\n"
    "movq %r11, %rdx\n"
    /* Call wrapped function */
    "call gzputs\n"
    /* Set PKRU to the intermediate value to move arguments */
    "movq %rax, %r10\n"
    "movq %rdx, %r11\n"
    "xorl %ecx, %ecx\n"
    "xorl %edx, %edx\n"
    "movl $0xffffffc0, %eax\n"
    "wrpkru\n"
    "movq %r10, %rax\n"
    "movq %r11, %rdx\n"
    /* Free stack space used for stack args */
    "addq $8, %rsp\n"
    /* Compute location to save old stack pointer (using r11) */
    "mov ia2_stackptr_2@GOTTPOFF(%rip), %r11\n"
    /* Write the old stack pointer to memory */
    "movq %rsp, %fs:(%r11)\n"
    /* Compute location to load new stack pointer (using r11) */
    "mov ia2_stackptr_1@GOTTPOFF(%rip), %r11\n"
    /* Read the new stack pointer from memory */
    "movq %fs:(%r11), %rsp\n"
    /* Preserve essential regs on stack */
    "pushq %rax\n"
    /* Scrub non-essential regs */
    "call __libia2_scrub_registers\n"
    /* Restore preserved regs */
    "popq %rax\n"
    /* Set PKRU to the caller's value */
    "movq %rax, %r10\n"
    "movq %rdx, %r11\n"
    "xorl %ecx, %ecx\n"
    "xorl %edx, %edx\n"
    "movl $0xfffffff0, %eax\n"
    "wrpkru\n"
    "movq %r10, %rax\n"
    "movq %r11, %rdx\n"
    "popq %r15\n"
    "popq %r14\n"
    "popq %r13\n"
    "popq %r12\n"
    "popq %rbx\n"
    "popq %rbp\n"
    /* Return to the caller */
    "ret\n"
    ".size __wrap_gzputs, .-__wrap_gzputs\n"
    ".previous\n"
);
asm(
    /* Wrapper for gzread(int, int, int) -> int: */
    ".text\n"
    ".global __wrap_gzread\n"
    ".type __wrap_gzread, @function\n"
    "__wrap_gzread:\n"
    "pushq %rbp\n"
    "movq %rsp, %rbp\n"
    "pushq %rbx\n"
    "pushq %r12\n"
    "pushq %r13\n"
    "pushq %r14\n"
    "pushq %r15\n"
    ASSERT_PKRU(0xfffffffffffffff0) "\n"
    /* Set PKRU to the intermediate value to move arguments */
    "movq %rcx, %r10\n"
    "movq %rdx, %r11\n"
    "xorl %ecx, %ecx\n"
    "xorl %edx, %edx\n"
    "movl $0xffffffc0, %eax\n"
    "wrpkru\n"
    "movq %r10, %rcx\n"
    "movq %r11, %rdx\n"
    /* Compute location to save old stack pointer (using r11) */
    "mov ia2_stackptr_1@GOTTPOFF(%rip), %r11\n"
    /* Write the old stack pointer to memory */
    "movq %rsp, %fs:(%r11)\n"
    /* Compute location to load new stack pointer (using r11) */
    "mov ia2_stackptr_2@GOTTPOFF(%rip), %r11\n"
    /* Read the new stack pointer from memory */
    "movq %fs:(%r11), %rsp\n"
    "subq $8, %rsp\n"
    /* Preserve essential regs on stack */
    "pushq %rdi\n"
    "pushq %rsi\n"
    "pushq %rdx\n"
    /* Scrub non-essential regs */
    "call __libia2_scrub_registers\n"
    /* Restore preserved regs */
    "popq %rdx\n"
    "popq %rsi\n"
    "popq %rdi\n"
    /* Set PKRU to the compartment's value */
    "movq %rcx, %r10\n"
    "movq %rdx, %r11\n"
    "xorl %ecx, %ecx\n"
    "xorl %edx, %edx\n"
    "movl $0xffffffcc, %eax\n"
    "wrpkru\n"
    "movq %r10, %rcx\n"
    "movq %r11, %rdx\n"
    /* Call wrapped function */
    "call gzread\n"
    /* Set PKRU to the intermediate value to move arguments */
    "movq %rax, %r10\n"
    "movq %rdx, %r11\n"
    "xorl %ecx, %ecx\n"
    "xorl %edx, %edx\n"
    "movl $0xffffffc0, %eax\n"
    "wrpkru\n"
    "movq %r10, %rax\n"
    "movq %r11, %rdx\n"
    /* Free stack space used for stack args */
    "addq $8, %rsp\n"
    /* Compute location to save old stack pointer (using r11) */
    "mov ia2_stackptr_2@GOTTPOFF(%rip), %r11\n"
    /* Write the old stack pointer to memory */
    "movq %rsp, %fs:(%r11)\n"
    /* Compute location to load new stack pointer (using r11) */
    "mov ia2_stackptr_1@GOTTPOFF(%rip), %r11\n"
    /* Read the new stack pointer from memory */
    "movq %fs:(%r11), %rsp\n"
    /* Preserve essential regs on stack */
    "pushq %rax\n"
    /* Scrub non-essential regs */
    "call __libia2_scrub_registers\n"
    /* Restore preserved regs */
    "popq %rax\n"
    /* Set PKRU to the caller's value */
    "movq %rax, %r10\n"
    "movq %rdx, %r11\n"
    "xorl %ecx, %ecx\n"
    "xorl %edx, %edx\n"
    "movl $0xfffffff0, %eax\n"
    "wrpkru\n"
    "movq %r10, %rax\n"
    "movq %r11, %rdx\n"
    "popq %r15\n"
    "popq %r14\n"
    "popq %r13\n"
    "popq %r12\n"
    "popq %rbx\n"
    "popq %rbp\n"
    /* Return to the caller */
    "ret\n"
    ".size __wrap_gzread, .-__wrap_gzread\n"
    ".previous\n"
);
asm(
    /* Wrapper for gzrewind(int) -> int: */
    ".text\n"
    ".global __wrap_gzrewind\n"
    ".type __wrap_gzrewind, @function\n"
    "__wrap_gzrewind:\n"
    "pushq %rbp\n"
    "movq %rsp, %rbp\n"
    "pushq %rbx\n"
    "pushq %r12\n"
    "pushq %r13\n"
    "pushq %r14\n"
    "pushq %r15\n"
    ASSERT_PKRU(0xfffffffffffffff0) "\n"
    /* Set PKRU to the intermediate value to move arguments */
    "movq %rcx, %r10\n"
    "movq %rdx, %r11\n"
    "xorl %ecx, %ecx\n"
    "xorl %edx, %edx\n"
    "movl $0xffffffc0, %eax\n"
    "wrpkru\n"
    "movq %r10, %rcx\n"
    "movq %r11, %rdx\n"
    /* Compute location to save old stack pointer (using r11) */
    "mov ia2_stackptr_1@GOTTPOFF(%rip), %r11\n"
    /* Write the old stack pointer to memory */
    "movq %rsp, %fs:(%r11)\n"
    /* Compute location to load new stack pointer (using r11) */
    "mov ia2_stackptr_2@GOTTPOFF(%rip), %r11\n"
    /* Read the new stack pointer from memory */
    "movq %fs:(%r11), %rsp\n"
    "subq $8, %rsp\n"
    /* Preserve essential regs on stack */
    "pushq %rdi\n"
    /* Scrub non-essential regs */
    "call __libia2_scrub_registers\n"
    /* Restore preserved regs */
    "popq %rdi\n"
    /* Set PKRU to the compartment's value */
    "movq %rcx, %r10\n"
    "movq %rdx, %r11\n"
    "xorl %ecx, %ecx\n"
    "xorl %edx, %edx\n"
    "movl $0xffffffcc, %eax\n"
    "wrpkru\n"
    "movq %r10, %rcx\n"
    "movq %r11, %rdx\n"
    /* Call wrapped function */
    "call gzrewind\n"
    /* Set PKRU to the intermediate value to move arguments */
    "movq %rax, %r10\n"
    "movq %rdx, %r11\n"
    "xorl %ecx, %ecx\n"
    "xorl %edx, %edx\n"
    "movl $0xffffffc0, %eax\n"
    "wrpkru\n"
    "movq %r10, %rax\n"
    "movq %r11, %rdx\n"
    /* Free stack space used for stack args */
    "addq $8, %rsp\n"
    /* Compute location to save old stack pointer (using r11) */
    "mov ia2_stackptr_2@GOTTPOFF(%rip), %r11\n"
    /* Write the old stack pointer to memory */
    "movq %rsp, %fs:(%r11)\n"
    /* Compute location to load new stack pointer (using r11) */
    "mov ia2_stackptr_1@GOTTPOFF(%rip), %r11\n"
    /* Read the new stack pointer from memory */
    "movq %fs:(%r11), %rsp\n"
    /* Preserve essential regs on stack */
    "pushq %rax\n"
    /* Scrub non-essential regs */
    "call __libia2_scrub_registers\n"
    /* Restore preserved regs */
    "popq %rax\n"
    /* Set PKRU to the caller's value */
    "movq %rax, %r10\n"
    "movq %rdx, %r11\n"
    "xorl %ecx, %ecx\n"
    "xorl %edx, %edx\n"
    "movl $0xfffffff0, %eax\n"
    "wrpkru\n"
    "movq %r10, %rax\n"
    "movq %r11, %rdx\n"
    "popq %r15\n"
    "popq %r14\n"
    "popq %r13\n"
    "popq %r12\n"
    "popq %rbx\n"
    "popq %rbp\n"
    /* Return to the caller */
    "ret\n"
    ".size __wrap_gzrewind, .-__wrap_gzrewind\n"
    ".previous\n"
);
asm(
    /* Wrapper for gzseek(int, int, int) -> int: */
    ".text\n"
    ".global __wrap_gzseek\n"
    ".type __wrap_gzseek, @function\n"
    "__wrap_gzseek:\n"
    "pushq %rbp\n"
    "movq %rsp, %rbp\n"
    "pushq %rbx\n"
    "pushq %r12\n"
    "pushq %r13\n"
    "pushq %r14\n"
    "pushq %r15\n"
    ASSERT_PKRU(0xfffffffffffffff0) "\n"
    /* Set PKRU to the intermediate value to move arguments */
    "movq %rcx, %r10\n"
    "movq %rdx, %r11\n"
    "xorl %ecx, %ecx\n"
    "xorl %edx, %edx\n"
    "movl $0xffffffc0, %eax\n"
    "wrpkru\n"
    "movq %r10, %rcx\n"
    "movq %r11, %rdx\n"
    /* Compute location to save old stack pointer (using r11) */
    "mov ia2_stackptr_1@GOTTPOFF(%rip), %r11\n"
    /* Write the old stack pointer to memory */
    "movq %rsp, %fs:(%r11)\n"
    /* Compute location to load new stack pointer (using r11) */
    "mov ia2_stackptr_2@GOTTPOFF(%rip), %r11\n"
    /* Read the new stack pointer from memory */
    "movq %fs:(%r11), %rsp\n"
    "subq $8, %rsp\n"
    /* Preserve essential regs on stack */
    "pushq %rdi\n"
    "pushq %rsi\n"
    "pushq %rdx\n"
    /* Scrub non-essential regs */
    "call __libia2_scrub_registers\n"
    /* Restore preserved regs */
    "popq %rdx\n"
    "popq %rsi\n"
    "popq %rdi\n"
    /* Set PKRU to the compartment's value */
    "movq %rcx, %r10\n"
    "movq %rdx, %r11\n"
    "xorl %ecx, %ecx\n"
    "xorl %edx, %edx\n"
    "movl $0xffffffcc, %eax\n"
    "wrpkru\n"
    "movq %r10, %rcx\n"
    "movq %r11, %rdx\n"
    /* Call wrapped function */
    "call gzseek\n"
    /* Set PKRU to the intermediate value to move arguments */
    "movq %rax, %r10\n"
    "movq %rdx, %r11\n"
    "xorl %ecx, %ecx\n"
    "xorl %edx, %edx\n"
    "movl $0xffffffc0, %eax\n"
    "wrpkru\n"
    "movq %r10, %rax\n"
    "movq %r11, %rdx\n"
    /* Free stack space used for stack args */
    "addq $8, %rsp\n"
    /* Compute location to save old stack pointer (using r11) */
    "mov ia2_stackptr_2@GOTTPOFF(%rip), %r11\n"
    /* Write the old stack pointer to memory */
    "movq %rsp, %fs:(%r11)\n"
    /* Compute location to load new stack pointer (using r11) */
    "mov ia2_stackptr_1@GOTTPOFF(%rip), %r11\n"
    /* Read the new stack pointer from memory */
    "movq %fs:(%r11), %rsp\n"
    /* Preserve essential regs on stack */
    "pushq %rax\n"
    /* Scrub non-essential regs */
    "call __libia2_scrub_registers\n"
    /* Restore preserved regs */
    "popq %rax\n"
    /* Set PKRU to the caller's value */
    "movq %rax, %r10\n"
    "movq %rdx, %r11\n"
    "xorl %ecx, %ecx\n"
    "xorl %edx, %edx\n"
    "movl $0xfffffff0, %eax\n"
    "wrpkru\n"
    "movq %r10, %rax\n"
    "movq %r11, %rdx\n"
    "popq %r15\n"
    "popq %r14\n"
    "popq %r13\n"
    "popq %r12\n"
    "popq %rbx\n"
    "popq %rbp\n"
    /* Return to the caller */
    "ret\n"
    ".size __wrap_gzseek, .-__wrap_gzseek\n"
    ".previous\n"
);
asm(
    /* Wrapper for gzseek64(int, int, int) -> int: */
    ".text\n"
    ".global __wrap_gzseek64\n"
    ".type __wrap_gzseek64, @function\n"
    "__wrap_gzseek64:\n"
    "pushq %rbp\n"
    "movq %rsp, %rbp\n"
    "pushq %rbx\n"
    "pushq %r12\n"
    "pushq %r13\n"
    "pushq %r14\n"
    "pushq %r15\n"
    ASSERT_PKRU(0xfffffffffffffff0) "\n"
    /* Set PKRU to the intermediate value to move arguments */
    "movq %rcx, %r10\n"
    "movq %rdx, %r11\n"
    "xorl %ecx, %ecx\n"
    "xorl %edx, %edx\n"
    "movl $0xffffffc0, %eax\n"
    "wrpkru\n"
    "movq %r10, %rcx\n"
    "movq %r11, %rdx\n"
    /* Compute location to save old stack pointer (using r11) */
    "mov ia2_stackptr_1@GOTTPOFF(%rip), %r11\n"
    /* Write the old stack pointer to memory */
    "movq %rsp, %fs:(%r11)\n"
    /* Compute location to load new stack pointer (using r11) */
    "mov ia2_stackptr_2@GOTTPOFF(%rip), %r11\n"
    /* Read the new stack pointer from memory */
    "movq %fs:(%r11), %rsp\n"
    "subq $8, %rsp\n"
    /* Preserve essential regs on stack */
    "pushq %rdi\n"
    "pushq %rsi\n"
    "pushq %rdx\n"
    /* Scrub non-essential regs */
    "call __libia2_scrub_registers\n"
    /* Restore preserved regs */
    "popq %rdx\n"
    "popq %rsi\n"
    "popq %rdi\n"
    /* Set PKRU to the compartment's value */
    "movq %rcx, %r10\n"
    "movq %rdx, %r11\n"
    "xorl %ecx, %ecx\n"
    "xorl %edx, %edx\n"
    "movl $0xffffffcc, %eax\n"
    "wrpkru\n"
    "movq %r10, %rcx\n"
    "movq %r11, %rdx\n"
    /* Call wrapped function */
    "call gzseek64\n"
    /* Set PKRU to the intermediate value to move arguments */
    "movq %rax, %r10\n"
    "movq %rdx, %r11\n"
    "xorl %ecx, %ecx\n"
    "xorl %edx, %edx\n"
    "movl $0xffffffc0, %eax\n"
    "wrpkru\n"
    "movq %r10, %rax\n"
    "movq %r11, %rdx\n"
    /* Free stack space used for stack args */
    "addq $8, %rsp\n"
    /* Compute location to save old stack pointer (using r11) */
    "mov ia2_stackptr_2@GOTTPOFF(%rip), %r11\n"
    /* Write the old stack pointer to memory */
    "movq %rsp, %fs:(%r11)\n"
    /* Compute location to load new stack pointer (using r11) */
    "mov ia2_stackptr_1@GOTTPOFF(%rip), %r11\n"
    /* Read the new stack pointer from memory */
    "movq %fs:(%r11), %rsp\n"
    /* Preserve essential regs on stack */
    "pushq %rax\n"
    /* Scrub non-essential regs */
    "call __libia2_scrub_registers\n"
    /* Restore preserved regs */
    "popq %rax\n"
    /* Set PKRU to the caller's value */
    "movq %rax, %r10\n"
    "movq %rdx, %r11\n"
    "xorl %ecx, %ecx\n"
    "xorl %edx, %edx\n"
    "movl $0xfffffff0, %eax\n"
    "wrpkru\n"
    "movq %r10, %rax\n"
    "movq %r11, %rdx\n"
    "popq %r15\n"
    "popq %r14\n"
    "popq %r13\n"
    "popq %r12\n"
    "popq %rbx\n"
    "popq %rbp\n"
    /* Return to the caller */
    "ret\n"
    ".size __wrap_gzseek64, .-__wrap_gzseek64\n"
    ".previous\n"
);
asm(
    /* Wrapper for gzsetparams(int, int, int) -> int: */
    ".text\n"
    ".global __wrap_gzsetparams\n"
    ".type __wrap_gzsetparams, @function\n"
    "__wrap_gzsetparams:\n"
    "pushq %rbp\n"
    "movq %rsp, %rbp\n"
    "pushq %rbx\n"
    "pushq %r12\n"
    "pushq %r13\n"
    "pushq %r14\n"
    "pushq %r15\n"
    ASSERT_PKRU(0xfffffffffffffff0) "\n"
    /* Set PKRU to the intermediate value to move arguments */
    "movq %rcx, %r10\n"
    "movq %rdx, %r11\n"
    "xorl %ecx, %ecx\n"
    "xorl %edx, %edx\n"
    "movl $0xffffffc0, %eax\n"
    "wrpkru\n"
    "movq %r10, %rcx\n"
    "movq %r11, %rdx\n"
    /* Compute location to save old stack pointer (using r11) */
    "mov ia2_stackptr_1@GOTTPOFF(%rip), %r11\n"
    /* Write the old stack pointer to memory */
    "movq %rsp, %fs:(%r11)\n"
    /* Compute location to load new stack pointer (using r11) */
    "mov ia2_stackptr_2@GOTTPOFF(%rip), %r11\n"
    /* Read the new stack pointer from memory */
    "movq %fs:(%r11), %rsp\n"
    "subq $8, %rsp\n"
    /* Preserve essential regs on stack */
    "pushq %rdi\n"
    "pushq %rsi\n"
    "pushq %rdx\n"
    /* Scrub non-essential regs */
    "call __libia2_scrub_registers\n"
    /* Restore preserved regs */
    "popq %rdx\n"
    "popq %rsi\n"
    "popq %rdi\n"
    /* Set PKRU to the compartment's value */
    "movq %rcx, %r10\n"
    "movq %rdx, %r11\n"
    "xorl %ecx, %ecx\n"
    "xorl %edx, %edx\n"
    "movl $0xffffffcc, %eax\n"
    "wrpkru\n"
    "movq %r10, %rcx\n"
    "movq %r11, %rdx\n"
    /* Call wrapped function */
    "call gzsetparams\n"
    /* Set PKRU to the intermediate value to move arguments */
    "movq %rax, %r10\n"
    "movq %rdx, %r11\n"
    "xorl %ecx, %ecx\n"
    "xorl %edx, %edx\n"
    "movl $0xffffffc0, %eax\n"
    "wrpkru\n"
    "movq %r10, %rax\n"
    "movq %r11, %rdx\n"
    /* Free stack space used for stack args */
    "addq $8, %rsp\n"
    /* Compute location to save old stack pointer (using r11) */
    "mov ia2_stackptr_2@GOTTPOFF(%rip), %r11\n"
    /* Write the old stack pointer to memory */
    "movq %rsp, %fs:(%r11)\n"
    /* Compute location to load new stack pointer (using r11) */
    "mov ia2_stackptr_1@GOTTPOFF(%rip), %r11\n"
    /* Read the new stack pointer from memory */
    "movq %fs:(%r11), %rsp\n"
    /* Preserve essential regs on stack */
    "pushq %rax\n"
    /* Scrub non-essential regs */
    "call __libia2_scrub_registers\n"
    /* Restore preserved regs */
    "popq %rax\n"
    /* Set PKRU to the caller's value */
    "movq %rax, %r10\n"
    "movq %rdx, %r11\n"
    "xorl %ecx, %ecx\n"
    "xorl %edx, %edx\n"
    "movl $0xfffffff0, %eax\n"
    "wrpkru\n"
    "movq %r10, %rax\n"
    "movq %r11, %rdx\n"
    "popq %r15\n"
    "popq %r14\n"
    "popq %r13\n"
    "popq %r12\n"
    "popq %rbx\n"
    "popq %rbp\n"
    /* Return to the caller */
    "ret\n"
    ".size __wrap_gzsetparams, .-__wrap_gzsetparams\n"
    ".previous\n"
);
asm(
    /* Wrapper for gztell(int) -> int: */
    ".text\n"
    ".global __wrap_gztell\n"
    ".type __wrap_gztell, @function\n"
    "__wrap_gztell:\n"
    "pushq %rbp\n"
    "movq %rsp, %rbp\n"
    "pushq %rbx\n"
    "pushq %r12\n"
    "pushq %r13\n"
    "pushq %r14\n"
    "pushq %r15\n"
    ASSERT_PKRU(0xfffffffffffffff0) "\n"
    /* Set PKRU to the intermediate value to move arguments */
    "movq %rcx, %r10\n"
    "movq %rdx, %r11\n"
    "xorl %ecx, %ecx\n"
    "xorl %edx, %edx\n"
    "movl $0xffffffc0, %eax\n"
    "wrpkru\n"
    "movq %r10, %rcx\n"
    "movq %r11, %rdx\n"
    /* Compute location to save old stack pointer (using r11) */
    "mov ia2_stackptr_1@GOTTPOFF(%rip), %r11\n"
    /* Write the old stack pointer to memory */
    "movq %rsp, %fs:(%r11)\n"
    /* Compute location to load new stack pointer (using r11) */
    "mov ia2_stackptr_2@GOTTPOFF(%rip), %r11\n"
    /* Read the new stack pointer from memory */
    "movq %fs:(%r11), %rsp\n"
    "subq $8, %rsp\n"
    /* Preserve essential regs on stack */
    "pushq %rdi\n"
    /* Scrub non-essential regs */
    "call __libia2_scrub_registers\n"
    /* Restore preserved regs */
    "popq %rdi\n"
    /* Set PKRU to the compartment's value */
    "movq %rcx, %r10\n"
    "movq %rdx, %r11\n"
    "xorl %ecx, %ecx\n"
    "xorl %edx, %edx\n"
    "movl $0xffffffcc, %eax\n"
    "wrpkru\n"
    "movq %r10, %rcx\n"
    "movq %r11, %rdx\n"
    /* Call wrapped function */
    "call gztell\n"
    /* Set PKRU to the intermediate value to move arguments */
    "movq %rax, %r10\n"
    "movq %rdx, %r11\n"
    "xorl %ecx, %ecx\n"
    "xorl %edx, %edx\n"
    "movl $0xffffffc0, %eax\n"
    "wrpkru\n"
    "movq %r10, %rax\n"
    "movq %r11, %rdx\n"
    /* Free stack space used for stack args */
    "addq $8, %rsp\n"
    /* Compute location to save old stack pointer (using r11) */
    "mov ia2_stackptr_2@GOTTPOFF(%rip), %r11\n"
    /* Write the old stack pointer to memory */
    "movq %rsp, %fs:(%r11)\n"
    /* Compute location to load new stack pointer (using r11) */
    "mov ia2_stackptr_1@GOTTPOFF(%rip), %r11\n"
    /* Read the new stack pointer from memory */
    "movq %fs:(%r11), %rsp\n"
    /* Preserve essential regs on stack */
    "pushq %rax\n"
    /* Scrub non-essential regs */
    "call __libia2_scrub_registers\n"
    /* Restore preserved regs */
    "popq %rax\n"
    /* Set PKRU to the caller's value */
    "movq %rax, %r10\n"
    "movq %rdx, %r11\n"
    "xorl %ecx, %ecx\n"
    "xorl %edx, %edx\n"
    "movl $0xfffffff0, %eax\n"
    "wrpkru\n"
    "movq %r10, %rax\n"
    "movq %r11, %rdx\n"
    "popq %r15\n"
    "popq %r14\n"
    "popq %r13\n"
    "popq %r12\n"
    "popq %rbx\n"
    "popq %rbp\n"
    /* Return to the caller */
    "ret\n"
    ".size __wrap_gztell, .-__wrap_gztell\n"
    ".previous\n"
);
asm(
    /* Wrapper for gztell64(int) -> int: */
    ".text\n"
    ".global __wrap_gztell64\n"
    ".type __wrap_gztell64, @function\n"
    "__wrap_gztell64:\n"
    "pushq %rbp\n"
    "movq %rsp, %rbp\n"
    "pushq %rbx\n"
    "pushq %r12\n"
    "pushq %r13\n"
    "pushq %r14\n"
    "pushq %r15\n"
    ASSERT_PKRU(0xfffffffffffffff0) "\n"
    /* Set PKRU to the intermediate value to move arguments */
    "movq %rcx, %r10\n"
    "movq %rdx, %r11\n"
    "xorl %ecx, %ecx\n"
    "xorl %edx, %edx\n"
    "movl $0xffffffc0, %eax\n"
    "wrpkru\n"
    "movq %r10, %rcx\n"
    "movq %r11, %rdx\n"
    /* Compute location to save old stack pointer (using r11) */
    "mov ia2_stackptr_1@GOTTPOFF(%rip), %r11\n"
    /* Write the old stack pointer to memory */
    "movq %rsp, %fs:(%r11)\n"
    /* Compute location to load new stack pointer (using r11) */
    "mov ia2_stackptr_2@GOTTPOFF(%rip), %r11\n"
    /* Read the new stack pointer from memory */
    "movq %fs:(%r11), %rsp\n"
    "subq $8, %rsp\n"
    /* Preserve essential regs on stack */
    "pushq %rdi\n"
    /* Scrub non-essential regs */
    "call __libia2_scrub_registers\n"
    /* Restore preserved regs */
    "popq %rdi\n"
    /* Set PKRU to the compartment's value */
    "movq %rcx, %r10\n"
    "movq %rdx, %r11\n"
    "xorl %ecx, %ecx\n"
    "xorl %edx, %edx\n"
    "movl $0xffffffcc, %eax\n"
    "wrpkru\n"
    "movq %r10, %rcx\n"
    "movq %r11, %rdx\n"
    /* Call wrapped function */
    "call gztell64\n"
    /* Set PKRU to the intermediate value to move arguments */
    "movq %rax, %r10\n"
    "movq %rdx, %r11\n"
    "xorl %ecx, %ecx\n"
    "xorl %edx, %edx\n"
    "movl $0xffffffc0, %eax\n"
    "wrpkru\n"
    "movq %r10, %rax\n"
    "movq %r11, %rdx\n"
    /* Free stack space used for stack args */
    "addq $8, %rsp\n"
    /* Compute location to save old stack pointer (using r11) */
    "mov ia2_stackptr_2@GOTTPOFF(%rip), %r11\n"
    /* Write the old stack pointer to memory */
    "movq %rsp, %fs:(%r11)\n"
    /* Compute location to load new stack pointer (using r11) */
    "mov ia2_stackptr_1@GOTTPOFF(%rip), %r11\n"
    /* Read the new stack pointer from memory */
    "movq %fs:(%r11), %rsp\n"
    /* Preserve essential regs on stack */
    "pushq %rax\n"
    /* Scrub non-essential regs */
    "call __libia2_scrub_registers\n"
    /* Restore preserved regs */
    "popq %rax\n"
    /* Set PKRU to the caller's value */
    "movq %rax, %r10\n"
    "movq %rdx, %r11\n"
    "xorl %ecx, %ecx\n"
    "xorl %edx, %edx\n"
    "movl $0xfffffff0, %eax\n"
    "wrpkru\n"
    "movq %r10, %rax\n"
    "movq %r11, %rdx\n"
    "popq %r15\n"
    "popq %r14\n"
    "popq %r13\n"
    "popq %r12\n"
    "popq %rbx\n"
    "popq %rbp\n"
    /* Return to the caller */
    "ret\n"
    ".size __wrap_gztell64, .-__wrap_gztell64\n"
    ".previous\n"
);
asm(
    /* Wrapper for gzungetc(int, int) -> int: */
    ".text\n"
    ".global __wrap_gzungetc\n"
    ".type __wrap_gzungetc, @function\n"
    "__wrap_gzungetc:\n"
    "pushq %rbp\n"
    "movq %rsp, %rbp\n"
    "pushq %rbx\n"
    "pushq %r12\n"
    "pushq %r13\n"
    "pushq %r14\n"
    "pushq %r15\n"
    ASSERT_PKRU(0xfffffffffffffff0) "\n"
    /* Set PKRU to the intermediate value to move arguments */
    "movq %rcx, %r10\n"
    "movq %rdx, %r11\n"
    "xorl %ecx, %ecx\n"
    "xorl %edx, %edx\n"
    "movl $0xffffffc0, %eax\n"
    "wrpkru\n"
    "movq %r10, %rcx\n"
    "movq %r11, %rdx\n"
    /* Compute location to save old stack pointer (using r11) */
    "mov ia2_stackptr_1@GOTTPOFF(%rip), %r11\n"
    /* Write the old stack pointer to memory */
    "movq %rsp, %fs:(%r11)\n"
    /* Compute location to load new stack pointer (using r11) */
    "mov ia2_stackptr_2@GOTTPOFF(%rip), %r11\n"
    /* Read the new stack pointer from memory */
    "movq %fs:(%r11), %rsp\n"
    "subq $8, %rsp\n"
    /* Preserve essential regs on stack */
    "pushq %rdi\n"
    "pushq %rsi\n"
    /* Scrub non-essential regs */
    "call __libia2_scrub_registers\n"
    /* Restore preserved regs */
    "popq %rsi\n"
    "popq %rdi\n"
    /* Set PKRU to the compartment's value */
    "movq %rcx, %r10\n"
    "movq %rdx, %r11\n"
    "xorl %ecx, %ecx\n"
    "xorl %edx, %edx\n"
    "movl $0xffffffcc, %eax\n"
    "wrpkru\n"
    "movq %r10, %rcx\n"
    "movq %r11, %rdx\n"
    /* Call wrapped function */
    "call gzungetc\n"
    /* Set PKRU to the intermediate value to move arguments */
    "movq %rax, %r10\n"
    "movq %rdx, %r11\n"
    "xorl %ecx, %ecx\n"
    "xorl %edx, %edx\n"
    "movl $0xffffffc0, %eax\n"
    "wrpkru\n"
    "movq %r10, %rax\n"
    "movq %r11, %rdx\n"
    /* Free stack space used for stack args */
    "addq $8, %rsp\n"
    /* Compute location to save old stack pointer (using r11) */
    "mov ia2_stackptr_2@GOTTPOFF(%rip), %r11\n"
    /* Write the old stack pointer to memory */
    "movq %rsp, %fs:(%r11)\n"
    /* Compute location to load new stack pointer (using r11) */
    "mov ia2_stackptr_1@GOTTPOFF(%rip), %r11\n"
    /* Read the new stack pointer from memory */
    "movq %fs:(%r11), %rsp\n"
    /* Preserve essential regs on stack */
    "pushq %rax\n"
    /* Scrub non-essential regs */
    "call __libia2_scrub_registers\n"
    /* Restore preserved regs */
    "popq %rax\n"
    /* Set PKRU to the caller's value */
    "movq %rax, %r10\n"
    "movq %rdx, %r11\n"
    "xorl %ecx, %ecx\n"
    "xorl %edx, %edx\n"
    "movl $0xfffffff0, %eax\n"
    "wrpkru\n"
    "movq %r10, %rax\n"
    "movq %r11, %rdx\n"
    "popq %r15\n"
    "popq %r14\n"
    "popq %r13\n"
    "popq %r12\n"
    "popq %rbx\n"
    "popq %rbp\n"
    /* Return to the caller */
    "ret\n"
    ".size __wrap_gzungetc, .-__wrap_gzungetc\n"
    ".previous\n"
);
asm(
    /* Wrapper for gzvprintf(int, int, int) -> int: */
    ".text\n"
    ".global __wrap_gzvprintf\n"
    ".type __wrap_gzvprintf, @function\n"
    "__wrap_gzvprintf:\n"
    "pushq %rbp\n"
    "movq %rsp, %rbp\n"
    "pushq %rbx\n"
    "pushq %r12\n"
    "pushq %r13\n"
    "pushq %r14\n"
    "pushq %r15\n"
    ASSERT_PKRU(0xfffffffffffffff0) "\n"
    /* Set PKRU to the intermediate value to move arguments */
    "movq %rcx, %r10\n"
    "movq %rdx, %r11\n"
    "xorl %ecx, %ecx\n"
    "xorl %edx, %edx\n"
    "movl $0xffffffc0, %eax\n"
    "wrpkru\n"
    "movq %r10, %rcx\n"
    "movq %r11, %rdx\n"
    /* Compute location to save old stack pointer (using r11) */
    "mov ia2_stackptr_1@GOTTPOFF(%rip), %r11\n"
    /* Write the old stack pointer to memory */
    "movq %rsp, %fs:(%r11)\n"
    /* Compute location to load new stack pointer (using r11) */
    "mov ia2_stackptr_2@GOTTPOFF(%rip), %r11\n"
    /* Read the new stack pointer from memory */
    "movq %fs:(%r11), %rsp\n"
    "subq $8, %rsp\n"
    /* Preserve essential regs on stack */
    "pushq %rdi\n"
    "pushq %rsi\n"
    "pushq %rdx\n"
    /* Scrub non-essential regs */
    "call __libia2_scrub_registers\n"
    /* Restore preserved regs */
    "popq %rdx\n"
    "popq %rsi\n"
    "popq %rdi\n"
    /* Set PKRU to the compartment's value */
    "movq %rcx, %r10\n"
    "movq %rdx, %r11\n"
    "xorl %ecx, %ecx\n"
    "xorl %edx, %edx\n"
    "movl $0xffffffcc, %eax\n"
    "wrpkru\n"
    "movq %r10, %rcx\n"
    "movq %r11, %rdx\n"
    /* Call wrapped function */
    "call gzvprintf\n"
    /* Set PKRU to the intermediate value to move arguments */
    "movq %rax, %r10\n"
    "movq %rdx, %r11\n"
    "xorl %ecx, %ecx\n"
    "xorl %edx, %edx\n"
    "movl $0xffffffc0, %eax\n"
    "wrpkru\n"
    "movq %r10, %rax\n"
    "movq %r11, %rdx\n"
    /* Free stack space used for stack args */
    "addq $8, %rsp\n"
    /* Compute location to save old stack pointer (using r11) */
    "mov ia2_stackptr_2@GOTTPOFF(%rip), %r11\n"
    /* Write the old stack pointer to memory */
    "movq %rsp, %fs:(%r11)\n"
    /* Compute location to load new stack pointer (using r11) */
    "mov ia2_stackptr_1@GOTTPOFF(%rip), %r11\n"
    /* Read the new stack pointer from memory */
    "movq %fs:(%r11), %rsp\n"
    /* Preserve essential regs on stack */
    "pushq %rax\n"
    /* Scrub non-essential regs */
    "call __libia2_scrub_registers\n"
    /* Restore preserved regs */
    "popq %rax\n"
    /* Set PKRU to the caller's value */
    "movq %rax, %r10\n"
    "movq %rdx, %r11\n"
    "xorl %ecx, %ecx\n"
    "xorl %edx, %edx\n"
    "movl $0xfffffff0, %eax\n"
    "wrpkru\n"
    "movq %r10, %rax\n"
    "movq %r11, %rdx\n"
    "popq %r15\n"
    "popq %r14\n"
    "popq %r13\n"
    "popq %r12\n"
    "popq %rbx\n"
    "popq %rbp\n"
    /* Return to the caller */
    "ret\n"
    ".size __wrap_gzvprintf, .-__wrap_gzvprintf\n"
    ".previous\n"
);
asm(
    /* Wrapper for gzwrite(int, int, int) -> int: */
    ".text\n"
    ".global __wrap_gzwrite\n"
    ".type __wrap_gzwrite, @function\n"
    "__wrap_gzwrite:\n"
    "pushq %rbp\n"
    "movq %rsp, %rbp\n"
    "pushq %rbx\n"
    "pushq %r12\n"
    "pushq %r13\n"
    "pushq %r14\n"
    "pushq %r15\n"
    ASSERT_PKRU(0xfffffffffffffff0) "\n"
    /* Set PKRU to the intermediate value to move arguments */
    "movq %rcx, %r10\n"
    "movq %rdx, %r11\n"
    "xorl %ecx, %ecx\n"
    "xorl %edx, %edx\n"
    "movl $0xffffffc0, %eax\n"
    "wrpkru\n"
    "movq %r10, %rcx\n"
    "movq %r11, %rdx\n"
    /* Compute location to save old stack pointer (using r11) */
    "mov ia2_stackptr_1@GOTTPOFF(%rip), %r11\n"
    /* Write the old stack pointer to memory */
    "movq %rsp, %fs:(%r11)\n"
    /* Compute location to load new stack pointer (using r11) */
    "mov ia2_stackptr_2@GOTTPOFF(%rip), %r11\n"
    /* Read the new stack pointer from memory */
    "movq %fs:(%r11), %rsp\n"
    "subq $8, %rsp\n"
    /* Preserve essential regs on stack */
    "pushq %rdi\n"
    "pushq %rsi\n"
    "pushq %rdx\n"
    /* Scrub non-essential regs */
    "call __libia2_scrub_registers\n"
    /* Restore preserved regs */
    "popq %rdx\n"
    "popq %rsi\n"
    "popq %rdi\n"
    /* Set PKRU to the compartment's value */
    "movq %rcx, %r10\n"
    "movq %rdx, %r11\n"
    "xorl %ecx, %ecx\n"
    "xorl %edx, %edx\n"
    "movl $0xffffffcc, %eax\n"
    "wrpkru\n"
    "movq %r10, %rcx\n"
    "movq %r11, %rdx\n"
    /* Call wrapped function */
    "call gzwrite\n"
    /* Set PKRU to the intermediate value to move arguments */
    "movq %rax, %r10\n"
    "movq %rdx, %r11\n"
    "xorl %ecx, %ecx\n"
    "xorl %edx, %edx\n"
    "movl $0xffffffc0, %eax\n"
    "wrpkru\n"
    "movq %r10, %rax\n"
    "movq %r11, %rdx\n"
    /* Free stack space used for stack args */
    "addq $8, %rsp\n"
    /* Compute location to save old stack pointer (using r11) */
    "mov ia2_stackptr_2@GOTTPOFF(%rip), %r11\n"
    /* Write the old stack pointer to memory */
    "movq %rsp, %fs:(%r11)\n"
    /* Compute location to load new stack pointer (using r11) */
    "mov ia2_stackptr_1@GOTTPOFF(%rip), %r11\n"
    /* Read the new stack pointer from memory */
    "movq %fs:(%r11), %rsp\n"
    /* Preserve essential regs on stack */
    "pushq %rax\n"
    /* Scrub non-essential regs */
    "call __libia2_scrub_registers\n"
    /* Restore preserved regs */
    "popq %rax\n"
    /* Set PKRU to the caller's value */
    "movq %rax, %r10\n"
    "movq %rdx, %r11\n"
    "xorl %ecx, %ecx\n"
    "xorl %edx, %edx\n"
    "movl $0xfffffff0, %eax\n"
    "wrpkru\n"
    "movq %r10, %rax\n"
    "movq %r11, %rdx\n"
    "popq %r15\n"
    "popq %r14\n"
    "popq %r13\n"
    "popq %r12\n"
    "popq %rbx\n"
    "popq %rbp\n"
    /* Return to the caller */
    "ret\n"
    ".size __wrap_gzwrite, .-__wrap_gzwrite\n"
    ".previous\n"
);
asm(
    /* Wrapper for inflate(int, int) -> int: */
    ".text\n"
    ".global __wrap_inflate\n"
    ".type __wrap_inflate, @function\n"
    "__wrap_inflate:\n"
    "pushq %rbp\n"
    "movq %rsp, %rbp\n"
    "pushq %rbx\n"
    "pushq %r12\n"
    "pushq %r13\n"
    "pushq %r14\n"
    "pushq %r15\n"
    ASSERT_PKRU(0xfffffffffffffff0) "\n"
    /* Set PKRU to the intermediate value to move arguments */
    "movq %rcx, %r10\n"
    "movq %rdx, %r11\n"
    "xorl %ecx, %ecx\n"
    "xorl %edx, %edx\n"
    "movl $0xffffffc0, %eax\n"
    "wrpkru\n"
    "movq %r10, %rcx\n"
    "movq %r11, %rdx\n"
    /* Compute location to save old stack pointer (using r11) */
    "mov ia2_stackptr_1@GOTTPOFF(%rip), %r11\n"
    /* Write the old stack pointer to memory */
    "movq %rsp, %fs:(%r11)\n"
    /* Compute location to load new stack pointer (using r11) */
    "mov ia2_stackptr_2@GOTTPOFF(%rip), %r11\n"
    /* Read the new stack pointer from memory */
    "movq %fs:(%r11), %rsp\n"
    "subq $8, %rsp\n"
    /* Preserve essential regs on stack */
    "pushq %rdi\n"
    "pushq %rsi\n"
    /* Scrub non-essential regs */
    "call __libia2_scrub_registers\n"
    /* Restore preserved regs */
    "popq %rsi\n"
    "popq %rdi\n"
    /* Set PKRU to the compartment's value */
    "movq %rcx, %r10\n"
    "movq %rdx, %r11\n"
    "xorl %ecx, %ecx\n"
    "xorl %edx, %edx\n"
    "movl $0xffffffcc, %eax\n"
    "wrpkru\n"
    "movq %r10, %rcx\n"
    "movq %r11, %rdx\n"
    /* Call wrapped function */
    "call inflate\n"
    /* Set PKRU to the intermediate value to move arguments */
    "movq %rax, %r10\n"
    "movq %rdx, %r11\n"
    "xorl %ecx, %ecx\n"
    "xorl %edx, %edx\n"
    "movl $0xffffffc0, %eax\n"
    "wrpkru\n"
    "movq %r10, %rax\n"
    "movq %r11, %rdx\n"
    /* Free stack space used for stack args */
    "addq $8, %rsp\n"
    /* Compute location to save old stack pointer (using r11) */
    "mov ia2_stackptr_2@GOTTPOFF(%rip), %r11\n"
    /* Write the old stack pointer to memory */
    "movq %rsp, %fs:(%r11)\n"
    /* Compute location to load new stack pointer (using r11) */
    "mov ia2_stackptr_1@GOTTPOFF(%rip), %r11\n"
    /* Read the new stack pointer from memory */
    "movq %fs:(%r11), %rsp\n"
    /* Preserve essential regs on stack */
    "pushq %rax\n"
    /* Scrub non-essential regs */
    "call __libia2_scrub_registers\n"
    /* Restore preserved regs */
    "popq %rax\n"
    /* Set PKRU to the caller's value */
    "movq %rax, %r10\n"
    "movq %rdx, %r11\n"
    "xorl %ecx, %ecx\n"
    "xorl %edx, %edx\n"
    "movl $0xfffffff0, %eax\n"
    "wrpkru\n"
    "movq %r10, %rax\n"
    "movq %r11, %rdx\n"
    "popq %r15\n"
    "popq %r14\n"
    "popq %r13\n"
    "popq %r12\n"
    "popq %rbx\n"
    "popq %rbp\n"
    /* Return to the caller */
    "ret\n"
    ".size __wrap_inflate, .-__wrap_inflate\n"
    ".previous\n"
);
asm(
    /* Wrapper for inflateBack(int, int, int, int, int) -> int: */
    ".text\n"
    ".global __wrap_inflateBack\n"
    ".type __wrap_inflateBack, @function\n"
    "__wrap_inflateBack:\n"
    "pushq %rbp\n"
    "movq %rsp, %rbp\n"
    "pushq %rbx\n"
    "pushq %r12\n"
    "pushq %r13\n"
    "pushq %r14\n"
    "pushq %r15\n"
    ASSERT_PKRU(0xfffffffffffffff0) "\n"
    /* Set PKRU to the intermediate value to move arguments */
    "movq %rcx, %r10\n"
    "movq %rdx, %r11\n"
    "xorl %ecx, %ecx\n"
    "xorl %edx, %edx\n"
    "movl $0xffffffc0, %eax\n"
    "wrpkru\n"
    "movq %r10, %rcx\n"
    "movq %r11, %rdx\n"
    /* Compute location to save old stack pointer (using r11) */
    "mov ia2_stackptr_1@GOTTPOFF(%rip), %r11\n"
    /* Write the old stack pointer to memory */
    "movq %rsp, %fs:(%r11)\n"
    /* Compute location to load new stack pointer (using r11) */
    "mov ia2_stackptr_2@GOTTPOFF(%rip), %r11\n"
    /* Read the new stack pointer from memory */
    "movq %fs:(%r11), %rsp\n"
    "subq $8, %rsp\n"
    /* Preserve essential regs on stack */
    "pushq %rdi\n"
    "pushq %rsi\n"
    "pushq %rdx\n"
    "pushq %rcx\n"
    "pushq %r8\n"
    /* Scrub non-essential regs */
    "call __libia2_scrub_registers\n"
    /* Restore preserved regs */
    "popq %r8\n"
    "popq %rcx\n"
    "popq %rdx\n"
    "popq %rsi\n"
    "popq %rdi\n"
    /* Set PKRU to the compartment's value */
    "movq %rcx, %r10\n"
    "movq %rdx, %r11\n"
    "xorl %ecx, %ecx\n"
    "xorl %edx, %edx\n"
    "movl $0xffffffcc, %eax\n"
    "wrpkru\n"
    "movq %r10, %rcx\n"
    "movq %r11, %rdx\n"
    /* Call wrapped function */
    "call inflateBack\n"
    /* Set PKRU to the intermediate value to move arguments */
    "movq %rax, %r10\n"
    "movq %rdx, %r11\n"
    "xorl %ecx, %ecx\n"
    "xorl %edx, %edx\n"
    "movl $0xffffffc0, %eax\n"
    "wrpkru\n"
    "movq %r10, %rax\n"
    "movq %r11, %rdx\n"
    /* Free stack space used for stack args */
    "addq $8, %rsp\n"
    /* Compute location to save old stack pointer (using r11) */
    "mov ia2_stackptr_2@GOTTPOFF(%rip), %r11\n"
    /* Write the old stack pointer to memory */
    "movq %rsp, %fs:(%r11)\n"
    /* Compute location to load new stack pointer (using r11) */
    "mov ia2_stackptr_1@GOTTPOFF(%rip), %r11\n"
    /* Read the new stack pointer from memory */
    "movq %fs:(%r11), %rsp\n"
    /* Preserve essential regs on stack */
    "pushq %rax\n"
    /* Scrub non-essential regs */
    "call __libia2_scrub_registers\n"
    /* Restore preserved regs */
    "popq %rax\n"
    /* Set PKRU to the caller's value */
    "movq %rax, %r10\n"
    "movq %rdx, %r11\n"
    "xorl %ecx, %ecx\n"
    "xorl %edx, %edx\n"
    "movl $0xfffffff0, %eax\n"
    "wrpkru\n"
    "movq %r10, %rax\n"
    "movq %r11, %rdx\n"
    "popq %r15\n"
    "popq %r14\n"
    "popq %r13\n"
    "popq %r12\n"
    "popq %rbx\n"
    "popq %rbp\n"
    /* Return to the caller */
    "ret\n"
    ".size __wrap_inflateBack, .-__wrap_inflateBack\n"
    ".previous\n"
);
asm(
    /* Wrapper for inflateBackEnd(int) -> int: */
    ".text\n"
    ".global __wrap_inflateBackEnd\n"
    ".type __wrap_inflateBackEnd, @function\n"
    "__wrap_inflateBackEnd:\n"
    "pushq %rbp\n"
    "movq %rsp, %rbp\n"
    "pushq %rbx\n"
    "pushq %r12\n"
    "pushq %r13\n"
    "pushq %r14\n"
    "pushq %r15\n"
    ASSERT_PKRU(0xfffffffffffffff0) "\n"
    /* Set PKRU to the intermediate value to move arguments */
    "movq %rcx, %r10\n"
    "movq %rdx, %r11\n"
    "xorl %ecx, %ecx\n"
    "xorl %edx, %edx\n"
    "movl $0xffffffc0, %eax\n"
    "wrpkru\n"
    "movq %r10, %rcx\n"
    "movq %r11, %rdx\n"
    /* Compute location to save old stack pointer (using r11) */
    "mov ia2_stackptr_1@GOTTPOFF(%rip), %r11\n"
    /* Write the old stack pointer to memory */
    "movq %rsp, %fs:(%r11)\n"
    /* Compute location to load new stack pointer (using r11) */
    "mov ia2_stackptr_2@GOTTPOFF(%rip), %r11\n"
    /* Read the new stack pointer from memory */
    "movq %fs:(%r11), %rsp\n"
    "subq $8, %rsp\n"
    /* Preserve essential regs on stack */
    "pushq %rdi\n"
    /* Scrub non-essential regs */
    "call __libia2_scrub_registers\n"
    /* Restore preserved regs */
    "popq %rdi\n"
    /* Set PKRU to the compartment's value */
    "movq %rcx, %r10\n"
    "movq %rdx, %r11\n"
    "xorl %ecx, %ecx\n"
    "xorl %edx, %edx\n"
    "movl $0xffffffcc, %eax\n"
    "wrpkru\n"
    "movq %r10, %rcx\n"
    "movq %r11, %rdx\n"
    /* Call wrapped function */
    "call inflateBackEnd\n"
    /* Set PKRU to the intermediate value to move arguments */
    "movq %rax, %r10\n"
    "movq %rdx, %r11\n"
    "xorl %ecx, %ecx\n"
    "xorl %edx, %edx\n"
    "movl $0xffffffc0, %eax\n"
    "wrpkru\n"
    "movq %r10, %rax\n"
    "movq %r11, %rdx\n"
    /* Free stack space used for stack args */
    "addq $8, %rsp\n"
    /* Compute location to save old stack pointer (using r11) */
    "mov ia2_stackptr_2@GOTTPOFF(%rip), %r11\n"
    /* Write the old stack pointer to memory */
    "movq %rsp, %fs:(%r11)\n"
    /* Compute location to load new stack pointer (using r11) */
    "mov ia2_stackptr_1@GOTTPOFF(%rip), %r11\n"
    /* Read the new stack pointer from memory */
    "movq %fs:(%r11), %rsp\n"
    /* Preserve essential regs on stack */
    "pushq %rax\n"
    /* Scrub non-essential regs */
    "call __libia2_scrub_registers\n"
    /* Restore preserved regs */
    "popq %rax\n"
    /* Set PKRU to the caller's value */
    "movq %rax, %r10\n"
    "movq %rdx, %r11\n"
    "xorl %ecx, %ecx\n"
    "xorl %edx, %edx\n"
    "movl $0xfffffff0, %eax\n"
    "wrpkru\n"
    "movq %r10, %rax\n"
    "movq %r11, %rdx\n"
    "popq %r15\n"
    "popq %r14\n"
    "popq %r13\n"
    "popq %r12\n"
    "popq %rbx\n"
    "popq %rbp\n"
    /* Return to the caller */
    "ret\n"
    ".size __wrap_inflateBackEnd, .-__wrap_inflateBackEnd\n"
    ".previous\n"
);
asm(
    /* Wrapper for inflateBackInit_(int, int, int, int, int) -> int: */
    ".text\n"
    ".global __wrap_inflateBackInit_\n"
    ".type __wrap_inflateBackInit_, @function\n"
    "__wrap_inflateBackInit_:\n"
    "pushq %rbp\n"
    "movq %rsp, %rbp\n"
    "pushq %rbx\n"
    "pushq %r12\n"
    "pushq %r13\n"
    "pushq %r14\n"
    "pushq %r15\n"
    ASSERT_PKRU(0xfffffffffffffff0) "\n"
    /* Set PKRU to the intermediate value to move arguments */
    "movq %rcx, %r10\n"
    "movq %rdx, %r11\n"
    "xorl %ecx, %ecx\n"
    "xorl %edx, %edx\n"
    "movl $0xffffffc0, %eax\n"
    "wrpkru\n"
    "movq %r10, %rcx\n"
    "movq %r11, %rdx\n"
    /* Compute location to save old stack pointer (using r11) */
    "mov ia2_stackptr_1@GOTTPOFF(%rip), %r11\n"
    /* Write the old stack pointer to memory */
    "movq %rsp, %fs:(%r11)\n"
    /* Compute location to load new stack pointer (using r11) */
    "mov ia2_stackptr_2@GOTTPOFF(%rip), %r11\n"
    /* Read the new stack pointer from memory */
    "movq %fs:(%r11), %rsp\n"
    "subq $8, %rsp\n"
    /* Preserve essential regs on stack */
    "pushq %rdi\n"
    "pushq %rsi\n"
    "pushq %rdx\n"
    "pushq %rcx\n"
    "pushq %r8\n"
    /* Scrub non-essential regs */
    "call __libia2_scrub_registers\n"
    /* Restore preserved regs */
    "popq %r8\n"
    "popq %rcx\n"
    "popq %rdx\n"
    "popq %rsi\n"
    "popq %rdi\n"
    /* Set PKRU to the compartment's value */
    "movq %rcx, %r10\n"
    "movq %rdx, %r11\n"
    "xorl %ecx, %ecx\n"
    "xorl %edx, %edx\n"
    "movl $0xffffffcc, %eax\n"
    "wrpkru\n"
    "movq %r10, %rcx\n"
    "movq %r11, %rdx\n"
    /* Call wrapped function */
    "call inflateBackInit_\n"
    /* Set PKRU to the intermediate value to move arguments */
    "movq %rax, %r10\n"
    "movq %rdx, %r11\n"
    "xorl %ecx, %ecx\n"
    "xorl %edx, %edx\n"
    "movl $0xffffffc0, %eax\n"
    "wrpkru\n"
    "movq %r10, %rax\n"
    "movq %r11, %rdx\n"
    /* Free stack space used for stack args */
    "addq $8, %rsp\n"
    /* Compute location to save old stack pointer (using r11) */
    "mov ia2_stackptr_2@GOTTPOFF(%rip), %r11\n"
    /* Write the old stack pointer to memory */
    "movq %rsp, %fs:(%r11)\n"
    /* Compute location to load new stack pointer (using r11) */
    "mov ia2_stackptr_1@GOTTPOFF(%rip), %r11\n"
    /* Read the new stack pointer from memory */
    "movq %fs:(%r11), %rsp\n"
    /* Preserve essential regs on stack */
    "pushq %rax\n"
    /* Scrub non-essential regs */
    "call __libia2_scrub_registers\n"
    /* Restore preserved regs */
    "popq %rax\n"
    /* Set PKRU to the caller's value */
    "movq %rax, %r10\n"
    "movq %rdx, %r11\n"
    "xorl %ecx, %ecx\n"
    "xorl %edx, %edx\n"
    "movl $0xfffffff0, %eax\n"
    "wrpkru\n"
    "movq %r10, %rax\n"
    "movq %r11, %rdx\n"
    "popq %r15\n"
    "popq %r14\n"
    "popq %r13\n"
    "popq %r12\n"
    "popq %rbx\n"
    "popq %rbp\n"
    /* Return to the caller */
    "ret\n"
    ".size __wrap_inflateBackInit_, .-__wrap_inflateBackInit_\n"
    ".previous\n"
);
asm(
    /* Wrapper for inflateCodesUsed(int) -> int: */
    ".text\n"
    ".global __wrap_inflateCodesUsed\n"
    ".type __wrap_inflateCodesUsed, @function\n"
    "__wrap_inflateCodesUsed:\n"
    "pushq %rbp\n"
    "movq %rsp, %rbp\n"
    "pushq %rbx\n"
    "pushq %r12\n"
    "pushq %r13\n"
    "pushq %r14\n"
    "pushq %r15\n"
    ASSERT_PKRU(0xfffffffffffffff0) "\n"
    /* Set PKRU to the intermediate value to move arguments */
    "movq %rcx, %r10\n"
    "movq %rdx, %r11\n"
    "xorl %ecx, %ecx\n"
    "xorl %edx, %edx\n"
    "movl $0xffffffc0, %eax\n"
    "wrpkru\n"
    "movq %r10, %rcx\n"
    "movq %r11, %rdx\n"
    /* Compute location to save old stack pointer (using r11) */
    "mov ia2_stackptr_1@GOTTPOFF(%rip), %r11\n"
    /* Write the old stack pointer to memory */
    "movq %rsp, %fs:(%r11)\n"
    /* Compute location to load new stack pointer (using r11) */
    "mov ia2_stackptr_2@GOTTPOFF(%rip), %r11\n"
    /* Read the new stack pointer from memory */
    "movq %fs:(%r11), %rsp\n"
    "subq $8, %rsp\n"
    /* Preserve essential regs on stack */
    "pushq %rdi\n"
    /* Scrub non-essential regs */
    "call __libia2_scrub_registers\n"
    /* Restore preserved regs */
    "popq %rdi\n"
    /* Set PKRU to the compartment's value */
    "movq %rcx, %r10\n"
    "movq %rdx, %r11\n"
    "xorl %ecx, %ecx\n"
    "xorl %edx, %edx\n"
    "movl $0xffffffcc, %eax\n"
    "wrpkru\n"
    "movq %r10, %rcx\n"
    "movq %r11, %rdx\n"
    /* Call wrapped function */
    "call inflateCodesUsed\n"
    /* Set PKRU to the intermediate value to move arguments */
    "movq %rax, %r10\n"
    "movq %rdx, %r11\n"
    "xorl %ecx, %ecx\n"
    "xorl %edx, %edx\n"
    "movl $0xffffffc0, %eax\n"
    "wrpkru\n"
    "movq %r10, %rax\n"
    "movq %r11, %rdx\n"
    /* Free stack space used for stack args */
    "addq $8, %rsp\n"
    /* Compute location to save old stack pointer (using r11) */
    "mov ia2_stackptr_2@GOTTPOFF(%rip), %r11\n"
    /* Write the old stack pointer to memory */
    "movq %rsp, %fs:(%r11)\n"
    /* Compute location to load new stack pointer (using r11) */
    "mov ia2_stackptr_1@GOTTPOFF(%rip), %r11\n"
    /* Read the new stack pointer from memory */
    "movq %fs:(%r11), %rsp\n"
    /* Preserve essential regs on stack */
    "pushq %rax\n"
    /* Scrub non-essential regs */
    "call __libia2_scrub_registers\n"
    /* Restore preserved regs */
    "popq %rax\n"
    /* Set PKRU to the caller's value */
    "movq %rax, %r10\n"
    "movq %rdx, %r11\n"
    "xorl %ecx, %ecx\n"
    "xorl %edx, %edx\n"
    "movl $0xfffffff0, %eax\n"
    "wrpkru\n"
    "movq %r10, %rax\n"
    "movq %r11, %rdx\n"
    "popq %r15\n"
    "popq %r14\n"
    "popq %r13\n"
    "popq %r12\n"
    "popq %rbx\n"
    "popq %rbp\n"
    /* Return to the caller */
    "ret\n"
    ".size __wrap_inflateCodesUsed, .-__wrap_inflateCodesUsed\n"
    ".previous\n"
);
asm(
    /* Wrapper for inflateCopy(int, int) -> int: */
    ".text\n"
    ".global __wrap_inflateCopy\n"
    ".type __wrap_inflateCopy, @function\n"
    "__wrap_inflateCopy:\n"
    "pushq %rbp\n"
    "movq %rsp, %rbp\n"
    "pushq %rbx\n"
    "pushq %r12\n"
    "pushq %r13\n"
    "pushq %r14\n"
    "pushq %r15\n"
    ASSERT_PKRU(0xfffffffffffffff0) "\n"
    /* Set PKRU to the intermediate value to move arguments */
    "movq %rcx, %r10\n"
    "movq %rdx, %r11\n"
    "xorl %ecx, %ecx\n"
    "xorl %edx, %edx\n"
    "movl $0xffffffc0, %eax\n"
    "wrpkru\n"
    "movq %r10, %rcx\n"
    "movq %r11, %rdx\n"
    /* Compute location to save old stack pointer (using r11) */
    "mov ia2_stackptr_1@GOTTPOFF(%rip), %r11\n"
    /* Write the old stack pointer to memory */
    "movq %rsp, %fs:(%r11)\n"
    /* Compute location to load new stack pointer (using r11) */
    "mov ia2_stackptr_2@GOTTPOFF(%rip), %r11\n"
    /* Read the new stack pointer from memory */
    "movq %fs:(%r11), %rsp\n"
    "subq $8, %rsp\n"
    /* Preserve essential regs on stack */
    "pushq %rdi\n"
    "pushq %rsi\n"
    /* Scrub non-essential regs */
    "call __libia2_scrub_registers\n"
    /* Restore preserved regs */
    "popq %rsi\n"
    "popq %rdi\n"
    /* Set PKRU to the compartment's value */
    "movq %rcx, %r10\n"
    "movq %rdx, %r11\n"
    "xorl %ecx, %ecx\n"
    "xorl %edx, %edx\n"
    "movl $0xffffffcc, %eax\n"
    "wrpkru\n"
    "movq %r10, %rcx\n"
    "movq %r11, %rdx\n"
    /* Call wrapped function */
    "call inflateCopy\n"
    /* Set PKRU to the intermediate value to move arguments */
    "movq %rax, %r10\n"
    "movq %rdx, %r11\n"
    "xorl %ecx, %ecx\n"
    "xorl %edx, %edx\n"
    "movl $0xffffffc0, %eax\n"
    "wrpkru\n"
    "movq %r10, %rax\n"
    "movq %r11, %rdx\n"
    /* Free stack space used for stack args */
    "addq $8, %rsp\n"
    /* Compute location to save old stack pointer (using r11) */
    "mov ia2_stackptr_2@GOTTPOFF(%rip), %r11\n"
    /* Write the old stack pointer to memory */
    "movq %rsp, %fs:(%r11)\n"
    /* Compute location to load new stack pointer (using r11) */
    "mov ia2_stackptr_1@GOTTPOFF(%rip), %r11\n"
    /* Read the new stack pointer from memory */
    "movq %fs:(%r11), %rsp\n"
    /* Preserve essential regs on stack */
    "pushq %rax\n"
    /* Scrub non-essential regs */
    "call __libia2_scrub_registers\n"
    /* Restore preserved regs */
    "popq %rax\n"
    /* Set PKRU to the caller's value */
    "movq %rax, %r10\n"
    "movq %rdx, %r11\n"
    "xorl %ecx, %ecx\n"
    "xorl %edx, %edx\n"
    "movl $0xfffffff0, %eax\n"
    "wrpkru\n"
    "movq %r10, %rax\n"
    "movq %r11, %rdx\n"
    "popq %r15\n"
    "popq %r14\n"
    "popq %r13\n"
    "popq %r12\n"
    "popq %rbx\n"
    "popq %rbp\n"
    /* Return to the caller */
    "ret\n"
    ".size __wrap_inflateCopy, .-__wrap_inflateCopy\n"
    ".previous\n"
);
asm(
    /* Wrapper for inflateEnd(int) -> int: */
    ".text\n"
    ".global __wrap_inflateEnd\n"
    ".type __wrap_inflateEnd, @function\n"
    "__wrap_inflateEnd:\n"
    "pushq %rbp\n"
    "movq %rsp, %rbp\n"
    "pushq %rbx\n"
    "pushq %r12\n"
    "pushq %r13\n"
    "pushq %r14\n"
    "pushq %r15\n"
    ASSERT_PKRU(0xfffffffffffffff0) "\n"
    /* Set PKRU to the intermediate value to move arguments */
    "movq %rcx, %r10\n"
    "movq %rdx, %r11\n"
    "xorl %ecx, %ecx\n"
    "xorl %edx, %edx\n"
    "movl $0xffffffc0, %eax\n"
    "wrpkru\n"
    "movq %r10, %rcx\n"
    "movq %r11, %rdx\n"
    /* Compute location to save old stack pointer (using r11) */
    "mov ia2_stackptr_1@GOTTPOFF(%rip), %r11\n"
    /* Write the old stack pointer to memory */
    "movq %rsp, %fs:(%r11)\n"
    /* Compute location to load new stack pointer (using r11) */
    "mov ia2_stackptr_2@GOTTPOFF(%rip), %r11\n"
    /* Read the new stack pointer from memory */
    "movq %fs:(%r11), %rsp\n"
    "subq $8, %rsp\n"
    /* Preserve essential regs on stack */
    "pushq %rdi\n"
    /* Scrub non-essential regs */
    "call __libia2_scrub_registers\n"
    /* Restore preserved regs */
    "popq %rdi\n"
    /* Set PKRU to the compartment's value */
    "movq %rcx, %r10\n"
    "movq %rdx, %r11\n"
    "xorl %ecx, %ecx\n"
    "xorl %edx, %edx\n"
    "movl $0xffffffcc, %eax\n"
    "wrpkru\n"
    "movq %r10, %rcx\n"
    "movq %r11, %rdx\n"
    /* Call wrapped function */
    "call inflateEnd\n"
    /* Set PKRU to the intermediate value to move arguments */
    "movq %rax, %r10\n"
    "movq %rdx, %r11\n"
    "xorl %ecx, %ecx\n"
    "xorl %edx, %edx\n"
    "movl $0xffffffc0, %eax\n"
    "wrpkru\n"
    "movq %r10, %rax\n"
    "movq %r11, %rdx\n"
    /* Free stack space used for stack args */
    "addq $8, %rsp\n"
    /* Compute location to save old stack pointer (using r11) */
    "mov ia2_stackptr_2@GOTTPOFF(%rip), %r11\n"
    /* Write the old stack pointer to memory */
    "movq %rsp, %fs:(%r11)\n"
    /* Compute location to load new stack pointer (using r11) */
    "mov ia2_stackptr_1@GOTTPOFF(%rip), %r11\n"
    /* Read the new stack pointer from memory */
    "movq %fs:(%r11), %rsp\n"
    /* Preserve essential regs on stack */
    "pushq %rax\n"
    /* Scrub non-essential regs */
    "call __libia2_scrub_registers\n"
    /* Restore preserved regs */
    "popq %rax\n"
    /* Set PKRU to the caller's value */
    "movq %rax, %r10\n"
    "movq %rdx, %r11\n"
    "xorl %ecx, %ecx\n"
    "xorl %edx, %edx\n"
    "movl $0xfffffff0, %eax\n"
    "wrpkru\n"
    "movq %r10, %rax\n"
    "movq %r11, %rdx\n"
    "popq %r15\n"
    "popq %r14\n"
    "popq %r13\n"
    "popq %r12\n"
    "popq %rbx\n"
    "popq %rbp\n"
    /* Return to the caller */
    "ret\n"
    ".size __wrap_inflateEnd, .-__wrap_inflateEnd\n"
    ".previous\n"
);
asm(
    /* Wrapper for inflateGetDictionary(int, int, int) -> int: */
    ".text\n"
    ".global __wrap_inflateGetDictionary\n"
    ".type __wrap_inflateGetDictionary, @function\n"
    "__wrap_inflateGetDictionary:\n"
    "pushq %rbp\n"
    "movq %rsp, %rbp\n"
    "pushq %rbx\n"
    "pushq %r12\n"
    "pushq %r13\n"
    "pushq %r14\n"
    "pushq %r15\n"
    ASSERT_PKRU(0xfffffffffffffff0) "\n"
    /* Set PKRU to the intermediate value to move arguments */
    "movq %rcx, %r10\n"
    "movq %rdx, %r11\n"
    "xorl %ecx, %ecx\n"
    "xorl %edx, %edx\n"
    "movl $0xffffffc0, %eax\n"
    "wrpkru\n"
    "movq %r10, %rcx\n"
    "movq %r11, %rdx\n"
    /* Compute location to save old stack pointer (using r11) */
    "mov ia2_stackptr_1@GOTTPOFF(%rip), %r11\n"
    /* Write the old stack pointer to memory */
    "movq %rsp, %fs:(%r11)\n"
    /* Compute location to load new stack pointer (using r11) */
    "mov ia2_stackptr_2@GOTTPOFF(%rip), %r11\n"
    /* Read the new stack pointer from memory */
    "movq %fs:(%r11), %rsp\n"
    "subq $8, %rsp\n"
    /* Preserve essential regs on stack */
    "pushq %rdi\n"
    "pushq %rsi\n"
    "pushq %rdx\n"
    /* Scrub non-essential regs */
    "call __libia2_scrub_registers\n"
    /* Restore preserved regs */
    "popq %rdx\n"
    "popq %rsi\n"
    "popq %rdi\n"
    /* Set PKRU to the compartment's value */
    "movq %rcx, %r10\n"
    "movq %rdx, %r11\n"
    "xorl %ecx, %ecx\n"
    "xorl %edx, %edx\n"
    "movl $0xffffffcc, %eax\n"
    "wrpkru\n"
    "movq %r10, %rcx\n"
    "movq %r11, %rdx\n"
    /* Call wrapped function */
    "call inflateGetDictionary\n"
    /* Set PKRU to the intermediate value to move arguments */
    "movq %rax, %r10\n"
    "movq %rdx, %r11\n"
    "xorl %ecx, %ecx\n"
    "xorl %edx, %edx\n"
    "movl $0xffffffc0, %eax\n"
    "wrpkru\n"
    "movq %r10, %rax\n"
    "movq %r11, %rdx\n"
    /* Free stack space used for stack args */
    "addq $8, %rsp\n"
    /* Compute location to save old stack pointer (using r11) */
    "mov ia2_stackptr_2@GOTTPOFF(%rip), %r11\n"
    /* Write the old stack pointer to memory */
    "movq %rsp, %fs:(%r11)\n"
    /* Compute location to load new stack pointer (using r11) */
    "mov ia2_stackptr_1@GOTTPOFF(%rip), %r11\n"
    /* Read the new stack pointer from memory */
    "movq %fs:(%r11), %rsp\n"
    /* Preserve essential regs on stack */
    "pushq %rax\n"
    /* Scrub non-essential regs */
    "call __libia2_scrub_registers\n"
    /* Restore preserved regs */
    "popq %rax\n"
    /* Set PKRU to the caller's value */
    "movq %rax, %r10\n"
    "movq %rdx, %r11\n"
    "xorl %ecx, %ecx\n"
    "xorl %edx, %edx\n"
    "movl $0xfffffff0, %eax\n"
    "wrpkru\n"
    "movq %r10, %rax\n"
    "movq %r11, %rdx\n"
    "popq %r15\n"
    "popq %r14\n"
    "popq %r13\n"
    "popq %r12\n"
    "popq %rbx\n"
    "popq %rbp\n"
    /* Return to the caller */
    "ret\n"
    ".size __wrap_inflateGetDictionary, .-__wrap_inflateGetDictionary\n"
    ".previous\n"
);
asm(
    /* Wrapper for inflateGetHeader(int, int) -> int: */
    ".text\n"
    ".global __wrap_inflateGetHeader\n"
    ".type __wrap_inflateGetHeader, @function\n"
    "__wrap_inflateGetHeader:\n"
    "pushq %rbp\n"
    "movq %rsp, %rbp\n"
    "pushq %rbx\n"
    "pushq %r12\n"
    "pushq %r13\n"
    "pushq %r14\n"
    "pushq %r15\n"
    ASSERT_PKRU(0xfffffffffffffff0) "\n"
    /* Set PKRU to the intermediate value to move arguments */
    "movq %rcx, %r10\n"
    "movq %rdx, %r11\n"
    "xorl %ecx, %ecx\n"
    "xorl %edx, %edx\n"
    "movl $0xffffffc0, %eax\n"
    "wrpkru\n"
    "movq %r10, %rcx\n"
    "movq %r11, %rdx\n"
    /* Compute location to save old stack pointer (using r11) */
    "mov ia2_stackptr_1@GOTTPOFF(%rip), %r11\n"
    /* Write the old stack pointer to memory */
    "movq %rsp, %fs:(%r11)\n"
    /* Compute location to load new stack pointer (using r11) */
    "mov ia2_stackptr_2@GOTTPOFF(%rip), %r11\n"
    /* Read the new stack pointer from memory */
    "movq %fs:(%r11), %rsp\n"
    "subq $8, %rsp\n"
    /* Preserve essential regs on stack */
    "pushq %rdi\n"
    "pushq %rsi\n"
    /* Scrub non-essential regs */
    "call __libia2_scrub_registers\n"
    /* Restore preserved regs */
    "popq %rsi\n"
    "popq %rdi\n"
    /* Set PKRU to the compartment's value */
    "movq %rcx, %r10\n"
    "movq %rdx, %r11\n"
    "xorl %ecx, %ecx\n"
    "xorl %edx, %edx\n"
    "movl $0xffffffcc, %eax\n"
    "wrpkru\n"
    "movq %r10, %rcx\n"
    "movq %r11, %rdx\n"
    /* Call wrapped function */
    "call inflateGetHeader\n"
    /* Set PKRU to the intermediate value to move arguments */
    "movq %rax, %r10\n"
    "movq %rdx, %r11\n"
    "xorl %ecx, %ecx\n"
    "xorl %edx, %edx\n"
    "movl $0xffffffc0, %eax\n"
    "wrpkru\n"
    "movq %r10, %rax\n"
    "movq %r11, %rdx\n"
    /* Free stack space used for stack args */
    "addq $8, %rsp\n"
    /* Compute location to save old stack pointer (using r11) */
    "mov ia2_stackptr_2@GOTTPOFF(%rip), %r11\n"
    /* Write the old stack pointer to memory */
    "movq %rsp, %fs:(%r11)\n"
    /* Compute location to load new stack pointer (using r11) */
    "mov ia2_stackptr_1@GOTTPOFF(%rip), %r11\n"
    /* Read the new stack pointer from memory */
    "movq %fs:(%r11), %rsp\n"
    /* Preserve essential regs on stack */
    "pushq %rax\n"
    /* Scrub non-essential regs */
    "call __libia2_scrub_registers\n"
    /* Restore preserved regs */
    "popq %rax\n"
    /* Set PKRU to the caller's value */
    "movq %rax, %r10\n"
    "movq %rdx, %r11\n"
    "xorl %ecx, %ecx\n"
    "xorl %edx, %edx\n"
    "movl $0xfffffff0, %eax\n"
    "wrpkru\n"
    "movq %r10, %rax\n"
    "movq %r11, %rdx\n"
    "popq %r15\n"
    "popq %r14\n"
    "popq %r13\n"
    "popq %r12\n"
    "popq %rbx\n"
    "popq %rbp\n"
    /* Return to the caller */
    "ret\n"
    ".size __wrap_inflateGetHeader, .-__wrap_inflateGetHeader\n"
    ".previous\n"
);
asm(
    /* Wrapper for inflateInit2_(int, int, int, int) -> int: */
    ".text\n"
    ".global __wrap_inflateInit2_\n"
    ".type __wrap_inflateInit2_, @function\n"
    "__wrap_inflateInit2_:\n"
    "pushq %rbp\n"
    "movq %rsp, %rbp\n"
    "pushq %rbx\n"
    "pushq %r12\n"
    "pushq %r13\n"
    "pushq %r14\n"
    "pushq %r15\n"
    ASSERT_PKRU(0xfffffffffffffff0) "\n"
    /* Set PKRU to the intermediate value to move arguments */
    "movq %rcx, %r10\n"
    "movq %rdx, %r11\n"
    "xorl %ecx, %ecx\n"
    "xorl %edx, %edx\n"
    "movl $0xffffffc0, %eax\n"
    "wrpkru\n"
    "movq %r10, %rcx\n"
    "movq %r11, %rdx\n"
    /* Compute location to save old stack pointer (using r11) */
    "mov ia2_stackptr_1@GOTTPOFF(%rip), %r11\n"
    /* Write the old stack pointer to memory */
    "movq %rsp, %fs:(%r11)\n"
    /* Compute location to load new stack pointer (using r11) */
    "mov ia2_stackptr_2@GOTTPOFF(%rip), %r11\n"
    /* Read the new stack pointer from memory */
    "movq %fs:(%r11), %rsp\n"
    "subq $8, %rsp\n"
    /* Preserve essential regs on stack */
    "pushq %rdi\n"
    "pushq %rsi\n"
    "pushq %rdx\n"
    "pushq %rcx\n"
    /* Scrub non-essential regs */
    "call __libia2_scrub_registers\n"
    /* Restore preserved regs */
    "popq %rcx\n"
    "popq %rdx\n"
    "popq %rsi\n"
    "popq %rdi\n"
    /* Set PKRU to the compartment's value */
    "movq %rcx, %r10\n"
    "movq %rdx, %r11\n"
    "xorl %ecx, %ecx\n"
    "xorl %edx, %edx\n"
    "movl $0xffffffcc, %eax\n"
    "wrpkru\n"
    "movq %r10, %rcx\n"
    "movq %r11, %rdx\n"
    /* Call wrapped function */
    "call inflateInit2_\n"
    /* Set PKRU to the intermediate value to move arguments */
    "movq %rax, %r10\n"
    "movq %rdx, %r11\n"
    "xorl %ecx, %ecx\n"
    "xorl %edx, %edx\n"
    "movl $0xffffffc0, %eax\n"
    "wrpkru\n"
    "movq %r10, %rax\n"
    "movq %r11, %rdx\n"
    /* Free stack space used for stack args */
    "addq $8, %rsp\n"
    /* Compute location to save old stack pointer (using r11) */
    "mov ia2_stackptr_2@GOTTPOFF(%rip), %r11\n"
    /* Write the old stack pointer to memory */
    "movq %rsp, %fs:(%r11)\n"
    /* Compute location to load new stack pointer (using r11) */
    "mov ia2_stackptr_1@GOTTPOFF(%rip), %r11\n"
    /* Read the new stack pointer from memory */
    "movq %fs:(%r11), %rsp\n"
    /* Preserve essential regs on stack */
    "pushq %rax\n"
    /* Scrub non-essential regs */
    "call __libia2_scrub_registers\n"
    /* Restore preserved regs */
    "popq %rax\n"
    /* Set PKRU to the caller's value */
    "movq %rax, %r10\n"
    "movq %rdx, %r11\n"
    "xorl %ecx, %ecx\n"
    "xorl %edx, %edx\n"
    "movl $0xfffffff0, %eax\n"
    "wrpkru\n"
    "movq %r10, %rax\n"
    "movq %r11, %rdx\n"
    "popq %r15\n"
    "popq %r14\n"
    "popq %r13\n"
    "popq %r12\n"
    "popq %rbx\n"
    "popq %rbp\n"
    /* Return to the caller */
    "ret\n"
    ".size __wrap_inflateInit2_, .-__wrap_inflateInit2_\n"
    ".previous\n"
);
asm(
    /* Wrapper for inflateInit_(int, int, int) -> int: */
    ".text\n"
    ".global __wrap_inflateInit_\n"
    ".type __wrap_inflateInit_, @function\n"
    "__wrap_inflateInit_:\n"
    "pushq %rbp\n"
    "movq %rsp, %rbp\n"
    "pushq %rbx\n"
    "pushq %r12\n"
    "pushq %r13\n"
    "pushq %r14\n"
    "pushq %r15\n"
    ASSERT_PKRU(0xfffffffffffffff0) "\n"
    /* Set PKRU to the intermediate value to move arguments */
    "movq %rcx, %r10\n"
    "movq %rdx, %r11\n"
    "xorl %ecx, %ecx\n"
    "xorl %edx, %edx\n"
    "movl $0xffffffc0, %eax\n"
    "wrpkru\n"
    "movq %r10, %rcx\n"
    "movq %r11, %rdx\n"
    /* Compute location to save old stack pointer (using r11) */
    "mov ia2_stackptr_1@GOTTPOFF(%rip), %r11\n"
    /* Write the old stack pointer to memory */
    "movq %rsp, %fs:(%r11)\n"
    /* Compute location to load new stack pointer (using r11) */
    "mov ia2_stackptr_2@GOTTPOFF(%rip), %r11\n"
    /* Read the new stack pointer from memory */
    "movq %fs:(%r11), %rsp\n"
    "subq $8, %rsp\n"
    /* Preserve essential regs on stack */
    "pushq %rdi\n"
    "pushq %rsi\n"
    "pushq %rdx\n"
    /* Scrub non-essential regs */
    "call __libia2_scrub_registers\n"
    /* Restore preserved regs */
    "popq %rdx\n"
    "popq %rsi\n"
    "popq %rdi\n"
    /* Set PKRU to the compartment's value */
    "movq %rcx, %r10\n"
    "movq %rdx, %r11\n"
    "xorl %ecx, %ecx\n"
    "xorl %edx, %edx\n"
    "movl $0xffffffcc, %eax\n"
    "wrpkru\n"
    "movq %r10, %rcx\n"
    "movq %r11, %rdx\n"
    /* Call wrapped function */
    "call inflateInit_\n"
    /* Set PKRU to the intermediate value to move arguments */
    "movq %rax, %r10\n"
    "movq %rdx, %r11\n"
    "xorl %ecx, %ecx\n"
    "xorl %edx, %edx\n"
    "movl $0xffffffc0, %eax\n"
    "wrpkru\n"
    "movq %r10, %rax\n"
    "movq %r11, %rdx\n"
    /* Free stack space used for stack args */
    "addq $8, %rsp\n"
    /* Compute location to save old stack pointer (using r11) */
    "mov ia2_stackptr_2@GOTTPOFF(%rip), %r11\n"
    /* Write the old stack pointer to memory */
    "movq %rsp, %fs:(%r11)\n"
    /* Compute location to load new stack pointer (using r11) */
    "mov ia2_stackptr_1@GOTTPOFF(%rip), %r11\n"
    /* Read the new stack pointer from memory */
    "movq %fs:(%r11), %rsp\n"
    /* Preserve essential regs on stack */
    "pushq %rax\n"
    /* Scrub non-essential regs */
    "call __libia2_scrub_registers\n"
    /* Restore preserved regs */
    "popq %rax\n"
    /* Set PKRU to the caller's value */
    "movq %rax, %r10\n"
    "movq %rdx, %r11\n"
    "xorl %ecx, %ecx\n"
    "xorl %edx, %edx\n"
    "movl $0xfffffff0, %eax\n"
    "wrpkru\n"
    "movq %r10, %rax\n"
    "movq %r11, %rdx\n"
    "popq %r15\n"
    "popq %r14\n"
    "popq %r13\n"
    "popq %r12\n"
    "popq %rbx\n"
    "popq %rbp\n"
    /* Return to the caller */
    "ret\n"
    ".size __wrap_inflateInit_, .-__wrap_inflateInit_\n"
    ".previous\n"
);
asm(
    /* Wrapper for inflateMark(int) -> int: */
    ".text\n"
    ".global __wrap_inflateMark\n"
    ".type __wrap_inflateMark, @function\n"
    "__wrap_inflateMark:\n"
    "pushq %rbp\n"
    "movq %rsp, %rbp\n"
    "pushq %rbx\n"
    "pushq %r12\n"
    "pushq %r13\n"
    "pushq %r14\n"
    "pushq %r15\n"
    ASSERT_PKRU(0xfffffffffffffff0) "\n"
    /* Set PKRU to the intermediate value to move arguments */
    "movq %rcx, %r10\n"
    "movq %rdx, %r11\n"
    "xorl %ecx, %ecx\n"
    "xorl %edx, %edx\n"
    "movl $0xffffffc0, %eax\n"
    "wrpkru\n"
    "movq %r10, %rcx\n"
    "movq %r11, %rdx\n"
    /* Compute location to save old stack pointer (using r11) */
    "mov ia2_stackptr_1@GOTTPOFF(%rip), %r11\n"
    /* Write the old stack pointer to memory */
    "movq %rsp, %fs:(%r11)\n"
    /* Compute location to load new stack pointer (using r11) */
    "mov ia2_stackptr_2@GOTTPOFF(%rip), %r11\n"
    /* Read the new stack pointer from memory */
    "movq %fs:(%r11), %rsp\n"
    "subq $8, %rsp\n"
    /* Preserve essential regs on stack */
    "pushq %rdi\n"
    /* Scrub non-essential regs */
    "call __libia2_scrub_registers\n"
    /* Restore preserved regs */
    "popq %rdi\n"
    /* Set PKRU to the compartment's value */
    "movq %rcx, %r10\n"
    "movq %rdx, %r11\n"
    "xorl %ecx, %ecx\n"
    "xorl %edx, %edx\n"
    "movl $0xffffffcc, %eax\n"
    "wrpkru\n"
    "movq %r10, %rcx\n"
    "movq %r11, %rdx\n"
    /* Call wrapped function */
    "call inflateMark\n"
    /* Set PKRU to the intermediate value to move arguments */
    "movq %rax, %r10\n"
    "movq %rdx, %r11\n"
    "xorl %ecx, %ecx\n"
    "xorl %edx, %edx\n"
    "movl $0xffffffc0, %eax\n"
    "wrpkru\n"
    "movq %r10, %rax\n"
    "movq %r11, %rdx\n"
    /* Free stack space used for stack args */
    "addq $8, %rsp\n"
    /* Compute location to save old stack pointer (using r11) */
    "mov ia2_stackptr_2@GOTTPOFF(%rip), %r11\n"
    /* Write the old stack pointer to memory */
    "movq %rsp, %fs:(%r11)\n"
    /* Compute location to load new stack pointer (using r11) */
    "mov ia2_stackptr_1@GOTTPOFF(%rip), %r11\n"
    /* Read the new stack pointer from memory */
    "movq %fs:(%r11), %rsp\n"
    /* Preserve essential regs on stack */
    "pushq %rax\n"
    /* Scrub non-essential regs */
    "call __libia2_scrub_registers\n"
    /* Restore preserved regs */
    "popq %rax\n"
    /* Set PKRU to the caller's value */
    "movq %rax, %r10\n"
    "movq %rdx, %r11\n"
    "xorl %ecx, %ecx\n"
    "xorl %edx, %edx\n"
    "movl $0xfffffff0, %eax\n"
    "wrpkru\n"
    "movq %r10, %rax\n"
    "movq %r11, %rdx\n"
    "popq %r15\n"
    "popq %r14\n"
    "popq %r13\n"
    "popq %r12\n"
    "popq %rbx\n"
    "popq %rbp\n"
    /* Return to the caller */
    "ret\n"
    ".size __wrap_inflateMark, .-__wrap_inflateMark\n"
    ".previous\n"
);
asm(
    /* Wrapper for inflatePrime(int, int, int) -> int: */
    ".text\n"
    ".global __wrap_inflatePrime\n"
    ".type __wrap_inflatePrime, @function\n"
    "__wrap_inflatePrime:\n"
    "pushq %rbp\n"
    "movq %rsp, %rbp\n"
    "pushq %rbx\n"
    "pushq %r12\n"
    "pushq %r13\n"
    "pushq %r14\n"
    "pushq %r15\n"
    ASSERT_PKRU(0xfffffffffffffff0) "\n"
    /* Set PKRU to the intermediate value to move arguments */
    "movq %rcx, %r10\n"
    "movq %rdx, %r11\n"
    "xorl %ecx, %ecx\n"
    "xorl %edx, %edx\n"
    "movl $0xffffffc0, %eax\n"
    "wrpkru\n"
    "movq %r10, %rcx\n"
    "movq %r11, %rdx\n"
    /* Compute location to save old stack pointer (using r11) */
    "mov ia2_stackptr_1@GOTTPOFF(%rip), %r11\n"
    /* Write the old stack pointer to memory */
    "movq %rsp, %fs:(%r11)\n"
    /* Compute location to load new stack pointer (using r11) */
    "mov ia2_stackptr_2@GOTTPOFF(%rip), %r11\n"
    /* Read the new stack pointer from memory */
    "movq %fs:(%r11), %rsp\n"
    "subq $8, %rsp\n"
    /* Preserve essential regs on stack */
    "pushq %rdi\n"
    "pushq %rsi\n"
    "pushq %rdx\n"
    /* Scrub non-essential regs */
    "call __libia2_scrub_registers\n"
    /* Restore preserved regs */
    "popq %rdx\n"
    "popq %rsi\n"
    "popq %rdi\n"
    /* Set PKRU to the compartment's value */
    "movq %rcx, %r10\n"
    "movq %rdx, %r11\n"
    "xorl %ecx, %ecx\n"
    "xorl %edx, %edx\n"
    "movl $0xffffffcc, %eax\n"
    "wrpkru\n"
    "movq %r10, %rcx\n"
    "movq %r11, %rdx\n"
    /* Call wrapped function */
    "call inflatePrime\n"
    /* Set PKRU to the intermediate value to move arguments */
    "movq %rax, %r10\n"
    "movq %rdx, %r11\n"
    "xorl %ecx, %ecx\n"
    "xorl %edx, %edx\n"
    "movl $0xffffffc0, %eax\n"
    "wrpkru\n"
    "movq %r10, %rax\n"
    "movq %r11, %rdx\n"
    /* Free stack space used for stack args */
    "addq $8, %rsp\n"
    /* Compute location to save old stack pointer (using r11) */
    "mov ia2_stackptr_2@GOTTPOFF(%rip), %r11\n"
    /* Write the old stack pointer to memory */
    "movq %rsp, %fs:(%r11)\n"
    /* Compute location to load new stack pointer (using r11) */
    "mov ia2_stackptr_1@GOTTPOFF(%rip), %r11\n"
    /* Read the new stack pointer from memory */
    "movq %fs:(%r11), %rsp\n"
    /* Preserve essential regs on stack */
    "pushq %rax\n"
    /* Scrub non-essential regs */
    "call __libia2_scrub_registers\n"
    /* Restore preserved regs */
    "popq %rax\n"
    /* Set PKRU to the caller's value */
    "movq %rax, %r10\n"
    "movq %rdx, %r11\n"
    "xorl %ecx, %ecx\n"
    "xorl %edx, %edx\n"
    "movl $0xfffffff0, %eax\n"
    "wrpkru\n"
    "movq %r10, %rax\n"
    "movq %r11, %rdx\n"
    "popq %r15\n"
    "popq %r14\n"
    "popq %r13\n"
    "popq %r12\n"
    "popq %rbx\n"
    "popq %rbp\n"
    /* Return to the caller */
    "ret\n"
    ".size __wrap_inflatePrime, .-__wrap_inflatePrime\n"
    ".previous\n"
);
asm(
    /* Wrapper for inflateReset(int) -> int: */
    ".text\n"
    ".global __wrap_inflateReset\n"
    ".type __wrap_inflateReset, @function\n"
    "__wrap_inflateReset:\n"
    "pushq %rbp\n"
    "movq %rsp, %rbp\n"
    "pushq %rbx\n"
    "pushq %r12\n"
    "pushq %r13\n"
    "pushq %r14\n"
    "pushq %r15\n"
    ASSERT_PKRU(0xfffffffffffffff0) "\n"
    /* Set PKRU to the intermediate value to move arguments */
    "movq %rcx, %r10\n"
    "movq %rdx, %r11\n"
    "xorl %ecx, %ecx\n"
    "xorl %edx, %edx\n"
    "movl $0xffffffc0, %eax\n"
    "wrpkru\n"
    "movq %r10, %rcx\n"
    "movq %r11, %rdx\n"
    /* Compute location to save old stack pointer (using r11) */
    "mov ia2_stackptr_1@GOTTPOFF(%rip), %r11\n"
    /* Write the old stack pointer to memory */
    "movq %rsp, %fs:(%r11)\n"
    /* Compute location to load new stack pointer (using r11) */
    "mov ia2_stackptr_2@GOTTPOFF(%rip), %r11\n"
    /* Read the new stack pointer from memory */
    "movq %fs:(%r11), %rsp\n"
    "subq $8, %rsp\n"
    /* Preserve essential regs on stack */
    "pushq %rdi\n"
    /* Scrub non-essential regs */
    "call __libia2_scrub_registers\n"
    /* Restore preserved regs */
    "popq %rdi\n"
    /* Set PKRU to the compartment's value */
    "movq %rcx, %r10\n"
    "movq %rdx, %r11\n"
    "xorl %ecx, %ecx\n"
    "xorl %edx, %edx\n"
    "movl $0xffffffcc, %eax\n"
    "wrpkru\n"
    "movq %r10, %rcx\n"
    "movq %r11, %rdx\n"
    /* Call wrapped function */
    "call inflateReset\n"
    /* Set PKRU to the intermediate value to move arguments */
    "movq %rax, %r10\n"
    "movq %rdx, %r11\n"
    "xorl %ecx, %ecx\n"
    "xorl %edx, %edx\n"
    "movl $0xffffffc0, %eax\n"
    "wrpkru\n"
    "movq %r10, %rax\n"
    "movq %r11, %rdx\n"
    /* Free stack space used for stack args */
    "addq $8, %rsp\n"
    /* Compute location to save old stack pointer (using r11) */
    "mov ia2_stackptr_2@GOTTPOFF(%rip), %r11\n"
    /* Write the old stack pointer to memory */
    "movq %rsp, %fs:(%r11)\n"
    /* Compute location to load new stack pointer (using r11) */
    "mov ia2_stackptr_1@GOTTPOFF(%rip), %r11\n"
    /* Read the new stack pointer from memory */
    "movq %fs:(%r11), %rsp\n"
    /* Preserve essential regs on stack */
    "pushq %rax\n"
    /* Scrub non-essential regs */
    "call __libia2_scrub_registers\n"
    /* Restore preserved regs */
    "popq %rax\n"
    /* Set PKRU to the caller's value */
    "movq %rax, %r10\n"
    "movq %rdx, %r11\n"
    "xorl %ecx, %ecx\n"
    "xorl %edx, %edx\n"
    "movl $0xfffffff0, %eax\n"
    "wrpkru\n"
    "movq %r10, %rax\n"
    "movq %r11, %rdx\n"
    "popq %r15\n"
    "popq %r14\n"
    "popq %r13\n"
    "popq %r12\n"
    "popq %rbx\n"
    "popq %rbp\n"
    /* Return to the caller */
    "ret\n"
    ".size __wrap_inflateReset, .-__wrap_inflateReset\n"
    ".previous\n"
);
asm(
    /* Wrapper for inflateReset2(int, int) -> int: */
    ".text\n"
    ".global __wrap_inflateReset2\n"
    ".type __wrap_inflateReset2, @function\n"
    "__wrap_inflateReset2:\n"
    "pushq %rbp\n"
    "movq %rsp, %rbp\n"
    "pushq %rbx\n"
    "pushq %r12\n"
    "pushq %r13\n"
    "pushq %r14\n"
    "pushq %r15\n"
    ASSERT_PKRU(0xfffffffffffffff0) "\n"
    /* Set PKRU to the intermediate value to move arguments */
    "movq %rcx, %r10\n"
    "movq %rdx, %r11\n"
    "xorl %ecx, %ecx\n"
    "xorl %edx, %edx\n"
    "movl $0xffffffc0, %eax\n"
    "wrpkru\n"
    "movq %r10, %rcx\n"
    "movq %r11, %rdx\n"
    /* Compute location to save old stack pointer (using r11) */
    "mov ia2_stackptr_1@GOTTPOFF(%rip), %r11\n"
    /* Write the old stack pointer to memory */
    "movq %rsp, %fs:(%r11)\n"
    /* Compute location to load new stack pointer (using r11) */
    "mov ia2_stackptr_2@GOTTPOFF(%rip), %r11\n"
    /* Read the new stack pointer from memory */
    "movq %fs:(%r11), %rsp\n"
    "subq $8, %rsp\n"
    /* Preserve essential regs on stack */
    "pushq %rdi\n"
    "pushq %rsi\n"
    /* Scrub non-essential regs */
    "call __libia2_scrub_registers\n"
    /* Restore preserved regs */
    "popq %rsi\n"
    "popq %rdi\n"
    /* Set PKRU to the compartment's value */
    "movq %rcx, %r10\n"
    "movq %rdx, %r11\n"
    "xorl %ecx, %ecx\n"
    "xorl %edx, %edx\n"
    "movl $0xffffffcc, %eax\n"
    "wrpkru\n"
    "movq %r10, %rcx\n"
    "movq %r11, %rdx\n"
    /* Call wrapped function */
    "call inflateReset2\n"
    /* Set PKRU to the intermediate value to move arguments */
    "movq %rax, %r10\n"
    "movq %rdx, %r11\n"
    "xorl %ecx, %ecx\n"
    "xorl %edx, %edx\n"
    "movl $0xffffffc0, %eax\n"
    "wrpkru\n"
    "movq %r10, %rax\n"
    "movq %r11, %rdx\n"
    /* Free stack space used for stack args */
    "addq $8, %rsp\n"
    /* Compute location to save old stack pointer (using r11) */
    "mov ia2_stackptr_2@GOTTPOFF(%rip), %r11\n"
    /* Write the old stack pointer to memory */
    "movq %rsp, %fs:(%r11)\n"
    /* Compute location to load new stack pointer (using r11) */
    "mov ia2_stackptr_1@GOTTPOFF(%rip), %r11\n"
    /* Read the new stack pointer from memory */
    "movq %fs:(%r11), %rsp\n"
    /* Preserve essential regs on stack */
    "pushq %rax\n"
    /* Scrub non-essential regs */
    "call __libia2_scrub_registers\n"
    /* Restore preserved regs */
    "popq %rax\n"
    /* Set PKRU to the caller's value */
    "movq %rax, %r10\n"
    "movq %rdx, %r11\n"
    "xorl %ecx, %ecx\n"
    "xorl %edx, %edx\n"
    "movl $0xfffffff0, %eax\n"
    "wrpkru\n"
    "movq %r10, %rax\n"
    "movq %r11, %rdx\n"
    "popq %r15\n"
    "popq %r14\n"
    "popq %r13\n"
    "popq %r12\n"
    "popq %rbx\n"
    "popq %rbp\n"
    /* Return to the caller */
    "ret\n"
    ".size __wrap_inflateReset2, .-__wrap_inflateReset2\n"
    ".previous\n"
);
asm(
    /* Wrapper for inflateResetKeep(int) -> int: */
    ".text\n"
    ".global __wrap_inflateResetKeep\n"
    ".type __wrap_inflateResetKeep, @function\n"
    "__wrap_inflateResetKeep:\n"
    "pushq %rbp\n"
    "movq %rsp, %rbp\n"
    "pushq %rbx\n"
    "pushq %r12\n"
    "pushq %r13\n"
    "pushq %r14\n"
    "pushq %r15\n"
    ASSERT_PKRU(0xfffffffffffffff0) "\n"
    /* Set PKRU to the intermediate value to move arguments */
    "movq %rcx, %r10\n"
    "movq %rdx, %r11\n"
    "xorl %ecx, %ecx\n"
    "xorl %edx, %edx\n"
    "movl $0xffffffc0, %eax\n"
    "wrpkru\n"
    "movq %r10, %rcx\n"
    "movq %r11, %rdx\n"
    /* Compute location to save old stack pointer (using r11) */
    "mov ia2_stackptr_1@GOTTPOFF(%rip), %r11\n"
    /* Write the old stack pointer to memory */
    "movq %rsp, %fs:(%r11)\n"
    /* Compute location to load new stack pointer (using r11) */
    "mov ia2_stackptr_2@GOTTPOFF(%rip), %r11\n"
    /* Read the new stack pointer from memory */
    "movq %fs:(%r11), %rsp\n"
    "subq $8, %rsp\n"
    /* Preserve essential regs on stack */
    "pushq %rdi\n"
    /* Scrub non-essential regs */
    "call __libia2_scrub_registers\n"
    /* Restore preserved regs */
    "popq %rdi\n"
    /* Set PKRU to the compartment's value */
    "movq %rcx, %r10\n"
    "movq %rdx, %r11\n"
    "xorl %ecx, %ecx\n"
    "xorl %edx, %edx\n"
    "movl $0xffffffcc, %eax\n"
    "wrpkru\n"
    "movq %r10, %rcx\n"
    "movq %r11, %rdx\n"
    /* Call wrapped function */
    "call inflateResetKeep\n"
    /* Set PKRU to the intermediate value to move arguments */
    "movq %rax, %r10\n"
    "movq %rdx, %r11\n"
    "xorl %ecx, %ecx\n"
    "xorl %edx, %edx\n"
    "movl $0xffffffc0, %eax\n"
    "wrpkru\n"
    "movq %r10, %rax\n"
    "movq %r11, %rdx\n"
    /* Free stack space used for stack args */
    "addq $8, %rsp\n"
    /* Compute location to save old stack pointer (using r11) */
    "mov ia2_stackptr_2@GOTTPOFF(%rip), %r11\n"
    /* Write the old stack pointer to memory */
    "movq %rsp, %fs:(%r11)\n"
    /* Compute location to load new stack pointer (using r11) */
    "mov ia2_stackptr_1@GOTTPOFF(%rip), %r11\n"
    /* Read the new stack pointer from memory */
    "movq %fs:(%r11), %rsp\n"
    /* Preserve essential regs on stack */
    "pushq %rax\n"
    /* Scrub non-essential regs */
    "call __libia2_scrub_registers\n"
    /* Restore preserved regs */
    "popq %rax\n"
    /* Set PKRU to the caller's value */
    "movq %rax, %r10\n"
    "movq %rdx, %r11\n"
    "xorl %ecx, %ecx\n"
    "xorl %edx, %edx\n"
    "movl $0xfffffff0, %eax\n"
    "wrpkru\n"
    "movq %r10, %rax\n"
    "movq %r11, %rdx\n"
    "popq %r15\n"
    "popq %r14\n"
    "popq %r13\n"
    "popq %r12\n"
    "popq %rbx\n"
    "popq %rbp\n"
    /* Return to the caller */
    "ret\n"
    ".size __wrap_inflateResetKeep, .-__wrap_inflateResetKeep\n"
    ".previous\n"
);
asm(
    /* Wrapper for inflateSetDictionary(int, int, int) -> int: */
    ".text\n"
    ".global __wrap_inflateSetDictionary\n"
    ".type __wrap_inflateSetDictionary, @function\n"
    "__wrap_inflateSetDictionary:\n"
    "pushq %rbp\n"
    "movq %rsp, %rbp\n"
    "pushq %rbx\n"
    "pushq %r12\n"
    "pushq %r13\n"
    "pushq %r14\n"
    "pushq %r15\n"
    ASSERT_PKRU(0xfffffffffffffff0) "\n"
    /* Set PKRU to the intermediate value to move arguments */
    "movq %rcx, %r10\n"
    "movq %rdx, %r11\n"
    "xorl %ecx, %ecx\n"
    "xorl %edx, %edx\n"
    "movl $0xffffffc0, %eax\n"
    "wrpkru\n"
    "movq %r10, %rcx\n"
    "movq %r11, %rdx\n"
    /* Compute location to save old stack pointer (using r11) */
    "mov ia2_stackptr_1@GOTTPOFF(%rip), %r11\n"
    /* Write the old stack pointer to memory */
    "movq %rsp, %fs:(%r11)\n"
    /* Compute location to load new stack pointer (using r11) */
    "mov ia2_stackptr_2@GOTTPOFF(%rip), %r11\n"
    /* Read the new stack pointer from memory */
    "movq %fs:(%r11), %rsp\n"
    "subq $8, %rsp\n"
    /* Preserve essential regs on stack */
    "pushq %rdi\n"
    "pushq %rsi\n"
    "pushq %rdx\n"
    /* Scrub non-essential regs */
    "call __libia2_scrub_registers\n"
    /* Restore preserved regs */
    "popq %rdx\n"
    "popq %rsi\n"
    "popq %rdi\n"
    /* Set PKRU to the compartment's value */
    "movq %rcx, %r10\n"
    "movq %rdx, %r11\n"
    "xorl %ecx, %ecx\n"
    "xorl %edx, %edx\n"
    "movl $0xffffffcc, %eax\n"
    "wrpkru\n"
    "movq %r10, %rcx\n"
    "movq %r11, %rdx\n"
    /* Call wrapped function */
    "call inflateSetDictionary\n"
    /* Set PKRU to the intermediate value to move arguments */
    "movq %rax, %r10\n"
    "movq %rdx, %r11\n"
    "xorl %ecx, %ecx\n"
    "xorl %edx, %edx\n"
    "movl $0xffffffc0, %eax\n"
    "wrpkru\n"
    "movq %r10, %rax\n"
    "movq %r11, %rdx\n"
    /* Free stack space used for stack args */
    "addq $8, %rsp\n"
    /* Compute location to save old stack pointer (using r11) */
    "mov ia2_stackptr_2@GOTTPOFF(%rip), %r11\n"
    /* Write the old stack pointer to memory */
    "movq %rsp, %fs:(%r11)\n"
    /* Compute location to load new stack pointer (using r11) */
    "mov ia2_stackptr_1@GOTTPOFF(%rip), %r11\n"
    /* Read the new stack pointer from memory */
    "movq %fs:(%r11), %rsp\n"
    /* Preserve essential regs on stack */
    "pushq %rax\n"
    /* Scrub non-essential regs */
    "call __libia2_scrub_registers\n"
    /* Restore preserved regs */
    "popq %rax\n"
    /* Set PKRU to the caller's value */
    "movq %rax, %r10\n"
    "movq %rdx, %r11\n"
    "xorl %ecx, %ecx\n"
    "xorl %edx, %edx\n"
    "movl $0xfffffff0, %eax\n"
    "wrpkru\n"
    "movq %r10, %rax\n"
    "movq %r11, %rdx\n"
    "popq %r15\n"
    "popq %r14\n"
    "popq %r13\n"
    "popq %r12\n"
    "popq %rbx\n"
    "popq %rbp\n"
    /* Return to the caller */
    "ret\n"
    ".size __wrap_inflateSetDictionary, .-__wrap_inflateSetDictionary\n"
    ".previous\n"
);
asm(
    /* Wrapper for inflateSync(int) -> int: */
    ".text\n"
    ".global __wrap_inflateSync\n"
    ".type __wrap_inflateSync, @function\n"
    "__wrap_inflateSync:\n"
    "pushq %rbp\n"
    "movq %rsp, %rbp\n"
    "pushq %rbx\n"
    "pushq %r12\n"
    "pushq %r13\n"
    "pushq %r14\n"
    "pushq %r15\n"
    ASSERT_PKRU(0xfffffffffffffff0) "\n"
    /* Set PKRU to the intermediate value to move arguments */
    "movq %rcx, %r10\n"
    "movq %rdx, %r11\n"
    "xorl %ecx, %ecx\n"
    "xorl %edx, %edx\n"
    "movl $0xffffffc0, %eax\n"
    "wrpkru\n"
    "movq %r10, %rcx\n"
    "movq %r11, %rdx\n"
    /* Compute location to save old stack pointer (using r11) */
    "mov ia2_stackptr_1@GOTTPOFF(%rip), %r11\n"
    /* Write the old stack pointer to memory */
    "movq %rsp, %fs:(%r11)\n"
    /* Compute location to load new stack pointer (using r11) */
    "mov ia2_stackptr_2@GOTTPOFF(%rip), %r11\n"
    /* Read the new stack pointer from memory */
    "movq %fs:(%r11), %rsp\n"
    "subq $8, %rsp\n"
    /* Preserve essential regs on stack */
    "pushq %rdi\n"
    /* Scrub non-essential regs */
    "call __libia2_scrub_registers\n"
    /* Restore preserved regs */
    "popq %rdi\n"
    /* Set PKRU to the compartment's value */
    "movq %rcx, %r10\n"
    "movq %rdx, %r11\n"
    "xorl %ecx, %ecx\n"
    "xorl %edx, %edx\n"
    "movl $0xffffffcc, %eax\n"
    "wrpkru\n"
    "movq %r10, %rcx\n"
    "movq %r11, %rdx\n"
    /* Call wrapped function */
    "call inflateSync\n"
    /* Set PKRU to the intermediate value to move arguments */
    "movq %rax, %r10\n"
    "movq %rdx, %r11\n"
    "xorl %ecx, %ecx\n"
    "xorl %edx, %edx\n"
    "movl $0xffffffc0, %eax\n"
    "wrpkru\n"
    "movq %r10, %rax\n"
    "movq %r11, %rdx\n"
    /* Free stack space used for stack args */
    "addq $8, %rsp\n"
    /* Compute location to save old stack pointer (using r11) */
    "mov ia2_stackptr_2@GOTTPOFF(%rip), %r11\n"
    /* Write the old stack pointer to memory */
    "movq %rsp, %fs:(%r11)\n"
    /* Compute location to load new stack pointer (using r11) */
    "mov ia2_stackptr_1@GOTTPOFF(%rip), %r11\n"
    /* Read the new stack pointer from memory */
    "movq %fs:(%r11), %rsp\n"
    /* Preserve essential regs on stack */
    "pushq %rax\n"
    /* Scrub non-essential regs */
    "call __libia2_scrub_registers\n"
    /* Restore preserved regs */
    "popq %rax\n"
    /* Set PKRU to the caller's value */
    "movq %rax, %r10\n"
    "movq %rdx, %r11\n"
    "xorl %ecx, %ecx\n"
    "xorl %edx, %edx\n"
    "movl $0xfffffff0, %eax\n"
    "wrpkru\n"
    "movq %r10, %rax\n"
    "movq %r11, %rdx\n"
    "popq %r15\n"
    "popq %r14\n"
    "popq %r13\n"
    "popq %r12\n"
    "popq %rbx\n"
    "popq %rbp\n"
    /* Return to the caller */
    "ret\n"
    ".size __wrap_inflateSync, .-__wrap_inflateSync\n"
    ".previous\n"
);
asm(
    /* Wrapper for inflateSyncPoint(int) -> int: */
    ".text\n"
    ".global __wrap_inflateSyncPoint\n"
    ".type __wrap_inflateSyncPoint, @function\n"
    "__wrap_inflateSyncPoint:\n"
    "pushq %rbp\n"
    "movq %rsp, %rbp\n"
    "pushq %rbx\n"
    "pushq %r12\n"
    "pushq %r13\n"
    "pushq %r14\n"
    "pushq %r15\n"
    ASSERT_PKRU(0xfffffffffffffff0) "\n"
    /* Set PKRU to the intermediate value to move arguments */
    "movq %rcx, %r10\n"
    "movq %rdx, %r11\n"
    "xorl %ecx, %ecx\n"
    "xorl %edx, %edx\n"
    "movl $0xffffffc0, %eax\n"
    "wrpkru\n"
    "movq %r10, %rcx\n"
    "movq %r11, %rdx\n"
    /* Compute location to save old stack pointer (using r11) */
    "mov ia2_stackptr_1@GOTTPOFF(%rip), %r11\n"
    /* Write the old stack pointer to memory */
    "movq %rsp, %fs:(%r11)\n"
    /* Compute location to load new stack pointer (using r11) */
    "mov ia2_stackptr_2@GOTTPOFF(%rip), %r11\n"
    /* Read the new stack pointer from memory */
    "movq %fs:(%r11), %rsp\n"
    "subq $8, %rsp\n"
    /* Preserve essential regs on stack */
    "pushq %rdi\n"
    /* Scrub non-essential regs */
    "call __libia2_scrub_registers\n"
    /* Restore preserved regs */
    "popq %rdi\n"
    /* Set PKRU to the compartment's value */
    "movq %rcx, %r10\n"
    "movq %rdx, %r11\n"
    "xorl %ecx, %ecx\n"
    "xorl %edx, %edx\n"
    "movl $0xffffffcc, %eax\n"
    "wrpkru\n"
    "movq %r10, %rcx\n"
    "movq %r11, %rdx\n"
    /* Call wrapped function */
    "call inflateSyncPoint\n"
    /* Set PKRU to the intermediate value to move arguments */
    "movq %rax, %r10\n"
    "movq %rdx, %r11\n"
    "xorl %ecx, %ecx\n"
    "xorl %edx, %edx\n"
    "movl $0xffffffc0, %eax\n"
    "wrpkru\n"
    "movq %r10, %rax\n"
    "movq %r11, %rdx\n"
    /* Free stack space used for stack args */
    "addq $8, %rsp\n"
    /* Compute location to save old stack pointer (using r11) */
    "mov ia2_stackptr_2@GOTTPOFF(%rip), %r11\n"
    /* Write the old stack pointer to memory */
    "movq %rsp, %fs:(%r11)\n"
    /* Compute location to load new stack pointer (using r11) */
    "mov ia2_stackptr_1@GOTTPOFF(%rip), %r11\n"
    /* Read the new stack pointer from memory */
    "movq %fs:(%r11), %rsp\n"
    /* Preserve essential regs on stack */
    "pushq %rax\n"
    /* Scrub non-essential regs */
    "call __libia2_scrub_registers\n"
    /* Restore preserved regs */
    "popq %rax\n"
    /* Set PKRU to the caller's value */
    "movq %rax, %r10\n"
    "movq %rdx, %r11\n"
    "xorl %ecx, %ecx\n"
    "xorl %edx, %edx\n"
    "movl $0xfffffff0, %eax\n"
    "wrpkru\n"
    "movq %r10, %rax\n"
    "movq %r11, %rdx\n"
    "popq %r15\n"
    "popq %r14\n"
    "popq %r13\n"
    "popq %r12\n"
    "popq %rbx\n"
    "popq %rbp\n"
    /* Return to the caller */
    "ret\n"
    ".size __wrap_inflateSyncPoint, .-__wrap_inflateSyncPoint\n"
    ".previous\n"
);
asm(
    /* Wrapper for inflateUndermine(int, int) -> int: */
    ".text\n"
    ".global __wrap_inflateUndermine\n"
    ".type __wrap_inflateUndermine, @function\n"
    "__wrap_inflateUndermine:\n"
    "pushq %rbp\n"
    "movq %rsp, %rbp\n"
    "pushq %rbx\n"
    "pushq %r12\n"
    "pushq %r13\n"
    "pushq %r14\n"
    "pushq %r15\n"
    ASSERT_PKRU(0xfffffffffffffff0) "\n"
    /* Set PKRU to the intermediate value to move arguments */
    "movq %rcx, %r10\n"
    "movq %rdx, %r11\n"
    "xorl %ecx, %ecx\n"
    "xorl %edx, %edx\n"
    "movl $0xffffffc0, %eax\n"
    "wrpkru\n"
    "movq %r10, %rcx\n"
    "movq %r11, %rdx\n"
    /* Compute location to save old stack pointer (using r11) */
    "mov ia2_stackptr_1@GOTTPOFF(%rip), %r11\n"
    /* Write the old stack pointer to memory */
    "movq %rsp, %fs:(%r11)\n"
    /* Compute location to load new stack pointer (using r11) */
    "mov ia2_stackptr_2@GOTTPOFF(%rip), %r11\n"
    /* Read the new stack pointer from memory */
    "movq %fs:(%r11), %rsp\n"
    "subq $8, %rsp\n"
    /* Preserve essential regs on stack */
    "pushq %rdi\n"
    "pushq %rsi\n"
    /* Scrub non-essential regs */
    "call __libia2_scrub_registers\n"
    /* Restore preserved regs */
    "popq %rsi\n"
    "popq %rdi\n"
    /* Set PKRU to the compartment's value */
    "movq %rcx, %r10\n"
    "movq %rdx, %r11\n"
    "xorl %ecx, %ecx\n"
    "xorl %edx, %edx\n"
    "movl $0xffffffcc, %eax\n"
    "wrpkru\n"
    "movq %r10, %rcx\n"
    "movq %r11, %rdx\n"
    /* Call wrapped function */
    "call inflateUndermine\n"
    /* Set PKRU to the intermediate value to move arguments */
    "movq %rax, %r10\n"
    "movq %rdx, %r11\n"
    "xorl %ecx, %ecx\n"
    "xorl %edx, %edx\n"
    "movl $0xffffffc0, %eax\n"
    "wrpkru\n"
    "movq %r10, %rax\n"
    "movq %r11, %rdx\n"
    /* Free stack space used for stack args */
    "addq $8, %rsp\n"
    /* Compute location to save old stack pointer (using r11) */
    "mov ia2_stackptr_2@GOTTPOFF(%rip), %r11\n"
    /* Write the old stack pointer to memory */
    "movq %rsp, %fs:(%r11)\n"
    /* Compute location to load new stack pointer (using r11) */
    "mov ia2_stackptr_1@GOTTPOFF(%rip), %r11\n"
    /* Read the new stack pointer from memory */
    "movq %fs:(%r11), %rsp\n"
    /* Preserve essential regs on stack */
    "pushq %rax\n"
    /* Scrub non-essential regs */
    "call __libia2_scrub_registers\n"
    /* Restore preserved regs */
    "popq %rax\n"
    /* Set PKRU to the caller's value */
    "movq %rax, %r10\n"
    "movq %rdx, %r11\n"
    "xorl %ecx, %ecx\n"
    "xorl %edx, %edx\n"
    "movl $0xfffffff0, %eax\n"
    "wrpkru\n"
    "movq %r10, %rax\n"
    "movq %r11, %rdx\n"
    "popq %r15\n"
    "popq %r14\n"
    "popq %r13\n"
    "popq %r12\n"
    "popq %rbx\n"
    "popq %rbp\n"
    /* Return to the caller */
    "ret\n"
    ".size __wrap_inflateUndermine, .-__wrap_inflateUndermine\n"
    ".previous\n"
);
asm(
    /* Wrapper for inflateValidate(int, int) -> int: */
    ".text\n"
    ".global __wrap_inflateValidate\n"
    ".type __wrap_inflateValidate, @function\n"
    "__wrap_inflateValidate:\n"
    "pushq %rbp\n"
    "movq %rsp, %rbp\n"
    "pushq %rbx\n"
    "pushq %r12\n"
    "pushq %r13\n"
    "pushq %r14\n"
    "pushq %r15\n"
    ASSERT_PKRU(0xfffffffffffffff0) "\n"
    /* Set PKRU to the intermediate value to move arguments */
    "movq %rcx, %r10\n"
    "movq %rdx, %r11\n"
    "xorl %ecx, %ecx\n"
    "xorl %edx, %edx\n"
    "movl $0xffffffc0, %eax\n"
    "wrpkru\n"
    "movq %r10, %rcx\n"
    "movq %r11, %rdx\n"
    /* Compute location to save old stack pointer (using r11) */
    "mov ia2_stackptr_1@GOTTPOFF(%rip), %r11\n"
    /* Write the old stack pointer to memory */
    "movq %rsp, %fs:(%r11)\n"
    /* Compute location to load new stack pointer (using r11) */
    "mov ia2_stackptr_2@GOTTPOFF(%rip), %r11\n"
    /* Read the new stack pointer from memory */
    "movq %fs:(%r11), %rsp\n"
    "subq $8, %rsp\n"
    /* Preserve essential regs on stack */
    "pushq %rdi\n"
    "pushq %rsi\n"
    /* Scrub non-essential regs */
    "call __libia2_scrub_registers\n"
    /* Restore preserved regs */
    "popq %rsi\n"
    "popq %rdi\n"
    /* Set PKRU to the compartment's value */
    "movq %rcx, %r10\n"
    "movq %rdx, %r11\n"
    "xorl %ecx, %ecx\n"
    "xorl %edx, %edx\n"
    "movl $0xffffffcc, %eax\n"
    "wrpkru\n"
    "movq %r10, %rcx\n"
    "movq %r11, %rdx\n"
    /* Call wrapped function */
    "call inflateValidate\n"
    /* Set PKRU to the intermediate value to move arguments */
    "movq %rax, %r10\n"
    "movq %rdx, %r11\n"
    "xorl %ecx, %ecx\n"
    "xorl %edx, %edx\n"
    "movl $0xffffffc0, %eax\n"
    "wrpkru\n"
    "movq %r10, %rax\n"
    "movq %r11, %rdx\n"
    /* Free stack space used for stack args */
    "addq $8, %rsp\n"
    /* Compute location to save old stack pointer (using r11) */
    "mov ia2_stackptr_2@GOTTPOFF(%rip), %r11\n"
    /* Write the old stack pointer to memory */
    "movq %rsp, %fs:(%r11)\n"
    /* Compute location to load new stack pointer (using r11) */
    "mov ia2_stackptr_1@GOTTPOFF(%rip), %r11\n"
    /* Read the new stack pointer from memory */
    "movq %fs:(%r11), %rsp\n"
    /* Preserve essential regs on stack */
    "pushq %rax\n"
    /* Scrub non-essential regs */
    "call __libia2_scrub_registers\n"
    /* Restore preserved regs */
    "popq %rax\n"
    /* Set PKRU to the caller's value */
    "movq %rax, %r10\n"
    "movq %rdx, %r11\n"
    "xorl %ecx, %ecx\n"
    "xorl %edx, %edx\n"
    "movl $0xfffffff0, %eax\n"
    "wrpkru\n"
    "movq %r10, %rax\n"
    "movq %r11, %rdx\n"
    "popq %r15\n"
    "popq %r14\n"
    "popq %r13\n"
    "popq %r12\n"
    "popq %rbx\n"
    "popq %rbp\n"
    /* Return to the caller */
    "ret\n"
    ".size __wrap_inflateValidate, .-__wrap_inflateValidate\n"
    ".previous\n"
);
asm(
    /* Wrapper for uncompress(int, int, int, int) -> int: */
    ".text\n"
    ".global __wrap_uncompress\n"
    ".type __wrap_uncompress, @function\n"
    "__wrap_uncompress:\n"
    "pushq %rbp\n"
    "movq %rsp, %rbp\n"
    "pushq %rbx\n"
    "pushq %r12\n"
    "pushq %r13\n"
    "pushq %r14\n"
    "pushq %r15\n"
    ASSERT_PKRU(0xfffffffffffffff0) "\n"
    /* Set PKRU to the intermediate value to move arguments */
    "movq %rcx, %r10\n"
    "movq %rdx, %r11\n"
    "xorl %ecx, %ecx\n"
    "xorl %edx, %edx\n"
    "movl $0xffffffc0, %eax\n"
    "wrpkru\n"
    "movq %r10, %rcx\n"
    "movq %r11, %rdx\n"
    /* Compute location to save old stack pointer (using r11) */
    "mov ia2_stackptr_1@GOTTPOFF(%rip), %r11\n"
    /* Write the old stack pointer to memory */
    "movq %rsp, %fs:(%r11)\n"
    /* Compute location to load new stack pointer (using r11) */
    "mov ia2_stackptr_2@GOTTPOFF(%rip), %r11\n"
    /* Read the new stack pointer from memory */
    "movq %fs:(%r11), %rsp\n"
    "subq $8, %rsp\n"
    /* Preserve essential regs on stack */
    "pushq %rdi\n"
    "pushq %rsi\n"
    "pushq %rdx\n"
    "pushq %rcx\n"
    /* Scrub non-essential regs */
    "call __libia2_scrub_registers\n"
    /* Restore preserved regs */
    "popq %rcx\n"
    "popq %rdx\n"
    "popq %rsi\n"
    "popq %rdi\n"
    /* Set PKRU to the compartment's value */
    "movq %rcx, %r10\n"
    "movq %rdx, %r11\n"
    "xorl %ecx, %ecx\n"
    "xorl %edx, %edx\n"
    "movl $0xffffffcc, %eax\n"
    "wrpkru\n"
    "movq %r10, %rcx\n"
    "movq %r11, %rdx\n"
    /* Call wrapped function */
    "call uncompress\n"
    /* Set PKRU to the intermediate value to move arguments */
    "movq %rax, %r10\n"
    "movq %rdx, %r11\n"
    "xorl %ecx, %ecx\n"
    "xorl %edx, %edx\n"
    "movl $0xffffffc0, %eax\n"
    "wrpkru\n"
    "movq %r10, %rax\n"
    "movq %r11, %rdx\n"
    /* Free stack space used for stack args */
    "addq $8, %rsp\n"
    /* Compute location to save old stack pointer (using r11) */
    "mov ia2_stackptr_2@GOTTPOFF(%rip), %r11\n"
    /* Write the old stack pointer to memory */
    "movq %rsp, %fs:(%r11)\n"
    /* Compute location to load new stack pointer (using r11) */
    "mov ia2_stackptr_1@GOTTPOFF(%rip), %r11\n"
    /* Read the new stack pointer from memory */
    "movq %fs:(%r11), %rsp\n"
    /* Preserve essential regs on stack */
    "pushq %rax\n"
    /* Scrub non-essential regs */
    "call __libia2_scrub_registers\n"
    /* Restore preserved regs */
    "popq %rax\n"
    /* Set PKRU to the caller's value */
    "movq %rax, %r10\n"
    "movq %rdx, %r11\n"
    "xorl %ecx, %ecx\n"
    "xorl %edx, %edx\n"
    "movl $0xfffffff0, %eax\n"
    "wrpkru\n"
    "movq %r10, %rax\n"
    "movq %r11, %rdx\n"
    "popq %r15\n"
    "popq %r14\n"
    "popq %r13\n"
    "popq %r12\n"
    "popq %rbx\n"
    "popq %rbp\n"
    /* Return to the caller */
    "ret\n"
    ".size __wrap_uncompress, .-__wrap_uncompress\n"
    ".previous\n"
);
asm(
    /* Wrapper for uncompress2(int, int, int, int) -> int: */
    ".text\n"
    ".global __wrap_uncompress2\n"
    ".type __wrap_uncompress2, @function\n"
    "__wrap_uncompress2:\n"
    "pushq %rbp\n"
    "movq %rsp, %rbp\n"
    "pushq %rbx\n"
    "pushq %r12\n"
    "pushq %r13\n"
    "pushq %r14\n"
    "pushq %r15\n"
    ASSERT_PKRU(0xfffffffffffffff0) "\n"
    /* Set PKRU to the intermediate value to move arguments */
    "movq %rcx, %r10\n"
    "movq %rdx, %r11\n"
    "xorl %ecx, %ecx\n"
    "xorl %edx, %edx\n"
    "movl $0xffffffc0, %eax\n"
    "wrpkru\n"
    "movq %r10, %rcx\n"
    "movq %r11, %rdx\n"
    /* Compute location to save old stack pointer (using r11) */
    "mov ia2_stackptr_1@GOTTPOFF(%rip), %r11\n"
    /* Write the old stack pointer to memory */
    "movq %rsp, %fs:(%r11)\n"
    /* Compute location to load new stack pointer (using r11) */
    "mov ia2_stackptr_2@GOTTPOFF(%rip), %r11\n"
    /* Read the new stack pointer from memory */
    "movq %fs:(%r11), %rsp\n"
    "subq $8, %rsp\n"
    /* Preserve essential regs on stack */
    "pushq %rdi\n"
    "pushq %rsi\n"
    "pushq %rdx\n"
    "pushq %rcx\n"
    /* Scrub non-essential regs */
    "call __libia2_scrub_registers\n"
    /* Restore preserved regs */
    "popq %rcx\n"
    "popq %rdx\n"
    "popq %rsi\n"
    "popq %rdi\n"
    /* Set PKRU to the compartment's value */
    "movq %rcx, %r10\n"
    "movq %rdx, %r11\n"
    "xorl %ecx, %ecx\n"
    "xorl %edx, %edx\n"
    "movl $0xffffffcc, %eax\n"
    "wrpkru\n"
    "movq %r10, %rcx\n"
    "movq %r11, %rdx\n"
    /* Call wrapped function */
    "call uncompress2\n"
    /* Set PKRU to the intermediate value to move arguments */
    "movq %rax, %r10\n"
    "movq %rdx, %r11\n"
    "xorl %ecx, %ecx\n"
    "xorl %edx, %edx\n"
    "movl $0xffffffc0, %eax\n"
    "wrpkru\n"
    "movq %r10, %rax\n"
    "movq %r11, %rdx\n"
    /* Free stack space used for stack args */
    "addq $8, %rsp\n"
    /* Compute location to save old stack pointer (using r11) */
    "mov ia2_stackptr_2@GOTTPOFF(%rip), %r11\n"
    /* Write the old stack pointer to memory */
    "movq %rsp, %fs:(%r11)\n"
    /* Compute location to load new stack pointer (using r11) */
    "mov ia2_stackptr_1@GOTTPOFF(%rip), %r11\n"
    /* Read the new stack pointer from memory */
    "movq %fs:(%r11), %rsp\n"
    /* Preserve essential regs on stack */
    "pushq %rax\n"
    /* Scrub non-essential regs */
    "call __libia2_scrub_registers\n"
    /* Restore preserved regs */
    "popq %rax\n"
    /* Set PKRU to the caller's value */
    "movq %rax, %r10\n"
    "movq %rdx, %r11\n"
    "xorl %ecx, %ecx\n"
    "xorl %edx, %edx\n"
    "movl $0xfffffff0, %eax\n"
    "wrpkru\n"
    "movq %r10, %rax\n"
    "movq %r11, %rdx\n"
    "popq %r15\n"
    "popq %r14\n"
    "popq %r13\n"
    "popq %r12\n"
    "popq %rbx\n"
    "popq %rbp\n"
    /* Return to the caller */
    "ret\n"
    ".size __wrap_uncompress2, .-__wrap_uncompress2\n"
    ".previous\n"
);
asm(
    /* Wrapper for zError(int) -> int: */
    ".text\n"
    ".global __wrap_zError\n"
    ".type __wrap_zError, @function\n"
    "__wrap_zError:\n"
    "pushq %rbp\n"
    "movq %rsp, %rbp\n"
    "pushq %rbx\n"
    "pushq %r12\n"
    "pushq %r13\n"
    "pushq %r14\n"
    "pushq %r15\n"
    ASSERT_PKRU(0xfffffffffffffff0) "\n"
    /* Set PKRU to the intermediate value to move arguments */
    "movq %rcx, %r10\n"
    "movq %rdx, %r11\n"
    "xorl %ecx, %ecx\n"
    "xorl %edx, %edx\n"
    "movl $0xffffffc0, %eax\n"
    "wrpkru\n"
    "movq %r10, %rcx\n"
    "movq %r11, %rdx\n"
    /* Compute location to save old stack pointer (using r11) */
    "mov ia2_stackptr_1@GOTTPOFF(%rip), %r11\n"
    /* Write the old stack pointer to memory */
    "movq %rsp, %fs:(%r11)\n"
    /* Compute location to load new stack pointer (using r11) */
    "mov ia2_stackptr_2@GOTTPOFF(%rip), %r11\n"
    /* Read the new stack pointer from memory */
    "movq %fs:(%r11), %rsp\n"
    "subq $8, %rsp\n"
    /* Preserve essential regs on stack */
    "pushq %rdi\n"
    /* Scrub non-essential regs */
    "call __libia2_scrub_registers\n"
    /* Restore preserved regs */
    "popq %rdi\n"
    /* Set PKRU to the compartment's value */
    "movq %rcx, %r10\n"
    "movq %rdx, %r11\n"
    "xorl %ecx, %ecx\n"
    "xorl %edx, %edx\n"
    "movl $0xffffffcc, %eax\n"
    "wrpkru\n"
    "movq %r10, %rcx\n"
    "movq %r11, %rdx\n"
    /* Call wrapped function */
    "call zError\n"
    /* Set PKRU to the intermediate value to move arguments */
    "movq %rax, %r10\n"
    "movq %rdx, %r11\n"
    "xorl %ecx, %ecx\n"
    "xorl %edx, %edx\n"
    "movl $0xffffffc0, %eax\n"
    "wrpkru\n"
    "movq %r10, %rax\n"
    "movq %r11, %rdx\n"
    /* Free stack space used for stack args */
    "addq $8, %rsp\n"
    /* Compute location to save old stack pointer (using r11) */
    "mov ia2_stackptr_2@GOTTPOFF(%rip), %r11\n"
    /* Write the old stack pointer to memory */
    "movq %rsp, %fs:(%r11)\n"
    /* Compute location to load new stack pointer (using r11) */
    "mov ia2_stackptr_1@GOTTPOFF(%rip), %r11\n"
    /* Read the new stack pointer from memory */
    "movq %fs:(%r11), %rsp\n"
    /* Preserve essential regs on stack */
    "pushq %rax\n"
    /* Scrub non-essential regs */
    "call __libia2_scrub_registers\n"
    /* Restore preserved regs */
    "popq %rax\n"
    /* Set PKRU to the caller's value */
    "movq %rax, %r10\n"
    "movq %rdx, %r11\n"
    "xorl %ecx, %ecx\n"
    "xorl %edx, %edx\n"
    "movl $0xfffffff0, %eax\n"
    "wrpkru\n"
    "movq %r10, %rax\n"
    "movq %r11, %rdx\n"
    "popq %r15\n"
    "popq %r14\n"
    "popq %r13\n"
    "popq %r12\n"
    "popq %rbx\n"
    "popq %rbp\n"
    /* Return to the caller */
    "ret\n"
    ".size __wrap_zError, .-__wrap_zError\n"
    ".previous\n"
);
asm(
    /* Wrapper for zlibCompileFlags() -> int: */
    ".text\n"
    ".global __wrap_zlibCompileFlags\n"
    ".type __wrap_zlibCompileFlags, @function\n"
    "__wrap_zlibCompileFlags:\n"
    "pushq %rbp\n"
    "movq %rsp, %rbp\n"
    "pushq %rbx\n"
    "pushq %r12\n"
    "pushq %r13\n"
    "pushq %r14\n"
    "pushq %r15\n"
    ASSERT_PKRU(0xfffffffffffffff0) "\n"
    /* Set PKRU to the intermediate value to move arguments */
    "movq %rcx, %r10\n"
    "movq %rdx, %r11\n"
    "xorl %ecx, %ecx\n"
    "xorl %edx, %edx\n"
    "movl $0xffffffc0, %eax\n"
    "wrpkru\n"
    "movq %r10, %rcx\n"
    "movq %r11, %rdx\n"
    /* Compute location to save old stack pointer (using r11) */
    "mov ia2_stackptr_1@GOTTPOFF(%rip), %r11\n"
    /* Write the old stack pointer to memory */
    "movq %rsp, %fs:(%r11)\n"
    /* Compute location to load new stack pointer (using r11) */
    "mov ia2_stackptr_2@GOTTPOFF(%rip), %r11\n"
    /* Read the new stack pointer from memory */
    "movq %fs:(%r11), %rsp\n"
    "subq $8, %rsp\n"
    /* Scrub non-essential regs */
    "call __libia2_scrub_registers\n"
    /* Set PKRU to the compartment's value */
    "movq %rcx, %r10\n"
    "movq %rdx, %r11\n"
    "xorl %ecx, %ecx\n"
    "xorl %edx, %edx\n"
    "movl $0xffffffcc, %eax\n"
    "wrpkru\n"
    "movq %r10, %rcx\n"
    "movq %r11, %rdx\n"
    /* Call wrapped function */
    "call zlibCompileFlags\n"
    /* Set PKRU to the intermediate value to move arguments */
    "movq %rax, %r10\n"
    "movq %rdx, %r11\n"
    "xorl %ecx, %ecx\n"
    "xorl %edx, %edx\n"
    "movl $0xffffffc0, %eax\n"
    "wrpkru\n"
    "movq %r10, %rax\n"
    "movq %r11, %rdx\n"
    /* Free stack space used for stack args */
    "addq $8, %rsp\n"
    /* Compute location to save old stack pointer (using r11) */
    "mov ia2_stackptr_2@GOTTPOFF(%rip), %r11\n"
    /* Write the old stack pointer to memory */
    "movq %rsp, %fs:(%r11)\n"
    /* Compute location to load new stack pointer (using r11) */
    "mov ia2_stackptr_1@GOTTPOFF(%rip), %r11\n"
    /* Read the new stack pointer from memory */
    "movq %fs:(%r11), %rsp\n"
    /* Preserve essential regs on stack */
    "pushq %rax\n"
    /* Scrub non-essential regs */
    "call __libia2_scrub_registers\n"
    /* Restore preserved regs */
    "popq %rax\n"
    /* Set PKRU to the caller's value */
    "movq %rax, %r10\n"
    "movq %rdx, %r11\n"
    "xorl %ecx, %ecx\n"
    "xorl %edx, %edx\n"
    "movl $0xfffffff0, %eax\n"
    "wrpkru\n"
    "movq %r10, %rax\n"
    "movq %r11, %rdx\n"
    "popq %r15\n"
    "popq %r14\n"
    "popq %r13\n"
    "popq %r12\n"
    "popq %rbx\n"
    "popq %rbp\n"
    /* Return to the caller */
    "ret\n"
    ".size __wrap_zlibCompileFlags, .-__wrap_zlibCompileFlags\n"
    ".previous\n"
);
asm(
    /* Wrapper for zlibVersion() -> int: */
    ".text\n"
    ".global __wrap_zlibVersion\n"
    ".type __wrap_zlibVersion, @function\n"
    "__wrap_zlibVersion:\n"
    "pushq %rbp\n"
    "movq %rsp, %rbp\n"
    "pushq %rbx\n"
    "pushq %r12\n"
    "pushq %r13\n"
    "pushq %r14\n"
    "pushq %r15\n"
    ASSERT_PKRU(0xfffffffffffffff0) "\n"
    /* Set PKRU to the intermediate value to move arguments */
    "movq %rcx, %r10\n"
    "movq %rdx, %r11\n"
    "xorl %ecx, %ecx\n"
    "xorl %edx, %edx\n"
    "movl $0xffffffc0, %eax\n"
    "wrpkru\n"
    "movq %r10, %rcx\n"
    "movq %r11, %rdx\n"
    /* Compute location to save old stack pointer (using r11) */
    "mov ia2_stackptr_1@GOTTPOFF(%rip), %r11\n"
    /* Write the old stack pointer to memory */
    "movq %rsp, %fs:(%r11)\n"
    /* Compute location to load new stack pointer (using r11) */
    "mov ia2_stackptr_2@GOTTPOFF(%rip), %r11\n"
    /* Read the new stack pointer from memory */
    "movq %fs:(%r11), %rsp\n"
    "subq $8, %rsp\n"
    /* Scrub non-essential regs */
    "call __libia2_scrub_registers\n"
    /* Set PKRU to the compartment's value */
    "movq %rcx, %r10\n"
    "movq %rdx, %r11\n"
    "xorl %ecx, %ecx\n"
    "xorl %edx, %edx\n"
    "movl $0xffffffcc, %eax\n"
    "wrpkru\n"
    "movq %r10, %rcx\n"
    "movq %r11, %rdx\n"
    /* Call wrapped function */
    "call zlibVersion\n"
    /* Set PKRU to the intermediate value to move arguments */
    "movq %rax, %r10\n"
    "movq %rdx, %r11\n"
    "xorl %ecx, %ecx\n"
    "xorl %edx, %edx\n"
    "movl $0xffffffc0, %eax\n"
    "wrpkru\n"
    "movq %r10, %rax\n"
    "movq %r11, %rdx\n"
    /* Free stack space used for stack args */
    "addq $8, %rsp\n"
    /* Compute location to save old stack pointer (using r11) */
    "mov ia2_stackptr_2@GOTTPOFF(%rip), %r11\n"
    /* Write the old stack pointer to memory */
    "movq %rsp, %fs:(%r11)\n"
    /* Compute location to load new stack pointer (using r11) */
    "mov ia2_stackptr_1@GOTTPOFF(%rip), %r11\n"
    /* Read the new stack pointer from memory */
    "movq %fs:(%r11), %rsp\n"
    /* Preserve essential regs on stack */
    "pushq %rax\n"
    /* Scrub non-essential regs */
    "call __libia2_scrub_registers\n"
    /* Restore preserved regs */
    "popq %rax\n"
    /* Set PKRU to the caller's value */
    "movq %rax, %r10\n"
    "movq %rdx, %r11\n"
    "xorl %ecx, %ecx\n"
    "xorl %edx, %edx\n"
    "movl $0xfffffff0, %eax\n"
    "wrpkru\n"
    "movq %r10, %rax\n"
    "movq %r11, %rdx\n"
    "popq %r15\n"
    "popq %r14\n"
    "popq %r13\n"
    "popq %r12\n"
    "popq %rbx\n"
    "popq %rbp\n"
    /* Return to the caller */
    "ret\n"
    ".size __wrap_zlibVersion, .-__wrap_zlibVersion\n"
    ".previous\n"
);
asm(
    /* Wrapper for ia2_compartment_destructor_1(): */
    ".text\n"
    ".global __wrap_ia2_compartment_destructor_1\n"
    ".type __wrap_ia2_compartment_destructor_1, @function\n"
    "__wrap_ia2_compartment_destructor_1:\n"
    "pushq %rbp\n"
    "movq %rsp, %rbp\n"
    "pushq %rbx\n"
    "pushq %r12\n"
    "pushq %r13\n"
    "pushq %r14\n"
    "pushq %r15\n"
    ASSERT_PKRU(0xfffffffffffffffc) "\n"
    /* Set PKRU to the intermediate value to move arguments */
    "movq %rcx, %r10\n"
    "movq %rdx, %r11\n"
    "xorl %ecx, %ecx\n"
    "xorl %edx, %edx\n"
    "movl $0xfffffff0, %eax\n"
    "wrpkru\n"
    "movq %r10, %rcx\n"
    "movq %r11, %rdx\n"
    /* Compute location to save old stack pointer (using r11) */
    "mov ia2_stackptr_0@GOTTPOFF(%rip), %r11\n"
    /* Write the old stack pointer to memory */
    "movq %rsp, %fs:(%r11)\n"
    /* Compute location to load new stack pointer (using r11) */
    "mov ia2_stackptr_1@GOTTPOFF(%rip), %r11\n"
    /* Read the new stack pointer from memory */
    "movq %fs:(%r11), %rsp\n"
    "subq $8, %rsp\n"
    /* Set PKRU to the compartment's value */
    "movq %rcx, %r10\n"
    "movq %rdx, %r11\n"
    "xorl %ecx, %ecx\n"
    "xorl %edx, %edx\n"
    "movl $0xfffffff0, %eax\n"
    "wrpkru\n"
    "movq %r10, %rcx\n"
    "movq %r11, %rdx\n"
    /* Call wrapped function */
    "call ia2_compartment_destructor_1\n"
    /* Set PKRU to the intermediate value to move arguments */
    "movq %rax, %r10\n"
    "movq %rdx, %r11\n"
    "xorl %ecx, %ecx\n"
    "xorl %edx, %edx\n"
    "movl $0xfffffff0, %eax\n"
    "wrpkru\n"
    "movq %r10, %rax\n"
    "movq %r11, %rdx\n"
    /* Free stack space used for stack args */
    "addq $8, %rsp\n"
    /* Compute location to save old stack pointer (using r11) */
    "mov ia2_stackptr_1@GOTTPOFF(%rip), %r11\n"
    /* Write the old stack pointer to memory */
    "movq %rsp, %fs:(%r11)\n"
    /* Compute location to load new stack pointer (using r11) */
    "mov ia2_stackptr_0@GOTTPOFF(%rip), %r11\n"
    /* Read the new stack pointer from memory */
    "movq %fs:(%r11), %rsp\n"
    /* Scrub non-essential regs */
    "call __libia2_scrub_registers\n"
    /* Set PKRU to the caller's value */
    "movq %rax, %r10\n"
    "movq %rdx, %r11\n"
    "xorl %ecx, %ecx\n"
    "xorl %edx, %edx\n"
    "movl $0xfffffffc, %eax\n"
    "wrpkru\n"
    "movq %r10, %rax\n"
    "movq %r11, %rdx\n"
    "popq %r15\n"
    "popq %r14\n"
    "popq %r13\n"
    "popq %r12\n"
    "popq %rbx\n"
    "popq %rbp\n"
    /* Return to the caller */
    "ret\n"
    ".size __wrap_ia2_compartment_destructor_1, .-__wrap_ia2_compartment_destructor_1\n"
    ".previous\n"
);
asm(
    /* Wrapper for ia2_compartment_destructor_2(): */
    ".text\n"
    ".global __wrap_ia2_compartment_destructor_2\n"
    ".type __wrap_ia2_compartment_destructor_2, @function\n"
    "__wrap_ia2_compartment_destructor_2:\n"
    "pushq %rbp\n"
    "movq %rsp, %rbp\n"
    "pushq %rbx\n"
    "pushq %r12\n"
    "pushq %r13\n"
    "pushq %r14\n"
    "pushq %r15\n"
    ASSERT_PKRU(0xfffffffffffffffc) "\n"
    /* Set PKRU to the intermediate value to move arguments */
    "movq %rcx, %r10\n"
    "movq %rdx, %r11\n"
    "xorl %ecx, %ecx\n"
    "xorl %edx, %edx\n"
    "movl $0xffffffcc, %eax\n"
    "wrpkru\n"
    "movq %r10, %rcx\n"
    "movq %r11, %rdx\n"
    /* Compute location to save old stack pointer (using r11) */
    "mov ia2_stackptr_0@GOTTPOFF(%rip), %r11\n"
    /* Write the old stack pointer to memory */
    "movq %rsp, %fs:(%r11)\n"
    /* Compute location to load new stack pointer (using r11) */
    "mov ia2_stackptr_2@GOTTPOFF(%rip), %r11\n"
    /* Read the new stack pointer from memory */
    "movq %fs:(%r11), %rsp\n"
    "subq $8, %rsp\n"
    /* Set PKRU to the compartment's value */
    "movq %rcx, %r10\n"
    "movq %rdx, %r11\n"
    "xorl %ecx, %ecx\n"
    "xorl %edx, %edx\n"
    "movl $0xffffffcc, %eax\n"
    "wrpkru\n"
    "movq %r10, %rcx\n"
    "movq %r11, %rdx\n"
    /* Call wrapped function */
    "call ia2_compartment_destructor_2\n"
    /* Set PKRU to the intermediate value to move arguments */
    "movq %rax, %r10\n"
    "movq %rdx, %r11\n"
    "xorl %ecx, %ecx\n"
    "xorl %edx, %edx\n"
    "movl $0xffffffcc, %eax\n"
    "wrpkru\n"
    "movq %r10, %rax\n"
    "movq %r11, %rdx\n"
    /* Free stack space used for stack args */
    "addq $8, %rsp\n"
    /* Compute location to save old stack pointer (using r11) */
    "mov ia2_stackptr_2@GOTTPOFF(%rip), %r11\n"
    /* Write the old stack pointer to memory */
    "movq %rsp, %fs:(%r11)\n"
    /* Compute location to load new stack pointer (using r11) */
    "mov ia2_stackptr_0@GOTTPOFF(%rip), %r11\n"
    /* Read the new stack pointer from memory */
    "movq %fs:(%r11), %rsp\n"
    /* Scrub non-essential regs */
    "call __libia2_scrub_registers\n"
    /* Set PKRU to the caller's value */
    "movq %rax, %r10\n"
    "movq %rdx, %r11\n"
    "xorl %ecx, %ecx\n"
    "xorl %edx, %edx\n"
    "movl $0xfffffffc, %eax\n"
    "wrpkru\n"
    "movq %r10, %rax\n"
    "movq %r11, %rdx\n"
    "popq %r15\n"
    "popq %r14\n"
    "popq %r13\n"
    "popq %r12\n"
    "popq %rbx\n"
    "popq %rbp\n"
    /* Return to the caller */
    "ret\n"
    ".size __wrap_ia2_compartment_destructor_2, .-__wrap_ia2_compartment_destructor_2\n"
    ".previous\n"
);
