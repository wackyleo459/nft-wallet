<script>
    import NFTGrid from '../components/NFTGrid.svelte';
    import { fetchAllOwnedNfts, isAuthorized } from '../nft.js';
    import Authenticator from '../components/Authenticator.svelte';
</script>
<ul class="list-group">
    {#await isAuthorized() then isAuthorized}
    {#if !isAuthorized}
        <div class="jumbotron">
            The wallet can hold countless NFTs, all in a single secure wallet! Start adding your NFTs here.

            Once in your wallet, you can view, send, receive NFTs, and more. Please login first.
        </div>
        {/if}
        {#if isAuthorized}
        <a class="button" href="/register">Register a new NFT</a>
        {#await fetchAllOwnedNfts() then nfts}

        <NFTGrid {nfts}/>
        {/await}
    {/if}
    {/await}
</ul>

<style>
    .list-group {
        width: 100%;
        display: flex;
        flex-direction: column;
        align-items: center;
    }
    .jumbotron {
        border: 2px solid #6d2482;
        margin: 2em 5em;
        line-height: 2rem;
        padding: 3em 4em;
        display: flex;
        flex-direction: column;
        align-items: center;
        border-radius: 15px;
        font-size: large;
        max-width: 500px;
    }
    .button {
        height: 3em;
        align-items: center;
        display: flex;
        margin: 2em;
    }
</style>
