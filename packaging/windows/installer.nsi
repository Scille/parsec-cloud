# Parsec Cloud (https://parsec.cloud) Copyright (c) AGPLv3 2019 Scille SAS

# Greatly inspired by Deluge NSIS installer
# https://github.com/deluge-torrent/deluge/blob/develop/packaging/win32/Win32%20README.txt

!addplugindir nsis_plugins
!addincludedir nsis_plugins
!include "WordFunc.nsh"
!include "cleanup_older_artefacts.nsi"

# Script version; displayed when running the installer
!define INSTALLER_SCRIPT_VERSION "1.0"

# Program information
!define PROGRAM_NAME "Parsec"
!define PROGRAM_WEB_SITE "http://parsec.cloud"
!define APPGUID "6C37F945-7EFC-480A-A444-A6D44A3D107F"
!define OBSOLETE_MOUNTPOINT "$PROFILE\Parsec"

# Icon overlays GUIDS
!define CHECK_ICON_GUID "{5449BC90-310B-40A8-9ABF-C5CFCEC7F430}"
!define REFRESH_ICON_GUID "{41E71DD9-368D-46B2-BB9D-4359599BBBC3}"

# Detect version from file
!define BUILD_DIR "build"
!searchparse /file ${BUILD_DIR}/manifest.ini `target = "` PROGRAM_FREEZE_BUILD_DIR `"`
!ifndef PROGRAM_FREEZE_BUILD_DIR
  !error "Cannot find freeze build directory"
!endif
!searchparse /file ${BUILD_DIR}/manifest.ini `program_version = "` PROGRAM_VERSION `"`
!ifndef PROGRAM_VERSION
  !error "Program Version Undefined"
!endif
!searchparse /file ${BUILD_DIR}/manifest.ini `platform = "` PROGRAM_PLATFORM `"`
!ifndef PROGRAM_PLATFORM
  !error "Program Platform Undefined"
!endif
!searchparse /file ${BUILD_DIR}/manifest.ini `winfsp_installer_path = "` WINFSP_INSTALLER_PATH `"`
!ifndef WINFSP_INSTALLER_PATH
  !error "WinFSP installer path Undefined"
!endif
!searchparse /file ${BUILD_DIR}/manifest.ini `winfsp_installer_name = "` WINFSP_INSTALLER_NAME `"`
!ifndef WINFSP_INSTALLER_NAME
  !error "WinFSP installer name Undefined"
!endif

# Python files generated
!define LICENSE_FILEPATH "${PROGRAM_FREEZE_BUILD_DIR}\LICENSE.txt"
!define INSTALLER_FILENAME "parsec-${PROGRAM_VERSION}-${PROGRAM_PLATFORM}-setup.exe"

# Icon handling
# Not the space before ` ParsecCheckIconHandler` and ` ParsecRefreshIconHandler`
# This is useful to prioritize our handlers since the number of icon handlers is limited to 15.
# Note that OneDrive already uses this trick.
!define ICON_HANDLER_CHECK_KEY "SOFTWARE\Microsoft\Windows\CurrentVersion\Explorer\ShellIconOverlayIdentifiers\ ParsecCheckIconHandler"
!define ICON_HANDLER_REFRESH_KEY "SOFTWARE\Microsoft\Windows\CurrentVersion\Explorer\ShellIconOverlayIdentifiers\ ParsecRefreshIconHandler"

# Uninstallation
!define PROGRAM_UNINST_KEY "Software\Microsoft\Windows\CurrentVersion\Uninstall\${PROGRAM_NAME}"
!define PROGRAM_UNINST_ROOT_KEY "HKLM"
!define PROGRAM_UNINST_FILENAME "$INSTDIR\uninstall.exe"

# Set default compressor
SetCompressor /FINAL /SOLID lzma
SetCompressorDictSize 64

# --- Interface settings ---
# Modern User Interface 2
!include MUI2.nsh
# Installer
!define MUI_ICON "icon.ico"
!define MUI_HEADERIMAGE
!define MUI_HEADERIMAGE_RIGHT
!define MUI_HEADERIMAGE_BITMAP "installer-top.bmp"
!define MUI_WELCOMEFINISHPAGE_BITMAP "installer-side.bmp"
!define MUI_COMPONENTSPAGE_SMALLDESC
!define MUI_ABORTWARNING
# Start Menu Folder Page Configuration
!define MUI_STARTMENUPAGE_DEFAULTFOLDER "${PROGRAM_NAME}"
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
# !define MUI_FINISHPAGE_SHOWREADME_NOTCHECKED
!define MUI_FINISHPAGE_SHOWREADME_TEXT "Create Desktop Shortcut"
!define MUI_FINISHPAGE_SHOWREADME_FUNCTION CreateDesktopShortcut
# Run program after install, using explorer.exe to un-elevate privileges
# More information: https://stackoverflow.com/a/15041823/2846140
!define MUI_FINISHPAGE_RUN "$WINDIR\explorer.exe"
!define MUI_FINISHPAGE_RUN_PARAMETERS "$INSTDIR\parsec.exe"
!define MUI_FINISHPAGE_RUN_TEXT "Run Parsec"
# !define MUI_FINISHPAGE_RUN_NOTCHECKED

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

; Taken from https://nsis.sourceforge.io/Get_parent_directory
; (NSIS project, Copyright (C) 1999-2021 Contributors, zlib/libpng license)
; GetParent
; input, top of stack  (e.g. C:\Program Files\Foo)
; output, top of stack (replaces, with e.g. C:\Program Files)
; modifies no other variables.
;
; Usage:
;   Push "C:\Program Files\Directory\Whatever"
;   Call GetParent
;   Pop $R0
;   ; at this point $R0 will equal "C:\Program Files\Directory"
Function GetParent

  Exch $R0
  Push $R1
  Push $R2
  Push $R3

  StrCpy $R1 0
  StrLen $R2 $R0

  loop:
    IntOp $R1 $R1 + 1
    IntCmp $R1 $R2 get 0 get
    StrCpy $R3 $R0 1 -$R1
    StrCmp $R3 "\" get
  Goto loop

  get:
    StrCpy $R0 $R0 -$R1

    Pop $R3
    Pop $R2
    Pop $R1
    Exch $R0

FunctionEnd


!macro checkProgramAlreadyRunning un message
Function ${un}checkProgramAlreadyRunning
    check:
        System::Call 'kernel32::OpenMutex(i 0x100000, b 0, t "parsec-cloud") i .R0'
        IntCmp $R0 0 notRunning
            System::Call 'kernel32::CloseHandle(i $R0)'
            MessageBox MB_OKCANCEL|MB_ICONEXCLAMATION \
                "Parsec is running, please close it first.$\n$\n \
                Click `OK` to retry or `Cancel` to cancel this ${message}." \
                /SD IDCANCEL IDOK check
            Abort
    notRunning:
FunctionEnd
!macroend
!insertmacro checkProgramAlreadyRunning "" "upgrade"
!insertmacro checkProgramAlreadyRunning "un." "removal"


; Run the uninstaller sequentially and silently
; https://nsis.sourceforge.io/Docs/Chapter3.html#installerusageuninstaller
Function runUninstaller
      ; Retrieve the previous version's install directory and store it into $R1.
      ; We cannot use ${INSTDIR} instead, given the previous version might
      ; have been installed in a custom directory.
      Push "$R0"
      Call GetParent
      Pop $R1
      ClearErrors
      ; If run without `_?=R1`, the uninstaller executable (i.e. `$R0`) will
      ; copy itself in a temporary directory, run this copy and exit right away.
      ; This is needed otherwise the installer won't be able to remove itself,
      ; however it also means `ExecWait` doesn't work here (the actual uninstall
      ; process is still running when ExecWait returns).
      ; So we provide the ugly `_?=$R1` which, on top of being absolutely
      ; unreadable, does two things:
      ; - it force the directory to work on for the uninstaller (but we pass the
      ;   same previous version install directory as argument, so we change nothing here)
      ; - it tells the uninstaller not to do the "temp copy, exec and return"
      ;   trick and instead leave the uninstaller untouched.
      ; At this point, I'm very much puzzled as why writing NSIS installer
      ; feels like reverse engineering a taiwanese NES clone...
      ExecWait '"$R0" /S _?=$R1'
      ; Remove the uninstaller since it hasn't removed iteself due to `_?=$R1`
      Delete $R0
      ; Remove older artefacts that might remain from previous installation
      !insertmacro CleanupOlderArtefacts "$R1"
      ; Remove the previous install directory if it's empty
      RmDir $R1
FunctionEnd

Function checkUninstaller
  ; Check either 32-bit or 64-bit registry for parsec uninstaller
  ReadRegStr $R0 ${PROGRAM_UNINST_ROOT_KEY} \
    "${PROGRAM_UNINST_KEY}" \
    "UninstallString"
  StrCmp $R0 "" done

  ; Check that the uninstaller file actually exists
  IfFileExists $R0 ask_for_uninstall cleanup

  ; Perform some registry cleanup if the installer wasn't found
  cleanup:
    DeleteRegKey ${PROGRAM_UNINST_ROOT_KEY} "${PROGRAM_UNINST_KEY}"
    Goto done

  ; Prompt the user for uninstaller the previous version of parsec
  ask_for_uninstall:
    MessageBox MB_OKCANCEL|MB_ICONEXCLAMATION \
      "${PROGRAM_NAME} is already installed. $\n$\nClick `OK` to remove the \
      previous version or `Cancel` to cancel this upgrade." \
      /SD IDOK IDOK uninstall IDCANCEL _abort

  ; Abort in case the user clicked "cancel"
  _abort:
    Abort

  ; Run the uninstaller
  uninstall:
    Call runUninstaller
    Goto done

  done:
FunctionEnd

Function .onInit
    # Check for running program instance.
    Call checkProgramAlreadyRunning
    # Check for existing installation in 64-bit registry
    # Due to the regression explained in issue #5845, it's possible that parsec 2.16.0
    # or 2.16.1 has overwritten a parsec 2.15.0 installation or older. For this reason,
    # both 32 and 64-bit registry might point to the same uninstaller. Since the uninstaller
    # corresponds to a 2.16.0 or 2.16.1 install, it will cleanup the 64-bit registry when it's
    # done. This is why it makes sense to perform the 64-bit uninstall first.
    SetRegView 64
    call checkUninstaller
    # Check for existing installation in 32-bit registry
    # In the case explained above, the 32-bit registry points to an uninstaller that no longer
    # exists (since the 64-bit uninstall has removed it). In this case, `checkUninstaller`
    # cleans up the registry so Parsec does not appear in the list of installed programs.
    SetRegView 32
    call checkUninstaller
    # Leave in 64 registry mode
    SetRegView 64
FunctionEnd

Function un.onUninstSuccess
    HideWindow
    MessageBox MB_ICONINFORMATION|MB_OK "$(^Name) was successfully removed from your computer." /SD IDOK
FunctionEnd

Function un.onInit
    Call un.checkProgramAlreadyRunning
    SetRegView 64
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

BrandingText "${PROGRAM_NAME} Windows Installer v${INSTALLER_SCRIPT_VERSION}"
Name "${PROGRAM_NAME} ${PROGRAM_VERSION}"
OutFile "${BUILD_DIR}\${INSTALLER_FILENAME}"
InstallDir "$PROGRAMFILES\Parsec Cloud"

# No need for such details
ShowInstDetails hide
ShowUnInstDetails hide

!macro DeleteOrMoveFile dir temp name
    # This routine ensures that we are not leaving an old `${dir}\${name}` file in place
    # Case 1: the DLL file is not used:
    #  - it's properly deleted
    # Case 2: the DLL file is used, and is in the same drive as `${temp}`:
    #  - the file is moved to `${temp}`
    # Case 3: the DLL file is used, and is not in the same drive as `${temp}`:
    #  - the file is renamed in-place as `${temp}.old`
    Delete "${dir}\${name}"
    Delete "${dir}\${name}.old"
    Delete "${temp}\${name}.old"
    Rename "${dir}\${name}" "${dir}\${name}.old"
    Rename "${dir}\${name}.old" "${temp}\${name}.old"
    Delete "${temp}\${name}.old"
!macroend

!macro MoveParsecShellExtension
    # When upgrading or uninstalling, shell extensions may be loaded by `explorer.exe`
    # thus windows will prevent us to delete or modify the dll extensions. The trick
    # here is to move the loaded dll somewhere else (like in a temp folder) and then resume
    # the upgrade/uninstall process as usual.
    RmDir /r "$TEMP\parsec_tmp"
    CreateDirectory "$TEMP\parsec_tmp"
    !insertmacro DeleteOrMoveFile "$INSTDIR" "$TEMP\parsec_tmp" "check-icon-handler.dll"
    !insertmacro DeleteOrMoveFile "$INSTDIR" "$TEMP\parsec_tmp" "refresh-icon-handler.dll"
    !insertmacro DeleteOrMoveFile "$INSTDIR" "$TEMP\parsec_tmp" "vcruntime140.dll"
    !insertmacro DeleteOrMoveFile "$INSTDIR" "$TEMP\parsec_tmp" "vcruntime140_1.dll"
!macroend

# Install main application
Section "Parsec Cloud Sharing" Section1
    !insertmacro MoveParsecShellExtension
    SectionIn RO

    ; Remove older artefacts that might remain from previous installation
    !insertmacro CleanupOlderArtefacts "$INSTDIR"
    # Just in case, make sure that no psutil artefact remains from a previous installation
    # See issue #5845
    Delete "$INSTDIR\psutil\_psutil_windows*.pyd"

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

    # Clean up older entry names for icon handling
    # This is useful since the number of icon handler is limited to 15
    DeleteRegKey HKLM "SOFTWARE\Microsoft\Windows\CurrentVersion\Explorer\ShellIconOverlayIdentifiers\CheckIconHandler"
    DeleteRegKey HKLM "SOFTWARE\Microsoft\Windows\CurrentVersion\Explorer\ShellIconOverlayIdentifiers\RefreshIconHandler"

    # Call regsvr32
    ExecWait '$SYSDIR\regsvr32.exe /s /n /i:user "$INSTDIR\check-icon-handler.dll"'
    ExecWait '$SYSDIR\regsvr32.exe /s /n /i:user "$INSTDIR\refresh-icon-handler.dll"'

    # Write Icons overlays to registry
    WriteRegStr HKLM "${ICON_HANDLER_CHECK_KEY}" "" "${CHECK_ICON_GUID}"
    WriteRegStr HKLM "${ICON_HANDLER_REFRESH_KEY}" "" "${REFRESH_ICON_GUID}"
SectionEnd

!macro InstallWinFSP
    SetOutPath "$TEMP"
    File ${WINFSP_INSTALLER_PATH}
    ; Use /qn to for silent installation
    ; Use a very high installation level to make sure it runs till the end
    ExecWait "msiexec /i ${WINFSP_INSTALLER_NAME} /qn INSTALLLEVEL=1000"
    Delete ${WINFSP_INSTALLER_NAME}
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
  !insertmacro MoveParsecShellExtension

    # Delete program files.
    Delete "$INSTDIR\homepage.url"
    Delete ${PROGRAM_UNINST_FILENAME}
    !include "${BUILD_DIR}\uninstall_files.nsh"
    ; Remove older artefacts that might remain from previous installation
    !insertmacro CleanupOlderArtefacts "$INSTDIR"
    ; Remove install directory if empty
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

  # Clean up older entry names for icon handling
  DeleteRegKey HKLM "SOFTWARE\Microsoft\Windows\CurrentVersion\Explorer\ShellIconOverlayIdentifiers\CheckIconHandler"
  DeleteRegKey HKLM "SOFTWARE\Microsoft\Windows\CurrentVersion\Explorer\ShellIconOverlayIdentifiers\RefreshIconHandler"

  # Clean up icon handling registry
  DeleteRegKey HKLM "${ICON_HANDLER_CHECK_KEY}"
  DeleteRegKey HKLM "${ICON_HANDLER_REFRESH_KEY}"

  ExecWait '$SYSDIR\regsvr32.exe /s /u /i:user "$INSTDIR\check-icon-handler.dll"'
  ExecWait '$SYSDIR\regsvr32.exe /s /u /i:user "$INSTDIR\refresh-icon-handler.dll"'
SectionEnd

# Add version info to installer properties.
VIProductVersion "${INSTALLER_SCRIPT_VERSION}.0.0"
VIAddVersionKey ProductName "${PROGRAM_NAME}"
VIAddVersionKey Comments "Parsec secure cloud sharing"
VIAddVersionKey CompanyName "Scille SAS"
VIAddVersionKey LegalCopyright "Scille SAS"
VIAddVersionKey FileDescription "${PROGRAM_NAME} Application Installer"
VIAddVersionKey FileVersion "${INSTALLER_SCRIPT_VERSION}.0.0"
VIAddVersionKey ProductVersion "${PROGRAM_VERSION}.0"
VIAddVersionKey OriginalFilename ${INSTALLER_FILENAME}

ManifestDPIAware true
