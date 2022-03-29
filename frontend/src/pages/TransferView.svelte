<script>
    import { Principal } from '@dfinity/principal';
    import * as nftAgent from '../nft';
    export let nft;
    import Loader, {loadSpinner, hideSpinner} from '../components/Loader.svelte';
    import { addTransaction } from '../storage/transactionHistory.js';
    import page from 'page';
    import { Modal, Form, FormGroup, Checkbox, Button, TextInput, Select, SelectItem } from "carbon-components-svelte";
    import { loop_guard } from 'svelte/internal';

    export let pageState;
    let loaderId = "transferLoader";
    let message;
    let nextPage = true;
    let principal;
    let notify;
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
    let errorMessage1;
    let errorMessage2;
    function validData({principal, notify}) {
        if (principal) {
            try {
                Principal.fromText(principal);
            } catch {
                errorMessage1 = 'Invalid principal';
                return false;
            }
        }
        if (notify) {
            if (!(['no', 'maybe', 'yes'].includes(notify))) {
                errorMessage2 = 'Select a valid notify option';
                return false;
            }
        }
        errorMessage1 = null;
        errorMessage2 = null;
        return true;
    }
    let _isAuthorized = nftAgent.isAuthorized();
</script>

<div id="transferView">
    <Loader named={loaderId}/>
    {#await _isAuthorized then isAuthorized}
    {#if isAuthorized}
    <Modal open=true modalHeading=""
        primaryButtonText="Transfer NFT"
        secondaryButtonText="Cancel"
        hasForm=true
        primaryButtonDisabled={!canSubmit? true: false}
        on:submit={(e)=> {
            e.preventDefault();
            transfer()}}
        on:click:button--secondary={(e)=> page(`/${nft.canister}/${nft.index}`)}
        >
        <h2>Transfer {nft.name} ({nft.symbol}) #{nft.index}</h2>
        <div id="snackbar">{message}
            <button id="snack_button" on:click={hideSnackbar}>Okay</button>
        </div>
        <Form
            on:submit={(e) => {
                e.preventDefault();
                transfer();
            }}
            >

            <FormGroup>
                <TextInput size="large" labelText="Principal to transfer to" placeholder="Enter valid Principal..." bind:value={principal} on:change={(e) => validData({principal})}/>
                {#if errorMessage1}
                    <p class="error">{errorMessage1}</p>
                {/if}
            </FormGroup>
            <FormGroup>
                <Select labelText="Notify Recipient?" selected="select" on:change={(e) =>
                    notify=e.detail}>
                    <SelectItem value="select" text="Select from the following..." />
                    <SelectItem value="no" text="No" />
                    <SelectItem value="maybe" text="Yes, but transfer anyway if unsupported" />
                    <SelectItem value="yes" text="Yes, and don't transfer without it" />
                </Select>
                {#if errorMessage2}
                    <p class="error">{errorMessage2}</p>
                {/if}
            </FormGroup>
            <FormGroup>
                <Checkbox id="checkbox-0" labelText="I understand that if this principal is wrong, I'm probably not getting this NFT back." bind:checked={confirmed}/>
            </FormGroup>
        </Form>
    </Modal>
    {:else}
        <p>You must be an authorized user to transfer NFTs out of this wallet.</p>
    {/if}
    {:catch}
    <p>You can't transfer NFTs out of this wallet if the canister can't be accessed.</p>
    {/await}
</div>

<style>
    h2 {
        /* text-align: center; */
        padding-bottom: 1em;
        font-size: 30px;
    }
    #transferView {
        display: flex;
        flex-direction: column;
        align-items: center;
    }
</style>
