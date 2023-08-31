use std::process::{Command, Stdio};
use std::thread::sleep;
use std::time::Duration;

pub(crate) fn otimizar_rede() {
    // Informações de cabeçalho
    println!("--------------------------------");
    println!("- Powered by TiagoDev -");
    println!("- https://youtube.com/@TiagoMonteiroDev -");
    println!("--------------------------------");
    println!("\nPressione Enter para começar a limpeza do DNS e otimização da rede...");

    // Aguardar a entrada do usuário
    let mut input = String::new();
    std::io::stdin()
        .read_line(&mut input)
        .expect("Erro ao ler a entrada do teclado.");

    // Limpando o cache DNS
    println!("Limpando o cache DNS...");
    Command::new("ipconfig")
        .arg("/flushdns")
        .output()
        .expect("Falhou ao limpar cache DNS");
    println!("Cache DNS limpo com sucesso.\n");

    // Desabilitando interfaces de rede
    println!("Desabilitando interface(s) de rede...");
    let output = Command::new("ipconfig")
        .output()
        .expect("Falhou ao executar ipconfig");
    let stdout = String::from_utf8_lossy(&output.stdout);

    for line in stdout.lines() {
        if line.contains("Adaptador") {
            let adapter: Vec<&str> = line.split(":").collect();
            if let Some(name) = adapter.get(0) {
                println!("Desabilitando {}...", name.trim());
                Command::new("netsh")
                    .arg("interface")
                    .arg("set")
                    .arg("interface")
                    .arg(name.trim())
                    .arg("admin=disable")
                    .stdout(Stdio::null())
                    .spawn()
                    .expect("Falhou ao desabilitar a interface de rede");
            }
        }
    }
    println!("Interfaces de rede desabilitadas.\n");

    // Aguardando 5 segundos
    println!("Aguardando 5 segundos para permitir que as interfaces sejam desativadas...");
    sleep(Duration::from_secs(5));

    // Habilitando interfaces de rede
    println!("Habilitando interface(s) de rede...");
    for line in stdout.lines() {
        if line.contains("Adaptador") {
            let adapter: Vec<&str> = line.split(":").collect();
            if let Some(name) = adapter.get(0) {
                println!("Habilitando {}...", name.trim());
                Command::new("netsh")
                    .arg("interface")
                    .arg("set")
                    .arg("interface")
                    .arg(name.trim())
                    .arg("admin=enable")
                    .stdout(Stdio::null())
                    .spawn()
                    .expect("Falhou ao habilitar a interface de rede");
            }
        }
    }
    println!("Interfaces de rede habilitadas.\n");

    // Finalizando
    println!("Otimização de DNS e rede concluída.");
    println!("Pressione Enter para sair...");
    input.clear();
    std::io::stdin()
        .read_line(&mut input)
        .expect("Erro ao ler a entrada do teclado.");

    /*Command::new("taskill")
    .arg("/F")
    .arg("/IM")
    .arg("cmd.exe")
    .output()
    .expect("Falhou ao fechar a janela");*/
}
