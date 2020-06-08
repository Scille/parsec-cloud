# Greatly inspired by Deluge NSIS installer
# https://github.com/deluge-torrent/deluge/blob/develop/packaging/win32/Win32%20README.txt

!addplugindir nsis_plugins
!addincludedir nsis_plugins
!include "WordFunc.nsh"

# Script version; displayed when running the installer
!define PARSEC_INSTALLER_VERSION "1.0"

# Parsec program information
!define PROGRAM_NAME "Parsec"
!define PROGRAM_WEB_SITE "http://parsec.cloud"
!define APPGUID "6C37F945-7EFC-480A-A444-A6D44A3D107F"
!define OBSOLETE_MOUNTPOINT "$PROFILE\Parsec"

# Detect version from file
!define BUILD_DIR "build"
!searchparse /file ${BUILD_DIR}/BUILD.tmp `target = "` PARSEC_FREEZE_BUILD_DIR `"`
!ifndef PARSEC_FREEZE_BUILD_DIR
   !error "Cannot find freeze build directory"
!endif
!searchparse /file ${BUILD_DIR}/BUILD.tmp `parsec_version = "` PROGRAM_VERSION `"`
!ifndef PROGRAM_VERSION
   !error "Program Version Undefined"
!endif
!searchparse /file ${BUILD_DIR}/BUILD.tmp `platform = "` PROGRAM_PLATFORM `"`
!ifndef PROGRAM_PLATFORM
   !error "Program Platform Undefined"
!endif

# Python files generated
!define LICENSE_FILEPATH "${PARSEC_FREEZE_BUILD_DIR}\LICENSE.txt"
!define INSTALLER_FILENAME "parsec-${PROGRAM_VERSION}-${PROGRAM_PLATFORM}-setup.exe"

!define WINFSP_INSTALLER "winfsp-1.7.20038.msi"

# Set default compressor
SetCompressor /FINAL /SOLID lzma
SetCompressorDictSize 64

# --- Interface settings ---
# Modern User Interface 2
!include MUI2.nsh
# Installer
!define MUI_ICON "parsec.ico"
!define MUI_HEADERIMAGE
!define MUI_HEADERIMAGE_RIGHT
!define MUI_HEADERIMAGE_BITMAP "installer-top.bmp"
!define MUI_WELCOMEFINISHPAGE_BITMAP "installer-side.bmp"
!define MUI_COMPONENTSPAGE_SMALLDESC
!define MUI_ABORTWARNING
# Start Menu Folder Page Configuration
!define MUI_STARTMENUPAGE_DEFAULTFOLDER ${PROGRAM_NAME}
!define MUI_STARTMENUPAGE_REGISTRY_ROOT "HKCR"
!define MUI_STARTMENUPAGE_REGISTRY_KEY "Software\${PROGRAM_NAME}"
!define MUI_STARTMENUPAGE_REGISTRY_VALUENAME "Start Menu Folder"
# Uninstaller
!define MUI_UNICON "${NSISDIR}\Contrib\Graphics\Icons\modern-uninstall.ico"
!define MUI_HEADERIMAGE_UNBITMAP "installer-top.bmp"
!define MUI_WELCOMEFINISHPAGE_UNBITMAP "installer-side.bmp"
!define MUI_UNFINISHPAGE_NOAUTOCLOSE
# Add shortcut
!define MUI_FINISHPAGE_SHOWREADME ""
!define MUI_FINISHPAGE_SHOWREADME_NOTCHECKED
!define MUI_FINISHPAGE_SHOWREADME_TEXT "Create Desktop Shortcut"
!define MUI_FINISHPAGE_SHOWREADME_FUNCTION CreateDesktopShortcut
# Run Parsec after install
!define MUI_FINISHPAGE_RUN "$INSTDIR\parsec.exe"
!define MUI_FINISHPAGE_RUN_TEXT "Run Parsec"
!define MUI_FINISHPAGE_RUN_NOTCHECKED

# --- Start of Modern User Interface ---
Var StartMenuFolder

# Welcome, License
!insertmacro MUI_PAGE_WELCOME
!insertmacro MUI_PAGE_LICENSE ${LICENSE_FILEPATH}

# Skipping the components page
# !insertmacro MUI_PAGE_COMPONENTS

# Let the user select the installation directory
!insertmacro MUI_PAGE_DIRECTORY
!insertmacro MUI_PAGE_STARTMENU Application $StartMenuFolder

# Run installation
!insertmacro MUI_PAGE_INSTFILES
# Popup Message if VC Redist missing
# Page Custom VCRedistMessage
# Display 'finished' page
!insertmacro MUI_PAGE_FINISH
# Uninstaller pages
!insertmacro MUI_UNPAGE_INSTFILES
# Language files
!insertmacro MUI_LANGUAGE "English"


# --- Functions ---

Function checkParsecRunning
    check:
        System::Call 'kernel32::OpenMutex(i 0x100000, b 0, t "parsec-cloud") i .R0'
        IntCmp $R0 0 notRunning
            System::Call 'kernel32::CloseHandle(i $R0)'
            MessageBox MB_OKCANCEL|MB_ICONEXCLAMATION \
                "Parsec is running, please close it first.$\n$\n \
                Click `OK` to retry or `Cancel` to cancel this upgrade." \
                /SD IDCANCEL IDOK check
            Abort
    notRunning:
FunctionEnd

# Check for running Parsec instance.
Function .onInit
    Call checkParsecRunning

    ReadRegStr $R0 HKLM \
    "Software\Microsoft\Windows\CurrentVersion\Uninstall\${PROGRAM_NAME}" \
    "UninstallString"
    StrCmp $R0 "" done

    MessageBox MB_OKCANCEL|MB_ICONEXCLAMATION \
    "${PROGRAM_NAME} is already installed. $\n$\nClick `OK` to remove the \
    previous version or `Cancel` to cancel this upgrade." \
    /SD IDOK IDOK uninst
    Abort

    ;Run the uninstaller sequentially and silently
    ;https://nsis.sourceforge.io/Docs/Chapter3.html#installerusageuninstaller
    uninst:
      ClearErrors
      ExecWait '"$R0" /S _?=$INSTDIR'
    done:

FunctionEnd

Function un.onUninstSuccess
    HideWindow
    MessageBox MB_ICONINFORMATION|MB_OK "$(^Name) was successfully removed from your computer." /SD IDOK
FunctionEnd

Function un.onInit
    MessageBox MB_ICONQUESTION|MB_YESNO|MB_DEFBUTTON2 "Do you want to completely remove $(^Name)?" /SD IDYES IDYES +2
    Abort
FunctionEnd

Function CreateDesktopShortcut
    CreateShortCut "$DESKTOP\Parsec.lnk" "$INSTDIR\parsec.exe"
FunctionEnd

# # Test if Visual Studio Redistributables 2008 SP1 installed and returns -1 if none installed
# Function CheckVCRedist2008
#     Push $R0
#     ClearErrors
#     ReadRegDword $R0 HKLM "SOFTWARE\Microsoft\Windows\CurrentVersion\Uninstall\{FF66E9F6-83E7-3A3E-AF14-8DE9A809A6A4}" "Version"
#     IfErrors 0 +2
#         StrCpy $R0 "-1"
#
#     Push $R1
#     ClearErrors
#     ReadRegDword $R1 HKLM "SOFTWARE\Microsoft\Windows\CurrentVersion\Uninstall\{9BE518E6-ECC6-35A9-88E4-87755C07200F}" "Version"
#     IfErrors 0 VSRedistInstalled
#         StrCpy $R1 "-1"
#
#     StrCmp $R0 "-1" +3 0
#         Exch $R0
#         Goto VSRedistInstalled
#     StrCmp $R1 "-1" +3 0
#         Exch $R1
#         Goto VSRedistInstalled
#     # else
#         Push "-1"
#     VSRedistInstalled:
# FunctionEnd
#
# Function VCRedistMessage
#     Call CheckVCRedist2008
#     Pop $R0
#     StrCmp $R0 "-1" 0 end
#     MessageBox MB_YESNO|MB_ICONEXCLAMATION "Parsec requires an MSVC package to run \
#     but the recommended package does not appear to be installed:$\r$\n$\r$\n\
#     Microsoft Visual C++ 2008 SP1 Redistributable Package (x86)$\r$\n$\r$\n\
#     Would you like to download it now?" /SD IDNO IDYES clickyes
#     Goto end
#     clickyes:
#         ExecShell open "https://www.microsoft.com/en-us/download/details.aspx?id=26368"
#     end:
# FunctionEnd

# --- Installation sections ---
!define PROGRAM_UNINST_KEY "Software\Microsoft\Windows\CurrentVersion\Uninstall\${PROGRAM_NAME}"
!define PROGRAM_UNINST_ROOT_KEY "HKLM"
!define PROGRAM_UNINST_FILENAME "$INSTDIR\parsec-uninst.exe"

BrandingText "${PROGRAM_NAME} Windows Installer v${PARSEC_INSTALLER_VERSION}"
Name "${PROGRAM_NAME} ${PROGRAM_VERSION}"
OutFile "${BUILD_DIR}\${INSTALLER_FILENAME}"
InstallDir "$PROGRAMFILES\Parsec"

# No need for such details
ShowInstDetails hide
ShowUnInstDetails hide

# Install main application
Section "Parsec Secure Cloud Sharing" Section1
    SectionIn RO
    !include "${BUILD_DIR}\install_files.nsh"

    SetOverwrite ifnewer
    SetOutPath "$INSTDIR"
    WriteIniStr "$INSTDIR\homepage.url" "InternetShortcut" "URL" "${PROGRAM_WEB_SITE}"

    !insertmacro MUI_STARTMENU_WRITE_BEGIN Application
        SetShellVarContext all
        CreateDirectory "$SMPROGRAMS\$StartMenuFolder"
        CreateShortCut "$SMPROGRAMS\$StartMenuFolder\Parsec.lnk" "$INSTDIR\parsec.exe"
        CreateShortCut "$SMPROGRAMS\$StartMenuFolder\Website.lnk" "$INSTDIR\homepage.url"
        CreateShortCut "$SMPROGRAMS\$StartMenuFolder\Uninstall Parsec.lnk" ${PROGRAM_UNINST_FILENAME}
        SetShellVarContext current
    !insertmacro MUI_STARTMENU_WRITE_END
SectionEnd

!macro InstallWinFSP
    SetOutPath "$TEMP"
    File ${WINFSP_INSTALLER}
    ; Use /qn to for silent installation
    ; Use a very high installation level to make sure it runs till the end
    ExecWait "msiexec /i ${WINFSP_INSTALLER} /qn INSTALLLEVEL=1000"
    Delete ${WINFSP_INSTALLER}
!macroend

# Install winfsp if necessary
Section "WinFSP" Section2
    ClearErrors
    ReadRegStr $0 HKCR "Installer\Dependencies\WinFsp" "Version"
    ${If} ${Errors}
      # WinFSP is not installed
      !insertmacro InstallWinFSP
    ${Else}
        ${VersionCompare} $0 "1.3.0" $R0
        ${VersionCompare} $0 "2.0.0" $R1
        ${If} $R0 == 2
            ${OrIf} $R1 == 1
                ${OrIf} $R1 == 0
                  # Incorrect WinSFP version (<1.4.0 or >=2.0.0)
                  !insertmacro InstallWinFSP
        ${EndIf}
    ${EndIf}
SectionEnd

# Create parsec:// uri association.
Section "Associate parsec:// URI links with Parsec" Section3
    DeleteRegKey HKCR "Parsec"
    WriteRegStr HKCR "Parsec" "" "URL:Parsec Protocol"
    WriteRegStr HKCR "Parsec" "URL Protocol" ""
    WriteRegStr HKCR "Parsec\shell\open\command" "" '"$INSTDIR\parsec.exe" "%1"'
SectionEnd

# Hidden: Remove obsolete entries
Section "-Remove obsolete entries" Section4
    # Remove obsolete parsec registry configuration
    DeleteRegKey HKCU "Software\Classes\CLSID\{${APPGUID}}"
    DeleteRegKey HKCU "Software\Classes\Wow6432Node\CLSID\{${APPGUID}}"
    DeleteRegKey HKCU "Software\Microsoft\Windows\CurrentVersion\Explorer\Desktop\NameSpace\{${APPGUID}}"
    DeleteRegKey HKCU "Software\Microsoft\Windows\CurrentVersion\Explorer\HideDesktopIcons\NewStartPanel\{${APPGUID}}"
    ClearErrors
    # Remove obsolete mountpoint folder
    Delete "${OBSOLETE_MOUNTPOINT}\desktop.ini"
    RMDir "${OBSOLETE_MOUNTPOINT}"
    ClearErrors
SectionEnd

# The components screen is skipped - this is no longer necessary
# LangString DESC_Section1 ${LANG_ENGLISH} "Install Parsec."
# LangString DESC_Section2 ${LANG_ENGLISH} "Install WinFSP."
# LangString DESC_Section3 ${LANG_ENGLISH} "Let Parsec handle parsec:// URI links from the web-browser."
# LangString DESC_Section4 ${LANG_ENGLISH} "Remove obsolete entries from outdated parsec installation."
# !insertmacro MUI_FUNCTION_DESCRIPTION_BEGIN
#     !insertmacro MUI_DESCRIPTION_TEXT ${Section1} $(DESC_Section1)
#     !insertmacro MUI_DESCRIPTION_TEXT ${Section2} $(DESC_Section2)
#     !insertmacro MUI_DESCRIPTION_TEXT ${Section3} $(DESC_Section3)
#     !insertmacro MUI_DESCRIPTION_TEXT ${Section4} $(DESC_Section4)
# !insertmacro MUI_FUNCTION_DESCRIPTION_END

# Create uninstaller.
Section -Uninstaller
    WriteUninstaller ${PROGRAM_UNINST_FILENAME}
    WriteRegStr ${PROGRAM_UNINST_ROOT_KEY} "${PROGRAM_UNINST_KEY}" "DisplayName" "$(^Name)"
    WriteRegStr ${PROGRAM_UNINST_ROOT_KEY} "${PROGRAM_UNINST_KEY}" "UninstallString" ${PROGRAM_UNINST_FILENAME}
SectionEnd

# --- Uninstallation section ---
Section Uninstall
    # Delete Parsec files.
    Delete "$INSTDIR\homepage.url"
    Delete ${PROGRAM_UNINST_FILENAME}
    !include "${BUILD_DIR}\uninstall_files.nsh"
    RmDir "$INSTDIR"

    # Delete Start Menu items.
    !insertmacro MUI_STARTMENU_GETFOLDER Application $StartMenuFolder
        SetShellVarContext all
        Delete "$SMPROGRAMS\$StartMenuFolder\Parsec.lnk"
        Delete "$SMPROGRAMS\$StartMenuFolder\Uninstall Parsec.lnk"
        Delete "$SMPROGRAMS\$StartMenuFolder\Parsec Website.lnk"
        RmDir "$SMPROGRAMS\$StartMenuFolder"
        DeleteRegKey /ifempty HKCR "Software\Parsec"
        SetShellVarContext current
    Delete "$DESKTOP\Parsec.lnk"

    # Delete registry keys.
    DeleteRegKey ${PROGRAM_UNINST_ROOT_KEY} "${PROGRAM_UNINST_KEY}"
    # This key is only used by Parsec, so we should always delete it
    DeleteRegKey HKCR "Parsec"

  # Explorer shortcut keys potentially set by the application's settings
  DeleteRegKey HKCU "Software\Classes\CLSID\{${APPGUID}}"
  DeleteRegKey HKCU "Software\Classes\Wow6432Node\CLSID\{${APPGUID}"
  DeleteRegKey HKCU "Software\Microsoft\Windows\CurrentVersion\Explorer\Desktop\NameSpace\{${APPGUID}"
  DeleteRegKey HKCU "Software\Microsoft\Windows\CurrentVersion\Explorer\HideDesktopIcons\NewStartPanel\{${APPGUID}"

SectionEnd

# Add version info to installer properties.
VIProductVersion "${PARSEC_INSTALLER_VERSION}.0.0"
VIAddVersionKey ProductName ${PROGRAM_NAME}
VIAddVersionKey Comments "Parsec secure cloud sharing"
VIAddVersionKey CompanyName "Scille SAS"
VIAddVersionKey LegalCopyright "Scille SAS"
VIAddVersionKey FileDescription "${PROGRAM_NAME} Application Installer"
VIAddVersionKey FileVersion "${PARSEC_INSTALLER_VERSION}.0.0"
VIAddVersionKey ProductVersion "${PROGRAM_VERSION}.0"
VIAddVersionKey OriginalFilename ${INSTALLER_FILENAME}
