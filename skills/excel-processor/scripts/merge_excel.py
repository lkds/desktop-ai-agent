#!/usr/bin/env python3
"""Merge multiple Excel files."""

import argparse
import sys
from pathlib import Path

import pandas as pd


def main():
    parser = argparse.ArgumentParser(description='Merge Excel files')
    parser.add_argument('inputs', nargs='+', help='Input Excel files')
    parser.add_argument('--output', required=True, help='Output Excel file')
    parser.add_argument('--sheet', default='Sheet1', help='Sheet name for output')
    args = parser.parse_args()

    dfs = []
    for f in args.inputs:
        if not Path(f).exists():
            print(f"Warning: File not found: {f}", file=sys.stderr)
            continue
        df = pd.read_excel(f)
        df['_source_file'] = Path(f).name
        dfs.append(df)

    if not dfs:
        print("Error: No valid input files", file=sys.stderr)
        sys.exit(1)

    merged = pd.concat(dfs, ignore_index=True)
    merged.to_excel(args.output, sheet_name=args.sheet, index=False)
    print(f"Merged: {len(dfs)} files -> {args.output} ({len(merged)} rows)")


if __name__ == '__main__':
    main()