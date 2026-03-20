pub mod raw_types;
pub mod walker;

pub mod uhdm_capnp {
    include!(concat!(env!("OUT_DIR"), "/schema/uhdm_capnp.rs"));
}

pub mod regression_test;

pub use capnp::message::ReaderOptions;

use capnp::serialize_packed;
use std::fs::File;
use std::io::BufReader;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum UhdmError {
    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),
    #[error("Capnp error: {0}")]
    Capnp(#[from] capnp::Error),
    #[error("Invalid UHDM file format")]
    InvalidFormat,
    #[error("Failed to get root node: {0}")]
    RootNodeError(String),
    #[error("Failed to read symbols: {0}")]
    SymbolsError(String),
    #[error("Failed to resolve object: type={0}, index={1}")]
    ResolveError(u32, u64),
    #[error("Index out of bounds: {0}")]
    IndexOutOfBounds(String),
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct ObjRef {
    pub raw_type: u32,
    pub index1: u64,
}

#[derive(Debug, Clone, Copy)]
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

pub struct UhdmContext<'a> {
    root: uhdm_capnp::uhdm_root::Reader<'a>,
}

impl<'a> UhdmContext<'a> {
    pub fn new(root: uhdm_capnp::uhdm_root::Reader<'a>) -> Self {
        Self { root }
    }

    pub fn symbol(&self, id: u64) -> Option<String> {
        let syms = self.root.get_symbols().ok()?;
        let s = syms.get(id as u32).ok()?;
        Some(s.to_str().ok()?.to_string())
    }

    pub fn parse_obj_index(
        &self,
        idx: uhdm_capnp::obj_index_type::Reader<'a>,
    ) -> Option<ObjRef> {
        Some(ObjRef {
            raw_type: idx.get_type(),
            index1: idx.get_index(),
        })
    }

    pub fn resolve_raw_type_index(&self, raw_type: u32, index0: u32) -> Option<Node<'a>> {
        use raw_types::*;
        match raw_type {
            RAW_UHDM_OPERATION => {
                let pool = self.root.get_factory_operation().ok()?;
                if index0 >= pool.len() { return None; }
                Some(Node::Operation(pool.get(index0)))
            }
            RAW_UHDM_CONSTANT => {
                let pool = self.root.get_factory_constant().ok()?;
                if index0 >= pool.len() { return None; }
                Some(Node::Constant(pool.get(index0)))
            }
            RAW_UHDM_PORT => {
                let pool = self.root.get_factory_port().ok()?;
                if index0 >= pool.len() { return None; }
                Some(Node::Port(pool.get(index0)))
            }
            RAW_UHDM_PORT_BIT => {
                let pool = self.root.get_factory_portbit().ok()?;
                if index0 >= pool.len() { return None; }
                Some(Node::PortBit(pool.get(index0)))
            }
            RAW_UHDM_BIT_SELECT => {
                let pool = self.root.get_factory_bitselect().ok()?;
                if index0 >= pool.len() { return None; }
                Some(Node::BitSelect(pool.get(index0)))
            }
            RAW_UHDM_PART_SELECT => {
                let pool = self.root.get_factory_partselect().ok()?;
                if index0 >= pool.len() { return None; }
                Some(Node::PartSelect(pool.get(index0)))
            }
            RAW_UHDM_INDEXED_PART_SELECT => {
                let pool = self.root.get_factory_indexedpartselect().ok()?;
                if index0 >= pool.len() { return None; }
                Some(Node::IndexedPartSelect(pool.get(index0)))
            }
            RAW_UHDM_REF_OBJ => {
                let pool = self.root.get_factory_refobj().ok()?;
                if index0 >= pool.len() { return None; }
                Some(Node::RefObj(pool.get(index0)))
            }
            RAW_UHDM_LOGIC_TYPESPEC => {
                let pool = self.root.get_factory_logictypespec().ok()?;
                if index0 >= pool.len() { return None; }
                Some(Node::LogicTypespec(pool.get(index0)))
            }
            RAW_UHDM_BIT_TYPESPEC => {
                let pool = self.root.get_factory_bittypespec().ok()?;
                if index0 >= pool.len() { return None; }
                Some(Node::BitTypespec(pool.get(index0)))
            }
            RAW_UHDM_ENUM_TYPESPEC => {
                let pool = self.root.get_factory_enumtypespec().ok()?;
                if index0 >= pool.len() { return None; }
                Some(Node::EnumTypespec(pool.get(index0)))
            }
            RAW_UHDM_STRUCT_TYPESPEC => {
                let pool = self.root.get_factory_structtypespec().ok()?;
                if index0 >= pool.len() { return None; }
                Some(Node::StructTypespec(pool.get(index0)))
            }
            RAW_UHDM_PACKED_ARRAY_TYPESPEC => {
                let pool = self.root.get_factory_packedarraytypespec().ok()?;
                if index0 >= pool.len() { return None; }
                Some(Node::PackedArrayTypespec(pool.get(index0)))
            }
            RAW_UHDM_MODULE_INST => {
                let pool = self.root.get_factory_moduleinst().ok()?;
                if index0 >= pool.len() { return None; }
                Some(Node::ModuleInst(pool.get(index0)))
            }
            RAW_UHDM_INTERFACE_INST => {
                let pool = self.root.get_factory_interfaceinst().ok()?;
                if index0 >= pool.len() { return None; }
                Some(Node::InterfaceInst(pool.get(index0)))
            }
            RAW_UHDM_PACKAGE => {
                let pool = self.root.get_factory_package().ok()?;
                if index0 >= pool.len() { return None; }
                Some(Node::Package(pool.get(index0)))
            }
            RAW_UHDM_PROGRAM => {
                let pool = self.root.get_factory_program().ok()?;
                if index0 >= pool.len() { return None; }
                Some(Node::Program(pool.get(index0)))
            }
            RAW_UHDM_DESIGN => {
                let pool = self.root.get_factory_design().ok()?;
                if index0 >= pool.len() { return None; }
                Some(Node::Design(pool.get(index0)))
            }
            RAW_UHDM_PARAMETER => {
                let pool = self.root.get_factory_parameter().ok()?;
                if index0 >= pool.len() { return None; }
                Some(Node::Parameter(pool.get(index0)))
            }
            RAW_UHDM_CONT_ASSIGN => {
                let pool = self.root.get_factory_contassign().ok()?;
                if index0 >= pool.len() { return None; }
                Some(Node::ContAssign(pool.get(index0)))
            }
            RAW_UHDM_ALWAYS => {
                let pool = self.root.get_factory_always().ok()?;
                if index0 >= pool.len() { return None; }
                Some(Node::Always(pool.get(index0)))
            }
            RAW_UHDM_LOGIC_VAR => {
                let pool = self.root.get_factory_logicvar().ok()?;
                if index0 >= pool.len() { return None; }
                Some(Node::LogicVar(pool.get(index0)))
            }
            RAW_UHDM_REG => {
                let pool = self.root.get_factory_reg().ok()?;
                if index0 >= pool.len() { return None; }
                Some(Node::Reg(pool.get(index0)))
            }
            RAW_UHDM_RANGE => {
                let pool = self.root.get_factory_range().ok()?;
                if index0 >= pool.len() { return None; }
                Some(Node::Range(pool.get(index0)))
            }
            RAW_UHDM_VAR_SELECT => {
                let pool = self.root.get_factory_varselect().ok()?;
                if index0 >= pool.len() { return None; }
                Some(Node::VarSelect(pool.get(index0)))
            }
            _ => None,
        }
    }

    pub fn resolve_obj_index(
        &self,
        idx: uhdm_capnp::obj_index_type::Reader<'a>,
    ) -> Option<Node<'a>> {
        let obj_ref = self.parse_obj_index(idx)?;
        let index0 = u32::try_from(obj_ref.index1.checked_sub(1)?).ok()?;
        self.resolve_raw_type_index(obj_ref.raw_type, index0)
    }

    pub fn get_root(&self) -> uhdm_capnp::uhdm_root::Reader<'a> {
        self.root
    }
}

pub struct UhdmDesign {
    pub symbols: Vec<String>,
    pub modules: Vec<String>,
    pub factory_counts: std::collections::HashMap<String, u32>,
}

impl UhdmDesign {
    pub fn load(path: &str) -> Result<Self, UhdmError> {
        let file = File::open(path)?;
        let mut reader = BufReader::new(file);

        let message_reader = serialize_packed::read_message(&mut reader, ReaderOptions::new())
            .map_err(|_e| UhdmError::InvalidFormat)?;

        let root = message_reader
            .get_root::<uhdm_capnp::uhdm_root::Reader>()
            .map_err(|e| UhdmError::RootNodeError(e.to_string()))?;

        let mut symbols = Vec::new();
        if let Ok(symbol_list) = root.get_symbols() {
            for symbol in symbol_list {
                let s = match symbol {
                    Ok(text) => text.to_string().unwrap_or_default(),
                    Err(_) => "".to_string(),
                };
                symbols.push(s);
            }
        } else {
            return Err(UhdmError::SymbolsError(
                "Failed to get symbols list".to_string(),
            ));
        }

        let mut factory_counts = std::collections::HashMap::new();

        if let Ok(factory) = root.get_factory_moduleinst() {
            factory_counts.insert("ModuleInsts".to_string(), factory.len());
        }
        if let Ok(factory) = root.get_factory_port() {
            factory_counts.insert("Ports".to_string(), factory.len());
        }
        if let Ok(factory) = root.get_factory_logicnet() {
            factory_counts.insert("Logicnets".to_string(), factory.len());
        }
        if let Ok(factory) = root.get_factory_refobj() {
            factory_counts.insert("Refobjs".to_string(), factory.len());
        }
        if let Ok(factory) = root.get_factory_contassign() {
            factory_counts.insert("ContAssigns".to_string(), factory.len());
        }
        if let Ok(factory) = root.get_factory_operation() {
            factory_counts.insert("Operations".to_string(), factory.len());
        }
        if let Ok(factory) = root.get_factory_constant() {
            factory_counts.insert("Constants".to_string(), factory.len());
        }
        if let Ok(factory) = root.get_factory_parameter() {
            factory_counts.insert("Parameters".to_string(), factory.len());
        }
        if let Ok(factory) = root.get_factory_always() {
            factory_counts.insert("Always".to_string(), factory.len());
        }

        let mut modules = Vec::new();
        if let Ok(factory_modules) = root.get_factory_moduleinst() {
            for m in factory_modules {
                if let Ok(inst) = m.get_base() {
                    if let Ok(scope) = inst.get_base() {
                        let name_id = scope.get_vpi_name();
                        if name_id > 0 && (name_id as usize) < symbols.len() {
                            let name = symbols[name_id as usize].clone();
                            if !name.starts_with("@@") {
                                modules.push(name);
                            }
                        }
                    }
                }
            }
        }

        Ok(UhdmDesign {
            symbols,
            modules,
            factory_counts,
        })
    }

    pub fn format_containers(&self) -> String {
        let mut output = String::new();

        output.push_str("========================================\n");
        output.push_str("       UHDM 所有容器信息 (All Factories)\n");
        output.push_str("========================================\n\n");

        output.push_str(&format!("【1. Symbols】 - 共 {} 个\n", self.symbols.len()));
        for (i, sym) in self.symbols.iter().enumerate() {
            if !sym.is_empty() && !sym.starts_with('$') && sym.len() < 40 {
                output.push_str(&format!("  [{}] \"{}\"\n", i, sym));
            }
        }

        output.push_str("\n【2. Factories】\n");
        let mut sorted: Vec<_> = self.factory_counts.iter().collect();
        sorted.sort_by(|a, b| a.0.cmp(b.0));
        for (name, count) in sorted {
            output.push_str(&format!("  {}: {} 个\n", name, count));
        }

        output.push_str("\n========================================\n");

        output
    }

    pub fn format_design(&self) -> String {
        let mut output = String::new();

        output.push_str("========================================\n");
        output.push_str("         用户 Design 结构化输出\n");
        output.push_str("========================================\n\n");

        output.push_str("// User Modules\n");
        for (i, module) in self.modules.iter().enumerate() {
            output.push_str(&format!("module #{} = {};\n", i + 1, module));
        }

        output.push_str("\n========================================\n");

        output
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::path::Path;

    #[test]
    fn test_load_counter() {
        let uhdm_path = "/Users/hh/Documents/TideEDA/tests/uhdm_regression/counter/counter.uhdm";

        if !Path::new(uhdm_path).exists() {
            eprintln!("Skipping test because {} does not exist.", uhdm_path);
            return;
        }

        match UhdmDesign::load(uhdm_path) {
            Ok(design) => {
                println!("=== Containers ===");
                println!("{}", design.format_containers());
                println!("=== Design ===");
                println!("{}", design.format_design());
            }
            Err(e) => panic!("Failed to load UHDM: {:?}", e),
        }
    }

    #[test]
    fn test_raw_types() {
        use raw_types::*;
        assert_eq!(RAW_UHDM_DESIGN, 2096);
        assert_eq!(RAW_UHDM_MODULE_INST, 2228);
        assert_eq!(RAW_UHDM_OPERATION, 2250);
        assert_eq!(RAW_UHDM_CONSTANT, 2070);
        assert_eq!(RAW_UHDM_PORT, 2267);

        assert_eq!(raw_type_name(2096), "uhdmdesign");
        assert_eq!(raw_type_name(2228), "uhdmmodule_inst");
        assert_eq!(raw_type_name(2250), "uhdmoperation");
    }

    #[test]
    fn test_context_resolve() {
        let uhdm_path = "/Users/hh/Documents/TideEDA/tests/uhdm_regression/counter/counter.uhdm";

        if !Path::new(uhdm_path).exists() {
            eprintln!("Skipping test because {} does not exist.", uhdm_path);
            return;
        }

        let file = File::open(uhdm_path).unwrap();
        let mut reader = BufReader::new(file);
        let message_reader =
            serialize_packed::read_message(&mut reader, ReaderOptions::new()).unwrap();
        let root = message_reader
            .get_root::<uhdm_capnp::uhdm_root::Reader>()
            .unwrap();

        let ctx = UhdmContext::new(root);

        if let Ok(designs) = ctx.get_root().get_designs() {
            println!("Found {} designs", designs.len());
            for i in 0..designs.len() {
                let design = designs.get(i);
                let base = design.get_base();
                if let Ok(base) = base {
                    println!("Design uhdmId: {}", base.get_uhdm_id());
                }

                let name_id = design.get_vpi_name();
                if name_id > 0 {
                    if let Some(name) = ctx.symbol(name_id) {
                        println!("Design name: {}", name);
                    }
                }

                if let Ok(top_modules) = design.get_top_modules() {
                    println!("Top modules count: {}", top_modules.len());
                }
            }
        }
    }

    #[test]
    fn test_walker_output() {
        use crate::walker::{walk_all, walk_design_tree};

        let uhdm_path = "/Users/hh/Documents/TideEDA/tests/uhdm_regression/counter/counter.uhdm";

        if !Path::new(uhdm_path).exists() {
            eprintln!("Skipping test because {} does not exist.", uhdm_path);
            return;
        }

        let file = File::open(uhdm_path).unwrap();
        let mut reader = BufReader::new(file);
        let message_reader =
            serialize_packed::read_message(&mut reader, ReaderOptions::new()).unwrap();
        let root = message_reader
            .get_root::<uhdm_capnp::uhdm_root::Reader>()
            .unwrap();

        let ctx = UhdmContext::new(root);

        println!("========================================");
        println!("       Walker 遍历输出 (walk_all)");
        println!("========================================");
        let output = walk_all(&ctx);
        println!("{}", output);
        println!("========================================");

        println!("========================================");
        println!("       Tree 遍历输出 (walk_design_tree)");
        println!("========================================");
        let tree_output = walk_design_tree(&ctx);
        println!("{}", tree_output);
        println!("========================================");
    }
}
