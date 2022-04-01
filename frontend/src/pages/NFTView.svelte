<script>
    import Carousel from '../components/Carousel.svelte';
    import Copier from '../components/Copier.svelte';
    import LocationTypeIcon from '../components/LocationTypeIcon.svelte';
    import mime from 'mime/lite';
    import { isAuthorized } from '../nft';
    import {
        Content,
        Media,
        MediaContent,
    } from '@smui/card';
    import Button, { Label } from '@smui/button';
    import { Tile, CopyButton } from "carbon-components-svelte";
    export let nft;
    export let current;
    $:current = current || 0;
    $:locationTypes = nft.content.map(elem => elem.locationType);
    $:locationType = locationTypes[current];
</script>

<div class="NFT_view">
    <div id="tile">
        <Tile istyle="max-width: 350px; margin:auto;">
            <div class="badge">NFT</div>
            <h2>
                {nft.symbol} #{nft.index}
            </h2>
            <Media >
                <MediaContent style="position:relative" id="media_content">
                <Carousel content={nft.content} bind:current --fallback-bg="gray">
                    <svelte:fragment slot="fallback" let:contentType let:src>
                        {@const extension = mime.getExtension(contentType)}
                        {@const filename = extension ? `${nft.symbol}_${current}.${extension}` : `${nft.symbol}_${current}`}
                        <a class="button" href={src} download={filename}>Download {extension.toUpperCase() || 'file'}</a>
                    </svelte:fragment>
                </Carousel>
                </MediaContent>
            </Media>
        </Tile>
    </div>
    <div class="nft_info">
        <div class="info_title">Minted Date</div>
        <hr>
        <div class="info_detail">12/25/21</div>
        <div class="info_title">Type</div>
        <hr>
        <div class="info_detail">
            <img src={nft.icon} alt="{nft.name} NFT icon" class="icon">
                {nft.name}
            {#if locationType}
            <a href={nft.location}><LocationTypeIcon {locationType}/></a>
            {/if}
        </div>

        <div class="info_title">Canister ID</div>
        <hr>
        <div class="info_detail">
            {nft.canister}
           <CopyButton text={nft.canister} feedback="Copied to clipboard" style="left:5px"></CopyButton>
        </div>
        <div id="action_calls">
            {#await isAuthorized() then isAuthorized}
            {#if isAuthorized}
            <Button id="button" variant="raised" style="width:100%" href="/{nft.canister}/{nft.index}/transfer">
                <Label>Transfer</Label>
            </Button>
            {/if}
            {/await}
        </div>
    </div>
</div>

<style>
    .NFT_view {
        max-height: 80vh;
    }
    #tile {
        margin: 1em;
    }
    @media (min-width: 630px) {
        .NFT_view {
            display: grid;
            grid-template-areas: "picture info";
        }
        #tile {
            grid-area: picture;
        }
        .nft_info {
            grid-area: info;
        }
    }
    .info_title {
        font-size: 18px;
    }
    .info_detail {
        margin-bottom: 2em;
        display: flex;
        align-items: center;
        flex-wrap: wrap;
    }
    .button {
        position: relative;
        margin: auto;
    }
    h2 {
        font-family: 'Noto Sans', sans-serif;
        font-family: 'Roboto Mono', monospace;
    }
    .badge {
        font-size: 18px;
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
    .nft_info {
        padding: 1em;
    }
    #action_calls {
        padding: 2em 0;
    }
</style>
