#!/usr/bin/env python3
"""Organize files by date."""

import argparse
import shutil
import sys
from datetime import datetime
from pathlib import Path


def main():
    parser = argparse.ArgumentParser(description='Organize files by date')
    parser.add_argument('directory', help='Directory to organize')
    parser.add_argument('--dry-run', action='store_true', help='Preview only')
    args = parser.parse_args()

    directory = Path(args.directory)
    if not directory.is_dir():
        print(f"Error: Not a directory: {directory}", file=sys.stderr)
        sys.exit(1)

    moved = {}
    
    for file in directory.iterdir():
        if file.is_file():
            mtime = datetime.fromtimestamp(file.stat().st_mtime)
            target_dir = directory / f"{mtime.year}" / f"{mtime.month:02d}"
            
            if not args.dry_run:
                target_dir.mkdir(parents=True, exist_ok=True)
                shutil.move(str(file), str(target_dir / file.name))
            
            key = f"{mtime.year}/{mtime.month:02d}"
            moved[key] = moved.get(key, 0) + 1

    print("=== Organization Summary ===")
    for month, count in sorted(moved.items()):
        print(f"{month}: {count} files")
    
    if args.dry_run:
        print("\n(Dry run - no files moved)")


if __name__ == '__main__':
    main()