
def pytest_addoption(parser):
    parser.addoption("--run-google-drive", action="store_true",
                     help="run test on Google Drive")
