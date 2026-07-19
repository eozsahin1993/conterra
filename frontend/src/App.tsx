import { Show } from "solid-js";
import { Board } from "./components/Board";
import { Lobby } from "./components/Lobby";
import { MarketRow } from "./components/MarketRow";
import { ResultModal } from "./components/ResultModal";
import { Sidebar } from "./components/Sidebar";
import type { Hex } from "./types";
import { clearSelection, rotation, selectedOption } from "./selection";
import { errorMessage, gameResult, myPlayerId, selectOption, snapshot } from "./store";
import "./App.css";

function GameScreen() {
  const s = () => snapshot.value!;
  const isMyTurn = () => s().current_player === myPlayerId();

  function handleHexClick(hex: Hex) {
    const opt = selectedOption();
    if (!opt) return;
    if (opt.type === "TerrainShape") {
      selectOption(opt.id, { type: "Terrain", origin: hex, rotation: rotation() });
    } else {
      selectOption(opt.id, { type: "Animal", hex });
    }
    clearSelection();
  }

  return (
    <div class="game-screen">
      <div class="turn-banner" classList={{ mine: isMyTurn() && s().phase === "InProgress" }}>
        <Show when={s().phase === "Lobby"}>Waiting in lobby…</Show>
        <Show when={s().phase === "InProgress"}>{isMyTurn() ? "Your turn!" : "Waiting for other player…"}</Show>
        <Show when={s().phase === "Ended"}>Game ended.</Show>
      </div>
      <div class="layout">
        <div class="board-wrap">
          <Board
            radius={s().board_radius}
            terrain={s().terrain}
            animals={s().animals}
            armed={selectedOption() !== null && isMyTurn()}
            onHexClick={handleHexClick}
          />
        </div>
        <div class="side-column">
          <Sidebar snapshot={s()} />
          <Show when={s().phase === "InProgress"}>
            <MarketRow options={s().market_row} isMyTurn={isMyTurn()} />
          </Show>
        </div>
      </div>
    </div>
  );
}

function App() {
  return (
    <>
      <Show when={snapshot.value} fallback={<Lobby />}>
        <GameScreen />
      </Show>
      <Show when={errorMessage()}>
        <div class="error-toast">{errorMessage()}</div>
      </Show>
      <Show when={gameResult()}>{(result) => <ResultModal result={result()} />}</Show>
    </>
  );
}

export default App;
