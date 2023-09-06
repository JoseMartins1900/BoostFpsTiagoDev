chcp 1252 > nul
cls
ECHO.
ECHO.
ECHO.
ECHO CLIQUE NO NUMERO CORRESPONDENTE A QUANTIDADE DE RAM QUE VOCE TEM NO SEU DISPOSITIVO
ECHO.
ECHO.
ECHO.
ECHO.
ECHO   1 - 4GB RAM
ECHO   2 - 6GB RAM
ECHO   3 - 8GB RAM
ECHO   4 - 12GB RAM
ECHO   5 - 16GB RAM
ECHO   6 - 32GB RAM
ECHO   7 - 64GB RAM
ECHO   8 - Sair
ECHO.
ECHO.
SET /P M= ESCOLHA UM NUMERO E PRESSIONE ENTER:
IF %M%==1 GOTO 4GB
IF %M%==2 GOTO 6GB
IF %M%==3 GOTO 8GB
IF %M%==4 GOTO 12GB
IF %M%==5 GOTO 16GB
IF %M%==6 GOTO 32GB
IF %M%==7 GOTO 64GB
IF %M%==8 GOTO EOF
:4GB
reg add "HKEY_LOCAL_MACHINE\SYSTEM\ControlSet001\Control" /v "SvcHostSplitThresholdInKB" /t REG_DWORD /d "4194304" /f
GOTO continue

:6GB
reg add "HKEY_LOCAL_MACHINE\SYSTEM\ControlSet001\Control" /v "SvcHostSplitThresholdInKB" /t REG_DWORD /d "6291456" /f
GOTO continue

:8GB
reg add "HKEY_LOCAL_MACHINE\SYSTEM\ControlSet001\Control" /v "SvcHostSplitThresholdInKB" /t REG_DWORD /d "8388608" /f
GOTO continue

:12GB
reg add "HKEY_LOCAL_MACHINE\SYSTEM\ControlSet001\Control" /v "SvcHostSplitThresholdInKB" /t REG_DWORD /d "12582912" /f
GOTO continue

:16GB 
reg add "HKEY_LOCAL_MACHINE\SYSTEM\ControlSet001\Control" /v "SvcHostSplitThresholdInKB" /t REG_DWORD /d "16777216" /f
GOTO continue

:32GB
reg add "HKEY_LOCAL_MACHINE\SYSTEM\ControlSet001\Control" /v "SvcHostSplitThresholdInKB" /t REG_DWORD /d "33554432" /f
GOTO continue

:64GB
reg add "HKEY_LOCAL_MACHINE\SYSTEM\ControlSet001\Control" /v "SvcHostSplitThresholdInKB" /t REG_DWORD /d "67108864" /f

:continue
cls
ECHO.
ECHO Configuracao aplicada. Pressione qualquer tecla para sair.
PAUSE