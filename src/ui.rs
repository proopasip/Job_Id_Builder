//! The egui user interface.
//!
//! The window is a sidebar of filters plus a main area holding the search box,
//! the results table and a status line. Network calls run on a worker thread
//! and report back over a channel, so the GUI never blocks.

use std::sync::mpsc::{self, Receiver, TryRecvError};
use std::time::Duration;

use eframe::egui;

use crate::domain::{self, ScanDepth, Server};
use crate::i18n::{self, Lang, Strings};
use crate::net;

/// Scan depth options offered in the sidebar.
const SCAN_OPTIONS: [usize; 7] = [10, 25, 50, 100, 250, 500, 1000];

/// Everything that should survive a restart.
#[derive(Debug, Clone, serde::Deserialize, serde::Serialize)]
#[serde(default)]
struct Settings {
    /// Stored as a [`Lang::code`] string so that `i18n` stays dependency-free.
    lang_code: String,
    dark: bool,
    /// How many servers to scan for.
    scan: usize,
    ping_limit_enabled: bool,
    ping_limit: u32,
    min_players: u32,
}

impl Default for Settings {
    fn default() -> Self {
        Self {
            lang_code: Lang::default().code().to_owned(),
            dark: true,
            scan: 100,
            ping_limit_enabled: true,
            ping_limit: 500,
            min_players: 0,
        }
    }
}

#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
enum StatusKind {
    #[default]
    Info,
    Success,
    Error,
}

#[derive(Debug, Clone, Default)]
struct Status {
    text: String,
    kind: StatusKind,
}

/// The application.
pub struct App {
    settings: Settings,

    /// Whatever the user typed into the search box.
    input: String,
    /// Place id the current result set belongs to.
    place_id: Option<u64>,
    /// Every server the API returned, unfiltered.
    servers: Vec<Server>,
    /// Job id of the server the user picked.
    selected: Option<String>,

    status: Status,
    settings_open: bool,
    loading: bool,
    /// Receiving end of the worker thread's result, while a scan is in flight.
    results: Option<Receiver<Result<Vec<Server>, net::Error>>>,
    /// Theme already pushed into egui, so the style is only touched on change.
    applied_dark: Option<bool>,
}

impl App {
    /// Build the app, restoring settings from `eframe`'s persistent storage.
    #[must_use]
    pub fn new(cc: &eframe::CreationContext<'_>) -> Self {
        let settings: Settings = cc
            .storage
            .and_then(|storage| eframe::get_value(storage, eframe::APP_KEY))
            .unwrap_or_default();

        // Start with the localised "ready" message rather than an empty bar.
        let status = Status {
            text: i18n::strings(Lang::from_code(&settings.lang_code))
                .status_ready
                .to_owned(),
            kind: StatusKind::Info,
        };

        Self {
            settings,
            input: String::new(),
            place_id: None,
            servers: Vec::new(),
            selected: None,
            status,
            settings_open: false,
            loading: false,
            results: None,
            applied_dark: None,
        }
    }

    fn lang(&self) -> Lang {
        Lang::from_code(&self.settings.lang_code)
    }

    fn strings(&self) -> &'static Strings {
        i18n::strings(self.lang())
    }

    fn filters(&self) -> domain::Filters {
        domain::Filters {
            max_ping: self
                .settings
                .ping_limit_enabled
                .then_some(self.settings.ping_limit),
            min_players: self.settings.min_players,
        }
    }

    fn depth(&self) -> ScanDepth {
        ScanDepth {
            requested: self.settings.scan.max(1),
        }
    }

    /// The servers matching the current filters, best ping first.
    fn visible(&self) -> Vec<Server> {
        domain::apply_filters(&self.servers, self.filters())
    }

    fn set_status(&mut self, text: impl Into<String>, kind: StatusKind) {
        self.status = Status {
            text: text.into(),
            kind,
        };
    }

    /// Collect whatever the worker thread has sent us, if anything.
    fn poll(&mut self) {
        let Some(receiver) = self.results.take() else {
            return;
        };

        match receiver.try_recv() {
            Ok(Ok(servers)) => {
                self.loading = false;
                let shown = domain::apply_filters(&servers, self.filters()).len();
                let total = servers.len();
                self.servers = servers;

                let strings = self.strings();
                if total == 0 {
                    self.set_status(strings.status_no_servers, StatusKind::Error);
                } else if shown == 0 {
                    self.set_status(strings.status_filtered, StatusKind::Error);
                } else {
                    self.set_status(
                        i18n::status_found(self.lang(), shown),
                        StatusKind::Success,
                    );
                }
            }
            Ok(Err(err)) => {
                self.loading = false;
                let detail = err.to_string();
                self.set_status(
                    i18n::status_error(self.lang(), &detail),
                    StatusKind::Error,
                );
            }
            // Still working — put the receiver back and check again next frame.
            Err(TryRecvError::Empty) => self.results = Some(receiver),
            Err(TryRecvError::Disconnected) => {
                self.loading = false;
                self.set_status(
                    i18n::status_error(self.lang(), "the scan thread stopped unexpectedly"),
                    StatusKind::Error,
                );
            }
        }
    }

    /// Kick off a scan on a worker thread.
    fn start_scan(&mut self, ctx: &egui::Context) {
        let Some(place_id) = domain::extract_place_id(&self.input) else {
            self.set_status(self.strings().invalid_place_id, StatusKind::Error);
            return;
        };

        let (sender, receiver) = mpsc::channel();
        self.results = Some(receiver);
        self.loading = true;
        self.servers.clear();
        self.selected = None;
        self.place_id = Some(place_id);
        self.set_status(self.strings().status_searching, StatusKind::Info);

        let depth = self.depth();
        let ctx = ctx.clone();

        std::thread::spawn(move || {
            // Building the client on the worker keeps the GUI thread responsive.
            let outcome =
                net::Client::new().and_then(|client| client.public_servers(place_id, depth));

            let _ = sender.send(outcome);
            // Wake the GUI so the result is painted immediately.
            ctx.request_repaint();
        });
    }

    /// Push the chosen theme into egui, but only when it actually changed.
    fn apply_theme(&mut self, ctx: &egui::Context) {
        if self.applied_dark != Some(self.settings.dark) {
            self.applied_dark = Some(self.settings.dark);
            let visuals = if self.settings.dark {
                egui::Visuals::dark()
            } else {
                egui::Visuals::light()
            };
            ctx.set_visuals(visuals);
        }
    }

    // ---------------------------------------------------------------- panels

    fn sidebar(&mut self, ui: &mut egui::Ui) {
        let strings = self.strings();

        ui.heading("🚀 ServerSearch");
        ui.add_space(4.0);

        if ui.button(format!("⚙️ {}", strings.settings)).clicked() {
            self.settings_open = true;
        }

        ui.separator();
        ui.add_space(6.0);

        ui.label(strings.limit);
        egui::ComboBox::from_id_salt("scan-depth")
            .selected_text(self.settings.scan.to_string())
            .show_ui(ui, |ui| {
                for option in SCAN_OPTIONS {
                    ui.selectable_value(&mut self.settings.scan, option, option.to_string());
                }
            });

        ui.add_space(10.0);

        ui.label(strings.ping_filter);
        ui.horizontal(|ui| {
            ui.checkbox(&mut self.settings.ping_limit_enabled, "");
            ui.add_enabled(
                self.settings.ping_limit_enabled,
                egui::DragValue::new(&mut self.settings.ping_limit)
                    .range(1..=2000)
                    .suffix(" ms"),
            );
            if !self.settings.ping_limit_enabled {
                ui.weak(strings.no_limit);
            }
        });

        ui.add_space(10.0);

        ui.label(strings.player_filter);
        ui.add(egui::DragValue::new(&mut self.settings.min_players).range(0..=200));
    }

    fn main_area(&mut self, ui: &mut egui::Ui) {
        let strings = self.strings();

        // --- search row -----------------------------------------------------
        ui.horizontal(|ui| {
            ui.label(strings.game_input);

            let response = ui.add(
                egui::TextEdit::singleline(&mut self.input)
                    .hint_text("https://www.roblox.com/games/…")
                    .desired_width(f32::INFINITY),
            );

            let pressed_enter =
                response.lost_focus() && ui.ctx().input(|i| i.key_pressed(egui::Key::Enter));
            let clicked_find = ui
                .add_enabled(!self.loading, egui::Button::new(strings.fetch_btn))
                .clicked();

            if !self.loading && (clicked_find || pressed_enter) {
                self.start_scan(ui.ctx());
            }
        });

        ui.add_space(8.0);

        // --- chosen server --------------------------------------------------
        match (self.selected.clone(), self.place_id) {
            (Some(job_id), Some(place_id)) => {
                let link = domain::join_link(place_id, &job_id);
                ui.horizontal(|ui| {
                    ui.monospace(link.as_str());
                    if ui.button(strings.copy_btn).clicked() {
                        ui.ctx().copy_text(link.clone());
                        self.set_status(strings.status_copied, StatusKind::Success);
                    }
                    if ui.button(strings.launch).clicked() {
                        open_with_os(&link);
                    }
                    // `roblox://` can't be pasted into a browser, so offer a
                    // normal https link to the game as well.
                    if ui.button(strings.game_page).clicked() {
                        open_with_os(&domain::web_link(place_id));
                    }
                });
            }
            _ => {
                ui.weak(strings.result_display);
            }
        }

        ui.add_space(8.0);

        // --- results --------------------------------------------------------
        // Filtered once up front: borrowing `self` inside the table closure is
        // only possible because the rows come from a local.
        let visible = self.visible();

        egui::ScrollArea::vertical()
            .auto_shrink([false, false])
            .show(ui, |ui| {
                egui::Grid::new("servers")
                    .num_columns(4)
                    .striped(true)
                    .spacing([24.0, 6.0])
                    .min_col_width(50.0)
                    .show(ui, |ui| {
                        ui.strong(strings.col_ping);
                        ui.strong(strings.col_players);
                        ui.strong(strings.col_job_id);
                        ui.label("");
                        ui.end_row();

                        let mut launched: Option<String> = None;

                        for server in &visible {
                            ui.label(server.ping_label());
                            ui.label(server.occupancy());
                            ui.monospace(server.id.as_str());
                            if ui.small_button(strings.launch).clicked() {
                                launched = Some(server.id.clone());
                            }
                            ui.end_row();
                        }

                        if let Some(job_id) = launched {
                            self.selected = Some(job_id);
                        }
                    });
            });

        // --- status line ----------------------------------------------------
        ui.separator();
        let colour = match self.status.kind {
            StatusKind::Info => egui::Color32::GRAY,
            StatusKind::Success => egui::Color32::from_rgb(0x43, 0xb5, 0x81),
            StatusKind::Error => egui::Color32::from_rgb(0xf0, 0x47, 0x47),
        };
        ui.colored_label(colour, self.status.text.as_str());
    }

    fn settings_window(&mut self, ctx: &egui::Context) {
        let strings = self.strings();
        let mut open = self.settings_open;
        // `.open()` holds a mutable borrow of `open` for as long as the window
        // is being drawn, so the close button can't assign to it directly. It
        // raises a flag instead, which is applied once the window is finished.
        let mut close_requested = false;

        egui::Window::new(strings.settings_title)
            .open(&mut open)
            // `Window::resizable` takes a `Vec2b` (per-axis), not a plain bool.
            .resizable([false, false])
            .collapsible(false)
            .show(ctx, |ui| {
                ui.strong(strings.lang_title);
                ui.add_space(6.0);
                ui.horizontal_wrapped(|ui| {
                    for language in Lang::ALL {
                        let selected = self.lang() == language;
                        if ui
                            .selectable_label(selected, language.code())
                            .on_hover_text(language.native_name())
                            .clicked()
                        {
                            self.settings.lang_code = language.code().to_owned();
                        }
                    }
                });

                ui.add_space(12.0);
                ui.separator();

                ui.strong(strings.theme_title);
                ui.add_space(6.0);
                ui.selectable_value(&mut self.settings.dark, true, strings.theme_dark);
                ui.selectable_value(&mut self.settings.dark, false, strings.theme_light);

                ui.add_space(12.0);
                ui.separator();

                if ui.button(strings.settings_close).clicked() {
                    close_requested = true;
                }
            });

        if close_requested {
            open = false;
        }
        self.settings_open = open;
    }
}

impl eframe::App for App {
    /// Called by eframe when it is time to persist state.
    fn save(&mut self, storage: &mut dyn eframe::Storage) {
        self.settings.lang_code = self.lang().code().to_owned();
        eframe::set_value(storage, eframe::APP_KEY, &self.settings);
    }

    /// Runs before every repaint, including while the worker is still busy.
    fn logic(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        self.poll();
        if self.loading {
            // Safety net: keeps frames coming while the scan is in flight.
            ctx.request_repaint_after(Duration::from_millis(250));
        }
    }

    fn ui(&mut self, ui: &mut egui::Ui, _frame: &mut eframe::Frame) {
        self.apply_theme(ui.ctx());

        egui::Panel::left("sidebar")
            .exact_size(280.0)
            .resizable(false)
            .show(ui, |ui| self.sidebar(ui));

        egui::CentralPanel::default().show(ui, |ui| self.main_area(ui));

        self.settings_window(ui.ctx());
    }
}

/// Hand a `roblox://` link to the operating system so the client launches.
///
/// The link is handed to the platform's URL opener rather than to egui's browser
/// helper, because web browsers refuse to open custom protocols.
fn open_with_os(url: &str) {
    let spawned = if cfg!(target_os = "windows") {
        std::process::Command::new("cmd")
            .args(["/C", "start", "", url])
            .spawn()
    } else if cfg!(target_os = "macos") {
        std::process::Command::new("open").arg(url).spawn()
    } else {
        std::process::Command::new("xdg-open").arg(url).spawn()
    };

    if let Err(err) = spawned {
        eprintln!("could not launch the Roblox client: {err}");
    }
}
