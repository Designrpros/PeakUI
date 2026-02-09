#!/usr/bin/env python3
"""
Convert markdown to PDF using markdown and pdfkit (or reportlab as fallback)
"""

import sys
import subprocess

# Read the markdown file
with open('pitch.md', 'r', encoding='utf-8') as f:
    markdown_content = f.read()

# Use pandoc with chromium/chrome for PDF generation
try:
    # Try using pandoc with context (ConTeXt) - usually available
    result = subprocess.run(
        ['pandoc', 'pitch.md', '-o', 'pitch.pdf', '--pdf-engine=context'],
        capture_output=True,
        text=True
    )
    
    if result.returncode != 0:
        print(f"ConTeXt failed: {result.stderr}")
        raise Exception("ConTeXt not available")
    
    print("✓ PDF created successfully using pandoc with ConTeXt!")
    print("  File: pitch.pdf")
    
except Exception as e:
    print(f"ConTeXt approach failed: {e}")
    
    # Fallback: Open the HTML in browser for manual export
    print("\n" + "="*60)
    print("Alternative: Opening HTML in browser for Print to PDF...")
    print("="*60)
    print("\nYou can:")
    print("1. Press Cmd+P (or File > Print)")
    print("2. Select 'Save as PDF' from the destination dropdown")
    print("3. Save the file as 'pitch.pdf'")
    
    subprocess.run(['open', 'pitch.html'])
    sys.exit(0)
