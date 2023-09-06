use crate::Regs::inputlag_mouse_and_keyboard;
use crate::{melhorar_internet, otimizacao_geral, reduzir_memoria};
use druid::piet::TextLayoutBuilder;
use druid::widget::{Align, Button, Flex, Label, SizedBox, CrossAxisAlignment, Slider, Scroll};
use druid::{AppLauncher, Color, LocalizedString, Widget, WidgetExt, WindowDesc};
use include_dir::{include_dir, Dir};
use std::fs::{remove_file, File};
use std::io::{Result, Write};
use std::path::Path;
use std::process::{exit, Command, Stdio};
use std::thread;
use std::time::Duration;

const VERTICAL_WIDGET_SPACING: f64 = 20.0;
const HORIZONTAL_WIDGET_SPACING: f64 = 20.0;
const BUTTON_WIDTH: f64 = 350.0;
const BUTTON_HEIGHT: f64 = 50.0;

// Inclui os recursos
const BAT_FILES: Dir = include_dir!("resources/bat_files");
const REG_FILES: Dir = include_dir!("resources/reg_files");

fn navbar() -> impl Widget<()> {
    let mut navbar = Flex::column();

    navbar.add_child(Label::new(
    "
    Criar um Ponto de Restauração:

    Antes de realizar qualquer ação que modifique
    o sistema,
    é recomendável criar um ponto de restauração. 
    Isso permitirá reverter o sistema para um estado
    anterior se algo der errado.
    
    Navegar e Executar Ações:
    A interface da ferramenta possui três colunas: 

    Uma para arquivos .bat, outra para arquivos .reg
    e uma terceira para ações adicionais.

    Na coluna Arquivos .bat e Arquivos .reg,
    você encontrará uma lista de scripts pré-configurados.

    Clique em um script para executá-lo.

    Na coluna Ações Adicionais, há botões para
    executar ações específicas,
    como otimizar a conexão de internet e ajustar
    configurações gerais do Windows."));

    // Você pode adicionar mais botões ou widgets aqui

    navbar
}

fn execute_file(file_type: &str, name: &str, content: &[u8]) -> Result<()> {
    let file_name = format!("temp_{}.{}", name, file_type);

    {
        let mut file = File::create(&file_name)?;
        file.write_all(content)?;
    } // O arquivo é fechado aqui ao sair do escopo

   // Para fins de depuração
   println!("Arquivo criado: {}", &file_name);

   match file_type {
    "bat" | "cmd" => {
        let status = Command::new("cmd")
            .args(&["/C", &file_name])
            .status()?;
        println!("Status: {:?}", status);
    }
    "reg" => {
        // Adicione um pequeno atraso (opcional)
        thread::sleep(Duration::from_millis(500));

        let status = Command::new("regedit")
            .args(&["/s", &file_name])
            .status()?;
        println!("Status: {:?}", status);
    }
    _ => {
        return Err(std::io::Error::new(
            std::io::ErrorKind::Other,
            "Tipo de arquivo desconhecido",
        ));
    }
}

// Remover o arquivo temporário (descomente quando estiver pronto)
 std::fs::remove_file(&file_name)?;

Ok(())
}

pub fn ui_builder() -> impl Widget<()> {
    let navbar = navbar();
    // Layout principal para as colunas
    let mut main_layout = Flex::row();

    // Primeira coluna para botões .bat
    let mut bat_column = Flex::column();
    let bat_label = Label::new("Arquivos .bat").with_text_size(24.0);
    bat_column.add_child(bat_label);
    for entry in BAT_FILES.files() {
        let name = entry.path().file_stem().unwrap().to_str().unwrap();
        let content = entry.contents();

        let button = create_button(name, move || {
            if let Err(err) = execute_file("bat", name, content) {
                eprintln!("Erro ao executar o arquivo BAT: {}", err);
            }
        });

        bat_column.add_child(button);
    }

    // Segunda coluna para botões .reg
    let mut reg_column = Flex::column();
    let reg_label = Label::new("Arquivos .reg").with_text_size(24.0);
    reg_column.add_child(reg_label);
    for entry in REG_FILES.files() {
        let name = entry.path().file_stem().unwrap().to_str().unwrap();
        let content = entry.contents();

        let button = create_button(name, move || {
            if let Err(err) = execute_file("reg", name, content) {
                eprintln!("Erro ao executar o arquivo REG: {}", err);
            }
        });

        reg_column.add_child(button);
    }

    // Terceira coluna para botões adicionais
    let mut additional_column = Flex::column();
    let additional_label = Label::new("Ações Adicionais").with_text_size(24.0);
    additional_column.add_child(additional_label);

    let button_internet = create_button("Melhorar Internet", melhorar_internet::otimizar_rede);
    let button_memoria = create_button("Reduzir Memória", reduzir_memoria::funcao_correspondente);
    let button_reg_otimizacao_geral = create_button(
        "Regedit - Otimização geral",
        || match otimizacao_geral::modify_registry() {
            Ok(()) => println!("Alterações aplicadas com sucesso."),
            Err(err) => eprintln!("Erro ao alterar o registro: {}", err),
        },
    );

    additional_column.add_child(button_internet);
    additional_column.add_child(button_memoria);
    additional_column.add_child(button_reg_otimizacao_geral);

        // Crie um widget Scroll com a terceira coluna
        let scrollable_additional_column = Scroll::new(reg_column);

    // Adicionando as colunas ao layout principal
    main_layout.add_flex_child(navbar, 1.0);
    main_layout.add_flex_child(bat_column, 1.0);
    main_layout.add_flex_child(additional_column, 1.0);  // Adicione a terceira coluna aqui
    main_layout.add_flex_child(scrollable_additional_column, 1.0);

    // Finaliza e retorna o layout principal
    main_layout
    
}


fn create_button(label_text: &str, on_click_action: impl Fn() + 'static) -> impl Widget<()> {
    let button = Button::new(label_text)
        .on_click(move |_ctx, _data, _env| on_click_action())
        .padding((10.0, 10.0))
        .fix_width(BUTTON_WIDTH)
        .fix_height(BUTTON_HEIGHT)
        .background(Color::rgb8(100, 100, 100))
        .rounded(5.0)
        .border(Color::BLACK, 2.0);

    Align::centered(button)
}

