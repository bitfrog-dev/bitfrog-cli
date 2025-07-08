Name "Bitfrog CLI"
!define VERSION "0.1.1"
OutFile "InstallBitfrogCLI-${VERSION}.exe"
InstallDir "$PROGRAMFILES\Bitfrog CLI"

RequestExecutionLevel admin

Page directory
Page instfiles

Section "Install"

    SetOutPath $INSTDIR\bin
    File "target\release\bitfrog.exe"
    
    SetOutPath $INSTDIR
    WriteUninstaller "$INSTDIR\Uninstall.exe"
    
    EnVar::SetHKCU
    EnVar::AddValue "PATH" "$INSTDIR\bin"
    
SectionEnd

Section "Uninstall"

    EnVar::SetHKCU
    EnVar::DeleteValue "PATH" "$INSTDIR\bin"
    
    RMDir /r "$INSTDIR\*.*"
    RMDir "$INSTDIR"
    
SectionEnd