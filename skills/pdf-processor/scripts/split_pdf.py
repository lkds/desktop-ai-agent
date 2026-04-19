#!/usr/bin/env python3
"""Split PDF by page ranges."""

import argparse
import re
import sys
from pathlib import Path

import fitz  # PyMuPDF


def parse_ranges(range_str: str) -> list[tuple[int, int]]:
    """Parse page ranges like '1-5,10-15' into list of (start, end) tuples."""
    ranges = []
    for part in range_str.split(','):
        part = part.strip()
        if '-' in part:
            start, end = part.split('-')
            ranges.append((int(start) - 1, int(end)))  # Convert to 0-indexed
        else:
            page = int(part) - 1  # Convert to 0-indexed
            ranges.append((page, page + 1))
    return ranges


def main():
    parser = argparse.ArgumentParser(description='Split PDF')
    parser.add_argument('input', help='Input PDF file')
    parser.add_argument('--pages', required=True, help='Page ranges (e.g., 1-5,10-15)')
    parser.add_argument('--output', required=True, help='Output directory')
    args = parser.parse_args()

    if not Path(args.input).exists():
        print(f"Error: File not found: {args.input}", file=sys.stderr)
        sys.exit(1)

    Path(args.output).mkdir(parents=True, exist_ok=True)
    doc = fitz.open(args.input)
    ranges = parse_ranges(args.pages)
    base_name = Path(args.input).stem

    for i, (start, end) in enumerate(ranges, 1):
        new_doc = fitz.open()
        new_doc.insert_pdf(doc, from_page=start, to_page=end - 1)
        output_path = Path(args.output) / f"{base_name}_part{i}.pdf"
        new_doc.save(str(output_path))
        new_doc.close()
        print(f"Created: {output_path} (pages {start + 1}-{end})")

    doc.close()
    print(f"Split into {len(ranges)} files")


if __name__ == '__main__':
    main()