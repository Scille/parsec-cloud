// RefreshIconHandler.h : Declaration of the CRefreshIconHandler

#pragma once
#include "resource.h"       // main symbols



#include "refreshiconhandler_i.h"



#if defined(_WIN32_WCE) && !defined(_CE_DCOM) && !defined(_CE_ALLOW_SINGLE_THREADED_OBJECTS_IN_MTA)
#error "Single-threaded COM objects are not properly supported on Windows CE platform, such as the Windows Mobile platforms that do not include full DCOM support. Define _CE_ALLOW_SINGLE_THREADED_OBJECTS_IN_MTA to force ATL to support creating single-thread COM object's and allow use of it's single-threaded COM object implementations. The threading model in your rgs file was set to 'Free' as that is the only threading model supported in non DCOM Windows CE platforms."
#endif

using namespace ATL;


// CRefreshIconHandler

class ATL_NO_VTABLE CRefreshIconHandler :
	public CComObjectRootEx<CComSingleThreadModel>,
	public CComCoClass<CRefreshIconHandler, &CLSID_RefreshIconHandler>,
	public IDispatchImpl<IRefreshIconHandler, &IID_IRefreshIconHandler, &LIBID_refreshiconhandlerLib, /*wMajor =*/ 1, /*wMinor =*/ 0>,
    public IShellIconOverlayIdentifier
{
public:
	CRefreshIconHandler()
	{
	}

DECLARE_REGISTRY_RESOURCEID(107)


BEGIN_COM_MAP(CRefreshIconHandler)
	COM_INTERFACE_ENTRY(IRefreshIconHandler)
	COM_INTERFACE_ENTRY(IDispatch)
    COM_INTERFACE_ENTRY(IShellIconOverlayIdentifier)
END_COM_MAP()



	DECLARE_PROTECT_FINAL_CONSTRUCT()

	HRESULT FinalConstruct()
	{
		return S_OK;
	}

	void FinalRelease()
	{
	}

public:

    // Inherited via IShellIconOverlayIdentifier
    HRESULT __stdcall IsMemberOf(LPCWSTR pwszPath, DWORD dwAttrib) override;
    HRESULT __stdcall GetOverlayInfo(LPWSTR pwszIconFile, int cchMax, int* pIndex, DWORD* pdwFlags) override;
    HRESULT __stdcall GetPriority(int* pPriority) override;

};

OBJECT_ENTRY_AUTO(__uuidof(RefreshIconHandler), CRefreshIconHandler)
