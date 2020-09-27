# Parsec Cloud (https://parsec.cloud) Copyright (c) AGPLv3 2019 Scille SAS

import os
import re
import shutil
from datetime import datetime

from SCons.Platform.virtualenv import ImportVirtualenv
from SCons.Errors import UserError


EnsurePythonVersion(3, 6)
EnsureSConsVersion(4, 0)


def extract_version():
    # Hold my beer...
    gl = {}
    exec(open("parsec/_version.py").read(), gl)
    return gl["__version__"]


vars = Variables("custom.py")
# vars.Add(
#     EnumVariable(
#         "venv",
#         "Virtual to use",
#         "",
#         allowed_values=("x11-64", "x11-32", "windows-64", "windows-32", "osx-64"),
#     )
# )


env = Environment(
    variables=vars,
    tools=["default"],
    # ENV={**os.environ, "SCONS_ENABLE_VIRTUALENV": 1}
    # ENV = {'PATH' : os.environ['PATH']},
)
Help(vars.GenerateHelpText(env))
ImportVirtualenv(env)  # Python sub-command should be run within our current venv


### Load sub scons scripts ###


Export(env=env)
SConscript(
    [
        "parsec/core/gui/SConscript",
        # "tests/SConscript",
        # "packaging/pypi/SConscript",
    ]
)


### Top-level commands ###


sync_venv_cmd = env.Alias(target="sync_venv", source="$SYNC_VENV_CMD")


dev_cmd = env.Alias(
    target="dev",
    source=[
        "sync_venv",  # Ensure venv is up to date with dependencies
        "parsec",  # Build the project
    ],
    action="py.test tests",
)


### Define default target ###

# def _display_usage(env, target, source):
#     print("""Build or be built
# dev:
# generate_
# """)

# display_usage = env.Command(
#     target="display_usage",
#     source=[],
#     action=Action(_display_usage, cmdstr="")
# )
# env.AlwaysBuild(display_usage)
# env.Default(display_usage)

# env.Default("parsec")
# env.Alias("build", "parsec")
