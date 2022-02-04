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
    </a>
    <div>
        <CanisterId/>
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
    }
</style>
