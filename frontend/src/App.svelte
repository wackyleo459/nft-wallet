<script lang="ts">
    import WalletView from './pages/WalletView.svelte';
    import NFTView from './pages/NFTView.svelte';
    import CollectionView from './pages/CollectionView.svelte';
    import CanisterId from './components/CanisterId.svelte';
    import TransferView from './pages/TransferView.svelte';
    import Authenticator, {isLoggedIn, _logout, login} from './components/Authenticator.svelte';
    import RegisterView, {pageModule} from './pages/RegisterView.svelte';
    import Loader from './components/Loader.svelte';
    import Transactions from './components/Transactions.svelte';
    import { getCanisterId } from './nft';

    export let page: object = {};
    let canisterId = getCanisterId();
    let named = "mainLoader";

    import { Menu, Menuitem } from 'svelte-mui/src';
    // import Menu from '@smui/menu';
    import type { MenuComponentDev } from '@smui/menu';
    import Button, {Label} from "@smui/button";
    import List, { Item, Separator, Text } from '@smui/list';
    import { Anchor } from '@smui/menu-surface';
    let menu: MenuComponentDev;
    let anchor: HTMLDivElement;

    import "carbon-components-svelte/css/all.css";
    import { Theme } from "carbon-components-svelte";
    let theme = "g90";

    function goHome() {
       pageModule('/');
    };

</script>

<Theme bind:theme persist persistKey="__carbon-theme" />
<div class="navBar">
        <div id="title">
            <img id="wallet" src="/images/wallet.png" alt="wallet"/>
            <a href="/" id="nft_wallet_title"class="ui">NFT Wallet</a>
        </div>

        <div class="buttons">
            <button type="button" id="home_button" class="nav_button button"
                on:click|preventDefault={goHome}>Home
                <!-- <a class="nav_b" href="/">Home</a> -->
            </button>
            <button type="button" id="register_button" class="nav_button button">
                <a class="nav_b" href="/register">Register</a>
            </button>
            <Menu class="menu" origin="top right" dy=38 duration=150 width=3>
            <!-- <Button on:click={()=> menu.setOpen(true)}>
                <Label>Account</Label>
            </Button>
            <Menu bind:this={menu}
                anchor={false}
                bind:anchorElement={anchor}
                anchorCorner='BOTTOM_RIGHT'
                > -->

                <div slot="activator">
                    <button type="button" id="collection_button" class="nav_button button">
                        Account
                    </button>
                </div>
                {#await isLoggedIn() then loggedIn}
                    {#if !loggedIn}
                    <Menuitem on:click={login}>Login</Menuitem>
                    {:else}
                    <Menuitem on:click={_logout}>Logout</Menuitem>
                    {/if}
                {/await}
                <Menuitem>
                    <a href="/transactions">Transactions</a></Menuitem>
                <hr />
                <div id="cid">Wallet Canister ID</div>
                <Menuitem id="cid_menu">
                    <CanisterId/>
                </Menuitem>
            </Menu>

        </div>
    </div>



<main class="main">
    <Authenticator/>

    <Loader named={named}/>


    <div class="content">
        {#if page.nft}
        <NFTView nft={page.nft} current={page.nftCurrent}/>
        {:else if page.register}
        <RegisterView/>
        {:else if page.transfer}
        <TransferView nft={page.transfer} pageState={page}/>
        {:else if page.collection}
        <CollectionView canister={page.collection}/>
        {:else if page.transactions}
        <Transactions />
        {:else}
        <WalletView/>
        {/if}
    </div>
    <img id="ic" src="/images/ic-badge-powered-by_bg-dark.svg" alt="powered by ic">

</main>

<style lang="scss" global>

    .navBar {
        border-bottom: solid 3px #7f7f7f;
        padding: 1em;
        background-color: grey;
    }
    main.main {
            margin: 0 15px auto;
            height: 92vh;
            display: grid;
            grid-template-rows: 1fr auto
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
    .content {
        grid-row-start: 1;
        grid-row-end: 2;
    }
    #title {
        grid-area: title;
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
        grid-area: icon;
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
        color: white;
        display: flex;
        justify-content: center;
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
        grid-row-start: 2;
        grid-row-end: 3;
        margin: auto;
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
    // #NFT_wallet {
    //     display: flex;
    //     flex-direction: column;
    //     align-items: baseline;
    //     margin: 0 1.3em;
    //     justify-content: space-between;
    // }
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
