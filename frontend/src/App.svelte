<script>
    import WalletView from './pages/WalletView.svelte';
    import NFTView from './pages/NFTView.svelte';
    import CollectionView from './pages/CollectionView.svelte';
    import CanisterId from './components/CanisterId.svelte';
    import TransferView from './pages/TransferView.svelte';
    import Authenticator from './components/Authenticator.svelte';
    import RegisterView from './pages/RegisterView.svelte';
    import * as nftAgent from './nft';
    export let page = {};
    let walletCID;

</script>

<main class="main">
    <a href="/" class="ui">
        <h1>NFT Wallet</h1>
        <CanisterId/>
    </a>
    <Authenticator/>
    <div class="navButtons">
        <button type="button" id="home_button" class="btn">
            <a href="/">Home</a>
        </button>
        <button type="button" id="register_button" class="btn">
            <a href="/register">Register</a>
        </button>
        <!-- {#await check then canisterId} -->
            <button type="button" id="collection_button" class="btn">
                <a href="/">Collection</a>
            </button>
        <!-- {/await} -->
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
    <img src="/images/ic-badge-powered-by_bg-dark.svg" alt="powered by ic">
</main>

<style>
    @media (min-width: 640px) {
        main {
            max-width: 90%;
            margin: 0 auto;
        }
    }
    .main {
        height: 100vh;
    }
    div {
        margin-bottom: 40px;
        margin-top: 20px;
    }
    h1 {
        text-align: center;
        line-height: 2.5;
        margin-bottom: 0;
        font-family: -apple-system, BlinkMacSystemFont, 'Segoe UI', Roboto, Oxygen, Ubuntu, Cantarell, 'Open Sans', 'Helvetica Neue', sans-serif;
    }
    .navButtons {
        display: flex;
        flex-direction: row;
        justify-content: space-evenly;
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
        display: flex;
        flex-direction: column;
        justify-content: space-between;
        align-items: center;
    }
    img {
        position: relative;
        top: 100px;
        left: 50%;
        transform: translate(-50%, -50%);
        padding-bottom: 1.5em;
    }
    button {
        flex-grow: 1;
        margin: 5px;
    }
    .btn a {
        display: block;
        width: 100%;
        height: 100%;
    }
</style>
