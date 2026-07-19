import { For, Show } from "solid-js";
import type { GameResult } from "../types";
import { objectiveText } from "../format";
import { setGameResult } from "../store";

export function ResultModal(props: { result: GameResult }) {
  return (
    <div class="modal-backdrop">
      <div class="modal-box">
        <h2>Game over</h2>
        <p>
          Group threshold:{" "}
          <b>{props.result.group_threshold_met ? "MET" : "NOT MET"}</b> (population{" "}
          {props.result.total_population}, longest corridor {props.result.longest_corridor})
        </p>
        <ul>
          <For each={props.result.players}>
            {(p) => (
              <li>
                <Show when={props.result.winners.includes(p.player_id)}>🏆 </Show>
                {p.name}: {objectiveText(p.objective)} — {p.met ? "met" : "not met"} (score {p.score})
              </li>
            )}
          </For>
        </ul>
        <button onClick={() => setGameResult(null)}>Close</button>
      </div>
    </div>
  );
}
