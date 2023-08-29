use druid::{AppLauncher, Widget, WindowDesc, LocalizedString, Data, Lens};
use druid::widget::{Button, Flex, Padding, Label, TextBox, Align};
use std::process::{Command, Stdio, exit};


use crate::Regs::inputlag_mouse_and_keyboard;
use crate::{melhorar_internet, reduzir_memoria, otimizacao_geral};

const VERTICAL_WIDGET_SPACING: f64 = 20.0;
const TEXT_BOX_WIDTH: f64 = 200.0;


pub fn ui_builder() -> impl Widget<()> {
    let button_internet = Button::new("Melhorar Internet")
        .on_click(|_ctx, _data, _env| melhorar_internet::otimizar_rede());
    
        let button_memoria = Button::new("Reduzir Memória")
        .on_click(|_ctx, _data, _env| reduzir_memoria::funcao_correspondente());

        let button_reg_otimizacao_geral = Button::new("Regedit - Otimização geral")
        .on_click(|_ctx, _data, _env| {
            match otimizacao_geral::modify_registry() {
                 Ok(()) => println!("Alterações aplicadas com sucesso."),
                 Err(err) => eprintln!("Erro ao alterar o registro: {}" , err),
                }
        });

        let inputlag_mouse_keyboard = Button::new("Regedit - Inputlag Mouse e Keyboard")
        .on_click(|_ctx, _data, _env| { 
            match inputlag_mouse_and_keyboard::inputlag_mouse_keyboard() {
                    Ok(()) => println!("Alterações aplicadas com sucesso."),
                    Err(err) => eprintln!("Erro ao alterar o registro: {}" , err),
                }
            });




    let column = Flex::column()
        .with_child(button_internet)
        .with_child(button_memoria)
        .with_child(button_reg_otimizacao_geral)
        .with_child(inputlag_mouse_keyboard)
        .with_spacer(VERTICAL_WIDGET_SPACING);

            
        Align::centered(column)
    //Padding::new(20.0, column)
}