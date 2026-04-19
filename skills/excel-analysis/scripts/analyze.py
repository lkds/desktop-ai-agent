#!/usr/bin/env python3
"""Analyze Excel data and show statistics."""

import argparse
import sys
from pathlib import Path

import pandas as pd


def main():
    parser = argparse.ArgumentParser(description='Analyze Excel data')
    parser.add_argument('input', help='Input Excel file')
    parser.add_argument('--sheet', help='Sheet name')
    args = parser.parse_args()

    if not Path(args.input).exists():
        print(f"Error: File not found: {args.input}", file=sys.stderr)
        sys.exit(1)

    df = pd.read_excel(args.input, sheet_name=args.sheet or 0)
    
    print(f"=== {args.input} ===")
    print(f"Shape: {df.shape[0]} rows x {df.shape[1]} columns")
    print(f"\nColumns: {list(df.columns)}")
    print(f"\nData types:\n{df.dtypes}")
    print(f"\n=== Statistics ===")
    print(df.describe(include='all').to_string())


if __name__ == '__main__':
    main()