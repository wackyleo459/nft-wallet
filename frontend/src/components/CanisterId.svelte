<svelte:options immutable={true}/>

<script>
    import Copier from './Copier.svelte';
    import { getCanisterId } from '../nft';
    let canisterId = getCanisterId();
</script>

<div id="wallet_cid">
    <p>Wallet canister id: <Copier text={canisterId} --default-color="#666" --highlight-color="#EEE">{canisterId}</Copier></p>
</div>

<style>
    p {
        color: #666;
        font-size: 14px;
        margin: 0 0 8px;
    }
    #wallet_cid{
        display: flex;
        justify-content: center;
    }
</style>
