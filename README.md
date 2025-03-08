# 区块链项目方案

---

## 1. 项目目标

### 1.1 核心目标
- **完成一个简单的区块链系统**，能够生成创世区块并通过程序逻辑添加新区块。
- **输出区块链信息**，包括区块哈希值、前一个区块的哈希值、时间戳等。
- **区块添加逻辑**：通过程序内部逻辑自动添加区块，区块数据为简单的字符串。

### 1.2 简化功能
- **聚焦核心功能**：确保区块链的基本数据结构（区块、区块链）和核心逻辑（创世区块生成、区块添加）能够正常运行。

---

## 2. 产品方案

### 2.1 功能需求
1. **创世区块生成**：
   - 系统启动时，自动生成创世区块。
2. **程序添加新区块**：
   - 通过程序逻辑自动添加新区块，区块数据为简单的字符串。
3. **区块链信息展示**：
   - 程序运行结束后，输出当前区块链的所有区块信息（哈希值、前一个区块的哈希值、时间戳等）。

### 2.2 非功能需求
1. **简单易用**：
   - 程序运行后自动完成区块添加和区块链信息展示。
2. **代码可读性**：
   - 代码结构清晰，注释完整，便于后续扩展。

---

## 3. 技术方案

### 3.1 技术栈
- **编程语言**：Rust
- **工具库**：
  - `bincode`：用于序列化和反序列化。
  - `serde`：支持`bincode`的序列化。
  - `crypto`：用于哈希计算。
  - `chrono`：用于生成时间戳。

### 3.2 系统架构
1. **区块结构**：

   - ##### 区块头:

     ​			`time`: 时间戳

     ​			`tx_hash`: data的哈希

     ​			`pre_hash`: 前一个区块的哈希值

   - `hash`: 当前区块的哈希值

   - `data`: 区块数据（简单字符串）

2. **区块链结构**：

   - `BlockChain`: 区块的链表
   - `add_block`: 添加新区块的方法
   - `new_genesis_block`: 新建创世区块的方法
   - `new_blockchain`: 新建区块链的方法

### 3.3 核心逻辑
1. **创世区块生成**：
   - 系统启动时，生成一个创世区块，`pre_hash`为空，`data`为“This is genesis block”。
2. **新区块添加**：
   - 通过程序逻辑自动添加新区块，区块数据为简单的字符串（如“This is bolck 1”）。
3. **区块链展示**：
   - 程序运行结束后，遍历区块链，输出每个区块的详细信息。

### 3.4 代码结构
```plaintext
src/
├── main.rs              # 主程序入口，区块添加逻辑
├── block.rs             # 区块结构定义
├── blockchain.rs        # 区块链结构定义
└── coder.rs             # 工具函数（序列化和反序列化，求哈希）
```

### 3.5 程序逻辑示例
```rust
use chrono::prelude::*;
use utils::coder;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, PartialEq)]
pub struct BlockHeader {
    pub time: i64,
    pub tx_hash: String, 
    pub pre_hash: String,
}

#[derive(Debug)]
pub struct Block {
    pub header: BlockHeader,
    pub hash: String,
    pub data: String, 
}

pub struct BlockChain {
    pub blocks: Vec<block::Block>,
}

impl BlockChain {
    pub  fn add_block(&mut self, data: String) {
        let pre_block = &self.blocks[self.blocks.len()-1];
        let new_block = block::Block::new_block(data, pre_block.hash.clone());
        self.blocks.push(new_block);
    }

    fn new_genesis_block() -> block::Block {
        block::Block::new_block("This is genesis block".to_string(), String::from(""))
    } 

    pub fn new_blockchain() -> BlockChain {
        BlockChain {
            blocks: vec![BlockChain::new_genesis_block()],
        }
    }
}
```

---

## 4. 开发计划

### 4.1 时间安排
- **第1天**：完善区块和区块链的现有代码，确保创世区块生成和区块添加功能正常。
- **第2天**：实现程序内部的区块添加逻辑，支持自动添加多个区块。
- **第3天**：编写单元测试，确保区块和区块链的核心逻辑正确。
- **第4天**：优化代码结构，增加注释，提高代码可读性。
- **第5天**：撰写项目文档，准备演示。

### 4.2 风险与应对
1. **时间不足**：
   - **应对措施**：优先实现核心功能（创世区块生成、区块添加、区块链展示），其他功能（如Merkle哈希计算）可以简化或延后。
2. **代码质量问题**：
   - **应对措施**：编写单元测试，确保核心逻辑正确；代码结构尽量简洁清晰。

---

## 5. 项目文档

### 5.1 使用说明
1. **克隆项目**：
   ```bash
   git clone https://github.com/littileye/blockchain_rust.git
   cd blockchain_rust
   ```
2. **运行项目**：
   
   ```bash
   cargo run
   ```

### 5.2 代码示例
#### 区块结构
```rust
#[derive(Serialize, Deserialize, Debug, PartialEq)]
pub struct BlockHeader {
    pub time: i64,
    pub tx_hash: String,
    pub pre_hash: String,
}

#[derive(Debug)]
pub struct Block {
    pub header: BlockHeader,
    pub hash: String,
    pub data: String,
}
```

#### 区块链结构
```rust
pub struct BlockChain {
    pub blocks: Vec<block::Block>,
}

impl BlockChain {
    pub  fn add_block(&mut self, data: String) {
        let pre_block = &self.blocks[self.blocks.len()-1];
        let new_block = block::Block::new_block(data, pre_block.hash.clone());
        self.blocks.push(new_block);
    }

    fn new_genesis_block() -> block::Block {
        block::Block::new_block("This is genesis block".to_string(), String::from(""))
    } 

    pub fn new_blockchain() -> BlockChain {
        BlockChain {
            blocks: vec![BlockChain::new_genesis_block()],
        }
    }
}
```

---

## 6. 后续扩展
如果时间允许，可以在现有基础上逐步添加以下功能：
1. **工作量证明（PoW）**：实现简单的挖矿功能。
2. **交易模拟**：支持模拟交易并打包到区块中。
3. **网络通信**：实现简单的P2P网络通信，支持多节点同步区块链。
