import subprocess
import logging

log = logging.getLogger("build")

YELLOW_FG = "\x1b[33m"
DEFAULT_FG = "\x1b[39m"

logging.basicConfig(
    level=logging.DEBUG, format=f"{YELLOW_FG}[%(levelname)s] %(message)s{DEFAULT_FG}"
)


def run(cmd, **kwargs):
    log.debug(f">>> {cmd}")
    ret = subprocess.run(cmd.split(), check=True, **kwargs)
    return ret


def build(setup_kargs):
    log.debug(f"setup_kargs: {str(setup_kargs)}")

    run("python --version")
    run("pip freeze")
    run("python misc/generate_pyqt.py")
    # run("maturin develop --release")
    pass


if __name__ == "__main__":
    log.debug("launching build script in manual")
    build({})
