// Parsec Cloud (https://parsec.cloud) Copyright (c) AGPL-3.0 2016-present Scille SAS

// RefreshIconHandler.cpp : Implementation of CRefreshIconHandler

#include "pch.h"
#include "RefreshIconHandler.h"

#include <array>
#include <cassert>
#include <memory>

#include <json/json.hpp>

// Json namespace shorthand
using json = nlohmann::json;

// Just a quick json test
static void test_json()
{
    auto ex1 = json::parse(R"(
      {
        "pi": 3.141,
        "happy": true
      }
    )");

    auto pi = ex1["pi"].get<double>();
    auto happy = ex1["happy"].get<bool>();

    assert(pi == 3.141);
    assert(happy);
}

// CRefreshIconHandler

HRESULT __stdcall CRefreshIconHandler::IsMemberOf(LPCWSTR pwszPath, DWORD dwAttrib)
{
    test_json();

    constexpr std::array<wchar_t, 26> DRIVES = {
        'A', 'B', 'C', 'D', 'E', 'F', 'G', 'H', 'I', 'J', 'K', 'L', 'M',
        'N', 'O', 'P', 'Q', 'R', 'S', 'T', 'U', 'V', 'W', 'X', 'Y', 'Z'
    };
    int drive_number = PathGetDriveNumberW(pwszPath);

    // Parsec supports drive from H to Z (included). If the drive letter is not
    // one of those, we can return FALSE immediately
    // A is 0, Z is 25, our drive should be between 7 and 25 included

    if (drive_number == -1 || drive_number < 7 || PathIsRootW(pwszPath))
        return S_FALSE;

    // Parsec write a registry key to set the icon for its workspaces.
    // We check if the key is present for the current drive.
    auto reg_key = std::make_unique<wchar_t[]>(MAX_PATH * 32);
    swprintf(
        reg_key.get(),
        MAX_PATH * 32,
        L"Software\\Classes\\Applications\\Explorer.exe\\Drives\\%c\\DefaultIcon",
        DRIVES[drive_number]
    );
    HKEY key = nullptr;

    if (RegOpenKeyEx(HKEY_CURRENT_USER, reg_key.get(), 0, KEY_READ, &key) != ERROR_SUCCESS)
        return S_FALSE;

    return S_OK;
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
