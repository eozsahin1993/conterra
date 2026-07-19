import { createMemo, For } from "solid-js";
import { TransitionGroup } from "solid-transition-group";
import type { Hex, Species, Terrain } from "../types";
import { HEX_SIZE, hexKey, hexPolygonPoints, hexToPixel, pixelToHex } from "../hex";
import { prettySpecies } from "../format";

const TERRAIN_COLORS: Record<Terrain, string> = {
  Forest: "#2e7d32",
  River: "#1976d2",
  Ocean: "#01579b",
  Savanna: "#c9a227",
  Mountain: "#757575",
};

export function Board(props: {
  radius: number;
  terrain: [Hex, Terrain][];
  animals: [Hex, Species][];
  armed: boolean;
  onHexClick: (hex: Hex) => void;
}) {
  let svgRef: SVGSVGElement | undefined;

  // Keyed-by-coordinate lookups so <For> only mounts/unmounts DOM nodes for
  // tiles that actually appeared/disappeared between snapshots, rather than
  // every tile whenever any tile changes (each snapshot deserializes fresh
  // objects, so naive reference-based keying would replay the pop-in
  // transition on the entire board every turn instead of just what's new).
  const terrainByKey = createMemo(() => {
    const map: Record<string, { hex: Hex; terrain: Terrain }> = {};
    for (const [hex, terrain] of props.terrain) map[hexKey(hex)] = { hex, terrain };
    return map;
  });
  const terrainKeys = createMemo(() => Object.keys(terrainByKey()));

  const animalByKey = createMemo(() => {
    const map: Record<string, { hex: Hex; species: Species }> = {};
    for (const [hex, species] of props.animals) map[hexKey(hex)] = { hex, species };
    return map;
  });
  const animalKeys = createMemo(() => Object.keys(animalByKey()));

  const margin = HEX_SIZE * 2;
  const w = 2 * (HEX_SIZE * Math.sqrt(3) * props.radius + margin);
  const h = 2 * (HEX_SIZE * 1.5 * props.radius + margin);

  function handleClick(ev: MouseEvent) {
    if (!props.armed || !svgRef) return;
    const pt = svgRef.createSVGPoint();
    pt.x = ev.clientX;
    pt.y = ev.clientY;
    const ctm = svgRef.getScreenCTM();
    if (!ctm) return;
    const local = pt.matrixTransform(ctm.inverse());
    props.onHexClick(pixelToHex(local.x, local.y));
  }

  return (
    <svg
      ref={svgRef}
      viewBox={`${-w / 2} ${-h / 2} ${w} ${h}`}
      class="board-svg"
      classList={{ armed: props.armed }}
      onClick={handleClick}
    >
      <TransitionGroup name="tile">
        <For each={terrainKeys()}>
          {(key) => {
            const entry = () => terrainByKey()[key];
            const p = () => hexToPixel(entry().hex);
            return (
              <polygon
                class="hex-tile"
                points={hexPolygonPoints(p().x, p().y, HEX_SIZE)}
                fill={TERRAIN_COLORS[entry().terrain]}
              />
            );
          }}
        </For>
      </TransitionGroup>
      <TransitionGroup name="token">
        <For each={animalKeys()}>
          {(key) => {
            const entry = () => animalByKey()[key];
            const p = () => hexToPixel(entry().hex);
            return (
              <g class="animal-token" transform={`translate(${p().x}, ${p().y})`}>
                <circle r={HEX_SIZE * 0.42} />
                <text text-anchor="middle" dominant-baseline="central" font-size={`${HEX_SIZE * 0.42}`}>
                  {entry().species[0]}
                </text>
                <title>{prettySpecies(entry().species)}</title>
              </g>
            );
          }}
        </For>
      </TransitionGroup>
    </svg>
  );
}
