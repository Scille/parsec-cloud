// Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 (eventually AGPL-3.0) 2016-present Scille SAS

// CheckIconHandler.h : Declaration of the CCheckIconHandler

#pragma once
#include "resource.h"       // main symbols



#include "checkiconhandler_i.h"



#if defined(_WIN32_WCE) && !defined(_CE_DCOM) && !defined(_CE_ALLOW_SINGLE_THREADED_OBJECTS_IN_MTA)
#error "Single-threaded COM objects are not properly supported on Windows CE platform, such as the Windows Mobile platforms that do not include full DCOM support. Define _CE_ALLOW_SINGLE_THREADED_OBJECTS_IN_MTA to force ATL to support creating single-thread COM object's and allow use of it's single-threaded COM object implementations. The threading model in your rgs file was set to 'Free' as that is the only threading model supported in non DCOM Windows CE platforms."
#endif

using namespace ATL;


// CCheckIconHandler

class ATL_NO_VTABLE CCheckIconHandler :
	public CComObjectRootEx<CComSingleThreadModel>,
	public CComCoClass<CCheckIconHandler, &CLSID_CheckIconHandler>,
	public IDispatchImpl<ICheckIconHandler, &IID_ICheckIconHandler, &LIBID_windowsiconhandlerLib, /*wMajor =*/ 1, /*wMinor =*/ 0>,
    public IShellIconOverlayIdentifier
{
public:
	CCheckIconHandler()
	{
	}

DECLARE_REGISTRY_RESOURCEID(106)


BEGIN_COM_MAP(CCheckIconHandler)
	COM_INTERFACE_ENTRY(ICheckIconHandler)
	COM_INTERFACE_ENTRY(IDispatch)
    COM_INTERFACE_ENTRY(IShellIconOverlayIdentifier)
END_COM_MAP()



	DECLARE_PROTECT_FINAL_CONSTRUCT()

	inline HRESULT FinalConstruct()
	{
		return S_OK;
	}

	inline void FinalRelease()
	{
	}

public:

    // Inherited via IShellIconOverlayIdentifier
    HRESULT __stdcall IsMemberOf(LPCWSTR pwszPath, DWORD dwAttrib) override;
    HRESULT __stdcall GetOverlayInfo(LPWSTR pwszIconFile, int cchMax, int* pIndex, DWORD* pdwFlags) override;
    HRESULT __stdcall GetPriority(int* pPriority) override;

};

OBJECT_ENTRY_AUTO(__uuidof(CheckIconHandler), CCheckIconHandler)
