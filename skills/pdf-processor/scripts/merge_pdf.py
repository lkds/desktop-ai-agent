#!/usr/bin/env python3
"""Merge multiple PDF files."""

import argparse
import sys
from pathlib import Path

import fitz  # PyMuPDF


def main():
    parser = argparse.ArgumentParser(description='Merge PDF files')
    parser.add_argument('inputs', nargs='+', help='Input PDF files')
    parser.add_argument('--output', required=True, help='Output PDF file')
    args = parser.parse_args()

    merged = fitz.open()
    
    for f in args.inputs:
        if not Path(f).exists():
            print(f"Warning: File not found: {f}", file=sys.stderr)
            continue
        doc = fitz.open(f)
        merged.insert_pdf(doc)
        doc.close()
        print(f"Added: {f}")

    merged.save(args.output)
    merged.close()
    print(f"Merged: {len(args.inputs)} files -> {args.output}")


if __name__ == '__main__':
    main()