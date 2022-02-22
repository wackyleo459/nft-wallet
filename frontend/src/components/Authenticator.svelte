<script>
  import * as nftAgent from '../nft';
  import Copier from './Copier.svelte';
  let _isAuthenticated = nftAgent.isAuthenticated();
  let _isAuthorized = nftAgent.isAuthorized();
  async function getCommand() {
      const principal = await nftAgent.getPrincipal();
      return `dfx canister --no-wallet${nftAgent.isMainnet() ? ' --network ic' : ''} call '${nftAgent.getCanisterId()}' set_authorized '(principal "${principal}", true)'`;
  }
  function retry() {
      _isAuthenticated = nftAgent.isAuthenticated();
  }
  function login() {
      nftAgent.authenticate(() => {
          _isAuthenticated = nftAgent.isAuthenticated();
      });
  }
  async function _logout() {
      await nftAgent.logout();
      _isAuthenticated = nftAgent.isAuthenticated();
  }
</script>

<div class="authenticator">
    {#await _isAuthenticated then isAuthenticated}
        {#if isAuthenticated}
            {#await _isAuthorized then isAuthorized}
            {#if isAuthorized}
            <button on:click={_logout}>Log out</button>
            {:else}
            <p class="info">Unregistered user -
                {#await getCommand()}
                <s>copy registration command &cross;</s>
                {:then command}
                <Copier always text={command} --default-color="#999">copy registration command</Copier>
                {/await}
            </p>
            <button on:click={_logout}>Log out</button>
            {/if}
            {:catch}
            <p>
                <span class="error">
                    Can't reach canister
                    {#if !nftAgent.isMainnet()}
                    (is the replica running?)
                    {/if}
                </span>
                <button on:click={_logout}>Log out</button>
            </p>
            {/await}
        {:else}
        <button on:click={login}>Log in with Internet Identity</button>
        {/if}
    {:catch}
    <p class="error">Could not reach Internet Identity <button on:click={retry}>Retry &circlearrowright;</button></p>
    {/await}
</div>

<style>
    button {
        background-color: #666;
        color: white;
        font-size: 14px;
    }
    p {
        margin: 0 0 8px;
    }
    .info {
        color: #666;
        font-size: 14px;
    }
    .authenticator {
        display:flex;
        align-items: center;
        flex-direction: column;
    }
</style>
