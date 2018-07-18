#!/usr/bin/env python3
import gather
import os
import subprocess
import shutil
from itertools import chain

# these files are passed through the closure compiler
OPTIMIZE_JS = ['sasm.js']

# gather files in run dir
gather.gather()

# Copy (optimized files) to run dir
shutil.rmtree(gather.deploy_dir)
gather.deploy_dir.mkdir(parents=True)
files = gather.run_dir.glob('*')
for f in files:
    name = os.path.basename(f)
    out = gather.deploy_dir.joinpath(name)
    if name in OPTIMIZE_JS:
        subprocess.run(['closure-compiler', '--js={}'.format(f), '--js_output_file={}'.format(out)])
    else:
        shutil.copyfile(f, out)

# Compress files for webserver
files = gather.deploy_dir.glob('*')
for f in files:
    subprocess.run(['gzip', '--keep', '--best', f])
    subprocess.run(['brotli', '--keep', '--best', f])




