//! The application's own settings, in `glue5.json` in the user directory.
//! Graphics quality is the game's (its menu, Settings › Graphics, saved in
//! its `settings.xml`).

use std::path::{Path, PathBuf};

use serde::{Deserialize, Serialize};

use crate::paths::{MOBILE, TV};

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(default)]
pub struct Settings {
    /// The game directory, once found or chosen.
    pub game_dir: Option<PathBuf>,
    /// Where the game's files come from: a folder on this device, or a
    /// server (another device sharing its folder, or glue5-serve).
    pub source: Source,
    /// The server: `host` or `host:port`.
    pub server: String,
    /// This computer shares its game folder on the network (desktop).
    pub share: bool,
    /// The whole game comes from the server to this device (in its game
    /// folder), to play without it.
    pub download_all: bool,
    /// Smaller engine pools, and the game's graphics settings lowered once
    /// (the browser build's `?low=1`).
    pub low_memory: bool,
    /// Whether the low-memory preset is in the game's settings (written at
    /// the first start with `low_memory`; the game's menu changes it after).
    pub low_preset_written: bool,
    /// The game's resolution as a fraction of the window's, per side (the
    /// back buffer follows the window). Chosen for the screen at the first
    /// start ([`Settings::for_screen`]).
    pub render_scale: Option<f32>,
    pub vsync: bool,
    /// At most this many frames per second; 0: no limit of the engine's
    /// (the display's refresh rate with VSync).
    pub max_fps: u32,
    /// How the game's frames are scaled up to the window (what the GPU has
    /// of it; the bicubic filter otherwise).
    pub upscaler: Upscaling,
    /// How much FSR sharpens its image, 0 to 1.
    pub sharpness: f32,
    /// A frame made between each two of the game's (MetalFX Temporal, on
    /// macOS and iOS 26): twice the frames shown, half a frame more of
    /// latency.
    pub frame_generation: bool,
    /// The GPU driver the user added that the renderer opens at launch
    /// (Android, Adreno GPUs: drivers.rs); none: the phone's own.
    pub gpu_driver: Option<String>,
    /// The performance overlay over the game, and how the application's
    /// own shows.
    pub overlay: PerfOverlay,
    pub overlay_detail: OverlayDetail,
    pub overlay_corner: Corner,
    pub touch: TouchMode,
    pub touch_opacity: f32,
    pub touch_size: f32,
    /// How fast a finger's drag turns the camera.
    pub touch_camera: f32,
    /// The aim assist while playing by touch (the game's Targeting Mode,
    /// in place of its menu's).
    pub touch_aim: TouchAim,
    /// The Taptic Engine taps under pressed touch controls and plays the game's rumble (iPhone).
    pub haptics: bool,
    /// The game starts in the sandbox (the map, no story) rather than the
    /// story: the start page's last choice.
    pub sandbox: bool,
    /// GTA Online was the start page's last choice.
    pub online: bool,
    /// The player's name online (made up when GTA Online is first opened).
    pub online_name: String,
    /// Friends' notifications show (Social Club's "Hide Notifications").
    pub friend_notices: bool,
    /// The mods in the user directory's `mods` folder can run (glue5-mods).
    pub mods: bool,
    /// With mods, the game serves GLUE 5 Studio.
    pub studio: bool,
    /// Other devices on the network may open the Studio (with its code).
    pub studio_lan: bool,
    /// The Studio's pairing code: kept, so that a browser paired once stays
    /// paired when the game starts again (the Mods page makes a new one).
    pub studio_code: String,
    /// Settings saved before the render scale: the back buffer's height.
    #[serde(skip_serializing)]
    render_height: Option<u32>,
    /// Settings saved before the choice of upscalers: MetalFX's spatial one
    /// on or off.
    #[serde(skip_serializing)]
    metalfx: Option<bool>,
    /// Settings saved before the choice of overlays: Apple's HUD on or off.
    #[serde(skip_serializing)]
    metal_hud: Option<bool>,
}

/// Where the game's files come from.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum Source {
    Folder,
    Server,
}

/// The performance overlay over the game.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum PerfOverlay {
    Off,
    /// Apple's, drawn by Metal (shown from the launch it is on at).
    MetalHud,
    /// The application's (ui/perf.rs), on every platform.
    Glue5,
}

/// How much the application's overlay shows.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum OverlayDetail {
    /// One line: frame rate, frame time, GPU time.
    Minimal,
    /// The frame times as a graph, and where they go.
    Standard,
    /// The busiest threads, the renderer's counts, resolution and memory too.
    Full,
}

/// A corner of the safe area.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum Corner {
    TopLeft,
    TopRight,
    BottomLeft,
    BottomRight,
}

/// How the game's frames are scaled up to the window.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum Upscaling {
    Bicubic,
    /// MetalFX's spatial upscaler (Apple).
    #[serde(rename = "metalfx")]
    MetalFx,
    /// MetalFX's temporal upscaler (Apple).
    #[serde(rename = "metalfx-temporal")]
    MetalFxTemporal,
    /// AMD's FSR 2 upscaler (temporal, on GPUs with the features it needs).
    Fsr,
}

impl Upscaling {
    pub fn upscaler(self) -> glue5_gpu::Upscaler {
        match self {
            Upscaling::Bicubic => glue5_gpu::Upscaler::Bicubic,
            Upscaling::MetalFx => glue5_gpu::Upscaler::MetalFx,
            Upscaling::MetalFxTemporal => glue5_gpu::Upscaler::MetalFxTemporal,
            Upscaling::Fsr => glue5_gpu::Upscaler::Fsr,
        }
    }
}

/// When the on-screen controls show.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum TouchMode {
    /// On touch screens, while no controller (or keyboard) is in use.
    Auto,
    Always,
    Off,
}

/// The aim assist while playing by touch: one of the game's targeting modes.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum TouchAim {
    /// Aiming locks on to the nearest target and follows it (Assisted Aim -
    /// Full).
    LockOn,
    /// Aiming snaps to the target, then the finger aims (Assisted Aim -
    /// Partial).
    Snap,
    /// The camera slows down over targets (Free Aim - Assisted).
    SlowDown,
    /// Free Aim.
    Off,
}

impl TouchAim {
    pub fn targeting(self) -> glue5_runtime::input::Targeting {
        use glue5_runtime::input::Targeting;
        match self {
            TouchAim::LockOn => Targeting::Full,
            TouchAim::Snap => Targeting::Partial,
            TouchAim::SlowDown => Targeting::FreeAssisted,
            TouchAim::Off => Targeting::Free,
        }
    }
}

/// Phones and tablets with less RAM start with the low-memory profile (the
/// game takes about 4 GB at the normal one).
const LOW_MEMORY_RAM_MB: i32 = 7000;

/// 4 GB devices (an iPhone SE 3 reports 4 GB): the engine wants about 3 GB,
/// leaving little for render targets, so they draw fewer pixels and let a
/// temporal upscaler put the detail back. A phone that reports 4096 MB can
/// round either way by a page or two, hence the margin.
const TINY_RAM_MB: i32 = 5200;

/// The render scale's range, in steps of 5%.
pub const RENDER_SCALES: std::ops::RangeInclusive<f32> = 0.25..=1.0;

/// The height a phone renders at, before upscaling, for its RAM and its
/// screen. 720 lines is what GLUE 5 has always used on phones; a 4 GB phone
/// with a small screen takes 600 (0.8x of a 750-line screen) because it
/// cannot feed the GPU fast enough to hold a steady frame otherwise, and
/// the temporal upscaler recovers what the missing pixels cost.
///
/// A big low-memory screen (an older iPad) keeps 720: scaling it down to
/// 600 lines would leave the upscaler too much to invent.
pub fn auto_render_height(ram_mb: i32, screen_h: u32) -> u32 {
    if ram_mb >= LOW_MEMORY_RAM_MB {
        return 720;
    }
    if ram_mb < TINY_RAM_MB && screen_h <= 800 {
        return 600;
    }
    720
}

/// The upscaler to start with: MetalFX Temporal on a 4 GB Apple phone, where
/// the lower render scale needs the temporal one's sharper result to make up
/// for it; MetalFX's spatial one everywhere else (cheaper, and what the
/// published benchmarks ran at).
pub fn auto_upscaler(ram_mb: i32, ios: bool) -> Upscaling {
    if ios && ram_mb < TINY_RAM_MB {
        Upscaling::MetalFxTemporal
    } else {
        Upscaling::MetalFx
    }
}

/// The frame cap to start with. 30 on a 4 GB phone: an even half of the
/// display's 60 Hz looks smoother than a rate that drifts between 30 and 60,
/// and it keeps the phone cool enough not to throttle mid-session. A desktop
/// with little RAM keeps no cap: it has a fan and a page file.
pub fn auto_max_fps(ram_mb: i32, mobile: bool) -> u32 {
    if !mobile {
        return 0;
    }
    if ram_mb < TINY_RAM_MB {
        30
    } else {
        60
    }
}

impl Default for Settings {
    fn default() -> Self {
        // Only phones ask the system for their RAM: on a desktop the answer
        // would not change anything, and asking costs an SDL call the
        // settings do not otherwise need.
        let ram = if MOBILE { sdl3::cpuinfo::system_ram() } else { 0 };
        Settings {
            game_dir: None,
            source: if TV { Source::Server } else { Source::Folder },
            server: String::new(),
            share: false,
            download_all: false,
            low_memory: MOBILE && ram < LOW_MEMORY_RAM_MB,
            low_preset_written: false,
            render_scale: None,
            vsync: true,
            // Phones hold a steady 60 for longer than they hold 120 (heat);
            // a 4 GB one holds a steady 30 for longer still.
            max_fps: auto_max_fps(ram, MOBILE),
            upscaler: auto_upscaler(ram, cfg!(target_os = "ios")),
            sharpness: 0.5,
            frame_generation: false,
            gpu_driver: None,
            overlay: PerfOverlay::Off,
            overlay_detail: OverlayDetail::Standard,
            overlay_corner: Corner::TopLeft,
            touch: TouchMode::Auto,
            touch_opacity: 0.6,
            touch_size: 1.0,
            touch_camera: 1.0,
            touch_aim: TouchAim::LockOn,
            haptics: true,
            sandbox: false,
            online: false,
            online_name: String::new(),
            friend_notices: true,
            mods: false,
            studio: true,
            // A phone's own browser is no place to write mods.
            studio_lan: crate::paths::MOBILE,
            studio_code: String::new(),
            render_height: None,
            metalfx: None,
            metal_hud: None,
        }
    }
}

/// A name for a new online player: "Player" and four digits.
pub fn default_online_name() -> String {
    format!("Player{:04}", random() % 10000)
}

fn random() -> u64 {
    use std::hash::{BuildHasher, Hasher};
    std::collections::hash_map::RandomState::new().build_hasher().finish()
}

impl Settings {
    fn file(user_dir: &Path) -> PathBuf {
        user_dir.join("glue5.json")
    }

    pub fn load(user_dir: &Path) -> Settings {
        let file = Self::file(user_dir);
        let mut settings = match std::fs::read(&file) {
            Ok(bytes) => serde_json::from_slice(&bytes).unwrap_or_else(|e| {
                // Kept aside (the defaults are saved over it soon), not lost.
                let aside = file.with_extension("json.bad");
                match std::fs::rename(&file, &aside) {
                    Ok(()) => log::warn!("settings unreadable ({e}): kept as {}, the defaults instead", aside.display()),
                    Err(r) => log::warn!("settings unreadable ({e}), not kept aside ({r}): the defaults instead"),
                }
                Settings::default()
            }),
            Err(_) => Settings::default(),
        };
        if settings.metalfx.take() == Some(false) {
            settings.upscaler = Upscaling::Bicubic;
        }
        if settings.metal_hud.take() == Some(true) {
            settings.overlay = PerfOverlay::MetalHud;
        }
        if TV {
            settings.source = Source::Server;
        }
        settings
    }

    pub fn save(&self, user_dir: &Path) {
        let json = serde_json::to_vec_pretty(self).expect("settings serialise");
        // Through a copy: a write cut short (disk full, the app killed)
        // leaves the old file, not one that loads as the defaults.
        let file = Self::file(user_dir);
        let tmp = file.with_extension("json.tmp");
        if let Err(e) = std::fs::write(&tmp, json).and_then(|()| std::fs::rename(&tmp, &file)) {
            log::warn!("cannot save settings: {e}");
        }
    }

    /// Picks the render scale, the first time, for a window `height` pixels
    /// tall: the height an older settings file had, else about 1080 lines on
    /// desktop and 720 on phones and tablets (what the game ran at before:
    /// its GPU time grows with the pixels, and phone screens have many).
    pub fn for_screen(&mut self, height: u32) {
        let ram = if MOBILE { sdl3::cpuinfo::system_ram() } else { 0 };
        self.for_screen_with(height, ram)
    }

    /// [`Settings::for_screen`] for a device with `ram_mb` of memory. Split
    /// out so the tests can choose the memory instead of reading the machine
    /// they run on.
    pub fn for_screen_with(&mut self, height: u32, ram_mb: i32) {
        if self.render_scale.is_none() {
            let lines = self.render_height.take().unwrap_or(if MOBILE {
                auto_render_height(ram_mb, height)
            } else {
                1080
            });
            self.render_scale = Some(round_scale(lines as f32 / height.max(1) as f32));
        }
    }

    pub fn render_scale(&self) -> f32 {
        self.render_scale.unwrap_or(1.0)
    }
}

/// A render scale in the range, to its 5% step.
pub fn round_scale(scale: f32) -> f32 {
    ((scale * 20.0).round() / 20.0).clamp(*RENDER_SCALES.start(), *RENDER_SCALES.end())
}

/// The game's back buffer for a window of `pixels` at `scale`, as the
/// browser build sizes it: even sides (the engine halves them for its
/// post-processing), at least the 320x240 the engine takes, at most 4K
/// (1440p with low memory: render targets the size of the screen are most
/// of the GPU's memory).
pub fn render_size((w, h): (u32, u32), scale: f32, low_memory: bool) -> (u32, u32) {
    let cap = if low_memory { (2560.0, 1440.0) } else { (3840.0, 2160.0) };
    let (w, h) = (w as f32 * scale, h as f32 * scale);
    let k = (cap.0 / w).min(cap.1 / h).min(1.0);
    let side = |v: f32, min: u32| ((v * k / 2.0).round() as u32 * 2).max(min);
    (side(w, 320), side(h, 240))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn render_sizes() {
        assert_eq!(render_size((2560, 1440), 0.75, false), (1920, 1080));
        assert_eq!(render_size((2532, 1170), 0.6, false), (1520, 702));
        assert_eq!(render_size((300, 100), 0.5, false), (320, 240));
        assert_eq!(render_size((6016, 3384), 1.0, false), (3840, 2160));
        assert_eq!(render_size((6016, 3384), 1.0, true), (2560, 1440));
    }

    #[test]
    fn upscaler_and_overlay_from_older_settings() {
        let dir = std::env::temp_dir().join(format!("glue5-settings-test-{}", std::process::id()));
        std::fs::create_dir_all(&dir).unwrap();
        std::fs::write(dir.join("glue5.json"), r#"{"metalfx": false}"#).unwrap();
        assert_eq!(Settings::load(&dir).upscaler, Upscaling::Bicubic);
        std::fs::write(dir.join("glue5.json"), r#"{"metalfx": true}"#).unwrap();
        assert_eq!(Settings::load(&dir).upscaler, Upscaling::MetalFx);
        std::fs::write(dir.join("glue5.json"), r#"{"upscaler": "metalfx-temporal"}"#).unwrap();
        assert_eq!(Settings::load(&dir).upscaler, Upscaling::MetalFxTemporal);
        std::fs::write(dir.join("glue5.json"), r#"{"metal_hud": true}"#).unwrap();
        assert_eq!(Settings::load(&dir).overlay, PerfOverlay::MetalHud);
        std::fs::write(dir.join("glue5.json"), r#"{"metal_hud": false}"#).unwrap();
        assert_eq!(Settings::load(&dir).overlay, PerfOverlay::Off);
        std::fs::remove_dir_all(&dir).unwrap();
    }

    /// Settings that cannot be read are kept aside, not saved over.
    #[test]
    fn unreadable_settings_kept_aside() {
        let dir = std::env::temp_dir().join(format!("glue5-settings-bad-{}", std::process::id()));
        std::fs::create_dir_all(&dir).unwrap();
        std::fs::write(dir.join("glue5.json"), b"{\"online\": tru").unwrap();
        let s = Settings::load(&dir);
        assert_eq!(s.online, Settings::default().online);
        s.save(&dir);
        assert_eq!(std::fs::read(dir.join("glue5.json.bad")).unwrap(), b"{\"online\": tru");
        assert!(serde_json::from_slice::<Settings>(&std::fs::read(dir.join("glue5.json")).unwrap()).is_ok());
        std::fs::remove_dir_all(&dir).unwrap();
    }

    #[test]
    fn first_scale() {
        let mut s = Settings { render_height: Some(720), ..Settings::default() };
        s.for_screen(1440);
        assert_eq!(s.render_scale, Some(0.5));
        s.for_screen(720);
        assert_eq!(s.render_scale, Some(0.5), "chosen once");
        let mut s = Settings::default();
        s.for_screen(1170);
        assert_eq!(s.render_scale, Some(if MOBILE { 0.6 } else { 0.9 }));
    }

    /// An iPhone SE 3 (4 GB, 1334x750) draws 600 lines and lets the temporal
    /// upscaler fill the rest: 1068x600 upscaled to 1334x750, at a steady 30.
    #[test]
    fn four_gb_phone_profile() {
        let lines = auto_render_height(4096, 750);
        assert_eq!(lines, 600);
        assert_eq!(round_scale(lines as f32 / 750.0), 0.8);
        assert_eq!(render_size((1334, 750), 0.8, true), (1068, 600));
        assert_eq!(auto_upscaler(4096, true), Upscaling::MetalFxTemporal);
        assert_eq!(auto_max_fps(4096, true), 30);
        assert_eq!(auto_max_fps(4096, false), 0);
    }

    /// A phone with more RAM keeps the profile the published benchmarks ran
    /// at: 720 lines, MetalFX's spatial upscaler, 60 fps.
    #[test]
    fn six_gb_phone_keeps_published_profile() {
        assert_eq!(auto_render_height(6144, 1170), 720);
        assert_eq!(auto_upscaler(6144, true), Upscaling::MetalFx);
        assert_eq!(auto_max_fps(6144, true), 60);
        assert_eq!(auto_max_fps(6144, false), 0);
    }

    /// A 4 GB device with a big screen stays at 720 lines: scaling it down
    /// to 600 would leave the upscaler too much to invent.
    #[test]
    fn four_gb_big_screen_keeps_720() {
        assert_eq!(auto_render_height(4096, 2048), 720);
    }
}
