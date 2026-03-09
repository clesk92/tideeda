use std::env;
use std::path::Path;

use uhdm_rs::UhdmDesign;
use hir::Design as HirDesign;

fn main() {
    // 获取命令行参数
    let args: Vec<String> = env::args().collect();
    
    if args.len() != 2 {
        eprintln!("Usage: tideeda <uhdm_file>");
        std::process::exit(1);
    }
    
    let uhdm_path = &args[1];
    
    // 检查文件是否存在
    if !Path::new(uhdm_path).exists() {
        eprintln!("Error: File {} does not exist", uhdm_path);
        std::process::exit(1);
    }
    
    println!("Loading UHDM file: {}", uhdm_path);
    
    // 加载UHDM文件
    match UhdmDesign::load(uhdm_path) {
        Ok(design) => {
            println!("Successfully loaded design with:");
            println!("- {} symbols", design.symbols.len());
            println!("- {} module instances", design.module_instances.len());
            
            for (i, instance) in design.module_instances.iter().enumerate() {
                println!("  {}. Module: {}", i + 1, instance.name);
            }
            
            // 构建HIR
            match HirDesign::from_uhdm(uhdm_path) {
                Ok(hir_design) => {
                    println!("\nBuilt HIR with:");
                    println!("- {} modules", hir_design.modules.len());
                    
                    for (i, module) in hir_design.modules.iter().enumerate() {
                        println!("  {}. Module: {} ({} nets, {} ports)", 
                                 i + 1, module.name, module.nets.len(), module.ports.len());
                    }
                },
                Err(e) => {
                    eprintln!("Error building HIR: {:?}", e);
                }
            }
        },
        Err(e) => {
            eprintln!("Error loading UHDM file: {:?}", e);
            std::process::exit(1);
        }
    }
}
