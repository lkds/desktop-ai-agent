#!/usr/bin/env python3
"""Write Excel file from JSON data."""

import argparse
import json
import sys
from pathlib import Path

import pandas as pd


def main():
    parser = argparse.ArgumentParser(description='Write Excel file')
    parser.add_argument('--data', required=True, help='JSON data')
    parser.add_argument('--output', required=True, help='Output Excel file')
    parser.add_argument('--sheet', default='Sheet1', help='Sheet name')
    args = parser.parse_args()

    try:
        data = json.loads(args.data)
    except json.JSONDecodeError as e:
        print(f"Error: Invalid JSON: {e}", file=sys.stderr)
        sys.exit(1)

    df = pd.DataFrame(data)
    df.to_excel(args.output, sheet_name=args.sheet, index=False)
    print(f"Written: {args.output} ({len(df)} rows)")


if __name__ == '__main__':
    main()