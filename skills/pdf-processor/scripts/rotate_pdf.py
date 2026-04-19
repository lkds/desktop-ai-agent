#!/usr/bin/env python3
"""Rotate PDF pages."""

import argparse
import sys
from pathlib import Path

import fitz  # PyMuPDF


def main():
    parser = argparse.ArgumentParser(description='Rotate PDF')
    parser.add_argument('input', help='Input PDF file')
    parser.add_argument('output', help='Output PDF file')
    parser.add_argument('--degrees', type=int, default=90, choices=[90, 180, 270],
                        help='Rotation degrees')
    args = parser.parse_args()

    if not Path(args.input).exists():
        print(f"Error: File not found: {args.input}", file=sys.stderr)
        sys.exit(1)

    doc = fitz.open(args.input)
    
    for page in doc:
        page.set_rotation((page.rotation + args.degrees) % 360)

    doc.save(args.output)
    doc.close()
    print(f"Rotated: {args.input} -> {args.output} ({args.degrees}°)")


if __name__ == '__main__':
    main()