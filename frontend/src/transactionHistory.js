import { writable } from "svelte/store";

export const transactionHistory = writable([]);

export function addTransaction(index, detailString) {
  transactionHistory.update((history) => [
    ...history,
    {
      date: new Date(),
      nft: index,
      detail: detailString,
    },
  ]);

  console.log("addTransactions called");
}
