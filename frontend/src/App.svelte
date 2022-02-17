<script>
    import WalletView from './pages/WalletView.svelte';
    import NFTView from './pages/NFTView.svelte';
    import CollectionView from './pages/CollectionView.svelte';
    import CanisterId from './components/CanisterId.svelte';
    import TransferView from './pages/TransferView.svelte';
    import Authenticator from './components/Authenticator.svelte';
    import RegisterView from './pages/RegisterView.svelte';

    export let page = {};
</script>

<main>
    <a href="/" class="ui">
        <h1>NFT Wallet</h1>
        <CanisterId/>
    </a>



    <div class="navbar">
        <a href="/">Home</a>
        <a href="/register">Register</a>
        <Authenticator/>
    </div>
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
</main>

<style>
    @media (min-width: 640px) {
        main {
            max-width: 80%;
          margin: 0 auto;
        }
    }
    div {
        margin-bottom: 40px;
        margin-top: 20px;
    }
    h1 {
        text-align: center;
        line-height: 2.5;
        margin-bottom: 0;
    }
    .navbar {
        outline: double #B484FD;
        border-radius: 20px;
        display: flex;
        justify-content: space-between;
    }
    .navbar a {
        float: left;
        color: #f2f2f2;
        text-align: center;
        padding: 14px 16px;
        text-decoration: none;
        font-size: 17px;
    }
</style>
