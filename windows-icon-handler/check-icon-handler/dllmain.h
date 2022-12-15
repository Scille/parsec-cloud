// Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 (eventually AGPL-3.0) 2016-present Scille SAS

// dllmain.h : Declaration of module class.

class CwindowsiconhandlerModule : public ATL::CAtlDllModuleT< CwindowsiconhandlerModule >
{
public :
	DECLARE_LIBID(LIBID_windowsiconhandlerLib)
	DECLARE_REGISTRY_APPID_RESOURCEID(IDR_WINDOWSICONHANDLER, "{78c79238-9970-4445-8ac8-2b9a8b5de7d4}")
};

extern class CwindowsiconhandlerModule _AtlModule;
