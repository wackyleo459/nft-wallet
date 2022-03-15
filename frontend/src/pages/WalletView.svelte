<script>
    import NFTGrid from '../components/NFTGrid.svelte';
    import { fetchAllOwnedNfts, isAuthorized } from '../nft.js';
    import Authenticator from '../components/Authenticator.svelte';
</script>
<div class="wallet-view">
    {#await isAuthorized() then isAuthorized}
    {#if !isAuthorized}
        <div class="jumbotron">
            <h2>
                Welcome to the nft wallet for the IC
            </h2>
            The wallet can hold countless NFTs, all in a single secure wallet! Start adding your NFTs here. Once in your wallet, you can view, send, receive NFTs, and more. Please login first.
        </div>
        {/if}
        {#if isAuthorized}

        {#await fetchAllOwnedNfts() then nfts}

        <NFTGrid {nfts}/>
        {/await}
    {/if}
    {/await}
</div>

<style>
    .wallet-view {
        width: 100%;
    }
    .jumbotron {
        border: 2px solid #6d2482;
        background-color: #4f1c5e;
        line-height: 1.5rem;
        padding: 3em;
        display: flex;
        flex-direction: column;
        border-radius: 15px;
        font-size: 14px;
    }
    .button {
        height: 3em;
        align-items: center;
        display: flex;
        margin: 2em;
    }
</style>
