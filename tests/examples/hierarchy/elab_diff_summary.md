# UHDM Elab 前后差异对比

## 文件信息
- **Pre-Elab**: `hierarchy_pre_elab.uhdm` (1509 行)
- **Post-Elab**: `hierarchy_post_elab.uhdm` (1829 行)
- **差异**: +320 行

## 主要差异

### 1. Design 级别
```diff
+ |vpiElaborated:1
```
Post-Elab 添加了 `vpiElaborated:1` 标记，表示已完成 elaboration。

### 2. Port 级别
#### Pre-Elab
- Port 的 `vpiActual` 指向简单的 logic_net
- 没有 `vpiInstance` 属性

#### Post-Elab
- Port 添加了 `vpiInstance` 属性，指向所属的 module_inst
- `vpiActual` 指向的 logic_net 带有完整路径（如 `work@hierarchy_top.u_sub.clk`）

```diff
+    |vpiInstance:
+    \_module_inst: work@hierarchy_top (work@hierarchy_top)
```

### 3. Module Instance 级别
#### Pre-Elab
- 子模块实例（u_sub）的 port 连接简单

#### Post-Elab
- 子模块实例的 port 有完整的 `vpiHighConn` 和 `vpiLowConn`
- 添加了 `vpiInstance` 指向实例化的 module
- logic_net 名称包含完整层级路径

```diff
-        \_logic_net: (work@hierarchy_top.clk)
+        \_logic_net: (work@hierarchy_top.u_sub.clk)
```

### 4. Always 块
#### Pre-Elab
- Always 块没有 `vpiParent`

#### Post-Elab
- Always 块添加了 `vpiParent`，指向所属的 module

```diff
+      |vpiParent:
+      \_module_inst: work@sub_module (work@hierarchy_top.u_sub)
```

### 5. Net 级别
#### Post-Elab 新增
- `internal_data` net 在 Post-Elab 中明确创建
- 所有 nets 都有完整的层级路径

## 对 Walker 的影响

### 当前 Walker 使用 Post-Elab 的优势
1. ✅ 可以识别 `vpiElaborated` 标记
2. ✅ 可以通过 `vpiParent` 确定 always 块所属 module
3. ✅ 可以通过 `vpiInstance` 确定 port 所属 module
4. ✅ 有完整的层级路径信息

### 需要改进的地方
1. Port 的 parent 可能是 `ref_module` 而不是 `module_inst`
2. 需要处理层级路径（如 `work@hierarchy_top.u_sub.clk`）
3. 需要区分 module 定义和 module 实例

## 建议
使用 Post-Elab UHDM 可以获得更完整的层级信息，但需要 walker 正确处理：
- `vpiParent` 和 `vpiInstance` 引用
- 层级化的 full_name
- ref_module 和 module_inst 的区别
