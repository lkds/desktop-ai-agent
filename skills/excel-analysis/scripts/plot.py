#!/usr/bin/env python3
"""Generate charts from Excel data."""

import argparse
import sys
from pathlib import Path

import matplotlib.pyplot as plt
import pandas as pd


def main():
    parser = argparse.ArgumentParser(description='Generate chart from Excel')
    parser.add_argument('input', help='Input Excel file')
    parser.add_argument('--sheet', help='Sheet name')
    parser.add_argument('--x', required=True, help='X-axis column')
    parser.add_argument('--y', required=True, help='Y-axis column')
    parser.add_argument('--type', default='bar', 
                        choices=['bar', 'line', 'pie', 'scatter'],
                        help='Chart type')
    parser.add_argument('--output', required=True, help='Output image file')
    parser.add_argument('--title', help='Chart title')
    args = parser.parse_args()

    if not Path(args.input).exists():
        print(f"Error: File not found: {args.input}", file=sys.stderr)
        sys.exit(1)

    df = pd.read_excel(args.input, sheet_name=args.sheet or 0)
    
    fig, ax = plt.subplots(figsize=(10, 6))
    
    if args.type == 'bar':
        ax.bar(df[args.x], df[args.y])
    elif args.type == 'line':
        ax.plot(df[args.x], df[args.y], marker='o')
    elif args.type == 'pie':
        ax.pie(df[args.y], labels=df[args.x], autopct='%1.1f%%')
    elif args.type == 'scatter':
        ax.scatter(df[args.x], df[args.y])
    
    ax.set_xlabel(args.x)
    ax.set_ylabel(args.y)
    if args.title:
        ax.set_title(args.title)
    
    plt.tight_layout()
    plt.savefig(args.output, dpi=150)
    plt.close()
    print(f"Chart saved: {args.output}")


if __name__ == '__main__':
    main()