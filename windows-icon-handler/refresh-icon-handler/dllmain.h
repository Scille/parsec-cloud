// Parsec Cloud (https://parsec.cloud) Copyright (c) AGPL-3.0 2016-present Scille SAS

// dllmain.h : Declaration of module class.

class CrefreshiconhandlerModule : public ATL::CAtlDllModuleT< CrefreshiconhandlerModule >
{
public :
	DECLARE_LIBID(LIBID_refreshiconhandlerLib)
	DECLARE_REGISTRY_APPID_RESOURCEID(IDR_REFRESHICONHANDLER, "{b168b5a2-1328-4d06-a4e4-b2865b610d72}")
};

extern class CrefreshiconhandlerModule _AtlModule;
