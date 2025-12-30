#!/usr/bin/env python3
"""
Generate trigrams for Kyrgyz language from corpus.
Output format is compatible with whatlang-rs data.json
"""

import re
from collections import Counter


def extract_trigrams(text: str, top_n: int = 300) -> str:
    """
    Extract top N trigrams from text.

    Trigrams are extracted from words with space padding, similar to how
    whatlang-rs processes text. For example, word "кыргыз" becomes " кыргыз "
    and produces trigrams: " кы", "кыр", "ырг", "ргы", "гыз", "ыз ".
    """
    # Lowercase the text
    text = text.lower()

    # Remove punctuation and digits, keep letters and spaces
    # Keep Cyrillic letters including Kyrgyz-specific ones: ң, ө, ү
    text = re.sub(r'[^\w\s]', ' ', text)
    text = re.sub(r'\d+', ' ', text)
    text = re.sub(r'\s+', ' ', text)

    trigrams = Counter()

    for word in text.split():
        if len(word) < 1:
            continue
        # Add space padding around word
        word = f' {word} '
        for i in range(len(word) - 2):
            trigram = word[i:i+3]
            # Only count trigrams with at least one letter
            if any(c.isalpha() for c in trigram):
                trigrams[trigram] += 1

    # Get top N trigrams and format as pipe-separated string
    top_trigrams = [t for t, _ in trigrams.most_common(top_n)]
    return '|'.join(top_trigrams)


def main():
    # Read the corpus
    with open('kyrgyz_corpus.txt', 'r', encoding='utf-8') as f:
        text = f.read()

    print(f"Corpus size: {len(text)} characters")

    # Generate trigrams
    trigram_str = extract_trigrams(text, 300)

    # Count how many trigrams we got
    trigrams = trigram_str.split('|')
    print(f"Generated {len(trigrams)} trigrams")

    # Show first 20 trigrams as sample
    print("\nFirst 20 trigrams:")
    for i, t in enumerate(trigrams[:20]):
        print(f"  {i+1}. '{t}'")

    # Save to file
    with open('trigrams.txt', 'w', encoding='utf-8') as f:
        f.write(trigram_str)

    print(f"\nTrigrams saved to trigrams.txt")

    # Also print the full string for easy copy-paste
    print("\n--- Full trigram string for data.json ---")
    print(trigram_str)


if __name__ == '__main__':
    main()
