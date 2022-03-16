<svelte:options immutable={true}/>

<script>
    import NFTBox from '../components/NFTBox.svelte';
    export let nfts;
</script>
<div class="collection">
    <div class="section">Collection</div>
    <hr style="color: rgb(123, 117, 117)">
    <div class="container">
        {#each nfts as nft}
        <div class="mdc-elevation--z4 flexy-boy">
            <li class="box">
                <NFTBox {nft}/>
            </li>
        </div>
        {/each}
    </div>
</div>

<style>
    @import "@material/elevation/mdc-elevation";
    .container {
        list-style: none;
        display: flex;
        flex-flow: row wrap;
        margin: 1em;
    }
    .box {
        width: 12em;
        padding: 1em;
    }
    .collection {
        width: inherit;
    }
    li.box:hover {
        margin-top: 3px;
        border-radius: 10px;
    }
</style>
