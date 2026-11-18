//! Texts the game data lacks: its scripts are newer and name labels of
//! later DLC packs, which show as "<LABEL> missing". The ones GTA Online's
//! jobs and activities show on the way (hosting, coronas, the job lists),
//! the pause menu's sessions, and the graphics settings the menu's data
//! lacks (`overlay.rs`), are given here, in English.

use std::collections::HashMap;
use std::sync::Mutex;

use glue5_game::{Env, orig};

use crate::memory::Mem;

/// The pause menu's friends list with no friend in it (friends/game.rs):
/// its warning's text and title.
pub(crate) const NO_FRIENDS: &str = "GLUE5_NO_FRIENDS";
pub(crate) const NO_FRIENDS_TITLE: &str = "NOFRIENDS_TITLE";

/// Labels and their texts.
const TEXTS: &[(&str, &str)] = &[
    (NO_FRIENDS, "你还没有好友。在好友页添加：分享你的好友码，或向一起玩过的玩家发起请求。"),
    (NO_FRIENDS_TITLE, "暂无好友"),
    ("FACES_WARNH2", "提示"),
    ("QM_NO_1", "未找到可加入的该差事战局。要自己主持吗？"),
    ("COR_YOUR_CHAR", "你的角色"),
    ("UI_FLOW_OP_NEWSESH", "寻找新战局"),
    ("UI_FLOW_OP_NEWSESH_M", "寻找新战局"),
    ("UI_FLOW_OP_NEWSESH_D", "离开此战局，在新的战局中游玩 GTA Online。"),
    ("UI_FLOW_SO_PBLCSESH", "公开战局"),
    ("UI_FLOW_SO_PBLCSESH_M", "公开战局"),
    ("UI_FLOW_SO_PBLCSESH_D", "直接进入 GTA Online。"),
    ("UI_FLOW_SO_INVT", "仅限邀请战局"),
    ("UI_FLOW_SO_INVT_M", "仅限邀请战局"),
    ("UI_FLOW_SO_INVT_D", "其他玩家只有在你邀请时才能加入此战局。"),
    ("UI_FLOW_SO_CRWSESH", "帮会战局"),
    ("UI_FLOW_SO_CRWSESH_M", "帮会战局"),
    (
        "UI_FLOW_SO_CRWSESH_D",
        "与你的帮会及其他帮会一起进入 GTA Online。选择本战局中一同游玩的帮会数量。",
    ),
    ("UI_FLOW_SO_CLSDCRW", "仅限帮会战局"),
    ("UI_FLOW_SO_CLSDCRW_M", "仅限帮会战局"),
    ("UI_FLOW_SO_CLSDCRW_D", "仅与你的帮会成员进入 GTA Online。"),
    ("UI_FLOW_SO_CLSDFRND", "仅限好友战局"),
    ("UI_FLOW_SO_CLSDFRND_M", "仅限好友战局"),
    ("UI_FLOW_SO_CLSDFRND_D", "仅与你的好友进入 GTA Online。"),
    ("UI_FLOW_SO_SOLO", "单人战局"),
    ("UI_FLOW_SO_SOLO_M", "单人战局"),
    (
        "UI_FLOW_SO_SOLO_D",
        "独自进入 GTA Online。其他玩家无法加入此战局，可参与的活动也会受限。",
    ),
    ("PM_EXIT_GAME", "退出游戏"),
    ("PM_ARENA", "竞技场之战"),
    ("PM_RC_STUNT", "特技竞速"),
    ("RC_TARGASSAULT", "目标突袭竞速"),
    ("FMMC_RSTAR_KOT", "山丘之王"),
    ("PM_TNEWVS", "对抗模式"),
    ("FMMC_SR", "特技竞速"),
    ("FMMC_OWR", "开轮式竞速"),
    ("MATC_DPADRIGHT", "按 ~INPUT_CONTEXT~ 开始。"),
    ("PM_JOIN_AREN", "加入竞技场之战"),
    // Settings › Graphics and Advanced Graphics, as the PC game names them.
    ("GFX_POP_VARIETY", "人口多样性"),
    ("GFX_REFLECTION_MSAA", "反射 MSAA"),
    ("GFX_PARTICLEQ", "粒子质量"),
    ("GFX_GRASSQ", "草地质量"),
    ("GFX_MB_STRENGTH", "运动模糊强度"),
    ("GFX_DOF", "游戏内景深效果"),
    ("PM_PANE_ADVGFX", "高级画面"),
    ("GFX_LONG_SHADOWS", "长阴影"),
    ("GFX_ULTRA_SHADOWS", "高分辨率阴影"),
    ("GFX_HD_FLIGHT", "飞行时高细节流式加载"),
    ("GFX_MAX_LOD", "扩展距离缩放"),
    ("GFX_SHADOWS_DIST", "扩展阴影距离"),
];

/// The engine's label hash (`rage::atStringHash`): Jenkins' one-at-a-time,
/// case-insensitive.
pub(crate) fn label_hash(label: &str) -> u32 {
    let mut h: u32 = 0;
    for b in label.bytes() {
        h = h.wrapping_add(u32::from(b.to_ascii_lowercase()));
        h = h.wrapping_add(h << 10);
        h ^= h >> 6;
    }
    h = h.wrapping_add(h << 3);
    h ^= h >> 11;
    h.wrapping_add(h << 15)
}

/// The texts in the engine's heap, by hash (made when first asked for).
static PLACED: Mutex<Option<HashMap<u32, u64>>> = Mutex::new(None);

/// Hook for `CTextFile::GetInternal(unsigned int)`: the game data's text,
/// else one of these.
pub(crate) unsafe extern "C" fn text_get_internal(env: *mut Env, hash: u32) -> u64 {
    let text = unsafe { orig::text_get_internal(env, hash) };
    if text != 0 {
        return text;
    }
    // Labels' hashes once: misses come every frame (labels the data lacks).
    static HASHES: std::sync::OnceLock<Vec<u32>> = std::sync::OnceLock::new();
    let hashes = HASHES.get_or_init(|| TEXTS.iter().map(|(label, _)| label_hash(label)).collect());
    let Some((_, ours)) = TEXTS.iter().zip(hashes).find(|(_, h)| **h == hash).map(|(t, _)| t) else { return 0 };
    let mut placed = PLACED.lock().unwrap();
    let placed = placed.get_or_insert_default();
    if let Some(&at) = placed.get(&hash) {
        return at;
    }
    let at = unsafe { orig::builtin_malloc(env, ours.len() as u64 + 1) };
    if at != 0 {
        unsafe { Mem::of(env) }.write_str(at, ours, ours.len() + 1);
        placed.insert(hash, at);
    }
    at
}

pub(crate) fn hooks(hooks: &mut glue5_game::Hooks) {
    hooks.text_get_internal = Some(text_get_internal);
}

#[cfg(test)]
mod tests {
    #[test]
    fn label_hashes() {
        // Known joaat values.
        assert_eq!(super::label_hash("a"), 0xca2e9442);
        assert_eq!(super::label_hash("ADDER"), super::label_hash("adder"));
        assert_eq!(super::label_hash("adder"), 0xb779a091);
    }
}
