<script>
    import NFTGrid from '../components/NFTGrid.svelte';
    import { fetchAllOwnedNfts, isAuthorized } from '../nft.js';
</script>

{#await isAuthorized() then isAuthorized}
{#if isAuthorized}
<a class="button" href="/register">Register a new NFT</a>
{/if}
{/await}
{#await fetchAllOwnedNfts() then nfts}
<NFTGrid {nfts}/>
{/await}
