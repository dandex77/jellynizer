#!/bin/bash
# This script creates a specific directory structure and empty files in test_media/ for testing purposes.

BASE_DIR="test_media"

# Create base directory
mkdir -p "$BASE_DIR"

echo "Creating directory structure in $BASE_DIR..."

# 1. The Capture (2019) Season 2
DIR1="$BASE_DIR/The Capture (2019) Season 2 S02 (1080p AMZN WEB-DL x265 HEVC 10bit EAC3 5.1 Ghost)"
mkdir -p "$DIR1"
touch "$DIR1/The Capture (2019) - S02E03 - Charlie Foxtrot (1080p AMZN WEB-DL x265 Ghost).mkv"
touch "$DIR1/The Capture (2019) - S02E02 - Made in China (1080p AMZN WEB-DL x265 Ghost).mkv"
touch "$DIR1/The Capture (2019) - S02E01 - Invisible Men (1080p AMZN WEB-DL x265 Ghost).mkv"
touch "$DIR1/The Capture (2019) - S02E06 - The Flip (1080p AMZN WEB-DL x265 Ghost).mkv"
touch "$DIR1/The Capture (2019) - S02E05 - Imposter Syndrome (1080p AMZN WEB-DL x265 Ghost).mkv"
touch "$DIR1/The Capture (2019) - S02E04 - #therealzacturner (1080p AMZN WEB-DL x265 Ghost).mkv"

# 2. The Capture (2019) Season 3
DIR2="$BASE_DIR/The Capture (2019) Season 3 S03 (1080p AMZN WEB-DL x265 HEVC 10bit DDP 5.1 Vyndros)"
mkdir -p "$DIR2"
touch "$DIR2/The.Capture.S03E05.Simon.Says.1080p.10bit.AMZN.WEB-DL.DDP5.1.HEVC-Vyndros.mkv"
touch "$DIR2/The.Capture.S03E01.Don’t.Look.at.the.Camera.1080p.10bit.AMZN.WEB-DL.DDP5.1.HEVC-Vyndros.mkv"
touch "$DIR2/The.Capture.S03E04.Kill.Switch.1080p.10bit.AMZN.WEB-DL.DDP5.1.HEVC-Vyndros.mkv"
touch "$DIR2/The.Capture.S03E03.The.Scarecrow.1080p.10bit.AMZN.WEB-DL.DDP5.1.HEVC-Vyndros.mkv"
touch "$DIR2/The.Capture.S03E02.Lone.Wolves.1080p.10bit.AMZN.WEB-DL.DDP5.1.HEVC-Vyndros.mkv"
touch "$DIR2/The.Capture.S03E06.Someone.to.Watch.Over.Me.1080p.10bit.AMZN.WEB-DL.DDP5.1.HEVC-Vyndros.mkv"

# 3. The Capture (2019) Season 1
DIR3="$BASE_DIR/The Capture (2019) Season 1 S01 (1080p AMZN WEB-DL x265 HEVC 10bit DDP 5.1 Vyndros)"
mkdir -p "$DIR3"
touch "$DIR3/The.Capture.S01E05.A.Pilgrim.of.Justice.1080p.10bit.AMZN.WEB-DL.DDP5.1.HEVC-Vyndros.mkv"
touch "$DIR3/The.Capture.S01E06.Correction.1080p.10bit.AMZN.WEB-DL.DDP5.1.HEVC-Vyndros.mkv"
touch "$DIR3/The.Capture.S01E04.Blind.Spots.1080p.10bit.AMZN.WEB-DL.DDP5.1.HEVC-Vyndros.mkv"
touch "$DIR3/The.Capture.S01E03.Truffle.Hog.1080p.10bit.AMZN.WEB-DL.DDP5.1.HEVC-Vyndros.mkv"
touch "$DIR3/The.Capture.S01E02.Toy.Soldier.1080p.10bit.AMZN.WEB-DL.DDP5.1.HEVC-Vyndros.mkv"
touch "$DIR3/The.Capture.S01E01.What.Happens.in.Helmand.1080p.10bit.AMZN.WEB-DL.DDP5.1.HEVC-Vyndros.mkv"

# 4. Euphoria.US.S03
DIR4="$BASE_DIR/Euphoria.US.S03.1080p.WEBRip.10Bit.DDP5.1.x265-NeoNoir"
mkdir -p "$DIR4"
touch "$DIR4/Euphoria.US.S03E08.In.God.We.Trust.1080p.WEBRip.10Bit.DDP5.1.x265-NeoNoir.mkv"
touch "$DIR4/Euphoria.US.S03E07.Rain.or.Shine.1080p.WEBRip.10Bit.DDP5.1.x265-NeoNoir.mkv"
touch "$DIR4/Euphoria.US.S03E06.Stand.Still.and.See.1080p.WEBRip.10Bit.DDP5.1.x265-NeoNoir.mkv"
touch "$DIR4/Euphoria.US.S03E05.This.Little.Piggy.1080p.WEBRip.10Bit.DDP5.1.x265-NeoNoir.mkv"
touch "$DIR4/Euphoria.US.S03E04.Kitty.Likes.to.Dance.1080p.WEBRip.10Bit.DDP5.1.x265-NeoNoir.mkv"
touch "$DIR4/Euphoria.US.S03E03.The.Ballad.of.Paladin.1080p.WEBRip.10Bit.DDP5.1.x265-NeoNoir.mkv"
touch "$DIR4/Euphoria.US.S03E02.America.My.Dream.1080p.WEBRip.10Bit.DDP5.1.x265-NeoNoir.mkv"
touch "$DIR4/Euphoria.US.S03E01.Ándale.1080p.WEBRip.10Bit.DDP5.1.x265-NeoNoir.mkv"

# 5. The Thing With Feathers (2025)
DIR5="$BASE_DIR/The Thing With Feathers (2025) [720p] [WEBRip] [YTS.LT]"
mkdir -p "$DIR5/Subs"
touch "$DIR5/The.Thing.With.Feathers.2025.720p.WEBRip.x264.AAC-[YTS.LT].mp4"
touch "$DIR5/The.Thing.With.Feathers.2025.720p.WEBRip.x264.AAC-[YTS.LT].srt"
touch "$DIR5/www.YTS.LT.jpg"
touch "$DIR5/YTSProxies.com.txt"
touch "$DIR5/Subs/English.srt"
touch "$DIR5/Subs/Latin American.spa.srt"
touch "$DIR5/Subs/SDH.eng.HI.srt"

# 6. Pokemon The First Movie
DIR6="$BASE_DIR/Pokemon The First Movie - Mewtwo Strikes Back (1998) [DUBBED] [REPACK] [1080p] [BluRay] [5.1] [YTS.MX]"
mkdir -p "$DIR6"
touch "$DIR6/Pokemon.The.First.Movie.-.Mewtwo.Strikes.Back.1998.DUBBED.REPACK.1080p.BluRay.x264.AAC5.1-[YTS.MX].mp4"
touch "$DIR6/Pokemon.The.First.Movie.-.Mewtwo.Strikes.Back.1998.DUBBED.REPACK.1080p.BluRay.x264.AAC5.1-[YTS.MX].srt"
touch "$DIR6/www.YTS.MX.jpg"
touch "$DIR6/YTSYifyUP... (TOR).txt"

# 7. Spider-Man Far From Home
DIR7="$BASE_DIR/Spider-Man Far From Home (2019) [BluRay] [1080p] [YTS.LT]"
mkdir -p "$DIR7"
touch "$DIR7/Spider-Man.Far.From.Home.2019.1080p.BluRay.x264-[YTS.LT].mp4"
touch "$DIR7/www.YTS.LT.jpg"

# 8. Spider-Man Homecoming
DIR8="$BASE_DIR/Spider-Man Homecoming (2017) [1080p] [YTS.AG]"
mkdir -p "$DIR8"
touch "$DIR8/Spider-Man.Homecoming.2017.1080p.BluRay.x264-[YTS.AG].mp4"
touch "$DIR8/WWW.YTS.AG.jpg"

# 9. Spider-Man No Way Home
DIR9="$BASE_DIR/Spider-Man No Way Home (2021) [1080p] [BluRay] [5.1] [YTS.MX]"
mkdir -p "$DIR9"
touch "$DIR9/Spider-Man.No.Way.Home.2021.1080p.BluRay.x264.AAC.5.1-[YTS.MX].mp4"
touch "$DIR9/www.YTS.MX.jpg"
touch "$DIR9/YTSProxies.com.txt"

# 10. Now You See Me
DIR10="$BASE_DIR/Now You See Me (2013) [1080p]"
mkdir -p "$DIR10"
touch "$DIR10/Now.You.See.Me.2013.1080p.BluRay.x264.YIFY.mp4"
touch "$DIR10/WWW.YIFY-TORRENTS.COM.jpg"

# 11. Perfect Days
DIR11="$BASE_DIR/Perfect.Days.2023.JAPANESE.1080p.AMZN.WEBRip.DDP5.1.x265.10bit-GalaxyRG265[TGx]"
mkdir -p "$DIR11"
touch "$DIR11/Perfect.Days.2023.JAPANESE.1080p.AMZN.WEBRip.DDP5.1.x265.10bit-GalaxyRG265.mkv"
touch "$DIR11/source.txt"
touch "$DIR11/[TGx]Downloaded from torrentgalaxy.to .txt"
touch "$DIR11/NEW upcoming releases by Xclusive.txt"

# 12. The Nice Guys
DIR12="$BASE_DIR/The Nice Guys (2016) [1080p] [YTS.AG]"
mkdir -p "$DIR12"
touch "$DIR12/The.Nice.Guys.2016.1080p.BluRay.x264-[YTS.AG].mp4"
touch "$DIR12/WWW.YTS.AG.jpg"

# 13. Mission Impossible
DIR13="$BASE_DIR/Mission Impossible The Final Reckoning 2025"
mkdir -p "$DIR13"
touch "$DIR13/Mission Impossible The Final Reckoning 2025 IMAX 1080p WEB-DL HEVC x265-RMTeam.mkv"

echo "Done!"
