import re

def clean_name(name):
    cleaned = name.replace('.', ' ').replace('_', ' ')
    parts = cleaned.split()
    return " ".join(parts)

def extract_title_from_parent(parent_name):
    year_re = re.compile(r"(?:19|20)\d{2}")
    
    mat = year_re.search(parent_name)
    if mat:
        year = int(mat.group())
        title_part = parent_name[:mat.start()]
        title = clean_name(title_part.strip())
        if title:
            return (title, year)
    
    return (clean_name(parent_name), None)

test_cases = [
    "The Capture (2019) Season 2 S02 (1080p AMZN WEB-DL x265 HEVC 10bit EAC3 5.1 Ghost)",
    "The Capture (2019) Season 1 S01 (1080p AMZN WEB-DL x265 HEVC 10bit DDP 5.1 Vyndros)",
    "The Capture (2019) Season 3 S03 (1080p AMZN WEB-DL x265 HEVC 10bit DDP 5.1 Vyndros)",
    "Euphoria.US.S03.1080p.WEBRip.10Bit.DDP5.1.x265-NeoNoir",
    "The Thing With Feathers (2025) [720p] [WEBRip] [YTS.LT]",
]

for tc in test_cases:
    print(f"Input: {tc}")
    print(f"Result: {extract_title_from_parent(tc)}")
    print("-" * 20)
