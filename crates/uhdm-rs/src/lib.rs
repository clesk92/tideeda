/// UHDM Capnp schema生成的代码
pub mod uhdm_capnp {
    include!(concat!(env!("OUT_DIR"), "/schema/uhdm_capnp.rs"));
}

/// 重新导出capnp的必要类型，避免其他crates直接依赖capnp
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
    #[error("UTF-8 error: {0}")]
    Utf8(#[from] std::str::Utf8Error),
    #[error("Invalid UHDM file format")]
    InvalidFormat,
    #[error("Failed to get root node: {0}")]
    RootNodeError(String),
    #[error("Failed to read symbols: {0}")]
    SymbolsError(String),
    #[error("Failed to read module instances: {0}")]
    ModuleInstancesError(String),
}

/// 模块实例结构体
#[derive(Debug)]
pub struct ModuleInstance {
    /// 模块名称
    pub name: String,
}

/// UHDM设计的结构体，包含设计的完整信息
#[derive(Debug)]
pub struct UhdmDesign {
    /// 符号表，用于解析ID到字符串的映射
    pub symbols: Vec<String>,
    /// 模块实例列表
    pub module_instances: Vec<ModuleInstance>,
}

impl UhdmDesign {
    /// 从UHDM文件加载设计
    pub fn load(path: &str) -> Result<Self, UhdmError> {
        let file = File::open(path)?;
        let mut reader = BufReader::new(file);
        
        // Try packed serialization first as it's common for UHDM files
        let message_reader = serialize_packed::read_message(&mut reader, ReaderOptions::new())
            .map_err(|_e| UhdmError::InvalidFormat)?;
        
        let root = message_reader.get_root::<uhdm_capnp::uhdm_root::Reader>()
            .map_err(|e| UhdmError::RootNodeError(e.to_string()))?;
        
        // 读取符号表
        let mut symbols = Vec::new();
        if let Ok(symbol_list) = root.get_symbols() {
            for symbol in symbol_list {
                // symbol 是 Result<Text::Reader, Error>
                // 我们尝试转换为 String，如果失败就用空字符串
                let s = match symbol {
                    Ok(text) => text.to_string().unwrap_or_default(),
                    Err(e) => {
                        eprintln!("Warning: Failed to read symbol: {}", e);
                        "".to_string()
                    },
                };
                symbols.push(s);
            }
        } else {
            return Err(UhdmError::SymbolsError("Failed to get symbols list".to_string()));
        }

        let mut module_instances = Vec::new();

        // 遍历 factory_moduleinst
        if let Ok(factory_modules) = root.get_factory_moduleinst() {
            for module in factory_modules {
                // Moduleinst -> Instance -> Scope -> vpiName
                // get_base() 返回基类 Reader
                match module.get_base() {
                    Ok(instance) => {
                        match instance.get_base() {
                            Ok(scope) => {
                                let name_id = scope.get_vpi_name();
                                
                                let name = if (name_id as usize) < symbols.len() {
                                    symbols[name_id as usize].clone()
                                } else {
                                    format!("UnknownSymbol({})", name_id)
                                };

                                let module_instance = ModuleInstance {
                                    name,
                                };

                                module_instances.push(module_instance);
                            },
                            Err(e) => {
                                eprintln!("Warning: Failed to get scope for module: {}", e);
                            },
                        }
                    },
                    Err(e) => {
                        eprintln!("Warning: Failed to get instance for module: {}", e);
                    },
                }
            }
        } else {
            return Err(UhdmError::ModuleInstancesError("Failed to get module instances list".to_string()));
        }
        
        Ok(UhdmDesign {
            symbols,
            module_instances,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::path::Path;

    #[test]
    fn test_load_uhdm() {
        // 使用 Surelog 默认生成的路径 (绝对路径)
        let uhdm_path = "/Users/hh/Documents/TideEDA/slpp_all/surelog.uhdm";
        
        // 确保文件存在（由之前的步骤生成）
        if !Path::new(uhdm_path).exists() {
            eprintln!("Skipping test because {} does not exist. Run 'surelog -parse -sverilog tests/test.sv' first.", uhdm_path);
            return;
        }

        match UhdmDesign::load(uhdm_path) {
            Ok(design) => {
                println!("Loaded design with {} symbols and {} module instances:", design.symbols.len(), design.module_instances.len());
                for instance in &design.module_instances {
                    println!(" - Module: {}", instance.name);
                }
                // 我们预期至少能读到一个模块
                assert!(!design.module_instances.is_empty(), "Should load at least one module");
            },
            Err(e) => panic!("Failed to load UHDM: {:?}", e),
        }
    }
}
