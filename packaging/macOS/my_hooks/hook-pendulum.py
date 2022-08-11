# Parsec Cloud (https://parsec.cloud) Copyright (c) AGPL-3.0 2016-present Scille SAS

from PyInstaller.utils.hooks import collect_data_files, collect_submodules


# Pendulum checks for locale modules via os.path.exists before import.
# If the include_py_files option is turned off, this check fails, pendulum
# will raise a ValueError.
datas = collect_data_files("pendulum.locales", include_py_files=True)
hiddenimports = collect_submodules("pendulum.locales")
