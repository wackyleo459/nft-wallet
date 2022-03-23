<script >
    import { transactionHistory } from '../storage/transactionHistory.js';
    import { isAuthorized } from '../nft.js';


      console.log('whenever transactions change', $transactionHistory);

</script>

<div id="transactions">
  Transactions
    {#await isAuthorized() then isAuthorized}
    {#if isAuthorized}
      {#if $transactionHistory.length > 0}
          {#each $transactionHistory as transaction, idx}
            <div class="transaction">
                <div class="trans_date">{transaction.date}</div>
                <div class="trans_nft">NFT ITEM {transaction.nft}</div>
                <div class="trans_details">{transaction.detail}</div>
            </div>
          {/each}
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
  .transaction>div {
    border: solid 1px green;
  }
</style>
