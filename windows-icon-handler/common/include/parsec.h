// Parsec Cloud (https://parsec.cloud) Copyright (c) AGPL-3.0 2016-present Scille SAS

#include <array>
#include <memory>
#include <string>
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

    inline SyncState is_member_file(const std::wstring& pwsz_path)
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
            return SyncState::NotSet;

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
            return SyncState::NotSet;

        auto entry_infos = std::ifstream(pwsz_path + L".__parsec_entry_info__");

        // Can't open file, stop there
        if (!entry_infos.good())
            return SyncState::NotSet;

        std::stringstream buffer;
        buffer << entry_infos.rdbuf();

        try
        {
            auto json_object = json::parse(buffer.str());
            return json_object["need_sync"].get<bool>() ? SyncState::Refresh : SyncState::Synced;
        }
        catch (json::exception&)
        {
            return SyncState::NotSet;
        }
    }
}
