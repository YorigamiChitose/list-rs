# List-rs

`List-rs` 是一个用 Rust 编写的简单的单向链表实现。它提供了基本的功能，如插入、删除、遍历链表等。

## 功能

- 添加元素到链表头部 (`push`)
- 从链表头部弹出元素 (`pop`)
- 获取链表头部的元素引用 (`peek`)
- 修改链表头部的元素 (`peek_mut`)
- 在指定索引插入元素 (`insert`)
- 删除指定索引的元素 (`remove`)
- 支持迭代器 (`iter`, `iter_mut`, `into_iter`)

## 示例

### 创建链表并插入元素

```rust
use list_rs::list::List;

let mut list = List::new();
list.push(1);
list.push(2);
list.push(3);
```

### 弹出元素

```rust
assert_eq!(list.pop(), Some(3));
assert_eq!(list.pop(), Some(2));
```

### 查看链表头部元素

```rust
assert_eq!(list.peek(), Some(&1));
```

### 修改链表头部元素

```rust
list.peek_mut().map(|value| *value = 4);
assert_eq!(list.peek(), Some(&4));
```

### 插入和删除元素

```rust
list.insert(1, 5);  // 在索引 1 插入元素 5
assert_eq!(list.remove(1), Ok(()));  // 删除索引 1 的元素
```

### 使用迭代器

```rust
let mut iter = list.iter();
assert_eq!(iter.next(), Some(&4));
assert_eq!(iter.next(), Some(&1));
```

## 如何运行测试

项目包含单元测试，可以使用以下命令运行测试：

```bash
cargo test
```

## 贡献

欢迎提交 Issue 或 Pull Request 来改进 `List-rs`。

## 许可证

本项目采用 MIT 许可证。详细信息请参阅 [LICENSE](LICENSE) 文件。