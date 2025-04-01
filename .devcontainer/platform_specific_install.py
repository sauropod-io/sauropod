#!/usr/bin/env python3
"""Fetch a file based on the host systems platform."""

import sys
import platform
import argparse
import urllib.request


def download_url(url: str) -> None:
    """Download a file from a URL."""
    urllib.request.urlretrieve(url, "/dev/stdout")


def main() -> None:
    """CLI entrypoint."""
    argparser = argparse.ArgumentParser()
    # Set up mandatory options for different platforms
    argparser.add_argument("--x86_64", required=True, help="URL for x86_64")
    argparser.add_argument("--aarch64", required=True, help="URL for aarch64")
    args = argparser.parse_args()

    machine = platform.machine()
    match machine:
        case "x86_64":
            print("Fetching x86_64 file", file=sys.stderr)
            download_url(args.x86_64)
        case "arm64":
            print("Fetching aarch64 file", file=sys.stderr)
            download_url(args.aarch64)
        case "":
            print("Could not determine host machine", file=sys.stderr)
            sys.exit(1)
        case _:
            print(f"Unsupported platform {machine}", file=sys.stderr)
            sys.exit(1)


if __name__ == "__main__":
    main()
