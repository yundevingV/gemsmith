#!/usr/bin/env python3
"""Re-slice assets/source_sheet.png — excludes bottom duplicate token row."""
from PIL import Image
from pathlib import Path

ROOT = Path(__file__).resolve().parents[1] / "assets"
SRC = ROOT / "source_sheet.png"

# Middle band ends above the second token row on the sheet (~y 420+).
Y0, Y1 = 132, 418


def trim(im, tol=248):
    im = im.convert("RGBA")
    w, h = im.size
    px = im.load()
    minx, miny, maxx, maxy = w, h, 0, 0
    for y in range(h):
        for x in range(w):
            r, g, b, a = px[x, y]
            if r < tol or g < tol or b < tol:
                minx = min(minx, x)
                miny = min(miny, y)
                maxx = max(maxx, x)
                maxy = max(maxy, y)
    if maxx <= minx:
        return im
    return im.crop((minx, miny, maxx + 1, maxy + 1))


def token_clusters(im):
    w, _ = im.size
    xs = set()
    for y in range(8, 118):
        for x in range(w):
            if sum(im.getpixel((x, y))[:3]) < 720:
                xs.add(x)
    xs = sorted(xs)
    cur = [xs[0]]
    out = []
    for x in xs[1:]:
        if x - cur[-1] <= 4:
            cur.append(x)
        else:
            out.append((cur[0], cur[-1]))
            cur = [x]
    out.append((cur[0], cur[-1]))
    fixed = []
    for a, b in out:
        if b - a > 90:
            m = (a + b) // 2
            fixed += [(a, m), (m, b)]
        else:
            fixed.append((a, b))
    return fixed


def main():
    im = Image.open(SRC)
    cs = token_clusters(im)
    pick = [0, 1, 2, 3, 5, 9]
    names = ["ruby", "sapphire", "emerald", "diamond", "onyx", "gold"]
    (ROOT / "tokens").mkdir(parents=True, exist_ok=True)
    for i, idx in enumerate(pick):
        a, b = cs[idx]
        trim(im.crop((a - 3, 6, b + 3, 118))).save(ROOT / "tokens" / f"{names[i]}.png")

    boxes = [
        ("cards/frame_t1.png", (16, Y0, 126, Y1)),
        ("cards/frame_t2.png", (131, Y0, 256, Y1)),
        ("cards/frame_t3.png", (261, Y0, 386, Y1)),
        ("patrons/patron_0.png", (391, Y0, 516, Y1)),
        ("patrons/patron_1.png", (521, Y0, 646, Y1)),
        ("patrons/patron_2.png", (651, Y0, 776, Y1)),
        ("cards/back.png", (786, Y0, 1014, Y1)),
    ]
    for rel, box in boxes:
        p = ROOT / rel
        p.parent.mkdir(parents=True, exist_ok=True)
        trim(im.crop(box)).save(p)
    stale = ROOT / "cards" / "frame_t0.png"
    if stale.exists():
        stale.unlink()
    print("sliced ok")


if __name__ == "__main__":
    main()
