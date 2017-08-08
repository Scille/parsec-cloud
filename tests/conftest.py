import sys


assert sys.version_info >= (3, 5, 3), 'Minimal Python version supported: 3.5.3'

def pytest_addoption(parser):
    parser.addoption("--run-google-drive", action="store_true",
                     help="run test on Google Drive")
    parser.addoption("--no-postgresql", action="store_true",
                     help="Don't run tests making use of PostgreSQL")
