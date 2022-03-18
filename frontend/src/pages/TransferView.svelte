<script>
    import { Principal } from '@dfinity/principal';
    import * as nftAgent from '../nft';
    export let nft;
    import Loader, {loadSpinner, hideSpinner} from '../components/Loader.svelte';
    import { addTransaction } from '../transactionHistory.js';
    import page from 'page';

    export let pageState;
    let loaderId = "transferLoader";
    let message;
    let nextPage = true;
    let principal;
    let notify = 'maybe';
    let confirmed = false;
    $:canSubmit = confirmed && validData({principal, notify});

    function showSnackbar() {
        document.getElementById("snackbar").className = "show";
    }
    function hideSnackbar() {
        const element = document.getElementById("snackbar");
        element.className = "";
        if (nextPage) {
            // window.location.href = '/';
            pageState.transactions = true;
            pageState = pageState;
            page('/transactions')
        }

    }

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
        loadSpinner(loaderId);
        nftAgent.transfer(nft, principal, shouldNotify).then((result) => {
            hideSpinner(loaderId)
            if ("Err" in result) {
                console.error(JSON.stringify(result.Err))
                message = result.Err;
                nextPage = false;
            }
            if ("Ok" in result) {
                message = "Successfully transferred."
                addTransaction(nft.index, `Transferred NFT to ${principal}`);
            }
            showSnackbar();
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

<div id="transferView">
    <Loader named={loaderId}/>
    {#await _isAuthorized then isAuthorized}
    {#if isAuthorized}
        <div id="snackbar">{message}
            <button id="snack_button" on:click={hideSnackbar}>Okay</button>
        </div>
        <form on:submit|preventDefault={transfer} class="form">
            <h2>Transfer {nft.name} ({nft.symbol}) #{nft.index}</h2>
            <label for="principal">Principal to transfer to:</label>
            <input type="text" id="principal" bind:value={principal}>

            <label id="label_notify" for="notify">Notify the recipient?</label>
            <select id="notify" bind:value={notify}>
                <option value="no">No</option>
                <option value="maybe" selected>Yes, but transfer anyway if unsupported</option>
                <option value="yes">Yes, and don't transfer without it</option>
            </select>

            <div class="agreement">
                <input type="checkbox" id="confirm" bind:checked={confirmed}>
                <label for="confirm">I understand that if this principal is wrong, I'm probably not getting this NFT back.</label>
            </div>
            <div class="div_button_primary">
                <button type="submit" class ="button_primary {!canSubmit ? "disabled": null}">
                    Transfer
                </button>
            </div>
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
    h2 {
        text-align: center;
        padding-bottom: 1em;
    }
    form {
        max-width: 500px;
        margin: 2.5em;
    }
    input {
        width: 100%;
    }
    .agreement {
        padding: 1.5em 0;
        display: flex;
        justify-content: space-between;
        align-items: center;
    }
    .agreement label {
        max-width: 90%;
        font-size: 14px;
    }
    label {
        margin-right: 1em;
        margin-bottom: 0.5em;
    }
    #principal {
        margin-bottom: 1em;
        margin-top: 0.5em;
    }
    #confirm {
        width: fit-content;
        margin-right: 15px;
        background-color: #282829;
    }
    #label_notify {
        margin-top: 1em;
    }
    #notify {
        height: 30px;
        padding: 0 5px;
    }
    #transferView {
        display: flex;
        flex-direction: column;
        align-items: center;
    }
</style>
