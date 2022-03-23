<svelte:options immutable={true}/>

<script>
    import NFTBox from '../components/NFTBox.svelte';
    export let nfts;
</script>
<div class="collection">
    <div class="section">Collection</div>
    <hr>
    <div class="container">
        {#each nfts as nft}
        <div class="paper-container box">
            <NFTBox {nft}/>
        </div>
        {/each}
    </div>
</div>

<style>
    /* @import "@material/elevation/mdc-elevation"; */
    .container {
        list-style: none;
        display: flex;
        flex-flow: row wrap;
        margin: 1em;
    }
    .box {
        width: 13em;
        padding: 1em;
    }
    .collection {
        width: inherit;
    }
    li.box:hover {
        margin-top: 3px;
        border-radius: 10px;
    }
    hr {
        color: rgb(123, 117, 117);
    }
</style>
