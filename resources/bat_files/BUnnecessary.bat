@echo off
>nul chcp 65001
color 0a
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
echo Desativando servicos desnecessarios...
sc config "wsearch" start= disabled
sc stop wsearch
sc config "WerSvc" start= disabled
sc config "DiagTrack" start= disabled
sc config SysMain start= disabled
sc stop SysMain
echo.
echo.
color 3
echo FINALIZADO
pause >nul