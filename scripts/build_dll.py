"""
MyDiskly Build Tool - Compile disktrex.dll
Author: Darkostepbro
Repository: https://github.com/darkostepbro/mydiskly-desktop

Compiles src/disktrex_engine.py into an optimized disktrex.dll library.
"""

import os
import sys
import py_compile
import zipfile
import tempfile
import shutil

REPO_ROOT = os.path.dirname(os.path.dirname(os.path.abspath(__file__)))
SRC_ENGINE = os.path.join(REPO_ROOT, "src", "disktrex_engine.py")
OUTPUT_DLL = os.path.join(REPO_ROOT, "disktrex.dll")

def compile_engine(output_path=OUTPUT_DLL):
    print(f"[1/3] Compiling {SRC_ENGINE} to optimized bytecode...")
    temp_dir = tempfile.mkdtemp(prefix="mydiskly_build_")
    try:
        pyc_path = os.path.join(temp_dir, "disktrex.pyc")
        init_py = os.path.join(temp_dir, "__init__.py")
        init_pyc = os.path.join(temp_dir, "__init__.pyc")

        # Compile disktrex_engine.py -> disktrex.pyc
        py_compile.compile(SRC_ENGINE, cfile=pyc_path, optimize=2)

        # Create __init__.py and compile
        with open(init_py, "w", encoding="utf-8") as f:
            f.write("# disktrex package\nfrom .disktrex import *\n")
        py_compile.compile(init_py, cfile=init_pyc, optimize=2)

        print(f"[2/3] Packing bytecode into binary library container: {output_path}...")
        os.makedirs(os.path.dirname(os.path.abspath(output_path)), exist_ok=True)
        with zipfile.ZipFile(output_path, "w", compression=zipfile.ZIP_DEFLATED) as z:
            z.write(pyc_path, arcname="disktrex.pyc")
            z.write(init_pyc, arcname="__init__.pyc")

        sz = os.path.getsize(output_path)
        print(f"[3/3] Successfully generated {output_path} ({sz:,} bytes)")
        return output_path
    finally:
        shutil.rmtree(temp_dir, ignore_errors=True)

if __name__ == "__main__":
    out = sys.argv[1] if len(sys.argv) > 1 else OUTPUT_DLL
    compile_engine(out)
