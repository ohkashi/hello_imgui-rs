#![windows_subsystem = "windows"]

use imgui::*;

mod support;

macro_rules! rgb {
    ($r:expr, $g:expr, $b:expr) => {[ $r as f32 / 255f32, $g as f32 / 255f32, $b as f32 / 255f32, 1.0 ]};
}

#[allow(unused_macros)]
macro_rules! rgba {
    ($r:expr, $g:expr, $b:expr, $a:expr) => {[ $r as f32 / 255f32, $g as f32 / 255f32, $b as f32 / 255f32, $a as f32 ]};
}

fn main() {
    let bgcolor = rgb!(37, 37, 38);
    let light_bgcolor = rgb!(82, 82, 85);
    let verylight_bgcolor = rgb!(100, 100, 105);
    let hilight_bgcolor = rgb!(128, 128, 135);

    let panel_color = rgb!(51, 51, 55);
    let panel_hover_color = rgb!(68, 68, 68); //rgb!(29, 151, 236);
    let panel_active_color = rgb!(54, 54, 54); //rgb!(0, 119, 200);
    let blue_active_color = rgb!(0, 119, 200);
    let blue_hover_color = rgb!(29, 151, 236);

    let text_color = rgb!(255, 255, 255);
    let text_disabled_color = rgb!(151, 151, 151);
    let border_color = rgb!(78, 78, 78);

    let mut system = support::init(env!("CARGO_PKG_NAME"),
        1024, 768, support::SystemColor::from_rgb(24, 24, 24));

    let style = system.imgui.style_mut();

    style.window_border_size = 1.0;
    style.window_rounding = 4.0;
    style.frame_rounding = 4.0;
    style.popup_border_size = 1.0;
    style.popup_rounding = 4.0;

    style.colors[StyleColor::Text as usize] = text_color;
    style.colors[StyleColor::TextDisabled as usize] = text_disabled_color;
    style.colors[StyleColor::TextSelectedBg as usize] = panel_active_color;
    style.colors[StyleColor::WindowBg as usize] = bgcolor;
    style.colors[StyleColor::ChildBg as usize] = bgcolor;
    style.colors[StyleColor::PopupBg as usize] = bgcolor;
    style.colors[StyleColor::Border as usize] = border_color;
    style.colors[StyleColor::BorderShadow as usize] = border_color;
    style.colors[StyleColor::FrameBg as usize] = panel_color;
    style.colors[StyleColor::FrameBgHovered as usize] = panel_hover_color;
    style.colors[StyleColor::FrameBgActive as usize] = light_bgcolor;// panelActiveColor;
    style.colors[StyleColor::TitleBg as usize] = bgcolor;
    style.colors[StyleColor::TitleBgActive as usize] = blue_active_color;// bgColor;
    style.colors[StyleColor::TitleBgCollapsed as usize] = bgcolor;
    style.colors[StyleColor::MenuBarBg as usize] = panel_color;
    style.colors[StyleColor::ScrollbarBg as usize] = panel_color;
    style.colors[StyleColor::ScrollbarGrab as usize] = light_bgcolor;
    style.colors[StyleColor::ScrollbarGrabHovered as usize] = verylight_bgcolor;
    style.colors[StyleColor::ScrollbarGrabActive as usize] = hilight_bgcolor;
    style.colors[StyleColor::CheckMark as usize] = blue_active_color;// panelActiveColor;
    style.colors[StyleColor::SliderGrab as usize] = blue_active_color;// panelHoverColor;
    style.colors[StyleColor::SliderGrabActive as usize] = blue_hover_color;
    style.colors[StyleColor::Button as usize] = panel_color;
    style.colors[StyleColor::ButtonHovered as usize] = panel_hover_color;
    style.colors[StyleColor::ButtonActive as usize] = blue_active_color;// panelHoverColor;
    style.colors[StyleColor::Header as usize] = panel_active_color;// panelColor;
    style.colors[StyleColor::HeaderHovered as usize] = panel_hover_color;
    style.colors[StyleColor::HeaderActive as usize] = blue_active_color;// panelActiveColor;
    style.colors[StyleColor::Separator as usize] = border_color;
    style.colors[StyleColor::SeparatorHovered as usize] = border_color;
    style.colors[StyleColor::SeparatorActive as usize] = border_color;
    style.colors[StyleColor::ResizeGrip as usize] = bgcolor;
    style.colors[StyleColor::ResizeGripHovered as usize] = panel_color;
    style.colors[StyleColor::ResizeGripActive as usize] = light_bgcolor;
    style.colors[StyleColor::PlotLines as usize] = blue_active_color;// panelActiveColor;
    style.colors[StyleColor::PlotLinesHovered as usize] = blue_hover_color;// panelHoverColor;
    style.colors[StyleColor::PlotHistogram as usize] = blue_active_color;// panelActiveColor;
    style.colors[StyleColor::PlotHistogramHovered as usize] = blue_hover_color;// panelHoverColor;
    style.colors[StyleColor::ModalWindowDimBg as usize] = bgcolor;
    style.colors[StyleColor::DragDropTarget as usize] = bgcolor;
    style.colors[StyleColor::NavHighlight as usize] = bgcolor;
    //colors[StyleColor::l_DockingPreview as usize] = panel_active_color;
    style.colors[StyleColor::Tab as usize] = bgcolor;
    style.colors[StyleColor::TabActive as usize] = panel_active_color;
    style.colors[StyleColor::TabUnfocused as usize] = bgcolor;
    style.colors[StyleColor::TabUnfocusedActive as usize] = panel_active_color;
    style.colors[StyleColor::TabHovered as usize] = panel_hover_color;

    let mut value = 0;
    let choices = ["test test this is 1", "test test this is 2"];
    let mut stable_str = String::new();

    system.main_loop(move |_, ui| {
        ui.window("Hello world")
            .size([300.0, 200.0], Condition::FirstUseEver)
            .build(|| {
                ui.text_wrapped("Hello world!");
                ui.text_wrapped("안녕 세상아!");
                ui.text_wrapped("こんにちは世界！");
                if ui.button(choices[value]) {
                    value += 1;
                    value %= 2;
                }

                if ui.input_text("input stable", &mut stable_str).build() {
                    dbg!(&stable_str);
                }

                ui.button("This...is...imgui-rs!");
                ui.separator();
                let mouse_pos = ui.io().mouse_pos;
                ui.text(format!(
                    "Mouse Position: ({:.1},{:.1})",
                    mouse_pos[0], mouse_pos[1]
                ));
            });
    });
}
