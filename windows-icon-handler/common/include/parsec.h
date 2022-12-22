// Parsec Cloud (https://parsec.cloud) Copyright (c) AGPL-3.0 2016-present Scille SAS

#include <array>
#include <memory>
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

#ifdef _DEBUG
    inline std::ostream& operator<<(std::ofstream& os, SyncState state)
    {
        switch (state)
        {
        case SyncState::Synced:
            return os << "Synced";
        case SyncState::Refresh:
            return os << "Refresh";
        case SyncState::NotSet:
            return os << "NotSet";
        }

        // Unreachable
        return os;
    }

    template<typename T>
    concept Streamable = requires(std::ofstream& os, T a) {
        { os << a } -> std::convertible_to<std::ostream&>;
    };

    void log_impl(std::ofstream& ofs)
    {
        ofs << '\n';
    }

    template<typename T, typename... Args> requires Streamable<T>
    void log_impl(std::ofstream& ofs, T value, Args... args)
    {
       ofs << value << ' ';
       log_impl(ofs, args...);
    }

    template<typename T, typename... Args>
    void log(T x, Args... args)
    {
        // Replace username to debug:
        std::ofstream ofs("C:\\Users\\MyUser\\parsec.log");

        if (!ofs.good())
            return;

        log_impl(ofs, x, args...);
        ofs.flush();
    }

#define TRACE(...) ::parsec::log(__FILE__, __LINE__, __VA_ARGS__)
#else
#define TRACE(...) /* nothing */
#endif

    inline SyncState is_member_file(const std::wstring_view& pwsz_path)
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
            TRACE("Drive number is", drive_number);
            return SyncState::NotSet;
        }

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
        {
            TRACE("Call to `RegOpenKeyEx` failed, return `NotSet`");
            return SyncState::NotSet;
        }

        auto entry_infos = std::ifstream(std::format(L"{}.__parsec_entry_info__", pwsz_path));

        // Can't open file for some reason, stop there
        if (!entry_infos.good())
            return SyncState::NotSet;

        std::stringstream buffer;
        buffer << entry_infos.rdbuf();

        try
        {
            auto json_object = json::parse(buffer.str());

            TRACE(
                "Json parsing OK `is_sync` value",
                json_object["need_sync"].get<bool>(),
                "return value is",
                json_object["need_sync"].get<bool>() ? SyncState::Refresh : SyncState::Synced
            );

            return json_object["need_sync"].get<bool>() ? SyncState::Refresh : SyncState::Synced;
        }
#ifdef _DEBUG
        catch (json::exception& e)
#else
        catch (json::exception&)
#endif
        {
            TRACE("Json parsing failed. Reason:", e.what());
            return SyncState::NotSet;
        }
    }
}
