#!/usr/bin/env python3
import sys
import os
from itertools import chain
from pathlib import Path

project_dir = Path(os.path.realpath(os.path.join(os.path.dirname(sys.argv[0]), '..')))
static_dir = project_dir.joinpath('static')
run_dir = project_dir.joinpath('target/run')
deploy_dir = project_dir.joinpath('target/deploy')
build_dir = project_dir.joinpath('target/wasm32-unknown-unknown/release')

def gather():
    # symlink build output to run dir
    run_dir.mkdir(parents=True, exist_ok=True)
    files = chain(static_dir.glob('*'),build_dir.glob('*.wasm'), build_dir.glob('*.js'))
    for f in files:
        try:
            os.symlink(src=f, dst=run_dir.joinpath(os.path.basename(f)))
        except FileExistsError:
            pass

if __name__ == '__main__':
    gather()
