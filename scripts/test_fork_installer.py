import hashlib
import http.server
import os
import pathlib
import subprocess
import tempfile
import threading
import unittest

ROOT = pathlib.Path(__file__).resolve().parents[1]
INSTALLER = ROOT / "install.sh"
ASSET = "herdr-linux-x86_64"


class QuietHandler(http.server.SimpleHTTPRequestHandler):
    def log_message(self, _format, *_args):
        pass


class ReleaseServer:
    def __init__(self, directory: pathlib.Path):
        handler = lambda *args, **kwargs: QuietHandler(*args, directory=str(directory), **kwargs)
        self.server = http.server.ThreadingHTTPServer(("127.0.0.1", 0), handler)
        self.thread = threading.Thread(target=self.server.serve_forever, daemon=True)

    def __enter__(self):
        self.thread.start()
        host, port = self.server.server_address
        self.url = f"http://{host}:{port}"
        return self

    def __exit__(self, *_args):
        self.server.shutdown()
        self.thread.join()
        self.server.server_close()


def write_executable(path: pathlib.Path, content: str):
    path.write_text(content, encoding="utf-8")
    path.chmod(0o755)


def write_release(directory: pathlib.Path, binary: str, checksum: str | None = None):
    asset = directory / ASSET
    write_executable(asset, binary)
    digest = checksum or hashlib.sha256(asset.read_bytes()).hexdigest()
    (directory / "checksums.txt").write_text(f"{digest}  {ASSET}\n", encoding="utf-8")


def installer_env(home: pathlib.Path, base_url: str):
    env = os.environ.copy()
    env.update(
        {
            "HOME": str(home),
            "HERDR_INSTALL_DIR": str(home / "bin"),
            "HERDR_RELEASE_BASE_URL": base_url,
            "HERDR_OS": "linux",
            "HERDR_ARCH": "x86_64",
        }
    )
    env.pop("HERDR_ENV", None)
    return env


FRESH_BINARY = """#!/bin/sh
if [ "${1:-}" = "--version" ]; then
  echo "herdr 0.7.5-fork.zh.1"
  exit 0
fi
if [ "${1:-}" = "status" ] && [ "${2:-}" = "client" ]; then
  echo '{"version":"0.7.5-fork.zh.1","protocol":18}'
  exit 0
fi
exit 0
"""


class ForkInstallerTests(unittest.TestCase):
    def test_shell_syntax(self):
        subprocess.run(["sh", "-n", str(INSTALLER)], check=True)

    def test_fresh_install_verifies_and_enables_chinese(self):
        with tempfile.TemporaryDirectory() as temp:
            root = pathlib.Path(temp)
            release = root / "release"
            home = root / "home"
            release.mkdir()
            home.mkdir()
            write_release(release, FRESH_BINARY)

            with ReleaseServer(release) as server:
                result = subprocess.run(
                    ["sh", str(INSTALLER)],
                    env=installer_env(home, server.url),
                    text=True,
                    capture_output=True,
                )

            self.assertEqual(result.returncode, 0, result.stderr)
            installed = home / "bin" / "herdr"
            self.assertTrue(installed.is_file())
            self.assertTrue(os.access(installed, os.X_OK))
            self.assertEqual(installed.read_text(encoding="utf-8"), FRESH_BINARY)
            config = home / ".config" / "herdr" / "config.toml"
            self.assertEqual(config.read_text(encoding="utf-8"), '[ui]\nlanguage = "zh-cn"\n')
            self.assertIn("SHA-256 校验通过", result.stdout)

    def test_checksum_mismatch_aborts_without_installing(self):
        with tempfile.TemporaryDirectory() as temp:
            root = pathlib.Path(temp)
            release = root / "release"
            home = root / "home"
            release.mkdir()
            home.mkdir()
            write_release(release, FRESH_BINARY, checksum="0" * 64)

            with ReleaseServer(release) as server:
                result = subprocess.run(
                    ["sh", str(INSTALLER)],
                    env=installer_env(home, server.url),
                    text=True,
                    capture_output=True,
                )

            self.assertNotEqual(result.returncode, 0)
            self.assertFalse((home / "bin" / "herdr").exists())
            self.assertIn("SHA-256 校验失败", result.stderr)

    def test_running_named_session_refuses_to_replace_binary(self):
        with tempfile.TemporaryDirectory() as temp:
            root = pathlib.Path(temp)
            release = root / "release"
            home = root / "home"
            release.mkdir()
            (home / "bin").mkdir(parents=True)
            old_content = """#!/bin/sh
if [ "${1:-}" = "session" ]; then
  echo '{"sessions":[{"default":false,"name":"work","running":true}]}'
  exit 0
fi
exit 0
"""
            old = home / "bin" / "herdr"
            write_executable(old, old_content)
            write_release(release, FRESH_BINARY)

            with ReleaseServer(release) as server:
                result = subprocess.run(
                    ["sh", str(INSTALLER)],
                    env=installer_env(home, server.url),
                    text=True,
                    capture_output=True,
                )

            self.assertNotEqual(result.returncode, 0)
            self.assertEqual(old.read_text(encoding="utf-8"), old_content)
            self.assertIn("命名 session", result.stderr)

    def test_running_default_session_uses_live_handoff(self):
        with tempfile.TemporaryDirectory() as temp:
            root = pathlib.Path(temp)
            release = root / "release"
            home = root / "home"
            log = root / "handoff.log"
            release.mkdir()
            (home / "bin").mkdir(parents=True)
            old = home / "bin" / "herdr"
            write_executable(
                old,
                """#!/bin/sh
if [ "${1:-}" = "session" ]; then
  echo '{"sessions":[{"default":true,"running":true}]}'
  exit 0
fi
if [ "${1:-}" = "status" ]; then
  echo '{"server":{"running":true}}'
  exit 0
fi
exit 0
""",
            )
            new_binary = """#!/bin/sh
if [ "${1:-}" = "--version" ]; then
  echo "herdr 0.7.5-fork.zh.1"
  exit 0
fi
if [ "${1:-}" = "status" ] && [ "${2:-}" = "client" ]; then
  echo '{"version":"0.7.5-fork.zh.1","protocol":18}'
  exit 0
fi
if [ "${1:-}" = "server" ] && [ "${2:-}" = "live-handoff" ]; then
  printf '%s\n' "$*" > "$HERDR_FAKE_LOG"
  exit 0
fi
exit 0
"""
            write_release(release, new_binary)

            with ReleaseServer(release) as server:
                env = installer_env(home, server.url)
                env["HERDR_FAKE_LOG"] = str(log)
                result = subprocess.run(
                    ["sh", str(INSTALLER)], env=env, text=True, capture_output=True
                )

            self.assertEqual(result.returncode, 0, result.stderr)
            self.assertIn("server live-handoff", log.read_text(encoding="utf-8"))
            self.assertEqual(old.read_text(encoding="utf-8"), new_binary)
            self.assertEqual(len(list((home / "bin").glob("herdr.backup-*"))), 1)


if __name__ == "__main__":
    unittest.main()
