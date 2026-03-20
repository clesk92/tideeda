# UHDM Rust Cap'n Proto 绑定框架 - 设计说明书

## 1. 项目概述

### 项目名称
`uhdm-rs` - UHDM (Universal Hardware Data Model) 的 Rust Cap'n Proto 绑定

### 核心目标
为 UHDM 实现一套"对象池 + 引用解引用 + 可控遍历"的 Rust 访问框架。

### 关键特性
- 基于 Cap'n Proto 序列化格式的高效读取
- 遵循 UHDM C++ 项目的遍历策略
- 支持回归测试系统
- 可扩展的遍历器设计

---

## 2. 架构设计

### 2.1 核心概念

#### UHDM 数据模型
```
uhdm_root
  ├─ designs: List(Design)           # 顶层入口
  ├─ symbols: List(Text)              # 字符串池
  ├─ factory_* : List(T)             # 对象池
  └─ ...
```

**关键理解**：
1. UHDM 不是"对象直接内嵌"的树结构
2. 对象存储在 `factory_*` 池中
3. 对象间引用通过 `ObjIndexType` 实现： `{ type: u32, index: u64 (1-based) }`
4. 解引用时 `index - 1` 转换为 0-based 下标

#### ObjIndexType 解引用流程
```
obj_index_type
  -> raw_type = get_type()        # UHDM_OBJECT_TYPE 数值
  -> index1 = get_index()         # 1-based object id
  -> index0 = index1 - 1          # 转换为 0-based
  -> raw_type 决定访问哪个 factory_*
  -> factory_*[index0] 取出真实对象
  -> 包装成统一 Node
```

### 2.2 核心组件

#### 文件结构
```
crates/uhdm-rs/src/
├── lib.rs              # 主模块
├── raw_types.rs        # 394 个 RAW_UHDM_* 常量定义
├── walker.rs            # 遍历器实现
└── regression_test.rs   # 回归测试系统
```

#### 组件关系图
```
┌─────────────────────────────────────────────────────┐
│                    UhdmContext                       │
│  ┌─────────────────────────────────────────────┐   │
│  │ symbol(id: u64) -> Option<String>           │   │
│  │ parse_obj_index(idx) -> Option<ObjRef>      │   │
│  │ resolve_obj_index(idx) -> Option<Node>       │   │
│  └─────────────────────────────────────────────┘   │
└─────────────────────────────────────────────────────┘
                          │
                          ▼
┌─────────────────────────────────────────────────────┐
│                      Node<'a>                        │
│  Operation | Constant | Port | ModuleInst | ...      │
│  (24 种支持的核心类型)                               │
└─────────────────────────────────────────────────────┘
                          │
                          ▼
┌─────────────────────────────────────────────────────┐
│                      Walker                           │
│  - visited 去重 (HashSet<ObjKey>)                    │
│  - depth 限制 (max_depth)                            │
│  - walk_designs() 入口                               │
└─────────────────────────────────────────────────────┘
```

---

## 3. 核心数据结构

### 3.1 ObjRef
```rust
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct ObjRef {
    pub raw_type: u32,  // UHDM_OBJECT_TYPE 数值
    pub index1: u64,     // 1-based object id
}
```

### 3.2 ObjKey (用于 visited 去重)
```rust
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct ObjKey {
    pub raw_type: u32,
    pub index1: u64,
}
```

### 3.3 Node<'a> (统一节点枚举)
```rust
pub enum Node<'a> {
    Operation(uhdm_capnp::operation::Reader<'a>),
    Constant(uhdm_capnp::constant::Reader<'a>),
    Port(uhdm_capnp::port::Reader<'a>),
    PortBit(uhdm_capnp::portbit::Reader<'a>),
    BitSelect(uhdm_capnp::bitselect::Reader<'a>),
    PartSelect(uhdm_capnp::partselect::Reader<'a>),
    IndexedPartSelect(uhdm_capnp::indexedpartselect::Reader<'a>),
    RefObj(uhdm_capnp::refobj::Reader<'a>),
    LogicTypespec(uhdm_capnp::logictypespec::Reader<'a>),
    BitTypespec(uhdm_capnp::bittypespec::Reader<'a>),
    EnumTypespec(uhdm_capnp::enumtypespec::Reader<'a>),
    StructTypespec(uhdm_capnp::structtypespec::Reader<'a>),
    PackedArrayTypespec(uhdm_capnp::packedarraytypespec::Reader<'a>),
    ModuleInst(uhdm_capnp::moduleinst::Reader<'a>),
    InterfaceInst(uhdm_capnp::interfaceinst::Reader<'a>),
    Package(uhdm_capnp::package::Reader<'a>),
    Program(uhdm_capnp::program::Reader<'a>),
    Design(uhdm_capnp::design::Reader<'a>),
    Parameter(uhdm_capnp::parameter::Reader<'a>),
    ContAssign(uhdm_capnp::contassign::Reader<'a>),
    Always(uhdm_capnp::always::Reader<'a>),
    LogicVar(uhdm_capnp::logicvar::Reader<'a>),
    Reg(uhdm_capnp::reg::Reader<'a>),
    Range(uhdm_capnp::range::Reader<'a>),
    VarSelect(uhdm_capnp::varselect::Reader<'a>),
}
```

### 3.4 RAW_UHDM_* 常量
- 来源：从 UHDM C++ 项目的 `uhdm_types.h` 复制
- 数量：394 个常量
- 起始值：2001
- 命名规则：`RAW_UHDM_{TYPE_NAME}`

---

## 4. UhdmContext 核心方法

### 4.1 symbol()
```rust
pub fn symbol(&self, id: u64) -> Option<String>
```
从 symbols 池解析字符串（id 是 0-based 索引）。

### 4.2 parse_obj_index()
```rust
pub fn parse_obj_index(&self, idx: ObjIndexType::Reader) -> Option<ObjRef>
```
解析 `ObjIndexType`，返回 `ObjRef`。

### 4.3 resolve_raw_type_index()
```rust
pub fn resolve_raw_type_index(&self, raw_type: u32, index0: u32) -> Option<Node<'a>>
```
核心解引用函数，直接复刻 C++ `Serializer::GetObject(type, index)`。

### 4.4 resolve_obj_index()
```rust
pub fn resolve_obj_index(&self, idx: ObjIndexType::Reader) -> Option<Node<'a>>
```
统一解引用入口，内部自动做 `index - 1` 转换。

---

## 5. Walker 遍历器

### 5.1 设计原则
1. **从 Design 入口**：语义遍历从 `root.get_designs()` 开始
2. **visited 去重**：UHDM 是图不是树，必须 visited
3. **depth 限制**：防止无限递归
4. **shallow 策略**：对部分类型不深遍历

### 5.2 Walker 结构
```rust
pub struct Walker {
    visited: HashSet<ObjKey>,   // 去重集合
    depth: usize,                 // 当前深度
    max_depth: usize,           // 最大深度限制
}
```

### 5.3 遍历流程
```
walk_designs()
  └─> visit_design()
        ├─> [visited 记录]
        ├─> 打印 Design 信息
        └─> 遍历子对象 (typespecs, parameters, taskfuncs, ...)
              └─> visit_obj_index()
                    ├─> [visited 检查]
                    ├─> parse_obj_index()
                    ├─> resolve_raw_type_index()
                    └─> match Node 类型调用具体 visitor
```

---

## 6. 回归测试系统

### 6.1 目录结构
```
tests/uhdm_regression/{test_name}/
├── {test_name}.uhdm              # UHDM 文件
├── src/{test_name}.sv            # 源码（可选）
├── golden/
│   ├── containers.golden.log     # 容器信息
│   ├── design.golden.log          # Design 信息
│   └── walker.golden.log         # Walker 遍历结果
└── actual/
    ├── containers.actual.log
    ├── design.actual.log
    └── walker.actual.log
```

### 6.2 测试类型
| 类型 | 生成函数 | 说明 |
|------|---------|------|
| containers | `format_containers()` | 所有 Factories 和 Symbols |
| design | `format_design()` | Design 结构化输出 |
| walker | `walk_design()` | Walker 遍历树 |

### 6.3 测试比较逻辑
- 去除空白行后逐行比较
- 三种输出全部匹配才通过

---

## 7. 实现状态

### 7.1 已完成
- [x] `raw_types.rs`: 394 个 RAW_UHDM_* 常量
- [x] `lib.rs`: ObjRef, Node, UhdmContext 核心框架
- [x] `walker.rs`: 基本遍历器
  - `walk_all()`: 扫描所有 factory 池
  - `walk_designs()`: 从 Design 入口遍历
  - 支持 ModuleInst、Port、Net、Constant、Operation、ContAssign、Parameter、RefObj、Always
- [x] `regression_test.rs`: 回归测试系统
- [x] Walker 集成到 regression test
- [x] 测试通过：counter.sv 示例完整遍历

### 7.2 进行中
- Walker 字段名称显示优化

### 7.3 待完成
- [ ] 完善 ContAssign lhs/rhs 名称解析
- [ ] 完善 Parameter 名称解析
- [ ] 完善 Net 名称解析
- [ ] 添加更多测试用例
- [ ] 生成更多 Node 类型支持

---

## 8. Cap'n Proto Schema 来源

### 8.1 Schema 文件位置
- **uhdm-rs**: `/Users/hh/Documents/TideEDA/crates/uhdm-rs/schema/uhdm.capnp`
- **UHDM 源码**: `/Users/hh/Documents/TideEDA/UHDM/src/uhdm.capnp`
- **Surelog 编译输出**: `/Users/hh/Desktop/eda/Surelog/build/third_party/UHDM/generated/src/UHDM.capnp`

### 8.2 Schema 生成流程
```
YAML 模型定义 (UHDM/model/*.yaml)
         ↓
    scripts/capnp.py
         ↓
Cap'n Proto Schema (src/uhdm.capnp)
         ↓
    capnp 编译器
         ↓
uhdm_capnp.rs (Rust 代码)
```

### 8.3 数据源
Schema 文件来源于 **Surelog 编译过程中 UHDM 子模块的输出**：
- 路径: `/Users/hh/Desktop/eda/Surelog/build/third_party/UHDM/generated/src/UHDM.capnp`
- 版本: UHDM v1.84 (与 Surelog v1.84 配套)
- MD5: `7d028968940b107747fa6ba0d992e184`

### 8.4 版本匹配
| 组件 | 版本 |
|------|------|
| Surelog | v1.84 |
| UHDM | v1.84 |
| uhdm.capnp | MD5: 7d028968940b107747fa6ba0d992e184 |
| uhdm-rs | v0.1.0 |

---

## 9. 参考资料

1. **UHDM 项目**: `/Users/hh/Documents/TideEDA/UHDM/`
2. **遍历策略文档**: `/Users/hh/Documents/TideEDA/doc/uhdm_walk.txt`
3. **Cap'n Proto Schema**: `/Users/hh/Documents/TideEDA/crates/uhdm-rs/schema/uhdm.capnp`
4. **Surelog 项目**: `/Users/hh/Desktop/eda/Surelog/`

---

## 10. 更新记录

| 日期 | 版本 | 更新内容 |
|------|------|---------|
| 2026-03-19 | v0.1.0 | 初始版本，实现核心框架和 Walker |
| 2026-03-20 | v0.1.1 | 补充 Cap'n Proto Schema 来源说明，版本匹配表 |
