use ratatui::{
    layout::{Alignment, Rect},
    style::{Modifier, Style},
    text::{Line, Span},
    widgets::{Clear, List, ListItem, ListState, Paragraph},
    Frame,
};

use super::widgets::{panel_contrast_fg, render_panel_shell};
use crate::app::AppState;

fn prefix_rhs_label(
    bindings: &crate::config::ActionKeybinds,
    language: crate::config::UiLanguage,
) -> String {
    bindings
        .prefix_rhs_label()
        .unwrap_or_else(|| crate::i18n::menu_text(language).unset.to_string())
}

fn keybind_label(
    bindings: &crate::config::ActionKeybinds,
    language: crate::config::UiLanguage,
) -> String {
    bindings
        .label()
        .unwrap_or_else(|| crate::i18n::menu_text(language).unset.to_string())
}

fn render_bottom_bar(frame: &mut Frame, area: Rect, line: Line<'_>, bg: ratatui::style::Color) {
    frame.render_widget(Clear, area);
    let buf = frame.buffer_mut();
    for x in area.x..area.x + area.width {
        buf[(x, area.y)].set_style(Style::default().bg(bg));
    }
    frame.render_widget(Paragraph::new(line), area);
}

pub(super) fn render_prefix_overlay(app: &AppState, frame: &mut Frame, area: Rect) {
    let key = Style::default()
        .fg(app.palette.accent)
        .add_modifier(Modifier::BOLD);
    let dim = Style::default().fg(app.palette.overlay0);
    let mode_style = Style::default()
        .fg(panel_contrast_fg(&app.palette))
        .bg(app.palette.accent)
        .add_modifier(Modifier::BOLD);

    let t = crate::i18n::menu_text(app.ui_language);
    let workspace_picker = prefix_rhs_label(&app.keybinds.workspace_picker, app.ui_language);
    let help = prefix_rhs_label(&app.keybinds.help, app.ui_language);
    let prefix = crate::config::format_key_combo((app.prefix_code, app.prefix_mods));

    let line = Line::from(vec![
        Span::styled(t.prefix_mode, mode_style),
        Span::raw(" "),
        Span::styled("esc", key),
        Span::styled(t.cancel, dim),
        Span::styled(prefix, key),
        Span::styled(t.send_prefix, dim),
        Span::styled(workspace_picker, key),
        Span::styled(t.workspace_nav, dim),
        Span::styled(help, key),
        Span::styled(t.keybinds, dim),
    ]);

    let overlay_y = area.y + area.height.saturating_sub(1);
    let overlay_area = Rect::new(area.x, overlay_y, area.width, 1);
    render_bottom_bar(frame, overlay_area, line, app.palette.panel_bg);
}

pub(super) fn render_copy_mode_overlay(app: &AppState, frame: &mut Frame, area: Rect) {
    let key = Style::default()
        .fg(app.palette.accent)
        .add_modifier(Modifier::BOLD);
    let dim = Style::default().fg(app.palette.overlay0);
    let mode_style = Style::default()
        .fg(panel_contrast_fg(&app.palette))
        .bg(app.palette.accent)
        .add_modifier(Modifier::BOLD);

    let Some(copy_mode) = app.copy_mode.as_ref() else {
        return;
    };
    let t = crate::i18n::menu_text(app.ui_language);
    let line = if let Some(prompt) = copy_mode.search.prompt.as_ref() {
        let marker = match prompt.direction {
            crate::app::state::CopyModeSearchDirection::Forward => "/",
            crate::app::state::CopyModeSearchDirection::Backward => "?",
        };
        Line::from(vec![
            Span::styled(t.copy_mode, mode_style),
            Span::raw(" "),
            Span::styled(marker, key),
            Span::styled(prompt.query.clone(), Style::default().fg(app.palette.text)),
            Span::styled("█", key),
            Span::styled(t.enter_search_cancel, dim),
        ])
    } else {
        let select = if copy_mode.selection.is_some() {
            t.selecting
        } else {
            t.select
        };
        let match_status = copy_mode
            .search
            .current
            .map(|current| format!(" {}/{}", current + 1, copy_mode.search.matches.len()))
            .or_else(|| (!copy_mode.search.query.is_empty()).then(|| " 0/0".to_string()))
            .unwrap_or_default();
        let (exit_keys, exit_label) =
            if copy_mode.search.query.is_empty() && copy_mode.selection.is_none() {
                ("q/esc", t.exit)
            } else {
                ("esc", t.clear_and_exit)
            };
        Line::from(vec![
            Span::styled(t.copy_mode, mode_style),
            Span::raw(" "),
            Span::styled("h/j/k/l w/b/e { }", key),
            Span::styled(t.move_hint, dim),
            Span::styled("/ ?", key),
            Span::styled(t.search_hint, dim),
            Span::styled("n/N", key),
            Span::styled(format!(" {}{match_status}  ", t.repeat_hint.trim()), dim),
            Span::styled("v/space", key),
            Span::styled(format!(" {select}  "), dim),
            Span::styled("y/enter", key),
            Span::styled(t.copy_hint, dim),
            Span::styled(exit_keys, key),
            Span::styled(exit_label, dim),
        ])
    };

    let overlay_y = area.y + area.height.saturating_sub(1);
    let overlay_area = Rect::new(area.x, overlay_y, area.width, 1);
    render_bottom_bar(frame, overlay_area, line, app.palette.panel_bg);
}

pub(super) fn render_navigate_overlay(app: &AppState, frame: &mut Frame, area: Rect) {
    let key = Style::default()
        .fg(app.palette.accent)
        .add_modifier(Modifier::BOLD);
    let dim = Style::default().fg(app.palette.overlay0);

    let mode_style = Style::default()
        .fg(panel_contrast_fg(&app.palette))
        .bg(app.palette.accent)
        .add_modifier(Modifier::BOLD);

    let t = crate::i18n::menu_text(app.ui_language);
    let kb = &app.keybinds;
    let new_tab = prefix_rhs_label(&kb.new_tab, app.ui_language);
    let split_vertical = prefix_rhs_label(&kb.split_vertical, app.ui_language);
    let split_horizontal = prefix_rhs_label(&kb.split_horizontal, app.ui_language);
    let close_pane = prefix_rhs_label(&kb.close_pane, app.ui_language);
    let zoom = prefix_rhs_label(&kb.zoom, app.ui_language);
    let resize = prefix_rhs_label(&kb.resize_mode, app.ui_language);
    let help = prefix_rhs_label(&kb.help, app.ui_language);
    let settings = prefix_rhs_label(&kb.settings, app.ui_language);
    let goto = prefix_rhs_label(&kb.goto, app.ui_language);
    let detach = prefix_rhs_label(&kb.detach, app.ui_language);
    let workspace_nav = format!(
        "{} / {}",
        keybind_label(&kb.navigate.workspace_up, app.ui_language),
        keybind_label(&kb.navigate.workspace_down, app.ui_language)
    );
    let line = Line::from(vec![
        Span::styled(t.navigate_mode, mode_style),
        Span::raw(" "),
        Span::styled("esc", key),
        Span::styled(t.back, dim),
        Span::styled(workspace_nav, key),
        Span::styled(t.workspace_short, dim),
        Span::styled("⇥", key),
        Span::styled(t.pane, dim),
        Span::styled(goto, key),
        Span::styled(t.navigator, dim),
        Span::styled(new_tab, key),
        Span::styled(t.new_tab, dim),
        Span::styled(split_vertical, key),
        Span::styled(t.split_vertical, dim),
        Span::styled(split_horizontal, key),
        Span::styled(t.split_horizontal, dim),
        Span::styled(close_pane, key),
        Span::styled(t.close, dim),
        Span::styled(zoom, key),
        Span::styled(t.zoom, dim),
        Span::styled(resize, key),
        Span::styled(t.resize, dim),
        Span::styled(help, key),
        Span::styled(format!("{}  ", t.keybinds), dim),
        Span::styled(settings, key),
        Span::styled(t.settings, dim),
        Span::styled(detach, key),
        Span::styled(t.detach, dim),
    ]);

    let overlay_y = area.y + area.height.saturating_sub(1);
    let overlay_area = Rect::new(area.x, overlay_y, area.width, 1);
    render_bottom_bar(frame, overlay_area, line, app.palette.panel_bg);

    if app.update_available.is_some() {
        let status = Line::from(vec![Span::styled(
            t.update_ready,
            Style::default()
                .fg(app.palette.accent)
                .add_modifier(Modifier::BOLD),
        )]);
        let width = super::text::display_width_u16(t.update_ready).min(overlay_area.width);
        let status_area = Rect::new(
            overlay_area.x + overlay_area.width.saturating_sub(width),
            overlay_area.y,
            width,
            overlay_area.height,
        );
        frame.render_widget(Clear, status_area);
        frame.render_widget(
            Paragraph::new(status).alignment(Alignment::Right),
            status_area,
        );
    }
}

pub(super) fn render_global_launcher_menu(app: &AppState, frame: &mut Frame) {
    let rect = app.global_menu_rect();
    let Some(inner) = render_panel_shell(frame, rect, app.palette.accent, app.palette.panel_bg)
    else {
        return;
    };

    let actions = app.global_menu_actions();
    for (idx, action) in actions.iter().copied().enumerate() {
        let y = inner.y + idx as u16;
        if y >= inner.y + inner.height {
            break;
        }
        let selected = idx == app.global_menu.highlighted;
        let rect = Rect::new(inner.x, y, inner.width, 1);

        let selected_style = Style::default()
            .fg(panel_contrast_fg(&app.palette))
            .bg(app.palette.accent)
            .add_modifier(Modifier::BOLD);
        let item_style = if selected {
            selected_style
        } else {
            Style::default().fg(app.palette.text)
        };
        let badge_style = if selected {
            selected_style
        } else {
            Style::default()
                .fg(app.palette.accent)
                .add_modifier(Modifier::BOLD)
        };
        let label =
            crate::i18n::global_menu_label(app.ui_language, action, app.update_available.is_some());

        let line = if app.global_menu_action_has_badge(action) {
            Line::from(vec![
                Span::styled(" ●", badge_style),
                Span::styled(format!(" {label} "), item_style),
            ])
        } else {
            Line::from(Span::styled(format!(" {label} "), item_style))
        };
        frame.render_widget(Paragraph::new(line).alignment(Alignment::Left), rect);
    }
}

pub(super) fn render_resize_overlay(app: &AppState, frame: &mut Frame, area: Rect) {
    let t = crate::i18n::menu_text(app.ui_language);
    let key = Style::default()
        .fg(app.palette.accent)
        .add_modifier(Modifier::BOLD);
    let dim = Style::default().fg(app.palette.overlay0);

    let mode_style = Style::default()
        .fg(panel_contrast_fg(&app.palette))
        .bg(app.palette.mauve)
        .add_modifier(Modifier::BOLD);

    let line = Line::from(vec![
        Span::styled(t.resize_mode, mode_style),
        Span::raw("  "),
        Span::styled("h/l", key),
        Span::styled(t.width, dim),
        Span::styled("j/k", key),
        Span::styled(t.height, dim),
        Span::styled("esc", key),
        Span::styled(t.done, dim),
    ]);

    let overlay_y = area.y + area.height.saturating_sub(1);
    let overlay_area = Rect::new(area.x, overlay_y, area.width, 1);
    render_bottom_bar(frame, overlay_area, line, app.palette.panel_bg);
}

pub(super) fn render_context_menu(app: &AppState, frame: &mut Frame) {
    let Some(menu) = &app.context_menu else {
        return;
    };

    let p = &app.palette;
    let Some(menu_rect) = app.context_menu_rect() else {
        return;
    };
    let Some(inner) = render_panel_shell(frame, menu_rect, p.accent, p.panel_bg) else {
        return;
    };

    let items: Vec<ListItem> = menu
        .actions()
        .iter()
        .map(|action| {
            ListItem::new(Line::from(crate::i18n::context_menu_label(
                app.ui_language,
                *action,
            )))
        })
        .collect();
    let list = List::new(items)
        .style(Style::default().fg(p.text))
        .highlight_style(
            Style::default()
                .bg(p.accent)
                .fg(panel_contrast_fg(p))
                .add_modifier(Modifier::BOLD),
        )
        .highlight_symbol(" ");
    let mut state = ListState::default().with_selected(Some(menu.list.highlighted));
    frame.render_stateful_widget(list, inner, &mut state);
}
