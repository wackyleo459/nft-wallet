<script>
    import * as nftAgent from '../nft';
    import { Principal } from '@dfinity/principal';
    let canister;
    let index;
    $:canSubmit = validPrincipal(canister) && !isNaN(index);
    let validCanister;
    function validPrincipal(principal) {
        try {
            Principal.fromText(principal);
            return true;
        } catch {
            return false;
        }
    }
    function validateCanister() {
        validCanister = validPrincipal(canister);
    }
    function removeError() {
        validCanister = undefined;
    }
    async function register() {
        await nftAgent.register(canister, index);
        window.location.href = '/';
    }
</script>

<div>
    {#await nftAgent.isAuthorized() then isAuthorized}
    {#if isAuthorized}
    <form on:submit|preventDefault={register}>
        <label for="canister">Canister principal:</label>
        <input type="text" id="canister" bind:value={canister} on:blur={validateCanister} on:focus={removeError}>
        {#if validCanister === false}
        <span class="error">Invalid principal</span>
        {/if}
        <br>
        <label for="index">Index #:</label>
        <input type="number" id="index" bind:value={index}>
        <br>
        <input type="submit" value="Register" disabled={!canSubmit}>
    </form>
    {:else}
    <p>You must be an authorized user to register new NFTs to this wallet.</p>
    {/if}
    {/await}
</div>
