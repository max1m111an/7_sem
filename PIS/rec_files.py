#!/usr/bin/env python3
"""
rec_files — скачивает файлы из публичной папки Яндекс.Диска по шаблону имени,
переименовывает в соответствии с номером задания (папки) и сохраняет
в каталог, из которого вызван скрипт (плоско, расширение сохраняется).

Использование:
    ./rec_files.py <ref> <pattern> [<prefix>]

    <ref>     публичная ссылка Яндекс.Диска или публичный ключ
    <pattern> шаблон файла, напр. "var04" (любое расширение) или "var04.js" (только .js)
    <prefix>  префикс целевого имени, по умолчанию "task"

Именование (по имени папки, содержащей файл):
    "Задание №N"          -> <prefix>N.<ext>        (напр. task1.js)
    "...Итоговое задание" -> <prefix>_itog.<ext>    (напр. task_itog.js)
    любая другая папка    -> <prefix>_<папка>.<ext>
"""

import argparse
import json
import re
import sys
import urllib.parse
from pathlib import Path

import requests

API_ROOT = "https://cloud-api.yandex.net/v1/disk/public/resources"
API_DOWNLOAD = f"{API_ROOT}/download"
PAGE_LIMIT = 100
TIMEOUT_LIST = 60
TIMEOUT_DOWNLOAD = 180


def api_get(url: str, params: dict) -> dict:
    """GET к API Яндекс.Диска с проверкой статуса и разбором JSON."""
    resp = requests.get(url, params=params, timeout=TIMEOUT_LIST)
    if resp.status_code != 200:
        raise RuntimeError(
            f"HTTP {resp.status_code} для {url}\n"
            f"Ответ: {resp.text}"
        )
    return resp.json()


def api_list(ref: str, path: str) -> list[dict]:
    """Возвращает все элементы (файлы и папки) по указанному пути, с пагинацией."""
    items: list[dict] = []
    offset = 0
    total = 1
    while offset < total:
        data = api_get(API_ROOT, {
            "public_key": ref,
            "path": path,
            "limit": PAGE_LIMIT,
            "offset": offset,
        })
        embedded = data.get("_embedded") or {}
        total = embedded.get("total") or 0
        items.extend(embedded.get("items") or [])
        offset += PAGE_LIMIT
    return items


def api_href(ref: str, path: str) -> str:
    """Возвращает прямую ссылку для скачивания файла по пути."""
    data = api_get(API_DOWNLOAD, {"public_key": ref, "path": path})
    href = data.get("href")
    if not href:
        raise RuntimeError(f"Не получена ссылка для {path}")
    return href


def api_root_type(ref: str) -> str:
    """Возвращает тип корневого ресурса: 'dir' или 'file'."""
    data = api_get(API_ROOT, {"public_key": ref})
    return data.get("type") or ""


def match_pattern(fname: str, pattern: str) -> str | None:
    """
    Проверяет, соответствует ли имя файла шаблону.
    Возвращает расширение с точкой или пустую строку, либо None, если не совпало.

    Логика повторяет bash:
      - если в шаблоне есть точка (pattern = 'var04.js'), имя должно совпасть точно;
      - иначе имя должно быть ровно 'var04' или начинаться с 'var04.'.
    """
    pat_ext = ""
    if "." in pattern:
        pat_ext = pattern.rsplit(".", 1)[1]
        if fname == pattern:
            return f".{pat_ext}"
        return None

    if fname == pattern:
        return ""
    if fname.startswith(pattern + "."):
        return fname[len(pattern):]
    return None


def target_name(folder: str, prefix: str) -> str:
    """
    Формирует целевое имя (без расширения) по имени папки.

    'Задание №N'          -> <prefix>N
    '...Итоговое задание' -> <prefix>_itog
    прочее                -> <prefix>_<папка>
    """
    m = re.search(r"[Зз]адание[^0-9]*([0-9]+)", folder)
    if m:
        return f"{prefix}{m.group(1)}"
    if "итог" in folder.lower():
        return f"{prefix}_itog"
    return f"{prefix}_{folder}"


def sanitize_filename(name: str) -> str:
    """Заменяет символы, недопустимые в именах файлов на большинстве ОС."""
    return re.sub(r'[<>:"/\\|?*\x00-\x1f]', "_", name)


def download_file(href: str, dest: Path) -> int:
    """Скачивает файл по ссылке в dest. Возвращает размер в байтах."""
    with requests.get(href, stream=True, timeout=TIMEOUT_DOWNLOAD) as resp:
        resp.raise_for_status()
        with dest.open("wb") as f:
            for chunk in resp.iter_content(chunk_size=65536):
                if chunk:
                    f.write(chunk)
    return dest.stat().st_size


def process_file(
    ref: str,
    out_dir: Path,
    prefix: str,
    fname: str,
    fpath: str,
    folder: str,
    pattern: str,
) -> bool:
    """
    Обрабатывает один файл: проверяет шаблон, формирует имя, скачивает.
    Возвращает True, если файл был скачан.
    """
    ext = match_pattern(fname, pattern)
    if ext is None:
        return False

    base = sanitize_filename(target_name(folder, prefix))
    target = out_dir / f"{base}{ext}"

    if target.exists():
        print(f"WARN: {target} уже существует, перезапись")

    print(f"  {fpath} -> {target.name}: ", end="", flush=True)
    try:
        href = api_href(ref, fpath)
    except RuntimeError as e:
        print(f"не удалось получить ссылку ({e})")
        return False

    try:
        size = download_file(href, target)
        print(f"сохранено ({size} байт)")
        return True
    except Exception as e:
        print(f"ошибка загрузки: {e}")
        if target.exists():
            target.unlink()
        return False


def walk_dir(
    ref: str,
    out_dir: Path,
    prefix: str,
    path: str,
    folder: str,
    pattern: str,
) -> None:
    """Рекурсивно обходит папку и обрабатывает файлы, подходящие под шаблон."""
    try:
        items = api_list(ref, path)
    except RuntimeError as e:
        print(f"ERROR: список не получен: {path}\n  {e}", file=sys.stderr)
        return

    for it in items:
        itype = it.get("type")
        iname = it.get("name") or ""
        ipath = it.get("path") or ""
        if itype == "dir":
            walk_dir(ref, out_dir, prefix, ipath, iname, pattern)
        elif itype == "file":
            process_file(ref, out_dir, prefix, iname, ipath, folder, pattern)


def parse_args(argv: list[str]) -> argparse.Namespace:
    parser = argparse.ArgumentParser(
        prog="rec_files.py",
        description="Скачивает файлы из публичной папки Яндекс.Диска по шаблону.",
    )
    parser.add_argument("ref", help="публичная ссылка Яндекс.Диска или публичный ключ")
    parser.add_argument("pattern", help="шаблон имени файла, напр. var04 или var04.js")
    parser.add_argument(
        "prefix",
        nargs="?",
        default="task",
        help="префикс целевого имени (по умолчанию: task)",
    )
    return parser.parse_args(argv)


def main(argv: list[str]) -> int:
    args = parse_args(argv)
    out_dir = Path.cwd()
    prefix = args.prefix
    pattern = args.pattern
    ref = args.ref

    print(f"Сохранение в: {out_dir}")

    try:
        root_type = api_root_type(ref)
    except RuntimeError as e:
        print(f"ERROR: не удалось получить корневой ресурс: {e}", file=sys.stderr)
        return 1

    if root_type == "file":
        # REF указывает на один файл — скачиваем его напрямую.
        root_name = urllib.parse.unquote(ref.rstrip("/").rsplit("/", 1)[-1])
        process_file(ref, out_dir, prefix, root_name, "", "", pattern)
    else:
        walk_dir(ref, out_dir, prefix, "", "", pattern)

    print("Готово.")
    return 0


if __name__ == "__main__":
    sys.exit(main(sys.argv[1:]))