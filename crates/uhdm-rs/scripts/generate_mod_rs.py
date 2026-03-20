#!/usr/bin/env python3
"""
Generate mod.rs that flattens the module namespace.
Each module file contains 'pub mod name { ... }', so we re-export with 'pub use'.
"""

import os

def generate_mod_rs(schema_dir):
    mod_rs_path = os.path.join(schema_dir, "mod.rs")

    # Find all .rs files (excluding mod.rs)
    modules = []
    for filename in sorted(os.listdir(schema_dir)):
        if filename.endswith('.rs') and filename != 'mod.rs':
            mod_name = filename[:-3]  # Remove .rs
            modules.append(mod_name)

    with open(mod_rs_path, 'w') as f:
        f.write("// Auto-generated module declarations\n")
        f.write("// Each module is re-exported to flatten the namespace\n\n")

        for mod_name in modules:
            # Include the module file
            f.write(f"pub mod {mod_name};\n")

        f.write("\n// Re-export to flatten namespace: uhdm_capnp::always::Reader instead of uhdm_capnp::always::always::Reader\n")
        for mod_name in modules:
            # Re-export all contents from the nested module
            f.write(f"pub use {mod_name}::{mod_name}::*;\n")

    print(f"Generated {mod_rs_path} with {len(modules)} modules")

if __name__ == "__main__":
    schema_dir = "/Users/hh/Documents/TideEDA/crates/uhdm-rs/src/schema"
    generate_mod_rs(schema_dir)
