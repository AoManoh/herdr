#!/bin/sh
set -eu

BIN="herdr"
REPO="${HERDR_REPO:-AoManoh/herdr}"
INSTALL_DIR="${HERDR_INSTALL_DIR:-$HOME/.local/bin}"
RELEASE_VERSION="${HERDR_VERSION:-latest}"
TMP=""
STAGE=""

log() { printf '  > %s\n' "$1"; }
warn() { printf '  ! %s\n' "$1" >&2; }
die() { printf '  x %s\n' "$1" >&2; exit 1; }

cleanup() {
    if [ -n "$TMP" ] && [ -d "$TMP" ]; then
        rm -rf "$TMP"
    fi
    if [ -n "$STAGE" ] && [ -e "$STAGE" ]; then
        rm -f "$STAGE"
    fi
}

need() {
    command -v "$1" >/dev/null 2>&1 || die "需要命令：$1"
}

sha256_file() {
    if command -v sha256sum >/dev/null 2>&1; then
        sha256sum "$1" | awk '{print $1}'
    elif command -v shasum >/dev/null 2>&1; then
        shasum -a 256 "$1" | awk '{print $1}'
    else
        die "需要 sha256sum 或 shasum 来校验下载文件"
    fi
}

clean_herdr_env() {
    env \
        -u HERDR_ENV \
        -u HERDR_SOCKET_PATH \
        -u HERDR_CLIENT_SOCKET_PATH \
        -u HERDR_SESSION \
        -u HERDR_WORKSPACE_ID \
        -u HERDR_TAB_ID \
        -u HERDR_PANE_ID \
        "$@"
}

main() {
    need curl
    need awk
    need grep
    need mktemp
    need env

    raw_os="${HERDR_OS:-$(uname -s)}"
    case "$raw_os" in
        Linux|linux) os="linux" ;;
        Darwin|macos) os="macos" ;;
        *) die "暂不支持的操作系统：$raw_os" ;;
    esac

    raw_arch="${HERDR_ARCH:-$(uname -m)}"
    case "$raw_arch" in
        x86_64|amd64) arch="x86_64" ;;
        aarch64|arm64) arch="aarch64" ;;
        *) die "暂不支持的架构：$raw_arch" ;;
    esac

    asset="herdr-${os}-${arch}"
    if [ -n "${HERDR_RELEASE_BASE_URL:-}" ]; then
        base_url="$HERDR_RELEASE_BASE_URL"
    elif [ "$RELEASE_VERSION" = "latest" ]; then
        base_url="https://github.com/${REPO}/releases/latest/download"
    else
        base_url="https://github.com/${REPO}/releases/download/${RELEASE_VERSION}"
    fi

    TMP="$(mktemp -d)"
    trap cleanup EXIT HUP INT TERM

    log "检测到 ${os}/${arch}"
    log "下载 ${RELEASE_VERSION}"
    curl -fsSL --retry 3 --connect-timeout 10 --max-time 180 \
        "${base_url}/checksums.txt" -o "${TMP}/checksums.txt" \
        || die "无法下载 checksums.txt"
    curl -fsSL --retry 3 --connect-timeout 10 --max-time 300 \
        "${base_url}/${asset}" -o "${TMP}/${BIN}" \
        || die "无法下载 ${asset}"

    expected="$(awk -v name="$asset" '$2 == name { print $1; exit }' "${TMP}/checksums.txt")"
    [ -n "$expected" ] || die "checksums.txt 中没有 ${asset}"
    actual="$(sha256_file "${TMP}/${BIN}")"
    [ "$actual" = "$expected" ] || die "SHA-256 校验失败"
    log "SHA-256 校验通过"

    chmod 0755 "${TMP}/${BIN}"
    "${TMP}/${BIN}" --version >/dev/null 2>&1 || die "下载的二进制无法运行"

    mkdir -p "$INSTALL_DIR"
    destination="${INSTALL_DIR}/${BIN}"

    if [ -x "$destination" ]; then
        current_hash="$(sha256_file "$destination")"
        if [ "$current_hash" = "$actual" ]; then
            log "当前已经是目标版本：$destination"
            return 0
        fi

        sessions="$(clean_herdr_env "$destination" session list --json 2>/dev/null || true)"
        if printf '%s\n' "$sessions" | grep -Eq '"default":false[^}]*"running":true'; then
            die "检测到运行中的命名 session；为避免协议不一致，请先逐个 live-handoff 后再升级"
        fi

        status="$(clean_herdr_env "$destination" status --json 2>/dev/null || true)"
        if printf '%s\n' "$status" | grep -Eq '"running":[[:space:]]*true'; then
            [ "${HERDR_ENV:-}" != "1" ] \
                || die "请在 Herdr TUI 外的普通终端中重新运行安装命令"

            backup="${destination}.backup-$(date -u +%Y%m%dT%H%M%SZ)"
            cp -p "$destination" "$backup"
            log "旧二进制已备份到 $backup"

            STAGE="${INSTALL_DIR}/.herdr-install-$$"
            cp "${TMP}/${BIN}" "$STAGE"
            chmod 0755 "$STAGE"
            new_version="$($STAGE --version | awk '{print $2}')"
            client_status="$(clean_herdr_env "$STAGE" status client --json)"
            new_protocol="$(printf '%s\n' "$client_status" | sed -n 's/.*"protocol":\([0-9][0-9]*\).*/\1/p')"
            if [ -z "$new_version" ] || [ -z "$new_protocol" ]; then
                die "无法读取新二进制的版本或协议"
            fi

            log "正在通过 live-handoff 保留运行中的 panes"
            clean_herdr_env "$STAGE" server live-handoff \
                --import-exe "$STAGE" \
                --expected-protocol "$new_protocol" \
                --expected-version "$new_version" \
                || die "live-handoff 失败，旧二进制和 server 保持不变"

            mv -f "$STAGE" "$destination" \
                || {
                    STAGE=""
                    die "handoff 已完成，但无法替换 $destination；新 server 仍从 staging 路径运行"
                }
            STAGE=""
        else
            STAGE="${INSTALL_DIR}/.herdr-install-$$"
            cp "${TMP}/${BIN}" "$STAGE"
            chmod 0755 "$STAGE"
            mv -f "$STAGE" "$destination"
            STAGE=""
        fi
    else
        STAGE="${INSTALL_DIR}/.herdr-install-$$"
        cp "${TMP}/${BIN}" "$STAGE"
        chmod 0755 "$STAGE"
        mv -f "$STAGE" "$destination"
        STAGE=""
    fi

    config_dir="${XDG_CONFIG_HOME:-$HOME/.config}/herdr"
    config_path="${config_dir}/config.toml"
    if [ ! -e "$config_path" ]; then
        mkdir -p "$config_dir"
        umask 077
        printf '[ui]\nlanguage = "zh-cn"\n' > "$config_path"
        log "已为首次安装启用简体中文"
    fi

    log "已安装到 $destination"
    "$destination" --version

    case ":${PATH}:" in
        *":${INSTALL_DIR}:"*) ;;
        *) warn "${INSTALL_DIR} 不在 PATH 中；请添加：export PATH=\"${INSTALL_DIR}:\$PATH\"" ;;
    esac

    log "运行 herdr 开始使用；设置中可在 Language / 语言切换中英文"
}

main "$@"
