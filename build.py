import subprocess


def display(line: str):
    YELLOW_FG = "\x1b[33m"
    DEFAULT_FG = "\x1b[39m"

    # Flush is required to prevent mixing with the output of sub-command with the output of the script
    print(f"{YELLOW_FG}{line}{DEFAULT_FG}", flush=True)


def run(cmd, **kwargs):
    display(f">>> {cmd}")
    ret = subprocess.run(cmd.split(), check=True, **kwargs)
    return ret


def build(setup_kargs):
    display(f"setup_kargs: {str(setup_kargs)}")

    run("python --version")
    run("pip freeze")
    run("python misc/generate_pyqt.py")
    run("maturin develop --release")
    pass


if __name__ == "__main__":
    display("manual")
    build({})
