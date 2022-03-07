<script>
    import WalletView from './pages/WalletView.svelte';
    import NFTView from './pages/NFTView.svelte';
    import CollectionView from './pages/CollectionView.svelte';
    import CanisterId from './components/CanisterId.svelte';
    import TransferView from './pages/TransferView.svelte';
    import Authenticator from './components/Authenticator.svelte';
    import RegisterView from './pages/RegisterView.svelte';
    import loadSpinner from './components/Loader.svelte';
    import Loader from './components/Loader.svelte';
    import * as nftAgent from './nft';
    export let page = {};

    let walletCID;
</script>

<main class="main">

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
            <button type="button" id="collection_button"  on:click={loadSpinner} class="nav_button button">
                <a class="nav_b" href="/">About</a>
            </button>
        </div>
    </div>


    <!-- <Loader/> -->
    <div id="NFT_wallet">
        <CanisterId style="margin-bottom: 5px"/>
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
        {:else}
        <WalletView/>
        {/if}
    </div>
    <img id="ic" src="/images/ic-badge-powered-by_bg-dark.svg" alt="powered by ic">

</main>

<style>

    @media (min-width: 640px) {
        main {
            max-width: 90%;
            margin: 0 auto;
        }
        .navBar {
            display: grid;
            grid-template-columns: 1fr 45%;
            grid-template-areas:
                "title buttons";;
            margin: 1.3em;
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
            margin: 1em;
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
        height: 100%;
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

</style>
