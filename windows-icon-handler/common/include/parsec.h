// Parsec Cloud (https://parsec.cloud) Copyright (c) AGPL-3.0 2016-present Scille SAS
#pragma once

#include <array>
#include <string_view>
#include <sstream>
#include <fstream>

#include <json/json.hpp>

#include <atlbase.h>

// Namespace shorthand
using json = nlohmann::json;

namespace parsec
{
    enum class SyncState {
        Synced,
        Refresh,
        NotSet
    };

    inline SyncState get_file_state(const std::wstring_view& pwsz_path)
    {
        constexpr std::array<wchar_t, 26> DRIVES = {
            'A', 'B', 'C', 'D', 'E', 'F', 'G', 'H', 'I', 'J', 'K', 'L', 'M',
            'N', 'O', 'P', 'Q', 'R', 'S', 'T', 'U', 'V', 'W', 'X', 'Y', 'Z'
        };

        int drive_number = PathGetDriveNumberW(pwsz_path.data());

        //// Parsec supports drive from H to Z (included). If the drive letter is not
        //// one of those, we can return FALSE immediately
        //// A is 0, Z is 25, our drive should be between 7 and 25 included
        if (drive_number == -1 || drive_number < 7 || PathIsRootW(pwsz_path.data()))
        {
            return SyncState::NotSet;
        }

        // Parsec write a registry key to set the icon for its workspaces.
        // We check if the key is present for the current drive.
        std::wstringstream ss;
        ss << L"Software\\Classes\\Applications\\Explorer.exe\\Drives\\"
            << DRIVES[drive_number]
            << L"\\DefaultIcon";

        HKEY key = nullptr;
        if (RegOpenKeyEx(HKEY_CURRENT_USER, ss.str().c_str(), 0, KEY_READ, &key) != ERROR_SUCCESS)
        {
            return SyncState::NotSet;
        }

        auto entry_infos = std::ifstream(std::format(L"{}.__parsec_entry_info__", pwsz_path));

        // Can't open file for some reason, stop there
        if (!entry_infos.good())
        {
            return SyncState::NotSet;
        }

        try
        {
            auto json_object = json::parse(entry_infos);
            return json_object["need_sync"].get<bool>() ? SyncState::Refresh : SyncState::Synced;
        }
        catch (json::exception&)
        {
            return SyncState::NotSet;
        }
    }
}
