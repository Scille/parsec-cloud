// Parsec Cloud (https://parsec.cloud) Copyright (c) AGPL-3.0 2016-present Scille SAS
#pragma once

#include <array>
#include <string_view>
#include <sstream>
#include <fstream>
#include <format>

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
