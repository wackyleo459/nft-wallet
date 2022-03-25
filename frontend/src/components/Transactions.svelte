<script>
    import { transactionHistory } from '../storage/transactionHistory.js';
    import { isAuthorized } from '../nft.js';
    import "carbon-components-svelte/css/white.css";
    import { Theme, DataTable } from "carbon-components-svelte";
    let theme = "g80";
    $: transaction_rows = $transactionHistory.map((transaction, i) => {
      const date = (new Date(transaction.date)).toLocaleString();
      return {
        "id": i,
        "date": date,
        "nft": transaction.nft,
        "detail": transaction.detail,
      }
    });

    setTimeout(() => console.log($transactionHistory), 3000);
  </script>
<Theme bind:theme />
<div id="transactions">
  {#await isAuthorized() then isAuthorized}
  {#if isAuthorized}
    {#if $transactionHistory.length > 0}
    <DataTable
      sortable
      bind:selecte={theme}
      title ="Transactions"
      headers={[
        { key: "date", value: "Date" },
        { key: "nft", value: "NFT" },
        { key: "detail", value: "Detail" },
      ]}
      rows= {transaction_rows}
      size = "tall"
    />
    {:else}
        <div class="transaction">
          No transactions record to display.
        </div>
    {/if}
    {:else}
      <div class="transaction">
        No transactions record, you must login first.
      </div>
  {/if}
  {/await}
</div>

<style>
  .transaction {
    display: flex;
    flex-direction: column;
    justify-content: space-between;
    margin: 15px 0;
  }

</style>
