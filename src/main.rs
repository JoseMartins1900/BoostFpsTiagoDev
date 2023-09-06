

mod Internet;
mod Regs;
mod Visual;
mod permission;
mod reduzir_memoria;

use druid::widget::{Button, Flex, Label, Padding};
use druid::{AppLauncher, LocalizedString, Widget, WindowDesc};
use include_dir::{include_dir, Dir};
use std::path::Path;
use std::process::{exit, Command, Stdio};
use Internet::melhorar_internet;
use Regs::otimizacao_geral;
use Visual::interface::ui_builder;

use crate::permission::{is_admin, run_as_admin};

const BAT_FILES: include_dir::Dir = include_dir!("resources/bat_files");



fn main() {
    env_logger::init();

    // Mostra a informação no terminal
    println!("=====================================");
    println!("https://youtube.com/@TiagoMonteiroDev");
    println!("=====================================");
    println!("========= BoostFpsTiagoDev ==========");
    println!("=====================================");

    // Aguarde o usuário pressionar Enter
    println!("\nPressione Enter para continuar...");
    let mut input = String::new();
    std::io::stdin()
        .read_line(&mut input)
        .expect("Erro ao ler da entrada padrão");

    if !is_admin() {
        println!("\n\n\n");
        println!("=====================================");
        println!("https://youtube.com/@TiagoMonteiroDev");
        println!("=====================================");
        println!("========= BoostFpsTiagoDev ==========");
        println!("=====================================");
        println!("\n\n");
        println!("Solicitando permissões do administrador");
        println!("Aguardando permissão...");
        println!("Este programa requer permissões de administrador.");
        println!("\n\n");
        println!("Executando como administrador...");
        std::thread::sleep(std::time::Duration::from_secs(1));
        run_as_admin();
        exit(0);
    }

    let main_window = WindowDesc::new(ui_builder())
        .title("Boost Fps by TiagoDev");

    AppLauncher::with_window(main_window)
        .log_to_console()
        .launch(())
        .expect("Falha ao iniciar a aplicação");
}
