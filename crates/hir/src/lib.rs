use uhdm_rs::UhdmDesign;
use thiserror::Error;
// use std::collections::HashMap;

#[derive(Error, Debug)]
pub enum HirError {
    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),
    #[error("UHDM error: {0}")]
    Uhdm(#[from] uhdm_rs::UhdmError),
    #[error("Schema error: {0}")]
    Schema(String),
    #[error("Failed to build HIR: {0}")]
    BuildError(String),
    #[error("No modules found in design")]
    NoModulesFound,
}

// === HIR 定义 ===

#[derive(Debug)]
pub struct Design {
    pub modules: Vec<Module>,
}

#[derive(Debug)]
pub struct Module {
    pub name: String,
    pub nets: Vec<Net>,
    pub ports: Vec<Port>,
}

#[derive(Debug)]
pub struct Net {
    pub name: String,
    pub width: u32,
}

#[derive(Debug)]
pub struct Port {
    pub name: String,
    pub direction: Direction,
}

#[derive(Debug)]
pub enum Direction {
    Input,
    Output,
    Inout,
    Unknown,
}

impl Design {
    pub fn from_uhdm(path: &str) -> Result<Self, HirError> {
        // 使用uhdm-rs的UhdmDesign::load方法加载UHDM文件
        let uhdm_design = UhdmDesign::load(path)?;
        
        // 从UhdmDesign构建HIR Design
        let mut modules = Vec::new();
        
        if uhdm_design.module_instances.is_empty() {
            return Err(HirError::NoModulesFound);
        }
        
        for instance in &uhdm_design.module_instances {
            // 创建模块，暂时只包含名称，后续需要添加更多信息
            let module = Module {
                name: instance.name.clone(),
                nets: Vec::new(),
                ports: Vec::new(),
            };
            modules.push(module);
        }
        
        if modules.is_empty() {
            return Err(HirError::BuildError("Failed to build any modules".to_string()));
        }
        
        Ok(Design { modules })
    }
}
