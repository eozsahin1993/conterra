import { For, Show } from "solid-js";
import type { StateSnapshot } from "../types";
import { objectiveText, prettySpecies } from "../format";
import { gameId, myPlayerId, startGame } from "../store";

export function Sidebar(props: { snapshot: StateSnapshot }) {
  const s = () => props.snapshot;

  return (
    <div class="sidebar">
      <div class="panel">
        <div>
          Game ID: <code>{gameId()}</code>
        </div>
        <Show when={s().phase === "InProgress" || s().phase === "Ended"}>
          <div>
            Turn {s().turns_taken} / {s().total_turns}
          </div>
        </Show>
        <ul class="players-list">
          <For each={s().players}>
            {(p) => (
              <li classList={{ current: p.id === s().current_player, me: p.id === myPlayerId() }}>
                {p.name}
                {p.id === myPlayerId() ? " (you)" : ""}
              </li>
            )}
          </For>
        </ul>
        <Show when={s().phase === "Lobby"}>
          <button onClick={() => startGame()}>Start game</button>
        </Show>
      </div>

      <Show when={s().phase !== "Lobby"}>
        <div class="panel">
          <b>Your secret objective</b>
          <div>{objectiveText(s().my_objective)}</div>
        </div>

        <div class="panel">
          <b>Last growth pass</b>
          <div>
            <Show when={s().last_spillover && s().last_spillover!.length} fallback="—">
              <For each={s().last_spillover}>
                {([sp, n], i) => `${i() > 0 ? ", " : ""}${prettySpecies(sp)} spilled over +${n}`}
              </For>
            </Show>
          </div>
          <Show when={s().last_starvation && s().last_starvation!.length}>
            <div class="consumed-line">
              <For each={s().last_starvation}>
                {([sp, n], i) => `${i() > 0 ? ", " : ""}${prettySpecies(sp)} starved -${n}`}
              </For>
            </div>
          </Show>
        </div>
      </Show>
    </div>
  );
}
