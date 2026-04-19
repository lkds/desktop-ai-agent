#!/usr/bin/env python3
"""Organize files by type."""

import argparse
import shutil
import sys
from pathlib import Path


# File type mappings
TYPE_MAP = {
    'Images': ['.jpg', '.jpeg', '.png', '.gif', '.bmp', '.svg', '.webp'],
    'Documents': ['.pdf', '.doc', '.docx', '.txt', '.md', '.xlsx', '.pptx'],
    'Archives': ['.zip', '.tar', '.gz', '.rar', '.7z'],
    'Videos': ['.mp4', '.mkv', '.avi', '.mov', '.webm'],
    'Music': ['.mp3', '.wav', '.flac', '.aac', '.ogg'],
    'Code': ['.py', '.js', '.ts', '.java', '.cpp', '.c', '.go', '.rs'],
    'Data': ['.json', '.yaml', '.yml', '.xml', '.csv'],
}


def get_category(ext: str) -> str:
    """Get category for file extension."""
    ext = ext.lower()
    for category, extensions in TYPE_MAP.items():
        if ext in extensions:
            return category
    return 'Other'


def main():
    parser = argparse.ArgumentParser(description='Organize files by type')
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
            category = get_category(file.suffix)
            target_dir = directory / category
            
            if not args.dry_run:
                target_dir.mkdir(exist_ok=True)
                shutil.move(str(file), str(target_dir / file.name))
            
            moved[category] = moved.get(category, 0) + 1

    print("=== Organization Summary ===")
    for category, count in sorted(moved.items()):
        print(f"{category}: {count} files")
    
    if args.dry_run:
        print("\n(Dry run - no files moved)")


if __name__ == '__main__':
    main()