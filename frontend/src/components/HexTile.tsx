import type { Hex, Terrain } from "../types";
import { HEX_SIZE, hexPolygonPoints, hexToPixel } from "../hex";

export const TERRAIN_COLORS: Record<Terrain, string> = {
  Forest: "#2e7d32",
  River: "#1976d2",
  Ocean: "#01579b",
  Savanna: "#c9a227",
  Mountain: "#757575",
};

export function HexTile(props: { hex: Hex; terrain: Terrain }) {
  const p = () => hexToPixel(props.hex);
  return (
    <polygon class="hex-tile" points={hexPolygonPoints(p().x, p().y, HEX_SIZE)} fill={TERRAIN_COLORS[props.terrain]} />
  );
}
