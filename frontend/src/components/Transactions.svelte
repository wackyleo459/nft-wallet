<script >
    import { transactionHistory } from '../transactionHistory.js';
    import { isAuthorized } from '../nft.js';
    let transactions;
    // let transactionHistory = writable([{
    //   date: new Date(),
    //   nft: 100,
    //   detail: 'test'
    // }]);

    // const unsubscribe = transactionHistory.subscribe(value => {
    //   transactions = value;
    // })
    setTimeout(() => console.log('transactions component', $transactionHistory), 3000);
</script>

<div id="transactions">
    {#await isAuthorized() then isAuthorized}
    {#if isAuthorized}
      {#if $transactionHistory.length > 0}
          {#each $transactionHistory as transaction, idx}
            <div class="transaction">
              Transaction
                <div class="trans_date">{transaction.date}</div>
                <div class="trans_nft">{transaction.nft}</div>
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

</style>
