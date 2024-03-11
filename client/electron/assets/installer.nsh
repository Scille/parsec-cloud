!define WINFSP_VERSION "2.0.23075"
!define WINFSP_FILE "winfsp-${WINFSP_VERSION}.msi"

!macro installWinFSP
    File /oname=$PLUGINSDIR\${WINFSP_FILE} ${PROJECT_DIR}\build\${WINFSP_FILE}
    ; Use /qn to for silent installation
    ExecWait '"msiexec" /i "$PLUGINSDIR\${WINFSP_FILE}" /qn'
!macroend

!macro mayInstallWinFSP
    ClearErrors
    ReadRegStr $0 HKCR "Installer\Dependencies\WinFsp" "Version"
    ${If} ${Errors}
        !insertmacro installWinFSP
    ${Else}
        ; Required to use `VersionCompare` macro
        !include "WordFunc.nsh"

        ${VersionCompare} "$0" "2.0.0" $R0
        ${VersionCompare} "$0" "2.1.0" $R1
        ${If} $R1 < 2 ; Currently installed version is >= 2.1.0
            ${OrIf} $R0 == 2 ; Currently installed version is < 2.0.0
            !insertmacro installWinFSP
        ${EndIf}
    ${EndIf}
!macroend

!macro customInstall
    !insertmacro mayInstallWinFSP
!macroend
