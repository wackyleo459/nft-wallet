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
        The wallet can hold countless NFTs, all in a single secure wallet! <br/>Start adding your NFTs here. Once in your wallet, you can view, send, receive NFTs, and more!"
        <div class="action">
            Please login
            <br><br>
            <Button variant="raised" on:click={login}>
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
    .wallet-view {
        width: 100%;
        line-height: 1.5rem;
        padding: 3em;
        display: flex;
        flex-direction: column;
        border-radius: 15px;
        font-size: 16px;
        justify-content: center;
    }
    .button {
        height: 3em;
        align-items: center;
        display: flex;
        margin: 2em;
    }
    .action {
        height: 300px;
        display: flex;
        flex-direction: column;
        justify-content: center;
        align-items: center;
        font-size: 1.5em;
    }
</style>
