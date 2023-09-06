@echo off
>nul chcp 65001
echo Verificando privilégios de administrador...
echo disponibilizado por TiagoDev https://youtube.com/@TiagoMonteiroDev
timeout /t 3 >nul
echo.
echo.
::: BoostFpsTiagoDev
net session >nul 2>&1
if %errorLevel% == 0 (
    echo Você está executando como administrador.
    pause
	goto :continue
) else (
    echo Este script precisa ser executado como administrador.
    echo Fechando o programa...
    timeout /t 5 >nul
	exit
)
:continue
@echo off


bcdedit /set useplatformtick yes
bcdedit /set disabledynamictick yes
bcdedit /deletevalue useplatformclock
echo.
echo.
echo terminado
pause >nul
