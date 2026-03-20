#!/usr/bin/env python3
"""
Split uhdm_capnp.rs into multiple files by module.
Each module gets its own file in the schema/ directory.
Removes nested pub mod declarations.
"""

import re
import os
from pathlib import Path

def split_uhdm_capnp(input_file, output_dir):
    # Read the entire file
    with open(input_file, 'r') as f:
        content = f.read()

    # Create output directory
    os.makedirs(output_dir, exist_ok=True)

    # Find all module definitions
    # Pattern: pub mod module_name { ... }
    module_pattern = r'^pub mod ([a-zA-Z_]+) \{'

    # Find all module positions
    modules = []
    for match in re.finditer(module_pattern, content, re.MULTILINE):
        mod_name = match.group(1)
        start_pos = match.start()
        modules.append((mod_name, start_pos))

    print(f"Found {len(modules)} modules")

    # Sort by position
    modules.sort(key=lambda x: x[1])

    # Extract each module
    mod_files = []
    for i, (mod_name, start_pos) in enumerate(modules):
        # Find the end of this module (start of next module or end of file)
        if i < len(modules) - 1:
            end_pos = modules[i + 1][1]
        else:
            end_pos = len(content)

        # Extract module content
        mod_content = content[start_pos:end_pos]

        # Remove the outer "pub mod name {" and matching "}"
        # Find the first "{" and remove it
        first_brace = mod_content.find('{')
        if first_brace != -1:
            mod_content = mod_content[first_brace + 1:]

        # Remove trailing "}" at the end (the module's closing brace)
        # Count braces to find the matching one
        brace_count = 1
        end_idx = len(mod_content)
        for idx, char in enumerate(mod_content):
            if char == '{':
                brace_count += 1
            elif char == '}':
                brace_count -= 1
                if brace_count == 0:
                    end_idx = idx
                    break

        mod_content = mod_content[:end_idx]

        # Write to file
        mod_file = os.path.join(output_dir, f"{mod_name}.rs")
        with open(mod_file, 'w') as f:
            f.write(mod_content)

        mod_files.append((mod_name, mod_file))
        print(f"  {mod_name} -> {mod_file}")

    # Create mod.rs to re-export all modules
    mod_rs_path = os.path.join(output_dir, "mod.rs")
    with open(mod_rs_path, 'w') as f:
        f.write("// Auto-generated module declarations\n\n")
        for mod_name, _ in mod_files:
            f.write(f"pub mod {mod_name};\n")

    print(f"\nCreated {mod_rs_path}")

    # Copy original file to reference directory
    ref_dir = os.path.join(os.path.dirname(output_dir), "reference")
    os.makedirs(ref_dir, exist_ok=True)
    ref_file = os.path.join(ref_dir, "uhdm_capnp_original.rs")

    # Copy content before first module (imports and common code)
    first_mod_pos = modules[0][1] if modules else 0
    header_content = content[:first_mod_pos]

    with open(ref_file, 'w') as f:
        f.write(header_content)
        f.write("\n// ... modules split into separate files ...\n")

    print(f"\nSaved header to {ref_file}")

if __name__ == "__main__":
    input_file = "/Users/hh/Documents/TideEDA/target/debug/build/uhdm-rs-4bf8e79a6dda49ff/out/schema/uhdm_capnp.rs"
    output_dir = "/Users/hh/Documents/TideEDA/crates/uhdm-rs/src/schema"

    split_uhdm_capnp(input_file, output_dir)
