// Parsec Cloud (https://parsec.cloud) Copyright (c) AGPL-3.0 2016-present Scille SAS

// CheckIconHandler.cpp : Implementation of CCheckIconHandler

#include "pch.h"
#include "CheckIconHandler.h"

#include <array>
#include <memory>
#include <cassert>

#include "parsec.h"

// CCheckIconHandler
static void log_result(parsec::SyncState state, std::wstring path)
{
    using parsec::SyncState;
    std::wofstream ofs("C:\\Users\\Corentin\\parsec.log", std::ios_base::app);
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

HRESULT __stdcall CCheckIconHandler::IsMemberOf(LPCWSTR pwszPath, DWORD dwAttrib)
{
    auto state = parsec::is_member_file(pwszPath);
    log_result(state, pwszPath);
    if (state == parsec::SyncState::Synced)
        return S_OK;

    return S_FALSE;
}

HRESULT __stdcall CCheckIconHandler::GetOverlayInfo(LPWSTR pwszIconFile, int cchMax, int* pIndex, DWORD* pdwFlags)
{
    // We use the icon present in dll resources
    GetModuleFileNameW(_AtlBaseModule.GetModuleInstance(), pwszIconFile, cchMax);
    *pdwFlags = ISIOI_ICONFILE | ISIOI_ICONINDEX; // We return an icon index
    *pIndex = 0;

    return S_OK;
}

HRESULT __stdcall CCheckIconHandler::GetPriority(int* pPriority)
{
    *pPriority = 50; // Set priority to a arbitrary value, not needed here
    return S_OK;
}
