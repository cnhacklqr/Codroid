import os
from pathlib import Path
import shutil

working_dir = Path.cwd()


def build_rust_analyzer(build_command: str):
    rust_analyzer_root = working_dir.joinpath("rust-analyzer")
    os.chdir(rust_analyzer_root)
    os.system(build_command)
    os.chdir(working_dir)


def collect_rust_analyzer(arch: str):
    source = (
        working_dir.joinpath("rust-analyzer")
        .joinpath("target")
        .joinpath(arch)
        .joinpath("release")
        .joinpath("rust-analyzer")
    )
    target = working_dir.parent.joinpath("rust-analyzer-{}".format(arch))
    try:
        target.unlink()
    except Exception:
        pass
    shutil.copy2(src=source, dst=target)


# android 64-bit
build_rust_analyzer("cargo ndk -t arm64-v8a -t x86_64 build -r")
collect_rust_analyzer("aarch64-linux-android")
collect_rust_analyzer("x86_64-linux-android")

# linux x86_64
build_rust_analyzer("cargo build -r --target x86_64-unknown-linux-gnu")
collect_rust_analyzer("x86_64-unknown-linux-gnu")

# reduce elf's size
os.system("strip {}/*".format(working_dir))
