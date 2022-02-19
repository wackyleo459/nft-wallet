<script>
    import NFTGrid from '../components/NFTGrid.svelte';
    import { fetchAllOwnedNfts, isAuthorized } from '../nft.js';
</script>
<ul class="list-group">
    {#await isAuthorized() then isAuthorized}
    {#if isAuthorized}

    <div class="jumbotron">
        The wallet can hold countless NFTs, all in a single secure wallet! Start adding your NFTs here. Once in your wallet, you can view, send, receive NFTs.
        <a class="button" href="/register">Register a new NFT</a>
    </div>
    {/if}
    {/await}
    {#await fetchAllOwnedNfts() then nfts}
    <hr class="my-4">
    <NFTGrid {nfts}/>
    {/await}
</ul>

<style>
    .jumbotron {
        display: flex;
        flex-direction: column;
        align-items: center;
    }
</style>
