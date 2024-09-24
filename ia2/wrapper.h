#include <ia2.h>
#include <scrub_registers.h>

#define IA2_TYPE__ZTSPF11block_stateP14internal_stateiE block_state (*)(struct internal_state *, int)
#define IA2_TYPE__ZTSPFiPvPhjE int (*)(void *, unsigned char *, unsigned int)
#define IA2_TYPE__ZTSPFjPvPPhE unsigned int (*)(void *, unsigned char **)
#define IA2_TYPE__ZTSPFvPvS_E void (*)(void *, void *)
#define IA2_TYPE__ZTSPFPvS_jjE void *(*)(void *, unsigned int, unsigned int)
extern char __ia2_indirect_callgate__ZTSPF11block_stateP14internal_stateiE_pkey_1;
extern char __ia2_indirect_callgate__ZTSPFiPvPhjE_pkey_1;
extern char __ia2_indirect_callgate__ZTSPFjPvPPhE_pkey_1;
extern char __ia2_indirect_callgate__ZTSPFvPvS_E_pkey_1;
extern char __ia2_indirect_callgate__ZTSPFPvS_jjE_pkey_1;
extern char __ia2_indirect_callgate__ZTSPF11block_stateP14internal_stateiE_pkey_2;
extern char __ia2_indirect_callgate__ZTSPFiPvPhjE_pkey_2;
extern char __ia2_indirect_callgate__ZTSPFjPvPPhE_pkey_2;
extern char __ia2_indirect_callgate__ZTSPFvPvS_E_pkey_2;
extern char __ia2_indirect_callgate__ZTSPFPvS_jjE_pkey_2;
struct IA2_fnptr__ZTSF11block_stateP14internal_stateiE { char *ptr; };
struct IA2_fnptr__ZTSFPvS_jjE { char *ptr; };
struct IA2_fnptr__ZTSFiPvPhjE { char *ptr; };
struct IA2_fnptr__ZTSFjPvPPhE { char *ptr; };
struct IA2_fnptr__ZTSFvPvS_E { char *ptr; };
extern void *ia2_fn_ptr;
extern struct IA2_fnptr__ZTSFPvS_jjE __ia2_zcalloc;
extern struct IA2_fnptr__ZTSFvPvS_E __ia2_zcfree;
extern struct IA2_fnptr__ZTSF11block_stateP14internal_stateiE __ia2_deflate_fast;
extern struct IA2_fnptr__ZTSF11block_stateP14internal_stateiE __ia2_deflate_slow;
extern struct IA2_fnptr__ZTSF11block_stateP14internal_stateiE __ia2_deflate_stored;
asm("__libia2_abort:\n"
    "ud2");
#define IA2_DEFINE_WRAPPER_deflate_fast \
asm(\
    /* Wrapper for deflate_fast(int, int) -> int: */ \
    ".text\n" \
    ".local __ia2_deflate_fast\n" \
    ".type __ia2_deflate_fast, @function\n" \
    "__ia2_deflate_fast:\n" \
    "pushq %rbp\n" \
    "movq %rsp, %rbp\n" \
    "pushq %rbx\n" \
    "pushq %r12\n" \
    "pushq %r13\n" \
    "pushq %r14\n" \
    "pushq %r15\n" \
    ASSERT_PKRU(0xfffffffffffffffc) "\n" \
    /* Set PKRU to the intermediate value to move arguments */ \
    "movq %rcx, %r10\n" \
    "movq %rdx, %r11\n" \
    "xorl %ecx, %ecx\n" \
    "xorl %edx, %edx\n" \
    "movl $0xffffffcc, %eax\n" \
    "wrpkru\n" \
    "movq %r10, %rcx\n" \
    "movq %r11, %rdx\n" \
    /* Compute location to save old stack pointer (using r11) */ \
    "mov ia2_stackptr_0@GOTTPOFF(%rip), %r11\n" \
    /* Write the old stack pointer to memory */ \
    "movq %rsp, %fs:(%r11)\n" \
    /* Compute location to load new stack pointer (using r11) */ \
    "mov ia2_stackptr_2@GOTTPOFF(%rip), %r11\n" \
    /* Read the new stack pointer from memory */ \
    "movq %fs:(%r11), %rsp\n" \
    "subq $8, %rsp\n" \
    /* Set PKRU to the compartment's value */ \
    "movq %rcx, %r10\n" \
    "movq %rdx, %r11\n" \
    "xorl %ecx, %ecx\n" \
    "xorl %edx, %edx\n" \
    "movl $0xffffffcc, %eax\n" \
    "wrpkru\n" \
    "movq %r10, %rcx\n" \
    "movq %r11, %rdx\n" \
    /* Call wrapped function */ \
    "call deflate_fast\n" \
    /* Set PKRU to the intermediate value to move arguments */ \
    "movq %rax, %r10\n" \
    "movq %rdx, %r11\n" \
    "xorl %ecx, %ecx\n" \
    "xorl %edx, %edx\n" \
    "movl $0xffffffcc, %eax\n" \
    "wrpkru\n" \
    "movq %r10, %rax\n" \
    "movq %r11, %rdx\n" \
    "addq $8, %rsp\n" \
    /* Compute location to save old stack pointer (using r11) */ \
    "mov ia2_stackptr_2@GOTTPOFF(%rip), %r11\n" \
    /* Write the old stack pointer to memory */ \
    "movq %rsp, %fs:(%r11)\n" \
    /* Compute location to load new stack pointer (using r11) */ \
    "mov ia2_stackptr_0@GOTTPOFF(%rip), %r11\n" \
    /* Read the new stack pointer from memory */ \
    "movq %fs:(%r11), %rsp\n" \
    /* Preserve essential regs on stack */ \
    "pushq %rax\n" \
    /* Scrub non-essential regs */ \
    "call __libia2_scrub_registers\n" \
    /* Restore preserved regs */ \
    "popq %rax\n" \
    /* Set PKRU to the caller's value */ \
    "movq %rax, %r10\n" \
    "movq %rdx, %r11\n" \
    "xorl %ecx, %ecx\n" \
    "xorl %edx, %edx\n" \
    "movl $0xfffffffc, %eax\n" \
    "wrpkru\n" \
    "movq %r10, %rax\n" \
    "movq %r11, %rdx\n" \
    "popq %r15\n" \
    "popq %r14\n" \
    "popq %r13\n" \
    "popq %r12\n" \
    "popq %rbx\n" \
    "popq %rbp\n" \
    /* Return to the caller */ \
    "ret\n" \
    ".size __ia2_deflate_fast, .-__ia2_deflate_fast\n" \
    ".previous\n" \
);
#define IA2_DEFINE_WRAPPER_deflate_slow \
asm(\
    /* Wrapper for deflate_slow(int, int) -> int: */ \
    ".text\n" \
    ".local __ia2_deflate_slow\n" \
    ".type __ia2_deflate_slow, @function\n" \
    "__ia2_deflate_slow:\n" \
    "pushq %rbp\n" \
    "movq %rsp, %rbp\n" \
    "pushq %rbx\n" \
    "pushq %r12\n" \
    "pushq %r13\n" \
    "pushq %r14\n" \
    "pushq %r15\n" \
    ASSERT_PKRU(0xfffffffffffffffc) "\n" \
    /* Set PKRU to the intermediate value to move arguments */ \
    "movq %rcx, %r10\n" \
    "movq %rdx, %r11\n" \
    "xorl %ecx, %ecx\n" \
    "xorl %edx, %edx\n" \
    "movl $0xffffffcc, %eax\n" \
    "wrpkru\n" \
    "movq %r10, %rcx\n" \
    "movq %r11, %rdx\n" \
    /* Compute location to save old stack pointer (using r11) */ \
    "mov ia2_stackptr_0@GOTTPOFF(%rip), %r11\n" \
    /* Write the old stack pointer to memory */ \
    "movq %rsp, %fs:(%r11)\n" \
    /* Compute location to load new stack pointer (using r11) */ \
    "mov ia2_stackptr_2@GOTTPOFF(%rip), %r11\n" \
    /* Read the new stack pointer from memory */ \
    "movq %fs:(%r11), %rsp\n" \
    "subq $8, %rsp\n" \
    /* Set PKRU to the compartment's value */ \
    "movq %rcx, %r10\n" \
    "movq %rdx, %r11\n" \
    "xorl %ecx, %ecx\n" \
    "xorl %edx, %edx\n" \
    "movl $0xffffffcc, %eax\n" \
    "wrpkru\n" \
    "movq %r10, %rcx\n" \
    "movq %r11, %rdx\n" \
    /* Call wrapped function */ \
    "call deflate_slow\n" \
    /* Set PKRU to the intermediate value to move arguments */ \
    "movq %rax, %r10\n" \
    "movq %rdx, %r11\n" \
    "xorl %ecx, %ecx\n" \
    "xorl %edx, %edx\n" \
    "movl $0xffffffcc, %eax\n" \
    "wrpkru\n" \
    "movq %r10, %rax\n" \
    "movq %r11, %rdx\n" \
    "addq $8, %rsp\n" \
    /* Compute location to save old stack pointer (using r11) */ \
    "mov ia2_stackptr_2@GOTTPOFF(%rip), %r11\n" \
    /* Write the old stack pointer to memory */ \
    "movq %rsp, %fs:(%r11)\n" \
    /* Compute location to load new stack pointer (using r11) */ \
    "mov ia2_stackptr_0@GOTTPOFF(%rip), %r11\n" \
    /* Read the new stack pointer from memory */ \
    "movq %fs:(%r11), %rsp\n" \
    /* Preserve essential regs on stack */ \
    "pushq %rax\n" \
    /* Scrub non-essential regs */ \
    "call __libia2_scrub_registers\n" \
    /* Restore preserved regs */ \
    "popq %rax\n" \
    /* Set PKRU to the caller's value */ \
    "movq %rax, %r10\n" \
    "movq %rdx, %r11\n" \
    "xorl %ecx, %ecx\n" \
    "xorl %edx, %edx\n" \
    "movl $0xfffffffc, %eax\n" \
    "wrpkru\n" \
    "movq %r10, %rax\n" \
    "movq %r11, %rdx\n" \
    "popq %r15\n" \
    "popq %r14\n" \
    "popq %r13\n" \
    "popq %r12\n" \
    "popq %rbx\n" \
    "popq %rbp\n" \
    /* Return to the caller */ \
    "ret\n" \
    ".size __ia2_deflate_slow, .-__ia2_deflate_slow\n" \
    ".previous\n" \
);
#define IA2_DEFINE_WRAPPER_deflate_stored \
asm(\
    /* Wrapper for deflate_stored(int, int) -> int: */ \
    ".text\n" \
    ".local __ia2_deflate_stored\n" \
    ".type __ia2_deflate_stored, @function\n" \
    "__ia2_deflate_stored:\n" \
    "pushq %rbp\n" \
    "movq %rsp, %rbp\n" \
    "pushq %rbx\n" \
    "pushq %r12\n" \
    "pushq %r13\n" \
    "pushq %r14\n" \
    "pushq %r15\n" \
    ASSERT_PKRU(0xfffffffffffffffc) "\n" \
    /* Set PKRU to the intermediate value to move arguments */ \
    "movq %rcx, %r10\n" \
    "movq %rdx, %r11\n" \
    "xorl %ecx, %ecx\n" \
    "xorl %edx, %edx\n" \
    "movl $0xffffffcc, %eax\n" \
    "wrpkru\n" \
    "movq %r10, %rcx\n" \
    "movq %r11, %rdx\n" \
    /* Compute location to save old stack pointer (using r11) */ \
    "mov ia2_stackptr_0@GOTTPOFF(%rip), %r11\n" \
    /* Write the old stack pointer to memory */ \
    "movq %rsp, %fs:(%r11)\n" \
    /* Compute location to load new stack pointer (using r11) */ \
    "mov ia2_stackptr_2@GOTTPOFF(%rip), %r11\n" \
    /* Read the new stack pointer from memory */ \
    "movq %fs:(%r11), %rsp\n" \
    "subq $8, %rsp\n" \
    /* Set PKRU to the compartment's value */ \
    "movq %rcx, %r10\n" \
    "movq %rdx, %r11\n" \
    "xorl %ecx, %ecx\n" \
    "xorl %edx, %edx\n" \
    "movl $0xffffffcc, %eax\n" \
    "wrpkru\n" \
    "movq %r10, %rcx\n" \
    "movq %r11, %rdx\n" \
    /* Call wrapped function */ \
    "call deflate_stored\n" \
    /* Set PKRU to the intermediate value to move arguments */ \
    "movq %rax, %r10\n" \
    "movq %rdx, %r11\n" \
    "xorl %ecx, %ecx\n" \
    "xorl %edx, %edx\n" \
    "movl $0xffffffcc, %eax\n" \
    "wrpkru\n" \
    "movq %r10, %rax\n" \
    "movq %r11, %rdx\n" \
    "addq $8, %rsp\n" \
    /* Compute location to save old stack pointer (using r11) */ \
    "mov ia2_stackptr_2@GOTTPOFF(%rip), %r11\n" \
    /* Write the old stack pointer to memory */ \
    "movq %rsp, %fs:(%r11)\n" \
    /* Compute location to load new stack pointer (using r11) */ \
    "mov ia2_stackptr_0@GOTTPOFF(%rip), %r11\n" \
    /* Read the new stack pointer from memory */ \
    "movq %fs:(%r11), %rsp\n" \
    /* Preserve essential regs on stack */ \
    "pushq %rax\n" \
    /* Scrub non-essential regs */ \
    "call __libia2_scrub_registers\n" \
    /* Restore preserved regs */ \
    "popq %rax\n" \
    /* Set PKRU to the caller's value */ \
    "movq %rax, %r10\n" \
    "movq %rdx, %r11\n" \
    "xorl %ecx, %ecx\n" \
    "xorl %edx, %edx\n" \
    "movl $0xfffffffc, %eax\n" \
    "wrpkru\n" \
    "movq %r10, %rax\n" \
    "movq %r11, %rdx\n" \
    "popq %r15\n" \
    "popq %r14\n" \
    "popq %r13\n" \
    "popq %r12\n" \
    "popq %rbx\n" \
    "popq %rbp\n" \
    /* Return to the caller */ \
    "ret\n" \
    ".size __ia2_deflate_stored, .-__ia2_deflate_stored\n" \
    ".previous\n" \
);
