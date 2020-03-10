// Parsec Cloud (https://parsec.cloud) Copyright (c) AGPLv3 2019 Scille SAS

#define PY_SSIZE_T_CLEAN
#include <Python.h>
#include <wchar.h>

int main(int argc, char *argv[])
{
    wchar_t** _argv = PyMem_Malloc(sizeof(wchar_t*)*argc);
    for (int i = 0; i < argc; i++) {
      wchar_t* arg = Py_DecodeLocale(argv[i], NULL);
        if (arg == NULL) {
            fprintf(stderr, "Fatal error: cannot decode argv\n");
            exit(1);
        }
      _argv[i] = arg;
    }

    Py_SetProgramName(_argv[0]);  /* optional but recommended */
    Py_Initialize();
    PySys_SetArgv(argc, _argv);

    PyRun_SimpleString(
        "import os, sys\n"
        "from parsec.cli import cli\n"
        "os.environ['SENTRY_URL'] = 'https://863e60bbef39406896d2b7a5dbd491bb@sentry.io/1212848'\n"
        "os.makedirs(os.path.expandvars('%APPDATA%\\\\parsec'), exist_ok=True)\n"
        "log_file = os.path.expandvars('%APPDATA%\\\\parsec\\\\parsec-core.log')\n"
        "args = ['core', 'gui', '--log-level', 'INFO', '--log-file', log_file, ' '.join(sys.argv[1:])]\n"
        "cli(args)\n"
    );

    for (int i = 0; i < argc; i++) {
        PyMem_RawFree(_argv[i]);
    }
    PyMem_RawFree(_argv);

    if (Py_FinalizeEx() < 0) {
        exit(120);
    }
    return 0;
}
