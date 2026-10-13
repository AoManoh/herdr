use std::borrow::Cow;

use crate::config::UiLanguage;

pub(crate) fn text(language: UiLanguage, english: &'static str) -> &'static str {
    if language == UiLanguage::En {
        return english;
    }
    match english {
        "settings" => "设置",
        "keybinds" => "快捷键",
        "reload config" => "重新加载配置",
        "update ready" => "更新已就绪",
        "what's new" => "版本动态",
        "detach" => "分离会话",
        "Rename" => "重命名",
        "Close" => "关闭",
        "Close group" => "关闭工作区组",
        "New worktree" => "新建工作树",
        "Open worktree..." => "打开工作树...",
        "Delete worktree checkout..." => "删除工作树检出...",
        "Collapse" => "折叠",
        "Expand" => "展开",
        "New tab" => "新建标签页",
        "Rename pane" => "重命名窗格",
        "Clear pane name" => "清除窗格名称",
        "Swap with focused pane" => "与当前窗格交换",
        "Split right" => "向右拆分",
        "Split down" => "向下拆分",
        "Zoom" => "缩放",
        "Use Herdr right-click menu" => "使用 Herdr 右键菜单",
        "Send right-clicks to pane" => "将右键点击发送到窗格",
        "Close pane" => "关闭窗格",
        "theme" => "主题",
        "indicators" => "状态指示",
        "sound" => "声音",
        "toasts" => "通知",
        "integrations" => "集成",
        "display language" => "显示语言",
        "choose the language used by Herdr's built-in interface" => "选择 Herdr 内置界面使用的语言",
        "English" => "English",
        "Simplified Chinese" => "简体中文",
        "agent status indicators" => "智能体状态指示",
        "choose color dots or distinct symbols for each state" => {
            "为每种状态选择彩色圆点或不同符号"
        }
        "color dots  ● ● ● ○ ·" => "彩色圆点  ● ● ● ○ ·",
        "distinct symbols  × ◐ ✓ ○ ·" => "不同符号  × ◐ ✓ ○ ·",
        "sound alerts" => "声音提醒",
        "play sounds when agents change state in background" => "后台智能体状态变化时播放提示音",
        "notification popups" => "弹出通知",
        "choose where background popup notifications should appear" => "选择后台通知的显示位置",
        "off" => "关闭",
        "inside herdr" => "在 Herdr 内显示",
        "via terminal" => "通过终端显示",
        "via system" => "通过系统显示",
        "agent integrations" => "智能体集成",
        "enable session restore and, where supported, direct status updates" => {
            "启用会话恢复，并在支持时直接更新状态"
        }
        "loading integrations…" => "正在加载集成…",
        "no integration targets available" => "没有可用的集成目标",
        "installed" => "已安装",
        "update available" => "有可用更新",
        "available" => "可用",
        "not found" => "未找到",
        "installing…" => "正在安装…",
        "on" => "开启",
        "close" => "关闭",
        "save" => "保存",
        "clear" => "清除",
        "cancel" => "取消",
        "confirm" => "确认",
        "continue" => "继续",
        "apply" => "应用",
        "install" => "安装",
        "select" => "选择",
        "section" => "分区",
        "new workspace" => "新建工作区",
        "rename workspace" => "重命名工作区",
        "new tab" => "新建标签页",
        "rename tab" => "重命名标签页",
        "rename pane" => "重命名窗格",
        "new worktree" => "新建工作树",
        "create and open" => "创建并打开",
        "delete anyway" => "仍然删除",
        "remove" => "移除",
        "open" => "打开",
        "checkouts" => "检出目录",
        "panes" => "窗格",
        " branch" => " 分支",
        " checkout" => " 检出目录",
        " creating…" => " 正在创建…",
        " open worktree" => " 打开工作树",
        " no matching worktrees" => " 没有匹配的工作树",
        "filter worktrees" => "筛选工作树",
        " delete worktree checkout?" => " 删除工作树检出？",
        " This removes the checkout folder:" => " 将删除以下检出文件夹：",
        " The branch is not deleted. The Herdr workspace will close." => {
            " 分支不会被删除，对应的 Herdr 工作区将关闭。"
        }
        " Dirty or untracked files will be permanently deleted." => {
            " 未提交或未跟踪的文件将被永久删除。"
        }
        " removing…" => " 正在移除…",
        " opening…" => " 正在打开…",
        "search agents and terminals" => "搜索智能体和终端",
        "Go to" => "跳转到",
        " No matching agents or terminals" => " 没有匹配的智能体或终端",
        "auth" => "需认证",
        "error" => "错误",
        "Close workspace?" => "关闭工作区？",
        "Close worktree group?" => "关闭工作树分组？",
        "Close target changed; try closing the tab again" => "关闭目标已变化，请重新关闭该标签页",
        "workspaces" => "工作区",
        "terminals" => "终端",
        "blocked" => "等待输入",
        "working" => "工作中",
        "idle" => "空闲",
        "done" => "已完成",
        "unknown" => "未知",
        "no workspace" => " 没有工作区",
        "no agents" => "没有智能体",
        "all idle" => "全部空闲",
        "agents" => "智能体",
        "machines" => "机器",
        "switch" => "切换",
        "spaces" => "工作区",
        "shell" => "终端",
        "tabs" => "标签页",
        "tab" => "标签页",
        "menu" => "菜单",
        "filtered" => "已筛选",
        "no matching agents" => "没有匹配的智能体",
        "grouped" => "按工作区",
        "priority" => "按优先级",
        "new" => "新建",
        "connecting" => "连接中",
        "online" => "在线",
        "reconnecting" => "重新连接",
        "attention" => "需要注意",
        "disabled" => "已禁用",
        "what's new in this release" => "本版本的新内容",
        "product announcement preview" => "产品公告预览",
        "product announcement" => "产品公告",
        " scroll " => " 滚动 ",
        "wheel ↑↓" => "滚轮 ↑↓",
        "wheel" => "滚轮",
        "esc close" => "esc 关闭",
        "esc back" => "esc 返回",
        "↵ continue" => "↵ 继续",
        "↵ save" => "↵ 保存",
        "^c clear" => "^c 清除",
        "esc cancel" => "esc 取消",
        "↵ confirm" => "↵ 确认",
        "press / to filter by command or shortcut" => "按 / 根据命令或快捷键筛选",
        "no matching keybinds" => "没有匹配的快捷键",
        "search type · move ↑↓/ctrl+n/p · open enter · back esc" => {
            "搜索输入 · ↑↓/ctrl+n/p 移动 · 回车打开 · esc 返回"
        }
        " ↑↓/j/k rows · ←→ workspace · / search · a/b/w/i/d filter · enter open · esc close" => {
            " ↑↓/j/k 选择 · ←→ 切换工作区 · / 搜索 · a/b/w/i/d 筛选 · 回车打开 · esc 关闭"
        }
        "edit ←→/home/end · kill ^u/^k · yank ^y · scroll ↑↓ · back esc" => {
            "←→/home/end 编辑 · ^u/^k 删除 · ^y 粘贴 · ↑↓ 滚动 · esc 返回"
        }
        "search / · scroll j/k/↑↓/pgup/pgdn · close esc/enter" => {
            "/ 搜索 · j/k/↑↓/pgup/pgdn 滚动 · esc/回车关闭"
        }
        " RESIZE " => " 调整大小 ",
        " PREFIX " => " 前缀 ",
        " NAVIGATE " => " 导航 ",
        " COPY " => " 复制 ",
        "cancel  " => "取消  ",
        "send prefix  " => "发送前缀  ",
        "workspace nav  " => "工作区导航  ",
        " keybinds" => " 快捷键",
        "esc back  " => "esc 返回  ",
        " workspace  " => " 工作区  ",
        " pane  " => " 窗格  ",
        " width  " => " 宽度  ",
        " height  " => " 高度  ",
        " done" => " 完成",
        "enter search  esc cancel" => "回车搜索  esc 取消",
        "selecting" => "选择中",
        " exit" => " 退出",
        " clear  q exit" => " 清除  q 退出",
        " move  " => " 移动  ",
        " search  " => " 搜索  ",
        " repeat" => " 重复",
        " copy  " => " 复制  ",
        " navigator  " => " 会话导航  ",
        " new tab  " => " 新建标签页  ",
        " split│  " => " 纵向拆分│  ",
        " split─  " => " 横向拆分─  ",
        " close  " => " 关闭  ",
        " zoom  " => " 缩放  ",
        " resize  " => " 调整大小  ",
        " settings  " => " 设置  ",
        " detach" => " 分离会话",
        " unset" => " 未设置",
        " ERROR " => " 错误 ",
        "  terminal workspace manager for coding agents" => "  面向编程智能体的终端工作区管理器",
        "  this is a mouse-first terminal." => "  这是一个优先支持鼠标操作的终端。",
        "  click the sidebar to switch workspaces, drag pane" => {
            "  点击侧边栏切换工作区，拖动窗格边框"
        }
        "  borders to resize, right-click for context menus." => "  调整大小，右键打开上下文菜单。",
        " enters prefix mode · " => " 进入前缀模式 · ",
        " shows keybinds and settings" => " 显示快捷键和设置",
        "  next: install optional agent integrations for more reliable state" => {
            "  下一步：安装可选的智能体集成，以获得更可靠的状态识别"
        }
        "  + new workspace" => "  + 新建工作区",
        "  + new tab" => "  + 新建标签页",
        _ => english,
    }
}

pub(crate) fn status_label(
    language: UiLanguage,
    status: crate::api::schema::AgentStatus,
) -> &'static str {
    use crate::api::schema::AgentStatus;
    match status {
        AgentStatus::Blocked => text(language, "blocked"),
        AgentStatus::Working => text(language, "working"),
        AgentStatus::Done => text(language, "done"),
        AgentStatus::Idle => text(language, "idle"),
        AgentStatus::Unknown => text(language, "unknown"),
    }
}

pub(crate) fn endpoint_label(
    language: UiLanguage,
    status: crate::client::endpoint::ClientEndpointStatus,
) -> &'static str {
    use crate::client::endpoint::ClientEndpointStatus;
    match status {
        ClientEndpointStatus::Connecting => text(language, "connecting"),
        ClientEndpointStatus::Online => text(language, "online"),
        ClientEndpointStatus::Reconnecting => text(language, "reconnecting"),
        ClientEndpointStatus::Attention => text(language, "attention"),
        ClientEndpointStatus::Disabled => text(language, "disabled"),
    }
}

pub(crate) fn count(
    language: UiLanguage,
    value: usize,
    singular: &'static str,
    plural: &'static str,
) -> String {
    let label = if value == 1 { singular } else { plural };
    if language == UiLanguage::En {
        format!("{value} {label}")
    } else {
        format!("{value} {}", text(language, plural))
    }
}

pub(crate) fn keybind_label(language: UiLanguage, label: &str) -> Cow<'_, str> {
    if language == UiLanguage::En {
        return Cow::Borrowed(label);
    }
    let translated = match label {
        "global" => "全局",
        "prefix mode" => "前缀模式",
        "keybinds" => "快捷键",
        "settings" => "设置",
        "detach" => "分离会话",
        "reload config" => "重新加载配置",
        "navigation" => "导航",
        "back" => "返回",
        "workspace list" => "工作区列表",
        "move focus" => "移动焦点",
        "cycle pane" => "循环切换窗格",
        "open workspace" => "打开工作区",
        "switch workspace" => "切换工作区",
        "workspaces / tabs" => "工作区 / 标签页",
        "workspace navigation" => "工作区导航",
        "session navigator" => "会话导航",
        "new workspace" => "新建工作区",
        "new worktree" => "新建工作树",
        "open worktree" => "打开工作树",
        "delete worktree checkout" => "删除工作树检出",
        "rename workspace" => "重命名工作区",
        "close workspace" => "关闭工作区",
        "previous workspace" => "上一个工作区",
        "next workspace" => "下一个工作区",
        "switch workspace 1-9" => "切换工作区 1-9",
        "previous agent" => "上一个智能体",
        "next agent" => "下一个智能体",
        "focus agent 1-9" => "聚焦智能体 1-9",
        "new tab" => "新建标签页",
        "rename tab" => "重命名标签页",
        "previous tab" => "上一个标签页",
        "next tab" => "下一个标签页",
        "switch tab 1-9" => "切换标签页 1-9",
        "close tab" => "关闭标签页",
        "panes" => "窗格",
        "split vertical" => "纵向拆分",
        "split horizontal" => "横向拆分",
        "close pane" => "关闭窗格",
        "clear pane" => "清空窗格",
        "rename pane" => "重命名窗格",
        "edit scrollback" => "编辑回滚内容",
        "copy mode" => "复制模式",
        "zoom pane" => "缩放窗格",
        "resize mode" => "调整大小模式",
        "toggle sidebar" => "切换侧边栏",
        "focus pane left" => "聚焦左侧窗格",
        "focus pane down" => "聚焦下方窗格",
        "focus pane up" => "聚焦上方窗格",
        "focus pane right" => "聚焦右侧窗格",
        "cycle pane next" => "循环到下一个窗格",
        "cycle pane previous" => "循环到上一个窗格",
        "last pane" => "上次使用的窗格",
        "custom" => "自定义",
        "custom command" => "自定义命令",
        other => return Cow::Borrowed(other),
    };
    Cow::Borrowed(translated)
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
    fn resolves_shell_and_plugin_messages_in_both_languages() {
        assert_eq!(text(UiLanguage::En, "settings"), "settings");
        assert_eq!(text(UiLanguage::ZhCn, "settings"), "设置");
        assert_eq!(
            status_label(UiLanguage::ZhCn, crate::api::schema::AgentStatus::Blocked),
            "等待输入"
        );
        assert_eq!(
            plugin_cli_text(UiLanguage::ZhCn).install_preview,
            "插件安装预览："
        );
        assert_eq!(
            plugin_installed_from(UiLanguage::ZhCn, "example.plugin", "owner/repo"),
            "已从 owner/repo 安装 example.plugin。"
        );
    }

    #[test]
    fn resolves_v0_9_3_shell_messages() {
        assert_eq!(text(UiLanguage::En, "Close workspace?"), "Close workspace?");
        assert_eq!(text(UiLanguage::ZhCn, "Close workspace?"), "关闭工作区？");
        assert_eq!(text(UiLanguage::ZhCn, "Go to"), "跳转到");
        assert_eq!(
            text(
                UiLanguage::ZhCn,
                " ↑↓/j/k rows · ←→ workspace · / search · a/b/w/i/d filter · enter open · esc close"
            ),
            " ↑↓/j/k 选择 · ←→ 切换工作区 · / 搜索 · a/b/w/i/d 筛选 · 回车打开 · esc 关闭"
        );
        assert_eq!(
            count(UiLanguage::En, 1, "terminal", "terminals"),
            "1 terminal"
        );
        assert_eq!(
            count(UiLanguage::ZhCn, 3, "terminal", "terminals"),
            "3 终端"
        );
        assert_eq!(keybind_label(UiLanguage::ZhCn, "clear pane"), "清空窗格");
    }
}
