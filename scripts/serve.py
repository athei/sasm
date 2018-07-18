#!/usr/bin/env python3
import gather
import os
from http import server

# gather all files to run dir
gather.gather()

# serve these files from run directory
print('Serving at http://localhost:8888')

class NoCacheHandler(server.SimpleHTTPRequestHandler):
    def end_headers(self):
        self.send_my_headers()
        server.SimpleHTTPRequestHandler.end_headers(self)

    def send_my_headers(self):
        self.send_header("Cache-Control", "no-cache, no-store, must-revalidate")
        self.send_header("Pragma", "no-cache")
        self.send_header("Expires", "0")

os.chdir(gather.run_dir)
NoCacheHandler.extensions_map['.wasm'] = 'application/wasm'
httpd = server.HTTPServer(('localhost', 8888), NoCacheHandler)
httpd.serve_forever()
