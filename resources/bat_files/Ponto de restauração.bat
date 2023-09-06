@echo off
>nul chcp 65001
Title Ferramenta de ponto de restauração
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
cls
@echo off
echo Script de ponto de restauração by TiagoDev
color 4
>nul chcp 65001
echo Em alguns segundos um ponto de restauração começará a ser criado.
echo.
echo.
timeout /t 5
cls
title Ponto de restauração sendo criado.
reg add "HKLM\Software\Microsoft\Windows NT\CurrentVersion\SystemRestore" /v "SystemRestorePointCreationFrequency" /t REG_DWORD /d 1 /f >nul 2>&1
powershell -ExecutionPolicy Unrestricted -NoProfile Enable-ComputerRestore -Drive 'C:\', 'D:\', 'E:\', 'F:\', 'G:\' >nul 2>&1
powershell -ExecutionPolicy Unrestricted -NoProfile Checkpoint-Computer -Description ' BoostFpsTiagoDev' >nul 2>&1
cls
title O ponto de restauração foi criado com sucesso.
echo O ponto de restauração foi criado com sucesso. pressione qualquer tecla para sair
echo.
pause