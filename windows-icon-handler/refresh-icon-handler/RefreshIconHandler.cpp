// RefreshIconHandler.cpp : Implementation of CRefreshIconHandler

#include "pch.h"
#include "RefreshIconHandler.h"

#include "parsec.h"


// CRefreshIconHandler


HRESULT __stdcall CRefreshIconHandler::IsMemberOf(LPCWSTR pwszPath, DWORD dwAttrib)
{
    auto state = parsec::get_file_state(pwszPath);
    if (state == parsec::SyncState::Refresh)
    {
        return S_OK;
    }

    return S_FALSE;
}

HRESULT __stdcall CRefreshIconHandler::GetOverlayInfo(LPWSTR pwszIconFile, int cchMax, int* pIndex, DWORD* pdwFlags)
{
    // We use the icon present in dll resources
    GetModuleFileNameW(_AtlBaseModule.GetModuleInstance(), pwszIconFile, cchMax);
    *pdwFlags = ISIOI_ICONFILE | ISIOI_ICONINDEX; // We return an icon index
    *pIndex = 0;

    return S_OK;
}

HRESULT __stdcall CRefreshIconHandler::GetPriority(int* pPriority)
{
    *pPriority = 50; // Set priority to a arbitrary value, not needed here
    return S_OK;
}
