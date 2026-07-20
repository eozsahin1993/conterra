import type { Hex } from "./types";

export const HEX_SIZE = 26;

export function hexKey(h: Hex): string {
  return `${h.q},${h.r}`;
}

// Matches the neighbor-direction convention used on the Rust side
// (`Hex::DIRECTIONS` in src/hex.rs).
const NEIGHBOR_DIRECTIONS: [number, number][] = [
  [1, 0],
  [1, -1],
  [0, -1],
  [-1, 0],
  [-1, 1],
  [0, 1],
];

export function hexNeighbors(h: Hex): Hex[] {
  return NEIGHBOR_DIRECTIONS.map(([dq, dr]) => ({ q: h.q + dq, r: h.r + dr }));
}

// Pointy-top axial -> pixel, matches the neighbor-direction convention used
// on the Rust side (src/hex.rs).
export function hexToPixel(h: Hex, size = HEX_SIZE): { x: number; y: number } {
  return {
    x: size * (Math.sqrt(3) * h.q + (Math.sqrt(3) / 2) * h.r),
    y: size * 1.5 * h.r,
  };
}

export function pixelToHex(x: number, y: number, size = HEX_SIZE): Hex {
  const q = ((Math.sqrt(3) / 3) * x - (1 / 3) * y) / size;
  const r = ((2 / 3) * y) / size;
  return axialRound(q, r);
}

function axialRound(q: number, r: number): Hex {
  const s = -q - r;
  let rq = Math.round(q);
  let rr = Math.round(r);
  const rs = Math.round(s);

  const qDiff = Math.abs(rq - q);
  const rDiff = Math.abs(rr - r);
  const sDiff = Math.abs(rs - s);

  if (qDiff > rDiff && qDiff > sDiff) rq = -rr - rs;
  else if (rDiff > sDiff) rr = -rq - rs;

  return { q: rq, r: rr };
}

export function hexCorners(cx: number, cy: number, size: number): [number, number][] {
  const pts: [number, number][] = [];
  for (let i = 0; i < 6; i++) {
    const angle = (Math.PI / 180) * (60 * i - 30);
    pts.push([cx + size * Math.cos(angle), cy + size * Math.sin(angle)]);
  }
  return pts;
}

export function hexPolygonPoints(cx: number, cy: number, size: number): string {
  return hexCorners(cx, cy, size)
    .map(([x, y]) => `${x.toFixed(2)},${y.toFixed(2)}`)
    .join(" ");
}

// Matches `Hex::spiral_from_origin`'s bounds check on the Rust side.
export function hexInBounds(h: Hex, radius: number): boolean {
  if (h.q < -radius || h.q > radius) return false;
  const r1 = Math.max(-radius, -h.q - radius);
  const r2 = Math.min(radius, -h.q + radius);
  return h.r >= r1 && h.r <= r2;
}

export function rotateHex(h: Hex, steps: number): Hex {
  let { q, r } = h;
  let s = -q - r;
  const n = ((steps % 6) + 6) % 6;
  for (let i = 0; i < n; i++) {
    const nq = -r;
    const nr = -s;
    const ns = -q;
    q = nq;
    r = nr;
    s = ns;
  }
  return { q, r };
}
