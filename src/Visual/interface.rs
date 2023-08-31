use crate::Regs::inputlag_mouse_and_keyboard;
use crate::{melhorar_internet, otimizacao_geral, reduzir_memoria};
use druid::widget::{Align, Button, Flex, Label, SizedBox};
use druid::{AppLauncher, Color, LocalizedString, Widget, WidgetExt, WindowDesc};
use include_dir::{include_dir, Dir};
use std::fs::{remove_file, File};
use std::io::{Result, Write};
use std::path::Path;
use std::process::{exit, Command, Stdio};

const VERTICAL_WIDGET_SPACING: f64 = 20.0;
const HORIZONTAL_WIDGET_SPACING: f64 = 20.0;
const BUTTON_WIDTH: f64 = 300.0;
const BUTTON_HEIGHT: f64 = 50.0;

// Inclui os recursos
const BAT_FILES: Dir = include_dir!("resources/bat_files");
const REG_FILES: Dir = include_dir!("resources/reg_files");

fn execute_file(file_type: &str, name: &str, content: &[u8]) -> Result<()> {
    let file_name = format!("temp_{}.{}", name, file_type);
    let mut file = File::create(&file_name)?;
    file.write_all(content)?;

    match file_type {
        "bat" | "cmd" => {
            Command::new("cmd").args(&["/C", &file_name]).status()?;
        }
        "reg" => {
            Command::new("regedit").args(&["/s", &file_name]).status()?;
        }
        _ => {
            return Err(std::io::Error::new(
                std::io::ErrorKind::Other,
                "Tipo de arquivo desconhecido",
            ))
        }
    }

    // Remover o arquivo temporário
    remove_file(&file_name)?;

    Ok(())
}

pub fn ui_builder() -> impl Widget<()> {
    let mut layout = Flex::column();

    // Botões já existentes
    let button_internet = create_button("Melhorar Internet", melhorar_internet::otimizar_rede);
    let button_memoria = create_button("Reduzir Memória", reduzir_memoria::funcao_correspondente);
    let button_reg_otimizacao_geral =
        create_button(
            "Regedit - Otimização geral",
            || match otimizacao_geral::modify_registry() {
                Ok(()) => println!("Alterações aplicadas com sucesso."),
                Err(err) => eprintln!("Erro ao alterar o registro: {}", err),
            },
        );

    layout.add_child(button_internet);
    layout.add_child(button_memoria);
    layout.add_child(button_reg_otimizacao_geral);

    // Adicionando botões dinâmicos para arquivos .bat
    for entry in BAT_FILES.files() {
        let name = entry.path().file_stem().unwrap().to_str().unwrap();
        let content = entry.contents();

        let button = create_button(name, move || {
            if let Err(err) = execute_file("bat", name, content) {
                eprintln!("Erro ao executar o arquivo BAT: {}", err);
            }
        });

        layout.add_child(button);
    }

    // Adicionando botões dinâmicos para arquivos .reg
    for entry in REG_FILES.files() {
        let name = entry.path().file_stem().unwrap().to_str().unwrap();
        let content = entry.contents();

        let button = create_button(name, move || {
            if let Err(err) = execute_file("reg", name, content) {
                eprintln!("Erro ao executar o arquivo REG: {}", err);
            }
        });

        layout.add_child(button);
    }

    layout.add_spacer(VERTICAL_WIDGET_SPACING);
    layout.add_spacer(HORIZONTAL_WIDGET_SPACING);

    Align::centered(layout)
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
