#!/usr/bin/env python3
"""
Generate mod.rs from actual file names in schema directory.
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
        f.write("// Auto-generated module declarations\n\n")
        for mod_name in modules:
            f.write(f"pub mod {mod_name};\n")

    print(f"Generated {mod_rs_path} with {len(modules)} modules")

if __name__ == "__main__":
    schema_dir = "/Users/hh/Documents/TideEDA/crates/uhdm-rs/src/schema"
    generate_mod_rs(schema_dir)
