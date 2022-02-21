<svelte:options immutable={true}/>

<script>
    import Copier from './Copier.svelte';
    import { getCanisterId } from '../nft';
    let canisterId = getCanisterId();
</script>

<div>
    <p>Wallet canister id: <Copier text={canisterId} --default-color="#666" --highlight-color="#EEE">{canisterId}</Copier></p>
</div>

<style>
    p {
        color: #666;
        font-size: 14px;
        margin: 0 0 8px;
    }
    div {
        position: relative;
        left: 25%;
    }
</style>
