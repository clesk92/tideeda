# TideEDA

TideEDA 是一个基于 Rust 的硬件设计处理工具链，用于解析、分析和转换 SystemVerilog 设计。

## 项目架构

```
TideEDA/
├── Cargo.toml              # Workspace 配置
├── crates/
│   ├── uhdm-rs/            # UHDM 解析核心库
│   │   ├── src/
│   │   │   ├── lib.rs              # 核心类型和 UhdmDesign
│   │   │   ├── walker.rs           # UHDM 遍历器（Tree/Flat 输出）
│   │   │   ├── raw_types.rs        # UHDM 原始类型定义
│   │   │   └── regression_test.rs  # 回归测试框架
│   │   ├── doc/
│   │   │   ├── spec.md             # 项目规格说明
│   │   │   └── UHDM_API.md         # UHDM API 文档
│   │   ├── schema/                 # 分割的 UHDM schema 文件（参考用）
│   │   └── reference/              # 原始 uhdm_capnp.rs（参考用）
│   └── cli/                # 命令行工具
│       └── src/main.rs     # tideeda 命令行入口
├── tests/
│   └── examples/           # 回归测试示例（从简单到复杂）
│       ├── counter/        # 基础计数器
│       ├── hierarchy/      # 层级模块实例化
│       └── ibex_csr/       # Ibex CSR 寄存器（叶子模块）
└── reference/              # 外部参考数据（不纳入版本控制）
    ├── ibex/               # Ibex RISC-V 处理器源码
    └── UHDM/               # UHDM 项目源码
```

## 当前阶段：UHDM 解析与遍历

### 已完成

1. **UHDM 文件解析**
   - 使用 Cap'n Proto 读取 Surelog 生成的 UHDM 文件
   - 支持 Designs, ModuleInsts, Ports, Nets, Always, ContAssigns 等类型

2. **Walker 遍历器**
   - `walk_all()` - 扁平输出所有容器信息
   - `walk_design_tree()` - 树形层次结构输出
   - 支持 Module, Port, Net, Always, ContAssign 等语法元素

3. **回归测试系统**
   - 自动发现测试用例
   - 对比 walker/tree 输出与 golden 文件
   - 支持从叶子模块开始渐进式测试

### 测试用例（从简单到复杂）

| 示例 | 描述 | 状态 |
|------|------|------|
| counter | 基础计数器（always, if-else） | ✅ |
| hierarchy | 模块实例化和端口连接 | ✅ |
| ibex_csr | Ibex CSR 寄存器（参数、生成块） | ✅ |

### 支持的 SystemVerilog 语法

- ✅ Module 声明和实例化
- ✅ Ports（input/output/wire）
- ✅ Parameters
- ✅ Assign 语句
- ✅ Always_ff 块
- ✅ If-else 语句
- ✅ Generate if（条件编译）

## 使用方法

### 编译项目

```bash
cargo build --release
```

### 运行回归测试

```bash
cd crates/uhdm-rs
cargo test test_regression
```

### 命令行工具

```bash
# 查看设计摘要
cargo run --bin tideeda <uhdm_file>

# 查看符号表
cargo run --bin tideeda <uhdm_file> symbols

# 查看模块列表
cargo run --bin tideeda <uhdm_file> modules

# 查看工厂计数
cargo run --bin tideeda <uhdm_file> factories
```

## 下一阶段规划

1. **扩展 Ibex 测试覆盖**
   - 逐个添加 Ibex 叶子模块（ibex_counter, ibex_alu, ibex_register_file_ff 等）
   - 逐步支持更复杂的语法（case, for, function 等）

2. **IR 生成**
   - 从 UHDM 生成统一的中间表示（IR）
   - 支持控制流图（CFG）和数据流分析

3. **仿真支持**
   - IR 解释器
   - VCD 波形输出

## 开发指南

### 添加新的测试用例

1. 在 `tests/examples/` 创建目录
2. 添加 SV 源文件到 `src/`
3. 使用 Surelog 编译生成 UHDM：
   ```bash
   surelog -parse -sverilog -elabuhdm <file.sv>
   ```
4. 复制 UHDM 文件到测试目录
5. 运行测试自动生成 golden 文件

### 参考数据

- `reference/ibex/` - Ibex 项目完整源码，用于提取测试用例
- `reference/UHDM/` - UHDM 项目源码，用于理解数据模型

## 依赖

- Rust 1.70+
- Surelog 1.84+（用于生成 UHDM 文件）
- Cap'n Proto（UHDM 序列化格式）

## 许可证

MIT License - 详见 [LICENSE](LICENSE) 文件。

## 致谢

- [Surelog](https://github.com/chipsalliance/Surelog) - SystemVerilog 预处理器和解析器
- [UHDM](https://github.com/chipsalliance/UHDM) - 通用硬件设计元模型
- [Cap'n Proto](https://capnproto.org/) - 数据交换格式
