use druid::widget::{Button, Flex, Label, Padding};
use druid::{AppLauncher, LocalizedString, Widget, WindowDesc};
use std::process::{exit, Command, Stdio};

use crate::{melhorar_internet, reduzir_memoria};

pub fn is_admin() -> bool {
    // Executa um comando que requer privilégios de administrador
    // O comando "net session" falha se executado sem privilégios de administrador
    let output = Command::new("net")
        .arg("session")
        .output()
        .expect("Falha ao executar comando");

    // Verifica se o comando foi executado com sucesso
    output.status.success()
}

pub fn run_as_admin() {
    Command::new("powershell")
        .args(&[
            "-ex",
            "unrestricted",
            "-c",
            &format!(
                "Start-Process -FilePath '{}' -ArgumentList '-runas' -Verb RunAs",
                std::env::current_exe().unwrap().to_str().unwrap()
            ),
        ])
        .status()
        .expect("Falha ao executar como administrador");
}
