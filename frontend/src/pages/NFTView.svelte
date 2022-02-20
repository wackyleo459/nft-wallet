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

<div>
    <h2>{nft.symbol} #{nft.index}</h2>
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
    <a class="btn" href="/{nft.canister}/{nft.index}/transfer">Transfer</a>
    {/if}
    {/await}
</div>

<style>
    div {
        display: flex;
        justify-items: center;
        flex-direction: column;
        width: 40%;
        max-height: 80vh;
    }
    p {
        font-size: 20px;
    }
    .canister {
        font-size: 12px;
    }
    .button {
        position: relative;
        margin: auto;
    }
    .btn {
        border: solid 2px #5f2684;
    }
    .btn:hover {
        background-color: #5f2684;
        color: white;
    }
    a:link { text-decoration: none; }
    a:visited { text-decoration: none; }
    a:hover { text-decoration: none; }
    a:active { text-decoration: none; }
</style>
