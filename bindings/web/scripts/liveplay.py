#! /usr/bin/env python
# Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS


import subprocess
import sys
from pathlib import Path

BASEDIR = Path(__file__).parent
PKG_DIR = BASEDIR / "../pkg"


INDEX_HTML_SRC = """<!DOCTYPE html>
<html>
  <head>
    <meta content="text/html;charset=utf-8" http-equiv="Content-Type"/>
  </head>
  <body>
    <script src="libparsec_bindings_web.js" type="module"></script>
    <script type="module">
        console.log("loading libparsec...");
        import initModule, * as libparsec from './libparsec_bindings_web.js';
        await initModule();
        libparsec.initLogger("trace");
        window.libparsec = libparsec;
        console.log("libparsec is loaded !");
    </script>
    <p>Open you browser console and start with e.g.
    <code>libparsec.listAvailableDevices(&quot;/foo&quot;)</code></p>
  </body>
</html>
"""


if __name__ == "__main__":
    if not PKG_DIR.exists():
        raise SystemExit(
            f"{PKG_DIR} doesn't exist, you should build the bindings with `make.py` first !"
        )
    print(f"Creating {PKG_DIR}/index.html...")
    (PKG_DIR / "index.html").write_text(INDEX_HTML_SRC)

    subprocess.check_call([sys.executable, "-m", "http.server", "8000"], cwd=PKG_DIR)
