# Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS

import argparse
import pathlib
import sys

if __name__ == "__main__":
    parser = argparse.ArgumentParser()
    parser.add_argument(
        "-i", "--input", type=pathlib.Path, required=True, help="File to convert to Uint8Array"
    )
    parser.add_argument("--name", type=str, required=True, help="Name of the variable")
    parser.add_argument("-o", "--output", default=sys.stdout, help="Where to put the result")
    args = parser.parse_args()
    content = args.input.read_bytes()
    array = ", ".join([str(c) for c in content])
    result = f"""// Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS

/*
Generated automatically with {sys.argv[0]}
*/

const {args.name} = new Uint8Array([{array}]);

export default {args.name};
"""

    if args.output is sys.stdout:
        print(result)
    else:
        with open(args.output, "w+") as fd:
            fd.write(result)
