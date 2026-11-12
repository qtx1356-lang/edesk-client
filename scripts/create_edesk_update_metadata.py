#!/usr/bin/env python3
"""Create signed eDesk update metadata for a Windows x64 stable release."""

from __future__ import annotations

import argparse
import base64
import hashlib
import json
import os
import re
from datetime import datetime, timezone
from pathlib import Path

from cryptography.hazmat.primitives.asymmetric.ed25519 import Ed25519PrivateKey


REPOSITORY = "qtx1356-lang/edesk-client"
APP_ID = "edesk"
KEY_ID = "edesk-2026-main"
SIGNATURE_CONTEXT = b"eDesk update metadata v1\n"


def main() -> None:
    parser = argparse.ArgumentParser()
    parser.add_argument("--version", required=True)
    parser.add_argument("--tag", required=True)
    parser.add_argument("--package", required=True, type=Path)
    parser.add_argument("--output-dir", default=Path("dist"), type=Path)
    args = parser.parse_args()

    if not re.fullmatch(r"[0-9]+\.[0-9]+\.[0-9]+", args.version):
        raise SystemExit("Version must be a stable x.y.z version.")
    expected_tag = f"edesk-v{args.version}"
    if args.tag != expected_tag:
        raise SystemExit(
            f"Tag/version mismatch: expected {expected_tag}, got {args.tag}."
        )

    package = args.package.resolve(strict=True)
    expected_name = f"edesk-{args.version}-install.exe"
    if package.name != expected_name or not package.is_file():
        raise SystemExit(f"Expected the update package {expected_name}.")

    encoded_seed = os.environ.get("EDESK_UPDATE_SIGNING_KEY", "").strip()
    if not encoded_seed:
        raise SystemExit("EDESK_UPDATE_SIGNING_KEY repository secret is missing.")
    try:
        seed = base64.b64decode(encoded_seed, validate=True)
    except Exception as exc:
        raise SystemExit("EDESK_UPDATE_SIGNING_KEY is not valid base64.") from exc
    if len(seed) != 32:
        raise SystemExit("EDESK_UPDATE_SIGNING_KEY must decode to exactly 32 bytes.")

    digest = hashlib.sha256(package.read_bytes()).hexdigest()
    metadata = {
        "schema_version": 1,
        "app": APP_ID,
        "package_id": APP_ID,
        "version": args.version,
        "release_id": args.tag,
        "published_at": datetime.now(timezone.utc).isoformat().replace("+00:00", "Z"),
        "signature_key_id": KEY_ID,
        "artifacts": [
            {
                "platform": "windows",
                "arch": "x86_64",
                "format": "exe",
                "url": (
                    f"https://github.com/{REPOSITORY}/releases/download/"
                    f"{args.tag}/{expected_name}"
                ),
                "file_name": expected_name,
                "size": package.stat().st_size,
                "sha256": digest,
            }
        ],
    }

    args.output_dir.mkdir(parents=True, exist_ok=True)
    metadata_bytes = json.dumps(
        metadata, ensure_ascii=False, separators=(",", ":")
    ).encode("utf-8")
    signature = Ed25519PrivateKey.from_private_bytes(seed).sign(
        SIGNATURE_CONTEXT + metadata_bytes
    )
    signature_document = {
        "schema_version": 1,
        "algorithm": "ed25519",
        "key_id": KEY_ID,
        "signature": base64.b64encode(signature).decode("ascii"),
    }

    (args.output_dir / "edesk-update.json").write_bytes(metadata_bytes)
    (args.output_dir / "edesk-update.json.sig").write_text(
        json.dumps(signature_document, separators=(",", ":")) + "\n",
        encoding="utf-8",
    )
    print(f"Created signed metadata for eDesk {args.version}.")
    print(f"Package size: {package.stat().st_size} bytes")
    print(f"SHA-256: {digest}")


if __name__ == "__main__":
    main()
