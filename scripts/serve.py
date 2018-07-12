#!/usr/bin/env python3

import sys
import os
from itertools import chain
from http import server
from pathlib import Path

# determine relevant pathes based on script location
project_dir = Path(os.path.realpath(os.path.join(os.path.dirname(sys.argv[0]), '..')))
static_dir = project_dir.joinpath('static')
deploy_dir = project_dir.joinpath('target/deploy')
build_dir = project_dir.joinpath('target/wasm32-unknown-unknown/release')

# symlink build output to deploy dir
deploy_dir.mkdir(parents=True, exist_ok=True)
files = chain(static_dir.glob('*'),build_dir.glob('*.wasm'), build_dir.glob('*.js'))
for f in files:
    try:
        os.symlink(src=f, dst=deploy_dir.joinpath(os.path.basename(f)))
    except FileExistsError:
        pass

# serve these files from deploy directory
print('Serving at http://localhost:8888')

class NoCacheHandler(server.SimpleHTTPRequestHandler):
    def end_headers(self):
        self.send_my_headers()
        server.SimpleHTTPRequestHandler.end_headers(self)

    def send_my_headers(self):
        self.send_header("Cache-Control", "no-cache, no-store, must-revalidate")
        self.send_header("Pragma", "no-cache")
        self.send_header("Expires", "0")

os.chdir(deploy_dir)
NoCacheHandler.extensions_map['.wasm'] = 'application/wasm'
httpd = server.HTTPServer(('localhost', 8888), NoCacheHandler)
httpd.serve_forever()
