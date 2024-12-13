# ect-summary

Usage:

```sh
~/Pictures % fd -e png -x ect --strict -9 {} | ect-summary
Total saved: 15369.70KB
```

## Motivation

I often use [ect](https://github.com/fhanau/Efficient-Compression-Tool) to optimize pictures losslessly. I use it through [fd](https://github.com/sharkdp/fd). I like to see at the end how much I saved.

## Scope creep

- Add percentage of saving
- Statistics?