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
    <h4 class="pb-3">Register a new NFT</h4>
    {#await nftAgent.isAuthorized() then isAuthorized}
    {#if isAuthorized}
    <form on:submit|preventDefault={register}>
        <div class="p-3">
            <label class="form-label pt-1" for="canister">NFT Canister ID:</label>
            <input type="text" class="form-control" id="canister" bind:value={canister} on:blur={validateCanister} on:focus={removeError}>
            <div id="nft_cid_help" class="form-text">
                Please enter the principal of your NFT management canister.
              </div>
            {#if validCanister === false}
            <span class="error">Invalid principal</span>
            {/if}
        </div>
        <div class="mb-3 px-3 pb-3">
            <label for="index">Index #:</label>
            <input type="number" class="form-control" min="0" id="index" bind:value={index}>
            <div id="nft_index_help" class="form-text">
                Provide the specific token id associated with the NFT you are registering.
            </div>
            <br>
            <button type="submit" class ="btn btn-primary {!canSubmit ? "disabled": null}" >
                Register
            </button>
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
    }
    form {
        border-radius: 15px;
        border: solid 3px white;
    }
    h4 {
        text-align: center;
    }
</style>
