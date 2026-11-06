use eframe::{
    egui::{style::Selection, Visuals},
    epaint::{Color32, Stroke, Vec2},
};

pub struct Theme {
    pub spacing: SpacingStyle,
    pub visuals: Visuals,
}

impl Default for Theme {
    fn default() -> Self {
        let mut visuals = Visuals::dark();
        
        visuals.window_corner_radius = eframe::egui::CornerRadius::same(12);
        visuals.menu_corner_radius = eframe::egui::CornerRadius::same(8);
        
        let bg_color = Color32::from_rgb(20, 20, 20);
        let panel_bg_color = Color32::from_rgb(28, 28, 28);
        let accent_color = Color32::from_rgb(64, 144, 255);
        let hover_color = Color32::from_rgb(84, 164, 255);
        let active_color = Color32::from_rgb(44, 124, 255);
        let text_color = Color32::from_gray(240);
        let border_color = Color32::from_rgb(45, 45, 45);

        visuals.window_fill = bg_color;
        visuals.panel_fill = panel_bg_color;
        
        visuals.selection = Selection {
            bg_fill: accent_color,
            stroke: Stroke::new(1.0, hover_color),
        };

        visuals.widgets.noninteractive.bg_fill = panel_bg_color;
        visuals.widgets.noninteractive.fg_stroke = Stroke::new(1.0, text_color);
        visuals.widgets.noninteractive.bg_stroke = Stroke::new(1.0, border_color);
        visuals.widgets.noninteractive.corner_radius = eframe::egui::CornerRadius::same(8);

        visuals.widgets.inactive.bg_fill = Color32::from_rgb(35, 35, 35);
        visuals.widgets.inactive.fg_stroke = Stroke::new(1.0, text_color);
        visuals.widgets.inactive.bg_stroke = Stroke::new(1.0, border_color);
        visuals.widgets.inactive.corner_radius = eframe::egui::CornerRadius::same(8);

        visuals.widgets.hovered.bg_fill = Color32::from_rgb(45, 45, 45);
        visuals.widgets.hovered.fg_stroke = Stroke::new(1.0, text_color);
        visuals.widgets.hovered.bg_stroke = Stroke::new(1.0, hover_color);
        visuals.widgets.hovered.corner_radius = eframe::egui::CornerRadius::same(8);

        visuals.widgets.active.bg_fill = active_color;
        visuals.widgets.active.fg_stroke = Stroke::new(1.0, Color32::WHITE);
        visuals.widgets.active.bg_stroke = Stroke::new(1.0, active_color);
        visuals.widgets.active.corner_radius = eframe::egui::CornerRadius::same(8);

        visuals.widgets.open.bg_fill = Color32::from_rgb(35, 35, 35);
        visuals.widgets.open.fg_stroke = Stroke::new(1.0, text_color);
        visuals.widgets.open.bg_stroke = Stroke::new(1.0, border_color);
        visuals.widgets.open.corner_radius = eframe::egui::CornerRadius::same(8);

        Self {
            spacing: SpacingStyle::default(),
            visuals,
        }
    }
}

pub struct SpacingStyle {
    pub default: Vec2,
    pub large: f32,
    pub medium: f32,
}

impl Default for SpacingStyle {
    fn default() -> Self {
        Self {
            default: Vec2::new(8.0, 5.0),
            large: 20.0,
            medium: 8.0,
        }
    }
}
