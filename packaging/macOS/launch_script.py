import parsec.cli
import sys

sys.argv = [sys.argv[0], "core", "gui", *sys.argv[1:]]
parsec.cli.cli.main()
