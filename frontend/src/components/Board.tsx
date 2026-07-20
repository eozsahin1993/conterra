import { createMemo, createSignal, For, Show } from "solid-js";
import { TransitionGroup } from "solid-transition-group";
import type { AnimalTileInfo, Direction, Hex, Terrain } from "../types";
import { HEX_SIZE, hexInBounds, hexKey, hexNeighbors, hexPolygonPoints, hexToPixel, pixelToHex, rotateHex } from "../hex";
import { prettySpecies } from "../format";
import { rotation, selectedOption } from "../selection";

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
  const [hoverHex, setHoverHex] = createSignal<Hex | null>(null);

  const margin = HEX_SIZE * 2;
  const w = 2 * (HEX_SIZE * Math.sqrt(3) * props.radius + margin);
  const h = 2 * (HEX_SIZE * 1.5 * props.radius + margin);

  function eventToHex(ev: MouseEvent): Hex | null {
    if (!svgRef) return null;
    const pt = svgRef.createSVGPoint();
    pt.x = ev.clientX;
    pt.y = ev.clientY;
    const ctm = svgRef.getScreenCTM();
    if (!ctm) return null;
    const local = pt.matrixTransform(ctm.inverse());
    return pixelToHex(local.x, local.y);
  }

  function handleClick(ev: MouseEvent) {
    if (!props.armed) return;
    const hex = eventToHex(ev);
    if (hex) props.onHexClick(hex);
  }

  function handleBoardMouseMove(ev: MouseEvent) {
    if (!props.armed) return;
    setHoverHex(eventToHex(ev));
  }

  const terrainByHexKey = createMemo(() => {
    const map: Record<string, Terrain> = {};
    for (const [hex, terrain] of props.terrain) map[hexKey(hex)] = terrain;
    return map;
  });
  const occupiedAnimal = createMemo(() => new Set(props.animals.map((a) => hexKey(a.hex))));

  // The armed piece previewed at the hovered hex, so placements can be
  // judged before clicking instead of only failing after the fact. Mirrors
  // the server's can_place_shape: in-bounds, empty, and — wherever the
  // piece touches already-placed terrain — matching it (mixing is only
  // allowed within the new piece itself).
  const preview = createMemo(() => {
    const opt = selectedOption();
    const at = hoverHex();
    if (!opt || !at || !props.armed) return [];
    if (opt.type === "TerrainShape") {
      const placedHexes = opt.offsets.map((o) => {
        const r = rotateHex(o, rotation());
        return { q: at.q + r.q, r: at.r + r.r };
      });
      const newKeys = new Set(placedHexes.map(hexKey));
      return placedHexes.map((hex, i) => {
        const empty = !terrainByHexKey()[hexKey(hex)];
        const seamsMatch = hexNeighbors(hex).every((n) => {
          const nk = hexKey(n);
          if (newKeys.has(nk)) return true;
          const existing = terrainByHexKey()[nk];
          return !existing || existing === opt.terrains[i];
        });
        const valid = hexInBounds(hex, props.radius) && empty && seamsMatch;
        return { hex, valid, terrain: opt.terrains[i] as Terrain | null };
      });
    }
    const valid =
      hexInBounds(at, props.radius) && !!terrainByHexKey()[hexKey(at)] && !occupiedAnimal().has(hexKey(at));
    return [{ hex: at, valid, terrain: null }];
  });

  return (
    <>
      <svg
        ref={svgRef}
        viewBox={`${-w / 2} ${-h / 2} ${w} ${h}`}
        class="board-svg"
        classList={{ armed: props.armed }}
        onClick={handleClick}
        onMouseMove={handleBoardMouseMove}
        onMouseLeave={() => setHoverHex(null)}
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
        <g class="placement-preview">
          <For each={preview()}>
            {(cell) => {
              const p = () => hexToPixel(cell.hex);
              return (
                <polygon
                  classList={{ "preview-valid": cell.valid, "preview-invalid": !cell.valid }}
                  points={hexPolygonPoints(p().x, p().y, HEX_SIZE)}
                  fill={cell.terrain ? TERRAIN_COLORS[cell.terrain] : "none"}
                />
              );
            }}
          </For>
        </g>
      </svg>
      <Show when={hovered()}>
        {(hov) => {
          const info = () => hov().info;
          return (
            <div class="colony-tooltip" style={{ left: `${hov().x + 16}px`, top: `${hov().y + 16}px` }}>
              <div class="colony-tooltip-title">{prettySpecies(info().species)}</div>
              <div>Population: {info().counter.toFixed(1)}</div>
              <div>Colony size: {info().colony_size} tiles</div>
              <div>
                Growth rate:{" "}
                <span style={{ color: DIRECTION_COLOR[info().direction] }}>
                  {info().rate >= 0 ? "+" : ""}
                  {info().rate.toFixed(2)}/turn {DIRECTION_SYMBOL[info().direction]}
                </span>
              </div>
              <div class="colony-tooltip-factors">
                Open: {info().open_adjacent} · Predators: {info().predator_adjacent} · Prey: {info().prey_adjacent}
                <Show when={info().contending_adjacent > 0}> · Contending: {info().contending_adjacent}</Show>
              </div>
              <div>Next expansion at population {props.spilloverThreshold}</div>
              <div>Starves at population {props.starvationThreshold}</div>
            </div>
          );
        }}
      </Show>
    </>
  );
}
