
mod reduzir_memoria;
mod bats;
mod permission;
mod Visual;
mod Internet;
mod Regs;

use druid::{AppLauncher, Widget, WindowDesc, LocalizedString};
use druid::widget::{Button, Flex, Padding, Label};
use std::process::{Command, Stdio, exit};
use Visual::interface::ui_builder;
use Internet::melhorar_internet;
use Regs::otimizacao_geral;

use crate::permission::{is_admin, run_as_admin};



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
        std::io::stdin().read_line(&mut input).expect("Erro ao ler da entrada padrão");

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
            std::thread::sleep(std::time::Duration::from_secs(4));
            run_as_admin();
            exit(0);
        }

    let main_window = WindowDesc::new(ui_builder()).title("Boost Fps by TiagoDev").window_size((650.0, 650.0));  
    AppLauncher::with_window(main_window)
        .launch(())
        .expect("Falha ao iniciar a aplicação");

        
}
