import { format } from "date-fns";

export function formatDateDb(timestamp: number): string {
  return format(new Date(timestamp * 1_000), "yyyy-MM-dd'T'HH:mm");
}

export function formatDateDisplay(timestamp: number): string {
  return format(new Date(timestamp * 1_000), "dd-MM-yyyy HH:mm");
}
