[Setup]
AppName=KjTodoApp
AppVersion=1.0
DefaultDirName={autopf}\kjTodoApp
DefaultGroupName=kjTodoApp
OutputBaseFilename=kjTodoAppSetup
Compression=lzma
SolidCompression=yes
SetupIconFile=icon.ico

[Files]
Source: "target\release\your_app.exe"; DestDir: "{app}"; Flags: ignoreversion

[Icons]
Name: "{autoprograms}\kjTodoApp"; Filename: "{app}\your_app.exe"
Name: "{autodesktop}\kjTodoApp"; Filename: "{app}\your_app.exe"; Tasks: desktopicon

[Tasks]
Name: "desktopicon"; Description: "{cm:CreateDesktopIcon}"; GroupDescription: "{cm:AdditionalIcons}"

[Run]
Filename: "{app}\your_app.exe"; Description: "{cm:LaunchProgram,YourTodoApp}"; Flags: nowait postinstall skipifsilent