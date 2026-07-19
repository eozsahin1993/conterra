import { createSignal } from "solid-js";
import { connectToGame, createGame, setErrorMessageDirect } from "../store";

export function Lobby() {
  const [name, setName] = createSignal("");
  const [joinId, setJoinId] = createSignal("");
  const [creating, setCreating] = createSignal(false);

  async function handleCreate() {
    setCreating(true);
    try {
      const id = await createGame();
      connectToGame(id, name().trim() || "Player");
    } catch (err) {
      setErrorMessageDirect(
        err instanceof Error ? `Could not create game: ${err.message}` : "Could not create game.",
      );
    } finally {
      setCreating(false);
    }
  }

  function handleJoin() {
    const id = joinId().trim();
    if (id) connectToGame(id, name().trim() || "Player");
  }

  return (
    <div class="lobby-screen">
      <h1>Conterra</h1>
      <p>A turn-based multiplayer habitat-building game.</p>
      <input placeholder="Your name" value={name()} onInput={(e) => setName(e.currentTarget.value)} />
      <div class="lobby-row">
        <button onClick={handleCreate} disabled={creating()}>
          {creating() ? "Creating…" : "Create new game"}
        </button>
      </div>
      <div class="lobby-row">
        <input
          placeholder="Game ID to join"
          value={joinId()}
          onInput={(e) => setJoinId(e.currentTarget.value)}
        />
        <button onClick={handleJoin}>Join game</button>
      </div>
    </div>
  );
}
