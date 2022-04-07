<script>
    import { addTransaction, transactionHistory } from '../storage/transactionHistory.js';
    import * as nftAgent from '../nft';
    import { Principal } from '@dfinity/principal';
    import Loader, {loadSpinner, hideSpinner} from '../components/Loader.svelte';
    import {Form, FormGroup, Button, TextInput, Loading } from "carbon-components-svelte";
    let loading = false;
    let canister;
    let index;
    let showButton = false;
    let message;
    let nextPage = true;
    $:canSubmit = validPrincipal(canister) && typeof Number(index) === 'number';
    console.log('from register:', $transactionHistory);

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
        if (!canSubmit) {return}
        // loadSpinner("registerLoader");
        loading = true;
        const collection = await nftAgent.fetchAllOwnedNfts();
        index = Number(index);
        collection.forEach(nft => {
            if (Number(nft.index) === index) {
                // hideSpinner("registerLoader");
                loading = false;
                showButton = false;
                message = "NFT by that index is already registered, \nbut will continue anyway...";
                showSnackbar();
            }
        });
        const result = await nftAgent.register(canister, index);
        // hideSpinner("registerLoader");
        loading = false;
        if (result) {
            result.status === "fail" ? nextPage = false : nextPage = true;
            if (result.status === "success") {
                addTransaction(index, `Registered NFT from canister ${canister}`);
            }
            message = result.message;
            showButton = true;
            showSnackbar();
        };
    }
    function showSnackbar() {
        document.getElementById("snackbar").className = "show";
    }
    function hideSnackbar() {
        const element = document.getElementById("snackbar");
        element.className = "";
        if (nextPage) {
            page("/");
        }
    }

</script>
<script context="module">
    import page from "page";
    export const pageModule = page;
</script>

<div class="register-view">
    <Loading active={loading}/>
    <Loader named="registerLoader"/>
    <div id="snackbar">{message}
        {#if showButton}
        <button id="snack_button" on:click={hideSnackbar}>Okay</button>
        {/if}
    </div>
    {#await nftAgent.isAuthorized() then isAuthorized}
    {#if isAuthorized}
    <Form on:submit={(e)=> {
        e.preventDefault();
        register()}}
        style="padding: 50px 30px 30px; border: solid 1px grey; broder-radius: 10px; border-radius: 15px;">
        <h2>Register a new NFT</h2>
        <FormGroup>
            <TextInput size="large" labelText="NFT Canister ID" placeholder="Principal of your NFT management canister..." bind:value={canister} on:blur={validateCanister} on:focus={removeError}/>
            <div id="nft_cid_help" class="form_text">
                Please enter the principal of your NFT management canister.
            </div>
            {#if validCanister === false}
            <span class="error">Invalid principal</span>
            {/if}
        </FormGroup>
        <FormGroup>
            <TextInput size="large" labelText="Index #" placeholder="Must be non-negative integer" bind:value={index} on:blur={e => console.log('ind', index)}/>
            <div id="nft_index_help" class="form_text">
                Provide the specific token id associated with the NFT you are registering.
            </div>
            {#if isNaN(Number(index))}
            <span class="error">Not a valid index</span>
            {:else if Number(index) < 0}
            <span class="error">Index must be 0 or higher</span>
            {:else if !index}
            <span class="error">Missing index</span>
            {/if}
        </FormGroup>
        <Button click={(e) => register()} type="submit">Register</Button>
    </Form>
    {:else}
    <p>You must be an authorized user to register new NFTs to this wallet.</p>
    {/if}
    {/await}
</div>

<style>
    .register-view {
        margin: auto;
        max-width: 650px;
        padding-top: 130px;
        padding-bottom: 130px;
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
    #snack_button {
        border-radius: 4px;
        background-color: transparent;
        margin-top: 10px;
        background-color: #fcc56f;
        width: auto;
        border: solid 2px #fcc56f;
        color: black;
        width: 40%;
    }
</style>
