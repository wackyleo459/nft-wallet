<script lang="ts">
    import WalletView from './pages/WalletView.svelte';
    import NFTView from './pages/NFTView.svelte';
    import CollectionView from './pages/CollectionView.svelte';
    import CanisterId from './components/CanisterId.svelte';
    import TransferView from './pages/TransferView.svelte';
    import Authenticator from './components/Authenticator.svelte';
    import RegisterView from './pages/RegisterView.svelte';
    import Loader, {loadSpinner} from './components/Loader.svelte';
    import Transactions from './components/Transactions.svelte';
    import {transactionHistory} from './transactionHistory.js';
    export let page = {};
    let named = "mainLoader";

    function viewTransactions() {
        app.$set({ page: { transactions: true } })
    }
    import { Menu, Menuitem } from 'svelte-mui/src';

    // setInterval(() => console.log('transactions array',$transactionHistory), 3000);
    import { getCanisterId } from './nft';
    let canisterId = getCanisterId();

    import {
        Theme,
        RadioButtonGroup,
        RadioButton,
        } from "carbon-components-svelte";

    let theme = "g90";
</script>
<!-- <Theme bind:theme/> -->
<div class="navBar">
        <div id="title" style="grid-area: title">
            <img id="wallet" style="grid-area: icon" src="/images/wallet.png" alt="wallet"/>
            <a href="/" id="nft_wallet_title"class="ui">NFT Wallet</a>
        </div>

        <div class="buttons">
            <button type="button" id="home_button" class="nav_button button">
                <a class="nav_b" href="/">Home</a>
            </button>
            <button type="button" id="register_button" class="nav_button button">
                <a class="nav_b" href="/register">Register</a>
            </button>
            <Menu class="menu" origin="top right" dy=38 duration=150 width=3 style="background-color: grey">
                <div slot="activator">
                    <button type="button" id="collection_button" class="nav_button button">
                        Account
                    </button>
                </div>

                <Menuitem><a class="nav_b" href="/">Login</a></Menuitem>
                <Menuitem><a class="nav_b" href="/">Logout</a></Menuitem>
                <Menuitem on:click={viewTransactions}>Transactions</Menuitem>
                <hr />
                <div id="cid">Wallet Canister ID</div>
                <Menuitem id="cid_menu">
                    {canisterId}
                </Menuitem>
            </Menu>

        </div>
    </div>
<main class="main">
    <Loader named={named}/>
    <div id="NFT_wallet">
        <CanisterId/>
        <Authenticator/>
    </div>

    <div class="content">
        {#if page.nft}
        <NFTView nft={page.nft} current={page.nftCurrent}/>
        {:else if page.register}
        <RegisterView/>
        {:else if page.transfer}
        <TransferView nft={page.transfer}/>
        {:else if page.collection}
        <CollectionView canister={page.collection}/>
        {:else if page.transactions}
        <Transactions/>
        {:else}
        <WalletView/>
        {/if}
    </div>
    <img id="ic" src="/images/ic-badge-powered-by_bg-dark.svg" alt="powered by ic">

</main>

<style lang="scss" global>
    .navBar {
        border-bottom: solid 1px #7f7f7f;
        padding: 1em;
    }
    main {
            margin: 0 15px auto;
        }
    @media (min-width: 640px) {
        .navBar {
            display: grid;
            grid-template-columns: 1fr 45%;
            grid-template-areas: "title buttons";
        }
        .buttons {
            justify-content: flex-end;
        }
    }
    .main {
        height: 100vh;
    }
    #title {
        display: flex;
        align-items: center;
        margin-bottom: 0;
        font-size:xx-large;
        font-family: -apple-system, BlinkMacSystemFont, 'Segoe UI', Roboto, Oxygen, Ubuntu, Cantarell, 'Open Sans', 'Helvetica Neue', sans-serif;
    }

    @media (max-width: 639px) {
        #title {
            justify-content: center;
        }
        .navBar {
            display: flex;
            flex-direction: column;
            justify-content: center;
        }
        .buttons {
            justify-content: center;
            display: flex;
            margin-top: 1.5em;
        }
    }
    img#wallet {
        object-fit: contain;
        max-height: 60px;
        height: 1em;
        margin-right: 10px;
    }
    a#nft_wallet_title {
        font-size-adjust:inherit;
    }
    .buttons {
        grid-area: buttons;
        display: flex;
        align-items: center;
    }
    #home_button {
        border: solid 1px #30ace3;
    }
    #register_button {
        border: solid 1px #893385;
    }
    #collection_button {
        border: solid 1px #fcc56f;
    }
    .content {
        margin-top: 3em;
        /* height: 70vh; */
        display: flex;
        flex-direction: column;
        justify-content: space-between;
        align-items: center;
    }
    img#ic {
        position: relative;
        top: 100px;
        left: 50%;
        transform: translate(-50%, -50%);
        padding-bottom: 1.5em;
    }
    button {
        /* flex-grow: 1; */
        margin: 0 5px;
    }
    .nav_button a {
        display: block;
        width: 100%;
        height: 100%;
    }
    .nav_button {
        border-radius: 4px;
        background-color: transparent;
    }

    a.nav_b {
        display: flex;
        align-items: center;
        justify-content: center;
    }

    #NFT_wallet {
        display: flex;
        flex-direction: column;
        align-items: baseline;
        margin: 0 1.3em;
        justify-content: space-between;
    }
    button {
        width: 100px;
    }

    #cid {
        margin: 0 15px;
    }
    #cid_menu {
        display: flex;
        flex-direction: column;
    }

</style>
