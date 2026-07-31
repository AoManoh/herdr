use std::borrow::Cow;

use ratatui::{
    layout::{Constraint, Layout, Rect},
    style::{Modifier, Style},
    text::{Line, Span},
    widgets::{Paragraph, Wrap},
    Frame,
};

use super::release_notes::release_notes_close_button_rect;
use super::scrollbar::{release_notes_scrollbar_rect, render_scrollbar};
use super::text::{display_width, pad_to_display_width};
use super::widgets::{
    modal_stack_areas, panel_contrast_fg, render_action_button, render_modal_header,
    render_modal_shell,
};
use crate::app::AppState;

pub(super) type HelpEntry = (String, Cow<'static, str>);
pub(super) type HelpGroup = (&'static str, Vec<HelpEntry>);

fn help_entry(key: impl Into<String>, label: &'static str) -> HelpEntry {
    (key.into(), Cow::Borrowed(label))
}

fn resolved_label(bindings: &crate::config::ActionKeybinds, unset: &str) -> String {
    bindings.label().unwrap_or_else(|| unset.to_string())
}

fn indexed_label(bindings: &[crate::config::IndexedKeybind], unset: &str) -> String {
    if bindings.is_empty() {
        return unset.to_string();
    }

    let mut parts = Vec::new();
    let mut index = 0;
    while index < bindings.len() {
        if let Some(prefix) = indexed_range_prefix(&bindings[index..]) {
            parts.push(format!("{prefix}1..9"));
            index += 9;
        } else {
            parts.push(bindings[index].label.clone());
            index += 1;
        }
    }

    parts.join(" / ")
}

fn indexed_range_prefix(bindings: &[crate::config::IndexedKeybind]) -> Option<&str> {
    let run = bindings.get(..9)?;
    let prefix = run[0].label.strip_suffix('1')?;
    for (offset, binding) in run.iter().enumerate() {
        let digit = char::from(b'1' + offset as u8);
        if binding.label.strip_suffix(digit) != Some(prefix) {
            return None;
        }
    }
    Some(prefix)
}

pub(super) fn keybind_help_groups(app: &AppState) -> Vec<HelpGroup> {
    let kb = &app.keybinds;
    let t = crate::i18n::help_text(app.ui_language);
    let label = |bindings| resolved_label(bindings, t.unset);
    let mut groups = Vec::new();

    groups.push((
        t.global,
        vec![
            help_entry(
                crate::config::format_key_combo((app.prefix_code, app.prefix_mods)),
                t.prefix_mode,
            ),
            help_entry(label(&kb.help), t.keybinds),
            help_entry(label(&kb.settings), t.settings),
            help_entry(label(&kb.detach), t.detach),
            help_entry(label(&kb.reload_config), t.reload_config),
            help_entry(
                label(&kb.open_notification_target),
                t.open_notification_target,
            ),
        ],
    ));

    groups.push((
        t.navigation,
        vec![
            help_entry("esc", t.back),
            help_entry(
                format!(
                    "{} / {}",
                    label(&kb.navigate.workspace_up),
                    label(&kb.navigate.workspace_down)
                ),
                t.workspace_list,
            ),
            help_entry(
                format!(
                    "{} / {} / {} / {} / left / right",
                    label(&kb.navigate.pane_left),
                    label(&kb.navigate.pane_down),
                    label(&kb.navigate.pane_up),
                    label(&kb.navigate.pane_right)
                ),
                t.move_focus,
            ),
            help_entry("tab / shift+tab", t.cycle_pane),
            help_entry("enter", t.open_workspace),
            help_entry("1..9", t.switch_workspace),
        ],
    ));

    let workspace_tab = vec![
        help_entry(label(&kb.workspace_picker), t.workspace_navigation),
        help_entry(label(&kb.goto), t.session_navigator),
        help_entry(label(&kb.new_workspace), t.new_workspace),
        help_entry(label(&kb.new_worktree), t.new_worktree),
        help_entry(label(&kb.open_worktree), t.open_worktree),
        help_entry(label(&kb.remove_worktree), t.delete_worktree),
        help_entry(label(&kb.rename_workspace), t.rename_workspace),
        help_entry(label(&kb.close_workspace), t.close_workspace),
        help_entry(label(&kb.previous_workspace), t.previous_workspace),
        help_entry(label(&kb.next_workspace), t.next_workspace),
        help_entry(
            indexed_label(&kb.switch_workspace, t.unset),
            t.switch_workspace_indexed,
        ),
        help_entry(label(&kb.previous_agent), t.previous_agent),
        help_entry(label(&kb.next_agent), t.next_agent),
        help_entry(indexed_label(&kb.focus_agent, t.unset), t.focus_agent),
        help_entry(label(&kb.new_tab), t.new_tab),
        help_entry(label(&kb.rename_tab), t.rename_tab),
        help_entry(label(&kb.previous_tab), t.previous_tab),
        help_entry(label(&kb.next_tab), t.next_tab),
        help_entry(indexed_label(&kb.switch_tab, t.unset), t.switch_tab),
        help_entry(label(&kb.close_tab), t.close_tab),
    ];
    groups.push((t.workspaces_tabs, workspace_tab));

    let panes = vec![
        help_entry(label(&kb.split_vertical), t.split_vertical),
        help_entry(label(&kb.split_horizontal), t.split_horizontal),
        help_entry(label(&kb.close_pane), t.close_pane),
        help_entry(label(&kb.rename_pane), t.rename_pane),
        help_entry(label(&kb.edit_scrollback), t.edit_scrollback),
        help_entry(label(&kb.copy_mode), t.copy_mode),
        help_entry(label(&kb.zoom), t.zoom_pane),
        help_entry(label(&kb.resize_mode), t.resize_mode),
        help_entry(label(&kb.toggle_sidebar), t.toggle_sidebar),
        help_entry(label(&kb.focus_pane_left), t.focus_left),
        help_entry(label(&kb.focus_pane_down), t.focus_down),
        help_entry(label(&kb.focus_pane_up), t.focus_up),
        help_entry(label(&kb.focus_pane_right), t.focus_right),
        help_entry(label(&kb.cycle_pane_next), t.cycle_next),
        help_entry(label(&kb.cycle_pane_previous), t.cycle_previous),
        help_entry(label(&kb.last_pane), t.last_pane),
    ];
    groups.push((t.panes, panes));

    if !kb.custom_commands.is_empty() {
        groups.push((
            t.custom,
            kb.custom_commands
                .iter()
                .map(|binding| {
                    (
                        binding.label.clone(),
                        binding
                            .description
                            .clone()
                            .map(Cow::Owned)
                            .unwrap_or(Cow::Borrowed(t.custom_command)),
                    )
                })
                .collect(),
        ));
    }

    groups
}

fn filter_keybind_help_groups(groups: Vec<HelpGroup>, query: &str) -> Vec<HelpGroup> {
    if query.is_empty() {
        return groups;
    }

    let query = query.to_lowercase();
    groups
        .into_iter()
        .filter_map(|(group, entries)| {
            let entries = entries
                .into_iter()
                .filter(|(key, label)| {
                    key.to_lowercase().contains(&query) || label.to_lowercase().contains(&query)
                })
                .collect::<Vec<_>>();
            (!entries.is_empty()).then_some((group, entries))
        })
        .collect()
}

pub(crate) fn keybind_help_lines(app: &AppState) -> Vec<(usize, Line<'static>)> {
    let heading_style = Style::default()
        .fg(app.palette.accent)
        .add_modifier(Modifier::BOLD);
    let key_style = Style::default()
        .fg(app.palette.mauve)
        .add_modifier(Modifier::BOLD);
    let label_style = Style::default().fg(app.palette.text);

    let groups = filter_keybind_help_groups(keybind_help_groups(app), &app.keybind_help.query);
    let t = crate::i18n::help_text(app.ui_language);
    let key_width = groups
        .iter()
        .flat_map(|(_, entries)| entries.iter().map(|(key, _)| display_width(key)))
        .max()
        .unwrap_or(8);

    let mut lines = Vec::new();

    if groups.is_empty() {
        let message = t.no_matches;
        return vec![(
            display_width(message),
            Line::from(Span::styled(
                message,
                Style::default().fg(app.palette.overlay1),
            )),
        )];
    }

    for (group, entries) in groups {
        lines.push((
            display_width(group) + 1,
            Line::from(vec![Span::styled(format!(" {group}"), heading_style)]),
        ));
        for (key, label) in entries {
            let padded_key = format!(" {} ", pad_to_display_width(&key, key_width));
            let width = display_width(&padded_key) + display_width(&label);
            lines.push((
                width,
                Line::from(vec![
                    Span::styled(padded_key, key_style),
                    Span::styled(label.into_owned(), label_style),
                ]),
            ));
        }
        lines.push((0, Line::raw("")));
    }

    lines
}

pub(super) fn render_keybind_help_overlay(app: &AppState, frame: &mut Frame) {
    let t = crate::i18n::help_text(app.ui_language);
    super::dim_background(frame, frame.area());

    let Some(inner) = render_modal_shell(frame, frame.area(), 76, 22, &app.palette) else {
        return;
    };
    if inner.height < 6 || inner.width < 20 {
        return;
    }

    let stack = modal_stack_areas(inner, 2, 1, 0, 1);
    let header_rows =
        Layout::vertical([Constraint::Length(1), Constraint::Length(1)]).areas::<2>(stack.header);

    render_modal_header(frame, header_rows[0], t.keybinds, &app.palette);
    render_action_button(
        frame,
        release_notes_close_button_rect(
            header_rows[0],
            if app.keybind_help.search_focused {
                t.back
            } else {
                t.close
            },
        ),
        Some("esc"),
        if app.keybind_help.search_focused {
            t.back
        } else {
            t.close
        },
        Style::default()
            .fg(panel_contrast_fg(&app.palette))
            .bg(app.palette.accent)
            .add_modifier(Modifier::BOLD),
    );
    let search_line = if app.keybind_help.search_focused {
        Line::from(vec![
            Span::styled(
                " / ",
                Style::default()
                    .fg(app.palette.accent)
                    .add_modifier(Modifier::BOLD),
            ),
            Span::styled(
                app.keybind_help.query.as_str(),
                Style::default()
                    .fg(app.palette.text)
                    .add_modifier(Modifier::BOLD),
            ),
        ])
    } else {
        Line::from(Span::styled(
            t.filter_hint,
            Style::default().fg(app.palette.overlay0),
        ))
    };
    frame.render_widget(Paragraph::new(search_line), header_rows[1]);

    let body_area = stack.content;
    let metrics = crate::pane::ScrollMetrics {
        offset_from_bottom: app
            .keybind_help_max_scroll()
            .saturating_sub(app.keybind_help.scroll) as usize,
        max_offset_from_bottom: app.keybind_help_max_scroll() as usize,
        viewport_rows: body_area.height.max(1) as usize,
    };
    let track = release_notes_scrollbar_rect(body_area, metrics);
    let text_area = track
        .map(|_| {
            Rect::new(
                body_area.x,
                body_area.y,
                body_area.width.saturating_sub(1),
                body_area.height,
            )
        })
        .unwrap_or(body_area);

    let body = Paragraph::new(
        keybind_help_lines(app)
            .into_iter()
            .map(|(_, line)| line)
            .collect::<Vec<_>>(),
    )
    .wrap(Wrap { trim: false })
    .scroll((app.keybind_help.scroll, 0));
    frame.render_widget(body, text_area);
    if let Some(track) = track {
        render_scrollbar(
            frame,
            metrics,
            track,
            app.palette.overlay0,
            app.palette.overlay1,
            "▐",
        );
    }

    let footer = if app.keybind_help.search_focused {
        Line::from(vec![
            Span::styled(t.filter, Style::default().fg(app.palette.overlay0)),
            Span::styled("type/backspace", Style::default().fg(app.palette.text)),
            Span::styled(" · ", Style::default().fg(app.palette.overlay0)),
            Span::styled(t.clear, Style::default().fg(app.palette.overlay0)),
            Span::styled("ctrl+u", Style::default().fg(app.palette.text)),
            Span::styled(" · ", Style::default().fg(app.palette.overlay0)),
            Span::styled(t.scroll, Style::default().fg(app.palette.overlay0)),
            Span::styled("↑↓/pgup/pgdn", Style::default().fg(app.palette.text)),
            Span::styled(" · ", Style::default().fg(app.palette.overlay0)),
            Span::styled(
                format!("{} ", t.back),
                Style::default().fg(app.palette.overlay0),
            ),
            Span::styled("esc", Style::default().fg(app.palette.text)),
        ])
    } else {
        Line::from(vec![
            Span::styled(t.search, Style::default().fg(app.palette.overlay0)),
            Span::styled("/", Style::default().fg(app.palette.text)),
            Span::styled(" · ", Style::default().fg(app.palette.overlay0)),
            Span::styled(t.scroll, Style::default().fg(app.palette.overlay0)),
            Span::styled("j/k/↑↓/pgup/pgdn", Style::default().fg(app.palette.text)),
            Span::styled(" · ", Style::default().fg(app.palette.overlay0)),
            Span::styled(
                format!("{} ", t.close),
                Style::default().fg(app.palette.overlay0),
            ),
            Span::styled("esc/enter", Style::default().fg(app.palette.text)),
        ])
    };
    frame.render_widget(Paragraph::new(footer), stack.footer.unwrap_or_default());
}

#[cfg(test)]
mod tests {
    use super::*;

    fn groups() -> Vec<HelpGroup> {
        vec![
            (
                "workspaces / tabs",
                vec![
                    help_entry("w", "workspace navigation"),
                    help_entry("c", "new tab"),
                ],
            ),
            (
                "panes",
                vec![
                    help_entry("v", "split vertical"),
                    help_entry("x", "close pane"),
                ],
            ),
        ]
    }

    #[test]
    fn keybind_help_filter_matches_labels_case_insensitively() {
        let filtered = filter_keybind_help_groups(groups(), "WoRk");

        assert_eq!(filtered.len(), 1);
        assert_eq!(filtered[0].0, "workspaces / tabs");
        assert_eq!(filtered[0].1.len(), 1);
        assert_eq!(filtered[0].1[0].1, "workspace navigation");
    }

    #[test]
    fn keybind_help_filter_matches_shortcuts_without_matching_group_headings() {
        let filtered = filter_keybind_help_groups(groups(), "x");

        assert_eq!(filtered.len(), 1);
        assert_eq!(filtered[0].0, "panes");
        assert_eq!(filtered[0].1.len(), 1);
        assert_eq!(filtered[0].1[0].1, "close pane");

        assert!(filter_keybind_help_groups(groups(), "panes").is_empty());
    }
}
