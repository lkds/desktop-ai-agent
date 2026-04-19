#!/usr/bin/env python3
"""Read Excel file and output as JSON."""

import argparse
import json
import sys
from pathlib import Path

import pandas as pd


def main():
    parser = argparse.ArgumentParser(description='Read Excel file')
    parser.add_argument('input', help='Input Excel file')
    parser.add_argument('--sheet', help='Sheet name (default: first sheet)')
    parser.add_argument('--format', choices=['json', 'csv'], default='json', help='Output format')
    args = parser.parse_args()

    if not Path(args.input).exists():
        print(f"Error: File not found: {args.input}", file=sys.stderr)
        sys.exit(1)

    df = pd.read_excel(args.input, sheet_name=args.sheet or 0)
    
    if args.format == 'json':
        print(df.to_json(orient='records', force_ascii=False, indent=2))
    else:
        print(df.to_csv(index=False))


if __name__ == '__main__':
    main()