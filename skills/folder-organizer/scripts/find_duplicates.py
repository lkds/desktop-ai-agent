#!/usr/bin/env python3
"""Find duplicate files by content hash."""

import argparse
import hashlib
import sys
from collections import defaultdict
from pathlib import Path


def get_file_hash(filepath: Path) -> str:
    """Calculate MD5 hash of file."""
    hasher = hashlib.md5()
    with open(filepath, 'rb') as f:
        for chunk in iter(lambda: f.read(8192), b''):
            hasher.update(chunk)
    return hasher.hexdigest()


def main():
    parser = argparse.ArgumentParser(description='Find duplicate files')
    parser.add_argument('directory', help='Directory to scan')
    parser.add_argument('--min-size', type=int, default=0, help='Minimum file size in bytes')
    args = parser.parse_args()

    directory = Path(args.directory)
    if not directory.is_dir():
        print(f"Error: Not a directory: {directory}", file=sys.stderr)
        sys.exit(1)

    hash_map = defaultdict(list)
    
    for file in directory.rglob('*'):
        if file.is_file():
            if file.stat().st_size >= args.min_size:
                file_hash = get_file_hash(file)
                hash_map[file_hash].append(file)

    duplicates = {h: files for h, files in hash_map.items() if len(files) > 1}

    if not duplicates:
        print("No duplicates found.")
        return

    print("=== Duplicate Files ===")
    total_wasted = 0
    
    for file_hash, files in duplicates.items():
        print(f"\nHash: {file_hash[:16]}...")
        for f in files:
            size = f.stat().st_size
            print(f"  {f} ({size} bytes)")
        total_wasted += files[0].stat().st_size * (len(files) - 1)

    print(f"\nTotal wasted space: {total_wasted / 1024 / 1024:.2f} MB")


if __name__ == '__main__':
    main()