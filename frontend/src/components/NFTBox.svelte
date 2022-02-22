<svelte:options immutable={true}/>

<script>
    import Copier from './Copier.svelte';
    import ContentBox from './ContentBox.svelte';

    export let nft;
</script>

<div class="card">
    <a class="ui" href="/{nft.canister}/{nft.index}">
        <div class="preview">
            <ContentBox src={nft.preview?.value} contentType={nft.preview?.contentType}/>
        </div>
    </a>
    <div class="desc">
        <a class="ui" href="/{nft.canister}/{nft.index}"><h4>{nft.symbol} #{nft.index}</h4></a>
        <p>
            <a class="ui" href="/{nft.canister}">
                <img class="icon" src={nft.icon} alt="icon">{nft.name}
            </a>
            <Copier text={nft.canister}/>
        </p>
    </div>
</div>

<style>
    .preview {
        border-radius: 5px 5px 0 0;
        background: white;
    }
    .card {
        transition: 0.3s;
        border-radius: 5px;
        background: slategray;
    }
    .desc {
        padding: 2px 16px;
    }
    h4 {
        font-weight: 200;
        font-family: 'Noto Sans', sans-serif;
        font-family: 'Roboto Mono', monospace;
    }
</style>
