use imgui::*;

use std::fs::File;
use std::io::prelude::*;
use std::path::Path;

use serde::{Serialize, Deserialize};
use serde_json::{Result, Value};

#[derive(Serialize, Deserialize)]
pub struct JsonStyle {
    Text: [f32; 4],
    TextDisabled: [f32; 4],
    WindowBg: [f32; 4],
    ChildBg: [f32; 4],
    PopupBg: [f32; 4],
    Border: [f32; 4],
    BorderShadow: [f32; 4],
    FrameBg: [f32; 4],
    FrameBgHovered: [f32; 4],
    FrameBgActive: [f32; 4],
    TitleBg: [f32; 4],
    TitleBgActive: [f32; 4],
    TitleBgCollapsed: [f32; 4],
    MenuBarBg: [f32; 4],
    ScrollbarBg: [f32; 4],
    ScrollbarGrab: [f32; 4],
    ScrollbarGrabHovered: [f32; 4],
    ScrollbarGrabActive: [f32; 4],
    CheckMark: [f32; 4],
    SliderGrab: [f32; 4],
    SliderGrabActive: [f32; 4],
    Button: [f32; 4],
    ButtonHovered: [f32; 4],
    ButtonActive: [f32; 4],
    Header: [f32; 4],
    HeaderHovered: [f32; 4],
    HeaderActive: [f32; 4],
    Separator: [f32; 4],
    SeparatorHovered: [f32; 4],
    SeparatorActive: [f32; 4],
    ResizeGrip: [f32; 4],
    ResizeGripHovered: [f32; 4],
    ResizeGripActive: [f32; 4],
    PlotLines: [f32; 4],
    PlotLinesHovered: [f32; 4],
    PlotHistogram: [f32; 4],
    PlotHistogramHovered: [f32; 4],
    TextSelectedBg: [f32; 4],
    ModalWindowDimBg: [f32; 4],
    DragDropTarget: [f32; 4],
    NavHighlight: [f32; 4],
    NavWindowingHighlight: [f32; 4],
}

pub fn style_ui(style: &mut imgui::Style) {
    let path = Path::new("ui_style.json");
    let display = path.display();

    let mut file = match File::open(&path) {
        Err(why) => panic!("Couldn't open {}: {}", display, why),
        Ok(file) => file,
    };

    let mut content = String::new();
    match file.read_to_string(&mut content) {
        Err(why) => panic!("Couldn't read {}: {}", display, why),
        Ok(_) => {},
    }

    let json_style: JsonStyle = serde_json::from_str(&content).unwrap();

    style.window_rounding = 0.0;
    style.window_border_size = 0.0;
    style.use_dark_colors();

    //Colours
    style.colors[StyleColor::Text as usize]                 = json_style.Text;
    style.colors[StyleColor::TextDisabled as usize]         = json_style.TextDisabled;
    style.colors[StyleColor::WindowBg as usize]             = json_style.WindowBg;
    style.colors[StyleColor::ChildBg as usize]              = json_style.ChildBg;
    style.colors[StyleColor::PopupBg as usize]              = json_style.PopupBg;
    style.colors[StyleColor::Border as usize]               = json_style.Border;
    style.colors[StyleColor::BorderShadow as usize]         = json_style.BorderShadow;
    style.colors[StyleColor::FrameBg as usize]              = json_style.FrameBg;
    style.colors[StyleColor::FrameBgHovered as usize]       = json_style.FrameBgHovered;
    style.colors[StyleColor::FrameBgActive as usize]        = json_style.FrameBgActive;
    style.colors[StyleColor::TitleBg as usize]              = json_style.TitleBg;
    style.colors[StyleColor::TitleBgActive as usize]        = json_style.TitleBgActive;
    style.colors[StyleColor::TitleBgCollapsed as usize]     = json_style.TitleBgCollapsed;
    style.colors[StyleColor::MenuBarBg as usize]            = json_style.MenuBarBg;
    style.colors[StyleColor::ScrollbarBg as usize]          = json_style.ScrollbarBg;
    style.colors[StyleColor::ScrollbarGrab as usize]        = json_style.ScrollbarGrab;
    style.colors[StyleColor::ScrollbarGrabHovered as usize] = json_style.ScrollbarGrabHovered;
    style.colors[StyleColor::ScrollbarGrabActive as usize]  = json_style.ScrollbarGrabActive;
    style.colors[StyleColor::CheckMark as usize]            = json_style.CheckMark;
    style.colors[StyleColor::SliderGrab as usize]           = json_style.SliderGrab;
    style.colors[StyleColor::SliderGrabActive as usize]     = json_style.SliderGrabActive;
    style.colors[StyleColor::Button as usize]               = json_style.Button;
    style.colors[StyleColor::ButtonHovered as usize]        = json_style.ButtonHovered;
    style.colors[StyleColor::ButtonActive as usize]         = json_style.ButtonActive;
    style.colors[StyleColor::Header as usize]               = json_style.Header;
    style.colors[StyleColor::HeaderHovered as usize]        = json_style.HeaderHovered;
    style.colors[StyleColor::HeaderActive as usize]         = json_style.HeaderActive;
    style.colors[StyleColor::Separator as usize]            = json_style.Separator;
    style.colors[StyleColor::ResizeGrip as usize]           = json_style.ResizeGrip;
    style.colors[StyleColor::ResizeGripHovered as usize]    = json_style.ResizeGripHovered;
    style.colors[StyleColor::ResizeGripActive as usize]     = json_style.ResizeGripActive;
    style.colors[StyleColor::PlotLines as usize]            = json_style.PlotLines;
    style.colors[StyleColor::PlotLinesHovered as usize]     = json_style.PlotLinesHovered;
    style.colors[StyleColor::PlotHistogram as usize]        = json_style.PlotHistogram;
    style.colors[StyleColor::PlotHistogramHovered as usize] = json_style.PlotHistogramHovered;
    style.colors[StyleColor::TextSelectedBg as usize]       = json_style.TextSelectedBg;
    style.colors[StyleColor::ModalWindowDimBg as usize]     = json_style.ModalWindowDimBg;
    style.colors[StyleColor::DragDropTarget as usize]       = json_style.DragDropTarget;
    style.colors[StyleColor::NavHighlight as usize]         = json_style.NavHighlight;
    style.colors[StyleColor::NavWindowingHighlight as usize]= json_style.NavWindowingHighlight;
}
