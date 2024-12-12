## Supported Sprite Sizes

| Size  | Width | Height |
|-------|-------|--------|
| 8x8   | 8     | 8      |
| 16x16 | 16    | 16     |
| 32x32 | 32    | 32     |
| 64x64 | 64    | 64     |
| 16x8  | 16    | 8      |
| 32x8  | 32    | 8      |
| 32x16 | 32    | 16     |
| 64x32 | 64    | 32     |
| 8x16  | 8     | 16     |
| 8x32  | 8     | 32     |
| 16x32 | 16    | 32     |
| 32x64 | 32    | 64     |

## Gameboy Screen Size in pixels

WIDTH = 240 \
HEIGHT = 160

## X/Y

X - Horizontal \
Y - Vertical \
Vector2D(x, y)

## Positioning

| location     | X   | Y   |
|--------------|-----|-----|
| Top Left     | 0   | 0   |
| Top Right    | 240 | 0   |
| Bottom Left  | 0   | 160 |
| Bottom Right | 240 | 160 |

```
______________________________
| 0,0                  240,0 |
|                            |
|                            |
|                            |
|                            |
|0,160                240,160|
------------------------------
```