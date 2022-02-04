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
        <form on:submit|preventDefault={transfer}>
            <label for="principal">Principal to transfer to:</label>
            <input type="text" id="principal" bind:value={principal}>
            <br>
            <label for="notify">Notify the recipient?</label>
            <select id="notify" bind:value={notify}>
                <option value="no">No</option>
                <option value="maybe" selected>Yes, but transfer anyway if unsupported</option>
                <option value="yes">Yes, and don't transfer without it</option>
            </select>
            <br>
            <input type="checkbox" id="confirm" bind:checked={confirmed}>
            <label for="confirm">I understand that if this principal is wrong, I'm probably not getting this NFT back</label>
            <br>
            <input type="submit" value="Transfer" disabled={!canSubmit}>
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
