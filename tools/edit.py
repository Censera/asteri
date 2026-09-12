#!/usr/bin/env python3
"""Apply small, verified edits to text files.

The editor is intentionally strict: every edit must match the expected number of
occurrences, and the file is written only after all edits succeed.
"""

from __future__ import annotations

import argparse
import difflib
import re
import sys
from dataclasses import dataclass
from pathlib import Path


class EditError(Exception):
    """Raised when an edit cannot be applied exactly as requested."""


@dataclass(frozen=True)
class Edit:
    kind: str
    old: str
    new: str = ""
    count: int = 1


def replace_exact(text: str, edit: Edit) -> str:
    found = text.count(edit.old)
    if found != edit.count:
        raise EditError(
            f"expected {edit.count} exact match(es), found {found}: {edit.old!r}"
        )
    return text.replace(edit.old, edit.new)


def replace_regex(text: str, edit: Edit) -> str:
    found = len(re.findall(edit.old, text, flags=re.MULTILINE))
    if found != edit.count:
        raise EditError(
            f"expected {edit.count} regex match(es), found {found}: {edit.old!r}"
        )
    return re.sub(edit.old, edit.new, text, count=0, flags=re.MULTILINE)


def apply_edit(text: str, edit: Edit) -> str:
    if edit.count < 1:
        raise EditError("count must be at least 1")

    if edit.kind == "replace":
        return replace_exact(text, edit)
    if edit.kind == "regex":
        return replace_regex(text, edit)
    if edit.kind == "before":
        return replace_exact(text, Edit("replace", edit.old, edit.old + edit.new, edit.count))
    if edit.kind == "after":
        return replace_exact(text, Edit("replace", edit.old, edit.new + edit.old, edit.count))
    if edit.kind == "delete":
        return replace_exact(text, Edit("replace", edit.old, "", edit.count))
    raise EditError(f"unknown edit kind: {edit.kind}")


def diff_text(old: str, new: str, path: Path) -> str:
    return "".join(
        difflib.unified_diff(
            old.splitlines(keepends=True),
            new.splitlines(keepends=True),
            fromfile=str(path),
            tofile=str(path),
        )
    )


def build_parser() -> argparse.ArgumentParser:
    parser = argparse.ArgumentParser(description="Apply verified edits to a text file.")
    parser.add_argument("path", type=Path)
    parser.add_argument("--write", action="store_true", help="write the result")
    parser.add_argument("--diff", action="store_true", help="print the resulting unified diff")
    parser.add_argument(
        "--edit",
        action="append",
        nargs=4,
        metavar=("KIND", "OLD", "NEW", "COUNT"),
        help="edit tuple: replace|before|after|delete|regex OLD NEW COUNT",
    )
    return parser


def main() -> int:
    args = build_parser().parse_args()

    try:
        original = args.path.read_text(encoding="utf-8")
    except OSError as exc:
        print(f"error: cannot read {args.path}: {exc}", file=sys.stderr)
        return 1

    edits: list[Edit] = []
    for raw in args.edit or []:
        kind, old, new, count = raw
        try:
            edits.append(Edit(kind=kind, old=old, new=new, count=int(count)))
        except ValueError:
            print(f"error: invalid count: {count!r}", file=sys.stderr)
            return 1

    result = original
    try:
        for edit in edits:
            result = apply_edit(result, edit)
    except EditError as exc:
        print(f"error: {exc}", file=sys.stderr)
        return 1

    if args.diff:
        diff = diff_text(original, result, args.path)
        if diff:
            sys.stdout.write(diff)
        else:
            print("no changes")

    if args.write:
        if result != original:
            try:
                args.path.write_text(result, encoding="utf-8")
            except OSError as exc:
                print(f"error: cannot write {args.path}: {exc}", file=sys.stderr)
                return 1
        elif not args.diff:
            print("no changes")

    if not args.write and not args.diff:
        sys.stdout.write(result)

    return 0


if __name__ == "__main__":
    raise SystemExit(main())
