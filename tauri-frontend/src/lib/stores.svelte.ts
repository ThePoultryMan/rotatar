import type { Config, State } from "./types";

export const frontendData: FrontendData = $state({
  config: undefined,
  state: undefined,
});

type FrontendData = {
  config?: Config;
  state?: State
};
