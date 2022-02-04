<svelte:options immutable={true}/>

<script>
    import NFTBox from '../components/NFTBox.svelte';
    export let nfts;
</script>

<ul class="container">
    {#each nfts as nft}
        <li class="box">
            <NFTBox {nft}/>
        </li>
    {/each}
</ul>

<style>
    .container {
        list-style: none;
        display: flex;
        flex-flow: row wrap;
    }
    .box {
        width: 12em;
        padding: 10px 10px 10px 10px;
    }
</style>
