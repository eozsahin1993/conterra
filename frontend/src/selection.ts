import { createSignal } from "solid-js";
import type { MarketOption } from "./types";

export const [selectedOption, setSelectedOption] = createSignal<MarketOption | null>(null);
export const [rotation, setRotation] = createSignal(0);

export function armOption(opt: MarketOption) {
  setSelectedOption(opt);
  setRotation(0);
}

export function clearSelection() {
  setSelectedOption(null);
  setRotation(0);
}

export function rotateSelection() {
  setRotation((r) => (r + 1) % 6);
}
