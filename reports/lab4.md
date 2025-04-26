# report

## 编程作业

写一会儿 写起来还挺快的，就是遇见两个bug：spawn + inodeid

## 问答作业

- 在我们的easy-fs中，root inode起着什么作用？如果root inode中的内容损坏了，会发生什么？
    - / inode，里面存储了根目录的信息。
    - 有可能某些目录再也找不到。
