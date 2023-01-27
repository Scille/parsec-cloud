// Parsec Cloud (https://parsec.cloud) Copyright (c) AGPL-3.0 2016-present Scille SAS

// dllmain.cpp : Implementation of DllMain.

#include "pch.h"
#include "framework.h"
#include "resource.h"
#include "checkiconhandler_i.h"
#include "dllmain.h"

CwindowsiconhandlerModule _AtlModule;

// DLL Entry Point
extern "C" BOOL WINAPI DllMain(HINSTANCE hInstance, DWORD dwReason, LPVOID lpReserved)
{
	hInstance;
	return _AtlModule.DllMain(dwReason, lpReserved);
}
