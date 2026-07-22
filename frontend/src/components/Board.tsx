import { createMemo, createSignal, For } from "solid-js";
import { TransitionGroup } from "solid-transition-group";
import type { AnimalTileInfo, Hex, Terrain } from "../types";
import { HEX_SIZE, hexInBounds, hexKey, hexNeighbors, hexPolygonPoints, hexToPixel, pixelToHex, rotateHex } from "../hex";
import { rotation, selectedOption } from "../selection";
import { Colony } from "./Colony";
import { HexTile, TERRAIN_COLORS } from "./HexTile";

export function Board(props: {
  radius: number;
  terrain: [Hex, Terrain][];
  animals: AnimalTileInfo[];
  armed: boolean;
  starvationThreshold: number;
  minMatchingSeams: number;
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
  // the server's can_place_shape exactly: every hex must be in-bounds and
  // empty, and at least `minMatchingSeams` of the piece's hexes must be
  // "seam-safe" (touching no existing terrain, or matching what they
  // touch) — not all of them, so the whole piece shares one valid/invalid
  // verdict rather than each hex judging itself in isolation.
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

      const allInBoundsAndEmpty = placedHexes.every(
        (hex) => hexInBounds(hex, props.radius) && !terrainByHexKey()[hexKey(hex)],
      );
      const seamSafeCount = placedHexes.filter((hex, i) =>
        hexNeighbors(hex).every((n) => {
          const nk = hexKey(n);
          if (newKeys.has(nk)) return true;
          const existing = terrainByHexKey()[nk];
          return !existing || existing === opt.terrains[i];
        }),
      ).length;
      const valid = allInBoundsAndEmpty && seamSafeCount >= props.minMatchingSeams;

      return placedHexes.map((hex, i) => ({ hex, valid, terrain: opt.terrains[i] as Terrain | null }));
    }
    const valid =
      hexInBounds(at, props.radius) && !!terrainByHexKey()[hexKey(at)] && !occupiedAnimal().has(hexKey(at));
    return [{ hex: at, valid, terrain: null }];
  });

  return (
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
            return <HexTile hex={entry().hex} terrain={entry().terrain} />;
          }}
        </For>
      </TransitionGroup>
      <TransitionGroup name="token">
        <For each={animalKeys()}>
          {(key) => {
            const entry = () => animalByKey()[key];
            return <Colony hex={entry().hex} info={entry()} starvationThreshold={props.starvationThreshold} />;
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
  );
}
