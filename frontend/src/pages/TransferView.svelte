<script>
    import { Principal } from '@dfinity/principal'
    import * as nftAgent from '../nft';
    export let nft;
    let principal;
    let notify = 'maybe';
    let confirmed = false;
    $:canSubmit = confirmed && validData({principal, notify});
    function transfer() {
        if (!validData({principal, notify})) {
            return;
        }
        let shouldNotify;
        switch (notify) {
            case 'no':
                shouldNotify = false;
                break;
            case 'yes':
                shouldNotify = true;
                break;
            case 'maybe':
                shouldNotify = null;
                break;
        }
        nftAgent.transfer(nft, principal, shouldNotify).then(() => {
            window.location.href = '/';
        });
    }
    let errorMessage;
    function validData({principal, notify}) {
        try {
            Principal.fromText(principal);
        } catch {
            errorMessage = 'Invalid principal';
            return false;
        }
        if (!(['no', 'maybe', 'yes'].includes(notify))) {
            errorMessage = 'Select a valid notify option';
            return false;
        }
        errorMessage = null;
        return true;
    }
    let _isAuthorized = nftAgent.isAuthorized();
</script>

<div>
    <h2>Transfer {nft.name} ({nft.symbol}) #{nft.index}</h2>
    {#await _isAuthorized then isAuthorized}
        {#if isAuthorized}
        <form on:submit|preventDefault={transfer} class="my-5">
            <label class="form-label pt-1" for="principal">Principal to transfer to:</label>
            <input class="form-control" type="text" id="principal" bind:value={principal}>
            <br>
            <label id="label_notify" for="notify">Notify the recipient?</label>
            <select id="notify" bind:value={notify}>
                <option value="no">No</option>
                <option value="maybe" selected>Yes, but transfer anyway if unsupported</option>
                <option value="yes">Yes, and don't transfer without it</option>
            </select>
            <br>
            <div class="agreement">
                <input type="checkbox" id="confirm" bind:checked={confirmed}>
                <label for="confirm">I understand that if this principal is wrong, I'm probably not getting this NFT back</label>
            </div>
            <button type="submit" class ="btn btn-primary {!canSubmit ? "disabled": null}">Transfer
            </button>
            {#if errorMessage}
            <p class="error">{errorMessage}</p>
            {/if}
        </form>
        {:else}
        <p>You must be an authorized user to transfer NFTs out of this wallet.</p>
        {/if}
    {:catch}
    <p>You can't transfer NFTs out of this wallet if the canister can't be accessed.</p>
    {/await}
</div>

<style>
    form {
        max-width: 400px;
        border-radius: 15px;
        border: solid 2.5px white;
        padding: 1rem;

    }
    .agreement {
        padding: 1.5em 0;
        display: flex;
        justify-content: space-between;
        align-items: center;
    }
    .agreement label {
        max-width: 90%;
    }
    #label_notify {
        margin-right: 1em;
    }
</style>
