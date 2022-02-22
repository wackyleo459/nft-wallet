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
        const collection = await nftAgent.fetchAllOwnedNfts();
        collection.forEach(nft => {
            if (Number(nft.index) === index) {
                alert(" NFT by that index is already registered, but will continue anyway.");
            }
        })
        const result = await nftAgent.register(canister, index);
        if (result) alert(result);
        window.location.href = '/';
    }
</script>

<div class="register-view">
    {#await nftAgent.isAuthorized() then isAuthorized}
    {#if isAuthorized}
    <form class="form" on:submit|preventDefault={register}>
        <h2>Register a new NFT</h2>
        <div id="form_top">
            <label for="canister">NFT Canister ID:</label>
            <input type="text" id="canister" bind:value={canister} on:blur={validateCanister} on:focus={removeError}>
            <div id="nft_cid_help" class="form_text">
                Please enter the principal of your NFT management canister.
              </div>
            {#if validCanister === false}
            <span class="error">Invalid principal</span>
            {/if}
        </div>
        <div>
            <label for="index" class="index">Index #:</label>
            <input type="number" min="0" id="index" bind:value={index}>
            <div id="nft_index_help" class="form_text">
                Provide the specific token id associated with the NFT you are registering.
            </div>
            <br>
            <div class="div_button_primary">
                <button type="submit" class ="button_primary {!canSubmit ? "disabled": null}" >
                    Register
                </button>
            </div>

        </div>
    </form>
    {:else}
    <p>You must be an authorized user to register new NFTs to this wallet.</p>
    {/if}
    {/await}
</div>

<style>
    .register-view {
        margin-top: 20px;
        margin-bottom: 20px;
        max-width: 550px;
        display: flex;
        flex-direction: column;
        align-items: center;
    }

    h2 {
        text-align: center;
        padding-bottom: 1em;
    }
    p {
        text-align: center;
    }
    #form_top {
        margin-bottom: 20px;
        display: flex;
        flex-direction: column
    }
    .index {
        margin-right: 15px;
    }
</style>
