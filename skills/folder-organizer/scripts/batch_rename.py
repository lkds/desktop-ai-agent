#!/usr/bin/env python3
"""Batch rename files."""

import argparse
import re
import sys
from pathlib import Path


def main():
    parser = argparse.ArgumentParser(description='Batch rename files')
    parser.add_argument('directory', help='Directory with files')
    parser.add_argument('--pattern', required=True, help='Regex pattern to match')
    parser.add_argument('--replacement', required=True, help='Replacement string')
    parser.add_argument('--dry-run', action='store_true', help='Preview only')
    args = parser.parse_args()

    directory = Path(args.directory)
    if not directory.is_dir():
        print(f"Error: Not a directory: {directory}", file=sys.stderr)
        sys.exit(1)

    renamed = []
    
    for file in directory.iterdir():
        if file.is_file():
            new_name = re.sub(args.pattern, args.replacement, file.name)
            if new_name != file.name:
                new_path = file.parent / new_name
                renamed.append((file.name, new_name))
                
                if not args.dry_run:
                    file.rename(new_path)

    print("=== Rename Summary ===")
    for old, new in renamed:
        print(f"{old} -> {new}")
    
    if args.dry_run:
        print(f"\n(Dry run - {len(renamed)} files would be renamed)")
    else:
        print(f"\nRenamed {len(renamed)} files")


if __name__ == '__main__':
    main()