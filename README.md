## desk

🔥一个由<u>rust</u>编写的ai cli工具🚀

[我的Github](https://github.com/linjingpy)

## 运行
```bash
cargo run
```

## 测试
```rust
use::std::stdin;

fn main (){

}
```

| 命令 | 说明 |
| --- | --- |
| cargo run | 运行 |
| cargo build | 编译 |
----------

## 功能完成
- [ ] 基础对话功能
- [ ] Agent功能
- [ ] cli功能
- [ ] 网络请求

> 这是一级引用
>> 这是二级引用

这是一个带脚注的文本[^1]。

[^1]: 这是脚注的内容。

<mark>高亮显示</mark>

```mermaid
graph TD
    A[启动程序] --> B[解析命令行参数]
        B --> C{参数验证}
            C -->|无效| D[显示帮助信息]
                C -->|有效| E[执行核心逻辑]
                    E --> F{执行结果}
                        F -->|成功| G[输出成功信息]
                            F -->|失败| H[输出错误信息]
                                D --> I[退出程序]
                                    G --> I
                                        H --> I
```

```mermaid
sequenceDiagram
    Alice->>John: 你好
    John-->>Alice: 你好吗？
```
