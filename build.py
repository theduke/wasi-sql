#! /usr/bin/env python3

import os
import subprocess

ROOT = os.path.dirname(os.path.realpath(__file__))

SCHEMA_PATH = os.path.join(ROOT, "schema", "sql_v1_alpha1.wit")

print(ROOT)

def build_rust():
    subprocess.run(["wit-bindgen", "rust-wasm", "--out-dir", os.path.join(ROOT, "rust", "wasi_sql", "src"), "--import",  SCHEMA_PATH], check=True)
    with open(os.path.join(ROOT, "rust", "wasi_sql", "src", "bindings.rs"), "r+") as f:
        contents = f.read().replace("mod sql_v1_alpha1", "pub mod sql_v1_alpha1")
        f.truncate(0)
        f.seek(0)
        f.write(contents)

    subprocess.run(["wit-bindgen", "wasmtime", "--out-dir", os.path.join(ROOT, "rust", "wasi_sql_wasmtime", "src"), "--export",  SCHEMA_PATH], check=True)
    # subprocess.run(["cargo", "fmt"], cwd=ROOT.join("rust"), check=True)

build_rust()
