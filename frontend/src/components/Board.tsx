import { createMemo, createSignal, For, Show } from "solid-js";
import { TransitionGroup } from "solid-transition-group";
import type { AnimalTileInfo, Direction, Hex, Terrain } from "../types";
import { HEX_SIZE, hexKey, hexPolygonPoints, hexToPixel, pixelToHex } from "../hex";
import { prettySpecies } from "../format";

const TERRAIN_COLORS: Record<Terrain, string> = {
  Forest: "#2e7d32",
  River: "#1976d2",
  Ocean: "#01579b",
  Savanna: "#c9a227",
  Mountain: "#757575",
};

const DIRECTION_SYMBOL: Record<Direction, string> = {
  Rising: "▲",
  Flat: "●",
  Falling: "▼",
};
const DIRECTION_COLOR: Record<Direction, string> = {
  Rising: "#2e7d32",
  Flat: "#9aa1ac",
  Falling: "#c62828",
};

export function Board(props: {
  radius: number;
  terrain: [Hex, Terrain][];
  animals: AnimalTileInfo[];
  armed: boolean;
  spilloverThreshold: number;
  starvationThreshold: number;
  onHexClick: (hex: Hex) => void;
}) {
  let svgRef: SVGSVGElement | undefined;

  // Keyed-by-coordinate lookups so <For> only mounts/unmounts DOM nodes for
  // tiles that actually appeared/disappeared between snapshots, rather than
  // every tile whenever any tile changes.
  const terrainByKey = createMemo(() => {
    const map: Record<string, { hex: Hex; terrain: Terrain }> = {};
    for (const [hex, terrain] of props.terrain) map[hexKey(hex)] = { hex, terrain };
    return map;
  });
  const terrainKeys = createMemo(() => Object.keys(terrainByKey()));

  const animalByKey = createMemo(() => {
    const map: Record<string, AnimalTileInfo> = {};
    for (const a of props.animals) map[hexKey(a.hex)] = a;
    return map;
  });
  const animalKeys = createMemo(() => Object.keys(animalByKey()));

  const [hovered, setHovered] = createSignal<{ info: AnimalTileInfo; x: number; y: number } | null>(null);

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
    <>
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
                <g
                  class="animal-token"
                  transform={`translate(${p().x}, ${p().y})`}
                  onMouseEnter={(ev) => setHovered({ info: entry(), x: ev.clientX, y: ev.clientY })}
                  onMouseMove={(ev) => setHovered({ info: entry(), x: ev.clientX, y: ev.clientY })}
                  onMouseLeave={() => setHovered(null)}
                >
                  <circle
                    r={HEX_SIZE * 0.42}
                    stroke={DIRECTION_COLOR[entry().direction]}
                    stroke-width={HEX_SIZE * 0.09}
                  />
                  <text
                    class="token-letter"
                    text-anchor="middle"
                    dominant-baseline="central"
                    font-size={`${HEX_SIZE * 0.38}`}
                  >
                    {entry().species[0]}
                  </text>
                </g>
              );
            }}
          </For>
        </TransitionGroup>
      </svg>
      <Show when={hovered()}>
        {(hov) => {
          const info = () => hov().info;
          const toSpillover = () => props.spilloverThreshold - info().counter;
          const toStarvation = () => info().counter - props.starvationThreshold;
          return (
            <div class="colony-tooltip" style={{ left: `${hov().x + 16}px`, top: `${hov().y + 16}px` }}>
              <div class="colony-tooltip-title">{prettySpecies(info().species)}</div>
              <div>Colony size: {info().colony_size}</div>
              <div>
                Counter: {info().counter.toFixed(1)}{" "}
                <span style={{ color: DIRECTION_COLOR[info().direction] }}>
                  {DIRECTION_SYMBOL[info().direction]} {info().direction.toLowerCase()}
                </span>
              </div>
              <Show when={toSpillover() > 0} fallback={<div>Spilling over this pass</div>}>
                <div>{toSpillover().toFixed(1)} to next spillover</div>
              </Show>
              <Show when={toStarvation() > 0} fallback={<div>Starving this pass</div>}>
                <div>{toStarvation().toFixed(1)} above starvation</div>
              </Show>
            </div>
          );
        }}
      </Show>
    </>
  );
}
