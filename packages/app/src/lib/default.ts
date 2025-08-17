import { PUBLIC_BASE_API_URL } from "$env/static/public";
import { createHttpClient } from "./http-client";

export const httpClient = createHttpClient(
  PUBLIC_BASE_API_URL !== "" ? PUBLIC_BASE_API_URL : window.location.origin,
);
