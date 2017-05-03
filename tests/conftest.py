def pytest_addoption(parser):
    parser.addoption("--run-google-drive", action="store_true",
                     help="run test on Google Drive")
    parser.addoption("--no-postgresql", action="store_true",
                     help="Don't run tests making use of PostgreSQL")
