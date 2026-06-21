import type { ApiClient } from "@promethea/frontend-shared";

export const api: ApiClient = {
  async hello() {
    return "Hello from the web frontend";
  },
};
