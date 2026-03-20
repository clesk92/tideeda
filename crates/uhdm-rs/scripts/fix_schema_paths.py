#!/usr/bin/env python3
"""
Fix schema module paths in generated files.
Replace 'crate::uhdm_capnp::' with 'crate::schema::'
"""

import os
import re
from pathlib import Path

def fix_schema_paths(schema_dir):
    """Fix paths in all .rs files in schema directory"""

    for filename in os.listdir(schema_dir):
        if not filename.endswith('.rs'):
            continue

        filepath = os.path.join(schema_dir, filename)

        with open(filepath, 'r') as f:
            content = f.read()

        # Replace crate::uhdm_capnp:: with crate::schema::
        new_content = re.sub(r'crate::uhdm_capnp::', 'crate::schema::', content)

        if content != new_content:
            with open(filepath, 'w') as f:
                f.write(new_content)
            print(f"Fixed: {filename}")

if __name__ == "__main__":
    schema_dir = "/Users/hh/Documents/TideEDA/crates/uhdm-rs/src/schema"
    fix_schema_paths(schema_dir)
    print("Done!")
