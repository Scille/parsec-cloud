// Parsec Cloud (https://parsec.cloud) Copyright (c) AGPL-3.0 2016-present Scille SAS

// CheckIconHandler.cpp : Implementation of CCheckIconHandler

#include "pch.h"
#include "CheckIconHandler.h"

#include <array>
#include <memory>
#include <cassert>

#include "parsec.h"

// CCheckIconHandler


HRESULT __stdcall CCheckIconHandler::IsMemberOf(LPCWSTR pwszPath, DWORD dwAttrib)
{
    auto state = parsec::get_file_state(pwszPath);
    if (state == parsec::SyncState::Synced)
    {
        return S_OK;
    }

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
