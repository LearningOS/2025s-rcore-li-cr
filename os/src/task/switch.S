.altmacro
.macro SAVE_SN n
    sd s\n, (\n+2)*8(a0)
.endm
.macro LOAD_SN n
    ld s\n, (\n+2)*8(a1)
.endm
    .section .text
    .globl __switch
    .global _sum_time
_sum_time:
    .quad 0
__switch:
    # 读取 time 到 a1
    csrr a2, time

    # __switch(
    #     current_task_cx_ptr: *mut TaskContext,
    #     next_task_cx_ptr: *const TaskContext
    # )
    # save kernel stack of current task
    sd sp, 8(a0)
    # save ra & s0~s11 of current execution
    sd ra, 0(a0)
    .set n, 0
    .rept 12
        SAVE_SN %n
        .set n, n + 1
    .endr
    # restore ra & s0~s11 of next execution
    ld ra, 0(a1)
    .set n, 0
    .rept 12
        LOAD_SN %n
        .set n, n + 1
    .endr
    # restore kernel stack of next task
    ld sp, 8(a1)

    csrr a3, time
    sub a3, a3, a2          # a3 = a3 - a2 = elapsed time
    la t0, _sum_time        # t0 = &_sum_time
    ld t1, 0(t0)            # t1 = *_sum_time
    add t1, t1, a3          # t1 += elapsed
    sd t1, 0(t0)            # *_sum_time = t1
    ret

