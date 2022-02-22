<script>
    import Carousel from '../components/Carousel.svelte';
    import Copier from '../components/Copier.svelte';
    import LocationTypeIcon from '../components/LocationTypeIcon.svelte';
    import mime from 'mime/lite';
    import { isAuthorized } from '../nft';
    export let nft;
    export let current;
    $:current = current || 0;
    $:locationTypes = nft.content.map(elem => elem.locationType);
    $:locationType = locationTypes[current];
</script>

<div class="NFT_view">
    <div class="badge">NFT</div>
    <h2>
        {nft.symbol} #{nft.index}
    </h2>
    <Carousel content={nft.content} bind:current --fallback-bg="gray">
        <svelte:fragment slot="fallback" let:contentType let:src>
            {@const extension = mime.getExtension(contentType)}
            {@const filename = extension ? `${nft.symbol}_${current}.${extension}` : `${nft.symbol}_${current}`}
            <a class="button" href={src} download={filename}>Download {extension.toUpperCase() || 'file'}</a>
        </svelte:fragment>
    </Carousel>
    <p>
        <img src={nft.icon} alt="{nft.name} NFT icon" class="icon">
        {nft.name}
        {#if locationType}
        <a href={nft.location}><LocationTypeIcon {locationType}/></a>
        {/if}
    </p>
    <p class="canister"><Copier always text={nft.canister}>{nft.canister}</Copier></p>
    {#await isAuthorized() then isAuthorized}
    {#if isAuthorized}
    <a class="transfer_button" href="/{nft.canister}/{nft.index}/transfer">Transfer</a>
    {/if}
    {/await}
</div>

<style>
    div {
        display: flex;
        justify-items: center;
        flex-direction: column;
        width: 330px;
        max-height: 80vh;
    }
    p {
        font-size: 20px;
        text-align: center;
    }
    .canister {
        font-size: 1em;
        text-align: center;
    }
    .button {
        position: relative;
        margin: auto;
    }
    .transfer_button {
        border: solid 2px #5f2684;
        margin: 5px;
        padding: 6px 8px;
        border-radius: 5px;
        text-align: center;
        margin: 15px 0;
    }
    .transfer_button:hover {
        background-color: #5f2684;
        color: white;
    }
    a:link { text-decoration: none; }
    a:visited { text-decoration: none; }
    a:hover { text-decoration: none; }
    a:active { text-decoration: none; }

    h2 {
        font-family: 'Noto Sans', sans-serif;
        font-family: 'Roboto Mono', monospace;
    }
    .badge {
        font-size: 18px;
        width: 2em;
        display: inline-block;
        padding: 0.35em 0.65em;
        font-weight: 700;
        color: #212529;
        text-align: center;
        white-space: nowrap;
        vertical-align: baseline;
        border-radius: 0.25rem;
        background-color: #ffc107;
        display: inline-block;
    }
</style>
