import { createSignal, Show } from "solid-js";
import { Portal } from "solid-js/web";
import type { AnimalTileInfo, Direction, Hex } from "../types";
import { HEX_SIZE, hexToPixel } from "../hex";
import { prettySpecies } from "../format";

const DIRECTION_SYMBOL: Record<Direction, string> = {
  Rising: "▲",
  Flat: "●",
  Falling: "▼",
};
export const DIRECTION_COLOR: Record<Direction, string> = {
  Rising: "#2e7d32",
  Flat: "#9aa1ac",
  Falling: "#c62828",
};

// Owns its own hover state so Board doesn't need to track "which colony is
// hovered" for every token on the board — each Colony only cares about
// itself. The tooltip is positioned via `position: fixed` in viewport
// coordinates, so it's portaled out of the <svg> (a <div> can't live inside
// an <svg>/<g>) rather than rendered in place.
export function Colony(props: { hex: Hex; info: AnimalTileInfo; starvationThreshold: number }) {
  const p = () => hexToPixel(props.hex);
  const [hoverPos, setHoverPos] = createSignal<{ x: number; y: number } | null>(null);

  return (
    <g
      class="animal-token"
      transform={`translate(${p().x}, ${p().y})`}
      onMouseEnter={(ev) => setHoverPos({ x: ev.clientX, y: ev.clientY })}
      onMouseMove={(ev) => setHoverPos({ x: ev.clientX, y: ev.clientY })}
      onMouseLeave={() => setHoverPos(null)}
    >
      <circle r={HEX_SIZE * 0.42} stroke={DIRECTION_COLOR[props.info.direction]} stroke-width={HEX_SIZE * 0.09} />
      <text class="token-letter" text-anchor="middle" dominant-baseline="central" font-size={`${HEX_SIZE * 0.38}`}>
        {props.info.species[0]}
      </text>
      <Show when={hoverPos()}>
        {(pos) => (
          <Portal>
            <div class="colony-tooltip" style={{ left: `${pos().x + 16}px`, top: `${pos().y + 16}px` }}>
              <div class="colony-tooltip-title">{prettySpecies(props.info.species)}</div>
              <div>Population: {props.info.counter.toFixed(1)}</div>
              <div>Colony size: {props.info.colony_size} tiles</div>
              <div>
                Growth rate:{" "}
                <span style={{ color: DIRECTION_COLOR[props.info.direction] }}>
                  {props.info.rate >= 0 ? "+" : ""}
                  {props.info.rate.toFixed(2)}/turn {DIRECTION_SYMBOL[props.info.direction]}
                </span>
              </div>
              <div class="colony-tooltip-factors">
                Open: {props.info.open_adjacent} · Predators: {props.info.predator_adjacent} · Prey:{" "}
                {props.info.prey_adjacent}
                <Show when={props.info.contending_adjacent > 0}> · Contending: {props.info.contending_adjacent}</Show>
              </div>
              <div>Next expansion at population {props.info.spillover_threshold} (scales with colony size)</div>
              <div>Starves at population {props.starvationThreshold}</div>
            </div>
          </Portal>
        )}
      </Show>
    </g>
  );
}
