//! The editor's column: tabs, toolbar, the welcome page.

use leptos::prelude::*;
use serde_json::json;

use crate::actions::{close_tab, import, new_mod, save};
use crate::agents::Agents;
use crate::sidebar::state_class;
use crate::icons;
use crate::net::request;
use crate::state::*;

#[component]
pub(crate) fn Tabs(s: S) -> impl IntoView {
    view! {
        <div class="tabs">
            {move || s.tabs.get().into_iter().map(|t| {
                let p = t.path.clone();
                let p2 = t.path.clone();
                let p3 = t.path.clone();
                let active = move || s.active.get().as_deref() == Some(p.as_str());
                let page = mod_page(&t.path).map(str::to_string);
                let (file, module) = match (&page, t.path.split_once('/')) {
                    // Its name as it changes (on its page, elsewhere).
                    (Some(m), _) => {
                        let m = m.clone();
                        let title = move || s.mods.with(|ms| ms.iter().find(|x| x.name == m).map(|x| x.title.clone())).unwrap_or_else(|| m.clone());
                        ((move || format!("⚙ {}", title())).into_any(), "mod".to_string())
                    }
                    (None, Some((m, f))) => (f.to_string().into_any(), m.to_string()),
                    (None, None) => (t.path.clone().into_any(), String::new()),
                };
                let read_only = t.read_only && page.is_none();
                view! {
                    <div class=move || if active() { "tab on" } else { "tab" }
                        on:click=move |_| {
                            s.active.set(Some(p2.clone()));
                            if mod_page(&p2).is_none() { with_editor(|e| e.open(&p2, "", false)); }
                        }>
                        <span>{file}</span>
                        <em>{module}</em>
                        {if read_only { view! { <span class="ico lock" inner_html=icons::LOCK></span> }.into_any() }
                         else if t.dirty { view! { <span class="unsaved"></span> }.into_any() }
                         else { ().into_any() }}
                        <button class="x" aria-label="关闭" title="关闭" on:click=move |ev| { ev.stop_propagation(); close_tab(s, &p3); }>"×"</button>
                    </div>
                }
            }).collect_view()}
        </div>
    }
}

#[component]
pub(crate) fn Toolbar(s: S) -> impl IntoView {
    let current = move || s.active.get().filter(|p| p.contains('/') && mod_page(p).is_none());
    let page = move || s.active.get().and_then(|p| mod_page(&p).map(str::to_string));
    let module = move || current().map(|p| mod_of(&p).to_string()).or_else(page);
    let info = move || module().and_then(|m| s.mods.with(|mods| mods.iter().find(|x| x.name == m).cloned()));
    let toggle_run = move |_| {
        let Some(m) = info() else { return };
        // Unsaved changes go first.
        if let Some(p) = current() && s.tabs.with_untracked(|t| t.iter().any(|t| t.path == p && t.dirty)) {
            save(s, p);
        }
        let method = if m.state == "running" { "mods.stop" } else { "mods.start" };
        request(method, json!({ "name": m.name }), move |r| if let Err(e) = r { s.notice.set(Some(e)) });
    };
    view! {
        <div class="toolbar">
            <div class="crumbs">
                {move || current().map(|p| {
                    let (m, f) = p.split_once('/').map(|(a, b)| (a.to_string(), b.to_string())).unwrap_or_default();
                    view! { <span>{m}</span><span class="sep">"›"</span><span class="here">{f}</span> }.into_any()
                }).or_else(|| page().map(|m| view! { <span>{m}</span><span class="sep">"›"</span><span class="here">"Mod"</span> }.into_any()))}
            </div>
            <div class="grow"></div>
            {move || info().map(|m| {
                let running = m.state == "running";
                let name = m.name.clone();
                view! {
                    <div class="runstate">
                        <span class=state_class(&m)></span>
                        <b>{match m.state.as_str() { "running" if m.waiting => "等待中", "running" => "运行中", "error" => "错误", _ => "已停止" }}</b>
                    </div>
                    {running.then(|| view! {
                        <button class="btn" on:click=move |_| request("mods.restart", json!({"name": name}), |_| {})>
                            <span class="ico small" inner_html=icons::RESTART></span>"重启"
                        </button>
                    })}
                    <button class="btn primary" on:click=toggle_run>
                        <span class="ico small" inner_html=if running { icons::STOP } else { icons::PLAY }></span>
                        {if running { "停止" } else { "启动" }}
                    </button>
                }
            })}
            <Show when=move || current().is_some()>
                <button class="btn ghost" title="保存（Ctrl/Cmd+S）" on:click=move |_| if let Some(p) = current() { save(s, p) }>"保存"</button>
            </Show>
        </div>
    }
}

/// What shows with no tab open: what to start from, and the agents.
#[component]
pub(crate) fn Welcome(s: S) -> impl IntoView {
    let counts = move || {
        s.mods.with(|m| match (m.len(), m.iter().filter(|m| m.state == "running").count()) {
            (0, _) => "暂无模组".to_string(),
            (1, r) => format!("1 mod, {r} running"),
            (n, r) => format!("{n} mods, {r} running"),
        })
    };
    view! {
        <div class="welcome">
            <div class="welcome-in">
                <header class="welcome-head">
                    <img src="/logo.png" alt=""/>
                    <div>
                        <h1>"GLUE 5 STUDIO"</h1>
                        <p>"模组是在游戏内运行的 Luau 脚本。在此编写：每次保存都会重新加载。"</p>
                    </div>
                </header>
                <p class=move || if s.connected.get() { "welcome-status" } else { "welcome-status off" }>
                    <span class="dot"></span>
                    {move || s.hello.with(|h| h["device"].as_str().unwrap_or("游戏").to_string())}
                    <span class="sep">"·"</span>
                    {counts}
                </p>
                <div class="quick">
                    <button class="quick-card" on:click=move |_| new_mod(s)>
                        <span class="ico" inner_html=icons::PLUS></span>
                        <b>"新建模组"</b>
                        <small>"含 main.luau 的文件夹，可直接运行。"</small>
                    </button>
                    <button class="quick-card" on:click=move |_| import(s)>
                        <span class="ico" inner_html=icons::IMPORT></span>
                        <b>"导入 .zip"</b>
                        <small>"他人导出并分享的模组。"</small>
                    </button>
                    <a class="quick-card" href="/control" target="_blank" rel="noopener">
                        <span class="ico" inner_html=icons::SLIDERS></span>
                        <b>"控制页"</b>
                        <small>"模组及其菜单，供手机或第二屏使用。"</small>
                    </a>
                </div>
                <Agents/>
            </div>
        </div>
    }
}
