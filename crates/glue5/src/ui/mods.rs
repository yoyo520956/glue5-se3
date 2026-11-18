//! Mods: switched on or off here; the mods in the user directory's `mods`
//! folder, started with the game or during it; their menus while it runs;
//! GLUE 5 Studio (the web IDE the app serves) and its control page (the
//! mods' menus, for a phone) with their addresses, the pairing code, a QR
//! code and the paired browsers and agents; the mods' log.

use egui::{Color32, CornerRadius, RichText, Sense, vec2};
use glue5_mods::ModState;

use super::menus::{MenusView, Out};
use super::pages::{card_title, sheet, small};
use super::widgets::{Control, action_button, action_buttons, inline_button, segmented, setting, toggle};
use super::{Status, WARN, theme};
use crate::settings::Settings;

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum Tab {
    Menus,
    #[default]
    Mods,
    Studio,
    Log,
}

/// What the page keeps between frames.
#[derive(Default)]
pub struct ModsPage {
    tab: Tab,
    menus: MenusView,
}

impl ModsPage {
    /// Opens on the mods' menus (where the player was in them).
    pub fn show_menus(&mut self) {
        self.tab = Tab::Menus;
    }

    /// Opens at menu `menu` of mod `module` (`menu:show()`). False when
    /// there is no such menu.
    pub fn show_menu(&mut self, module: &str, menu: u32) -> bool {
        let shown = self.menus.open_at(&glue5_mods::menus(), module, menu);
        if shown {
            self.tab = Tab::Menus;
        }
        shown
    }

    /// Back (B, Esc) inside the page: false when the page should close.
    pub fn back(&mut self, ctx: &egui::Context) -> bool {
        self.tab == Tab::Menus && self.menus.back(ctx)
    }

    /// The page closed: its menus are no longer looked at.
    pub fn closed(&mut self) {
        send(self.menus.hidden());
    }
}

/// What the player did with the menus, to the runtime.
fn send(outs: Vec<Out>) {
    for out in outs {
        match out {
            Out::Input { module, item, input } => {
                if let Err(e) = glue5_mods::menu_input(&module, item, input) {
                    log::debug!("mods: menu input: {e}");
                }
            }
            Out::Viewer { module, menu, open } => glue5_mods::menu_viewer(&module, menu, open),
        }
    }
}

/// The Studio as it serves, for the page.
pub struct StudioInfo {
    /// The Studio's address.
    pub url: String,
    /// The control page's address.
    pub control: String,
    /// The pairing code as people type it.
    pub code: String,
    pub lan: bool,
}

impl StudioInfo {
    /// `url` with the code to pair: the page opens paired (this device's
    /// browser, a phone's camera).
    pub fn paired(&self, url: &str) -> String {
        format!("{url}?pair={}", self.code)
    }
}

const ERR: Color32 = Color32::from_rgb(240, 149, 149);

/// The mods in a line, for Settings: off, on, what runs.
pub(super) fn summary(settings: &Settings, status: &Status) -> String {
    match status.started.as_ref() {
        Some(s) if s.mods != settings.mods => {
            return if settings.mods { "下次启动时开启".into() } else { "下次启动时关闭".into() };
        }
        _ => {}
    }
    if !settings.mods {
        return "关闭 · 改变游戏的脚本".into();
    }
    let mods = glue5_mods::mods();
    let mut parts = vec!["On".to_owned()];
    match mods.len() {
        0 => {}
        1 => parts.push("1 mod".into()),
        n => parts.push(format!("{n} mods")),
    }
    let running = mods.iter().filter(|m| m.state == ModState::Running).count();
    if running > 0 {
        parts.push(format!("{running} running"));
    }
    if status.studio.is_some() {
        parts.push("Studio 已开启".into());
    }
    parts.join(" · ")
}

/// The start page's card while mods are on: what there is, and the way
/// into the Studio.
pub(super) fn home_card(ui: &mut egui::Ui, settings: &Settings, status: &Status, go: &mut Option<Option<super::Page>>) {
    let state = match status.started.as_ref() {
        Some(s) if !s.mods => ("下次启动", WARN),
        _ => ("On", super::pages::READY),
    };
    card_title(ui, "MODS", Some(state));
    let mods = glue5_mods::mods();
    let running = mods.iter().filter(|m| m.state == ModState::Running).count();
    let mut line = match mods.len() {
        0 => "暂无模组".to_owned(),
        1 => "1 mod".to_owned(),
        n => format!("{n} mods"),
    };
    if running > 0 {
        line.push_str(&format!(" · {running} running"));
    }
    match &status.studio {
        Some(info) => {
            line.push_str(if info.lan { " · Studio 在本网络中" } else { " · Studio 在本机" });
            small(ui, &line, theme::MUTED);
            let address = info.url.trim_end_matches('/').to_owned();
            ui.label(RichText::new(format!("{address}  ·  code {}", info.code)).monospace().color(Color32::WHITE));
            let [open, control, copy, page] = <[_; 4]>::try_from(
                ui.horizontal(|ui| action_buttons(ui, &["打开 Studio", "Control", "复制地址", "模组"])).inner,
            )
            .unwrap();
            if open.on_hover_text("在本机浏览器中").clicked() {
                ui.ctx().open_url(egui::OpenUrl::new_tab(info.paired(&info.url)));
            }
            if control.on_hover_text("模组菜单，在本机浏览器中").clicked() {
                ui.ctx().open_url(egui::OpenUrl::new_tab(info.paired(&info.control)));
            }
            if copy.clicked() {
                ui.ctx().copy_text(info.url.clone());
            }
            if page.clicked() {
                *go = Some(Some(super::Page::Mods));
            }
        }
        None => {
            if !settings.studio {
                line.push_str(" · Studio 关闭");
            }
            small(ui, &line, theme::MUTED);
            if ui.horizontal(|ui| action_button(ui, "模组")).inner.clicked() {
                *go = Some(Some(super::Page::Mods));
            }
        }
    }
}

/// True when closed.
pub(super) fn mods_page(ctx: &egui::Context, page: &mut ModsPage, settings: &mut Settings, status: &Status) -> bool {
    // Mods change by themselves (they end, fail, the Studio starts them).
    ctx.request_repaint_after(std::time::Duration::from_millis(500));
    sheet(ctx, "mods-window", "MODS", |ui| {
        let scale = theme::density(ui);
        // Inside a menu, its nav bar takes the place of the switch and the
        // tabs.
        let menus = (status.mods && page.tab == Tab::Menus).then(glue5_mods::menus);
        match &menus {
            Some(menus) => page.menus.prune(menus),
            None => send(page.menus.hidden()),
        }
        if let Some(menus) = menus.as_ref().filter(|_| page.menus.inside()) {
            menus_tab(ui, &mut page.menus, menus);
            return;
        }
        let running_with = status.started.as_ref().map(|s| s.mods);
        let note = match running_with {
            Some(with) if with != settings.mods => {
                Some(if settings.mods { "游戏下次启动时开启" } else { "下次启动时关闭；正在运行的模组会持续到那时" })
            }
            _ => None,
        };
        setting(ui, "模组", Control::Toggle, Some(note.unwrap_or("改变游戏的 Luau 脚本，随游戏运行")), |ui| {
            toggle(ui, &mut settings.mods);
        });
        if !settings.mods && !status.mods {
            ui.add_space(4.0 * scale);
            panel(ui, |ui| {
                card_title(ui, "什么是模组", None);
                small(
                    ui,
                    "A mod is a folder of Luau scripts that call the game's own functions: spawn a car, change \
                     the weather, add a menu of its own. Turn mods on, then write them in GLUE 5 Studio from a \
                     browser on your computer or tablet, or let Claude Code write them.",
                    theme::MUTED,
                );
            });
            return;
        }
        ui.add_space(6.0 * scale);
        // The menus are there while the game runs with mods.
        let mut tabs = vec![(Tab::Mods, "MODS"), (Tab::Studio, "STUDIO"), (Tab::Log, "LOG")];
        if status.mods {
            tabs.insert(0, (Tab::Menus, "MENUS"));
        } else if page.tab == Tab::Menus {
            page.tab = Tab::Mods;
        }
        segmented(ui, &mut page.tab, &tabs, 40.0 * scale);
        ui.add_space(10.0 * scale);
        match page.tab {
            Tab::Menus => menus_tab(ui, &mut page.menus, menus.as_deref().map_or(&[], |m| m.as_slice())),
            Tab::Mods => mods_tab(ui, page, status),
            Tab::Studio => studio_tab(ui, settings, status),
            Tab::Log => log_tab(ui),
        }
    })
}

/// The mods' menus: the list of them, or the level open in one.
fn menus_tab(ui: &mut egui::Ui, view: &mut MenusView, menus: &[glue5_mods::Menu]) {
    let mods = glue5_mods::mods();
    let names: Vec<(&str, &str)> = mods.iter().map(|m| (m.name.as_str(), m.info.title.as_str())).collect();
    send(view.view(ui, menus, &names));
}

fn mods_tab(ui: &mut egui::Ui, page: &mut ModsPage, status: &Status) {
    let held = glue5_mods::held_back();
    if !held.is_empty() {
        small(
            ui,
            &format!("{} did not start with the game: it was running when the game stopped last time.", held.join(", ")),
            WARN,
        );
    }
    let mods = glue5_mods::mods();
    if mods.is_empty() {
        panel(ui, |ui| {
            card_title(ui, "暂无模组", None);
            small(ui, "在本网络的浏览器中用 GLUE 5 Studio 编写。", theme::MUTED);
            if let Some(dir) = glue5_mods::dir().filter(|_| !crate::paths::MOBILE) {
                small(ui, &format!("Or put a folder with a main.luau in {}", dir.display()), theme::MUTED);
            }
            ui.add_space(4.0 * theme::density(ui));
            if inline_button(ui, "打开 Studio").clicked() {
                page.tab = Tab::Studio;
            }
        });
        return;
    }
    // Before the game: which start with it.
    if !status.mods {
        for m in &mods {
            let mut on = m.info.autostart;
            let note = if on { "随游戏启动" } else { "未随游戏启动" };
            setting(ui, &m.info.title, Control::Toggle, Some(note), |ui| {
                if toggle(ui, &mut on).changed()
                    && let Err(e) = glue5_mods::set_autostart(&m.name, on)
                {
                    log::warn!("mods: {e}");
                }
            });
        }
        return;
    }
    for m in &mods {
        let subtitle = match m.state {
            ModState::Running if m.waiting => "等待中".to_owned(),
            ModState::Running => format!("Running · {:.1} ms per frame", m.cpu_ms),
            ModState::Error => format!("Error · {}", m.error.as_deref().unwrap_or("")),
            ModState::Stopped if m.info.autostart => "已停止 · 随游戏启动".to_owned(),
            ModState::Stopped => "已停止".to_owned(),
        };
        let dot = match m.state {
            ModState::Running => "• ",
            _ => "",
        };
        setting(ui, &format!("{dot}{}", m.info.title), Control::Action, Some(&subtitle), |ui| {
            let running = m.state == ModState::Running;
            if action_button(ui, if running { "停止" } else { "启动" }).clicked() {
                if running {
                    glue5_mods::stop(&m.name);
                } else {
                    glue5_mods::start(&m.name);
                }
            }
        });
    }
    let running: Vec<_> = mods.iter().filter(|m| m.state == ModState::Running).collect();
    if !running.is_empty() {
        let total: f32 = running.iter().map(|m| m.cpu_ms).sum();
        setting(ui, &format!("{} running", running.len()), Control::Action, Some(&format!("{total:.1} ms per frame")), |ui| {
            if action_button(ui, "全部停止").clicked() {
                for m in &running {
                    glue5_mods::stop(&m.name);
                }
            }
        });
    }
    if glue5_mods::tainted() {
        small(ui, "游戏内有原生调用失败：请尽快重启游戏。", WARN);
    }
}

fn studio_tab(ui: &mut egui::Ui, settings: &mut Settings, status: &Status) {
    setting(ui, "GLUE 5 Studio", Control::Toggle, Some("在浏览器中编写和运行模组；用手机使用其菜单"), |ui| {
        toggle(ui, &mut settings.studio);
    });
    let others = if settings.studio_lan { "本网络的浏览器用代码配对" } else { "仅限本机浏览器" };
    setting(ui, "其他设备", Control::Toggle, Some(others), |ui| {
        toggle(ui, &mut settings.studio_lan);
    });
    match &status.studio {
        Some(info) => {
            let place = if info.lan { "on this network" } else { "on this device" };
            for (url, title, note) in [
                (&info.url, "Studio", format!("The editor, in a browser {place}")),
                (&info.control, "控制页", format!("The mods and their menus, for a phone or a tablet {place}")),
            ] {
                let address = format!("{title} · {}", url.trim_end_matches('/'));
                setting(ui, &address, Control::Actions, Some(&note), |ui| {
                    let [go, copy] = <[_; 2]>::try_from(action_buttons(ui, &["打开", "复制"])).unwrap();
                    if go.on_hover_text("在本机浏览器中，已配对").clicked() {
                        ui.ctx().open_url(egui::OpenUrl::new_tab(info.paired(url)));
                    }
                    if copy.clicked() {
                        ui.ctx().copy_text(url.clone());
                    }
                });
            }
            setting(ui, &info.code, Control::Actions, Some("配对码：浏览器或代理用它配对一次"), |ui| {
                let [new, copy] = <[_; 2]>::try_from(action_buttons(ui, &["新", "复制"])).unwrap();
                if new.on_hover_text("新代码：已配对的不受影响").clicked() {
                    settings.studio_code.clear();
                }
                if copy.clicked() {
                    ui.ctx().copy_text(info.code.clone());
                }
            });
            if info.lan {
                qr_row(ui, &info.paired(&info.control));
            }
            paired(ui);
        }
        None if settings.studio => small(ui, "Starting…", theme::MUTED),
        None => {}
    }
    muted(
        ui,
        "Whoever has the code can pair, and a paired Studio runs code in the game: make a new code if someone should no \
         longer have it, and remove what should no longer be paired. A pairing unused for 90 days goes by itself.",
    );
}

/// The browsers and agents paired with the Studio, each removable: its
/// token no longer works, its pages close.
fn paired(ui: &mut egui::Ui) {
    let pairings = glue5_mods::studio::pairing::list();
    if pairings.is_empty() {
        return;
    }
    let scale = theme::density(ui);
    ui.add_space(6.0 * scale);
    ui.label(theme::heading("PAIRED", 15.0 * scale).color(theme::MUTED));
    let now = std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH).map_or(0, |d| d.as_secs());
    for p in &pairings {
        let what = match p.scope {
            glue5_mods::studio::Scope::Studio => "Studio",
            glue5_mods::studio::Scope::Control => "控制页",
        };
        let note = format!("{what} · used {}", ago(now.saturating_sub(p.last_used)));
        setting(ui, &p.name, Control::Actions, Some(&note), |ui| {
            if action_button(ui, "移除").on_hover_text("需用代码重新配对").clicked() {
                glue5_mods::studio::pairing::forget(&p.id);
            }
        });
    }
    if pairings.len() > 1 {
        setting(ui, "已全部配对", Control::Actions, Some("所有浏览器和代理需重新配对"), |ui| {
            if action_button(ui, "全部移除").clicked() {
                glue5_mods::studio::pairing::forget_all();
            }
        });
    }
}

/// `seconds` ago, as people say it.
fn ago(seconds: u64) -> String {
    match seconds {
        0..60 => "just now".into(),
        60..3600 => format!("{} min ago", seconds / 60),
        3600..86400 => format!("{} h ago", seconds / 3600),
        _ => match seconds / 86400 {
            1 => "yesterday".into(),
            d => format!("{d} days ago"),
        },
    }
}

/// The control page's address (with the code) as a QR code, in a row like
/// the settings': a phone's camera opens it, paired for the control page.
fn qr_row(ui: &mut egui::Ui, url: &str) {
    let scale = theme::density(ui);
    egui::Frame::new()
        .fill(Color32::from_gray(28))
        .corner_radius((14.0 * scale) as u8)
        .inner_margin((12.0 * scale) as i8)
        .show(ui, |ui| {
            ui.set_width(ui.available_width());
            ui.horizontal(|ui| {
                qr(ui, url, 112.0 * scale);
                ui.add_space(8.0 * scale);
                ui.vertical(|ui| {
                    ui.label(RichText::new("扫码打开").size(14.0 * scale).color(Color32::WHITE));
                    ui.label(
                        RichText::new("用手机或平板摄像头扫描：控制页会打开并完成配对，包含模组菜单。")
                            .size(11.0 * scale)
                            .color(theme::MUTED),
                    );
                });
            });
        });
}

fn log_tab(ui: &mut egui::Ui) {
    let scale = theme::density(ui);
    // The last lines only (a copy of the whole log each frame held the lock
    // the mods' update takes).
    let lines = glue5_mods::recent_logs(80);
    if lines.is_empty() {
        muted(ui, "还没有内容。");
    }
    panel(ui, |ui| {
        for l in &lines {
            let color = if l.level == glue5_mods::LogLevel::Error { ERR } else { Color32::from_gray(213) };
            ui.label(RichText::new(format!("{} · {}", l.module, l.text)).monospace().size(12.0 * scale).color(color));
        }
    });
}

/// `text` as a QR code, dark on white.
fn qr(ui: &mut egui::Ui, text: &str, size: f32) {
    // Encoded once per address (the page is drawn every frame).
    thread_local! {
        static CODE: std::cell::RefCell<Option<(String, usize, Vec<qrcode::Color>)>> = const { std::cell::RefCell::new(None) };
    }
    let code = CODE.with_borrow_mut(|cached| {
        if cached.as_ref().is_none_or(|(t, ..)| t != text) {
            *cached = qrcode::QrCode::new(text.as_bytes()).ok().map(|c| (text.to_string(), c.width(), c.to_colors()));
        }
        cached.as_ref().map(|(_, modules, colors)| (*modules, colors.clone()))
    });
    let Some((modules, colors)) = code else { return };
    let (rect, _) = ui.allocate_exact_size(vec2(size, size), Sense::hover());
    let painter = ui.painter_at(rect);
    painter.rect_filled(rect, CornerRadius::same((10.0 * theme::density(ui)) as u8), Color32::WHITE);
    let quiet = 2.0;
    let cell = size / (modules as f32 + 2.0 * quiet);
    for (i, c) in colors.iter().enumerate() {
        if *c == qrcode::Color::Dark {
            let (x, y) = ((i % modules) as f32, (i / modules) as f32);
            let min = rect.min + vec2((x + quiet) * cell, (y + quiet) * cell);
            painter.rect_filled(egui::Rect::from_min_size(min, vec2(cell + 0.5, cell + 0.5)), 0, theme::INK);
        }
    }
}

fn panel(ui: &mut egui::Ui, add: impl FnOnce(&mut egui::Ui)) {
    let scale = theme::density(ui);
    egui::Frame::new()
        .fill(Color32::from_gray(28))
        .corner_radius((14.0 * scale) as u8)
        .inner_margin((14.0 * scale) as i8)
        .show(ui, |ui| {
            ui.set_width(ui.available_width());
            ui.spacing_mut().item_spacing.y = 6.0 * scale;
            add(ui);
        });
}

fn muted(ui: &mut egui::Ui, text: &str) {
    ui.label(RichText::new(text).size(12.0 * theme::density(ui)).color(theme::MUTED));
}
