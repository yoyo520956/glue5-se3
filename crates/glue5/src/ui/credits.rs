//! The credits: what Glue 5 is, whose game it runs, and the open-source
//! projects it is built on, with their licenses and websites.
use super::pages::sheet;
use super::theme;
use super::widgets::{Control, action_button, setting};
use egui::{Color32, RichText};

/// A project: its name, what it does here, its license, its website.
const PROJECTS: &[(&str, &str, &str, &str)] = &[
    ("WABT", "wasm2c：游戏代码转 C", "Apache-2.0", "https://github.com/WebAssembly/wabt"),
    ("wgpu", "Metal 与 Vulkan 图形", "MIT / Apache-2.0", "https://wgpu.rs"),
    ("naga", "游戏着色器转译", "MIT / Apache-2.0", "https://github.com/gfx-rs/wgpu"),
    ("SDL", "窗口、输入、声音、手柄", "Zlib", "https://libsdl.org"),
    ("sdl3-rs", "SDL from Rust", "MIT", "https://github.com/vhspace/sdl3-rs"),
    ("egui", "本界面", "MIT / Apache-2.0", "https://github.com/emilk/egui"),
    ("Barlow", "字体，Jeremy Tribby 设计", "SIL OFL 1.1", "https://github.com/jpt/barlow"),
    (
        "game-icons.net",
        "触屏图标，Lorc、Delapouite、John Colburn 与 Guard13007 设计",
        "CC BY 3.0",
        "https://game-icons.net",
    ),
    ("Phosphor Icons", "触屏图标", "MIT", "https://phosphoricons.com"),
    ("Font Awesome Free", "触屏图标", "CC BY 4.0", "https://fontawesome.com"),
    ("Material Symbols", "触屏图标，Google 出品", "Apache-2.0", "https://fonts.google.com/icons"),
    ("Lucide", "触屏图标", "ISC", "https://lucide.dev"),
    ("Tabler Icons", "触屏图标", "MIT", "https://tabler.io/icons"),
    ("MingCute", "触屏图标", "Apache-2.0", "https://www.mingcute.com"),
    ("etcpak", "供无 BC 支持 GPU 使用的纹理", "BSD-3-Clause", "https://github.com/wolfpld/etcpak"),
    ("bcdec_rs", "纹理解码", "MIT", "https://github.com/ScanMountGoat/image_dds"),
    ("snow", "Noise 协议：好友加密信道", "Apache-2.0 / MIT", "https://github.com/mcginty/snow"),
    ("Zstandard", "游戏小文件已打包供串流", "BSD-3-Clause", "https://facebook.github.io/zstd"),
    #[cfg(target_os = "android")]
    (
        "libadrenotools",
        "为 Adreno GPU 添加的驱动，含 liblinkernsbypass，Billy Laws 出品",
        "BSD-2-Clause",
        "https://github.com/bylaws/libadrenotools",
    ),
    #[cfg(target_vendor = "apple")]
    ("objc2", "Apple 框架的 Rust 绑定", "MIT", "https://github.com/madsmtm/objc2"),
    #[cfg(windows)]
    ("mimalloc", "内存分配器", "MIT", "https://github.com/microsoft/mimalloc"),
    ("Rust", "语言、编译器与标准库", "MIT / Apache-2.0", "https://www.rust-lang.org"),
];

/// True when closed.
pub(super) fn credits_page(ctx: &egui::Context) -> bool {
    sheet(ctx, "credits-window", "CREDITS", |ui| {
        let scale = theme::density(ui);
        let text = |ui: &mut egui::Ui, text: &str| {
            ui.label(RichText::new(text).size(12.5 * scale).color(theme::MUTED));
        };
        egui::Frame::new()
            .fill(Color32::from_gray(28))
            .corner_radius((14.0 * scale) as u8)
            .inner_margin((16.0 * scale) as i8)
            .show(ui, |ui| {
                ui.set_width(ui.available_width());
                ui.spacing_mut().item_spacing.y = 6.0 * scale;
                ui.horizontal(|ui| {
                    ui.label(theme::heading("GLUE 5", 26.0 * scale));
                    ui.label(RichText::new(env!("CARGO_PKG_VERSION")).size(12.0 * scale).color(theme::MUTED));
                });
                text(
                    ui,
                    "GTA V's WebAssembly build as a native application for phones, tablets and computers: \
                     the game's code recompiled for this device, with its graphics, sound, controls and \
                     online play. Free software, under the GNU General Public License, version 3 or later.",
                );
            });
        text(
            ui,
            "Grand Theft Auto V is © Rockstar Games and Take-Two Interactive. Glue 5 is an independent \
             project, neither affiliated with nor endorsed by them, and includes none of the game's files: \
             it runs your own copy.",
        );
        ui.add_space(4.0 * scale);
        ui.label(theme::heading("开源项目", 17.0 * scale).color(theme::MUTED));
        for &(name, what, license, url) in PROJECTS {
            setting(ui, name, Control::Action, Some(&format!("{what} · {license}")), |ui| {
                if action_button(ui, "网站").on_hover_text(url).clicked() {
                    ui.ctx().open_url(egui::OpenUrl::new_tab(url));
                }
            });
        }
        text(
            ui,
            "With the libraries these use, each under its own license (MIT, Apache-2.0, BSD, Zlib, \
             Unicode, …). GTA Online's players and friends find each other through the BitTorrent DHT, \
             kept by the people who run it.",
        );
    })
}
