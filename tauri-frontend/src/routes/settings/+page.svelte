<script lang="ts">
  import { frontendData } from "$lib/stores.svelte";
</script>

{#if frontendData.state}
  <select>
    {#each frontendData.state.audio_devices as device, index}
      <option>{device}</option>
    {/each}
  </select>
{/if}
