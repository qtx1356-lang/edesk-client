#ifndef MyAppVersion
  #define MyAppVersion "1.5.3"
#endif

#define MyAppName "eDesk"
#define MyAppExeName "edesk.exe"

[Setup]
AppId={{A6C42B19-3F3B-4C84-9D9D-ED35D4A7B2C1}
AppName={#MyAppName}
AppVersion={#MyAppVersion}
AppVerName={#MyAppName} {#MyAppVersion}
AppPublisher=eDesk
DefaultDirName={autopf}\eDesk
DefaultGroupName=eDesk
DisableProgramGroupPage=yes
PrivilegesRequired=admin
ArchitecturesAllowed=x64
ArchitecturesInstallIn64BitMode=x64
MinVersion=10.0
OutputDir=..\..\dist
OutputBaseFilename=eDesk-Setup
SetupIconFile=..\..\flutter\windows\runner\resources\app_icon.ico
UninstallDisplayIcon={app}\edesk.exe
Compression=lzma2
SolidCompression=yes
WizardStyle=modern
CloseApplications=yes
RestartApplications=no
Uninstallable=yes
VersionInfoVersion={#MyAppVersion}.0
VersionInfoProductName=eDesk
VersionInfoCompany=eDesk


[UninstallRun]
Filename: "{sys}\netsh.exe"; Parameters: "advfirewall firewall delete rule name=""eDesk LAN Discovery"""; Flags: runhidden waituntilterminated
Filename: "{app}\{#MyAppExeName}"; Parameters: "--uninstall-service"; RunOnceId: "Remove eDesk background service"; Flags: runhidden waituntilterminated

[Languages]
Name: "english"; MessagesFile: "compiler:Default.isl"

[Tasks]
Name: "desktopicon"; Description: "Create a &desktop shortcut"; GroupDescription: "Additional shortcuts:"; Flags: unchecked

[Files]
Source: "..\..\edesk-baseline\*"; DestDir: "{app}"; Flags: ignoreversion recursesubdirs createallsubdirs

[Icons]
Name: "{group}\eDesk"; Filename: "{app}\{#MyAppExeName}"
Name: "{autodesktop}\eDesk"; Filename: "{app}\{#MyAppExeName}"; Tasks: desktopicon

[Run]
Filename: "{sys}\netsh.exe"; Parameters: "advfirewall firewall add rule name=""eDesk LAN Discovery"" dir=in action=allow protocol=UDP localport=21119 remoteip=LocalSubnet profile=any enable=yes"; StatusMsg: "Configuring eDesk local network discovery..."; Flags: runhidden waituntilterminated
Filename: "{app}\{#MyAppExeName}"; Parameters: "--install-service"; StatusMsg: "Installing eDesk background service..."; Flags: runhidden waituntilterminated
Filename: "{app}\{#MyAppExeName}"; Description: "Launch eDesk"; Flags: postinstall nowait skipifsilent runasoriginaluser
