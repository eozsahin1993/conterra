import { createMemo, For, Show } from "solid-js";
import { TransitionGroup } from "solid-transition-group";
import type { MarketOption } from "../types";
import { hexPolygonPoints, hexToPixel, rotateHex } from "../hex";
import { prettySpecies } from "../format";
import { armOption, clearSelection, rotateSelection, rotation, selectedOption } from "../selection";
import { shuffleMarket } from "../store";

const PREVIEW_HEX_SIZE = 11;
const TERRAIN_COLORS: Record<string, string> = {
  Forest: "#2e7d32",
  River: "#1976d2",
  Ocean: "#01579b",
  Savanna: "#c9a227",
  Mountain: "#757575",
};

function ShapePreview(props: { opt: Extract<MarketOption, { type: "TerrainShape" }>; rotationSteps: number }) {
  const cells = createMemo(() =>
    props.opt.offsets.map((o, i) => ({
      p: hexToPixel(rotateHex(o, props.rotationSteps), PREVIEW_HEX_SIZE),
      terrain: props.opt.terrains[i],
    })),
  );
  return (
    <svg viewBox="-45 -45 90 90" class="shape-preview">
      <For each={cells()}>
        {(cell) => (
          <polygon points={hexPolygonPoints(cell.p.x, cell.p.y, PREVIEW_HEX_SIZE)} fill={TERRAIN_COLORS[cell.terrain]} />
        )}
      </For>
    </svg>
  );
}

function distinctTerrains(opt: Extract<MarketOption, { type: "TerrainShape" }>): string {
  return Array.from(new Set(opt.terrains)).join(" + ");
}

function MarketCard(props: { opt: MarketOption; isMyTurn: boolean }) {
  const isSelected = createMemo(() => selectedOption()?.id === props.opt.id);

  return (
    <div
      class="market-card"
      classList={{ selected: isSelected(), disabled: !props.isMyTurn }}
      onClick={() => props.isMyTurn && armOption(props.opt)}
    >
      <Show
        when={props.opt.type === "TerrainShape"}
        fallback={<div class="card-label">Place {prettySpecies((props.opt as any).species)}</div>}
      >
        <div class="card-label">{distinctTerrains(props.opt as Extract<MarketOption, { type: "TerrainShape" }>)}</div>
        <ShapePreview
          opt={props.opt as Extract<MarketOption, { type: "TerrainShape" }>}
          rotationSteps={isSelected() ? rotation() : 0}
        />
      </Show>
      <Show when={isSelected() && props.opt.type === "TerrainShape"}>
        <div class="rotate-row">
          <button
            onClick={(e) => {
              e.stopPropagation();
              rotateSelection();
            }}
          >
            rotate ↻
          </button>
        </div>
      </Show>
    </div>
  );
}

function OptionRow(props: { title: string; options: MarketOption[]; isMyTurn: boolean }) {
  // Keyed by option id (stable UUID from the server) rather than array
  // reference, so TransitionGroup only animates cards that actually entered
  // or left the row, not every card whenever any one option changes.
  const byId = createMemo(() => {
    const map: Record<string, MarketOption> = {};
    for (const opt of props.options) map[opt.id] = opt;
    return map;
  });
  const ids = createMemo(() => props.options.map((o) => o.id));

  return (
    <div class="option-row-block">
      <div class="row-title">{props.title}</div>
      <div class="market-row">
        <TransitionGroup name="card">
          <For each={ids()}>{(id) => <MarketCard opt={byId()[id]} isMyTurn={props.isMyTurn} />}</For>
        </TransitionGroup>
      </div>
    </div>
  );
}

export function MarketRow(props: { terrainOptions: MarketOption[]; animalOptions: MarketOption[]; isMyTurn: boolean }) {
  return (
    <div class="market-panel">
      <OptionRow title="Terrain shapes" options={props.terrainOptions} isMyTurn={props.isMyTurn} />
      <Show when={props.animalOptions.length > 0}>
        <OptionRow title="Animal placements" options={props.animalOptions} isMyTurn={props.isMyTurn} />
      </Show>
      <div class="market-actions">
        <Show when={selectedOption()}>
          <button onClick={clearSelection}>Cancel selection</button>
        </Show>
        <button disabled={!props.isMyTurn} onClick={() => shuffleMarket()}>
          Shuffle (costs your turn)
        </button>
      </div>
    </div>
  );
}
