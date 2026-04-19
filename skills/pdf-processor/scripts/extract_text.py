#!/usr/bin/env python3
"""Extract text from PDF."""

import argparse
import sys
from pathlib import Path

import fitz  # PyMuPDF


def main():
    parser = argparse.ArgumentParser(description='Extract text from PDF')
    parser.add_argument('input', help='Input PDF file')
    parser.add_argument('--output', help='Output text file (default: stdout)')
    args = parser.parse_args()

    if not Path(args.input).exists():
        print(f"Error: File not found: {args.input}", file=sys.stderr)
        sys.exit(1)

    doc = fitz.open(args.input)
    text_parts = []
    
    for page_num, page in enumerate(doc, 1):
        text = page.get_text()
        text_parts.append(f"--- Page {page_num} ---\n{text}")

    full_text = "\n".join(text_parts)
    doc.close()

    if args.output:
        Path(args.output).write_text(full_text, encoding='utf-8')
        print(f"Extracted: {args.input} -> {args.output}")
    else:
        print(full_text)


if __name__ == '__main__':
    main()