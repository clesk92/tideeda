# UHDM Cap'n Proto 访问接口文档

## 概述

UHDM (Unified Hardware Data Model) 使用 Cap'n Proto 作为序列化格式。本文档记录 uhdm-rs 中 UHDM 对象的访问接口，基于 `uhdm_capnp.rs` 生成的代码。

## 核心概念

### 1. ObjRef 结构

```rust
pub struct ObjRef {
    pub raw_type: u32,  // RAW_UHDM_* 类型常量
    pub index1: u64,   // 1-based 索引 (pool 中的位置)
}
```

### 2. obj_index_type

从 UHDM 获取的对象引用是 `obj_index_type::Reader`，包含：

```rust
pub fn get_index(&self) -> u64    // 返回 u64 - 对象在 pool 中的 1-based 索引
pub fn get_type(&self) -> u32     // 返回 u32 - RAW_UHDM_* 类型常量
```

### 3. parse_obj_index 转换

将 `obj_index_type::Reader` 转换为 `ObjRef`：

```rust
fn parse_obj_index(&self, index: obj_index_type::Reader) -> Option<ObjRef> {
    Some(ObjRef {
        raw_type: index.get_type(),
        index1: index.get_index(),
    })
}
```

## 常用 API

### 符号表 (Symbol Table)

```rust
ctx.symbol(symbol_id: u64) -> Option<String>
```

根据符号 ID 获取字符串。符号表存储了 UHDM 中所有的名称字符串。

### 对象池 (Factory Pools)

通过 `ctx.get_root()` 访问：

```rust
ctx.get_root().get_factory_contassign()  // 连续赋值
ctx.get_root().get_factory_always()      // always 块
ctx.get_root().get_factory_refobj()      // 对象引用
ctx.get_root().get_factory_operation()   // 操作符
ctx.get_root().get_factory_constant()    // 常量
ctx.get_root().get_factory_ifelse()      // if-else 语句
ctx.get_root().get_factory_ifstmt()      // if 语句
ctx.get_root().get_factory_begin()       // begin-end 块
ctx.get_root().get_factory_assignment()  // 赋值语句
ctx.get_root().get_factory_eventcontrol()// 事件控制 (@*)
```

每个 pool 的使用方式：

```rust
if let Ok(pool) = ctx.get_root().get_factory_contassign() {
    for i in 0..pool.len() {
        let obj = pool.get(i);  // 获取 Reader
        // 访问对象属性...
    }
}
```

## 关键对象结构

### 1. cont_assign (连续赋值)

```yaml
- obj_def: cont_assign
  - group_ref: lhs
    type: expr
    vpi: vpiLHS
  - group_ref: rhs
    type: expr
    vpi: vpiRHS
```

**访问方式：**

```rust
let ca = pool.get(index);
// LHS (左操作数)
if let Ok(lhs_ref) = ca.get_lhs() {
    if let Some(obj) = ctx.parse_obj_index(lhs_ref) {
        // obj 是表达式对象
    }
}
// RHS (右操作数)
if let Ok(rhs_ref) = ca.get_rhs() {
    if let Some(obj) = ctx.parse_obj_index(rhs_ref) {
        // obj 是表达式对象
    }
}
```

### 2. always (always 块)

```yaml
- obj_def: always
  - group_ref: base
    type: stmt
```

**访问方式：**

```rust
let a = pool.get(index);
if let Ok(proc) = a.get_base() {
    if let Ok(stmt_ref) = proc.get_stmt() {
        if let Some(stmt_obj) = ctx.parse_obj_index(stmt_ref) {
            // stmt_obj 通常是 event_control
        }
    }
}
```

### 3. event_control (敏感信号列表)

```yaml
- obj_def: event_control
  - extends: atomic_stmt
  - group_ref: vpiCondition
    type: expr_sequence_inst_named_event_group
    vpi: vpiCondition
    card: 1
  - group_ref: stmt
    type: stmt
    vpi: vpiStmt
    card: 1
```

**访问方式：**

```rust
let ec = pool.get(index);
// 敏感信号 (expr_sequence_inst_named_event_group)
// 返回 obj_index_type::Reader
if let Ok(cond_ref) = ec.get_vpi_condition() {
    if let Some(cond_obj) = ctx.parse_obj_index(cond_ref) {
        // cond_obj 是操作树，根节点是 "or" 操作 (vpiOpType:35)
        // 子节点是 posedge (vpiOpType:39) 或 negedge (vpiOpType:40)
    }
}
// 语句
if let Ok(stmt_ref) = ec.get_stmt() {
    if let Some(stmt_obj) = ctx.parse_obj_index(stmt_ref) {
        // stmt_obj 通常是 begin 块
    }
}
```

### 4. if_else (if-else 语句)

```yaml
- obj_def: if_else
  - group_ref: vpiCondition
    type: expr
    vpi: vpiCondition
  - group_ref: vpiStmt
    type: stmt
    vpi: vpiStmt
  - group_ref: vpiElseStmt
    type: stmt
    vpi: vpiElseStmt
```

**访问方式：**

```rust
let ifelse = pool.get(index);
// 条件
if let Ok(cond_ref) = ifelse.get_vpi_condition() {
    if let Some(cond_obj) = ctx.parse_obj_index(cond_ref) {
        // cond_obj 是表达式
    }
}
// then 分支
if let Ok(then_ref) = ifelse.get_vpi_stmt() {
    if let Some(then_obj) = ctx.parse_obj_index(then_ref) {
        // then_obj 是语句
    }
}
// else 分支
if let Ok(else_ref) = ifelse.get_vpi_else_stmt() {
    if let Some(else_obj) = ctx.parse_obj_index(else_ref) {
        // else_obj 是语句
    }
}
```

### 5. if_stmt (if 语句，无 else)

```yaml
- obj_def: if_stmt
  - group_ref: vpiCondition
    type: expr
    vpi: vpiCondition
  - group_ref: vpiStmt
    type: stmt
    vpi: vpiStmt
```

**访问方式：** 与 if_else 类似，但没有 `get_vpi_else_stmt()`。

### 6. operation (操作符)

```yaml
- obj_def: operation
  - property: vpiOpType
    type: uvm_logic_u
    vpi: vpiOpType
```

**访问方式：**

```rust
let op = pool.get(index);
let op_type = op.get_vpi_op_type();  // 返回 u32
// 操作数列表
if let Ok(operands) = op.get_operands() {
    for i in 0..operands.len() {
        let operand = operands.get(i);
        if let Some(operand_obj) = ctx.parse_obj_index(operand) {
            // operand_obj 是操作数
        }
    }
}
```

### 7. constant (常量)

```yaml
- obj_def: constant
  - group_ref: base
    type: expr
```

**访问方式：**

```rust
let c = pool.get(index);
if let Ok(expr_reader) = c.get_base() {
    // 获取值的符号表 ID
    let decompile_id = expr_reader.get_vpi_decompile();
    if decompile_id > 0 {
        if let Some(s) = ctx.symbol(decompile_id) {
            // s 是常量值的字符串表示，如 "1'b1", "8'd42"
        }
    }
}
```

### 8. assignment (赋值语句)

```yaml
- obj_def: assignment
  - property: vpiOpType
    type: uvm_logic_u
    vpi: vpiOpType
  - group_ref: lhs
    type: expr
    vpi: vpiLHS
  - group_ref: rhs
    type: expr
    vpi: vpiRHS
```

**访问方式：**

```rust
let assign = pool.get(index);
// 判断是否非阻塞赋值
let op_type = assign.get_vpi_op_type();
let is_non_blocking = (op_type == 82);  // 82 = vpiNonBlockingAssign
// LHS
if let Ok(lhs_ref) = assign.get_lhs() {
    if let Some(lhs_obj) = ctx.parse_obj_index(lhs_ref) {
        // lhs_obj 是左操作数
    }
}
// RHS
if let Ok(rhs_ref) = assign.get_rhs() {
    if let Some(rhs_obj) = ctx.parse_obj_index(rhs_ref) {
        // rhs_obj 是右操作数
    }
}
```

### 9. begin (begin-end 块)

```yaml
- obj_def: begin
  - group_ref: stmts
    type: stmt
    vpi: vpiStmt
    card: infinite
```

**访问方式：**

```rust
let begin = pool.get(index);
if let Ok(stmts) = begin.get_stmts() {
    for i in 0..stmts.len() {
        let s = stmts.get(i);
        if let Some(stmt_obj) = ctx.parse_obj_index(s) {
            // stmt_obj 是语句
        }
    }
}
```

## VPI 操作符类型 (vpiOpType)

| 值 | 名称 | 说明 |
|----|------|------|
| 1 | vpiMinusOp | 一元负号 |
| 2 | vpiPlusOp | 一元正号 |
| 3 | vpiNotOp | 逻辑非 (!) |
| 4 | vpiBitNegOp | 位反 (~) |
| 5 | vpiUnaryAndOp | 归约与 (&) |
| 6 | vpiUnaryNandOp | 归约与非 (~&) |
| 7 | vpiUnaryOrOp | 归约或 (\|) |
| 8 | vpiUnaryNorOp | 归约或非 (~\|) |
| 9 | vpiUnaryXorOp | 归约异或 (^) |
| 10 | vpiUnaryXNorOp | 归约异或非 (~^) |
| 11 | vpiSubOp | 二元减 (-) |
| 12 | vpiDivOp | 除 (/) |
| 13 | vpiModOp | 模 (%) |
| 14 | vpiEqOp | 等于 (==) |
| 15 | vpiNeqOp | 不等于 (!=) |
| 16 | vpiCaseEqOp | 案例等于 (===) |
| 17 | vpiCaseNeqOp | 案例不等于 (!==) |
| 18 | vpiGtOp | 大于 (>) |
| 19 | vpiGeOp | 大于等于 (>=) |
| 20 | vpiLtOp | 小于 (<) |
| 21 | vpiLeOp | 小于等于 (<=) |
| 22 | vpiLShiftOp | 左移 (<<) |
| 23 | vpiRShiftOp | 右移 (>>) |
| 24 | vpiAddOp | 二元加 (+) |
| 25 | vpiMultOp | 乘 (*) |
| 26 | vpiLogAndOp | 逻辑与 (&&) |
| 27 | vpiLogOrOp | 逻辑或 (\|\|) |
| 28 | vpiBitAndOp | 位与 (&) |
| 29 | vpiBitOrOp | 位或 (\|) |
| 30 | vpiBitXorOp | 位异或 (^) |
| 31 | vpiBitXNorOp | 位异或非 (~^) |
| 32 | vpiConditionOp | 条件运算符 (?:) |
| 33 | vpiConcatOp | 连接符 ({...}) |
| 34 | vpiMultiConcatOp | 多重连接 ({n{}}) |
| 35 | vpiEventOrOp | 事件或 (or) - 用于敏感信号列表 |
| 39 | (扩展) | posedge |
| 40 | (扩展) | negedge |
| 82 | vpiNonBlockingAssign | 非阻塞赋值 (<=) |

## RAW_UHDM 类型常量

定义在 `raw_types.rs`：

```rust
RAW_UHDM_DESIGN = 1
RAW_UHDM_MODULE = 2
RAW_UHDM_PORT = 3
// ... 等等
```

## 表达式解析示例

解析表达式树并生成字符串表示：

```rust
fn resolve_expression(&self, ctx: &UhdmContext, obj_ref: ObjRef) -> String {
    match obj_ref.raw_type {
        RAW_UHDM_REF_OBJ => {
            // 获取变量名
            let pool = ctx.get_root().get_factory_refobj().unwrap();
            let ref_obj = pool.get((obj_ref.index1 - 1) as u32);
            let name_id = ref_obj.get_vpi_name();
            ctx.symbol(name_id).unwrap_or("?".to_string())
        }
        RAW_UHDM_OPERATION => {
            // 获取操作符和操作数
            let pool = ctx.get_root().get_factory_operation().unwrap();
            let op = pool.get((obj_ref.index1 - 1) as u32);
            self.format_operation(ctx, op)
        }
        RAW_UHDM_CONSTANT => {
            // 获取常量值
            let pool = ctx.get_root().get_factory_constant().unwrap();
            let c = pool.get((obj_ref.index1 - 1) as u32);
            let expr = c.get_base().unwrap();
            let decompile_id = expr.get_vpi_decompile();
            ctx.symbol(decompile_id).unwrap_or("?".to_string())
        }
        // ...
    }
}

fn format_operation(&self, ctx: &UhdmContext, op: operation::Reader) -> String {
    let op_type = op.get_vpi_op_type();
    let op_name = match op_type {
        24 => "+",   // vpiAddOp
        11 => "-",   // vpiSubOp
        25 => "*",   // vpiMultOp
        5 => "&",    // vpiUnaryAndOp (reduction)
        35 => "or",  // vpiEventOrOp
        39 => "posedge",
        40 => "negedge",
        // ...
    };

    if let Ok(operands) = op.get_operands() {
        if operands.len() == 1 {
            // 一元操作
            let operand = operands.get(0);
            if let Some(obj) = ctx.parse_obj_index(operand) {
                return format!("{}{}", op_name, self.resolve_expression(ctx, obj));
            }
        } else if operands.len() == 2 {
            // 二元操作
            let left = operands.get(0);
            let right = operands.get(1);
            if let (Some(l), Some(r)) = (ctx.parse_obj_index(left), ctx.parse_obj_index(right)) {
                return format!("{} {} {}",
                    self.resolve_expression(ctx, l),
                    op_name,
                    self.resolve_expression(ctx, r)
                );
            }
        }
    }
    "?".to_string()
}
```

## 调试技巧

### 1. 查看对象池大小

```rust
if let Ok(pool) = ctx.get_root().get_factory_contassign() {
    eprintln!("cont_assign pool size: {}", pool.len());
}
```

### 2. 查看对象类型

```rust
fn type_name(raw_type: u32) -> &'static str {
    match raw_type {
        RAW_UHDM_DESIGN => "design",
        RAW_UHDM_ALWAYS => "always",
        RAW_UHDM_EVENT_CONTROL => "event_control",
        RAW_UHDM_IF_ELSE => "if_else",
        RAW_UHDM_ASSIGNMENT => "assignment",
        // ...
    }
}
```

### 3. 使用 uhdm-dump 查看原始数据

```bash
uhdm-dump design.uhdm 2>&1 | grep -B2 -A30 "event_control.*line:19"
```
