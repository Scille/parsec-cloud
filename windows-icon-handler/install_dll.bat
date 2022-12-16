:: Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 (eventually AGPL-3.0) 2016-present Scille SAS

msbuild -maxCpuCount -property:Configuration=Release .\check-icon-handler\check-icon-handler.vcxproj
msbuild -maxCpuCount -property:Configuration=Release .\refresh-icon-handler\refresh-icon-handler.vcxproj
