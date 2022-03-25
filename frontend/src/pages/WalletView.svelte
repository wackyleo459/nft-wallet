<script>
    import NFTGrid from '../components/NFTGrid.svelte';
    import { fetchAllOwnedNfts, isAuthorized } from '../nft.js';
    import {loadSpinner, hideSpinner} from '../components/Loader.svelte';
    import Button, { Label } from '@smui/button';
    import { login } from '../components/Authenticator.svelte'
</script>
<div class="wallet-view">
    {#await isAuthorized() then isAuthorized}
    {#if !isAuthorized}
        <h2>
            Welcome to the nft wallet for the IC
        </h2><br/>
        The wallet can hold countless NFTs, all in a single secure wallet. <br/>Once in your wallet, you can view, send, receive NFTs and more!
        <div class="action">
            Start adding your NFTs here.
            <br><br>
            <Button id="button" variant="raised" on:click={login}>
                <Label>LOGIN</Label>
            </Button>
        </div>
    {/if}
    {#if isAuthorized}
        {#await fetchAllOwnedNfts() then nfts}
            <!-- {#if !nfts }
                {loadSpinner("mainLoader")}
            {:else}
            {hideSpinner("mainLoader")} -->
            <NFTGrid {nfts}/>
            <!-- {/if} -->
        {/await}
    {/if}
    {/await}
</div>

<style>
    @media (max-width: 375px) {
        .wallet-view {
            width: 100%;
            line-height: 1.2rem;
            padding: 1em;
        }
        h2 {
            font-size: 28px;
        }
    }
    @media (min-width: 425px) {
        .wallet-view {
            text-align: center;
            padding: 3em;
        }
    }
    .wallet-view {
        width: 100%;
        line-height: 1.5rem;
        display: flex;
        flex-direction: column;
        font-size: 16px;
        justify-content: center;
    }

    .action {
        height: 300px;
        display: flex;
        flex-direction: column;
        justify-content: center;
        align-items: center;
    }
</style>
