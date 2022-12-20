// RefreshIconHandler.cpp : Implementation of CRefreshIconHandler

#include "pch.h"
#include "RefreshIconHandler.h"

#include "parsec.h"


// CRefreshIconHandler
static void log_result(parsec::SyncState state, std::wstring path)
{
    using parsec::SyncState;
    std::wofstream ofs("C:\\Users\\Corentin\\parsec-refresh.log", std::ios_base::app);
    ofs << path;

    switch (state)
    {
    case SyncState::Synced:
        ofs << L"  Synced\n";
        break;
    case SyncState::NotSet:
        ofs << L"  NotSet\n";
        break;
    case SyncState::Refresh:
        ofs << L"  Refresh\n";
        break;

    default:
        ofs << L"  ???\n";
        break;
    }

    ofs.flush();
}

HRESULT __stdcall CRefreshIconHandler::IsMemberOf(LPCWSTR pwszPath, DWORD dwAttrib)
{
    auto state = parsec::is_member_file(pwszPath);
    log_result(state, pwszPath);
    if (state == parsec::SyncState::Refresh)
        return S_OK;

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
