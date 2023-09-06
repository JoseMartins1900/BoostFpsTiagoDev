@echo off
setlocal


:: Disable scheduled tasks
echo Disabling scheduled tasks...
schtasks /change /disable /TN "\Microsoft\Windows\Application Experience\Microsoft Compatibility Appraiser"
schtasks /change /disable /TN "\Microsoft\Windows\Application Experience\ProgramDataUpdater"
schtasks /change /disable /TN "\Microsoft\Windows\Application Experience\StartupAppTask"
schtasks /change /disable /TN "\Microsoft\Windows\Application Experience\PcaPatchDbTask"
schtasks /change /disable /TN "\Microsoft\Windows\Customer Experience Improvement Program\Consolidator"
schtasks /change /disable /TN "\Microsoft\Windows\Customer Experience Improvement Program\KernelCEIPTask"
schtasks /change /disable /TN "\Microsoft\Windows\Customer Experience Improvement Program\USBCEIP"
schtasks /change /disable /TN "\Microsoft\Windows\Autochk\Proxy"
schtasks /change /disable /TN "\Microsoft\Windows\DiskDiagnostic\Microsoft-Windows-DiskDiagnosticDataCollector"
schtasks /change /disable /TN "\Microsoft\Windows\Feedback\Siuf\DmClient"
schtasks /change /disable /TN "\Microsoft\Windows\Feedback\Siuf\DmClientOnScenarioDownload"
schtasks /change /disable /TN "\Microsoft\Windows\Maps\MapsUpdateTask"
schtasks /change /disable /TN "\Microsoft\Windows\Windows Error Reporting\QueueReporting"
schtasks /change /disable /TN "\Microsoft\Windows\WindowsUpdate\sih"
schtasks /change /disable /TN "\Microsoft\Windows\WindowsUpdate\Automatic App Update"
schtasks /change /disable /TN "\Microsoft\Windows\WindowsUpdate\Scheduled Start"
schtasks /change /disable /TN "\Microsoft\Windows\AppID\SmartScreenSpecific"
takeown /F %SystemRoot%\System32\Tasks\Microsoft\Windows\UpdateOrchestrator /A /R
icacls %SystemRoot%\System32\Tasks\Microsoft\Windows\UpdateOrchestrator /grant *S-1-5-32-544:F /T
schtasks /change /disable /TN "\Microsoft\Windows\UpdateOrchestrator\Refresh Settings"
schtasks /change /disable /TN "\Microsoft\Windows\UpdateOrchestrator\Report policies"
schtasks /change /disable /TN "\Microsoft\Windows\UpdateOrchestrator\Schedule scan"
schtasks /change /disable /TN "\Microsoft\Windows\UpdateOrchestrator\Schedule scan static task"
schtasks /change /disable /TN "\Microsoft\Windows\UpdateOrchestrator\UpdateModelTask"
schtasks /change /disable /TN "\Microsoft\Windows\UpdateOrchestrator\USO_UxBroker"
schtasks /change /disable /TN "\Microsoft\Windows\UpdateOrchestrator\Schedule work"
schtasks /change /disable /TN "\Microsoft\Windows\UpdateOrchestrator\Start Oobe Expedite Work"
schtasks /change /disable /TN "\Microsoft\Windows\UpdateOrchestrator\StartOobeAppsScan"
schtasks /change /disable /TN "\Microsoft\Windows\UpdateOrchestrator\StartOobeAppsScanAfterUpdate"
schtasks /change /disable /TN "\Microsoft\Windows\UpdateOrchestrator\UUS Failover Task"
schtasks /change /disable /TN "\Microsoft\Windows\Setup\EOSNotify"
schtasks /change /disable /TN "\Microsoft\Windows\SettingSync\BackgroundUploadTask"
schtasks /change /disable /TN "\Microsoft\Windows\SettingSync\NetworkStateChangeTask"
schtasks /change /disable /TN "\Microsoft\Windows\License Manager\TempSignedLicenseExchange"
schtasks /change /disable /TN "\Microsoft\Windows\PushToInstall\Registration"
schtasks /change /disable /TN "\Microsoft\Windows\Subscription\EnableLicenseAcquisition"
schtasks /change /disable /TN "\Microsoft\Windows\WindowsUpdate\Scheduled Start"
schtasks /end /TN "\MicrosoftEdgeUpdateTaskMachineCore"
schtasks /change /disable /TN "\MicrosoftEdgeUpdateTaskMachineCore"
schtasks /change /disable /TN "\MicrosoftEdgeUpdateTaskMachineUA"



echo All tasks completed successfully.
endlocal
