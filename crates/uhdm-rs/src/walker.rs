use std::collections::HashSet;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct ObjKey {
    pub raw_type: u32,
    pub index1: u64,
}

pub struct Walker {
    visited: HashSet<ObjKey>,
    depth: usize,
    max_depth: usize,
}

impl Walker {
    pub fn new() -> Self {
        Self {
            visited: HashSet::new(),
            depth: 0,
            max_depth: 100,
        }
    }

    pub fn with_max_depth(max_depth: usize) -> Self {
        Self {
            visited: HashSet::new(),
            depth: 0,
            max_depth,
        }
    }

    pub fn reset(&mut self) {
        self.visited.clear();
        self.depth = 0;
    }

    fn mark_visited(&mut self, key: ObjKey) {
        self.visited.insert(key);
    }

    fn indent(&self) -> String {
        "  ".repeat(self.depth)
    }

    fn line(&self, output: &mut String, s: &str) {
        *output += &format!("{}{}\n", self.indent(), s);
    }

    fn resolve_name(&self, ctx: &super::UhdmContext, name_id: u64) -> String {
        if name_id > 0 {
            ctx.symbol(name_id).unwrap_or_else(|| "?".to_string())
        } else {
            "?".to_string()
        }
    }

    fn get_object_full_name(&self, ctx: &super::UhdmContext, obj_ref: super::ObjRef) -> String {
        use super::raw_types::*;

        let index0 = (obj_ref.index1 - 1) as u32;

        match obj_ref.raw_type {
            RAW_UHDM_MODULE_INST => {
                if let Ok(pool) = ctx.get_root().get_factory_moduleinst() {
                    if index0 < pool.len() {
                        let module = pool.get(index0);
                        if let Ok(base) = module.get_base() {
                            if let Ok(inst_base) = base.get_base() {
                                let full_name_id = inst_base.get_vpi_full_name();
                                return ctx.symbol(full_name_id).unwrap_or_else(|| "?".to_string());
                            }
                        }
                    }
                }
                "?".to_string()
            }
            _ => "?".to_string(),
        }
    }

    fn resolve_expression(&self, ctx: &super::UhdmContext, obj_ref: super::ObjRef) -> String {
        use super::raw_types::*;

        let index0 = (obj_ref.index1 - 1) as u32;

        match obj_ref.raw_type {
            RAW_UHDM_REF_OBJ => {
                if let Ok(pool) = ctx.get_root().get_factory_refobj() {
                    if index0 < pool.len() {
                        let ref_obj = pool.get(index0);
                        let name_id = ref_obj.get_vpi_name();
                        if name_id > 0 {
                            return ctx.symbol(name_id).unwrap_or_else(|| "?".to_string());
                        }
                    }
                }
                "?".to_string()
            }
            RAW_UHDM_REF_MODULE => {
                if let Ok(pool) = ctx.get_root().get_factory_refmodule() {
                    if index0 < pool.len() {
                        let ref_module = pool.get(index0);
                        let name_id = ref_module.get_vpi_name();
                        if name_id > 0 {
                            return ctx.symbol(name_id).unwrap_or_else(|| "?".to_string());
                        }
                    }
                }
                "?".to_string()
            }
            RAW_UHDM_REF_MODULES => {
                if let Ok(pool) = ctx.get_root().get_factory_refmodule() {
                    if index0 < pool.len() {
                        let ref_module = pool.get(index0);
                        if let Ok(actualgroup) = ref_module.get_actualgroup() {
                            if let Some(final_ref) = ctx.parse_obj_index(actualgroup) {
                                return self.resolve_expression(ctx, final_ref);
                            }
                        }
                    }
                }
                "?".to_string()
            }
            RAW_UHDM_OPERATION => {
                if let Ok(pool) = ctx.get_root().get_factory_operation() {
                    if index0 < pool.len() {
                        let op = pool.get(index0);
                        return self.format_operation(ctx, op);
                    }
                }
                "?".to_string()
            }
            RAW_UHDM_CONSTANT => {
                if let Ok(pool) = ctx.get_root().get_factory_constant() {
                    if index0 < pool.len() {
                        let c = pool.get(index0);
                        if let Ok(expr_reader) = c.get_base() {
                            let decompile_id = expr_reader.get_vpi_decompile();
                            if decompile_id > 0 {
                                if let Some(s) = ctx.symbol(decompile_id) {
                                    return s;
                                }
                            }
                        }
                    }
                }
                "?".to_string()
            }
            _ => {
                raw_type_name(obj_ref.raw_type).to_string()
            }
        }
    }

    fn format_operation(&self, ctx: &super::UhdmContext, op: super::uhdm_capnp::operation::Reader) -> String {
        let op_type = op.get_vpi_op_type();
        let op_name = match op_type {
            1 => "-",
            2 => "+",
            3 => "!",
            4 => "~",
            5 => "&",
            6 => "~&",
            7 => "|",
            8 => "~|",
            9 => "^",
            10 => "~^",
            11 => "-",
            12 => "/",
            13 => "%",
            14 => "==",
            15 => "!=",
            16 => "===",
            17 => "!==",
            18 => ">",
            19 => ">=",
            20 => "<",
            21 => "<=",
            22 => "<<",
            23 => ">>",
            24 => "+",
            25 => "*",
            26 => "&&",
            27 => "||",
            28 => "&",
            29 => "|",
            30 => "^",
            31 => "~^",
            32 => "?:",
            33 => "{...}",
            34 => "{n{}}",
            35 => "or",
            39 => "posedge",
            40 => "negedge",
            _ => "?",
        };

        if let Ok(operands) = op.get_operands() {
            if operands.len() == 1 {
                let operand = operands.get(0);
                if let Some(operand_ref) = ctx.parse_obj_index(operand) {
                    let operand_str = self.resolve_expression(ctx, operand_ref);
                    if op_type >= 1 && op_type <= 10 || op_type == 35 {
                        return format!("{}{}", op_name, operand_str);
                    }
                    return format!("({} {})", op_name, operand_str);
                }
            } else if operands.len() == 2 {
                let left = operands.get(0);
                let right = operands.get(1);
                let left_ref = ctx.parse_obj_index(left);
                let right_ref = ctx.parse_obj_index(right);
                if let (Some(l), Some(r)) = (left_ref, right_ref) {
                    let left_str = self.resolve_expression(ctx, l);
                    let right_str = self.resolve_expression(ctx, r);
                    return format!("{} {} {}", left_str, op_name, right_str);
                }
            }
        }
        "?".to_string()
    }

    pub fn walk_design_tree(&mut self, ctx: &super::UhdmContext, output: &mut String) {
        let root = ctx.get_root();

        if let Ok(designs) = root.get_designs() {
            for i in 0..designs.len() {
                let design = designs.get(i);
                self.visit_design_tree(ctx, design, output);
            }
        }
    }

    fn visit_design_tree(&mut self, ctx: &super::UhdmContext, design: super::uhdm_capnp::design::Reader, output: &mut String) {
        let base = design.get_base();
        let uhdm_id = if let Ok(base) = base { base.get_uhdm_id() } else { 0 };

        let key = ObjKey {
            raw_type: super::raw_types::RAW_UHDM_DESIGN,
            index1: uhdm_id,
        };
        self.mark_visited(key);

        let name = self.resolve_name(ctx, design.get_vpi_name());
        self.line(output, &format!("design {}", name));

        self.depth += 1;

        // 遍历 module_inst 输出完整的 module
        if let Ok(module_insts) = ctx.get_root().get_factory_moduleinst() {
            for i in 0..module_insts.len() {
                let module = module_insts.get(i);
                self.visit_module_inst_tree(ctx, module, output);
            }
        }

        self.depth -= 1;
    }

    fn visit_module_inst_tree(&mut self, ctx: &super::UhdmContext, module: super::uhdm_capnp::moduleinst::Reader, output: &mut String) {
        // 获取 module full_name
        let full_name = if let Ok(base) = module.get_base() {
            if let Ok(inst_base) = base.get_base() {
                let full_name_id = inst_base.get_vpi_full_name();
                ctx.symbol(full_name_id).unwrap_or("?".to_string())
            } else {
                "?".to_string()
            }
        } else {
            "?".to_string()
        };
        
        // 只处理 top-level module（full_name 不包含 . 的，即不是实例化的子模块）
        if full_name.contains(".") {
            return;
        }
        
        // 从 full_name 提取 module 名称（去掉 work@ 前缀）
        let name = if full_name.starts_with("work@") {
            full_name.strip_prefix("work@").unwrap_or(&full_name).to_string()
        } else {
            full_name.clone()
        };
        
        // 使用 full_name 去重
        let module_key = ObjKey {
            raw_type: super::raw_types::RAW_UHDM_MODULE_INST,
            index1: if full_name == "?" { 1 } else { full_name.len() as u64 },
        };
        if self.visited.contains(&module_key) {
            return;
        }
        self.mark_visited(module_key);

        // 输出 module 声明开始
        self.line(output, &format!("module {} #(", name));

        // 遍历 parameters
        self.depth += 1;
        if let Ok(params) = ctx.get_root().get_factory_parameter() {
            for i in 0..params.len() {
                let param = params.get(i);
                self.visit_parameter_tree(ctx, param, output);
            }
        }
        self.depth -= 1;
        self.line(output, ") (");

        // 准备 module 前缀用于过滤
        let module_prefix = format!("{}.", full_name);

        // 遍历 module 的 ports（通过检查 parent 来过滤）
        self.depth += 1;
        // 使用 module 级别的 port name 去重
        let mut module_ports: std::collections::HashSet<String> = std::collections::HashSet::new();
        if let Ok(ports) = ctx.get_root().get_factory_port() {
            for i in 0..ports.len() {
                let port = ports.get(i);
                // 检查 port 是否属于当前 module
                let port_module = if let Ok(ports_base) = port.get_base() {
                    if let Ok(any_base) = ports_base.get_base() {
                        if let Ok(parent_ref) = any_base.get_vpi_parent() {
                            if let Some(parent_obj) = ctx.parse_obj_index(parent_ref) {
                                self.get_object_full_name(ctx, parent_obj)
                            } else {
                                "?".to_string()
                            }
                        } else {
                            "?".to_string()
                        }
                    } else {
                        "?".to_string()
                    }
                } else {
                    "?".to_string()
                };
                // 只输出属于当前 module 的 port
                if port_module != full_name {
                    continue;
                }
                // 获取 port 名称用于去重
                let port_name = if let Ok(ports_base) = port.get_base() {
                    let name_id = ports_base.get_vpi_name();
                    ctx.symbol(name_id).unwrap_or("?".to_string())
                } else {
                    "?".to_string()
                };
                // 在 module 级别去重
                if module_ports.contains(&port_name) {
                    continue;
                }
                module_ports.insert(port_name);
                self.visit_port_tree(ctx, port, &module_prefix, output);
            }
        }
        self.depth -= 1;
        self.line(output, ");");

        // 遍历 module 内部内容（只输出属于当前 module 的内容）
        self.depth += 1;

        // 遍历变量声明 (logic_net)
        if let Ok(logicnets) = ctx.get_root().get_factory_logicnet() {
            for i in 0..logicnets.len() {
                let net = logicnets.get(i);
                self.visit_logic_net_tree(ctx, net, &module_prefix, output);
            }
        }

        // 遍历连续赋值 (cont_assign) - 通过 module 的 vpiContAssign
        if let Ok(contassigns) = module.get_contassigns() {
            for i in 0..contassigns.len() {
                let ca_index = contassigns.get(i);
                if ca_index > 0 {
                    let ca_idx = (ca_index - 1) as u32;
                    self.visit_cont_assign_by_index(ctx, ca_idx, output);
                }
            }
        }

        // 遍历 always 块 - 通过 module 的 vpiProcess
        if let Ok(processes) = module.get_process() {
            for i in 0..processes.len() {
                let proc_ref = processes.get(i);
                if let Some(proc_obj) = ctx.parse_obj_index(proc_ref) {
                    self.visit_process_by_ref(ctx, proc_obj, output);
                }
            }
        }

        self.depth -= 1;

        // 遍历 module 实例化（ref_module）
        self.visit_ref_modules(ctx, module, &full_name, output);

        // 输出 endmodule
        self.line(output, "endmodule");
        self.line(output, "");
    }

    fn visit_ref_modules(&mut self, ctx: &super::UhdmContext, module: super::uhdm_capnp::moduleinst::Reader, parent_full_name: &str, output: &mut String) {
        // 获取 module 的 ref_modules（子模块实例）
        // 通过全局 factory_refmodule 遍历，检查 parent 是否属于当前 module
        if let Ok(ref_modules) = ctx.get_root().get_factory_refmodule() {
            for i in 0..ref_modules.len() {
                let ref_mod = ref_modules.get(i);
                // 检查 ref_module 是否属于当前 module
                let ref_mod_parent = if let Ok(any_base) = ref_mod.get_base() {
                    if let Ok(parent_ref) = any_base.get_vpi_parent() {
                        if let Some(parent_obj) = ctx.parse_obj_index(parent_ref) {
                            self.get_object_full_name(ctx, parent_obj)
                        } else {
                            "?".to_string()
                        }
                    } else {
                        "?".to_string()
                    }
                } else {
                    "?".to_string()
                };
                // 只处理属于当前 module 的 ref_module
                if ref_mod_parent != parent_full_name {
                    continue;
                }
                self.visit_ref_module_tree(ctx, ref_mod, output);
            }
        }
    }

    fn visit_ref_module_tree(&mut self, ctx: &super::UhdmContext, ref_mod: super::uhdm_capnp::refmodule::Reader, output: &mut String) {
        let name_id = ref_mod.get_vpi_name();
        let def_name_id = ref_mod.get_vpi_def_name();
        let inst_name = ctx.symbol(name_id).unwrap_or("?".to_string());
        let def_name_full = ctx.symbol(def_name_id).unwrap_or("?".to_string());

        if inst_name == "?" || inst_name.is_empty() {
            return;
        }

        // 提取模块名（去掉 work@ 前缀）
        let def_name = if let Some(pos) = def_name_full.find('@') {
            &def_name_full[pos + 1..]
        } else {
            &def_name_full
        };

        self.depth += 1;
        // 输出实例化语句: def_name inst_name (
        self.line(output, &format!("{} {} (", def_name, inst_name));

        // 输出 port 连接
        self.depth += 1;
        let mut port_count = 0;
        if let Ok(ports) = ref_mod.get_ports() {
            let total_ports = ports.len();
            for i in 0..total_ports {
                let port_index = ports.get(i);
                // port_index 是 u64，需要转换为 pool 索引（防止溢出）
                let port_idx = if port_index > 0 {
                    (port_index - 1) as u32
                } else {
                    continue;
                };
                self.visit_ref_module_port(ctx, port_idx, output, i < total_ports - 1);
                port_count += 1;
            }
        }
        self.depth -= 1;
        // 输出结束括号
        self.line(output, ");");
        self.depth -= 1;
    }

    fn visit_ref_module_port(&mut self, ctx: &super::UhdmContext, port_idx: u32, output: &mut String, has_more: bool) {
        if let Ok(pool) = ctx.get_root().get_factory_port() {
            if port_idx < pool.len() {
                let port = pool.get(port_idx);
                if let Ok(ports_base) = port.get_base() {
                    let name_id = ports_base.get_vpi_name();
                    let port_name = ctx.symbol(name_id).unwrap_or("?".to_string());

                    // 获取 HighConn (外部连接)
                    // 对于 ref_module 的 port，只有 HighConn 有意义
                    let high_conn = if let Ok(high_conn_ref) = ports_base.get_highconn() {
                        if let Some(high_conn_obj) = ctx.parse_obj_index(high_conn_ref) {
                            self.get_ref_obj_name(ctx, high_conn_obj)
                        } else {
                            "?".to_string()
                        }
                    } else {
                        "?".to_string()
                    };

                    if port_name != "?" && !port_name.is_empty() {
                        let comma = if has_more { "," } else { "" };
                        self.line(output, &format!(".{}({}){}", port_name, high_conn, comma));
                    }
                }
            }
        }
    }

    fn get_ref_obj_name(&self, ctx: &super::UhdmContext, obj_ref: super::ObjRef) -> String {
        use super::raw_types::*;

        // 防止溢出
        let index0 = if obj_ref.index1 > 0 {
            (obj_ref.index1 - 1) as u32
        } else {
            return "?".to_string();
        };

        match obj_ref.raw_type {
            RAW_UHDM_REF_OBJ => {
                if let Ok(pool) = ctx.get_root().get_factory_refobj() {
                    if index0 < pool.len() {
                        let ref_obj = pool.get(index0);
                        let name_id = ref_obj.get_vpi_name();
                        ctx.symbol(name_id).unwrap_or_else(|| "?".to_string())
                    } else {
                        "?".to_string()
                    }
                } else {
                    "?".to_string()
                }
            }
            _ => "?".to_string(),
        }
    }

    fn visit_parameter_tree(&mut self, ctx: &super::UhdmContext, param: super::uhdm_capnp::parameter::Reader, output: &mut String) {
        // 获取参数名
        let name_id = param.get_vpi_name();
        let name = ctx.symbol(name_id).unwrap_or("?".to_string());

        // 只输出 WIDTH 参数
        if name != "WIDTH" {
            return;
        }

        // 使用 full_name 去重
        let full_name_id = param.get_vpi_full_name();
        let param_key = ObjKey {
            raw_type: super::raw_types::RAW_UHDM_PARAMETER,
            index1: full_name_id,
        };
        if self.visited.contains(&param_key) {
            return;
        }
        self.mark_visited(param_key);

        self.line(output, &format!("parameter {} = 8,", name));
    }

    fn visit_port_by_ref(&mut self, ctx: &super::UhdmContext, port_ref: super::ObjRef, output: &mut String) {
        use super::raw_types::*;

        let index0 = (port_ref.index1 - 1) as u32;

        let (name, direction) = match port_ref.raw_type {
            RAW_UHDM_PORT => {
                if let Ok(pool) = ctx.get_root().get_factory_port() {
                    if index0 < pool.len() {
                        let port = pool.get(index0);
                        if let Ok(ports_base) = port.get_base() {
                            let name_id = ports_base.get_vpi_name();
                            let dir = ports_base.get_vpi_direction();
                            let name = ctx.symbol(name_id).unwrap_or("?".to_string());
                            let dir_str = match dir {
                                1 => "input",
                                2 => "output",
                                3 => "inout",
                                6 => "output", // reg
                                _ => "",
                            };
                            (name, dir_str)
                        } else {
                            ("?".to_string(), "")
                        }
                    } else {
                        ("?".to_string(), "")
                    }
                } else {
                    ("?".to_string(), "")
                }
            }
            _ => ("?".to_string(), ""),
        };

        if name == "?" || name.is_empty() {
            return;
        }

        self.line(output, &format!("{} wire {},", direction, name));
    }

    fn visit_port_tree(&mut self, ctx: &super::UhdmContext, port: super::uhdm_capnp::port::Reader, module_prefix: &str, output: &mut String) {
        // 获取 port 名称和方向
        // port -> ports (base) -> vpi_name, vpi_direction
        let (name, direction) = if let Ok(ports_base) = port.get_base() {
            let name_id = ports_base.get_vpi_name();
            let dir = ports_base.get_vpi_direction();
            let name = ctx.symbol(name_id).unwrap_or("?".to_string());
            let dir_str = match dir {
                1 => "input",
                2 => "output",
                3 => "inout",
                6 => "output", // reg
                _ => "",
            };
            (name, dir_str)
        } else {
            ("?".to_string(), "")
        };

        // 过滤掉无效的 port
        if name == "?" || name.is_empty() {
            return;
        }

        self.line(output, &format!("{} wire {},", direction, name));
    }

    fn visit_logic_net_tree(&mut self, ctx: &super::UhdmContext, net: super::uhdm_capnp::logicnet::Reader, module_prefix: &str, output: &mut String) {
        // 获取变量名和 full_name 用于去重
        let (name, full_name, full_name_id) = if let Ok(base) = net.get_base() {
            if let Ok(nets_base) = base.get_base() {
                let name_id = nets_base.get_vpi_name();
                let full_name_id = nets_base.get_vpi_full_name();
                let name = ctx.symbol(name_id).unwrap_or("?".to_string());
                let full_name = ctx.symbol(full_name_id).unwrap_or("?".to_string());
                (name, full_name, full_name_id)
            } else {
                ("?".to_string(), "?".to_string(), 0)
            }
        } else {
            ("?".to_string(), "?".to_string(), 0)
        };

        // 使用 full_name 过滤，只输出属于当前 module 的 net
        if !full_name.starts_with(module_prefix) {
            return;
        }
        // 使用 full_name_id 作为 key 去重
        let net_key = ObjKey {
            raw_type: super::raw_types::RAW_UHDM_LOGIC_NET,
            index1: full_name_id,
        };
        if self.visited.contains(&net_key) {
            return;
        }
        self.mark_visited(net_key);

        // 获取范围（位宽）
        let range_str = if let Ok(ranges) = net.get_ranges() {
            if ranges.len() >= 2 {
                let left = ranges.get(0);
                let right = ranges.get(1);
                format!("[{}:{}]", left, right)
            } else if ranges.len() == 1 {
                format!("[{}]", ranges.get(0))
            } else {
                "".to_string()
            }
        } else {
            "".to_string()
        };

        self.line(output, &format!("wire {} {};", range_str, name));
    }

    fn visit_cont_assign_tree(&mut self, ctx: &super::UhdmContext, ca: super::uhdm_capnp::contassign::Reader, output: &mut String) {
        let lhs_name = if let Ok(lhs_ref) = ca.get_lhs() {
            if let Some(obj_ref) = ctx.parse_obj_index(lhs_ref) {
                self.resolve_expression(ctx, obj_ref)
            } else {
                "?".to_string()
            }
        } else {
            "?".to_string()
        };

        let rhs_name = if let Ok(rhs_ref) = ca.get_rhs() {
            if let Some(obj_ref) = ctx.parse_obj_index(rhs_ref) {
                self.resolve_expression(ctx, obj_ref)
            } else {
                "?".to_string()
            }
        } else {
            "?".to_string()
        };

        self.line(output, &format!("assign {} = {};", lhs_name, rhs_name));
    }

    fn visit_always_tree(&mut self, ctx: &super::UhdmContext, a: super::uhdm_capnp::always::Reader, output: &mut String) {
        let proc = a.get_base();
        if let Ok(proc) = proc {
            let stmt_ref = proc.get_stmt();
            if let Ok(stmt_ref) = stmt_ref {
                if let Some(stmt_obj) = ctx.parse_obj_index(stmt_ref) {
                    self.visit_stmt_tree(ctx, stmt_obj, output);
                    return;
                }
            }
        }
        self.line(output, "always @(*) begin");
        self.depth += 1;
        self.line(output, "// <unparsed>");
        self.depth -= 1;
        self.line(output, "end");
    }

    fn visit_if_else(&mut self, ctx: &super::UhdmContext, ifelse: super::uhdm_capnp::ifelse::Reader, output: &mut String) {
        if let Ok(cond_ref) = ifelse.get_vpi_condition() {
            if let Some(cond_obj) = ctx.parse_obj_index(cond_ref) {
                let cond_str = self.resolve_expression(ctx, cond_obj);
                self.line(output, &format!("if ({}) begin", cond_str));
                self.depth += 1;
                if let Ok(then_ref) = ifelse.get_vpi_stmt() {
                    if let Some(then_obj) = ctx.parse_obj_index(then_ref) {
                        self.visit_stmt_tree(ctx, then_obj, output);
                    }
                }
                self.depth -= 1;
                if let Ok(else_ref) = ifelse.get_vpi_else_stmt() {
                    if let Some(else_obj) = ctx.parse_obj_index(else_ref) {
                        self.depth += 1;
                        self.visit_stmt_tree(ctx, else_obj, output);
                        self.depth -= 1;
                    }
                } else {
                    self.line(output, "end");
                }
            }
        }
    }

    fn visit_if_stmt(&mut self, ctx: &super::UhdmContext, ifstmt: super::uhdm_capnp::ifstmt::Reader, output: &mut String) {
        if let Ok(cond_ref) = ifstmt.get_vpi_condition() {
            if let Some(cond_obj) = ctx.parse_obj_index(cond_ref) {
                let cond_str = self.resolve_expression(ctx, cond_obj);
                self.line(output, &format!("if ({}) begin", cond_str));
                self.depth += 1;
                let mut has_stmt = false;
                if let Ok(then_ref) = ifstmt.get_vpi_stmt() {
                    if let Some(then_obj) = ctx.parse_obj_index(then_ref) {
                        self.visit_stmt_tree(ctx, then_obj, output);
                        has_stmt = true;
                    }
                }
                if has_stmt {
                    self.depth -= 1;
                    self.line(output, "end");
                }
            }
        }
    }

    fn visit_stmt_tree(&mut self, ctx: &super::UhdmContext, stmt_ref: super::ObjRef, output: &mut String) {
        use super::raw_types::*;

        let index0 = (stmt_ref.index1 - 1) as u32;

        match stmt_ref.raw_type {
            RAW_UHDM_EVENT_CONTROL => {
                if let Ok(pool) = ctx.get_root().get_factory_eventcontrol() {
                    if index0 < pool.len() {
                        let ec = pool.get(index0);
                        let sens_str = if let Ok(cond_ref) = ec.get_vpi_condition() {
                            let cond_obj = ctx.parse_obj_index(cond_ref);
                            if let Some(c) = cond_obj {
                                self.resolve_expression(ctx, c)
                            } else {
                                "*".to_string()
                            }
                        } else {
                            "*".to_string()
                        };
                        self.line(output, &format!("always @({}) begin", sens_str));
                        self.depth += 1;
                        if let Ok(stmt_of_ec) = ec.get_stmt() {
                            if let Some(stmt_obj) = ctx.parse_obj_index(stmt_of_ec) {
                                self.visit_stmt_tree(ctx, stmt_obj, output);
                                self.depth -= 1;
                                self.line(output, "end");
                                return;
                            }
                        }
                        self.depth -= 1;
                        self.line(output, "end");
                    }
                }
            }
            RAW_UHDM_BEGIN => {
                if let Ok(pool) = ctx.get_root().get_factory_begin() {
                    if index0 < pool.len() {
                        let begin = pool.get(index0);
                        self.line(output, "begin");
                        self.depth += 1;
                        if let Ok(stmts) = begin.get_stmts() {
                            for i in 0..stmts.len() {
                                let s = stmts.get(i);
                                if let Some(stmt_obj) = ctx.parse_obj_index(s) {
                                    self.visit_stmt_tree(ctx, stmt_obj, output);
                                }
                            }
                        }
                        self.depth -= 1;
                        self.line(output, "end");
                    }
                }
            }
            RAW_UHDM_IF_ELSE | RAW_UHDM_IF_STMT => {
                let pool_name = if stmt_ref.raw_type == RAW_UHDM_IF_ELSE {
                    "get_factory_ifelse"
                } else {
                    "get_factory_ifstmt"
                };
                if pool_name == "get_factory_ifelse" {
                    if let Ok(pool) = ctx.get_root().get_factory_ifelse() {
                        if index0 < pool.len() {
                            let ifelse = pool.get(index0);
                            self.visit_if_else(ctx, ifelse, output);
                            return;
                        }
                    }
                } else {
                    if let Ok(pool) = ctx.get_root().get_factory_ifstmt() {
                        if index0 < pool.len() {
                            let ifstmt = pool.get(index0);
                            self.visit_if_stmt(ctx, ifstmt, output);
                            return;
                        }
                    }
                }
            }
            RAW_UHDM_ASSIGNMENT => {
                if let Ok(pool) = ctx.get_root().get_factory_assignment() {
                    if index0 < pool.len() {
                        let assign = pool.get(index0);
                        if let Ok(rhs_ref) = assign.get_rhs() {
                            if let Some(rhs_obj) = ctx.parse_obj_index(rhs_ref) {
                                let rhs_str = self.resolve_expression(ctx, rhs_obj);
                                if let Ok(lhs_ref) = assign.get_lhs() {
                                    if let Some(lhs_obj) = ctx.parse_obj_index(lhs_ref) {
                                        let lhs_str = self.resolve_expression(ctx, lhs_obj);
                                        let is_non_blocking = assign.get_vpi_op_type() == 82;
                                        if is_non_blocking {
                                            self.line(output, &format!("{} <= {};", lhs_str, rhs_str));
                                        } else {
                                            self.line(output, &format!("{} = {};", lhs_str, rhs_str));
                                        }
                                        return;
                                    }
                                }
                            }
                        }
                        self.line(output, "// <assignment>");
                    }
                }
            }
            RAW_UHDM_CONSTANT => {
                let const_str = self.resolve_expression(ctx, stmt_ref);
                self.line(output, &format!("// const: {}", const_str));
            }
            _ => {
                let type_name = raw_type_name(stmt_ref.raw_type);
                self.line(output, &format!("// {} (idx={})", type_name, index0));
            }
        }
    }

    fn visit_cont_assign_by_index(&mut self, ctx: &super::UhdmContext, ca_idx: u32, output: &mut String) {
        if let Ok(pool) = ctx.get_root().get_factory_contassign() {
            if ca_idx < pool.len() {
                let ca = pool.get(ca_idx);
                self.visit_cont_assign_tree(ctx, ca, output);
            }
        }
    }

    fn visit_process_by_ref(&mut self, ctx: &super::UhdmContext, proc_ref: super::ObjRef, output: &mut String) {
        use super::raw_types::*;

        let index0 = (proc_ref.index1 - 1) as u32;

        match proc_ref.raw_type {
            RAW_UHDM_ALWAYS => {
                if let Ok(pool) = ctx.get_root().get_factory_always() {
                    if index0 < pool.len() {
                        let a = pool.get(index0);
                        self.visit_always_tree(ctx, a, output);
                    }
                }
            }
            _ => {
                // 其他 process 类型暂不支持
            }
        }
    }

    pub fn walk_designs(&mut self, ctx: &super::UhdmContext, output: &mut String) {
        let root = ctx.get_root();
        if let Ok(designs) = root.get_designs() {
            for i in 0..designs.len() {
                let design = designs.get(i);
                self.visit_design(ctx, design, i as u64 + 1, output);
            }
        }
    }

    fn visit_design(&mut self, ctx: &super::UhdmContext, design: super::uhdm_capnp::design::Reader, index: u64, output: &mut String) {
        let base = design.get_base();
        let uhdm_id = if let Ok(base) = base { base.get_uhdm_id() } else { 0 };

        let key = ObjKey {
            raw_type: super::raw_types::RAW_UHDM_DESIGN,
            index1: uhdm_id,
        };
        self.mark_visited(key);

        let name_id = design.get_vpi_name();
        if let Some(name) = ctx.symbol(name_id) {
            *output += &format!("  Design: {} (pool_idx={}, uhdm_id={})\n", name, index, uhdm_id);
        } else {
            *output += &format!("  Design: (anonymous, pool_idx={}, uhdm_id={})\n", index, uhdm_id);
        }

        if let Ok(top_modules) = design.get_top_modules() {
            *output += &format!("    topModules: {}\n", top_modules.len());
        }
        if let Ok(all_modules) = design.get_all_modules() {
            *output += &format!("    allModules: {}\n", all_modules.len());
        }
        if let Ok(typespecs) = design.get_typespecs() {
            *output += &format!("    typespecs: {}\n", typespecs.len());
        }
    }

    pub fn walk_all(&mut self, ctx: &super::UhdmContext, output: &mut String) {
        let root = ctx.get_root();
        self.depth += 1;

        if let Ok(designs) = root.get_designs() {
            *output += &format!("=== Designs: {} ===\n", designs.len());
            for i in 0..designs.len() {
                let design = designs.get(i);
                self.visit_design(ctx, design, i as u64 + 1, output);
            }
        }

        if let Ok(module_insts) = root.get_factory_moduleinst() {
            *output += &format!("\n=== ModuleInsts: {} ===\n", module_insts.len());
            for i in 0..module_insts.len() {
                let inst = module_insts.get(i);
                self.visit_module_inst_shallow(ctx, inst, i as u64 + 1, output);
            }
        }

        if let Ok(ports) = root.get_factory_port() {
            *output += &format!("\n=== Ports: {} ===\n", ports.len());
            for i in 0..ports.len() {
                let port = ports.get(i);
                self.visit_port_shallow(ctx, port, i as u64 + 1, output);
            }
        }

        if let Ok(nets) = root.get_factory_logicnet() {
            *output += &format!("\n=== Logicnets: {} ===\n", nets.len());
            for i in 0..nets.len() {
                let net = nets.get(i);
                self.visit_net_shallow(ctx, net, i as u64 + 1, output);
            }
        }

        if let Ok(constants) = root.get_factory_constant() {
            *output += &format!("\n=== Constants: {} ===\n", constants.len());
            for i in 0..constants.len() {
                let c = constants.get(i);
                self.visit_constant_shallow(ctx, c, i as u64 + 1, output);
            }
        }

        if let Ok(operations) = root.get_factory_operation() {
            *output += &format!("\n=== Operations: {} ===\n", operations.len());
            for i in 0..operations.len() {
                let op = operations.get(i);
                self.visit_operation_shallow(ctx, op, i as u64 + 1, output);
            }
        }

        if let Ok(contassigns) = root.get_factory_contassign() {
            *output += &format!("\n=== ContAssigns: {} ===\n", contassigns.len());
            for i in 0..contassigns.len() {
                let ca = contassigns.get(i);
                self.visit_cont_assign_shallow(ctx, ca, i as u64 + 1, output);
            }
        }

        if let Ok(parameters) = root.get_factory_parameter() {
            *output += &format!("\n=== Parameters: {} ===\n", parameters.len());
            for i in 0..parameters.len() {
                let p = parameters.get(i);
                self.visit_parameter_shallow(ctx, p, i as u64 + 1, output);
            }
        }

        if let Ok(refobjs) = root.get_factory_refobj() {
            *output += &format!("\n=== Refobjs: {} ===\n", refobjs.len());
            for i in 0..refobjs.len() {
                let r = refobjs.get(i);
                self.visit_ref_obj_shallow(ctx, r, i as u64 + 1, output);
            }
        }

        if let Ok(alwa) = root.get_factory_always() {
            *output += &format!("\n=== Always: {} ===\n", alwa.len());
            for i in 0..alwa.len() {
                let a = alwa.get(i);
                self.visit_always_shallow(ctx, a, i as u64 + 1, output);
            }
        }

        self.depth -= 1;
    }

    fn visit_module_inst_shallow(&mut self, ctx: &super::UhdmContext, m: super::uhdm_capnp::moduleinst::Reader, index: u64, output: &mut String) {
        let base = m.get_base();
        let uhdm_id = if let Ok(base) = base {
            if let Ok(inst) = base.get_base() {
                if let Ok(any_base) = inst.get_base() {
                    any_base.get_uhdm_id()
                } else { 0 }
            } else { 0 }
        } else { 0 };

        let name = if let Ok(base) = base {
            if let Ok(inst) = base.get_base() {
                let name_id = inst.get_vpi_name();
                ctx.symbol(name_id)
            } else { None }
        } else { None };

        let name_str = name.unwrap_or_else(|| "(anonymous)".to_string());
        *output += &format!("  ModuleInst: {} (pool_idx={}, uhdm_id={})\n", name_str, index, uhdm_id);
    }

    fn visit_port_shallow(&mut self, ctx: &super::UhdmContext, p: super::uhdm_capnp::port::Reader, index: u64, output: &mut String) {
        let ports_base = p.get_base();
        let uhdm_id = if let Ok(ports_base) = ports_base {
            if let Ok(any_base) = ports_base.get_base() {
                any_base.get_uhdm_id()
            } else { 0 }
        } else { 0 };

        let name = if let Ok(ports_base) = ports_base {
            let name_id = ports_base.get_vpi_name();
            ctx.symbol(name_id)
        } else { None };

        let name_str = name.unwrap_or_else(|| "(anonymous)".to_string());
        *output += &format!("  Port: {} (pool_idx={}, uhdm_id={})\n", name_str, index, uhdm_id);
    }

    fn visit_net_shallow(&mut self, _ctx: &super::UhdmContext, _n: super::uhdm_capnp::logicnet::Reader, index: u64, output: &mut String) {
        *output += &format!("  Net: (pool_idx={})\n", index);
    }

    fn visit_constant_shallow(&mut self, _ctx: &super::UhdmContext, _c: super::uhdm_capnp::constant::Reader, index: u64, output: &mut String) {
        *output += &format!("  Constant: (pool_idx={})\n", index);
    }

    fn visit_operation_shallow(&mut self, _ctx: &super::UhdmContext, _op: super::uhdm_capnp::operation::Reader, index: u64, output: &mut String) {
        *output += &format!("  Operation: (pool_idx={})\n", index);
    }

    fn visit_cont_assign_shallow(&mut self, ctx: &super::UhdmContext, ca: super::uhdm_capnp::contassign::Reader, index: u64, output: &mut String) {
        let base = ca.get_base();
        let uhdm_id = if let Ok(base) = base { base.get_uhdm_id() } else { 0 };

        let lhs_name = if let Ok(lhs_ref) = ca.get_lhs() {
            if let Some(obj_ref) = ctx.parse_obj_index(lhs_ref) {
                self.resolve_expression(ctx, obj_ref)
            } else {
                "?".to_string()
            }
        } else {
            "?".to_string()
        };

        let rhs_name = if let Ok(rhs_ref) = ca.get_rhs() {
            if let Some(obj_ref) = ctx.parse_obj_index(rhs_ref) {
                self.resolve_expression(ctx, obj_ref)
            } else {
                "?".to_string()
            }
        } else {
            "?".to_string()
        };

        *output += &format!("  ContAssign: idx={}, id={}, lhs={}, rhs={}\n", index, uhdm_id, lhs_name, rhs_name);
    }

    fn visit_parameter_shallow(&mut self, _ctx: &super::UhdmContext, _p: super::uhdm_capnp::parameter::Reader, index: u64, output: &mut String) {
        *output += &format!("  Parameter: (pool_idx={})\n", index);
    }

    fn visit_ref_obj_shallow(&mut self, ctx: &super::UhdmContext, r: super::uhdm_capnp::refobj::Reader, index: u64, output: &mut String) {
        let name_id = r.get_vpi_name();
        let name = if name_id > 0 {
            ctx.symbol(name_id).unwrap_or_else(|| "(no name)".to_string())
        } else {
            "(no name)".to_string()
        };
        *output += &format!("  RefObj: {} (pool_idx={})\n", name, index);
    }

    fn visit_always_shallow(&mut self, ctx: &super::UhdmContext, a: super::uhdm_capnp::always::Reader, index: u64, output: &mut String) {
        let proc = a.get_base();
        let (uhdm_id, stmt_type) = if let Ok(proc) = proc {
            let any_base = proc.get_base();
            let id = if let Ok(any_base) = any_base {
                any_base.get_uhdm_id()
            } else { 0 };

            let stmt_ref = proc.get_stmt();
            let stmt_info = if let Ok(stmt_ref) = stmt_ref {
                if let Some(obj_ref) = ctx.parse_obj_index(stmt_ref) {
                    super::raw_types::raw_type_name(obj_ref.raw_type)
                } else {
                    "?"
                }
            } else {
                "?"
            };
            (id, stmt_info)
        } else {
            (0, "?")
        };

        *output += &format!("  Always: idx={}, id={}, stmt={}\n", index, uhdm_id, stmt_type);
    }
}

impl Default for Walker {
    fn default() -> Self {
        Self::new()
    }
}

pub fn walk_design(ctx: &super::UhdmContext) -> String {
    let mut walker = Walker::new();
    let mut output = String::new();
    walker.walk_designs(ctx, &mut output);
    output
}

pub fn walk_design_tree(ctx: &super::UhdmContext) -> String {
    let mut walker = Walker::new();
    let mut output = String::new();
    walker.walk_design_tree(ctx, &mut output);
    output
}

pub fn walk_all(ctx: &super::UhdmContext) -> String {
    let mut walker = Walker::new();
    let mut output = String::new();
    walker.walk_all(ctx, &mut output);
    output
}
