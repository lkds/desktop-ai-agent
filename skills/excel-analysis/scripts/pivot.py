#!/usr/bin/env python3
"""Create pivot table from Excel data."""

import argparse
import json
import sys
from pathlib import Path

import pandas as pd


def main():
    parser = argparse.ArgumentParser(description='Create pivot table')
    parser.add_argument('input', help='Input Excel file')
    parser.add_argument('--sheet', help='Sheet name')
    parser.add_argument('--index', required=True, help='Index column')
    parser.add_argument('--columns', help='Columns for pivot')
    parser.add_argument('--values', required=True, help='Values column')
    parser.add_argument('--agg', default='sum', 
                        choices=['sum', 'mean', 'count', 'max', 'min'],
                        help='Aggregation function')
    parser.add_argument('--output', help='Output Excel file')
    args = parser.parse_args()

    if not Path(args.input).exists():
        print(f"Error: File not found: {args.input}", file=sys.stderr)
        sys.exit(1)

    df = pd.read_excel(args.input, sheet_name=args.sheet or 0)
    
    pivot = df.pivot_table(
        index=args.index,
        columns=args.columns,
        values=args.values,
        aggfunc=args.agg
    )
    
    print(f"=== Pivot Table ({args.agg} of {args.values} by {args.index}) ===")
    print(pivot.to_string())
    
    if args.output:
        pivot.to_excel(args.output)
        print(f"\nSaved to: {args.output}")


if __name__ == '__main__':
    main()