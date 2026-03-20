use std::env;
use std::path::Path;

use uhdm_rs::UhdmDesign;

fn main() {
    // 获取命令行参数
    let args: Vec<String> = env::args().collect();
    
    if args.len() < 2 {
        eprintln!("Usage: tideeda <uhdm_file> [command]");
        eprintln!("Commands:");
        eprintln!("  summary     - 打印设计摘要（默认）");
        eprintln!("  symbols     - 列出所有符号");
        eprintln!("  modules     - 列出所有模块");
        eprintln!("  factories   - 列出所有工厂计数");
        std::process::exit(1);
    }
    
    let uhdm_path = &args[1];
    let command = args.get(2).map(|s| s.as_str()).unwrap_or("summary");
    
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
            println!("- {} modules", design.modules.len());
            
            // 根据命令执行不同的操作
            match command {
                "summary" => print_summary(&design),
                "symbols" => list_symbols(&design),
                "modules" => list_modules(&design),
                "factories" => list_factories(&design),
                _ => {
                    eprintln!("Unknown command: {}", command);
                    eprintln!("Available commands: summary, symbols, modules, factories");
                    std::process::exit(1);
                }
            }
        },
        Err(e) => {
            eprintln!("Error loading UHDM file: {:?}", e);
            std::process::exit(1);
        }
    }
}

/// 打印设计摘要
fn print_summary(design: &UhdmDesign) {
    println!("Design Summary:");
    println!("===============");
    println!("Symbols: {}", design.symbols.len());
    println!("Modules: {}", design.modules.len());
    println!("Factory Types: {}", design.factory_counts.len());
}

/// 列出所有符号
fn list_symbols(design: &UhdmDesign) {
    println!("Symbols in Design:");
    println!("==================");
    
    for (idx, symbol) in design.symbols.iter().take(50).enumerate() {
        if !symbol.is_empty() {
            println!("  [{}] {}", idx, symbol);
        }
    }
    
    if design.symbols.len() > 50 {
        println!("  ... and {} more symbols", design.symbols.len() - 50);
    }
}

/// 列出所有模块
fn list_modules(design: &UhdmDesign) {
    println!("Modules in Design:");
    println!("==================");
    
    for (idx, module) in design.modules.iter().enumerate() {
        println!("  {}. {}", idx + 1, module);
    }
}

/// 列出所有工厂计数
fn list_factories(design: &UhdmDesign) {
    println!("Factory Counts:");
    println!("===============");
    
    let mut factories: Vec<_> = design.factory_counts.iter().collect();
    factories.sort_by(|a, b| b.1.cmp(a.1));
    
    for (name, count) in factories {
        println!("  {}: {}", name, count);
    }
}
