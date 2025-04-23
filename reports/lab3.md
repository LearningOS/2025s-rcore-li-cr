# 报告

## 编程作业

主要参考了 fork 和 exec 的代码部分。

## 问答作业

- 实际情况是轮到 p1 执行吗？为什么？
    - 整数溢出
- 为什么？尝试简单说明（不要求严格证明）。
    - 假设 TRIDE_MAX – STRIDE_MIN > BigStride / 2
        - 则 不存在任意上一状态 + pass 后 等于现在状态。
        - 要达到这一状态 必须 本次最大的stride【taskA】 在上一次执行，但是在上一状态中，这个【taskA】stride不是最小的。
- 已知以上结论，考虑溢出的情况下，可....
    - ```rust
        use core::cmp::Ordering;

        struct Stride(u64);

        impl PartialOrd for Stride {
            fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
                return (self.0 as i64).cmp(other.0 as i64)
            }
        }

        impl PartialEq for Stride {
            fn eq(&self, other: &Self) -> bool {
                self.0 == other.0
            }
        }
        TIPS
        