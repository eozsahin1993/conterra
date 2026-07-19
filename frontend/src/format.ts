import type { SecretObjective, Species } from "./types";

// The wire format carries Rust enum-variant identifiers (e.g.
// "GreatWhiteShark"); this is purely a display-layer transform, not a
// protocol concern.
export function prettySpecies(species: Species): string {
  return species.replace(/([a-z0-9])([A-Z])/g, "$1 $2");
}

export function objectiveText(obj: SecretObjective | null): string {
  if (!obj) return "—";
  if (obj.type === "PopulationTarget") {
    return `Have ${obj.target}+ ${prettySpecies(obj.species)} on the map.`;
  }
  return `Have a ${prettySpecies(obj.species)} adjacent to ${obj.distinct_terrains}+ distinct terrain types.`;
}
