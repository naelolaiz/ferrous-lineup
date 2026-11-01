; Inno Setup script for Ferrous Lineup.
;
; The release workflow calls this with:
;   ISCC.exe /DAppVersion=0.5.0 /DBuildLabel=v0.5.0 ferrous-lineup.iss
;
; It builds an installer in ..\..\dist: a Start-menu entry, an optional
; desktop shortcut, an uninstaller, and the .lineup file association.
; The portable .zip is still made by the workflow for people who prefer
; not to install anything.

#ifndef AppVersion
  #define AppVersion "0.5.0"
#endif
#ifndef BuildLabel
  #define BuildLabel "local"
#endif
#ifndef AppArch
  #define AppArch "x86_64"
#endif
#ifndef SourceExe
  #define SourceExe "..\..\target\x86_64-pc-windows-msvc\release\ferrous-lineup.exe"
#endif

[Setup]
AppId={{CE6C278D-640C-49CA-8B73-61480E54EEEE}
AppName=Ferrous Lineup
AppVersion={#AppVersion}
AppVerName=Ferrous Lineup {#AppVersion}
AppPublisher=Ferrous Lineup contributors
AppPublisherURL=https://github.com/naelolaiz/ferrous-lineup
AppSupportURL=https://github.com/naelolaiz/ferrous-lineup/issues
AppUpdatesURL=https://github.com/naelolaiz/ferrous-lineup/releases
DefaultDirName={autopf}\Ferrous Lineup
DefaultGroupName=Ferrous Lineup
DisableProgramGroupPage=yes
PrivilegesRequired=lowest
ArchitecturesAllowed=x64
ArchitecturesInstallIn64BitMode=x64
OutputDir=..\..\dist
OutputBaseFilename=ferrous-lineup-{#BuildLabel}-windows-{#AppArch}-setup
SetupIconFile=ferrous-lineup.ico
UninstallDisplayIcon={app}\ferrous-lineup.exe
UninstallDisplayName=Ferrous Lineup
LicenseFile=..\..\LICENSE
Compression=lzma2
SolidCompression=yes
WizardStyle=modern
CloseApplications=yes
ChangesAssociations=yes

[Tasks]
Name: "desktopicon"; Description: "Create a &desktop shortcut"; GroupDescription: "Additional shortcuts:"

[Files]
Source: "{#SourceExe}"; DestDir: "{app}"; Flags: ignoreversion
Source: "..\..\README.md"; DestDir: "{app}"; Flags: ignoreversion
Source: "..\..\LICENSE"; DestDir: "{app}"; Flags: ignoreversion

[Icons]
Name: "{group}\Ferrous Lineup"; Filename: "{app}\ferrous-lineup.exe"
Name: "{group}\Uninstall Ferrous Lineup"; Filename: "{uninstallexe}"
Name: "{autodesktop}\Ferrous Lineup"; Filename: "{app}\ferrous-lineup.exe"; Tasks: desktopicon

[Registry]
; .lineup files open in Ferrous Lineup.
Root: HKCU; Subkey: "Software\Classes\.lineup"; ValueType: string; ValueName: ""; ValueData: "FerrousLineup.Project"; Flags: uninsdeletevalue
Root: HKCU; Subkey: "Software\Classes\FerrousLineup.Project"; ValueType: string; ValueName: ""; ValueData: "Ferrous Lineup project"; Flags: uninsdeletekey
Root: HKCU; Subkey: "Software\Classes\FerrousLineup.Project\DefaultIcon"; ValueType: string; ValueName: ""; ValueData: "{app}\ferrous-lineup.exe,0"
Root: HKCU; Subkey: "Software\Classes\FerrousLineup.Project\shell\open\command"; ValueType: string; ValueName: ""; ValueData: """{app}\ferrous-lineup.exe"" ""%1"""

[Run]
Filename: "{app}\ferrous-lineup.exe"; Description: "Launch Ferrous Lineup"; Flags: nowait postinstall skipifsilent
