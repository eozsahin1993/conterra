import { createStore } from "solid-js/store";
import { createSignal } from "solid-js";
import type { ClientMessage, GameResult, PlacementInput, ServerMessage, StateSnapshot } from "./types";

// In dev, Vite serves the frontend on its own port while the Rust backend
// runs separately on 4173; in the production build both are served by Axum
// from the same origin, so relative URLs are enough.
const HTTP_BASE = import.meta.env.DEV ? "http://127.0.0.1:4173" : "";
const WS_BASE = import.meta.env.DEV ? "ws://127.0.0.1:4173" : `${location.protocol === "https:" ? "wss" : "ws"}://${location.host}`;

export const [snapshot, setSnapshot] = createStore<{ value: StateSnapshot | null }>({ value: null });
export const [myPlayerId, setMyPlayerId] = createSignal<string | null>(null);
export const [gameId, setGameId] = createSignal<string | null>(null);
export const [errorMessage, setErrorMessage] = createSignal<string | null>(null);
export const [gameResult, setGameResult] = createSignal<GameResult | null>(null);
export const [connected, setConnected] = createSignal(false);

let socket: WebSocket | null = null;
let errorTimer: ReturnType<typeof setTimeout> | undefined;

export function flashError(message: string) {
  setErrorMessage(message);
  clearTimeout(errorTimer);
  errorTimer = setTimeout(() => setErrorMessage(null), 4000);
}
export const setErrorMessageDirect = flashError;

export async function createGame(): Promise<string> {
  const res = await fetch(`${HTTP_BASE}/api/games`, { method: "POST" });
  const data = await res.json();
  return data.game_id as string;
}

export function connectToGame(id: string, name: string) {
  setGameId(id);
  const ws = new WebSocket(`${WS_BASE}/ws/${id}`);
  socket = ws;

  ws.onopen = () => {
    setConnected(true);
    send({ type: "Join", name });
  };
  ws.onclose = () => {
    setConnected(false);
    flashError("Disconnected from server.");
  };
  ws.onmessage = (ev) => handleMessage(JSON.parse(ev.data));
}

function handleMessage(msg: ServerMessage) {
  switch (msg.type) {
    case "Joined":
      setMyPlayerId(msg.player_id);
      break;
    case "State":
      setSnapshot("value", msg.snapshot);
      break;
    case "Result":
      setGameResult(msg.result);
      break;
    case "Error":
      flashError(msg.message);
      break;
  }
}

function send(msg: ClientMessage) {
  socket?.send(JSON.stringify(msg));
}

export function startGame() {
  send({ type: "Start" });
}

export function selectOption(optionId: string, placement: PlacementInput) {
  send({ type: "Select", option_id: optionId, placement });
}

export function shuffleMarket() {
  send({ type: "Shuffle" });
}
