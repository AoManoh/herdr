use crate::config::UiLanguage;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum UiText {
    ReloadedConfig,
    UsingConfigToml,
    WithWarnings,
}

pub(crate) fn text(language: UiLanguage, message: UiText) -> &'static str {
    match (language, message) {
        (UiLanguage::En, UiText::ReloadedConfig) => "reloaded config",
        (UiLanguage::En, UiText::UsingConfigToml) => "using config.toml",
        (UiLanguage::En, UiText::WithWarnings) => "with warnings",
        (UiLanguage::ZhCn, UiText::ReloadedConfig) => "已重新加载配置",
        (UiLanguage::ZhCn, UiText::UsingConfigToml) => "正在使用 config.toml",
        (UiLanguage::ZhCn, UiText::WithWarnings) => "存在警告",
    }
}

pub(crate) fn global_menu_label(
    language: UiLanguage,
    action: crate::app::state::GlobalMenuAction,
    update_available: bool,
) -> &'static str {
    use crate::app::state::GlobalMenuAction;

    match (language, action, update_available) {
        (UiLanguage::En, GlobalMenuAction::Settings, _) => "settings",
        (UiLanguage::En, GlobalMenuAction::Keybinds, _) => "keybinds",
        (UiLanguage::En, GlobalMenuAction::ReloadConfig, _) => "reload config",
        (UiLanguage::En, GlobalMenuAction::WhatsNew, true) => "update ready",
        (UiLanguage::En, GlobalMenuAction::WhatsNew, false) => "what's new",
        (UiLanguage::En, GlobalMenuAction::Detach, _) => "detach",
        (UiLanguage::ZhCn, GlobalMenuAction::Settings, _) => "设置",
        (UiLanguage::ZhCn, GlobalMenuAction::Keybinds, _) => "快捷键",
        (UiLanguage::ZhCn, GlobalMenuAction::ReloadConfig, _) => "重新加载配置",
        (UiLanguage::ZhCn, GlobalMenuAction::WhatsNew, true) => "更新已就绪",
        (UiLanguage::ZhCn, GlobalMenuAction::WhatsNew, false) => "版本动态",
        (UiLanguage::ZhCn, GlobalMenuAction::Detach, _) => "分离会话",
    }
}

pub(crate) fn settings_section_label(
    language: UiLanguage,
    section: crate::app::state::SettingsSection,
) -> &'static str {
    use crate::app::state::SettingsSection;

    match (language, section) {
        (UiLanguage::En, SettingsSection::Language) => "language",
        (UiLanguage::En, SettingsSection::Theme) => "theme",
        (UiLanguage::En, SettingsSection::Sound) => "sound",
        (UiLanguage::En, SettingsSection::Toast) => "toasts",
        (UiLanguage::En, SettingsSection::PaneLabels) => "labels",
        (UiLanguage::En, SettingsSection::Integrations) => "agents",
        (UiLanguage::ZhCn, SettingsSection::Language) => "语言",
        (UiLanguage::ZhCn, SettingsSection::Theme) => "主题",
        (UiLanguage::ZhCn, SettingsSection::Sound) => "声音",
        (UiLanguage::ZhCn, SettingsSection::Toast) => "通知",
        (UiLanguage::ZhCn, SettingsSection::PaneLabels) => "窗格标签",
        (UiLanguage::ZhCn, SettingsSection::Integrations) => "集成",
    }
}

pub(crate) fn context_menu_label(
    language: UiLanguage,
    action: crate::app::state::ContextMenuAction,
) -> &'static str {
    use crate::app::state::ContextMenuAction;

    match (language, action) {
        (UiLanguage::En, ContextMenuAction::Rename) => "Rename",
        (UiLanguage::En, ContextMenuAction::Close) => "Close",
        (UiLanguage::En, ContextMenuAction::CloseGroup) => "Close group",
        (UiLanguage::En, ContextMenuAction::NewWorktree) => "New worktree",
        (UiLanguage::En, ContextMenuAction::OpenWorktree) => "Open worktree...",
        (UiLanguage::En, ContextMenuAction::DeleteWorktreeCheckout) => {
            "Delete worktree checkout..."
        }
        (UiLanguage::En, ContextMenuAction::Collapse) => "Collapse",
        (UiLanguage::En, ContextMenuAction::Expand) => "Expand",
        (UiLanguage::En, ContextMenuAction::NewTab) => "New tab",
        (UiLanguage::En, ContextMenuAction::RenamePane) => "Rename pane",
        (UiLanguage::En, ContextMenuAction::ClearPaneName) => "Clear pane name",
        (UiLanguage::En, ContextMenuAction::SwapWithFocusedPane) => "Swap with focused pane",
        (UiLanguage::En, ContextMenuAction::SplitRight) => "Split right",
        (UiLanguage::En, ContextMenuAction::SplitDown) => "Split down",
        (UiLanguage::En, ContextMenuAction::Zoom) => "Zoom",
        (UiLanguage::En, ContextMenuAction::ClosePane) => "Close pane",
        (UiLanguage::ZhCn, ContextMenuAction::Rename) => "重命名",
        (UiLanguage::ZhCn, ContextMenuAction::Close) => "关闭",
        (UiLanguage::ZhCn, ContextMenuAction::CloseGroup) => "关闭工作区组",
        (UiLanguage::ZhCn, ContextMenuAction::NewWorktree) => "新建工作树",
        (UiLanguage::ZhCn, ContextMenuAction::OpenWorktree) => "打开工作树...",
        (UiLanguage::ZhCn, ContextMenuAction::DeleteWorktreeCheckout) => "删除工作树检出...",
        (UiLanguage::ZhCn, ContextMenuAction::Collapse) => "折叠",
        (UiLanguage::ZhCn, ContextMenuAction::Expand) => "展开",
        (UiLanguage::ZhCn, ContextMenuAction::NewTab) => "新建标签页",
        (UiLanguage::ZhCn, ContextMenuAction::RenamePane) => "重命名窗格",
        (UiLanguage::ZhCn, ContextMenuAction::ClearPaneName) => "清除窗格名称",
        (UiLanguage::ZhCn, ContextMenuAction::SwapWithFocusedPane) => "与当前窗格交换",
        (UiLanguage::ZhCn, ContextMenuAction::SplitRight) => "向右拆分",
        (UiLanguage::ZhCn, ContextMenuAction::SplitDown) => "向下拆分",
        (UiLanguage::ZhCn, ContextMenuAction::Zoom) => "缩放",
        (UiLanguage::ZhCn, ContextMenuAction::ClosePane) => "关闭窗格",
    }
}

pub(crate) fn toast_event_text(
    language: UiLanguage,
    kind: crate::app::state::ToastKind,
) -> &'static str {
    use crate::app::state::ToastKind;

    match (language, kind) {
        (UiLanguage::En, ToastKind::NeedsAttention) => "needs attention",
        (UiLanguage::En, ToastKind::Finished) => "finished",
        (UiLanguage::En, ToastKind::UpdateInstalled) => "updated",
        (UiLanguage::ZhCn, ToastKind::NeedsAttention) => "正在等待输入",
        (UiLanguage::ZhCn, ToastKind::Finished) => "已完成",
        (UiLanguage::ZhCn, ToastKind::UpdateInstalled) => "已更新",
    }
}

pub(crate) fn agent_state_label(
    language: UiLanguage,
    state: crate::detect::AgentState,
    seen: bool,
) -> &'static str {
    use crate::detect::AgentState;

    match (language, state, seen) {
        (UiLanguage::En, AgentState::Blocked, _) => "blocked",
        (UiLanguage::En, AgentState::Working, _) => "working",
        (UiLanguage::En, AgentState::Idle, false) => "done",
        (UiLanguage::En, AgentState::Idle, true) => "idle",
        (UiLanguage::En, AgentState::Unknown, _) => "idle",
        (UiLanguage::ZhCn, AgentState::Blocked, _) => "等待输入",
        (UiLanguage::ZhCn, AgentState::Working, _) => "工作中",
        (UiLanguage::ZhCn, AgentState::Idle, false) => "已完成",
        (UiLanguage::ZhCn, AgentState::Idle, true) => "空闲",
        (UiLanguage::ZhCn, AgentState::Unknown, _) => "未知",
    }
}

pub(crate) struct ReleaseText {
    pub update_ready: &'static str,
    pub whats_new: &'static str,
    pub close: &'static str,
    pub scroll: &'static str,
    pub announcement_preview: &'static str,
    pub announcement: &'static str,
}

const RELEASE_EN: ReleaseText = ReleaseText {
    update_ready: "update ready",
    whats_new: "what's new in this release",
    close: "close",
    scroll: " scroll ",
    announcement_preview: "product announcement preview",
    announcement: "product announcement",
};

const RELEASE_ZH_CN: ReleaseText = ReleaseText {
    update_ready: "更新已就绪",
    whats_new: "本版本的新内容",
    close: "关闭",
    scroll: " 滚动 ",
    announcement_preview: "产品公告预览",
    announcement: "产品公告",
};

pub(crate) fn release_text(language: UiLanguage) -> &'static ReleaseText {
    match language {
        UiLanguage::En => &RELEASE_EN,
        UiLanguage::ZhCn => &RELEASE_ZH_CN,
    }
}

pub(crate) struct MobileText {
    pub switch: &'static str,
    pub no_workspace: &'static str,
    pub close: &'static str,
    pub agents: &'static str,
    pub filtered: &'static str,
    pub no_matching_agents: &'static str,
    pub spaces: &'static str,
    pub shell: &'static str,
    pub tabs: &'static str,
    pub menu: &'static str,
    pub no_agents: &'static str,
    pub all_idle: &'static str,
    pub update_ready: &'static str,
    pub new_workspace: &'static str,
    pub new_tab: &'static str,
}

const MOBILE_EN: MobileText = MobileText {
    switch: "switch",
    no_workspace: " no workspace",
    close: "close",
    agents: "agents",
    filtered: "filtered",
    no_matching_agents: "  no matching agents",
    spaces: "spaces",
    shell: "shell",
    tabs: "tabs",
    menu: "menu",
    no_agents: "no agents",
    all_idle: "all idle",
    update_ready: "update ready",
    new_workspace: "+ new workspace",
    new_tab: "+ new tab",
};

const MOBILE_ZH_CN: MobileText = MobileText {
    switch: "切换",
    no_workspace: " 没有工作区",
    close: "关闭",
    agents: "智能体",
    filtered: "已筛选",
    no_matching_agents: "  没有匹配的智能体",
    spaces: "工作区",
    shell: "终端",
    tabs: "标签页",
    menu: "菜单",
    no_agents: "没有智能体",
    all_idle: "全部空闲",
    update_ready: "更新已就绪",
    new_workspace: "+ 新建工作区",
    new_tab: "+ 新建标签页",
};

pub(crate) fn mobile_text(language: UiLanguage) -> &'static MobileText {
    match language {
        UiLanguage::En => &MOBILE_EN,
        UiLanguage::ZhCn => &MOBILE_ZH_CN,
    }
}

pub(crate) fn mobile_tab_label(
    language: UiLanguage,
    label: &str,
    current: Option<(usize, usize)>,
) -> String {
    let tab = match language {
        UiLanguage::En => "tab",
        UiLanguage::ZhCn => "标签页",
    };
    match current {
        Some((current, total)) => format!("{tab} {label} · {current}/{total}"),
        None => format!("{tab} {label}"),
    }
}

pub(crate) struct EmptyText {
    pub no_workspaces: &'static str,
    pub workspace_description: &'static str,
    pub root_pane_description: &'static str,
    pub press: &'static str,
    pub create_one: &'static str,
    pub unset: &'static str,
}

const EMPTY_EN: EmptyText = EmptyText {
    no_workspaces: "  No workspaces yet",
    workspace_description: "  A workspace is one project context.",
    root_pane_description: "  Its root pane (top-left) sets the default repo or folder name.",
    press: "  Press ",
    create_one: " to create one",
    unset: "unset",
};

const EMPTY_ZH_CN: EmptyText = EmptyText {
    no_workspaces: "  还没有工作区",
    workspace_description: "  一个工作区对应一个项目上下文。",
    root_pane_description: "  左上角的根窗格决定默认仓库或文件夹名称。",
    press: "  按 ",
    create_one: " 创建工作区",
    unset: "未设置",
};

pub(crate) fn empty_text(language: UiLanguage) -> &'static EmptyText {
    match language {
        UiLanguage::En => &EMPTY_EN,
        UiLanguage::ZhCn => &EMPTY_ZH_CN,
    }
}

pub(crate) struct NavigatorText {
    pub search_panes: &'static str,
    pub tab: &'static str,
    pub pane: &'static str,
    pub shell: &'static str,
    pub enter: &'static str,
    pub switch: &'static str,
    pub move_hint: &'static str,
    pub clear: &'static str,
    pub back: &'static str,
    pub search: &'static str,
    pub states: &'static str,
    pub close: &'static str,
}

const NAVIGATOR_EN: NavigatorText = NavigatorText {
    search_panes: "search panes",
    tab: "tab",
    pane: "pane",
    shell: "shell",
    enter: " enter",
    switch: " switch  ",
    move_hint: " move  ",
    clear: " clear  ",
    back: " back",
    search: " search  ",
    states: " states  ",
    close: " close",
};

const NAVIGATOR_ZH_CN: NavigatorText = NavigatorText {
    search_panes: "搜索窗格",
    tab: "标签页",
    pane: "窗格",
    shell: "终端",
    enter: " 回车",
    switch: " 切换  ",
    move_hint: " 移动  ",
    clear: " 清除  ",
    back: " 返回",
    search: " 搜索  ",
    states: " 状态筛选  ",
    close: " 关闭",
};

pub(crate) fn navigator_text(language: UiLanguage) -> &'static NavigatorText {
    match language {
        UiLanguage::En => &NAVIGATOR_EN,
        UiLanguage::ZhCn => &NAVIGATOR_ZH_CN,
    }
}

pub(crate) fn pane_total(language: UiLanguage, count: usize) -> String {
    match language {
        UiLanguage::En => format!("{count} panes"),
        UiLanguage::ZhCn => format!("{count} 个窗格"),
    }
}

pub(crate) fn aligned_pane_total(language: UiLanguage, count: usize, width: usize) -> String {
    match language {
        UiLanguage::En => format!("{count:>width$} panes"),
        UiLanguage::ZhCn => format!("{count:>width$} 个窗格"),
    }
}

pub(crate) struct DialogText {
    pub save: &'static str,
    pub clear: &'static str,
    pub cancel: &'static str,
    pub new_workspace: &'static str,
    pub rename_workspace: &'static str,
    pub new_tab: &'static str,
    pub rename_tab: &'static str,
    pub rename_pane: &'static str,
    pub create_and_open: &'static str,
    pub delete_anyway: &'static str,
    pub remove: &'static str,
    pub open: &'static str,
    pub new_worktree: &'static str,
    pub branch: &'static str,
    pub checkout: &'static str,
    pub creating: &'static str,
    pub delete_worktree: &'static str,
    pub removes_checkout: &'static str,
    pub branch_kept: &'static str,
    pub dirty_warning: &'static str,
    pub removing: &'static str,
    pub open_worktree: &'static str,
    pub no_matching_worktrees: &'static str,
    pub filter_worktrees: &'static str,
    pub close_group: &'static str,
    pub close_workspace: &'static str,
    pub confirm: &'static str,
}

const DIALOG_EN: DialogText = DialogText {
    save: "save",
    clear: "clear",
    cancel: "cancel",
    new_workspace: "new workspace",
    rename_workspace: "rename workspace",
    new_tab: "new tab",
    rename_tab: "rename tab",
    rename_pane: "rename pane",
    create_and_open: "create and open",
    delete_anyway: "delete anyway",
    remove: "remove",
    open: "open",
    new_worktree: "new worktree",
    branch: " branch",
    checkout: " checkout",
    creating: " creating…",
    delete_worktree: " delete worktree checkout?",
    removes_checkout: " This removes the checkout folder:",
    branch_kept: " The branch is not deleted. The Herdr workspace will close.",
    dirty_warning: " Dirty or untracked files will be permanently deleted.",
    removing: " removing…",
    open_worktree: "open worktree",
    no_matching_worktrees: " no matching worktrees",
    filter_worktrees: "filter worktrees",
    close_group: "Close worktree group?",
    close_workspace: "Close workspace?",
    confirm: "confirm",
};

const DIALOG_ZH_CN: DialogText = DialogText {
    save: "保存",
    clear: "清除",
    cancel: "取消",
    new_workspace: "新建工作区",
    rename_workspace: "重命名工作区",
    new_tab: "新建标签页",
    rename_tab: "重命名标签页",
    rename_pane: "重命名窗格",
    create_and_open: "创建并打开",
    delete_anyway: "仍然删除",
    remove: "移除",
    open: "打开",
    new_worktree: "新建工作树",
    branch: " 分支",
    checkout: " 检出目录",
    creating: " 正在创建…",
    delete_worktree: " 删除工作树检出？",
    removes_checkout: " 将删除以下检出文件夹：",
    branch_kept: " 分支不会被删除，对应的 Herdr 工作区将关闭。",
    dirty_warning: " 未提交或未跟踪的文件将被永久删除。",
    removing: " 正在移除…",
    open_worktree: "打开工作树",
    no_matching_worktrees: " 没有匹配的工作树",
    filter_worktrees: "筛选工作树",
    close_group: "关闭工作树组？",
    close_workspace: "关闭工作区？",
    confirm: "确认",
};

pub(crate) fn dialog_text(language: UiLanguage) -> &'static DialogText {
    match language {
        UiLanguage::En => &DIALOG_EN,
        UiLanguage::ZhCn => &DIALOG_ZH_CN,
    }
}

pub(crate) fn pane_count(language: UiLanguage, count: usize) -> String {
    match language {
        UiLanguage::En if count == 1 => "1 pane".to_string(),
        UiLanguage::En => format!("{count} panes"),
        UiLanguage::ZhCn => format!("{count} 个窗格"),
    }
}

pub(crate) fn workspace_count_prefix(language: UiLanguage, count: usize) -> String {
    match language {
        UiLanguage::En if count == 1 => "1 workspace, ".to_string(),
        UiLanguage::En => format!("{count} workspaces, "),
        UiLanguage::ZhCn => format!("{count} 个工作区，"),
    }
}

pub(crate) fn integrations_current(language: UiLanguage) -> &'static str {
    match language {
        UiLanguage::En => "all detected integrations are current",
        UiLanguage::ZhCn => "检测到的集成都已是最新版本",
    }
}

pub(crate) fn integration_installed(language: UiLanguage, label: &str) -> String {
    match language {
        UiLanguage::En => format!("installed {label}"),
        UiLanguage::ZhCn => format!("已安装 {label}"),
    }
}

pub(crate) fn checkout_count(
    language: UiLanguage,
    filtered: Option<usize>,
    total: usize,
) -> String {
    match (language, filtered) {
        (UiLanguage::En, None) => format!("{total} checkouts"),
        (UiLanguage::En, Some(filtered)) => format!("{filtered}/{total} checkouts"),
        (UiLanguage::ZhCn, None) => format!("{total} 个检出目录"),
        (UiLanguage::ZhCn, Some(filtered)) => format!("{filtered}/{total} 个检出目录"),
    }
}

pub(crate) struct OnboardingText {
    pub tagline: &'static str,
    pub mouse_intro: &'static str,
    pub enters_prefix: &'static str,
    pub shows_help: &'static str,
    pub next_step: &'static str,
    pub continue_label: &'static str,
}

const ONBOARDING_EN: OnboardingText = OnboardingText {
    tagline: "  terminal workspace manager for coding agents",
    mouse_intro: "  this is a mouse-first terminal.\n  click the sidebar to switch workspaces, drag pane\n  borders to resize, right-click for context menus.",
    enters_prefix: " enters prefix mode · ",
    shows_help: " shows keybinds and settings",
    next_step: "  next: install optional agent integrations for more reliable state",
    continue_label: "continue",
};

const ONBOARDING_ZH_CN: OnboardingText = OnboardingText {
    tagline: "  面向编程智能体的终端工作区管理器",
    mouse_intro: "  这是一个优先支持鼠标操作的终端。\n  点击侧边栏切换工作区，拖动窗格边框调整大小，\n  右键打开上下文菜单。",
    enters_prefix: " 进入前缀模式 · ",
    shows_help: " 显示快捷键和设置",
    next_step: "  下一步：安装可选的智能体集成，以获得更可靠的状态识别",
    continue_label: "继续",
};

pub(crate) fn onboarding_text(language: UiLanguage) -> &'static OnboardingText {
    match language {
        UiLanguage::En => &ONBOARDING_EN,
        UiLanguage::ZhCn => &ONBOARDING_ZH_CN,
    }
}

pub(crate) struct HelpText {
    pub unset: &'static str,
    pub global: &'static str,
    pub prefix_mode: &'static str,
    pub keybinds: &'static str,
    pub settings: &'static str,
    pub detach: &'static str,
    pub reload_config: &'static str,
    pub open_notification_target: &'static str,
    pub navigation: &'static str,
    pub back: &'static str,
    pub workspace_list: &'static str,
    pub move_focus: &'static str,
    pub cycle_pane: &'static str,
    pub open_workspace: &'static str,
    pub switch_workspace: &'static str,
    pub workspaces_tabs: &'static str,
    pub workspace_navigation: &'static str,
    pub session_navigator: &'static str,
    pub new_workspace: &'static str,
    pub new_worktree: &'static str,
    pub open_worktree: &'static str,
    pub delete_worktree: &'static str,
    pub rename_workspace: &'static str,
    pub close_workspace: &'static str,
    pub previous_workspace: &'static str,
    pub next_workspace: &'static str,
    pub switch_workspace_indexed: &'static str,
    pub previous_agent: &'static str,
    pub next_agent: &'static str,
    pub focus_agent: &'static str,
    pub new_tab: &'static str,
    pub rename_tab: &'static str,
    pub previous_tab: &'static str,
    pub next_tab: &'static str,
    pub switch_tab: &'static str,
    pub close_tab: &'static str,
    pub panes: &'static str,
    pub split_vertical: &'static str,
    pub split_horizontal: &'static str,
    pub close_pane: &'static str,
    pub rename_pane: &'static str,
    pub edit_scrollback: &'static str,
    pub copy_mode: &'static str,
    pub zoom_pane: &'static str,
    pub resize_mode: &'static str,
    pub toggle_sidebar: &'static str,
    pub focus_left: &'static str,
    pub focus_down: &'static str,
    pub focus_up: &'static str,
    pub focus_right: &'static str,
    pub cycle_next: &'static str,
    pub cycle_previous: &'static str,
    pub last_pane: &'static str,
    pub custom: &'static str,
    pub custom_command: &'static str,
    pub no_matches: &'static str,
    pub close: &'static str,
    pub filter_hint: &'static str,
    pub filter: &'static str,
    pub clear: &'static str,
    pub scroll: &'static str,
    pub search: &'static str,
}

const HELP_EN: HelpText = HelpText {
    unset: "unset",
    global: "global",
    prefix_mode: "prefix mode",
    keybinds: "keybinds",
    settings: "settings",
    detach: "detach",
    reload_config: "reload config",
    open_notification_target: "open notification target",
    navigation: "navigation",
    back: "back",
    workspace_list: "workspace list",
    move_focus: "move focus",
    cycle_pane: "cycle pane",
    open_workspace: "open workspace",
    switch_workspace: "switch workspace",
    workspaces_tabs: "workspaces / tabs",
    workspace_navigation: "workspace navigation",
    session_navigator: "session navigator",
    new_workspace: "new workspace",
    new_worktree: "new worktree",
    open_worktree: "open worktree",
    delete_worktree: "delete worktree checkout",
    rename_workspace: "rename workspace",
    close_workspace: "close workspace",
    previous_workspace: "previous workspace",
    next_workspace: "next workspace",
    switch_workspace_indexed: "switch workspace 1-9",
    previous_agent: "previous agent",
    next_agent: "next agent",
    focus_agent: "focus agent 1-9",
    new_tab: "new tab",
    rename_tab: "rename tab",
    previous_tab: "previous tab",
    next_tab: "next tab",
    switch_tab: "switch tab 1-9",
    close_tab: "close tab",
    panes: "panes",
    split_vertical: "split vertical",
    split_horizontal: "split horizontal",
    close_pane: "close pane",
    rename_pane: "rename pane",
    edit_scrollback: "edit scrollback",
    copy_mode: "copy mode",
    zoom_pane: "zoom pane",
    resize_mode: "resize mode",
    toggle_sidebar: "toggle sidebar",
    focus_left: "focus pane left",
    focus_down: "focus pane down",
    focus_up: "focus pane up",
    focus_right: "focus pane right",
    cycle_next: "cycle pane next",
    cycle_previous: "cycle pane previous",
    last_pane: "last pane",
    custom: "custom",
    custom_command: "custom command",
    no_matches: " no matching keybinds",
    close: "close",
    filter_hint: " press / to filter by command or shortcut",
    filter: " filter ",
    clear: "clear ",
    scroll: "scroll ",
    search: " search ",
};

const HELP_ZH_CN: HelpText = HelpText {
    unset: "未设置",
    global: "全局",
    prefix_mode: "前缀模式",
    keybinds: "快捷键",
    settings: "设置",
    detach: "分离会话",
    reload_config: "重新加载配置",
    open_notification_target: "打开通知目标",
    navigation: "导航",
    back: "返回",
    workspace_list: "工作区列表",
    move_focus: "移动焦点",
    cycle_pane: "循环切换窗格",
    open_workspace: "打开工作区",
    switch_workspace: "切换工作区",
    workspaces_tabs: "工作区 / 标签页",
    workspace_navigation: "工作区导航",
    session_navigator: "会话导航",
    new_workspace: "新建工作区",
    new_worktree: "新建工作树",
    open_worktree: "打开工作树",
    delete_worktree: "删除工作树检出",
    rename_workspace: "重命名工作区",
    close_workspace: "关闭工作区",
    previous_workspace: "上一个工作区",
    next_workspace: "下一个工作区",
    switch_workspace_indexed: "切换工作区 1-9",
    previous_agent: "上一个智能体",
    next_agent: "下一个智能体",
    focus_agent: "聚焦智能体 1-9",
    new_tab: "新建标签页",
    rename_tab: "重命名标签页",
    previous_tab: "上一个标签页",
    next_tab: "下一个标签页",
    switch_tab: "切换标签页 1-9",
    close_tab: "关闭标签页",
    panes: "窗格",
    split_vertical: "纵向拆分",
    split_horizontal: "横向拆分",
    close_pane: "关闭窗格",
    rename_pane: "重命名窗格",
    edit_scrollback: "编辑回滚内容",
    copy_mode: "复制模式",
    zoom_pane: "缩放窗格",
    resize_mode: "调整大小模式",
    toggle_sidebar: "切换侧边栏",
    focus_left: "聚焦左侧窗格",
    focus_down: "聚焦下方窗格",
    focus_up: "聚焦上方窗格",
    focus_right: "聚焦右侧窗格",
    cycle_next: "循环到下一个窗格",
    cycle_previous: "循环到上一个窗格",
    last_pane: "上次使用的窗格",
    custom: "自定义",
    custom_command: "自定义命令",
    no_matches: " 没有匹配的快捷键",
    close: "关闭",
    filter_hint: " 按 / 根据命令或快捷键筛选",
    filter: " 筛选 ",
    clear: "清除 ",
    scroll: "滚动 ",
    search: " 搜索 ",
};

pub(crate) fn help_text(language: UiLanguage) -> &'static HelpText {
    match language {
        UiLanguage::En => &HELP_EN,
        UiLanguage::ZhCn => &HELP_ZH_CN,
    }
}

pub(crate) struct SettingsText {
    pub title: &'static str,
    pub language_title: &'static str,
    pub language_description: &'static str,
    pub english: &'static str,
    pub simplified_chinese: &'static str,
    pub sound_alerts: &'static str,
    pub sound_description: &'static str,
    pub notification_popups: &'static str,
    pub notification_description: &'static str,
    pub off: &'static str,
    pub inside_herdr: &'static str,
    pub via_terminal: &'static str,
    pub via_system: &'static str,
    pub agent_border_labels: &'static str,
    pub agent_border_description: &'static str,
    pub close: &'static str,
    pub select: &'static str,
    pub section: &'static str,
    pub install: &'static str,
    pub apply: &'static str,
    pub install_hint: &'static str,
    pub all_installed: &'static str,
    pub no_agent_clis: &'static str,
    pub agent_integrations: &'static str,
    pub integrations_description: &'static str,
    pub no_integration_targets: &'static str,
    pub on: &'static str,
}

const SETTINGS_EN: SettingsText = SettingsText {
    title: " settings",
    language_title: "display language",
    language_description: "choose the language used by Herdr's built-in interface",
    english: "English",
    simplified_chinese: "简体中文",
    sound_alerts: "sound alerts",
    sound_description: "play sounds when agents change state in background",
    notification_popups: "notification popups",
    notification_description: "choose where background popup notifications should appear",
    off: "off",
    inside_herdr: "inside herdr",
    via_terminal: "via terminal",
    via_system: "via system",
    agent_border_labels: "agent border labels",
    agent_border_description: "show detected agent names in split pane borders",
    close: "close",
    select: " select  ",
    section: " section",
    install: "install",
    apply: "apply",
    install_hint: " press install to add available or outdated integrations",
    all_installed: " all detected integrations are installed",
    no_agent_clis: " no supported agent CLIs found on PATH",
    agent_integrations: "agent integrations",
    integrations_description:
        "let agents report state directly instead of relying only on process detection",
    no_integration_targets: " no integration targets available",
    on: "on",
};

const SETTINGS_ZH_CN: SettingsText = SettingsText {
    title: " 设置",
    language_title: "显示语言",
    language_description: "选择 Herdr 内置界面使用的语言",
    english: "English",
    simplified_chinese: "简体中文",
    sound_alerts: "声音提醒",
    sound_description: "后台智能体状态变化时播放提示音",
    notification_popups: "弹出通知",
    notification_description: "选择后台通知的显示位置",
    off: "关闭",
    inside_herdr: "在 Herdr 内显示",
    via_terminal: "通过终端显示",
    via_system: "通过系统显示",
    agent_border_labels: "智能体边框标签",
    agent_border_description: "在拆分窗格边框中显示检测到的智能体名称",
    close: "关闭",
    select: " 选择  ",
    section: " 切换分区",
    install: "安装",
    apply: "应用",
    install_hint: " 选择安装以添加可用或需要更新的集成",
    all_installed: " 检测到的集成都已安装",
    no_agent_clis: " PATH 中未发现支持的智能体命令行工具",
    agent_integrations: "智能体集成",
    integrations_description: "让智能体直接报告状态，而不只依赖进程检测",
    no_integration_targets: " 没有可用的集成目标",
    on: "开启",
};

pub(crate) fn settings_text(language: UiLanguage) -> &'static SettingsText {
    match language {
        UiLanguage::En => &SETTINGS_EN,
        UiLanguage::ZhCn => &SETTINGS_ZH_CN,
    }
}

pub(crate) struct SidebarText {
    pub grouped: &'static str,
    pub priority: &'static str,
    pub filtered: &'static str,
    pub spaces: &'static str,
    pub new: &'static str,
    pub menu: &'static str,
    pub agents: &'static str,
    pub no_matching_agents: &'static str,
}

const SIDEBAR_EN: SidebarText = SidebarText {
    grouped: "grouped",
    priority: "priority",
    filtered: "filtered",
    spaces: " spaces",
    new: " new",
    menu: "menu",
    agents: " agents",
    no_matching_agents: " no matching agents",
};

const SIDEBAR_ZH_CN: SidebarText = SidebarText {
    grouped: "按工作区",
    priority: "按优先级",
    filtered: "已筛选",
    spaces: " 工作区",
    new: " 新建",
    menu: "菜单",
    agents: " 智能体",
    no_matching_agents: " 没有匹配的智能体",
};

pub(crate) fn sidebar_text(language: UiLanguage) -> &'static SidebarText {
    match language {
        UiLanguage::En => &SIDEBAR_EN,
        UiLanguage::ZhCn => &SIDEBAR_ZH_CN,
    }
}

pub(crate) struct MenuText {
    pub unset: &'static str,
    pub prefix_mode: &'static str,
    pub cancel: &'static str,
    pub send_prefix: &'static str,
    pub workspace_nav: &'static str,
    pub keybinds: &'static str,
    pub copy_mode: &'static str,
    pub enter_search_cancel: &'static str,
    pub selecting: &'static str,
    pub select: &'static str,
    pub exit: &'static str,
    pub clear_and_exit: &'static str,
    pub move_hint: &'static str,
    pub search_hint: &'static str,
    pub repeat_hint: &'static str,
    pub copy_hint: &'static str,
    pub navigate_mode: &'static str,
    pub back: &'static str,
    pub workspace_short: &'static str,
    pub pane: &'static str,
    pub navigator: &'static str,
    pub new_tab: &'static str,
    pub split_vertical: &'static str,
    pub split_horizontal: &'static str,
    pub close: &'static str,
    pub zoom: &'static str,
    pub resize: &'static str,
    pub settings: &'static str,
    pub detach: &'static str,
    pub update_ready: &'static str,
    pub resize_mode: &'static str,
    pub width: &'static str,
    pub height: &'static str,
    pub done: &'static str,
}

const MENU_EN: MenuText = MenuText {
    unset: "unset",
    prefix_mode: " PREFIX ",
    cancel: " cancel  ",
    send_prefix: " send prefix  ",
    workspace_nav: " workspace nav  ",
    keybinds: " keybinds",
    copy_mode: " COPY ",
    enter_search_cancel: "  enter search  esc cancel",
    selecting: "selecting",
    select: "select",
    exit: " exit",
    clear_and_exit: " clear  q exit",
    move_hint: " move  ",
    search_hint: " search  ",
    repeat_hint: " repeat",
    copy_hint: " copy  ",
    navigate_mode: " NAVIGATE ",
    back: " back  ",
    workspace_short: " ws  ",
    pane: " pane  ",
    navigator: " navigator  ",
    new_tab: " new tab  ",
    split_vertical: " split│  ",
    split_horizontal: " split─  ",
    close: " close  ",
    zoom: " zoom  ",
    resize: " resize  ",
    settings: " settings  ",
    detach: " detach",
    update_ready: " update ready",
    resize_mode: " RESIZE ",
    width: " width  ",
    height: " height  ",
    done: " done",
};

const MENU_ZH_CN: MenuText = MenuText {
    unset: "未设置",
    prefix_mode: " 前缀 ",
    cancel: " 取消  ",
    send_prefix: " 发送前缀  ",
    workspace_nav: " 工作区导航  ",
    keybinds: " 快捷键",
    copy_mode: " 复制 ",
    enter_search_cancel: "  回车搜索  esc 取消",
    selecting: "选择中",
    select: "选择",
    exit: " 退出",
    clear_and_exit: " 清除  q 退出",
    move_hint: " 移动  ",
    search_hint: " 搜索  ",
    repeat_hint: " 重复",
    copy_hint: " 复制  ",
    navigate_mode: " 导航 ",
    back: " 返回  ",
    workspace_short: " 工作区  ",
    pane: " 窗格  ",
    navigator: " 会话导航  ",
    new_tab: " 新建标签页  ",
    split_vertical: " 纵向拆分│  ",
    split_horizontal: " 横向拆分─  ",
    close: " 关闭  ",
    zoom: " 缩放  ",
    resize: " 调整大小  ",
    settings: " 设置  ",
    detach: " 分离会话",
    update_ready: " 更新已就绪",
    resize_mode: " 调整大小 ",
    width: " 宽度  ",
    height: " 高度  ",
    done: " 完成",
};

pub(crate) fn menu_text(language: UiLanguage) -> &'static MenuText {
    match language {
        UiLanguage::En => &MENU_EN,
        UiLanguage::ZhCn => &MENU_ZH_CN,
    }
}

pub(crate) struct PluginCliText {
    pub plugin_about: &'static str,
    pub install_about: &'static str,
    pub uninstall_about: &'static str,
    pub link_about: &'static str,
    pub unlink_about: &'static str,
    pub enable_about: &'static str,
    pub disable_about: &'static str,
    pub list_about: &'static str,
    pub config_dir_about: &'static str,
    pub action_about: &'static str,
    pub action_list_about: &'static str,
    pub action_invoke_about: &'static str,
    pub log_about: &'static str,
    pub log_list_about: &'static str,
    pub pane_about: &'static str,
    pub pane_open_about: &'static str,
    pub pane_focus_about: &'static str,
    pub pane_close_about: &'static str,
    pub commands_heading: &'static str,
    pub action_commands_heading: &'static str,
    pub pane_commands_heading: &'static str,
    pub usage: &'static str,
    pub unknown_option: &'static str,
    pub missing_value: &'static str,
    pub invalid_limit: &'static str,
    pub noninteractive_install: &'static str,
    pub install_cancelled: &'static str,
    pub install_prompt: &'static str,
    pub missing_required: &'static str,
    pub invalid_placement: &'static str,
    pub invalid_direction: &'static str,
    pub no_plugins: &'static str,
    pub enabled: &'static str,
    pub disabled: &'static str,
    pub config: &'static str,
    pub install_config: &'static str,
    pub warning: &'static str,
    pub install_preview: &'static str,
    pub id: &'static str,
    pub name: &'static str,
    pub version: &'static str,
    pub source: &'static str,
    pub reference: &'static str,
    pub commit: &'static str,
    pub actions: &'static str,
    pub startup_commands: &'static str,
    pub events: &'static str,
    pub panes: &'static str,
    pub link_handlers: &'static str,
    pub build_commands: &'static str,
    pub build: &'static str,
    pub startup: &'static str,
    pub action: &'static str,
    pub event: &'static str,
    pub pane: &'static str,
    pub build_failed: &'static str,
    pub build_plugin: &'static str,
    pub build_step: &'static str,
    pub build_cwd: &'static str,
    pub build_command: &'static str,
    pub failed_to_start: &'static str,
    pub failed_to_wait: &'static str,
    pub build_status: &'static str,
    pub stderr: &'static str,
    pub stdout: &'static str,
    pub output_separator: &'static str,
    pub showing_last: &'static str,
    pub earlier_output_omitted: &'static str,
    pub plugin_not_installed: &'static str,
}

const PLUGIN_CLI_EN: PluginCliText = PluginCliText {
    plugin_about: "Install and run workflow plugins",
    install_about: "Install a plugin from GitHub",
    uninstall_about: "Uninstall a plugin",
    link_about: "Link a local plugin",
    unlink_about: "Unlink a local plugin",
    enable_about: "Enable a plugin",
    disable_about: "Disable a plugin",
    list_about: "List installed plugins",
    config_dir_about: "Print a plugin config directory",
    action_about: "List or invoke plugin actions",
    action_list_about: "List plugin actions",
    action_invoke_about: "Invoke a plugin action",
    log_about: "Inspect plugin command logs",
    log_list_about: "List plugin command logs",
    pane_about: "Manage plugin-owned panes",
    pane_open_about: "Open a plugin pane",
    pane_focus_about: "Focus a plugin pane",
    pane_close_about: "Close a plugin pane",
    commands_heading: "herdr plugin commands:",
    action_commands_heading: "herdr plugin action commands:",
    pane_commands_heading: "herdr plugin pane commands:",
    usage: "usage: ",
    unknown_option: "unknown option: ",
    missing_value: "missing value for ",
    invalid_limit: "invalid --limit value: ",
    noninteractive_install: "remote plugin install requires --yes when stdin is not interactive",
    install_cancelled: "plugin install cancelled",
    install_prompt: "Install this plugin?",
    missing_required: "missing required ",
    invalid_placement: "invalid pane placement: ",
    invalid_direction: "invalid split direction: ",
    no_plugins: "No plugins installed.",
    enabled: "enabled",
    disabled: "disabled",
    config: "config: ",
    install_config: "Config: ",
    warning: "warning: ",
    install_preview: "Plugin install preview:",
    id: "id: ",
    name: "name: ",
    version: "version: ",
    source: "source: ",
    reference: "ref: ",
    commit: "commit: ",
    actions: "actions: ",
    startup_commands: "startup commands: ",
    events: "events: ",
    panes: "panes: ",
    link_handlers: "link handlers: ",
    build_commands: "build commands: ",
    build: "build",
    startup: "startup",
    action: "action",
    event: "event",
    pane: "pane",
    build_failed: "error: plugin build failed",
    build_plugin: "  plugin: ",
    build_step: "  build: ",
    build_cwd: "  cwd: ",
    build_command: "  command: ",
    failed_to_start: "  error: failed to start: ",
    failed_to_wait: "  error: failed to wait for command: ",
    build_status: "  status: ",
    stderr: "stderr",
    stdout: "stdout",
    output_separator: ":",
    showing_last: " showing last ",
    earlier_output_omitted: " bytes; earlier output omitted",
    plugin_not_installed: "Plugin was not installed.",
};

const PLUGIN_CLI_ZH_CN: PluginCliText = PluginCliText {
    plugin_about: "安装并运行工作流插件",
    install_about: "从 GitHub 安装插件",
    uninstall_about: "卸载插件",
    link_about: "链接本地插件",
    unlink_about: "取消链接本地插件",
    enable_about: "启用插件",
    disable_about: "禁用插件",
    list_about: "列出已安装插件",
    config_dir_about: "输出插件配置目录",
    action_about: "列出或调用插件操作",
    action_list_about: "列出插件操作",
    action_invoke_about: "调用插件操作",
    log_about: "查看插件命令日志",
    log_list_about: "列出插件命令日志",
    pane_about: "管理插件所属窗格",
    pane_open_about: "打开插件窗格",
    pane_focus_about: "聚焦插件窗格",
    pane_close_about: "关闭插件窗格",
    commands_heading: "herdr 插件命令：",
    action_commands_heading: "herdr 插件操作命令：",
    pane_commands_heading: "herdr 插件窗格命令：",
    usage: "用法：",
    unknown_option: "未知选项：",
    missing_value: "缺少以下选项的值：",
    invalid_limit: "无效的 --limit 值：",
    noninteractive_install: "标准输入不是交互式终端时，远程插件安装需要 --yes",
    install_cancelled: "已取消插件安装",
    install_prompt: "安装此插件？",
    missing_required: "缺少必需选项：",
    invalid_placement: "无效的窗格位置：",
    invalid_direction: "无效的拆分方向：",
    no_plugins: "未安装任何插件。",
    enabled: "已启用",
    disabled: "已禁用",
    config: "配置：",
    install_config: "配置：",
    warning: "警告：",
    install_preview: "插件安装预览：",
    id: "ID：",
    name: "名称：",
    version: "版本：",
    source: "来源：",
    reference: "引用：",
    commit: "提交：",
    actions: "操作数：",
    startup_commands: "启动命令：",
    events: "事件数：",
    panes: "窗格数：",
    link_handlers: "链接处理器：",
    build_commands: "构建命令：",
    build: "构建",
    startup: "启动",
    action: "操作",
    event: "事件",
    pane: "窗格",
    build_failed: "错误：插件构建失败",
    build_plugin: "  插件：",
    build_step: "  构建：",
    build_cwd: "  工作目录：",
    build_command: "  命令：",
    failed_to_start: "  错误：启动失败：",
    failed_to_wait: "  错误：等待命令失败：",
    build_status: "  状态：",
    stderr: "标准错误",
    stdout: "标准输出",
    output_separator: "：",
    showing_last: "仅显示最后 ",
    earlier_output_omitted: " 字节；更早的输出已省略",
    plugin_not_installed: "插件未安装。",
};

pub(crate) fn plugin_cli_text(language: UiLanguage) -> &'static PluginCliText {
    match language {
        UiLanguage::En => &PLUGIN_CLI_EN,
        UiLanguage::ZhCn => &PLUGIN_CLI_ZH_CN,
    }
}

pub(crate) fn plugin_installed_count(language: UiLanguage, count: usize) -> String {
    match language {
        UiLanguage::En => format!(
            "{count} plugin{} installed:",
            if count == 1 { "" } else { "s" }
        ),
        UiLanguage::ZhCn => format!("已安装 {count} 个插件："),
    }
}

pub(crate) fn plugin_warning_count(language: UiLanguage, count: usize) -> String {
    match language {
        UiLanguage::En => format!("; {count} warning(s)"),
        UiLanguage::ZhCn => format!("；{count} 个警告"),
    }
}

pub(crate) fn plugin_installed_from(language: UiLanguage, plugin_id: &str, source: &str) -> String {
    match language {
        UiLanguage::En => format!("Installed {plugin_id} from {source}."),
        UiLanguage::ZhCn => format!("已从 {source} 安装 {plugin_id}。"),
    }
}

pub(crate) fn plugin_not_installed(language: UiLanguage, target: &str) -> String {
    match language {
        UiLanguage::En => format!("plugin not installed: {target}"),
        UiLanguage::ZhCn => format!("未安装插件：{target}"),
    }
}

pub(crate) fn plugin_uninstalled(language: UiLanguage, plugin_id: &str) -> String {
    match language {
        UiLanguage::En => format!("Uninstalled {plugin_id}."),
        UiLanguage::ZhCn => format!("已卸载 {plugin_id}。"),
    }
}

pub(crate) fn plugin_skipped_on(language: UiLanguage, platform: &str) -> String {
    match language {
        UiLanguage::En => format!(" (skipped on {platform})"),
        UiLanguage::ZhCn => format!("（在 {platform} 上跳过）"),
    }
}

pub(crate) fn plugin_replaces(language: UiLanguage, plugin_id: &str, source: &str) -> String {
    match language {
        UiLanguage::En => format!("  replaces: {plugin_id} from {source}"),
        UiLanguage::ZhCn => format!("  替换：{plugin_id}（来源：{source}）"),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn resolves_representative_messages_in_both_languages() {
        assert_eq!(
            text(UiLanguage::En, UiText::ReloadedConfig),
            "reloaded config"
        );
        assert_eq!(
            text(UiLanguage::ZhCn, UiText::ReloadedConfig),
            "已重新加载配置"
        );
        assert_eq!(
            context_menu_label(
                UiLanguage::ZhCn,
                crate::app::state::ContextMenuAction::ClosePane,
            ),
            "关闭窗格"
        );
        assert_eq!(settings_text(UiLanguage::ZhCn).title, " 设置");
        assert_eq!(pane_count(UiLanguage::ZhCn, 3), "3 个窗格");
        assert_eq!(
            plugin_cli_text(UiLanguage::En).install_preview,
            "Plugin install preview:"
        );
        assert_eq!(
            plugin_cli_text(UiLanguage::ZhCn).install_preview,
            "插件安装预览："
        );
        assert_eq!(
            plugin_installed_from(UiLanguage::ZhCn, "example.plugin", "owner/repo"),
            "已从 owner/repo 安装 example.plugin。"
        );
        assert_eq!(
            plugin_cli_text(UiLanguage::ZhCn).install_prompt,
            "安装此插件？"
        );
        assert_eq!(
            plugin_cli_text(UiLanguage::ZhCn).install_cancelled,
            "已取消插件安装"
        );
        assert_eq!(
            plugin_not_installed(UiLanguage::ZhCn, "example.plugin"),
            "未安装插件：example.plugin"
        );
    }
}
