use eframe::egui::{self, Style, Visuals};
use eframe::emath::vec2;
use egui_extras_xt::displays::{DisplayStylePreset, LedDisplay, SegmentedDisplayWidget};

use chrono::{DateTime, TimeZone, Timelike};
use chrono_tz::Tz;

struct TimeCircuitSegment {
    label: String,
    datetime: DateTime<Tz>,
    style_preset: DisplayStylePreset,
}

struct DeLoreanDemoApp {
    time_circuit_segments: Vec<TimeCircuitSegment>,
}

impl Default for DeLoreanDemoApp {
    fn default() -> Self {
        use chrono_tz::US::Pacific;

        Self {
            time_circuit_segments: vec![
                TimeCircuitSegment {
                    label: "DESTINATION TIME".to_owned(),
                    datetime: Pacific.ymd(1885, 1, 1).and_hms(12, 0, 0),
                    style_preset: DisplayStylePreset::DeLoreanRed,
                },
                TimeCircuitSegment {
                    label: "PRESENT TIME".to_owned(),
                    datetime: Pacific.ymd(1955, 11, 12).and_hms(9, 28, 0),
                    style_preset: DisplayStylePreset::DeLoreanGreen,
                },
                TimeCircuitSegment {
                    label: "LAST TIME DEPARTED".to_owned(),
                    datetime: Pacific.ymd(1985, 10, 27).and_hms(14, 42, 0),
                    style_preset: DisplayStylePreset::DeLoreanAmber,
                },
            ],
        }
    }
}

impl eframe::App for DeLoreanDemoApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            for TimeCircuitSegment {
                label,
                datetime,
                style_preset,
            } in &self.time_circuit_segments
            {
                let str_month = datetime.format("%b").to_string().to_uppercase();
                let str_day = datetime.format("%d").to_string();
                let str_year = datetime.format("%Y").to_string();
                let (ampm, _) = datetime.hour12();
                let str_hour = datetime.format("%I").to_string();
                let str_min = datetime.format("%M").to_string();
                let tick = datetime.time().nanosecond() < 500_000_000;

                ui.group(|ui| {
                    egui::Grid::new(label).min_col_width(20.0).show(ui, |ui| {
                        ui.vertical_centered(|ui| ui.label("MONTH"));
                        ui.vertical_centered(|ui| ui.label("DAY"));
                        ui.vertical_centered(|ui| ui.label("YEAR"));
                        ui.vertical_centered(|ui| ui.label(""));
                        ui.vertical_centered(|ui| ui.label("HOUR"));
                        ui.vertical_centered(|ui| ui.label(""));
                        ui.vertical_centered(|ui| ui.label("MIN"));
                        ui.end_row();

                        ui.add(
                            SegmentedDisplayWidget::sixteen_segment(&str_month)
                                .style_preset(*style_preset)
                                .show_dots(false)
                                .show_colons(false)
                                .show_apostrophes(false)
                                .digit_height(64.0),
                        );
                        ui.add(
                            SegmentedDisplayWidget::seven_segment(&str_day)
                                .style_preset(*style_preset)
                                .show_dots(true)
                                .show_colons(false)
                                .show_apostrophes(false)
                                .digit_height(64.0),
                        );
                        ui.add(
                            SegmentedDisplayWidget::seven_segment(&str_year)
                                .style_preset(*style_preset)
                                .show_dots(true)
                                .show_colons(false)
                                .show_apostrophes(false)
                                .digit_height(64.0),
                        );

                        ui.vertical_centered(|ui| {
                            ui.label("AM");
                            ui.add(
                                LedDisplay::from_bool(!ampm)
                                    .style_preset(*style_preset)
                                    .diameter(12.0),
                            );
                            ui.label("PM");
                            ui.add(
                                LedDisplay::from_bool(ampm)
                                    .style_preset(*style_preset)
                                    .diameter(12.0),
                            );
                        });

                        ui.add(
                            SegmentedDisplayWidget::seven_segment(&str_hour)
                                .style_preset(*style_preset)
                                .show_dots(true)
                                .show_colons(false)
                                .show_apostrophes(false)
                                .digit_height(64.0),
                        );

                        ui.vertical_centered(|ui| {
                            ui.add_space(15.0);
                            ui.add(
                                LedDisplay::from_bool(tick)
                                    .style_preset(*style_preset)
                                    .diameter(12.0),
                            );
                            ui.add_space(10.0);
                            ui.add(
                                LedDisplay::from_bool(tick)
                                    .style_preset(*style_preset)
                                    .diameter(12.0),
                            );
                        });

                        ui.add(
                            SegmentedDisplayWidget::seven_segment(&str_min)
                                .style_preset(*style_preset)
                                .show_dots(true)
                                .show_colons(false)
                                .show_apostrophes(false)
                                .digit_height(64.0),
                        );
                        ui.end_row();
                    });

                    ui.shrink_width_to_current();
                    ui.vertical_centered(|ui| {
                        ui.heading(label);
                    });
                });
            }
        });
    }
}

fn main() {
    let options = eframe::NativeOptions {
        initial_window_size: Some(vec2(1000.0, 500.0)),
        ..Default::default()
    };

    eframe::run_native(
        "DeLorean Time Machine",
        options,
        Box::new(|cc| {
            cc.egui_ctx.set_style(Style {
                visuals: Visuals::dark(),
                ..Style::default()
            });

            Box::new(DeLoreanDemoApp::default())
        }),
    );
}
