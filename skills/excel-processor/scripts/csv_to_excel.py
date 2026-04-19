#!/usr/bin/env python3
"""Convert CSV to Excel."""

import argparse
import sys
from pathlib import Path

import pandas as pd


def main():
    parser = argparse.ArgumentParser(description='Convert CSV to Excel')
    parser.add_argument('input', help='Input CSV file')
    parser.add_argument('output', help='Output Excel file')
    parser.add_argument('--sheet', default='Sheet1', help='Sheet name')
    args = parser.parse_args()

    if not Path(args.input).exists():
        print(f"Error: File not found: {args.input}", file=sys.stderr)
        sys.exit(1)

    df = pd.read_csv(args.input)
    df.to_excel(args.output, sheet_name=args.sheet, index=False)
    print(f"Converted: {args.input} -> {args.output} ({len(df)} rows)")


if __.name__ == '__main__':
    main()